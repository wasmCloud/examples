mod methods;

use crate::methods::{data::Oauth2Request, login::login, logincallback::login_callback};
use anyhow::{anyhow, Error};
use jammin_interfaces_apigw::*;
use std::clone::Clone;
use std::fmt::{Debug, Display};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::{debug, error, info, log, warn};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, RoutedSubscriber)]
struct PkceLogin {}

#[async_trait]

impl RoutedSubscriber for PkceLogin {
    async fn route(&self, ctx: &Context, msg: &RoutedRequest) -> RpcResult<RoutedResponse> {
        // extract social provider and grant type from RoutedRequest
        // SubMessage struct = pub struct SubMessage { pub body: Vec<u8>, pub reply_to: Option<String>, pub subject: String }
        // access msg.body -> deserialize -> set vector containing provider and grant type as login_params variable below.
        info!(
            "IN LOGIN ACTOR: {:?} || {:?}",
            msg,
            std::str::from_utf8(&msg.body)
        );

        let qs = serde_json::from_slice::<Oauth2Request>(&msg.body)
            .map_err(|e| tag_err("deserializing", e))?;
        info!(
            "Path: {}, Method: {}, Body: {:?},{:?}, Context: {:?}, Struct: {:?}",
            &msg.path.to_lowercase(),
            &msg.method.to_lowercase(),
            qs.provider,
            qs.grant,
            &qs.code,
            &qs.state,
        );

        let res = match msg.path.as_str() {
            "apilogin" => {
                let res = match login(&ctx, &qs, &msg.path).await {
                    Ok(res) => RoutedResponse {
                        path: msg.path.clone(),
                        success: true,
                        body: serde_json::to_string(&res).unwrap().as_bytes().to_vec(),
                        error: None,
                    },
                    Err(e) => RoutedResponse {
                        path: msg.path.clone(),
                        success: false,
                        body: serde_json::to_string("404").unwrap().as_bytes().to_vec(),
                        error: Some(stringify(e)),
                    },
                };
                res
            }
            "apilogincallback" => {
                let res = match login_callback(&ctx, &qs).await {
                    Ok(res) => RoutedResponse {
                        path: msg.path.clone(),
                        success: true,
                        // update body to return session, expiry, id
                        body: serde_json::to_string(&res.session)
                            .unwrap()
                            .as_bytes()
                            .to_vec(),
                        error: None,
                    },
                    Err(e) => RoutedResponse {
                        path: msg.path.clone(),
                        success: false,
                        body: serde_json::to_string("404").unwrap().as_bytes().to_vec(),
                        error: Some(stringify(e)),
                    },
                };
                res
            }
            &_ => RoutedResponse {
                path: msg.path.clone(),
                success: false,
                body: serde_json::to_string("error").unwrap().as_bytes().to_vec(),
                error: Some("404".to_string()),
            },
        };
        Ok(res)
    }
}

// helper function to give a little more information about where the error came from
fn tag_err<T: std::string::ToString>(msg: &str, e: T) -> RpcError {
    RpcError::ActorHandler(format!("{}: {}", msg, e.to_string()))
}

fn stringify<T: Display + Debug>(e: T) -> String {
    format!("{:?}", e)
}
