use async_trait::async_trait;
use wasmbus_rpc::common::{Context, Message, MessageDispatch};
use wasmbus_rpc::error::RpcError;
use wasmbus_rpc::error::RpcResult;

pub const ALIAS: &'static str = "responder";

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub message: String,
}

#[derive(Debug)]
pub struct Response {
    pub message: String,
}

#[async_trait]
pub trait Call {
    fn contract_id() -> &'static str {
        "wasmcloud:example:cross"
    }
    async fn call(&self, ctx: &Context, req: &Request) -> RpcResult<Response>;
}

#[async_trait]
pub trait CallReceiver: MessageDispatch + Call {
    async fn dispatch(&self, ctx: &Context, message: Message<'_>) -> Result<Vec<u8>, RpcError> {
        match self
            .call(
                ctx,
                &Request {
                    method: message.method.to_string(),
                    message: "ok".to_string(),
                },
            )
            .await
        {
            Ok(a) => Ok(a.message.as_bytes().to_vec()),
            Err(_e) => Ok(vec![]),
        }
    }
}
