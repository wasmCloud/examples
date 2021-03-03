extern crate ping_interface as demo;
extern crate wapc_guest as guest;

use serde::{Deserialize, Serialize};
use wasmcloud_actor_core as actor;
use wasmcloud_actor_http_server as http;

use guest::prelude::*;

#[actor::init]
fn init() {
    // Register your message handlers here
    http::Handlers::register_handle_request(handle_request);
}

fn handle_request(_req: http::Request) -> HandlerResult<http::Response> {
    let p: demo::Pong = actor::call_actor(
        "wasmcloud/examples/ponger",
        "Ping",
        &demo::Ping { value: 11 },
    )?;
    Ok(http::Response::json(&p, 200, "OK"))
}
