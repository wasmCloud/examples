//! This example actor is based on the todo actor, which was written to use a keyvalue
//! provider as the back end. Both examples (todo and todo-sql) follow the conventions
//! of [Todo-Backend](https://www.todobackend.com)
use minicbor::{decode, Decode, Encode};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{
    HeaderMap, HttpRequest, HttpResponse, HttpServer, HttpServerReceiver,
};
use wasmcloud_interface_logging::{debug, error, info, warn};
use wasmcloud_interface_numbergen::generate_guid;
use wasmcloud_interface_sqldb::{minicbor, SqlDb, SqlDbError, SqlDbSender};

type Db = SqlDbSender<WasmHost>;
const TABLE_NAME: &str = "todos";

/// limit characters to avoid sql injection attacks
/// - english letters uppercase & lowercase
/// - digits 0-9
/// - dash, space, comma, period, underscore, slash
/// that's it!
static REGEX: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r"^[-a-zA-Z0-9 ,._/]+$").unwrap());

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

/// Todo data structure for communicating with web UI
#[derive(Serialize, Deserialize)]
struct Todo {
    url: String,
    title: String,
    completed: bool,
    order: i32,
}

/// Todo data structure for sending to database. It's best practice to separate front-end data
/// structures from back-end, to preserve the abstraction between layers of an application,
/// and also to perform validation and sanitization steps. This mirrors the ToDo structure,
/// except that 'order' is renamed to 'priority' because 'order' is an sql keyword.
#[derive(Encode, Decode)]
struct DbTodo {
    #[n(0)]
    url: String,
    #[n(1)]
    title: String,
    #[n(2)]
    completed: bool,
    #[n(3)]
    priority: i32,
}

impl From<DbTodo> for Todo {
    fn from(t: DbTodo) -> Todo {
        Todo {
            url: t.url,
            title: t.title,
            completed: t.completed,
            order: t.priority,
        }
    }
}
impl From<Todo> for DbTodo {
    fn from(t: Todo) -> DbTodo {
        DbTodo {
            url: t.url,
            title: t.title,
            completed: t.completed,
            priority: t.order,
        }
    }
}

impl DbTodo {
    /// create a new DbTodo.
    /// Safety check is built into the constructor so you cannot
    /// create an invalid object
    fn new(url: String, title: String, priority: i32) -> Result<Self, SqlDbError> {
        let db_todo = DbTodo {
            url,
            title,
            completed: false,
            priority,
        };
        db_todo.check_safety()?;
        Ok(db_todo)
    }

    /// update fields from input - fails if fields are invalid
    fn update(self, update: UpdateTodo) -> Result<DbTodo, SqlDbError> {
        let db_todo = DbTodo {
            url: self.url,
            title: update.title.unwrap_or(self.title),
            completed: update.completed.unwrap_or(self.completed),
            priority: update.order.unwrap_or(self.priority),
        };
        db_todo.check_safety()?;
        Ok(db_todo)
    }

    fn check_safety(&self) -> Result<(), SqlDbError> {
        check_safety("url", &self.url)?;
        check_safety("title", &self.title)?;
        Ok(())
    }
}

// check safety of any user input string before it used in an sql statement
// If the provider (&interface) supported prepared statements,
// this could allow more characters
fn check_safety(tag: &str, uncertain_input: &str) -> Result<(), SqlDbError> {
    if !REGEX.is_match(uncertain_input) {
        return Err(SqlDbError::new(
            "invalid",
            format!("{} contains invalid characters", tag),
        ));
    }
    Ok(())
}

/// create an empty table with the proper schema
async fn create_table(ctx: &Context, client: &Db) -> Result<(), SqlDbError> {
    let sql = format!(
        r#"create table if not exists {} (
            id varchar(36) not null,
            url varchar(42) not null,
            title varchar(100) not null,
            priority int4 not null default 0,
            completed bool not null default false
        );"#,
        TABLE_NAME
    );
    let _resp = client.execute(ctx, &sql).await?;
    Ok(())
}

