#[macro_use]
extern crate log;

use rooms::*;
use wapc_guest as guest;
use wasmcloud_actor_core as actor;
use wasmcloud_actor_logging as logging;
use wasmcloudchat_rooms_interface as rooms;

use guest::prelude::*;

mod kv;

#[actor::init]
fn init() {
    Handlers::register_create_room(create_room);
    Handlers::register_delete_room(delete_room);
    Handlers::register_join_room(join_room);
    Handlers::register_leave_room(leave_room);
    Handlers::register_query_members(query_members);
    Handlers::register_query_rooms(query_rooms);

    logging::enable_macros();
}

fn create_room(id: String, description: String) -> HandlerResult<RoomAck> {
    info!("Creating room {}", id);
    Ok(kv::add_room(id, description).map_or_else(
        |e| RoomAck::fail(&format!("{}", e)),
        |_v| RoomAck::success(),
    ))
}

fn delete_room(id: String) -> HandlerResult<RoomAck> {
    info!("Deleting room {}", id);
    Ok(kv::remove_room(id)
        .map_or_else(|e| RoomAck::fail(&format!("{}", e)), |_| RoomAck::success()))
}

fn join_room(room_id: String, member_id: String) -> HandlerResult<RoomAck> {
    info!("Adding member {} to room {}", member_id, room_id);
    Ok(kv::join_room(room_id, member_id)
        .map_or_else(|e| RoomAck::fail(&format!("{}", e)), |_| RoomAck::success()))
}

fn leave_room(room_id: String, member_id: String) -> HandlerResult<RoomAck> {
    info!("Removing member {} from room {}", member_id, room_id);
    Ok(kv::leave_room(room_id, member_id)
        .map_or_else(|e| RoomAck::fail(&format!("{}", e)), |_| RoomAck::success()))
}

fn query_members(room_id: String) -> HandlerResult<MemberList> {
    info!("Querying room members {}", room_id);
    Ok(MemberList {
        members: kv::get_room_members(room_id)?
            .iter()
            .map(|s| Member { id: s.to_string() })
            .collect(),
    })
}

fn query_rooms() -> HandlerResult<RoomList> {
    info!("Querying list of all rooms");
    Ok(RoomList {
        rooms: kv::get_rooms()?
            .iter()
            .map(|r| Room {
                id: r.id.to_string(),
                description: r.description.to_string(),
                member_count: kv::get_room_members(r.id.to_string())
                    .unwrap_or(vec![])
                    .len() as _,
            })
            .collect(),
    })
}
