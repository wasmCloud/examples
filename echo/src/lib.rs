use serde::Serialize;
use std::collections::HashMap;
use wasmcloud_actor_core as actor;
use wasmcloud_actor_http_server as http;

use wapc_guest::prelude::*;

#[derive(Serialize)]
struct EchoResponse {
    method: String,
    path: String,
    query_string: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

pub fn handle_request(r: http::Request) -> HandlerResult<http::Response> {
    let echo = EchoResponse {
        method: r.method,
        path: r.path,
        query_string: r.query_string,
        headers: r.header,
        body: r.body,
    };

    Ok(http::Response::json(echo, 200, "OK"))
}

fn health(_h: actor::HealthCheckRequest) -> HandlerResult<actor::HealthCheckResponse> {
    Ok(actor::HealthCheckResponse::healthy())
}

#[no_mangle]
pub fn wapc_init() {
    actor::Handlers::register_health_request(health);
    http::Handlers::register_handle_request(handle_request);
}
