use anyhow::{anyhow, Result};
use guest::prelude::*;
use http::{Method, Request, Response};
use log::{info, trace, warn};
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
    order: Option<i32>,
}
#[derive(Serialize, Deserialize)]
struct UpdateTodo {
    title: Option<String>,
    completed: Option<bool>,
    order: Option<i32>,
}
#[derive(Serialize, Deserialize)]
struct Todo {
    url: String,
    title: String,
    completed: bool,
    order: i32,
}

impl Todo {
    fn new(url: String, title: String, order: i32) -> Self {
        Self {
            url,
            title,
            completed: false,
            order,
        }
    }

    fn update(self, update: UpdateTodo) -> Todo {
        Todo {
            url: self.url,
            title: update.title.unwrap_or(self.title),
            completed: update.completed.unwrap_or(self.completed),
            order: update.order.unwrap_or(self.order),
        }
    }
}

fn create_todo(input: InputTodo) -> Result<Todo> {
    info!("Creating a todo...");
    let id = kv::default()
        .add("next_id".to_string(), 1)
        .map_err(|e| anyhow!(e))?
        .value;

    let todo = Todo::new(
        format!("/api/{}", id),
        input.title,
        input.order.unwrap_or(0),
    );

    kv::default()
        .set(todo.url.clone(), serde_json::to_string(&todo)?, 0)
        .map_err(|e| anyhow!(e))?;

    kv::default()
        .set_add("all_urls".to_string(), todo.url.clone())
        .map_err(|e| anyhow!(e))?;

    Ok(todo)
}

fn update_todo(url: &str, update: UpdateTodo) -> Result<Todo> {
    let todo = get_todo(url)?;
    let todo = todo.update(update);

    kv::default()
        .set(url.to_string(), serde_json::to_string(&todo)?, 0)
        .map_err(|e| anyhow!(e))?;
    Ok(todo)
}

fn get_all_todos() -> Result<Vec<Todo>> {
    let urls = kv::default()
        .set_query("all_urls".to_string())
        .map_err(|e| anyhow!(e))?
        .values;

    urls.iter().map(|url| get_todo(url)).collect()
}

fn get_todo(url: &str) -> Result<Todo> {
    let todo_str = kv::default()
        .get(url.to_string())
        .map_err(|e| anyhow!(e))?
        .value;
    let todo = serde_json::from_str(&todo_str)?;

    Ok(todo)
}

fn delete_all_todos() -> Result<()> {
    let urls = kv::default()
        .set_query("all_urls".to_string())
        .map_err(|e| anyhow!(e))?
        .values;

    for url in urls {
        delete_todo(&url)?
    }

    Ok(())
}

fn delete_todo(url: &str) -> Result<()> {
    kv::default()
        .set_remove("all_urls".to_string(), url.to_string())
        .map_err(|e| anyhow!(e))?;

    kv::default().del(url.to_string()).map_err(|e| anyhow!(e))?;

    Ok(())
}

fn request_handler(msg: Request) -> HandlerResult<Response> {
    let trimmed_path = msg.path.trim_end_matches('/');
    trace!("incoming msg: {:?}, path: {:?}", msg, trimmed_path);

    match (msg.method(), trimmed_path) {
        (Method::Post, "/api") => create_todo(serde_json::from_slice(&msg.body)?)
            .map(|todo| Response::json(todo, 200, "OK")),

        (Method::Get, "/api") => get_all_todos().map(|todos| Response::json(todos, 200, "OK")),

        (Method::Get, url) => get_todo(url)
            .map(|todo| Response::json(todo, 200, "OK"))
            .or_else(|_| Ok(Response::not_found())),

        (Method::Patch, url) => update_todo(url, serde_json::from_slice(&msg.body)?)
            .map(|todo| Response::json(todo, 200, "OK")),

        (Method::Delete, "/api") => delete_all_todos().map(|_| Response::ok()),

        (Method::Delete, url) => delete_todo(url).map(|()| Response::ok()),

        (_, _) => {
            warn!("not even a thing: {:?}", msg);
            Ok(Response::not_found())
        }
    }
    .or_else(|e| {
        Ok(Response::internal_server_error(&format!(
            "Something went wrong: {:?}",
            e
        )))
    })
}
