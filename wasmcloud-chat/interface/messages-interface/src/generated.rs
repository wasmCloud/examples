extern crate rmp_serde as rmps;
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

extern crate log;
extern crate wapc_guest as guest;
use guest::prelude::*;

use lazy_static::lazy_static;
use std::sync::RwLock;


pub struct Handlers {}

impl Handlers {
    pub fn register_process_message(f: fn(ChannelMessage) -> HandlerResult<ProcessAck>) {
        *PROCESS_MESSAGE.write().unwrap() = Some(f);
        register_function(&"ProcessMessage", process_message_wrapper);
    }
}

lazy_static! {
    static ref PROCESS_MESSAGE: RwLock<Option<fn(ChannelMessage) -> HandlerResult<ProcessAck>>> =
        RwLock::new(None);
}

fn process_message_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<ChannelMessage>(input_payload)?;
    let lock = PROCESS_MESSAGE.read().unwrap().unwrap();
    let result = lock(input)?;
    Ok(serialize(result)?)
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct ChannelMessage {
    #[serde(rename = "message_id")]
    pub message_id: String,
    #[serde(rename = "origin_channel")]
    pub origin_channel: String,
    #[serde(rename = "origin_user_id")]
    pub origin_user_id: String,
    #[serde(rename = "origin_room")]
    pub origin_room: Option<String>,
    #[serde(rename = "message_text")]
    pub message_text: String,
    #[serde(rename = "created_on")]
    pub created_on: u64,
    #[serde(rename = "target_url")]
    pub target_url: String,
    #[serde(rename = "message_type")]
    pub message_type: String,
    #[serde(rename = "data")]
    pub data: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct ProcessAck {
    #[serde(rename = "message_id")]
    pub message_id: String,
    #[serde(rename = "processed")]
    pub processed: bool,
    #[serde(rename = "error")]
    pub error: Option<String>,
}

/// The standard function for serializing codec structs into a format that can be
/// used for message exchange between actor and host. Use of any other function to
/// serialize could result in breaking incompatibilities.
pub fn serialize<T>(
    item: T,
) -> ::std::result::Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>
where
    T: Serialize,
{
    let mut buf = Vec::new();
    item.serialize(&mut Serializer::new(&mut buf).with_struct_map())?;
    Ok(buf)
}

/// The standard function for de-serializing codec structs from a format suitable
/// for message exchange between actor and host. Use of any other function to
/// deserialize could result in breaking incompatibilities.
pub fn deserialize<'de, T: Deserialize<'de>>(
    buf: &[u8],
) -> ::std::result::Result<T, Box<dyn std::error::Error + Send + Sync>> {
    let mut de = Deserializer::new(Cursor::new(buf));
    match Deserialize::deserialize(&mut de) {
        Ok(t) => Ok(t),
        Err(e) => Err(format!("Failed to de-serialize: {}", e).into()),
    }
}
