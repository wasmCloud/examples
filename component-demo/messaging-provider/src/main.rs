//! Nats implementation for wasmcloud:wasi:messaging.
//!

#![allow(unused_macros)] // for export in wit_bindgen

use std::borrow::Cow;
use std::{collections::HashMap, convert::Infallible, sync::Arc};

use base64::{engine::general_purpose::STANDARD_NO_PAD, engine::Engine};
use dashmap::DashMap;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use tokio::sync::Semaphore;
use tokio::task::JoinHandle;
use tracing::{debug, error, instrument, warn};
use tracing_futures::Instrument;
use wascap::prelude::KeyPair;
use wasmbus_rpc::common::Transport;
use wasmbus_rpc::{
    async_nats,
    core::{HostData, LinkDefinition},
    otel::OtelHeaderInjector,
    provider::prelude::*,
};

// from wit-bindgen
use wasmcloud_messaging::{PublishDataParam, PublishDataResult};

wit_bindgen::generate!({
    path: "../messaging-demo/wit",
    world: "messaging-capability-provider",
    serialize: "serde::Serialize",
    deserialize: "serde::Deserialize",
});
use messaging_types::{EventParam, EventResult};

const DEFAULT_NATS_URI: &str = "0.0.0.0:4222";
const ENV_NATS_SUBSCRIPTION: &str = "SUBSCRIPTION";
const ENV_NATS_URI: &str = "URI";
const ENV_NATS_CLIENT_JWT: &str = "CLIENT_JWT";
const ENV_NATS_CLIENT_SEED: &str = "CLIENT_SEED";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // handle lattice control messages and forward rpc to the provider dispatch
    // returns when provider receives a shutdown control message
    let host_data = load_host_data()?;
    let provider = generate_provider(host_data);
    provider_main(provider, Some("NATS Messaging Provider".to_string()))?;

    eprintln!("NATS messaging provider exiting");
    Ok(())
}

fn generate_provider(host_data: HostData) -> NatsMessagingProvider {
    if let Some(c) = host_data.config_json.as_ref() {
        // empty string becomes the default configuration
        if c.trim().is_empty() {
            NatsMessagingProvider::default()
        } else {
            let config: ConnectionConfig = serde_json::from_str(c)
                .expect("JSON deserialization from connection config should have worked");
            NatsMessagingProvider {
                default_config: config,
                ..Default::default()
            }
        }
    } else {
        NatsMessagingProvider::default()
    }
}

/// Configuration for connecting a nats client.
/// More options are available if you use the json than variables in the values string map.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct ConnectionConfig {
    /// list of topics to subscribe to
    #[serde(default)]
    subscriptions: Vec<String>,
    #[serde(default)]
    cluster_uris: Vec<String>,
    #[serde(default)]
    auth_jwt: Option<String>,
    #[serde(default)]
    auth_seed: Option<String>,

    /// ping interval in seconds
    #[serde(default)]
    ping_interval_sec: Option<u16>,
}

impl ConnectionConfig {
    fn merge(&self, extra: &ConnectionConfig) -> ConnectionConfig {
        let mut out = self.clone();
        if !extra.subscriptions.is_empty() {
            out.subscriptions = extra.subscriptions.clone();
        }
        // If the default configuration has a URL in it, and then the link definition
        // also provides a URL, the assumption is to replace/override rather than combine
        // the two into a potentially incompatible set of URIs
        if !extra.cluster_uris.is_empty() {
            out.cluster_uris = extra.cluster_uris.clone();
        }
        if extra.auth_jwt.is_some() {
            out.auth_jwt = extra.auth_jwt.clone()
        }
        if extra.auth_seed.is_some() {
            out.auth_seed = extra.auth_seed.clone()
        }
        if extra.ping_interval_sec.is_some() {
            out.ping_interval_sec = extra.ping_interval_sec.clone()
        }
        out
    }
}

impl Default for ConnectionConfig {
    fn default() -> ConnectionConfig {
        ConnectionConfig {
            subscriptions: vec![],
            cluster_uris: vec![DEFAULT_NATS_URI.to_string()],
            auth_jwt: None,
            auth_seed: None,
            ping_interval_sec: None,
        }
    }
}

