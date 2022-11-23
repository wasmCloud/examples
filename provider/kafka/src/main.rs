//! Implementation for wasmcloud:messaging
//!
use std::{
    convert::Infallible,
    sync::{Arc, RwLock},
};

use futures::StreamExt;
use rskafka::client::{
    consumer::{StartOffset, StreamConsumerBuilder},
    ClientBuilder,
};

use kafka::consumer::Consumer;
use tokio::runtime::Handle;
use tracing::{debug, instrument};
use wasmbus_rpc::{core::LinkDefinition, provider::prelude::*};
use wasmcloud_interface_messaging::{
    MessageSubscriber, MessageSubscriberSender, Messaging, MessagingReceiver, PubMessage,
    ReplyMessage, RequestMessage, SubMessage,
};

const KAFKA_HOSTS: &str = "WASMCLOUD_KAFKA_HOSTS";
const DEFAULT_HOST: &str = "127.0.0.1:9092";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // handle lattice control messages and forward rpc to the provider dispatch
    // returns when provider receives a shutdown control message
    let hosts = std::env::var(KAFKA_HOSTS)
        .unwrap_or_else(|_| DEFAULT_HOST.to_string())
        .trim()
        .split(',')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    // get partition client
    let client = ClientBuilder::new(hosts).build().await.unwrap();
    let partition_client = Arc::new(client.partition_client("my-topic", 0).unwrap());

    // construct stream consumer
    let mut stream = StreamConsumerBuilder::new(partition_client, StartOffset::Earliest)
        .with_max_wait_ms(100)
        .build();

    // consume data
    while let Some(Ok((record, water_mark))) = stream.next().await {
        println!(
            "FROM REDPANDA {:?} {:?}",
            String::from_utf8_lossy(&record.record.value.unwrap()),
            water_mark
        );
    }

    // let kafka = KafkaProvider::new(hosts);
    //TODO: adjust args
    // provider_run(KafkaProvider::new(), Some("Kafka Provider".to_string()))?;
    // println!("consumer: {:?}", kafka.consumer);
    // println!(
    //     "client stuff: {:?}",
    //     kafka.consumer.write().unwrap();
    // );
    // consume_messages(kafka.consumer).unwrap();

    std::thread::park();
    eprintln!("Kafka provider exiting");
    Ok(())
}

/// Implementation for wasmcloud:messaging
#[derive(Clone, Provider)]
#[services(Messaging)]
struct KafkaProvider {
    // producer: Arc<RwLock<Producer>>,
    consumer: Arc<RwLock<Consumer>>,
}

impl KafkaProvider {
    fn new(hosts: Vec<String>) -> Self {
        Self {
            //TODO: configure this
            // producer: Arc::new(RwLock::new(
            //     Producer::from_hosts(hosts.clone()).create().unwrap(),
            // )),
            consumer: Arc::new(RwLock::new(
                Consumer::from_hosts(hosts)
                    .with_topic("my-topic".to_owned())
                    // .with_fallback_offset(FetchOffset::Earliest)
                    .with_group("my-group-unique".to_owned())
                    // .with_offset_storage(GroupOffsetStorage::Kafka)
                    .create()
                    .unwrap(),
            )),
        }
    }
}
// use default implementations of provider message handlers
impl ProviderDispatch for KafkaProvider {}

/// Handle provider control commands
/// put_link (new actor link command), del_link (remove link command), and shutdown
#[async_trait]
impl ProviderHandler for KafkaProvider {
    /// Provider should perform any operations needed for a new link,
    /// including setting up per-actor resources, and checking authorization.
    /// If the link is allowed, return true, otherwise return false to deny the link.
    #[instrument(level = "info", skip(self))]
    async fn put_link(&self, ld: &LinkDefinition) -> RpcResult<bool> {
        debug!("putting link for actor {:?}", ld);
        // let consumer = self.consumer.clone();
        // let ld = ld.clone();

        Ok(true)
    }

    /// Handle notification that a link is dropped: close the connection
    #[instrument(level = "info", skip(self))]
    async fn delete_link(&self, actor_id: &str) {
        debug!("deleting link for actor {}", actor_id);
    }

    /// Handle shutdown request with any cleanup necessary
    async fn shutdown(&self) -> std::result::Result<(), Infallible> {
        Ok(())
    }
}

/// Handle Messaging methods
#[async_trait]
impl Messaging for KafkaProvider {
    #[instrument(level = "debug", skip(self, msg), fields(subject = %msg.subject, reply_to = ?msg.reply_to, body_len = %msg.body.len()))]
    async fn publish(&self, _ctx: &Context, msg: &PubMessage) -> RpcResult<()> {
        debug!("Publishing message: {:?}", msg);
        Err(RpcError::NotImplemented)
    }

    #[instrument(level = "debug", skip(self, msg), fields(subject = %msg.subject))]
    async fn request(&self, _ctx: &Context, msg: &RequestMessage) -> RpcResult<ReplyMessage> {
        debug!("Sending message request: {:?}", msg);
        Err(RpcError::NotImplemented)
    }
}

fn consume_messages(consumer: Arc<RwLock<Consumer>>) -> RpcResult<()> {
    tokio::task::spawn_blocking(move || loop {
        loop {
            let mut consumer = consumer.write().unwrap();
            if let Ok(msgs) = consumer.poll() {
                println!("msg sets");
                for ms in msgs.iter() {
                    for m in ms.messages() {
                        println!("{}", String::from_utf8_lossy(m.value));
                    }
                    consumer.consume_messageset(ms).unwrap();
                }
                consumer.commit_consumed().unwrap();
            }
        }
        // let actor = MessageSubscriberSender::for_actor(&ld);
        // let handle = Handle::current();
        // if let Err(e) = handle.block_on(actor.handle_message(
        //     &Context::default(),
        //     &SubMessage {
        //         body: m.value.to_vec(),
        //         reply_to: None,
        //         subject: "my-topic".to_owned(),
        //     },
        // )) {
        //     eprintln!("Unable to send subscription: {:?}", e);
        // }
    });
    Ok(())
}
