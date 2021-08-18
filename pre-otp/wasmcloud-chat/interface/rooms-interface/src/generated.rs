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
    pub fn register_create_room(f: fn(String, String) -> HandlerResult<RoomAck>) {
        *CREATE_ROOM.write().unwrap() = Some(f);
        register_function(&"CreateRoom", create_room_wrapper);
    }
    pub fn register_query_rooms(f: fn() -> HandlerResult<RoomList>) {
        *QUERY_ROOMS.write().unwrap() = Some(f);
        register_function(&"QueryRooms", query_rooms_wrapper);
    }
    pub fn register_delete_room(f: fn(String) -> HandlerResult<RoomAck>) {
        *DELETE_ROOM.write().unwrap() = Some(f);
        register_function(&"DeleteRoom", delete_room_wrapper);
    }
    pub fn register_query_members(f: fn(String) -> HandlerResult<MemberList>) {
        *QUERY_MEMBERS.write().unwrap() = Some(f);
        register_function(&"QueryMembers", query_members_wrapper);
    }
    pub fn register_join_room(f: fn(String, String) -> HandlerResult<RoomAck>) {
        *JOIN_ROOM.write().unwrap() = Some(f);
        register_function(&"JoinRoom", join_room_wrapper);
    }
    pub fn register_leave_room(f: fn(String, String) -> HandlerResult<RoomAck>) {
        *LEAVE_ROOM.write().unwrap() = Some(f);
        register_function(&"LeaveRoom", leave_room_wrapper);
    }
}

lazy_static! {
    static ref CREATE_ROOM: RwLock<Option<fn(String, String) -> HandlerResult<RoomAck>>> =
        RwLock::new(None);
    static ref QUERY_ROOMS: RwLock<Option<fn() -> HandlerResult<RoomList>>> = RwLock::new(None);
    static ref DELETE_ROOM: RwLock<Option<fn(String) -> HandlerResult<RoomAck>>> =
        RwLock::new(None);
    static ref QUERY_MEMBERS: RwLock<Option<fn(String) -> HandlerResult<MemberList>>> =
        RwLock::new(None);
    static ref JOIN_ROOM: RwLock<Option<fn(String, String) -> HandlerResult<RoomAck>>> =
        RwLock::new(None);
    static ref LEAVE_ROOM: RwLock<Option<fn(String, String) -> HandlerResult<RoomAck>>> =
        RwLock::new(None);
}

fn create_room_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<CreateRoomArgs>(input_payload)?;
    let lock = CREATE_ROOM.read().unwrap().unwrap();
    let result = lock(input.id, input.description)?;
    Ok(serialize(result)?)
}

fn query_rooms_wrapper(input_payload: &[u8]) -> CallResult {
    let _input = deserialize::<QueryRoomsArgs>(input_payload)?;
    let lock = QUERY_ROOMS.read().unwrap().unwrap();
    let result = lock()?;
    Ok(serialize(result)?)
}

fn delete_room_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<DeleteRoomArgs>(input_payload)?;
    let lock = DELETE_ROOM.read().unwrap().unwrap();
    let result = lock(input.id)?;
    Ok(serialize(result)?)
}

fn query_members_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<QueryMembersArgs>(input_payload)?;
    let lock = QUERY_MEMBERS.read().unwrap().unwrap();
    let result = lock(input.id)?;
    Ok(serialize(result)?)
}

fn join_room_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<JoinRoomArgs>(input_payload)?;
    let lock = JOIN_ROOM.read().unwrap().unwrap();
    let result = lock(input.room_id, input.member_id)?;
    Ok(serialize(result)?)
}

fn leave_room_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<LeaveRoomArgs>(input_payload)?;
    let lock = LEAVE_ROOM.read().unwrap().unwrap();
    let result = lock(input.room_id, input.member_id)?;
    Ok(serialize(result)?)
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct CreateRoomArgs {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "description")]
    pub description: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct QueryRoomsArgs {}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct DeleteRoomArgs {
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct QueryMembersArgs {
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct JoinRoomArgs {
    #[serde(rename = "room_id")]
    pub room_id: String,
    #[serde(rename = "member_id")]
    pub member_id: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct LeaveRoomArgs {
    #[serde(rename = "room_id")]
    pub room_id: String,
    #[serde(rename = "member_id")]
    pub member_id: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct RoomAck {
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "error")]
    pub error: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct RoomList {
    #[serde(rename = "rooms")]
    pub rooms: Vec<Room>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct Room {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "member_count")]
    pub member_count: u32,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct MemberList {
    #[serde(rename = "members")]
    pub members: Vec<Member>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct Member {
    #[serde(rename = "id")]
    pub id: String,
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
