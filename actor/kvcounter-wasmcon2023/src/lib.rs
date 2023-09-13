wit_bindgen::generate!({
    world: "kvcounter",
    exports: {
        "wasi:http/incoming-handler": KvCounter
    }
});

use anyhow::anyhow;
use http::StatusCode;
use serde::Serialize;
use wasi::{
    http::http_types::{
        finish_outgoing_stream, incoming_request_method, incoming_request_path_with_query,
        new_fields, new_outgoing_response, outgoing_response_write, set_response_outparam, Method,
        ResponseOutparam,
    },
    io::streams::write,
    keyvalue::{atomic::increment, types::open_bucket},
};

mod ui;
use ui::get_static_asset;

use crate::exports::wasi::http::incoming_handler::{Guest, IncomingRequest};

// NOTE: custom buckets are not yet supported
const BUCKET: &str = "";

/// Implementation struct for the 'kvcounter' world (see: wit/kvcounter.wit)
struct KvCounter;

/// Implementation of the WIT-driven incoming-handler interface for our implementation struct
impl Guest for KvCounter {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        // Decipher method
        let method = incoming_request_method(request);

        // Get path and query params on the incoming request
        let request_path = match incoming_request_path_with_query(request) {
            Some(p) => p,
            None => {
                write_http_response(
                    response,
                    StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    &[],
                    ApiResponse::error("failed to parse request path").into_vec(),
                );
                eprintln!("[error] failed to retrieve path and query from request");
                return;
            }
        };

        // Parse the path & query into a known type
        let request_path = match http::uri::PathAndQuery::from_maybe_shared(request_path) {
            Ok(pnq) => pnq,
            Err(e) => {
                write_http_response(
                    response,
                    StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    &[],
                    ApiResponse::error(format!("failed to parse path & query: {e}")).into_vec(),
                );
                return;
            }
        };

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
                        ApiResponse::error("failed to retrieve bucket").into_vec(),
                    );
                    return;
                };

                // Increment the counter
                let updated_value = match increment(bucket, &String::from("default"), 1)
                    .map_err(|_| anyhow!("failed to increment value in bucket"))
                {
                    Ok(v) => v,
                    Err(_) => {
                        write_http_response(
                            response,
                            500,
                            &content_type_json(),
                            ApiResponse::error("failed to increment default counter").into_vec(),
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
                        ApiResponse::error("failed to retrieve bucket").into_vec(),
                    );
                    return;
                };

                // Increment the counter
                let updated_value = match increment(bucket, &counter.to_string(), 1)
                    .map_err(|_| anyhow!("failed to increment value in bucket"))
                {
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
                        eprintln!("[error] failed to retrieve static asset @ [{path}]: {err:?}");
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
#[serde(untagged)]
pub enum ApiResponse {
    Error { error: String },
    Success { counter: u64 },
}

impl ApiResponse {
    /// Generate an error response
    fn error(error: impl AsRef<str>) -> Self {
        ApiResponse::Error {
            error: error.as_ref().to_string(),
        }
    }

    /// Generate an error response
    fn success(counter: u64) -> Self {
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
