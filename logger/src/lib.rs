extern crate wasmcloud_actor_core as core;
extern crate wasmcloud_actor_http_server as http;
extern crate wasmcloud_actor_logging as logging;
use log::{debug, error, info, warn};
use wapc_guest::HandlerResult;

#[no_mangle]
pub fn wapc_init() {
    http::Handlers::register_handle_request(method_logger);
    core::Handlers::register_health_request(health);
    logging::enable_macros();
}

/// Actor must be signed with `wasmcloud:logging` to log messages
fn method_logger(msg: http::Request) -> HandlerResult<http::Response> {
    logging::default().write_log("LOGGING_ACTORINFO", "info", "Coercing Rust String to str")?;
    match &*msg.method {
        "GET" => info!(target: "GETLOG", "Received a GET request"),
        "POST" => info!("Received a POST request"),
        "PUT" => info!("Received a PUT request"),
        "DELETE" => warn!(target: "SYSTEM_WARNINGS", "Received a DELETE request"),
        req => error!("Received an unsupported HTTP Request: {}", req),
    };
    debug!(target: "LOGGING_ACTORINFO", "Finished matching HTTP method, returning OK");
    Ok(http::Response::ok())
}

fn health(_h: core::HealthCheckRequest) -> HandlerResult<core::HealthCheckResponse> {
    Ok(core::HealthCheckResponse::healthy())
}
