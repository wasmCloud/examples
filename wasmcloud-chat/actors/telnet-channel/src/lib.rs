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
const ROOM_PREFIX: &str = "wcchat://rooms/";
// Constants for telnet commands
const HELP_CMD: &str = "/help";
const LEAVE_CMD: &str = "/leave";
const JOIN_CMD: &str = "/join";
const MSG_CMD: &str = "/msg";
const ROOMS_CMD: &str = "/rooms";
const WHO_CMD: &str = "/who";
const HELP_TEXT: &str = "Commands:\r
    /help               - Show this help text\r
    /join <room>        - Join a room\r
    /leave <room>       - Leave a room\r
    /msg <user> <text>  - Send a direct message to a user\r
    /rooms              - Show the list of rooms\r
    /who                - Show all online users\r\n";

#[actor::init]
fn init() {
    messaging::Handlers::register_handle_message(handle_message);
    telnet::Handlers::register_session_started(session_started);
    telnet::Handlers::register_receive_text(receive_text);
    logging::enable_macros();
}

fn handle_message(msg: BrokerMessage) -> HandlerResult<()> {
    //TODO: consider replacing message with user command
    if msg.subject == INIT_SUBJECT {
        init_session(msg)
    } else {
        backend_message(msg)
    }
}

/// Handles initialization of a user session, including setting
/// username, session_id
fn init_session(msg: BrokerMessage) -> HandlerResult<()> {
    //TODO: validate username, update session for user
    let session = serde_json::from_slice::<UserSession>(&msg.body)?;
    let session_host = keyvalue::host(KEYVALUE_LINK);
    // If user already exists, remove their session and replace with updated session
    if let Some(user_session) = user_session_for_id(session.id.clone()) {
        session_host.set_remove(
            SESSIONS_KEY.to_string(),
            serde_json::to_string(&user_session)?,
        )?;
    }
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
        //TODO: This delivers the message immediately, might be inline with what the user is currently typing
        telnet::default()
            .send_text(
                target.id.to_string(),
                format!(
                    "{}: {}\r\n> ",
                    channel_msg.origin_user_id, channel_msg.message_text
                ),
            )
            .map(|_| ())
    } else {
        Ok(())
    }
}

/// Prompts user to create a username when session begins
/// TODO: consider assigning usernames, or basing them off of authentication
/// OR command to create a user
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

    handle_telnet_command(&message_text, origin_session)
}

/// Parse and handle command sent by user
fn handle_telnet_command(command: &str, origin_session: UserSession) -> HandlerResult<bool> {
    debug!(
        "command {} received from user {}",
        command, origin_session.username
    );
    let telnet_host = telnet::default();
    let res = match command {
        HELP_CMD => telnet_host.send_text(origin_session.id.clone(), HELP_TEXT.to_string()),
        ROOMS_CMD => Ok(true),
        WHO_CMD => Ok(true),
        _ if command.starts_with(MSG_CMD) => {
            //TODO: catch unwraps here
            let cmd = command.strip_prefix(MSG_CMD).unwrap().trim();
            let user = &cmd[0..cmd.find(char::is_whitespace).unwrap()];
            let msg = cmd.strip_prefix(user).unwrap().trim();
            send_telnet_message(&user_target(user), msg, origin_session.clone())
        }
        _ => telnet_host.send_text(
            origin_session.id.clone(),
            format!(
                "Command \"{}\" not supported\r\n",
                command.split_ascii_whitespace().next().unwrap_or_default()
            ),
        ),
    };

    // Replace prompt in user side TODO: Is this possible in another way?
    telnet::default().send_text(origin_session.id, "> ".to_string())?;
    res
}

fn send_telnet_message(
    target_url: &str,
    message_text: &str,
    origin_session: UserSession,
) -> HandlerResult<bool> {
    debug!(
        "text ({}) received from session ({})",
        message_text, origin_session.id
    );

    let channelmsg = new_channel_message(
        message_text,
        &origin_session.username,
        Some("telnet_room".to_string()),
        target_url,
    )?;

    info!("Submitting message to message actor for processing");
    // Replace prompt in user side TODO: Is this possible in another way?
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

//TODO: Alter something like 'focus', or query presence actor?
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

/// Helper function to create ChannelMessage struct
fn new_channel_message(
    message_text: &str,
    origin_user_id: &str,
    origin_room: Option<String>,
    target_url: &str,
) -> Result<ChannelMessage, Box<dyn ::std::error::Error + Send + Sync>> {
    let new_guid = extras::default()
        .request_guid()?
        .unwrap_or("???".to_string());
    Ok(ChannelMessage {
        message_id: new_guid,
        message_type: MSGTYPE_MESSAGE.to_string(),
        origin_channel: CHANNEL_ID.to_string(),
        message_text: message_text.to_string(),
        data: None,
        origin_user_id: origin_user_id.to_string(),
        created_on: 0, //TODO: find appropriate value
        origin_room,
        target_url: target_url.to_string(),
    })
}

/// Helper function to create target_url for a user
fn user_target(user: &str) -> String {
    format!("{}{}", USER_PREFIX, user)
}

/// Helper function to create target_url for a room
fn room_target(room: &str) -> String {
    format!("{}{}", ROOM_PREFIX, room)
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
