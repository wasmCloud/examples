//! Echo Actor component, which compiles to WASI Preview 2

// wit-bindgen generates the traits and implementation that
// power WIT-based WASM development.
//
// To see what wit-bindgen generates, use [cargo expand](https://github.com/dtolnay/cargo-expand)
wit_bindgen::generate!("echo");

use std::io::{stdin, stdout, Write};

use serde_json::json;
use wasmcloud_actor::{debug, error};
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse};

/// Operation used by wasmCloud httpserver capability contract
/// for handling a single request
/// see [wasmCloud/interfaces httpserver](https://github.com/wasmCloud/interfaces/tree/main/httpserver#-rust)
const OPERATION_HTTPSERVER_HANDLE_REQUEST: &str = "HttpServer.HandleRequest";

/// Echo (HTTP) actor
///
/// This unit struct will hold all implementations of
/// traits generated the by WIT contract (see echo-wasi-preview2/wit/echo.wit)
struct EchoActor;

/// Implementation of a wasmCloud guest module (guest module) that can
/// receive messages (RPCs) from a wasmCloud lattice.
///
/// NOTE: this implementation follows closely the tests written
/// for wasmcloud/wasmcloud, specific to reactor components.
/// https://github.com/wasmCloud/wasmCloud/blob/main/tests/actors/rust/builtins-component-reactor/src/lib.rs
impl exports::wasmcloud::bus::guest::Guest for EchoActor {
    fn call(operation: String) -> Result<(), String> {
        // Ensure the right operation
        if operation != OPERATION_HTTPSERVER_HANDLE_REQUEST {
            error!("received unexpected operation [{operation}]");
            return Err(format!("invalid operation [{operation}]"));
        }

        // Parse the request from stdin, as requests are streamed in via STDIN
        let HttpRequest {
            method,
            path,
            query_string,
            header,
            body,
        } = rmp_serde::from_read(stdin()).map_err(|e| format!("failed to parse request: {e}"))?;
        debug!("received & parsed request ({method} {path})");

        // Build and serialize the echo response
        let body = serde_json::to_vec(&json!({
            "method": &method,
            "path": &path,
            "headers": &header,
            "query_string": &query_string,
            "body": &body,
        }))
            .map_err(|e| format!("failed to encode request body: {e}"))?;

        // Encode the HttpResponse (messagepack/CBOR) to match compatibility
        // with wasmcloud
        let response = rmp_serde::to_vec(&HttpResponse {
            body: body,
            ..HttpResponse::default()
        })
            .map_err(|e| format!("failed to serialize HTTP response: {e}"))?;
        debug!("built & serialize response ({method} {path})");

        // Write the response to STDOUT
        let mut stdout = stdout();
        stdout
            .lock()
            .write_all(&response)
            .map_err(|e| format!("failed to write response bytes: {e}"))?;
        stdout
            .flush()
            .map_err(|e| format!("failed to flush stdout: {e}"))?;

        Ok(())
    }
}

// Export the actor's WIT binding
export_echo!(EchoActor);
