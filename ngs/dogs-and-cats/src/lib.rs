use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpclient::{HttpClient, HttpClientSender, HttpRequest};
use wasmcloud_interface_httpserver::{
    HttpRequest as ServerHttpRequest, HttpResponse, HttpServer, HttpServerReceiver,
};
use wasmcloud_interface_numbergen::random_in_range;
mod helpers;

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
        // Randomly request a cat or dog image
        let api_response = match random_in_range(0, 1).await {
            Ok(0) => client.request(ctx, &HttpRequest::get(CAT_URL)).await?,
            Ok(1) | _ => client.request(ctx, &HttpRequest::get(DOG_URL)).await?,
        };
        let animal_image_url = helpers::parse_api_response(api_response);

        // Download the animal picture from the specified URL
        let img = client
            .request(ctx, &HttpRequest::get(&animal_image_url))
            .await?;
        Ok(HttpResponse {
            body: img.body,
            ..Default::default()
        })
    }
}
