use anyhow::Result;
use guest::prelude::*;
use log::warn;
use serde::{Deserialize, Serialize};
use wapc_guest as guest;
use wasmcloud_actor_core as actor;
use wasmcloud_actor_http_server as http;
use wasmcloud_actor_keyvalue as kv;
use wasmcloud_actor_logging as logging;

#[actor::init]
pub fn init() {
    http::Handlers::register_handle_request(request_handler);
    logging::enable_macros();
    warn!("starting up");
}

#[derive(Serialize, Deserialize)]
struct InputTodo {
    title: String,
}
#[derive(Serialize, Deserialize)]
struct Todo {
    url: String,
    title: String,
    completed: bool,
}

impl Todo {
    fn new(id: i32, title: String) -> Self {
        Self {
            url: format!("/{}", id),
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
        .set_add("all_ids".to_string(), id.to_string())
        .map_err(|e| anyhow::anyhow!(e))?;

    Ok(todo)
}

fn get_all_todos() -> Result<Vec<Todo>> {
    let ids = kv::default()
        .set_query("all_ids".to_string())
        .map_err(|e| anyhow::anyhow!(e))?
        .values;

    ids.into_iter().map(|id| get_todo(id.parse()?)).collect()
}

fn get_todo(id: i32) -> Result<Todo> {
    let todo_str = kv::default()
        .get(id.to_string())
        .map_err(|e| anyhow::anyhow!(e))?
        .value;
    let todo = serde_json::from_str(&todo_str)?;

    Ok(todo)
}

fn delete_all_todos() -> Result<()> {
    let ids = kv::default()
        .set_query("all_ids".to_string())
        .map_err(|e| anyhow::anyhow!(e))?
        .values;

    for id in ids {
        kv::default()
            .set_remove("all_ids".to_string(), id.clone())
            .map_err(|e| anyhow::anyhow!(e))?;

        kv::default().del(id).map_err(|e| anyhow::anyhow!(e))?;
    }

    Ok(())
}

fn request_handler(msg: http::Request) -> HandlerResult<http::Response> {
    match (msg.path.as_ref(), msg.method.as_ref()) {
        ("/", "POST") => create_todo(serde_json::from_slice(&msg.body)?)
            .map(|todo| http::Response::json(todo, 200, "OK")),
        ("/", "GET") => get_all_todos().map(|todos| http::Response::json(todos, 200, "OK")),
        (path, "GET") => {
            if let Ok(id) = path.trim_matches('/').parse() {
                get_todo(id).map(|todo| http::Response::json(todo, 200, "OK"))
            } else {
                Ok(http::Response::not_found())
            }
        }
        ("/", "DELETE") => delete_all_todos().map(|_| http::Response::ok()),
        (_, _) => Ok(http::Response::not_found()),
    }
    .or_else(|e| {
        Ok(http::Response::internal_server_error(&format!(
            "Something went wrong: {:?}",
            e
        )))
    })
}
