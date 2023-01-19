use std::{collections::HashMap, str::FromStr};

use http::{Method, StatusCode};
use tokio::sync::OnceCell;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_blobstore::{
    Blobstore, BlobstoreSender, Chunk, ContainerObject, GetObjectRequest, PutObjectRequest,
    RemoveObjectsRequest,
};
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_logging::info;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct BlobbyActor {}

static CONTAINER: OnceCell<String> = OnceCell::const_new();

lazy_static::lazy_static! {
    static ref ALLOW_HEADERS: HashMap<String, Vec<String>> = vec![
        (
            "Allow".to_string(),
            vec![[Method::GET.to_string(), Method::POST.to_string(), Method::PUT.to_string(), Method::DELETE.to_string()].join(", ")]
        )
    ].into_iter().collect();
}

/// A helper that will automatically create a container exactly once and return that container name
async fn container_name(ctx: &Context) -> RpcResult<&String> {
    CONTAINER
        .get_or_try_init(|| async {
            let container_name = "examples".to_string();
            let blobstore = BlobstoreSender::new();
            if !blobstore.container_exists(ctx, &container_name).await? {
                blobstore.create_container(ctx, &container_name).await?
            }
            Ok(container_name)
        })
        .await
}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for BlobbyActor {
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        let cleaned_id = req.path.trim_matches('/');
        // Check that there isn't any subpathing. If we can split, it means that we have more than
        // one path element
        if cleaned_id.split_once('/').is_some() {
            return Ok(HttpResponse::bad_request(
                "Cannot use a subpathed file name (e.g. foo/bar.txt)",
            ));
        }

        // Lazy error handling: Just unwrap to a method we don't support so we fall into the right
        // block below
        let method = Method::from_str(&req.method).unwrap_or(Method::HEAD);

        match method {
            Method::GET => get_object(ctx, cleaned_id).await,
            Method::POST | Method::PUT => {
                let content_type = req
                    .header
                    .get(http::header::CONTENT_TYPE.as_str())
                    .and_then(|vals| {
                        if !vals.is_empty() {
                            Some(vals[0].clone())
                        } else {
                            None
                        }
                    });
                put_object(ctx, cleaned_id, req.body.clone(), content_type).await
            }
            Method::DELETE => delete_object(ctx, cleaned_id).await,
            _ => Ok(HttpResponse {
                status_code: StatusCode::METHOD_NOT_ALLOWED.as_u16(),
                header: ALLOW_HEADERS.clone(),
                body: Vec::with_capacity(0),
            }),
        }
    }
}

async fn get_object(ctx: &Context, object_name: &str) -> RpcResult<HttpResponse> {
    let blobstore = BlobstoreSender::new();
    let container = container_name(ctx).await?.to_owned();
    // Check that the object exists first. If it doesn't return the proper http response
    if !blobstore
        .object_exists(
            ctx,
            &ContainerObject {
                container_id: container.clone(),
                object_id: object_name.to_owned(),
            },
        )
        .await?
    {
        return Ok(HttpResponse::not_found());
    }

    let get_object_request = GetObjectRequest {
        object_id: object_name.to_owned(),
        container_id: container,
        range_start: Some(0),
        range_end: None,
    };
    let o = blobstore.get_object(ctx, &get_object_request).await?;
    if !o.success {
        return Ok(HttpResponse {
            status_code: StatusCode::BAD_GATEWAY.as_u16(),
            body: o.error.unwrap_or_default().into_bytes(),
            ..Default::default()
        });
    }
    info!("successfully got an object!");
    let body = match o.initial_chunk {
        Some(c) => c.bytes,
        None => {
            return Ok(HttpResponse {
                status_code: StatusCode::BAD_GATEWAY.as_u16(),
                body: "Blobstore sent empty data chunk when full file was requested"
                    .as_bytes()
                    .to_vec(),
                ..Default::default()
            })
        }
    };

    Ok(HttpResponse::ok(body))
}

async fn delete_object(ctx: &Context, object_name: &str) -> RpcResult<HttpResponse> {
    let blobstore = BlobstoreSender::new();

    let mut res = blobstore
        .remove_objects(
            ctx,
            &RemoveObjectsRequest {
                container_id: container_name(ctx).await?.to_owned(),
                objects: vec![object_name.to_owned()],
            },
        )
        .await?;

    if !res.is_empty() {
        // SAFETY: We checked that the vec wasn't empty above
        let res = res.remove(0);
        return Ok(HttpResponse {
            status_code: StatusCode::BAD_GATEWAY.as_u16(),
            body: format!(
                "Error when deleting object from store: {}",
                res.error.unwrap_or_default()
            )
            .into_bytes(),
            ..Default::default()
        });
    }

    Ok(HttpResponse::ok(""))
}

async fn put_object(
    ctx: &Context,
    object_name: &str,
    data: Vec<u8>,
    content_type: Option<String>,
) -> RpcResult<HttpResponse> {
    let blobstore = BlobstoreSender::new();
    let container = container_name(ctx).await?.to_owned();

    blobstore
        .put_object(
            ctx,
            &PutObjectRequest {
                chunk: Chunk {
                    container_id: container,
                    object_id: object_name.to_owned(),
                    bytes: data,
                    offset: 0,
                    is_last: true,
                },
                content_type,
                ..Default::default()
            },
        )
        .await?;

    Ok(HttpResponse::ok(""))
}
