use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::debug;

use types::{Call, CallReceiver, Request, Response};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, Call)]
struct Responder {}

#[async_trait]
impl Call for Responder {
    async fn method1(&self, _ctx: &Context, req: &Request) -> RpcResult<Response> {
        debug!("method1 received request {:?}", req);
        Ok(Response {
            message: format!("method1 was called from {} with {}", req.from, req.message),
        })
    }
    async fn method2(&self, _ctx: &Context, req: &Request) -> RpcResult<Response> {
        debug!("method2 received request {:?}", req);
        Ok(Response {
            message: format!("method2 was called from {} with {}", req.from, req.message),
        })
    }
}
