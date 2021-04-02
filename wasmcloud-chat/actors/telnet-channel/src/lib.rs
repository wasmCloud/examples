extern crate wapc_guest as guest;
use guest::prelude::*;
use log::{debug, error, info};
use messaging::BrokerMessage;
use msgactor::{ChannelMessage, ProcessAck};
use serde::{Deserialize, Serialize};
use wasmcloud_actor_core as actor;
use wasmcloud_actor_extras as extras;
use wasmcloud_actor_keyvalue as keyvalue;
use wasmcloud_actor_logging as logging;
use wasmcloud_actor_messaging as messaging;
use wasmcloud_actor_telnet as telnet;
use wasmcloudchat_messages_interface as msgactor;

const MSG_ACTOR_CALL_ALIAS: &str = "wasmcloud/chat/messages";
const OP_PROCESS_MESSAGE: &str = "ProcessMessage";
const MSGTYPE_MESSAGE: &str = "MSG";

const CHANNEL_ID: &str = "telnetbroker";
const KEYVALUE_LINK: &str = "telnetsession";

const MSG_LINK: &str = "frontend";
const SESSIONS_KEY: &str = "wcc:telnet:sessions";

const INIT_SUBJECT: &str = "wcc.telnet.init";
const USER_PREFIX: &str = "wcchat://users/";

#[actor::init]
fn init() {
    messaging::Handlers::register_handle_message(handle_message);
    telnet::Handlers::register_session_started(session_started);
    telnet::Handlers::register_receive_text(receive_text);
    logging::enable_macros();
}

fn handle_message(msg: BrokerMessage) -> HandlerResult<()> {
    if msg.subject == INIT_SUBJECT {
        init_session(msg)
    } else {
        backend_message(msg)
    }
}

/// Handles initialization of a user session, including setting
/// username, session_id
fn init_session(msg: BrokerMessage) -> HandlerResult<()> {
    //TODO: validate username, no dupes, etc
    let session = serde_json::from_slice::<UserSession>(&msg.body)?;
    let session_host = keyvalue::host(KEYVALUE_LINK);
    if let Ok(_) = session_host
        .set_add(SESSIONS_KEY.to_string(), serde_json::to_string(&session)?)
        .map(|_| ())
    {
        info!("User {} has joined the telnet channel", session.username);
        telnet::default().send_text(
            session.id,
            "Welcome to wasmcloud chat\r\nType /help at any time to see a list of commands\r\n> "
                .to_string(),
        )?;
    }
    Ok(())
}

/// Forwards backend messages to frontend telnet clients
fn backend_message(msg: BrokerMessage) -> HandlerResult<()> {
    // Convert message bytes to CloudEvent
    let cloud_event = match serde_json::from_slice::<CloudEvent>(&msg.body) {
        Ok(v) => v,
        Err(e) => {
            error!("{}", &format!("{:?}", e));
            return Err(Box::new(e));
        }
    };
    // Extract channel message from CloudEvent
    let channel_msg = match serde_json::from_value::<ChannelMessage>(cloud_event.data) {
        Ok(m) => m,
        Err(e) => {
            error!("{}", &format!("{:?}", e));
            return Err(Box::new(e));
        }
    };
    // Get target session_id and send message if exists
    if let Some(target) = keyvalue::host(KEYVALUE_LINK)
        .set_query(SESSIONS_KEY.to_string())?
        .values
        .iter()
        .filter_map(|v| serde_json::from_str::<UserSession>(&v).ok())
        .filter(|us| us.username == channel_msg.target_url)
        .collect::<Vec<_>>()
        .get(0)
    {
        telnet::default()
            .send_text(
                target.id.to_string(),
                format!("INCOMING: {}\r\n", channel_msg.message_text),
            )
            .map(|_| ())
    } else {
        Ok(())
    }
}

/// Prompts user to create a username when session begins
/// TODO: consider assigning usernames, or basing them off of authentication
fn session_started(session_id: String) -> HandlerResult<bool> {
    debug!("telnet session started: {}", session_id);
    telnet::default().send_text(session_id, "Create a username: ".to_string())?;
    Ok(true)
}

/// Handle message from telnet user
///
/// Text starting with '/' are interpreted as commands, otherwise interpreted
/// as an outbound message
///
/// Dispatch message to messaging actor, which then forwards the message
/// to the applicable users / rooms
fn receive_text(session: String, message_text: String) -> HandlerResult<bool> {
    // Retrieve session for origin user, if it doesn't exist then
    // return early and send init message
    let origin_session = if let Some(session) = user_session_for_id(session.clone()) {
        session
    } else {
        info!("Initializing user for session {}", session);
        let _ = messaging::host(MSG_LINK)
            .publish(
                INIT_SUBJECT.to_string(),
                "".to_string(),
                serde_json::to_vec(&UserSession::new(
                    session,
                    format!("{}{}", USER_PREFIX, message_text.clone()),
                ))
                .unwrap(),
            )
            .map(|_| true);
        return Ok(false);
    };

    // Handle telnet command instead of sending message
    if message_text.starts_with('/') {
        handle_telnet_command(message_text, origin_session)
    } else {
        send_telnet_message(message_text, origin_session)
    }
}

fn send_telnet_message(message_text: String, origin_session: UserSession) -> HandlerResult<bool> {
    debug!(
        "text ({}) received from session ({})",
        message_text, origin_session.id
    );

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
        origin_user_id: origin_session.username,
        created_on: 0,                                  //TODO: find appropriate value
        origin_room: Some("telnet_room".to_string()),   //find appropriate value
        target_url: "wcchat://users/bobby".to_string(), //find appropriate value
    };
    info!("Submitting message to message actor for processing");
    // Replace prompt in user side TODO: Is this possible in another way?
    telnet::default().send_text(origin_session.id.clone(), "> ".to_string())?;
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

// Constants for telnet commands
const HELP_CMD: &str = "/help";
const HELP_TEXT: &str = "Commands:\r
/help      - Show this help text\r
/listusers - Show all connected telnet users\r\n";

fn handle_telnet_command(command: String, origin_session: UserSession) -> HandlerResult<bool> {
    debug!(
        "command {} received from user {}",
        command, origin_session.username
    );
    let telnet_host = telnet::default();
    let res = match &*command {
        HELP_CMD => telnet_host.send_text(origin_session.id.clone(), HELP_TEXT.to_string()),
        _ => Ok(false),
    };

    // Replace prompt in user side TODO: Is this possible in another way?
    telnet::default().send_text(origin_session.id, "> ".to_string())?;
    res
}

#[derive(Serialize, Deserialize, Clone)]
/// Associates a username and telnet session, stored in keyvalue
/// database for persistent sessions
struct UserSession {
    id: String,
    username: String,
}

impl UserSession {
    fn new(id: String, username: String) -> Self {
        UserSession { id, username }
    }
}

/// Examines keyvalue set for session id and returns
/// corresponding user session
fn user_session_for_id(session_id: String) -> Option<UserSession> {
    if let Ok(sessions) = keyvalue::host(KEYVALUE_LINK).set_query(SESSIONS_KEY.to_string()) {
        sessions
            .values
            .iter()
            .filter_map(|v| serde_json::from_str::<UserSession>(v).ok())
            .filter(|us| us.id == session_id)
            .collect::<Vec<_>>()
            .get(0)
            .cloned()
    } else {
        None
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
