use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::error;
use wasmcloud_interface_messaging::*;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, MessageSubscriber)]
struct EchoMessagingActor {}

#[async_trait]
impl MessageSubscriber for EchoMessagingActor {
    /// handle subscription response
    async fn handle_message(&self, ctx: &Context, msg: &SubMessage) -> RpcResult<()> {
        if let Some(reply_to) = &msg.reply_to {
            let response = format!("reply: {}", &String::from_utf8_lossy(&msg.body));
            let provider = MessagingSender::new();
            if let Err(e) = provider
                .publish(
                    ctx,
                    &PubMessage {
                        body: response.as_bytes().to_vec(),
                        reply_to: None,
                        subject: reply_to.to_owned(),
                    },
                )
                .await
            {
                error!("sending reply: {}", e.to_string());
            }
        }
        Ok(())
    }
}
