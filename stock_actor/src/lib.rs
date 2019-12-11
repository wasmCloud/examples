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

extern crate serde_json;
extern crate wascc_actor as actor;

const CAPABILITY_ID: &str = "acme:stock";
const STOCK_REQUEST: &str = "StockRequest";

use actor::prelude::*;
use serde::{Deserialize, Serialize};

actor_receive!(receive);

pub fn receive(ctx: &CapabilitiesContext, operation: &str, msg: &[u8]) -> CallResult {
    ctx.log(&format!("Handling operation {}", operation));
    match operation {
        http::OP_HANDLE_REQUEST => query_stock(ctx, msg),
        core::OP_HEALTH_REQUEST => Ok(vec![]),
        _ => Err("bad dispatch".into()),
    }
}

fn query_stock(ctx: &CapabilitiesContext, payload: impl Into<http::Request>) -> CallResult {
    let req = StockRequest {
        sku: payload.into().path[1..].to_string()
    };
    let req_raw = serde_json::to_vec(&req)?;
    let res_raw = ctx.raw().call(CAPABILITY_ID, STOCK_REQUEST, &req_raw)?;
    let res: StockResponse = serde_json::from_slice(&res_raw)?;

    let http_res = http::Response::json(res, 200, "OK");
    Ok(protobytes(http_res)?)
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
