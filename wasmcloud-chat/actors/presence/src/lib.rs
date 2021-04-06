#[macro_use]
extern crate log;

use guest::prelude::*;

use presence::*;
use wapc_guest as guest;
use wasmcloud_actor_core as actor;
use wasmcloud_actor_logging as logging;
use wasmcloudchat_presence_interface as presence;

mod kv;

#[actor::init]
fn init() {
    // Use this message to query the list of all users who have contributed a tracked activity
    // within the last `n` minutes
    Handlers::register_query_online(query_online);
    // Send this actor a track message whenever a user does something (e.g. sends a message) or
    // when a heartbeat ticks for a given user on a given channel
    Handlers::register_track(track);

    logging::enable_macros();
}

fn query_online() -> HandlerResult<OnlineUserList> {
    info!("Handling request to query online users");
    Ok(kv::get_online_users()?)
}

fn track(report: PresenceTrackReport) -> HandlerResult<()> {
    info!("Recording presence tracking report");
    Ok(kv::store_tracking_report(report)?)
}
