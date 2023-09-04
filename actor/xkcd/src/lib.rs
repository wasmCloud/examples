//!
//! This actor creates a simple web page with a random xkcd comic.
//!
//! The actor first selects a random comic number using the builtin number generator ,
//! then requests metadata for that comic from the xkcd site,
//! and generates and html page with the title and image url from the metadata.
//!
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::json;

use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpclient::{HttpClient, HttpClientSender, HttpRequest as CHttpRequest};
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_numbergen::random_in_range;

mod ui;
use ui::Asset;

// the highest numbered comic available. (around 2705 as of Sep 4, 2023)
// xkcd comics are numbered continuously starting at 1
const MAX_COMIC_ID: u32 = 2822;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct XkcdActor {}

/// implement HttpServer handle_request method
#[async_trait]
impl HttpServer for XkcdActor {
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        match req.path.trim_start_matches('/') {
            // Handle requests to retrieve comic data
            "comic" =>
            // all the work happens inside handle_inner.
            // The purpose of this wrapper is to catch any errors generated
            // by the inner function and turn them into a valid HttpResponse.
            {
                Ok(self
                    .handle_inner(ctx, req)
                    .await
                    .unwrap_or_else(|e| HttpResponse {
                        body: json!({ "error": e.to_string() }).to_string().into_bytes(),
                        status_code: 500,
                        ..Default::default()
                    }))
            }
            ui_asset_path => {
                let path = if ui_asset_path.is_empty() {
                    "index.html"
                } else {
                    ui_asset_path
                };
                // Request for UI asset
                Ok(Asset::get(path)
                    .map(|asset| {
                        let mut header = HashMap::new();
                        if let Some(content_type) = mime_guess::from_path(path).first() {
                            header
                                .insert("Content-Type".to_string(), vec![content_type.to_string()]);
                        }
                        HttpResponse {
                            status_code: 200,
                            header,
                            body: Vec::from(asset.data),
                        }
                    })
                    .unwrap_or_else(|| HttpResponse::not_found()))
            }
        }
    }
}

impl XkcdActor {
    async fn handle_inner(&self, ctx: &Context, _req: &HttpRequest) -> RpcResult<HttpResponse> {
        let comic_num = random_in_range(1, MAX_COMIC_ID).await?;

        // make a request to get the json metadata
        let url = format!("https://xkcd.com/{}/info.0.json", comic_num);
        let client = HttpClientSender::new();
        let resp = client
            .request(ctx, &CHttpRequest::get(&url))
            .await
            .map_err(|e| tag_err("sending req", e))?;
        if !(200..300).contains(&resp.status_code) {
            return Err(tag_err(
                "unexpected http status",
                resp.status_code.to_string(),
            ));
        }
        // Extract the 'title' and 'img' fields from the json response,
        // and build html page
        let info = serde_json::from_slice::<XkcdMetadata>(&resp.body)
            .map_err(|e| tag_err("decoding metadata", e))?;
        let resp = HttpResponse {
            body: serde_json::to_vec(&info).unwrap_or_default(),
            ..Default::default()
        };
        Ok(resp)
    }
}

/// Metadata returned as json
/// (this is a subset of the full metadata, but we only need two fields)
#[derive(Deserialize, Serialize)]
struct XkcdMetadata {
    title: String,
    img: String,
}

/// helper function to give a little more information about where the error came from
fn tag_err<T: std::string::ToString>(msg: &str, e: T) -> RpcError {
    RpcError::ActorHandler(format!("{}: {}", msg, e.to_string()))
}