/// insert a new to-do item
async fn create_todo(ctx: &Context, client: &Db, input: InputTodo) -> Result<Todo, SqlDbError> {
    let id = generate_guid().await.unwrap();
    let db_todo = DbTodo::new(
        format!("/api/{}", id),
        input.title,
        input.order.unwrap_or(0),
    )?;
    let _resp = client
        .execute(
            ctx,
            &format!(
                r#"
        insert into {} (id, url, title, priority) values ('{}','{}', '{}', {})
        "#,
                TABLE_NAME, id, &db_todo.url, &db_todo.title, db_todo.priority
            ),
        )
        .await?;

    Ok(db_todo.into())
}

/// updates an existing to-do item
async fn update_todo(
    ctx: &Context,
    client: &Db,
    url: &str,
    update: UpdateTodo,
) -> Result<Todo, SqlDbError> {
    info!("Updating a todo...");

    let db_todo = get_db_todo(ctx, client, url).await?;
    let db_todo = db_todo.update(update)?;
    let _resp = client
        .execute(
            ctx,
            &format!(
                r#"
        update {} set title='{}', priority={}, completed={} where url='{}';
        "#,
                TABLE_NAME, &db_todo.title, db_todo.priority, db_todo.completed, &db_todo.url
            ),
        )
        .await?;
    Ok(db_todo.into())
}

async fn get_all_todos(ctx: &Context, client: &Db) -> Result<Vec<Todo>, SqlDbError> {
    info!("Getting all todos...");
    let resp = client
        .fetch(
            ctx,
            &format!("select url, title, completed, priority from {}", TABLE_NAME),
        )
        .await?;
    info!(
        "SQLDB: result of get all todos: err={:?}, count={}",
        &resp.error, resp.num_rows
    );
    if resp.num_rows == 0 {
        Ok(Vec::new())
    } else {
        let rows: Vec<DbTodo> = decode(&resp.rows)?;
        let rows: Vec<Todo> = rows.into_iter().map(|t| t.into()).collect();
        Ok(rows)
    }
}

async fn get_todo(ctx: &Context, client: &Db, url: &str) -> Result<Todo, SqlDbError> {
    Ok(get_db_todo(ctx, client, url).await?.into())
}

async fn get_db_todo(ctx: &Context, client: &Db, url: &str) -> Result<DbTodo, SqlDbError> {
    info!("Getting a todo...");
    check_safety("url", url)?;
    let resp = client
        .fetch(
            ctx,
            &format!(
                "select url, title, completed, priority from {} where url='{}'",
                TABLE_NAME, url
            ),
        )
        .await?;
    if resp.num_rows == 0 {
        return Err(SqlDbError::new("notFound", "url not found".to_string()));
    }
    let mut rows: Vec<DbTodo> = decode(&resp.rows)?;
    let db_todo = rows.remove(0);
    Ok(db_todo)
}

async fn delete_all_todos(ctx: &Context, client: &Db) -> Result<(), SqlDbError> {
    info!("Deleting all todos...");
    let _resp = client
        .execute(ctx, &format!("delete from {}", TABLE_NAME))
        .await?;
    Ok(())
}

async fn delete_todo(ctx: &Context, client: &Db, url: &str) -> Result<(), SqlDbError> {
    info!("Deleting a todo...");

    check_safety("url", url)?;
    let _resp = client
        .execute(
            ctx,
            &format!("delete from {} where url='{}'", TABLE_NAME, url),
        )
        .await?;

    Ok(())
}

