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

#[macro_use]
extern crate serde_derive;

const CUSTOM_OPERATION: &str = "DoCustomThing";

actor_handlers! { crate::CUSTOM_OPERATION => do_custom,
                  core::OP_CONFIGURE => configure, 
                  core::OP_HEALTH_REQUEST => health }


// All capability providers _must_ respond to the configure operation, even if they
// do nothing with the data
fn configure(_ctx: &CapabilitiesContext, payload: core::CapabilityConfiguration) -> ReceiveResult {
    // We can do println because it's WASI
    println!("Received configuration: {:?}", payload);
    Ok(vec![])
}

fn do_custom(_ctx: &CapabilitiesContext, msg: CustomMessage) -> ReceiveResult {
    Ok(serialize(
        CustomReply{
            reply_value: msg.super_secret * 10,
        }
    )?)
}

fn health(
    _ctx: &CapabilitiesContext,
    _req: core::HealthRequest
) -> ReceiveResult {
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