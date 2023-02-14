use rust_embed::RustEmbed;
use wasmbus_rpc::actor::prelude::RpcResult;
use wasmcloud_interface_httpserver::{HeaderMap, HttpResponse};

/// This embeds the compiled static assets inside of the WebAssembly module
#[derive(RustEmbed)]
#[folder = "./ui/dist"]
pub(crate) struct Asset;

pub(crate) fn get_asset(asset: String) -> RpcResult<HttpResponse> {
    let asset_request = if asset.trim() == "/" || asset.trim().is_empty() {
        "index.html"
    } else {
        asset.trim().trim_start_matches('/')
    };

    let mut header = HeaderMap::new();
    let content_type = mime_guess::from_path(asset_request)
        .first()
        .map(|m| m.to_string())
        .unwrap_or_else(|| "application/javascript".to_string());

    header.insert("Content-Type".to_string(), vec![content_type]);

    if let Some(static_asset) = Asset::get(asset_request) {
        Ok(HttpResponse {
            body: Vec::from(static_asset.data),
            header,
            ..Default::default()
        })
    } else {
        Ok(HttpResponse::not_found())
    }
}
