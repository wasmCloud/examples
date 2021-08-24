//!
//! This actor creates a simple web page with a random xkcd comic.
//!
//! The actor first selects a random comic number using the builtin number generator ,
//! then requests metadata for that comic from the xkcd site,
//! and generates and html page with the title and image url from the metadata.
//!
use serde::Deserialize;
use serde_json::json;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpclient::{HttpClient, HttpClientSender, HttpRequest as CHttpRequest};
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_numbergen::random_in_range;

// the highest numbered comic available. (around 2500 as of August 2021)
// xkcd comics are numbered continuously starting at 1
const MAX_COMIC_ID: u32 = 2500;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct XkcdActor {}

/// implement HttpServer handle_request method
#[async_trait]
impl HttpServer for XkcdActor {
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        // all the work happens inside handle_inner.
        // The purpose of this wrapper is to catch any errors generated
        // by the inner function and turn them into a valid HttpResponse.
        Ok(self
            .handle_inner(ctx, req)
            .await
            .unwrap_or_else(|e| HttpResponse {
                body: json!({ "error": e.to_string() }).to_string().into_bytes(),
                status_code: 500,
                ..Default::default()
            }))
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
        let html = format!(
            r#"<!DOCTYPE html>
        <html>
        <head>
            <title>Your XKCD random comic</title>
        </head>
        <body>
            <h1>{}</h1>
            <img src="{}"/>
        </body>
        </html>
        "#,
            &info.title, &info.img
        );
        let resp = HttpResponse {
            body: html.into_bytes(),
            ..Default::default()
        };
        Ok(resp)
    }
}

/// Metadata returned as json
/// (this is a subset of the full metadata, but we only need two fields)
#[derive(Deserialize)]
struct XkcdMetadata {
    title: String,
    img: String,
}

/// helper function to give a little more information about where the error came from
fn tag_err<T: std::string::ToString>(msg: &str, e: T) -> RpcError {
    RpcError::ActorHandler(format!("{}: {}", msg, e.to_string()))
}
