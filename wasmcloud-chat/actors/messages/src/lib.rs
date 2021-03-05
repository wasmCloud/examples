extern crate serde;

extern crate wapc_guest as guest;
extern crate wasmcloud_actor_core as actor;
extern crate wasmcloud_actor_eventstreams as streams;
extern crate wasmcloud_actor_messaging as broker;
extern crate wasmcloudchat_messages_interface as messages;

use chrono::prelude::*;
use guest::prelude::*;
use messages::{ChannelMessage, ProcessAck};
use std::collections::HashMap;
use url::Url;

mod cloudevent;

use cloudevent::CloudEvent;

const WCCHAT_SCHEME: &str = "wcchat";
const EVENT_TYPE_MESSAGE_PUBLISHED: &str = "com.wasmcloud.chat.events.messagepublished";
const EVENT_SOURCE: &str = "/actors/messaging";

const HMK_ORIGIN_CHANNEL: &str = "origin_channel";
const HMK_ORIGIN_USER: &str = "origin_user";
const HMK_TIMESTAMP: &str = "timestamp";
const HMK_MESSAGE_TEXT: &str = "messagetext";
const HMK_ID: &str = "message_id";

const MSGTYPE_MESSAGE: &str = "MSG";
const VALID_MESSAGE_TYPES: [&str; 1] = [MSGTYPE_MESSAGE];

#[actor::init]
fn init() {
    messages::Handlers::register_process_message(process_message);
}

// Processing a message:
// 1 - validate the message
// 2 - publish it to the appropriate message broker subject
// 3 - emit event to event stream
// 4 - ACK
fn process_message(message: ChannelMessage) -> HandlerResult<ProcessAck> {
    let target = MessageTarget::from(&message);
    if let MessageTarget::Unknown(message) = target {
        return Err(format!("Unable to select target for message: {}", message).into());
    }

    Ok(validate_message(&message)
        .and_then(|_| publish_broker_message(&target, &message))
        .and_then(|_| emit_stream_event(&target, &message))
        .map_or_else(
            |_| ProcessAck::success(&message.message_id),
            |_| {
                ProcessAck::fail(
                    &message.message_id,
                    "Did not publish and emit to event stream",
                )
            },
        ))
}

fn validate_message(message: &messages::ChannelMessage) -> HandlerResult<()> {
    if message.message_id.is_empty() {
        return Err("No message ID".into());
    }
    if message.target_url == format!("wcchat://users/{}", message.origin_user_id) {
        return Err("Cannot send messages to yourself".into());
    }
    if message.message_text.is_empty() {
        return Err("Cannot send empty messages".into());
    }
    if !VALID_MESSAGE_TYPES
        .to_vec()
        .contains(&message.message_type.as_str())
    {
        return Err("Invalid/unrecognized message type".into());
    }
    Ok(())
}

fn publish_broker_message(
    target: &MessageTarget,
    message: &messages::ChannelMessage,
) -> HandlerResult<()> {
    let topic = match target {
        MessageTarget::User(s) => format!("wcc.events.user.{}", s),
        MessageTarget::Room(s) => format!("wcc.events.room.{}", s),
        _ => "".to_string(),
    };
    let ce = CloudEvent::new_json(
        &message.message_id,
        EVENT_TYPE_MESSAGE_PUBLISHED,
        EVENT_SOURCE,
        message.created_on,
        &message,
    );

    broker::default().publish(topic, "".to_string(), serde_json::to_vec(&ce)?)?;
    Ok(())
}

fn emit_stream_event(
    target: &MessageTarget,
    message: &messages::ChannelMessage,
) -> HandlerResult<()> {
    let message = message.clone();

    let stream_name = match target {
        MessageTarget::User(s) => format!("user-{}", s),
        MessageTarget::Room(s) => format!("room-{}", s),
        _ => "".to_string(),
    };

    let mut hm = HashMap::new();
    hm.insert(HMK_ORIGIN_CHANNEL.to_string(), message.origin_channel);
    hm.insert(HMK_ORIGIN_USER.to_string(), message.origin_user_id);
    hm.insert(
        HMK_TIMESTAMP.to_string(),
        Utc.timestamp(message.created_on as i64, 0).to_rfc2822(),
    );
    hm.insert(HMK_MESSAGE_TEXT.to_string(), message.message_text);
    hm.insert(HMK_ID.to_string(), message.message_id);

    streams::default().write_event(stream_name, hm)?;

    Ok(())
}

// target URLS:
// wcchat://rooms/(room_id)
// wcchat://users/(user_id)

enum MessageTarget {
    User(String),
    Room(String),
    Unknown(String),
}

impl From<&messages::ChannelMessage> for MessageTarget {
    fn from(source: &messages::ChannelMessage) -> MessageTarget {
        Url::parse(&source.target_url).map_or(
            MessageTarget::Unknown("Unparseable URL".to_string()),
            |url| extract_target(&url),
        )
    }
}

fn extract_target(url: &Url) -> MessageTarget {
    if url.scheme() != WCCHAT_SCHEME {
        return MessageTarget::Unknown("Unrecognized target URL scheme".to_string());
    }

    if url.path_segments().is_none() {
        return MessageTarget::Unknown("No path segments in URL".to_string());
    }
    let mut path_segments = url.path_segments().unwrap();

    match path_segments.next() {
        Some(qualifier) if qualifier == "rooms" => match path_segments.next() {
            Some(s) => MessageTarget::Room(s.to_string()),
            None => MessageTarget::Unknown("No room specified in room target URL".to_string()),
        },
        Some(qualifier) if qualifier == "users" => match path_segments.next() {
            Some(s) => MessageTarget::User(s.to_string()),
            None => MessageTarget::Unknown("No user specified in user target URL".to_string()),
        },
        Some(qualifier) => {
            MessageTarget::Unknown(format!("Unknown target qualifier: {}", qualifier).into())
        }
        None => MessageTarget::Unknown("No qualifier specified in target URL".to_string()),
    }
}
