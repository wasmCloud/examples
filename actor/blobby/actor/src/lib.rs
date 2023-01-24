use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    str::FromStr,
};

use http::{Method, StatusCode};
use tokio::sync::RwLock;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_blobstore::{
    Blobstore, BlobstoreSender, Chunk, ContainerObject, GetObjectRequest, PutObjectRequest,
    RemoveObjectsRequest,
};
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_logging::info;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct BlobbyActor {
    initialized_containers: RwLock<HashSet<String>>,
}

lazy_static::lazy_static! {
    static ref ALLOW_HEADERS: HashMap<String, Vec<String>> = vec![
        (
            http::header::ALLOW.to_string(),
            vec![[Method::GET.to_string(), Method::POST.to_string(), Method::PUT.to_string(), Method::DELETE.to_string()].join(", ")]
        )
    ].into_iter().collect();
}

const DEFAULT_CONTAINER_NAME: &str = "default";
const CONTAINER_HEADER_NAME: &str = "blobby-container";
const CONTAINER_PARAM_NAME: &str = "container";

impl BlobbyActor {
    /// A helper that will automatically create a container if it doesn't exist and returns an owned copy of the name for immediate use
    async fn ensure_container(&self, ctx: &Context, name: &str) -> RpcResult<String> {
        let owned_name = name.to_owned();
        if !self.initialized_containers.read().await.contains(name) {
            let blobstore = BlobstoreSender::new();
            if !blobstore.container_exists(ctx, &owned_name).await? {
                blobstore.create_container(ctx, &owned_name).await?
            }
            self.initialized_containers
                .write()
                .await
                .insert(owned_name.clone());
        }
        Ok(owned_name)
    }
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
        let cleaned_id = cleaned_id.to_owned();

        // Get the container name from the request
        let container_name = self.ensure_container(ctx, &get_container_name(req)).await?;

        // Lazy error handling: Just unwrap to a method we don't support so we fall into the right
        // block below
        let method = Method::from_str(&req.method).unwrap_or(Method::HEAD);

        match method {
            Method::GET => get_object(ctx, container_name, cleaned_id).await,
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
                put_object(
                    ctx,
                    container_name,
                    cleaned_id,
                    req.body.clone(),
                    content_type,
                )
                .await
            }
            Method::DELETE => delete_object(ctx, container_name, cleaned_id).await,
            _ => Ok(HttpResponse {
                status_code: StatusCode::METHOD_NOT_ALLOWED.as_u16(),
                header: ALLOW_HEADERS.clone(),
                body: Vec::with_capacity(0),
            }),
        }
    }
}

async fn get_object(
    ctx: &Context,
    container_name: String,
    object_name: String,
) -> RpcResult<HttpResponse> {
    let blobstore = BlobstoreSender::new();
    // Check that the object exists first. If it doesn't return the proper http response
    if !blobstore
        .object_exists(
            ctx,
            &ContainerObject {
                container_id: container_name.clone(),
                object_id: object_name.clone(),
            },
        )
        .await?
    {
        return Ok(HttpResponse::not_found());
    }

    let get_object_request = GetObjectRequest {
        object_id: object_name,
        container_id: container_name,
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

    let headers = o
        .content_type
        .map(|c| {
            vec![(http::header::CONTENT_TYPE.to_string(), vec![c])]
                .into_iter()
                .collect::<HashMap<String, Vec<String>>>()
        })
        .unwrap_or_default();

    Ok(HttpResponse {
        body,
        header: headers,
        ..Default::default()
    })
}

async fn delete_object(
    ctx: &Context,
    container_name: String,
    object_name: String,
) -> RpcResult<HttpResponse> {
    let blobstore = BlobstoreSender::new();

    let mut res = blobstore
        .remove_objects(
            ctx,
            &RemoveObjectsRequest {
                container_id: container_name,
                objects: vec![object_name],
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
    container_name: String,
    object_name: String,
    data: Vec<u8>,
    content_type: Option<String>,
) -> RpcResult<HttpResponse> {
    let blobstore = BlobstoreSender::new();

    blobstore
        .put_object(
            ctx,
            &PutObjectRequest {
                chunk: Chunk {
                    container_id: container_name,
                    object_id: object_name,
                    bytes: data,
                    offset: 0,
                    is_last: true,
                },
                content_type,
                ..Default::default()
            },
        )
        .await?;

    Ok(HttpResponse::ok(Vec::with_capacity(0)))
}

// Gets the container name from the header or a query param. The query param takes precedence
fn get_container_name(req: &HttpRequest) -> Cow<'_, str> {
    if let Some(param) = form_urlencoded::parse(req.query_string.as_bytes())
        .find(|(n, _)| n == CONTAINER_PARAM_NAME)
        .map(|(_, v)| v)
    {
        param
    } else if let Some(header) = req.header.get(CONTAINER_HEADER_NAME).and_then(|vals| {
        // Not using `vals.get` because you can't return data owned by the current function
        if !vals.is_empty() {
            // There should only be one, but if there are more than one, only grab the first one
            Some(Cow::from(vals[0].as_str()))
        } else {
            None
        }
    }) {
        header
    } else {
        Cow::from(DEFAULT_CONTAINER_NAME)
    }
}
