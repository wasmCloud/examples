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

#[macro_use]
extern crate serde_json;

use actor::prelude::*;

actor_handlers! { codec::http::OP_HANDLE_REQUEST => increment_counter,
                  codec::core::OP_HEALTH_REQUEST => health }

fn increment_counter(msg: codec::http::Request) -> CallResult {
    let key = msg.path.replace('/', ":");
    let value = keyvalue::default().atomic_add(&key, 1)?;

    //let result = json!({"counter": value});
    let result = json!({ "counter": value, "tweaked": true });
    Ok(serialize(codec::http::Response::json(result, 200, "OK"))?)
}

fn health(_h: codec::core::HealthRequest) -> ReceiveResult {
    Ok(vec![])
}
