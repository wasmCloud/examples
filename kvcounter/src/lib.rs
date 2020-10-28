#[macro_use]
extern crate serde_json;

extern crate wapc_guest as guest;
use actor_core as actorcore;
use actor_http_server as http;
use actor_keyvalue as kv;
use guest::prelude::*;

#[no_mangle]
pub fn wapc_init() {
    actorcore::Handlers::register_health_request(health);
    http::Handlers::register_handle_request(increment_counter);
}

fn increment_counter(msg: http::Request) -> HandlerResult<http::Response> {
    let key = msg.path.replace('/', ":");
    let resp = kv::default().add(key, 1)?;

    let result = json!({"counter": resp.value });
    Ok(http::Response::json(&result, 200, "OK"))
}

fn health(_h: actorcore::HealthCheckRequest) -> HandlerResult<actorcore::HealthCheckResponse> {
    Ok(actorcore::HealthCheckResponse::healthy())
}
