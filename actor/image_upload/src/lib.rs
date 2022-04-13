//! Image uploader
//! Actor that uploads images to S3 buckets using the blobstore interface.
//! Responds to HTTP GET and PUT requests:
//!    `curl server:8080/containers`   - list buckets
//!    `curl server:8080/images`       - list images in image bucket
//!    `curl -T file.jpg server:8080/image/file.jpg`   - upload file to bucket
//!   
use log::{error, info};
use serde::Serialize;
use serde_json::json;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_blobstore::{
    Blobstore, BlobstoreSender, Chunk, ContainerObject, ListObjectsRequest, PutObjectRequest,
};
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};

// LIMIT max size of data to avoid DOS
const MAX_IMAGE_SIZE: usize = 200 * 1024 * 1024; // 200MB

// bucket to use for storing images
// "alias_images" will be replaced with the linkdef-defined alias for 'images',
// if there is an alias defined, otherwise, the bucket name will be 'images'.
// If the bucket does not already exist, an error will occur
const IMAGE_BUCKET: &str = "alias_images";

#[allow(dead_code)]
#[allow(clippy::new_without_default)]
pub mod wasmcloud_interface_blobstore {
    include!(concat!(env!("OUT_DIR"), "/gen/blobstore.rs"));
}

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct ImageUploadActor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for ImageUploadActor {
    /// Returns a greeting, "Hello World", in the response body.
    /// If the request contains a query parameter 'name=NAME', the
    /// response is changed to "Hello NAME"
    async fn handle_request(
        &self,
        ctx: &Context,
        req: &HttpRequest,
    ) -> std::result::Result<HttpResponse, RpcError> {
        info!(
            "request: {} {} (content_length:{})",
            &req.method,
            &req.path,
            &req.body.len(),
        );
        let prov = BlobstoreSender::new();
        let path = req
            .path
            .trim_end_matches('/')
            .split('/')
            .skip(1)
            .collect::<Vec<&str>>();
        let method = req.method.to_ascii_uppercase();
        match (method.as_str(), &path[..]) {
            ("GET", ["containers"]) => {
                let containers = match prov.list_containers(ctx).await {
                    Ok(containers) => containers,
                    Err(e) => {
                        error!("failed to list containers: {}", e);
                        return Ok(HttpResponse::not_found());
                    }
                };
                let body = json!({ "containers": containers });
                let resp = match HttpResponse::json(body, 200) {
                    Ok(resp) => resp,
                    Err(e) => {
                        error!("failed to convert metadata to json: {}", e);
                        return Ok(HttpResponse::not_found());
                    }
                };
                Ok(resp)
            }
            ("GET", ["images"]) => {
                let objects = match prov
                    .list_objects(
                        ctx,
                        &ListObjectsRequest {
                            container_id: IMAGE_BUCKET.to_string(),
                            ..Default::default()
                        },
                    )
                    .await
                {
                    Ok(objects) => objects,
                    Err(e) => {
                        error!("failed to list objects: {}", e);
                        return Ok(HttpResponse::not_found());
                    }
                };
                let body = json!({ "images": objects.objects });
                let resp = match HttpResponse::json(body, 200) {
                    Ok(resp) => resp,
                    Err(e) => {
                        error!("failed to convert metadata to json: {}", e);
                        return Ok(HttpResponse::not_found());
                    }
                };
                Ok(resp)
            }
            ("PUT", ["image", name_hint]) => Ok(put_file(ctx, &prov, req, name_hint).await),
            ("PUT", ["image"]) => Ok(put_file(ctx, &prov, req, "_unnamed_.data").await),
            _ => {
                error!("invalid request {} {}", &req.method, &req.path);
                Ok(HttpResponse::not_found())
            }
        }
    }
}

