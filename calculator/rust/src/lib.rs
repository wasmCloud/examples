extern crate wapc_guest as guest;
use wasmcloud_actor_core as actor;
extern crate wasmcloud_actor_http_server as httpserver;

use guest::prelude::*;

#[actor::init]
fn init() {
    httpserver::Handlers::register_handle_request(test_body);
}

fn test_body(msg: httpserver::Request) -> HandlerResult<httpserver::Response> {
    let nums: Vec<&str> = msg.query_string.split(",").collect();

    match msg.path.as_str() {
        "/add" => {
            let sum = nums[0].parse::<i32>().unwrap() + nums[1].parse::<i32>().unwrap();
            let ret = format!("add: {} + {} = {}", nums[0], nums[1], sum);
            return Ok(httpserver::Response {
                status_code: 200,
                status: "OK".to_string(),
                header: msg.header,
                body: ret.as_bytes().to_vec(),
            });
        }
        "/sub" => {
            let sub = nums[0].parse::<i32>().unwrap() - nums[1].parse::<i32>().unwrap();
            let ret = format!("subtract: {} - {} = {}", nums[0], nums[1], sub);
            return Ok(httpserver::Response {
                status_code: 200,
                status: "OK".to_string(),
                header: msg.header,
                body: ret.as_bytes().to_vec(),
            });
        }
        // TODO: add multiply capabilities
        "/div" => {
            if nums[1] == "0" {
                return Ok(httpserver::Response {
                    status_code: 200,
                    status: "OK".to_string(),
                    header: msg.header,
                    body: "Can not divide by zero!".as_bytes().to_vec(),
                });
            }
            let div = nums[0].parse::<i32>().unwrap() / nums[1].parse::<i32>().unwrap();
            let ret = format!("divide: {} / {} = {}", nums[0], nums[1], div);
            return Ok(httpserver::Response {
                status_code: 200,
                status: "OK".to_string(),
                header: msg.header,
                body: ret.as_bytes().to_vec(),
            });
        }
        _ => {
            return Ok(httpserver::Response {
                status_code: 200,
                status: "OK".to_string(),
                header: msg.header,
                body: b"Welcome to the wasmcloud calculator".to_vec(),
            });
        }
    }
}
