extern crate wapc_guest as guest;
use ping_interface as demo;
use wasmcloud_actor_core as actor;

use guest::prelude::*;

#[actor::init]
fn init() {
    demo::Handlers::register_ping(handle_ping);
}

fn handle_ping(ping: demo::Ping) -> HandlerResult<demo::Pong> {
    Ok(demo::Pong {
        value: ping.value + 42,
    })
}