impl ConnectionConfig {
    fn new_from(values: &HashMap<String, String>) -> RpcResult<ConnectionConfig> {
        let mut config = if let Some(config_b64) = values.get("config_b64") {
            let bytes = STANDARD_NO_PAD.decode(config_b64.as_bytes()).map_err(|e| {
                RpcError::InvalidParameter(format!("invalid base64 encoding: {}", e))
            })?;
            serde_json::from_slice::<ConnectionConfig>(&bytes)
                .map_err(|e| RpcError::InvalidParameter(format!("corrupt config_b64: {}", e)))?
        } else if let Some(config) = values.get("config_json") {
            serde_json::from_str::<ConnectionConfig>(config)
                .map_err(|e| RpcError::InvalidParameter(format!("corrupt config_json: {}", e)))?
        } else {
            ConnectionConfig::default()
        };
        if let Some(sub) = values.get(ENV_NATS_SUBSCRIPTION) {
            config
                .subscriptions
                .extend(sub.split(',').map(|s| s.to_string()));
        }
        if let Some(url) = values.get(ENV_NATS_URI) {
            config.cluster_uris.push(url.clone());
        }
        if let Some(jwt) = values.get(ENV_NATS_CLIENT_JWT) {
            config.auth_jwt = Some(jwt.clone());
        }
        if let Some(seed) = values.get(ENV_NATS_CLIENT_SEED) {
            config.auth_seed = Some(seed.clone());
        }
        if config.auth_jwt.is_some() && config.auth_seed.is_none() {
            return Err(RpcError::InvalidParameter(
                "if you specify jwt, you must also specify a seed".to_string(),
            ));
        }
        if config.cluster_uris.is_empty() {
            config.cluster_uris.push(DEFAULT_NATS_URI.to_string());
        }
        Ok(config)
    }
}

/// NatsClientBundles hold a NATS client and information (subscriptions)
/// related to it.
///
/// This struct is necssary because subscriptions are *not* automatically removed on client drop,
/// meaning that we must keep track of all subscriptions to close once the client is done
#[derive(Debug)]
struct NatsClientBundle {
    pub client: async_nats::Client,
    pub ld: LinkDefinition,
    pub sub_handles: Vec<(String, JoinHandle<()>)>,
}

impl Drop for NatsClientBundle {
    fn drop(&mut self) {
        for handle in &self.sub_handles {
            handle.1.abort()
        }
    }
}

/// Nats implementation for wasmcloud:messaging
#[derive(Default, Clone)] // note no 'Provider' this time
struct NatsMessagingProvider {
    // store nats connection client per actor
    actors: Arc<DashMap<String, NatsClientBundle>>,
    //actors: Arc<RwLock<HashMap<String, NatsClientBundle>>>,
    default_config: ConnectionConfig,
}

// use default implementations of provider message handlers
impl ProviderDispatch for NatsMessagingProvider {}

impl NatsMessagingProvider {
    /// Attempt to connect to nats url (with jwt credentials, if provided)
    async fn connect(
        &self,
        cfg: ConnectionConfig,
        ld: &LinkDefinition,
    ) -> Result<NatsClientBundle, RpcError> {
        let opts = match (cfg.auth_jwt, cfg.auth_seed) {
            (Some(jwt), Some(seed)) => {
                let key_pair = std::sync::Arc::new(
                    KeyPair::from_seed(&seed)
                        .map_err(|e| RpcError::ProviderInit(format!("key init: {}", e)))?,
                );
                async_nats::ConnectOptions::with_jwt(jwt, move |nonce| {
                    let key_pair = key_pair.clone();
                    async move { key_pair.sign(&nonce).map_err(async_nats::AuthError::new) }
                })
            }
            (None, None) => async_nats::ConnectOptions::default(),
            _ => {
                return Err(RpcError::InvalidParameter(
                    "must provide both jwt and seed for jwt authentication".into(),
                ));
            }
        };
        let url = cfg.cluster_uris.get(0).unwrap();

        let client = opts
            .name("NATS Messaging Provider") // allow this to show up uniquely in a NATS connection list
            .connect(url)
            .await
            .map_err(|e| RpcError::ProviderInit(format!("NATS connection to {}: {}", url, e)))?;

        // Connections
        let mut sub_handles = Vec::new();
        for sub in cfg.subscriptions.iter().filter(|s| !s.is_empty()) {
            let (sub, queue) = match sub.split_once('|') {
                Some((sub, queue)) => (sub, Some(queue.to_string())),
                None => (sub.as_str(), None),
            };

            sub_handles.push((
                sub.to_string(),
                self.subscribe(&client, ld.clone(), sub.to_string(), queue)
                    .await?,
            ));
        }

        Ok(NatsClientBundle {
            client,
            sub_handles,
            ld: ld.clone(),
        })
    }

