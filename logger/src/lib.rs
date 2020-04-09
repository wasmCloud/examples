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

#[macro_use]
extern crate log;

extern crate wascc_actor as actor;

use actor::prelude::*;

actor_handlers! { codec::http::OP_HANDLE_REQUEST => hello_world, 
                  codec::core::OP_HEALTH_REQUEST => health }

fn hello_world(payload: codec::http::Request) -> ReceiveResult {
    println("Received an HTTP request");    
    info!("Received request: {:?}", payload);
    logger::default().warn("Received an HTTP request")?;
    Ok(serialize(codec::http::Response::ok())?)
}

fn health(_req: codec::core::HealthRequest) -> ReceiveResult {
    Ok(vec![])
}