async fn put_file<Tp: Transport + Sync>(
    ctx: &Context,
    prov: &BlobstoreSender<Tp>,
    req: &HttpRequest,
    name_hint: &str,
) -> HttpResponse {
    if req.body.len() > MAX_IMAGE_SIZE {
        return HttpResponse::bad_request("image too large");
    }
    if req.body.is_empty() {
        return HttpResponse::bad_request("no image");
    }
    let (md, mut errors) = compute_metadata(&req.body);
    let ext = if !md.guess_format.is_empty() {
        &md.guess_format
    } else if let Some((_, ext)) = name_hint.rsplit_once('.') {
        // we couldn't guess the format - use the extension provided
        ext
    } else {
        // no period in name
        errors.push("invalid_extension".to_string());
        "data"
    };
    let sha256 = hash(&req.body);
    let stored_name = format!("{}.{}", &sha256, ext);
    let content_type = extension_to_mime(ext);

    // don't bother uploading duplicates
    if let Ok(true) = prov
        .object_exists(
            ctx,
            &ContainerObject {
                container_id: IMAGE_BUCKET.to_string(),
                object_id: stored_name.clone(),
            },
        )
        .await
    {
        info!("file already exists: {}", &stored_name);
        HttpResponse::json(
            &PutImageResponse {
                // don't return metadata since it's not being computed
                metadata: None, // Some(md)
                sha256,
                original_name: name_hint.to_string(),
                stored_name,
                errors,
            },
            200,
        )
        .unwrap()
    } else {
        match prov
            .put_object(
                ctx,
                &PutObjectRequest {
                    chunk: Chunk {
                        container_id: IMAGE_BUCKET.to_string(),
                        object_id: stored_name.clone(),
                        // compiler should optimize this so it's move not copy
                        bytes: req.body.to_owned(),
                        offset: 0,
                        is_last: true,
                    },
                    content_type,
                    ..Default::default()
                },
            )
            .await
        {
            Ok(_) => {
                HttpResponse::json(
                    &PutImageResponse {
                        // don't return metadata since it's not being computed
                        metadata: None, // Some(md)
                        sha256,
                        original_name: name_hint.to_string(),
                        stored_name,
                        errors,
                    },
                    200,
                )
                .unwrap()
            }
            Err(e) => {
                let msg = format!("object_not_stored: {}: {}", name_hint, e);
                error!("{}", &msg);
                errors.push(msg);
                HttpResponse::json(
                    &PutImageResponse {
                        sha256,
                        original_name: name_hint.to_string(),
                        errors,
                        ..Default::default()
                    },
                    400,
                )
                .unwrap()
            }
        }
    }
}

// compute sha256 hash of data and return as lowercase hex
fn hash(data: &[u8]) -> String {
    use core::fmt::Write as _;
    use sha2::Digest as _;
    let hash = sha2::Sha256::digest(data);
    // convert to lowercase hex
    let mut s = String::with_capacity(2 * hash.len());
    for byte in hash.as_slice() {
        write!(s, "{:02x}", byte).unwrap();
    }
    s
}

#[derive(Debug, Default, Serialize)]
struct PutImageResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    errors: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    sha256: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    original_name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    stored_name: String,
}

/// A small amount of image metadata.
#[derive(Debug, Default, Serialize)]
struct Metadata {
    // format guessed from header bytes
    guess_format: String,
    // size of file
    size: u64,
    // width in pixels
    width: u32,
    // height in pixels
    height: u32,
    // true if image is in color
    has_color: bool,
    // color depth in bits
    bits_per_pixel: u16,
}

/// Compute simple metadata on image
fn compute_metadata(bytes: &[u8]) -> (Metadata, Vec<String>) {
    let mut md = Metadata::default();

    let mut errors = Vec::new();
    match image::guess_format(bytes) {
        Ok(format) => {
            md.guess_format = if !format.extensions_str().is_empty() {
                // use first-listed extension, if available
                // (api says this array might be empty)
                format.extensions_str()[0].to_string()
            } else {
                format!("{:?}", &format).to_ascii_lowercase()
            };
        }
        Err(e) => {
            errors.push(format!("guess_format: {}", e));
        }
    }
    /*
    // this hasn't been tested, so leaving disabled for now
    match image::load_from_memory(bytes) {
        Ok(dyn_image) => {
            md.width = dyn_image.width();
            md.height = dyn_image.height();
            md.has_color = dyn_image.color().has_color();
            md.bits_per_pixel = dyn_image.color().bits_per_pixel();
        }
        Err(e) => {
            errors.push(format!("load_image: {}", e.to_string()));
        }
    }
    */
    (md, errors)
}

fn extension_to_mime(ext: &str) -> Option<String> {
    // ImageFormat has from_mime but not to_mime,
    // this is based on ImageFormat defined types
    match ext {
        "avif" => Some("image/avif".to_string()),
        "jpg" | "jpeg" => Some("image/jpeg".to_string()),
        "png" => Some("image/png".to_string()),
        "gif" => Some("image/gif".to_string()),
        "webp" => Some("image/webp".to_string()),
        "tiff" => Some("image/tiff".to_string()),
        "tga" => Some("image/x-targa".to_string()),
        "dds" => Some("image/vnd-ms.dds".to_string()),
        "bmp" => Some("image/bmp".to_string()),
        "ico" => Some("image/x-icon".to_string()),
        "hdr" => Some("image/vnd.radiance".to_string()),
        "openexr" => Some("image/x-exr".to_string()),
        "pnm" => Some("image/x-portable-bitmap".to_string()),
        _ => None,
    }
}
