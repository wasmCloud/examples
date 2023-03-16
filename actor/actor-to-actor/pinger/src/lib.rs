use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_pingpong::{Pingpong, PingpongSender};

const PONGER_ACTOR: &str = "wasmcloud/ponger";

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct PingerActor {}

#[async_trait]
impl HttpServer for PingerActor {
    async fn handle_request(&self, ctx: &Context, _req: &HttpRequest) -> RpcResult<HttpResponse> {
        let pong = PingpongSender::to_actor(PONGER_ACTOR).ping(ctx).await?;

        HttpResponse::ok(format!("Ping {pong}"))
    }
}
