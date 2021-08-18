use serde::{Deserialize, Serialize};
use wapc_guest::prelude::*;
use wasmcloud_actor_keyvalue as keyvalue;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub(crate) struct RoomData {
    pub id: String,
    pub description: String,
}

pub(crate) fn add_room(id: String, description: String) -> HandlerResult<()> {
    let key = format!("rooms:{}", id);
    let val = serde_json::to_string(&RoomData {
        id: id.to_string(),
        description,
    })?;
    let _ = keyvalue::default().set(key.to_string(), val, 0)?;

    let _ = keyvalue::default().set_add("rooms".to_string(), id)?;
    Ok(())
}

pub(crate) fn remove_room(id: String) -> HandlerResult<()> {
    let key = format!("rooms:{}", id);
    let _ = keyvalue::default().del(key)?;
    let _ = keyvalue::default().set_remove("rooms".into(), id);
    Ok(())
}

pub(crate) fn join_room(room_id: String, member_id: String) -> HandlerResult<()> {
    let key = format!("rooms:{}:members", room_id);
    let _ = keyvalue::default().set_add(key, member_id)?;
    Ok(())
}

pub(crate) fn leave_room(room_id: String, member_id: String) -> HandlerResult<()> {
    let key = format!("rooms:{}:members", room_id);
    let _ = keyvalue::default().set_remove(key, member_id)?;
    Ok(())
}

pub(crate) fn get_room_members(room_id: String) -> HandlerResult<Vec<String>> {
    let key = format!("rooms:{}:members", room_id);
    Ok(keyvalue::default()
        .set_query(key)?
        .values
        .into_iter()
        .collect())
}

pub(crate) fn get_rooms() -> HandlerResult<Vec<RoomData>> {
    Ok(keyvalue::default()
        .set_query("rooms".to_string())?
        .values
        .into_iter()
        .flat_map(|s| serde_json::from_str(&s))
        .collect())
}
