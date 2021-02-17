use serde::Serialize;
use std::collections::HashMap;
extern crate wapc_guest as guest;
use guest::prelude::*;
use wasmcloud_actor_core as actorcore;
use wasmcloud_actor_http_server as http;

#[no_mangle]
pub fn wapc_init() {
    actorcore::Handlers::register_health_request(health);
    http::Handlers::register_handle_request(hello_world);
}

pub fn hello_world(r: http::Request) -> HandlerResult<http::Response> {
    let echo = EchoResponse {
        method: r.method,
        path: r.path,
        query_string: r.query_string,
        headers: r.header,
        body: r.body,
    };

    Ok(http::Response::json(echo, 200, "OK"))
}

fn health(_h: actorcore::HealthCheckRequest) -> HandlerResult<actorcore::HealthCheckResponse> {
    Ok(actorcore::HealthCheckResponse::healthy())
}

#[derive(Serialize)]
struct EchoResponse {
    method: String,
    path: String,
    query_string: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}
