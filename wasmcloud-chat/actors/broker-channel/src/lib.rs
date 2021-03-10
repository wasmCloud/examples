#[macro_use]
extern crate log;

extern crate serde_json;

use serde::{Deserialize, Serialize};
use wapc_guest as guest;
use wasmcloud_actor_core as actor;
use wasmcloud_actor_extras as extras;
use wasmcloud_actor_messaging as messaging;
use wasmcloud_actor_logging as logging;
use wasmcloudchat_messages_interface as msgactor;

use guest::prelude::*;

const INBOUND_SUBJECT: &str = "wcc.frontend.requests";
const BACKEND_SUBJECT_PREFIX: &str = "wcc.backend.events.";
const FRONTEND_SUBJECT_PREFIX: &str = "wcc.frontend.events.";
const MSG_ACTOR_CALL_ALIAS: &str = "wasmcloud/chat/messages";
const OP_PROCESS_MESSAGE: &str = "ProcessMessage";

const CHANNEL_ID: &str = "messagebroker";
const MSGTYPE_MESSAGE: &str = "MSG";

const LINK_NAME_FRONTEND: &str = "frontend";
const _LINK_NAME_BACKEND: &str = "backend";

#[actor::init]
fn init() {
    messaging::Handlers::register_handle_message(handle_message);

    logging::enable_macros();
}

fn handle_message(msg: messaging::BrokerMessage) -> HandlerResult<()> {
    // TODO: handle request to obtain access token

    if msg.subject == INBOUND_SUBJECT {
        handle_inbound_message(msg)
    } else if msg.subject.starts_with(BACKEND_SUBJECT_PREFIX) {
        handle_outbound_message(msg)
    } else {
        Err(format!("Unrecognized subject {}", msg.subject).into())
    }
}

/// Validates that the inbound request has a legitimate security authorization to
/// publish messages.
/// Takes an `InboundRequest` message delivered from the `frontend` link binding
/// and converts that to a `ChannelMessage`, to be used in directly invoking
/// the `messages` actor's `ProcessMessage` operation. Responds to the front-end
/// with a copy of the acknowledgement received from the internal actor.
fn handle_inbound_message(msg: messaging::BrokerMessage) -> HandlerResult<()> {
    info!("Handling inbound message request");
    // TODO: validate access token
    let req: InboundRequest = serde_json::from_slice(&msg.body)?;
    let new_guid = extras::default()
        .request_guid()?
        .unwrap_or("???".to_string());
    let channelmsg = msgactor::ChannelMessage {
        created_on: req.created_on,
        message_id: new_guid,
        data: None,
        origin_channel: CHANNEL_ID.to_string(),
        origin_user_id: req.origin_user_id,
        origin_room: req.origin_room,
        target_url: req.target_url,
        message_type: MSGTYPE_MESSAGE.to_string(),
        message_text: req.message_text,
    };
    info!("Submitting message to message actor for processing");
    let ack: msgactor::ProcessAck =
        match actor::call_actor(MSG_ACTOR_CALL_ALIAS, OP_PROCESS_MESSAGE, &channelmsg) {
            Ok(ack) => ack,
            Err(e) => {
                error!("Failed to process message: {}", e);
                return Err("Failed to process message".into());
            }
        };
    let response = InboundResponse {
        acknowledged: ack.processed,
        error: ack.error,
        message_id: ack.message_id,
    }; 
    
    if !msg.reply_to.is_empty() {
        let _ = messaging::host(LINK_NAME_FRONTEND).publish(
            msg.reply_to,
            "".to_string(),
            serde_json::to_vec(&response)?,
        )?;
    }
    Ok(())
}

/// Takes a CloudEvent received from the back end and simply forwards it to the front-end
/// by publishing it on the `frontend` link binding and replacing the subject prefix
fn handle_outbound_message(msg: messaging::BrokerMessage) -> HandlerResult<()> {
    let cloud_event: CloudEvent = serde_json::from_slice(&msg.body)?;
    let subject = msg
        .subject
        .replace(BACKEND_SUBJECT_PREFIX, FRONTEND_SUBJECT_PREFIX);
    let _ = messaging::host(LINK_NAME_FRONTEND).publish(
        subject,
        "".to_string(),
        serde_json::to_vec(&cloud_event)?,
    )?;
    Ok(())
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
struct InboundRequest {
    pub source_app_id: String,
    pub source_app_version: String,
    pub origin_room: Option<String>,
    pub message_text: String,
    pub origin_user_id: String,
    pub target_url: String,
    pub created_on: u64,
    pub access_token: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
struct InboundResponse {
    pub message_id: String,
    pub acknowledged: bool,
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Default)]
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