async fn handle_request(ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
    debug!("incoming req: {:?}", req);

    let trimmed_path = req.path.trim_end_matches('/');
    let db = SqlDbSender::new();

    match (req.method.as_ref(), trimmed_path) {
        ("GET", "") | ("GET", "/help") => Ok(HttpResponse {
            body: "todo server lives at /api.  Initialize empty table with 'curl -XPOST localhost:POST/init'"
                .to_string()
                .into_bytes(),
            ..Default::default()
        }),

        ("POST", "/init") => match create_table(ctx, &db).await {
            Ok(()) => Ok(HttpResponse {
                body: "table created".to_string().into_bytes(),
                ..Default::default()
            }),
            Err(e) => Err(RpcError::ActorHandler(format!(
                "creating table: {}",
                e
            ))),
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
            Err(e) => Err(RpcError::ActorHandler(format!(
                "getting all todos: {}",
                e
            ))),
        },

        ("GET", url) if url.starts_with("/api") => match get_todo(ctx, &db, url).await {
            Ok(todo) => HttpResponse::json(todo, 200),
            Err(_) => Ok(HttpResponse::not_found()),
        },

        ("PATCH", url) => match serde_json::from_slice(&req.body) {
            Ok(update) => match update_todo(ctx, &db, url, update).await {
                Ok(todo) => HttpResponse::json(todo, 200),
                Err(e) => Err(RpcError::ActorHandler(format!(
                    "updating todo: {}",
                    e
                ))),
            },
            Err(e) => Ok(HttpResponse::bad_request(format!(
                "malformed body: {:?}",
                e
            ))),
        },

        ("DELETE", "/api") => match delete_all_todos(ctx, &db).await {
            Ok(_) => Ok(HttpResponse::default()),
            Err(e) => Err(RpcError::ActorHandler(format!(
                "deleting all todos: {}",
                e
            ))),
        },

        ("DELETE", url) => match delete_todo(ctx, &db, url).await {
            Ok(_) => Ok(HttpResponse::default()),
            Err(e) => Err(RpcError::ActorHandler(format!(
                "deleting todo: {}",
                e
            ))),
        },

        // tests
        ("GET", url) if url.starts_with("/test") => {
            if &req.path == "/test" {
                let mut header = HeaderMap::new();
                header.insert("Location".to_string(), vec!["/test/".to_string()]);
                return Ok(HttpResponse{
                    header,
                    status_code: 301,
                    ..Default::default()
                })
            }
            match dispatch_static(&req.path).await {
                Some(body) => Ok(HttpResponse{
                    body: body.into_bytes(),
                    ..Default::default()
                }),
                None => {
                    error!("Invalid test url '{}'", url);
                    Ok(HttpResponse::not_found())
                }
            }
        }

        (_, _) => {
            warn!("no route for this request: {:?}", req);
            Ok(HttpResponse::not_found())
        }
    }
}

// quick-and-dirty and not recommend method for embedding a static test suite
// a better way would be to use a back-end file-server capability provider
async fn dispatch_static(url: &str) -> Option<String> {
    match url {
        "/test/" => Some(include_str!("../js-test/index.html")),
        "/test/crowdsource.html" => Some(include_str!("../js-test/crowdsource.html")),
        "/test/css/mocha.css" => Some(include_str!("../js-test/css/mocha.css")),
        "/test/css/style.css" => Some(include_str!("../js-test/css/style.css")),
        "/test/js/crowdsource.js" => Some(include_str!("../js-test/js/crowdsource.js")),
        "/test/js/lib/chai-as-promised.js" => {
            Some(include_str!("../js-test/js/lib/chai-as-promised.js"))
        }
        "/test/js/lib/chai.js" => Some(include_str!("../js-test/js/lib/chai.js")),
        "/test/js/lib/jquery.js" => Some(include_str!("../js-test/js/lib/jquery.js")),
        "/test/js/lib/mocha.js" => Some(include_str!("../js-test/js/lib/mocha.js")),
        "/test/js/lib/q.js" => Some(include_str!("../js-test/js/lib/q.js")),
        "/test/js/lib/underscore.js" => Some(include_str!("../js-test/js/lib/underscore.js")),
        "/test/js/lib/underscore-min.map" => {
            Some(include_str!("../js-test/js/lib/underscore-min.map"))
        }
        "/test/js/setup.js" => Some(include_str!("../js-test/js/setup.js")),
        "/test/js/specs.js" => Some(include_str!("../js-test/js/specs.js")),
        _ => None,
    }
    .map(|s| s.to_string())
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
