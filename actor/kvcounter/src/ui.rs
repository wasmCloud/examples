use rust_embed::RustEmbed;
use wasmbus_rpc::actor::prelude::RpcResult;
use wasmcloud_interface_httpserver::HttpResponse;

/// This embeds the compiled static assets inside of the WebAssembly module
#[derive(RustEmbed)]
#[folder = "./ui/build"]
pub(crate) struct Asset;

pub(crate) fn get_asset(asset: String) -> RpcResult<HttpResponse> {
    let asset_request = if asset.trim() == "/" || asset.trim().is_empty() {
        "index.html"
    } else {
        asset.trim().trim_start_matches('/')
    };

    if let Some(static_asset) = Asset::get(asset_request) {
        Ok(HttpResponse {
            body: Vec::from(static_asset.data),
            ..Default::default()
        })
    } else {
        Ok(HttpResponse::not_found())
    }
}
