use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{borrow::Cow, time::Duration};
use test_log::test;
use tokio::sync::oneshot;
use tracing::{debug, error};
use wasmbus_rpc::{
    async_nats,
    core::InvocationResponse,
    error::{RpcError, RpcResult},
    provider::ProviderTransport,
};
//use wasmbus_rpc::provider::prelude::*;
use wasmcloud_test_util::{
    check,
    cli::print_test_results,
    provider_test::test_provider,
    run_selected_spawn,
    testing::{self, TestOptions},
};

// nats topic used to test publish
const TEST_PUBLISH_TOPIC: &str = "test.nats.pub";

// nats topic used to test subscribe and event delivery
const TEST_SUBSCRIBE_TOPIC: &str = "test.subscription";

#[derive(Serialize, Deserialize)]
struct PubMessage {
    subject: String,
    message: Vec<u8>,
}

#[test(tokio::test(flavor = "multi_thread"))]
async fn run_all() {
    let opts = TestOptions::default();
    let res = run_selected_spawn!(
        opts,
        health_check,
        send_publish,
        send_subscribe,
        send_events
    );

    debug!("DGG: run_seleted returned .. prining results");
    print_test_results(&res);

    let passed = res.iter().filter(|tr| tr.passed).count();
    let total = res.len();
    assert_eq!(passed, total, "{} passed out of {}", passed, total);

    let provider = test_provider().await;
    // try to let the provider shut dowwn gracefully
    let _ = provider.shutdown().await;
}

/// test that health check returns healthy
async fn health_check(_opt: &TestOptions) -> RpcResult<()> {
    let prov = test_provider().await;
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    // health check
    let hc = prov.health_check().await;
    check!(hc.is_ok())?;
    Ok(())
}

/// create a thread to subscribe to a topic, respond to 'count' messages, and exit
async fn make_responder(
    topic: String,
    count: usize,
) -> (oneshot::Receiver<()>, tokio::task::JoinHandle<usize>) {
    use futures::StreamExt as _;
    let (tx, rx) = oneshot::channel();
    let join = tokio::spawn(async move {
        let conn = match async_nats::ConnectOptions::default()
            .connect("127.0.0.1:4222")
            .await
        {
            Ok(c) => c,
            Err(error) => {
                error!(
                    error = error.to_string(),
                    "failed to connect test responder to nats"
                );
                return 0;
            }
        };
        debug!(topic, count, "nats subscriber listening");
        eprintln!("nats subscriber listening subject={topic}, count={count}");
        let mut sub = match conn.subscribe(topic).await {
            Err(error) => {
                error!(error, "test failed to subscribe");
                return 0;
            }
            Ok(conn) => conn,
        };
        assert!(tx.send(()).is_ok());
        for completed in 0..count {
            let msg = if let Some(msg) = sub.next().await {
                msg
            } else {
                break;
            };
            debug!(
                subject=%msg.subject, payload=%String::from_utf8_lossy(&msg.payload),
                "nats subscriber received message",
            );
            println!(
                "nats subscriber received message: subject={}, payload={}",
                &msg.subject,
                String::from_utf8_lossy(&msg.payload).as_ref()
            );

            if let Some(reply) = msg.reply {
                let response = format!("{}:{}", completed, &String::from_utf8_lossy(&msg.payload));
                if let Err(error) = conn.publish(reply, response.into_bytes().into()).await {
                    error!(error=error.to_string(), num_completed=%completed, "responder failed sending reply");
                }
                let _ = conn.flush().await.unwrap();
            } else {
                debug!(num_completed=%completed, "message did not have a reply address");
            }
        }
        debug!(count, "nats subscriber reached max count, exiting");
        eprintln!("nats subscriber reached max count {count}, exiting");
        count
    });
    (rx, join)
}

/// send publish
/// Tests that capability provider responds to rpc message for producer.publish
//
async fn send_publish(_opt: &TestOptions) -> RpcResult<()> {
    let provider = test_provider().await;
    let timeout = Duration::from_millis(match provider.config.get("timeout_ms") {
        Some(toml::Value::String(s)) => s.parse::<u64>().unwrap(),
        _ => 2000,
    });
    let ld = provider.host_data.link_definitions.get(0).unwrap();

    // start responder thread, wait till it's running
    let (rx, _responder) = make_responder(TEST_PUBLISH_TOPIC.to_string(), 1).await;
    assert!(rx.await.is_ok());

    // create client and ctx
    let tp = ProviderTransport::new(ld, None);
    let body = rmp_serde::to_vec(&PubMessage {
        subject: TEST_PUBLISH_TOPIC.to_string(),
        message: b"hello".to_vec(),
    })
    .unwrap();
    let msg = wasmbus_rpc::common::Message {
        method: "Messaging.Producer.publish",
        arg: Cow::Owned(body),
    };
    debug!(
        method=%msg.method, subject=%TEST_PUBLISH_TOPIC,
        "sending message to provider",
    );
    let resp = provider
        .rpc_client
        .send_timeout(
            // these parameters simulate what would happen if actor sent the request to the provider
            // origin: some actor that is linked to the provider
            // target: the wasi-messaging capability provider
            ld.actor_entity(),
            ld.provider_entity(),
            tp.bridge.lattice_prefix(),
            msg,
            timeout,
        )
        .await;
    assert!(resp.is_ok());
    Ok(())
}

