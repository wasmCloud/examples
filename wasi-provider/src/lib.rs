// Copyright 2015-2020 Capital One Services, LLC
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
use codec::capabilities::{CapabilityDescriptor, OperationDirection, OP_GET_CAPABILITY_DESCRIPTOR};

#[macro_use]
extern crate serde_derive;

const CUSTOM_OPERATION: &str = "DoCustomThing";
const CAPABILITY_ID: &str = "wascc:wasidemo";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const REVISION: u32 = 0;

actor_handlers! {
    crate::CUSTOM_OPERATION => do_custom,
    codec::core::OP_BIND_ACTOR => configure,
    codec::core::OP_HEALTH_REQUEST => health,
    OP_GET_CAPABILITY_DESCRIPTOR => get_descriptor
}

fn get_descriptor(_payload: codec::core::HealthRequest) -> HandlerResult<CapabilityDescriptor> {
    Ok(CapabilityDescriptor::builder()
        .id(CAPABILITY_ID)
        .name("waSCC Portable Provider Demo")
        .long_description("Sample illustrating that an actor can also be a capability provider")
        .version(VERSION)
        .revision(REVISION)
        .with_operation(
            CUSTOM_OPERATION,
            OperationDirection::ToActor,
            "Performs the custom, ultra-top secret operation",
        )
        .build())
}

// All capability providers _must_ respond to the configure operation, even if they
// do nothing with the data
fn configure(_payload: codec::core::CapabilityConfiguration) -> HandlerResult<()> {
    Ok(())
}

fn do_custom(msg: CustomMessage) -> HandlerResult<CustomReply> {
    println!(
        " ** WASI PROVIDER STDOUT ** Received Invocation! Super Secret Value - {}",
        msg.super_secret
    );
    Ok(CustomReply {
        reply_value: msg.super_secret * 10,
    })
}

fn health(_req: codec::core::HealthRequest) -> HandlerResult<()> {
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomMessage {
    pub super_secret: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomReply {
    pub reply_value: i32,
}
