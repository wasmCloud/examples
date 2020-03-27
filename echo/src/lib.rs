// Copyright 2015-2019 Capital One Services, LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate wascc_actor as actor;

use actor::prelude::*;
use serde::Serialize;
use std::collections::HashMap;

actor_handlers!{ http::OP_HANDLE_REQUEST => hello_world, 
    core::OP_HEALTH_REQUEST => health }

pub fn hello_world(ctx: &CapabilitiesContext, r: http::Request) -> CallResult {
    ctx.println(&format!("Received HTTP request: {:?}", &r));
    let echo = EchoRequest {
        method: r.method,
        path: r.path,
        query_string: r.query_string,
        headers: r.header,
        body: r.body,
    };

    let resp = http::Response::json(echo, 200, "OK");
    Ok(serialize(resp)?)      
}

pub fn health(_ctx: &CapabilitiesContext, _h: core::HealthRequest) -> CallResult {
    Ok(vec![])
}


#[derive(Serialize)]
struct EchoRequest {
    method: String,
    path: String,
    query_string: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}
