extern crate wapc_guest as guest;
use guest::prelude::*;
use wasmcloud_actor_blobstore as blob;
use wasmcloud_actor_core as actor;
use wasmcloud_actor_http_server as http;

#[actor::init]
fn init() {
    // Register handler for inbound HTTP request
    http::Handlers::register_handle_request(handle_request);
}

fn handle_request(req: http::Request) -> HandlerResult<http::Response> {
    match &*req.method {
        "GET" => download_image(),
        "POST" => upload_image(&req.path, &req.body),
        _ => Ok(http::Response::bad_request()),
    }
}

fn upload_image(path: &str, image_bytes: &[u8]) -> HandlerResult<http::Response> {
    let blobstore = blob::default();
    let container = blob::Container::new("wasmcloud-bucket".to_string());
    let image = blob::FileChunk {
        sequence_no: 0,
        container,
        id: path.to_string().replace("/", ""),
        total_bytes: image_bytes.len() as u64,
        chunk_size: image_bytes.len() as u64,
        chunk_bytes: image_bytes.to_vec(),
        context: None,
    };
    blobstore.start_upload(image.clone())?;
    blobstore.upload_chunk(image)?;
    Ok(http::Response::ok())
}

fn download_image() -> HandlerResult<http::Response> {
    Ok(http::Response::internal_server_error(
        "downloading not implemented",
    ))
}
