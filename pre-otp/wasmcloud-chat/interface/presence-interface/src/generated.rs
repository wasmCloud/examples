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
    pub fn register_track(f: fn(PresenceTrackReport) -> HandlerResult<()>) {
        *TRACK.write().unwrap() = Some(f);
        register_function(&"Track", track_wrapper);
    }
    pub fn register_query_online(f: fn() -> HandlerResult<OnlineUserList>) {
        *QUERY_ONLINE.write().unwrap() = Some(f);
        register_function(&"QueryOnline", query_online_wrapper);
    }
}

lazy_static! {
    static ref TRACK: RwLock<Option<fn(PresenceTrackReport) -> HandlerResult<()>>> =
        RwLock::new(None);
    static ref QUERY_ONLINE: RwLock<Option<fn() -> HandlerResult<OnlineUserList>>> =
        RwLock::new(None);
}

fn track_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<PresenceTrackReport>(input_payload)?;
    let lock = TRACK.read().unwrap().unwrap();
    let result = lock(input)?;
    Ok(serialize(result)?)
}

fn query_online_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<QueryOnlineArgs>(input_payload)?;
    let lock = QUERY_ONLINE.read().unwrap().unwrap();
    let result = lock()?;
    Ok(serialize(result)?)
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct QueryOnlineArgs {}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct PresenceTrackReport {
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "channel_id")]
    pub channel_id: String,
    #[serde(rename = "last_activity")]
    pub last_activity: u64,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct OnlineUserList {
    #[serde(rename = "users")]
    pub users: Vec<PresenceUser>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct PresenceUser {
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "channels")]
    pub channels: Vec<String>,
    #[serde(rename = "last_seen_mins")]
    pub last_seen_mins: u64,
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
