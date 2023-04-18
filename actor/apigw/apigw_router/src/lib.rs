#![allow(unused_imports)]
pub mod methods;

use crate::methods::parse_http_req::*;
use crate::methods::router::*;
use jammin_interfaces_apigw::*;
use serde_urlencoded;
use std::fmt::{Debug, Display};
use std::str::FromStr;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_logging::{debug, error, info, log, warn};

const BASE_URL: &str = "http://jamminmusic.dev";

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct ApiGwActor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for ApiGwActor {
    async fn handle_request(&self, _ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        info!("req: {:?}", req);

        let rreq = Method::from_str(&req.method.as_str())
            .unwrap()
            .create_request(&_ctx, &req)
            .await
            .unwrap();

        info!("msg: {:?}", rreq);

        if let Ok(router) = Router::from_str(&rreq.path.as_str()) {
            let response = router.send(_ctx, &rreq).await.map_err(stringify)?;
            if response.error == None {
                // info!("Response: {:?}", response.body);
                Ok(HttpResponse {
                    status_code: 200,
                    body: response.body,
                    ..Default::default()
                })
            } else {
                Ok(HttpResponse {
                    status_code: 500,
                    body: serde_urlencoded::to_string(response.error.unwrap_or_default())
                        .unwrap()
                        .as_bytes()
                        .to_vec(),
                    ..Default::default()
                })
            }
        } else {
            let response = RoutedResponse {
                path: rreq.path,
                success: false,
                body: vec![0; 0],
                error: Some("Path Does Not Exist".to_string()),
            };
            // info!("Response: {:?}", response.body);
            Ok(HttpResponse {
                status_code: 404,
                body: response.body,
                ..Default::default()
            })
        }
    }
}

fn stringify<T: Display + Debug>(e: T) -> String {
    format!("{:?}", e)
}
