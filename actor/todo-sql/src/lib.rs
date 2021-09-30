use serde::{Deserialize, Serialize};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_numbergen::generate_guid;
use wasmcloud_interface_sqldb::{
    SqlDb, SqlDbError, SqlDbSender,
    //FetchResult, ExecuteResult,
    minicbor,
};
use wasmcloud_interface_logging::{debug, info, warn};
use once_cell::sync::Lazy;
use minicbor::{decode,Decode};

type Db = SqlDbSender<WasmHost>;
const TABLE_NAME :&str = "todos";
/// limit characters to avoid sql injection attacks
/// - english letters uppercase & lowercase
/// - digits 0-9
/// - space, comma, period, underscore, dash, slash
/// that's it!
static REGEX: Lazy<regex::Regex> = Lazy::new(|| {
    let re = regex::Regex::new(r"^[a-zA-Z0-9 ,._-/]+$").unwrap();
    re
});

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
// because 'order' is a keyword in sql, we've renamed the field 'priority',
// but for serde serialization, leave it as 'order' so that it works
// with existing javascript written for this app
#[derive(Serialize, Deserialize, Decode)]
struct Todo {
    #[n(0)] url: String,
    #[n(1)] title: String,
    #[n(2)] completed: bool,
    #[serde(rename="order")]
    #[n(3)] priority: i32,
}

impl Todo {
    fn new(url: String, title: String, priority: i32) -> Self {
        Self {
            url,
            title,
            completed: false,
            priority,
        }
    }

    fn update(self, update: UpdateTodo) -> Todo {
        Todo {
            url: self.url,
            title: update.title.unwrap_or(self.title),
            completed: update.completed.unwrap_or(self.completed),
            priority: update.order.unwrap_or(self.priority),
        }
    }
}

// check safety of any user input string before it used in an sql statement
// If the provider (&interface) supported prepared statements,
// this could allow more characters
fn check_safety(tag: &str, uncertain_input: &str) -> Result<(), SqlDbError> {
    if !REGEX.is_match(&uncertain_input) {
        return Err(SqlDbError::new("invalid",
           format!("{} contains invalid characters", tag)))
    }
    Ok(())
}

/// create an empty table with the proper schema
async fn create_table(ctx: &Context, client: &Db) -> Result<(),SqlDbError> {

    let sql = format!(r#"create table {} (
            id varchar(36) not null,
            url varchar(42) not null,
            title varchar(100) not null,
            priority int4 not null default 0,
            completed bool not null default false
        );"#, TABLE_NAME);
    let _resp = client.execute(&ctx, &sql) .await?;
    Ok(())
}

