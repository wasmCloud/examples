#[macro_use]
extern crate serde_json;

extern crate wapc_guest as guest;
use wasmcloud_actor_core as actor;
use wasmcloud_actor_http_server as http;
use wasmcloud_actor_keyvalue as kv;
use guest::prelude::*;

#[actor::init]
pub fn init() {
    http::Handlers::register_handle_request(increment_counter);
}

fn increment_counter(msg: http::Request) -> HandlerResult<http::Response> {
    let key = msg.path.replace('/', ":");
    let resp = kv::default().add(key, 1)?;

    let result = json!({"counter": resp.value });
    Ok(http::Response::json(&result, 200, "OK"))
}

