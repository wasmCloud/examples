//! example actor demonstrating random number generation
//! in wasmcloud:builtin:numbergen

use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::debug;

use types::{Call, Request, Response, CallReceiver};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor,Call)]
struct Responder {}

#[async_trait]
impl Call for Responder {
    async fn call(&self, _ctx: &Context, req: &Request) -> RpcResult<Response> {
        debug!("received message {:?}", req);
        Ok(Response {
            message: "ohai".to_string(),
        })
    }
}
