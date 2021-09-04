use anyhow::{anyhow, Result};
use log::{info, trace, warn};
use serde::{Deserialize, Serialize};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_keyvalue::{
    IncrementRequest, KeyValue, KeyValueSender, SetAddRequest, SetDelRequest, SetRequest,
};

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

async fn create_todo(ctx: &Context, input: InputTodo) -> Result<Todo> {
    info!("Creating a todo...");
    let id = KeyValueSender::new()
        .increment(
            ctx,
            &IncrementRequest {
                key: "next_id".to_string(),
                value: 1,
            },
        )
        .await
        .map_err(|e| anyhow!(e))?;

    let todo = Todo::new(
        format!("/api/{}", id),
        input.title,
        input.order.unwrap_or(0),
    );

    KeyValueSender::new()
        .set(
            ctx,
            &SetRequest {
                key: todo.url.clone(),
                value: serde_json::to_string(&todo)?,
                expires: 0,
            },
        )
        .await
        .map_err(|e| anyhow!(e))?;

    KeyValueSender::new()
        .set_add(
            ctx,
            &SetAddRequest {
                set_name: "all_urls".to_string(),
                value: todo.url.clone(),
            },
        )
        .await
        .map_err(|e| anyhow!(e))?;

    Ok(todo)
}

async fn update_todo(ctx: &Context, url: &str, update: UpdateTodo) -> Result<Todo> {
    let todo = get_todo(ctx, url).await?;
    let todo = todo.update(update);

    KeyValueSender::new()
        .set(
            ctx,
            &SetRequest {
                key: todo.url.clone(),
                value: serde_json::to_string(&todo)?,
                expires: 0,
            },
        )
        .await
        .map_err(|e| anyhow!(e))?;
    Ok(todo)
}

async fn get_all_todos(ctx: &Context) -> Result<Vec<Todo>> {
    let urls = KeyValueSender::new()
        .set_query(ctx, "all_urls")
        .await
        .map_err(|e| anyhow!(e))?;

    let mut result = Vec::new();
    for url in urls {
        result.push(get_todo(ctx, &url).await?)
    }
    Ok(result)
}

async fn get_todo(ctx: &Context, url: &str) -> Result<Todo> {
    let todo_str = KeyValueSender::new()
        .get(ctx, url)
        .await
        .map_err(|e| anyhow!(e))?
        .value;
    let todo = serde_json::from_str(&todo_str)?;

    Ok(todo)
}

async fn delete_all_todos(ctx: &Context) -> Result<()> {
    let urls = KeyValueSender::new()
        .set_query(ctx, "all_urls")
        .await
        .map_err(|e| anyhow!(e))?;

    for url in urls {
        delete_todo(ctx, &url).await?
    }

    Ok(())
}

async fn delete_todo(ctx: &Context, url: &str) -> Result<()> {
    KeyValueSender::new()
        .set_del(
            ctx,
            &SetDelRequest {
                set_name: "all_urls".to_string(),
                value: url.to_string(),
            },
        )
        .await
        .map_err(|e| anyhow!(e))?;

    KeyValueSender::new()
        .del(ctx, url)
        .await
        .map_err(|e| anyhow!(e))?;

    Ok(())
}

async fn handle_request(ctx: &Context, req: &HttpRequest) -> Result<HttpResponse, HttpResponse> {
    let trimmed_path = req.path.trim_end_matches('/');
    trace!("incoming req: {:?}, path: {:?}", req, trimmed_path);

    match (req.method.as_ref(), trimmed_path) {
        ("GET", "/") => Ok(HttpResponse {
            body: "todo server lives at /api".to_string().into_bytes(),
            ..Default::default()
        }),

        ("POST", "/api") => create_todo(
            ctx,
            serde_json::from_slice(&req.body)
                .map_err(|e| HttpResponse::bad_request(&e.to_string()))?,
        )
        .await
        .map(|todo| HttpResponse::json(todo, 200).unwrap()),

        ("GET", "/api") => get_all_todos(ctx)
            .await
            .map(|todos| HttpResponse::json(todos, 200).unwrap()),

        ("GET", url) => get_todo(ctx, url)
            .await
            .map(|todo| HttpResponse::json(todo, 200).unwrap())
            .or_else(|_| Ok(HttpResponse::not_found())),

        ("PATCH", url) => update_todo(
            ctx,
            url,
            serde_json::from_slice(&req.body)
                .map_err(|e| HttpResponse::bad_request(&e.to_string()))?,
        )
        .await
        .map(|todo| HttpResponse::json(todo, 200).unwrap()),

        ("DELETE", "/api") => delete_all_todos(ctx).await.map(|_| HttpResponse::default()),

        ("DELETE", url) => delete_todo(ctx, url)
            .await
            .map(|()| HttpResponse::default()),

        (_, _) => {
            warn!("not even a thing: {:?}", req);
            Ok(HttpResponse::not_found())
        }
    }
    .map_err(|e| HttpResponse::internal_server_error(format!("Something went wrong: {:?}", e)))
}

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct TodoActor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for TodoActor {
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        handle_request(ctx, req).await.or_else(Ok)
    }
}