    /// Add a regular or queue subscription
    async fn subscribe(
        &self,
        client: &async_nats::Client,
        link_def: LinkDefinition,
        sub: String,
        queue: Option<String>,
    ) -> RpcResult<JoinHandle<()>> {
        let mut subscriber = match queue {
            Some(queue) => client.queue_subscribe(sub.clone(), queue).await,
            None => client.subscribe(sub.clone()).await,
        }
        .map_err(|e| {
            error!(subject = %sub, error = %e, "error subscribing subscribing");
            RpcError::Nats(format!("subscription to {}: {}", sub, e))
        })?;

        // Spawn a thread that listens for messages coming from NATS
        // this thread is expected to run the full duration that the provider is available
        let join_handle = tokio::spawn(async move {
            // MAGIC NUMBER: Based on our benchmark testing, this seems to be a good upper limit
            // where we start to get diminishing returns. We can consider making this
            // configurable down the line.
            // NOTE (thomastaylor312): It may be better to have a semaphore pool on the
            // NatsMessagingProvider struct that has a global limit of permits so that we don't end
            // up with 20 subscriptions all getting slammed with up to 75 tasks, but we should wait
            // to do anything until we see what happens with real world usage and benchmarking
            let semaphore = Arc::new(Semaphore::new(75));
            let ld = link_def.clone();

            // Listen for NATS message(s)
            while let Some(msg) = subscriber.next().await {
                // Set up tracing context for the NATS message
                let span = tracing::debug_span!("handle_message", actor_id = %link_def.actor_id);
                span.in_scope(|| {
                    wasmbus_rpc::otel::attach_span_context(&msg);
                });

                let _permit = match semaphore.clone().acquire_owned().await {
                    Ok(p) => p,
                    Err(_) => {
                        warn!("Work pool has been closed, exiting queue subscribe");
                        break;
                    }
                };

                // for compatibility with wasi-messaging as of 2023-03-14
                // we don't have to do this but it feels safer
                let message = match serde_json::from_slice::<EventResult>(&msg.payload) {
                    Ok(_) => msg.payload,
                    Err(_) => {
                        let time = chrono::Utc::now().to_rfc3339();
                        let uuid = uuid::Uuid::new_v4().to_string();
                        let event = EventParam {
                            specversion: "1.0",
                            ty: "org.wasmcloud.wasi-nats.event",
                            source: &msg.subject,
                            id: &uuid,
                            data: Some(&msg.payload),
                            datacontenttype: Some("application/json"),
                            dataschema: None,
                            subject: Some(&msg.subject),
                            time: Some(&time),
                            extensions: None,
                        };
                        match rmp_serde::to_vec(&event) {
                            Err(error) => {
                                // highly unlikely to get this since it derives Serialize
                                error!(subject=%msg.subject,
                           actor_id=%link_def.actor_id,
                           error=error.to_string(),
                            "subscription event not sent: couldn't serialize generated event");
                                continue;
                            }
                            Ok(m) => m.into(),
                        }
                    }
                };
                //
                // once we complete the transition to wasi-kk
                /*
                 */

                // assuming it's already a CloudEvent, just use it!
                // should we deserialize to test?

                let data = PublishDataParam {
                    subject: &msg.subject,
                    message: &message,
                };
                debug!(actor_id=%link_def.actor_id, subject=%msg.subject, "subscription event received");
                // unwrap ok here: serialize can't fail here
                let data = rmp_serde::to_vec(&data).unwrap();
                tokio::spawn(subscriber_event(ld.clone(), data).instrument(span));
            }
        });

        Ok(join_handle)
    }
}

