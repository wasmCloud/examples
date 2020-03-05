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

const CUSTOM_OPERATION: &str = "DoCustomThing";
const CAPABILITY_ID: &str = "wascc:wasidemo";

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

use actor::prelude::*;

actor_handlers! { http::OP_HANDLE_REQUEST => hello_world,
core::OP_HEALTH_REQUEST => health }

fn hello_world(ctx: &CapabilitiesContext, _payload: http::Request) -> ReceiveResult {
    let res = ctx.raw().call(
        CAPABILITY_ID,
        CUSTOM_OPERATION,
        &serialize(CustomMessage { super_secret: 12 })?,
    )?;
    let reply: CustomReply = deserialize(&res)?;

    let result = json!({ "result": reply.reply_value });
    Ok(serialize(http::Response::json(result, 200, "OK"))?)
}

fn health(_ctx: &CapabilitiesContext, _req: core::HealthRequest) -> ReceiveResult {
    Ok(vec![])
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomMessage {
    pub super_secret: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomReply {
    pub reply_value: i32,
}
