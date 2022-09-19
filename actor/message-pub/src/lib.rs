use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_messaging::{Messaging, MessagingSender, PubMessage};

const MESSAGE_PREFIX: &str = "wasmcloud.http";

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct MessagePubActor {}

#[async_trait]
impl HttpServer for MessagePubActor {
    async fn handle_request(
        &self,
        ctx: &Context,
        req: &HttpRequest,
    ) -> std::result::Result<HttpResponse, RpcError> {
        // Format the path as a dot separated subject instead
        let subject = format!(
            "{}{}",
            MESSAGE_PREFIX,
            req.path.replace('.', "_").replace('/', ".")
        );
        // Publish the body of the HTTP request in a Message
        if let Err(e) = MessagingSender::new()
            .publish(
                ctx,
                &PubMessage {
                    body: req.body.clone(),
                    reply_to: None,
                    subject: subject.to_owned(),
                },
            )
            .await
        {
            Err(format!("Could not publish message {}", e.to_string()).into())
        } else {
            Ok(HttpResponse {
                body: format!("Published on subject {}", subject).into_bytes(),
                ..Default::default()
            })
        }
    }
}
