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
extern crate serde_json;

const STOCK_REQUEST: &str = "StockRequest";

use wascc_actor::prelude::core::CapabilityConfiguration;
use actor::prelude::*;
use serde::{Serialize, Deserialize};

actor_receive!(receive);

pub fn receive(ctx: &CapabilitiesContext, operation: &str, msg: &[u8]) -> CallResult {
    ctx.log(&format!("Handling operation {}", operation));
    match operation {
        STOCK_REQUEST => handle_stock(ctx, msg),
        core::OP_CONFIGURE => configure(ctx, CapabilityConfiguration::decode(msg)?),
        core::OP_HEALTH_REQUEST => Ok(vec![]),
        _ => Err("bad dispatch".into()),
    }
}

fn handle_stock(_ctx: &CapabilitiesContext, msg: &[u8]) -> CallResult {
    let req: StockRequest = serde_json::from_slice(msg)?;
    let resp = StockResponse {
        sku: req.sku.clone(),
        quantity: 21,
        ships_within: "six months".to_string()
    };

    Ok(serde_json::to_vec(&resp)?)
}

fn configure(_ctx: &CapabilitiesContext, 
    config: CapabilityConfiguration) -> CallResult {
    
    println!("Received actor configuration from module {}: ", config.module);
    Ok(vec![])
}

#[derive(Serialize, Deserialize)]
struct StockRequest {
    sku: String,
}

#[derive(Serialize, Deserialize)]
struct StockResponse {
    sku: String,
    quantity: u16,
    ships_within: String,
}
