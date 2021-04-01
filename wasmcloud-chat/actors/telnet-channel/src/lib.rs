extern crate wapc_guest as guest;
use guest::prelude::*;
use log::{error, info};
use messaging::BrokerMessage;
use msgactor::{ChannelMessage, ProcessAck};
use serde::{Deserialize, Serialize};
use wasmcloud_actor_core as actor;
use wasmcloud_actor_extras as extras;
use wasmcloud_actor_logging as logging;
use wasmcloud_actor_messaging as messaging;
use wasmcloud_actor_telnet as telnet;
use wasmcloudchat_messages_interface as msgactor;

const MSG_ACTOR_CALL_ALIAS: &str = "wasmcloud/chat/messages";
const OP_PROCESS_MESSAGE: &str = "ProcessMessage";
const CHANNEL_ID: &str = "telnetbroker";
const MSGTYPE_MESSAGE: &str = "MSG";

#[actor::init]
fn init() {
    messaging::Handlers::register_handle_message(handle_message);
    telnet::Handlers::register_session_started(session_started);
    telnet::Handlers::register_receive_text(receive_text);
    logging::enable_macros();
}

//TODO: deliver message to respective client
fn handle_message(msg: BrokerMessage) -> HandlerResult<()> {
    info!(
        "Received Event from Backend:\n{:?}",
        String::from_utf8_lossy(&msg.body)
    );
    Ok(())
}

//TODO: this happens when you connect for the first time. good for debug and error handling?
fn session_started(msg: String) -> HandlerResult<bool> {
    info!("telnet session started: {}", msg);
    Ok(true)
}

fn receive_text(session: String, message_text: String) -> HandlerResult<bool> {
    info!("text ({}) received on session ({})", message_text, session);
    // TODO: validate access token
    let new_guid = extras::default()
        .request_guid()?
        .unwrap_or("???".to_string());
    let channelmsg = ChannelMessage {
        message_id: new_guid,
        message_type: MSGTYPE_MESSAGE.to_string(),
        origin_channel: CHANNEL_ID.to_string(),
        message_text,
        data: None,
        created_on: 0,                                //TODO: find appropriate value
        origin_user_id: "".to_string(),               //find appropriate value
        origin_room: Some("telnet_room".to_string()), //find appropriate value
        target_url: "wcchat://rooms/testroom".to_string(), //find appropriate value
    };
    info!("Submitting message to message actor for processing");
    match actor::call_actor::<ChannelMessage, ProcessAck>(
        MSG_ACTOR_CALL_ALIAS,
        OP_PROCESS_MESSAGE,
        &channelmsg,
    ) {
        Ok(_) => Ok(true),
        Err(e) => {
            error!("Failed to process message: {}", e);
            Err("Failed to process message".into())
        }
    }
}

/// Repesents a cloudevent v1.0 JSON format message.
/// for ease of use, copy this struct into any library that needs
/// to round-trip these structures. Don't create the unnecessary headache
/// of creating coupling just for the use of cloud events.
#[derive(Serialize, Deserialize, PartialEq, Clone, Default, Debug)]
pub struct CloudEvent {
    #[serde(rename = "specversion")]
    pub cloud_events_version: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub source: String, // URI
    #[serde(rename = "id")]
    pub event_id: String,
    #[serde(rename = "time")]
    pub event_time: String,
    #[serde(rename = "datacontenttype")]
    pub content_type: String,
    pub data: serde_json::Value,
}
