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

actor_handlers! { codec::http::OP_HANDLE_REQUEST => gen_and_render_counts, 
                  codec::core::OP_HEALTH_REQUEST => health }

fn gen_and_render_counts(payload: codec::http::Request) -> ReceiveResult {
    let key = payload.path.replace('/', ":");
    
    let source1 = keyvalue::host("source1");    
    let source2 = keyvalue::host("source2");

    // To visually illustrate these are different sources, increment the
    // same key by a different amount
    let val1 = source1.atomic_add(&key, 1)?;
    let val2 = source2.atomic_add(&key, 2)?;

    let result = json!(
        { "counter_1": val1,
          "counter_2": val2 });

    Ok(serialize(codec::http::Response::json(result, 200, "OK"))?)    
}

fn health(_req: codec::core::HealthRequest) -> ReceiveResult {
    Ok(vec![])
}