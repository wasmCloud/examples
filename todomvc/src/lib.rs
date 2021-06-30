use anyhow::Result;
use guest::prelude::*;
use serde::{ser, Deserialize, Serialize};
use wapc_guest as guest;
use wasmcloud_actor_core as actor;
use wasmcloud_actor_http_server as http;
use wasmcloud_actor_keyvalue as kv;

#[actor::init]
pub fn init() {
    http::Handlers::register_handle_request(request_handler);
}

#[derive(Serialize, Deserialize)]
struct InputTodo {
    title: String,
}
#[derive(Serialize, Deserialize)]
struct Todo {
    id: i32,
    title: String,
    completed: bool,
}

impl Todo {
    fn new(id: i32, title: String) -> Self {
        Self {
            id,
            title,
            completed: false,
        }
    }
}

fn create_todo(input: InputTodo) -> Result<Todo> {
    let id = kv::default()
        .add("next_id".to_string(), 1)
        .map_err(|e| anyhow::anyhow!(e))?
        .value;

    let todo = Todo::new(id, input.title);

    kv::default()
        .set(id.to_string(), serde_json::to_string(&todo)?, 0)
        .map_err(|e| anyhow::anyhow!(e))?;

    kv::default()
        .push("all_ids".to_string(), id.to_string())
        .map_err(|e| anyhow::anyhow!(e))?;

    Ok(todo)
}

fn get_all_todos() -> Result<Vec<Todo>> {
    let ids = kv::default()
        .range("all_ids".to_string(), 0, -1)
        .map_err(|e| anyhow::anyhow!(e))?
        .values;

    let result: Result<Vec<_>, _> = ids.into_iter().map(|id| kv::default().get(id)).collect();
    let res_vec = result.map_err(|e| anyhow::anyhow!(e))?;
    let todos = res_vec
        .into_iter()
        .map(|response| serde_json::from_str(&response.value))
        .collect::<Result<Vec<Todo>, _>>()?;

    Ok(todos)
}

fn request_handler(msg: http::Request) -> HandlerResult<http::Response> {
    match (msg.path.as_ref(), msg.method.as_ref()) {
        ("/", "POST") => create_todo(serde_json::from_slice(&msg.body)?)
            .map(|todo| http::Response::json(todo, 200, "OK")),
        ("/", "GET") => get_all_todos()
            .map(|todos| http::Response::json(todos, 200, "OK")),
        (_, _) => Ok(http::Response::not_found()),
    }
    .or_else(|_| {
        Ok(http::Response::internal_server_error(
            "Something went wrong",
        ))
    })
}
