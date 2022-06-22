use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpclient::{HttpClient, HttpClientSender, HttpRequest};
use wasmcloud_interface_httpserver::{
    HttpRequest as ServerHttpRequest, HttpResponse, HttpServer, HttpServerReceiver,
};
use wasmcloud_interface_numbergen::random_in_range;
mod animalpic;

const CAT_URL: &str = "https://api.thecatapi.com/v1/images/search";
const DOG_URL: &str = "https://dog.ceo/api/breeds/image/random";

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct DogsAndCatsActor {}

#[async_trait]
impl HttpServer for DogsAndCatsActor {
    async fn handle_request(
        &self,
        ctx: &Context,
        _req: &ServerHttpRequest,
    ) -> Result<HttpResponse, RpcError> {
        let client = HttpClientSender::new();
        let api_response = match random_in_range(0, 1).await {
            Ok(0) => client.request(ctx, &HttpRequest::get(CAT_URL)).await?,
            Ok(1) | _ => client.request(ctx, &HttpRequest::get(DOG_URL)).await?,
        };
        let animal = animalpic::AnimalPic::from(api_response.body);
        let img = client
            .request(ctx, &HttpRequest::get(&animal.image_url))
            .await?;
        Ok(HttpResponse {
            body: img.body,
            ..Default::default()
        })
    }
}