/// send subscribe
/// Tests that capability provider responds to rpc message for consumer.subscribe
async fn send_subscribe(_opt: &TestOptions) -> RpcResult<()> {
    let provider = test_provider().await;
    let timeout = Duration::from_millis(match provider.config.get("timeout_ms") {
        Some(toml::Value::String(s)) => s.parse::<u64>().unwrap(),
        _ => 2000,
    });
    let ld = provider.host_data.link_definitions.get(0).unwrap();

    // create client and ctx
    let tp = ProviderTransport::new(ld, None);
    let msg = wasmbus_rpc::common::Message {
        method: "Messaging.Consumer.subscribe",
        arg: Cow::Owned(rmp_serde::to_vec(TEST_SUBSCRIBE_TOPIC).unwrap()),
    };
    let resp = provider
        .rpc_client
        .send_timeout(
            // these parameters simulate what would happen if actor sent the request to the provider
            // origin: some actor that is linked to the provider
            // target: the wasi-messaging capability provider
            ld.actor_entity(),
            ld.provider_entity(),
            tp.bridge.lattice_prefix(),
            msg,
            timeout,
        )
        .await;
    assert!(resp.is_ok());
    Ok(())
}

/// test that subscribe works and that provider delivers events
/// Tests that capability provider responds to rpc message for consumer.subscribe
async fn send_events(_opt: &TestOptions) -> RpcResult<()> {
    let _provider = test_provider().await;

    // create actor that subscribes on topic
    let (rx, actor_thread) = mock_subscriber_actor(2).await;
    // wait till actor thread has started and ready to recieve events
    // actor is istening on the nats topic that corresponds with wasmbus rpc for that actor id
    let _ = rx.await;

    // if we've already run send_subscribe test, then provider is already subscribing on this topic
    // and waiting for events on TEST_SUBSCRIBE_TOPIC

    // start sending events on that topic, using a different nats client
    let nats = async_nats::connect("127.0.0.1:4222").await.unwrap();
    nats.publish(
        TEST_SUBSCRIBE_TOPIC.into(),
        json!({
                "specversion": "1.0",
                "ty": "org.example.test",
                "source": "test",
                "id": "314159",
                "data": Some(b"hello".to_vec()),
                "datacontenttype": Some("application/json"),
                //"dataschema": None,
                "subject": TEST_SUBSCRIBE_TOPIC,
                //"time": None,
                //"extensions": None,
        })
        .to_string()
        .as_bytes()
        .to_vec()
        .into(),
    )
    .await
    .map_err(|e| RpcError::Nats(e.to_string()))?;
    nats.flush()
        .await
        .map_err(|e| RpcError::Nats(e.to_string()))?;

    // if payload is not a CloudEvent, it should be converted to one
    nats.publish(TEST_SUBSCRIBE_TOPIC.into(), b"hello-raw".to_vec().into())
        .await
        .map_err(|e| RpcError::Nats(e.to_string()))?;

    nats.flush()
        .await
        .map_err(|e| RpcError::Nats(e.to_string()))?;

    let result = actor_thread.await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 2); // number of messages

    Ok(())
}

/// This mock actor runs in a separate thread, listening for rpc requests.
/// The thread quits if the number of expected messages has been completed,
/// or if there was any error.
async fn mock_subscriber_actor(
    num_requests: usize,
) -> (
    oneshot::Receiver<()>,
    tokio::task::JoinHandle<Result<usize, anyhow::Error>>,
) {
    use futures::StreamExt as _;
    use wasmbus_rpc::{common::deserialize, core::Invocation};
    let (tx, rx) = oneshot::channel();
    let handle = tokio::runtime::Handle::current();
    (
        rx,
        handle.spawn(async move {
            let mut completed = 0usize;

            let prov = test_provider().await;
            let actor_rpc_topic = format!("wasmbus.rpc.TEST.{}", &prov.actor_id);
            // a host would use queue_subscribe
            let mut sub = prov
                .nats_client
                .subscribe(actor_rpc_topic)
                .await
                .map_err(|e| RpcError::Nats(e.to_string()))?;
            let _ = tx.send(());
            loop {
                if completed >= num_requests {
                    break;
                }
                match sub.next().await {
                    None => {
                        break;
                    }
                    Some(msg) => {
                        let inv: Invocation = deserialize(&msg.payload)?;
                        eprintln!(
                            "mock actor subscriber received invocation, op={}",
                            &inv.operation
                        );
                        if &inv.operation != "Messaging.Handler.on_receive" {
                            return Err(anyhow!(
                                "unexpected method received by actor: {}",
                                &inv.operation
                            ));
                        }
                        let rpc = rmp_serde::from_slice::<PubMessage>(&inv.msg).map_err(|e| {
                            eprintln!("invalid PubMessage: {:?}", &e);
                            anyhow!("invalid PubMessage: {}", e)
                        })?;
                        eprintln!(
                            "json payload (len={}):{}",
                            rpc.message.len(),
                            String::from_utf8_lossy(&rpc.message)
                        );
                        let event = serde_json::from_slice::<serde_json::Value>(&rpc.message)
                            .map_err(|e| {
                                eprintln!("invalid json :{:?}", &e);
                                anyhow!("invalid json: {}", e)
                            })?;

                        eprintln!("mock actor received event: {:#?}", &event);
                        if let Some(reply) = msg.reply {
                            // send invocation response, if they were asking. For this message, payload is empty
                            let mut resp = InvocationResponse::default();
                            resp.msg = rmp_serde::to_vec(&Vec::<u8>::new()).unwrap();
                            resp.invocation_id = inv.id;

                            prov.nats_client
                                .publish(reply, rmp_serde::to_vec(&resp).unwrap().into())
                                .await?;
                            let _ = prov.nats_client.flush().await;
                        }
                        completed += 1;
                    }
                }
            }
            Ok(completed)
        }),
    )
}
