wit_bindgen::generate!({
    world: "kvcounter",
    exports: {
        "wasi:http/incoming-handler": KvCounter
    }
});

use anyhow::{anyhow, Context};
use serde::Serialize;
use wasi::{
    http::http_types::{
        finish_outgoing_stream, incoming_request_method, incoming_request_path_with_query,
        new_fields, new_outgoing_response, outgoing_response_write, set_response_outparam, Method,
        ResponseOutparam,
    },
    io::streams::write,
    keyvalue::{
        readwrite::{get, set},
        types::{
            incoming_value_consume_sync, new_outgoing_value, open_bucket, outgoing_value_write_body,
        },
    },
};

mod ui;
use ui::get_static_asset;

use crate::exports::wasi::http::incoming_handler::{IncomingHandler, IncomingRequest};

// NOTE: custom buckets are not yet supported
const BUCKET: &str = "";

/// Implementation struct for the 'kvcounter' world (see: wit/kvcounter.wit)
struct KvCounter;

impl KvCounter {
    /// Increment (possibly negatively) the counter for a given key
    fn increment_counter(bucket: u32, key: &String, amount: i32) -> anyhow::Result<i32> {
        let current_value: i32 = match get(bucket, key) {
            // If the value exists, parse it into an i32
            Ok(incoming_value) => {
                // Read bytes from incoming value
                let bytes = incoming_value_consume_sync(incoming_value)
                    .map_err(|count| anyhow!("failed to parse incoming bytes, read [{count}]"))?;
                // Convert the bytes to a i32
                String::from_utf8(bytes)
                    .context("failed to parse string from returned bytes")?
                    .trim()
                    .parse()
                    .context("failed to parse i32 from bytes")?
            }
            // If the value is missing or we fail to get it, assume it is zero
            Err(_) => {
                eprintln!("[warn] encountered missing key [{key}], defaulting to 0");
                0
            }
        };

        // Calculate the new value
        let new_value: i32 = current_value + amount;

        // Build outgoing value to use
        let outgoing_value = new_outgoing_value();
        let stream =
            outgoing_value_write_body(outgoing_value).expect("failed to write outgoing value");

        // Write out the new value
        write(stream, new_value.to_string().as_bytes())
            .expect("failed to write to outgoing value stream");

        // Set the key to the updated value
        set(bucket, key, outgoing_value).expect("failed to set value");

        // Cheat and just assume the new value will be the increment
        Ok(new_value)
    }
}

/// Implementation of the WIT-driven incoming-handler interface for our implementation struct
impl IncomingHandler for KvCounter {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        // Decipher method
        let method = incoming_request_method(request);

        // Get path of request, then trim and split
        let request_path = http::uri::PathAndQuery::from_maybe_shared(
            incoming_request_path_with_query(request)
                .expect("failed to retrieve path and query from request"),
        )
        .expect("failed to parse path & query");
        let trimmed_path: Vec<&str> = request_path.path().trim_matches('/').split('/').collect();

        // Generate an outgoing request
        match (method, trimmed_path.as_slice()) {
            // GET /api/counter
            //
            //Retrieve value of the counter
            (Method::Get, ["api", "counter"]) => {
                // Retrieve the bucket
                // Retrieve bucket or return early with error
                let bucket = if let Ok(v) = open_bucket(BUCKET) {
                    v
                } else {
                    write_http_response(
                        response,
                        500,
                        &content_type_json(),
                        ApiResponse::error("failed to retreive bucket").into_vec(),
                    );
                    return;
                };

                // Increment the counter
                let updated_value =
                    match KvCounter::increment_counter(bucket, &String::from("default"), 1) {
                        Ok(v) => v,
                        Err(_) => {
                            write_http_response(
                                response,
                                500,
                                &content_type_json(),
                                ApiResponse::error("failed to increment default counter")
                                    .into_vec(),
                            );
                            return;
                        }
                    };

                // Build & write the response the response
                eprintln!("[success] successfully incremented default counter");
                write_http_response(
                    response,
                    200,
                    &content_type_json(),
                    ApiResponse::success(updated_value).into_vec(),
                )
            }

            // GET /api/counter/:counter_name
            //
            // Update a counter
            (Method::Get, ["api", "counter", counter]) => {
                // Retrieve bucket or return early with error
                let bucket = if let Ok(v) = open_bucket(BUCKET) {
                    v
                } else {
                    write_http_response(
                        response,
                        500,
                        &content_type_json(),
                        ApiResponse::error("failed to retreive bucket").into_vec(),
                    );
                    return;
                };

                // Increment the counter
                let updated_value =
                    match KvCounter::increment_counter(bucket, &counter.to_string(), 1) {
                        Ok(v) => v,
                        Err(e) => {
                            write_http_response(
                                response,
                                500,
                                &content_type_json(),
                                ApiResponse::error(format!("{e}")).into_vec(),
                            );
                            return;
                        }
                    };

                // Write out HTTP response
                eprintln!("[success] successfully incremented [{counter}] counter");
                write_http_response(
                    response,
                    200,
                    &content_type_json(),
                    ApiResponse::success(updated_value).into_vec(),
                );
            }

            // GET /*
            //
            // Any other GET request is interpreted as a static asset request for the UI
            (Method::Get, asset_path) => {
                let path = asset_path.join("/");
                match get_static_asset(&path) {
                    Ok((content_type, bytes)) => write_http_response(
                        response,
                        200,
                        &[("Content-Type".into(), content_type.into_bytes())],
                        bytes,
                    ),
                    Err(err) => {
                        eprintln!("[error] failed to retreive static asset @ [{path}]: {err:?}");
                        write_http_response(response, 404, &Vec::new(), "not found");
                    }
                };
            }

            // ???
            //
            // All other method + path combinations are unrecognized operations
            _ => write_http_response(
                response,
                400,
                &content_type_json(),
                ApiResponse::error("unrecognized operation").into_vec(),
            ),
        };
    }
}

/// Helper for writing a HTTP response out, using WIT-driven (WASI) interfaces
fn write_http_response(
    response_outparam: ResponseOutparam,
    status_code: u16,
    headers: &[(String, Vec<u8>)],
    body: impl AsRef<[u8]>,
) {
    // Add headers
    let headers = new_fields(headers);

    // Create new outgoing response and related stream
    let outgoing_response =
        new_outgoing_response(status_code, headers).expect("failed to create response");
    let outgoing_stream =
        outgoing_response_write(outgoing_response).expect("failed to write outgoing response");

    // Write out repsonse body to outgoing straem
    write(outgoing_stream, body.as_ref()).expect("failed to write output to stream");
    finish_outgoing_stream(outgoing_stream);

    // Set the response on the param
    set_response_outparam(response_outparam, Ok(outgoing_response))
        .expect("failed to set response");
}

/// The response that is sent by the API after an operation
#[derive(Debug, Serialize)]
pub enum ApiResponse {
    Error { error: String },
    Success { counter: i32 },
}

impl ApiResponse {
    /// Generate an error response
    fn error(error: impl AsRef<str>) -> Self {
        ApiResponse::Error {
            error: error.as_ref().to_string(),
        }
    }

    /// Generate an error response
    fn success(counter: i32) -> Self {
        ApiResponse::Success { counter }
    }

    /// Convert the ApiResponse into a bytes
    fn into_vec(self) -> Vec<u8> {
        serde_json::to_vec(&self).expect("failed to serialize API response")
    }
}

/// Helper that returns content type of a json response
fn content_type_json() -> [(String, Vec<u8>); 1] {
    [("Content-Type".into(), "application/json".into())]
}
