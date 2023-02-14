use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};

mod ui;
use ui::get_asset;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct AdsbUiActor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for AdsbUiActor {
    async fn handle_request(&self, _ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        let trimmed_path: Vec<&str> = req.path.trim_matches('/').split('/').collect();
        match (req.method.as_ref(), trimmed_path.as_slice()) {
            ("GET", asset_path) => get_asset(asset_path.join("/")),
            (_, _) => Ok(HttpResponse::not_found()),
        }
    }
}
