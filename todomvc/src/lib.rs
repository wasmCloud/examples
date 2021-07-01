use anyhow::Result;
use guest::prelude::*;
use log::{info, warn};
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
    fn new(url: String, title: String) -> Self {
        Self {
            url,
            title,
            completed: false,
        }
    }
}

fn create_todo(input: InputTodo) -> Result<Todo> {
    info!("Creating a todo...");
    let id = kv::default()
        .add("next_id".to_string(), 1)
        .map_err(|e| anyhow::anyhow!(e))?
        .value;

    let todo = Todo::new(format!("/api/{}", id), input.title);

    kv::default()
        .set(todo.url.clone(), serde_json::to_string(&todo)?, 0)
        .map_err(|e| anyhow::anyhow!(e))?;

    kv::default()
        .set_add("all_urls".to_string(), todo.url.clone())
        .map_err(|e| anyhow::anyhow!(e))?;

    Ok(todo)
}

fn update_todo(url: &str, input: InputTodo) -> Result<Todo> {
    let mut todo = get_todo(url)?;
    todo.title = input.title;

    kv::default()
        .set(url.to_string(), serde_json::to_string(&todo)?, 0)
        .map_err(|e| anyhow::anyhow!(e))?;
    Ok(todo)
}

fn get_all_todos() -> Result<Vec<Todo>> {
    let urls = kv::default()
        .set_query("all_urls".to_string())
        .map_err(|e| anyhow::anyhow!(e))?
        .values;

    urls.into_iter().map(|url| get_todo(&url)).collect()
}

fn get_todo(url: &str) -> Result<Todo> {
    let todo_str = kv::default()
        .get(url.to_string())
        .map_err(|e| anyhow::anyhow!(e))?
        .value;
    let todo = serde_json::from_str(&todo_str)?;

    Ok(todo)
}

fn delete_all_todos() -> Result<()> {
    let urls = kv::default()
        .set_query("all_urls".to_string())
        .map_err(|e| anyhow::anyhow!(e))?
        .values;

    for url in urls {
        delete_todo(&url)?
    }

    Ok(())
}

fn delete_todo(url: &str) -> Result<()> {
    kv::default()
        .set_remove("all_urls".to_string(), url.to_string())
        .map_err(|e| anyhow::anyhow!(e))?;

    kv::default()
        .del(url.to_string())
        .map_err(|e| anyhow::anyhow!(e))?;

    Ok(())
}

fn handle_delete_todo(url: &str) -> Result<http::Response> {
    delete_todo(url).map(|()| http::Response::json("", 200, "OK"))
}

fn handle_get_todo(url: &str) -> Result<http::Response> {
    get_todo(url)
        .map(|todo| http::Response::json(todo, 200, "OK"))
        .or_else(|_| Ok(http::Response::not_found()))
}

fn request_handler(msg: http::Request) -> HandlerResult<http::Response> {
    let trimmed_path = msg.path.trim_end_matches("/");
    warn!("incoming msg: {:?}, path: {:?}", msg, trimmed_path);

    match (msg.method.as_ref(), trimmed_path) {
        ("POST", "/api") => create_todo(serde_json::from_slice(&msg.body)?)
            .map(|todo| http::Response::json(todo, 200, "OK")),
        ("GET", "/api") => get_all_todos().map(|todos| http::Response::json(todos, 200, "OK")),
        ("GET", url) => handle_get_todo(url),
        ("PATCH", url) => update_todo(url, serde_json::from_slice(&msg.body)?)
            .map(|todo| http::Response::json(todo, 200, "OK")),
        ("DELETE", "/api") => delete_all_todos().map(|_| http::Response::ok()),
        ("DELETE", url) => handle_delete_todo(url),
        (_, _) => {
            warn!("not even a thing: {:?}", msg);
            Ok(http::Response::not_found())
        }
    }
    .or_else(|e| {
        Ok(http::Response::internal_server_error(&format!(
            "Something went wrong: {:?}",
            e
        )))
    })
}