/// insert a new to-do item
async fn create_todo(ctx: &Context, client: &Db, input: InputTodo) -> Result<Todo,SqlDbError> {

    check_safety("title", &input.title)?;
    let id = generate_guid().await.unwrap();
    let todo = Todo::new(
        format!("/api/{}", id),
        input.title,
        input.order.unwrap_or(0),
    );

    let _resp = client
        .execute(&ctx, &format!(r#"
        insert into {} (id, url, title, order, completed) values ('{}','{}', '{}', {})
        "#, TABLE_NAME, id, &todo.url, &todo.title, todo.priority))
        .await?;

    Ok(todo)
}

/// updates an existing to-do item
async fn update_todo(ctx: &Context, client: &Db, url: &str, update: UpdateTodo) -> Result<Todo, SqlDbError> {
    info!("Updating a todo...");

    check_safety("url", url)?;
    if let Some(title) = &update.title {
        check_safety("title", &title)?;
    }
    let todo = get_todo(ctx, client, url).await?;
    let todo = todo.update(update);

    let _resp = client
        .execute(&ctx, &format!(r#"
        update {} set title='{}', priority={} where url='{}';
        "#, TABLE_NAME, &todo.title, todo.priority, &todo.url))
        .await?;
    Ok(todo)
}

async fn get_all_todos(ctx: &Context, client: &Db) -> Result<Vec<Todo>, SqlDbError> {
    info!("Getting all todos...");
    let resp = client
        .fetch(
            &ctx,
            &format!("select url, title, completed, order from {}", TABLE_NAME)
        )
        .await?;
    let rows: Vec<Todo> = decode(&resp.rows)?;
    Ok(rows)
}

async fn get_todo(ctx: &Context, client: &Db, url: &str) -> Result<Todo, SqlDbError> {
    info!("Getting a todo...");
    check_safety("url", url)?;
    let resp = client
        .fetch(
            &ctx,
            &format!("select url, title, completed, order from {} where url='{}'", TABLE_NAME, url)
        )
        .await?;
    if resp.num_rows == 0 {
        return Err(SqlDbError::new("notFound", "url not found".to_string()));
    }
    let mut rows: Vec<Todo> = decode(&resp.rows)?;
    let todo = rows.remove(0);
    Ok(todo)
}

async fn delete_all_todos(ctx: &Context, client: &Db) -> Result<(), SqlDbError> {
    info!("Deleting all todos...");

    let _resp = client
        .execute(&ctx, &format!("delete from {}", TABLE_NAME))
        .await?;

    Ok(())
}

async fn delete_todo(ctx: &Context, client: &Db, url: &str) -> Result<(), SqlDbError> {
    info!("Deleting a todo...");

    check_safety("url", url)?;
    let _resp = client
        .execute(&ctx, &format!("delete from {} where url='{}'", TABLE_NAME, url))
        .await?;

    Ok(())
}

async fn handle_request(ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
    debug!("incoming req: {:?}", req);

    let trimmed_path = req.path.trim_end_matches('/');
    let db = SqlDbSender::new();

    match (req.method.as_ref(), trimmed_path) {
        ("GET", "/") => Ok(HttpResponse {
            body: "todo server lives at /api.  Initialize empty table with /init".to_string().into_bytes(),
            ..Default::default()
        }),
        ("GET", "/init") => match create_table(ctx, &db).await {
            Ok(()) => Ok(HttpResponse {
                body: "table created".to_string().into_bytes(),
                ..Default::default()
            }),
            Err(e) => Err(RpcError::ActorHandler(format!("creating table: {}", e.to_string()))),
        },

        ("POST", "/api") => match serde_json::from_slice(&req.body) {
            Ok(input) => match create_todo(ctx, &db, input).await {
                Ok(todo) => HttpResponse::json(todo, 200),
                Err(e) => Err(RpcError::ActorHandler(format!("creating todo: {:?}", e))),
            },
            Err(e) => Ok(HttpResponse::bad_request(format!(
                "malformed body: {:?}",
                e
            ))),
        },

        ("GET", "/api") => match get_all_todos(ctx, &db).await {
            Ok(todos) => HttpResponse::json(todos, 200),
            Err(e) => Err(RpcError::ActorHandler(format!("getting all todos: {}", e.to_string()))),
        },

        ("GET", url) => match get_todo(ctx,  &db, url).await {
            Ok(todo) => HttpResponse::json(todo, 200),
            Err(_) => Ok(HttpResponse::not_found()),
        },

        ("PATCH", url) => match serde_json::from_slice(&req.body) {
            Ok(update) => match update_todo(ctx, &db, url, update).await {
                Ok(todo) => HttpResponse::json(todo, 200),
                Err(e) => Err(RpcError::ActorHandler(format!("updating todo: {}", e.to_string()))),
            },
            Err(e) => Ok(HttpResponse::bad_request(format!(
                "malformed body: {:?}",
                e
            ))),
        },

        ("DELETE", "/api") => match delete_all_todos(ctx, &db).await {
            Ok(_) => Ok(HttpResponse::default()),
            Err(e) => Err(RpcError::ActorHandler(format!("deleting all todos: {}", e.to_string()))),
        },

        ("DELETE", url) => match delete_todo(ctx, &db, url).await {
            Ok(_) => Ok(HttpResponse::default()),
            Err(e) => Err(RpcError::ActorHandler(format!("deleting todo: {}", e.to_string()))),
        },

        (_, _) => {
            warn!("no route for this request: {:?}", req);
            Ok(HttpResponse::not_found())
        }
    }
}

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct TodoActor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for TodoActor {
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        handle_request(ctx, req).await
    }
}
