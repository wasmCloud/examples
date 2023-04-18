#![allow(unused_imports)]
use jammin_interfaces_apigw::*;
use strum::EnumString;
use wasmbus_rpc::actor::prelude::*;
use wasmbus_rpc::error::RpcError;
use wasmcloud_interface_logging::{debug, error, info, log, warn};

#[derive(Clone, Debug, PartialEq, EnumString)]
pub enum Router {
    #[strum(ascii_case_insensitive)]
    ApiLogin,
    #[strum(ascii_case_insensitive)]
    ApiLoginCallback,
    #[strum(ascii_case_insensitive)]
    None,
}

impl Router {
    pub async fn send(&self, ctx: &Context, req: &RoutedRequest) -> RpcResult<RoutedResponse> {
        info!(
            "IN ROUTER IN ROUTER IN ROUTER Route: {:?} || Method: {:?}",
            self,
            req.method.as_str()
        );

        let res = match self {
            // User Flow - User interaction with auth_url needed.
            Router::ApiLogin => {
                // Route to login actor, there will be different routes for different login types based on query parameter
                // may need nested match of if statement for each method that handles various login types routing to specific actors
                RoutedSubscriberSender::to_actor("login_pkce")
                    .route(ctx, req)
                    .await?
            }
            Router::ApiLoginCallback => {
                // Route to login actor, there will be different routes for different login types
                RoutedSubscriberSender::to_actor("login_pkce")
                    .route(ctx, req)
                    .await?
            }
            Router::None => match req.method.as_str() {
                _ => Err(RpcError::MethodNotHandled(req.method.clone()))?,
            },
        };
        Ok(res)
    }
}