#[async_trait::async_trait]
impl wasmbus_rpc::common::MessageDispatch for NatsMessagingProvider {
    /// handle rpc from linked actor
    async fn dispatch(
        &self,
        ctx: &wasmbus_rpc::common::Context,
        message: wasmbus_rpc::common::Message<'_>,
    ) -> Result<Vec<u8>, RpcError> {
        let actor_id = match ctx.actor.as_ref() {
            None => {
                error!("invalid request 'Messaging.Producer.publish, ctx.actor missing");
                return Err(RpcError::InvalidParameter("missing ctx.actor".into()));
            }
            Some(r) => r.to_string(),
        };
        let nats_bundle = self
            .actors
            .get(&actor_id)
            .ok_or_else(|| RpcError::InvalidParameter(format!("actor not linked:{}", &actor_id)))?;

        let nats_client = &nats_bundle.client;
        let headers = OtelHeaderInjector::default_with_span().into();
        match message.method {
            "Messaging.Producer.publish" => {
                let request =
                    rmp_serde::from_slice::<PublishDataResult>(&message.arg).map_err(|error| {
                        error!(%error, "deserializing PublishDataResult");
                        RpcError::Deser(error.to_string())
                    })?;
                // for now, don't have a reply-to in this api
                nats_client
                    .publish_with_headers(request.subject, headers, request.message.into())
                    .await
                    .map_err(|e| RpcError::Nats(e.to_string()))
                    .map(|_| Vec::new())
            }
            "Messaging.Consumer.subscribe" => {
                let subject = rmp_serde::from_slice::<String>(&message.arg).map_err(|error| {
                    error!(%error, "deserializing string subject from 'subscribe'");
                    RpcError::Deser(error.to_string())
                })?;
                // TODO: (SECURITY) - this allows subscribing to _any_ nats topic.
                // for safety, the linkdef should include a list of allowed subscriptions,
                // or possibly a regex. before subscribing, check that subscribe request,
                // matches the regex. If not, return error for subscribe call.
                let _join = self
                    .subscribe(&nats_client, nats_bundle.ld.clone(), subject, None)
                    .await;
                Ok(Vec::new())
            }
            "Messaging.Consumer.unsubscribe" => {
                // TODO: (unimplemented)
                // would require creating channel, and changing wait loop
                // in subscribe loop to use select() channel or next()
                // also requires keepling list of current subscriptions
                // and only unsubscribing to ones already subscribed
                Ok(Vec::new())
            }
            _ => Err(wasmbus_rpc::error::RpcError::MethodNotHandled({
                format!("{} - unknown method", message.method)
            })),
        }
    }
}

// handle message received on a subscription. Send CloudEvent to subscribing Component
#[instrument(level = "debug", skip_all, fields(actor_id = %link_def.actor_id))]
async fn subscriber_event(link_def: LinkDefinition, message: Vec<u8>) {
    let msg = wasmbus_rpc::common::Message {
        method: "Messaging.Handler.on_receive",
        arg: Cow::Owned(message),
    };
    let tp = wasmbus_rpc::provider::ProviderTransport::new(&link_def, None);
    if let Err(error) = tp
        .send(&wasmbus_rpc::common::Context::default(), msg, None)
        .await
    {
        error!(%error, actor_id=%link_def.actor_id,
            "sending subscription event, calling on_receive",
        );
    }
}

/// Handle provider control commands
/// put_link (new actor link command), del_link (remove link command), and shutdown
#[async_trait]
impl ProviderHandler for NatsMessagingProvider {
    /// Provider should perform any operations needed for a new link,
    /// including setting up per-actor resources, and checking authorization.
    /// If the link is allowed, return true, otherwise return false to deny the link.
    #[instrument(level = "debug", skip(self, ld), fields(actor_id = %ld.actor_id))]
    async fn put_link(&self, ld: &LinkDefinition) -> RpcResult<bool> {
        // If the link definition values are empty, use the default connection configuration
        let config = if ld.values.is_empty() {
            self.default_config.clone()
        } else {
            // create a config from the supplied values and merge that with the existing default
            match ConnectionConfig::new_from(&ld.values) {
                Ok(cc) => self.default_config.merge(&cc),
                Err(e) => {
                    error!("Failed to build connection configuration: {e:?}");
                    return Ok(false);
                }
            }
        };

        self.actors
            .insert(ld.actor_id.to_string(), self.connect(config, ld).await?);

        Ok(true)
    }

