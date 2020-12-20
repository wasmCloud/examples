extern crate wascc_actor as actor;

use serde::Serialize;
use std::collections::HashMap;
use actor::prelude::*;

actor_handlers! { 
    codec::http::OP_HANDLE_REQUEST => echo, 
    codec::core::OP_HEALTH_REQUEST => health }

#[derive(Serialize)]
struct EchoResponse {
    method: String,
    path: String,
    query_string: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

pub fn echo(r:  codec::http::Request) ->  HandlerResult<codec::http::Response> {
    let echo = EchoResponse {
        method: r.method,
        path: r.path,
        query_string: r.query_string,
        headers: r.header,
        body: r.body,
    };

    Ok(codec::http::Response::json(echo, 200, "OK"))
}

fn health(_h: codec::core::HealthRequest) -> HandlerResult<()> {
    Ok(())
}