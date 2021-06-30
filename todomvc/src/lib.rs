#[macro_use]
extern crate serde_json;

extern crate wapc_guest as guest;
use guest::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use wasmcloud_actor_core as actor;
use wasmcloud_actor_http_server as http;
use wasmcloud_actor_keyvalue as kv;

#[actor::init]
pub fn init() {
    http::Handlers::register_handle_request(request_handler);
}

/**
 * Create a todo --> assigning a new key/value
 * Delete a todo --> delete by key
 */

#[derive(Serialize, Deserialize)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

fn create_todo(msg: std::vec::Vec<u8>) {
    let key = kv::default().add("sequence.key".to_owned(), 1).unwrap();
    let todo: Todo = serde_json::from_slice(&msg).unwrap();
    let resp = kv::default().set(
        format!("{}", key.value),
        serde_json::to_string(&todo).unwrap(),
        0,
    );
}

fn request_handler(msg: http::Request) -> HandlerResult<http::Response> {
    match (msg.path.as_ref(), msg.method.as_ref()) {
        ("/", "POST") => create_todo(msg.body),
        //    ("/", "DELETE") => delete_todo(&msg.body),
        //    ("/", "GET") => get_todos(&msg.body),
    }
    Ok(())
}