    /// Handle notification that a link is dropped: close the connection
    #[instrument(level = "info", skip(self))]
    async fn delete_link(&self, actor_id: &str) {
        if let Some(bundle) = self.actors.remove(actor_id) {
            // Note: subscriptions will be closed via Drop on the NatsClientBundle
            debug!(
                "closing [{}] NATS subscriptions for actor [{}]...",
                bundle.1.sub_handles.len(),
                actor_id,
            );
        }

        debug!("finished processing delete link for actor [{}]", actor_id);
    }

    /// Handle shutdown request by closing all connections
    async fn shutdown(&self) -> Result<(), Infallible> {
        // empty the actor link data and stop all servers
        self.actors.clear();
        // dropping all connections should send unsubscribes and close the connections.
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{generate_provider, ConnectionConfig, NatsMessagingProvider};
    use wasmbus_rpc::{
        core::{HostData, LinkDefinition},
        provider::ProviderHandler,
    };

    #[test]
    fn test_default_connection_serialize() {
        // test to verify that we can default a config with partial input
        let input = r#"
{
    "cluster_uris": ["nats://soyvuh"],
    "auth_jwt": "authy",
    "auth_seed": "seedy"
}
"#;

        let config: ConnectionConfig = serde_json::from_str(&input).unwrap();
        assert_eq!(config.auth_jwt.unwrap(), "authy");
        assert_eq!(config.auth_seed.unwrap(), "seedy");
        assert_eq!(config.cluster_uris, ["nats://soyvuh"]);
        assert!(config.subscriptions.is_empty());
        assert!(config.ping_interval_sec.is_none());
    }

    #[test]
    fn test_generate_provider_works_with_empty_string() {
        let mut host_data = HostData::default();
        host_data.config_json = Some("".to_string());
        let prov = generate_provider(host_data);
        assert_eq!(prov.default_config, ConnectionConfig::default());
    }

    #[test]
    fn test_generate_provider_works_with_none() {
        let mut host_data = HostData::default();
        host_data.config_json = None;
        let prov = generate_provider(host_data);
        assert_eq!(prov.default_config, ConnectionConfig::default());
    }

    #[test]
    fn test_connectionconfig_merge() {
        // second > original, individual vec fields are replace not extend
        let mut cc1 = ConnectionConfig::default();
        cc1.cluster_uris = vec!["old_server".to_string()];
        cc1.subscriptions = vec!["topic1".to_string()];
        let mut cc2 = ConnectionConfig::default();
        cc2.cluster_uris = vec!["server1".to_string(), "server2".to_string()];
        cc2.auth_jwt = Some("jawty".to_string());
        let cc3 = cc1.merge(&cc2);
        assert_eq!(cc3.cluster_uris, cc2.cluster_uris);
        assert_eq!(cc3.subscriptions, cc1.subscriptions);
        assert_eq!(cc3.auth_jwt, Some("jawty".to_string()))
    }

    /// Ensure that unlink triggers subscription removal
    /// https://github.com/wasmCloud/capability-providers/issues/196
    ///
    /// NOTE: this is tested here for easy access to put_link/del_link without
    /// the fuss of loading/managing individual actors in the lattice
    #[tokio::test]
    async fn test_unlink_unsub() {
        // Build a nats messaging provider
        let prov = NatsMessagingProvider::default();

        // Actor should have no clients and no subs before hand
        assert_eq!(prov.actors.len(), 0);

        // Add a provider
        let mut ld = LinkDefinition::default();
        ld.actor_id = String::from("???");
        ld.link_name = String::from("test");
        ld.contract_id = String::from("test");
        ld.values = HashMap::<String, String>::from([
            (
                String::from("SUBSCRIPTION"),
                String::from("test.wasmcloud.unlink"),
            ),
            (String::from("URI"), String::from("127.0.0.1:4222")),
        ]);
        let _ = prov.put_link(&ld).await;

        // After putting a link there should be one sub
        assert_eq!(prov.actors.len(), 1);
        assert_eq!(prov.actors.get("???").unwrap().sub_handles.len(), 1);

        // Remove link (this should kill the subscription)
        let _ = prov.delete_link(&ld.actor_id).await;

        // After removing a link there should be no subs
        assert_eq!(prov.actors.len(), 0);

        let _ = prov.shutdown().await;
    }
}
