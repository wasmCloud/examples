use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_pingpong::{Pingpong, PingpongReceiver};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, Pingpong)]
struct PongerActor {}

#[async_trait]
impl Pingpong for PongerActor {
    async fn ping(&self, _ctx: &Context) -> RpcResult<String> {
        Ok("pong".to_string())
    }
}
