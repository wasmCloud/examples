use async_trait::async_trait;
use wasmbus_rpc::common::{Context, Message, MessageDispatch};
use wasmbus_rpc::error::RpcError;
use wasmbus_rpc::error::RpcResult;
use wasmcloud_interface_logging::debug;

use serde::{Deserialize, Serialize};

// must be in wasmcloud.toml for the responding actor otherwise the to_actor call will fail
pub const RESPONDER_ALIAS: &'static str = "responder";

// XRESPONDER methods provider for cross actor calls
pub const XRESPONDER_METHOD1: &'static str = "Call.Method1";
pub const XRESPONDER_METHOD2: &'static str = "Call.Method2";
// wasmbus-rpc capitalizes the first letter of the function name in the trait
pub const RESPONDER_METHOD1: &'static str = "Method1";
pub const RESPONDER_METHOD2: &'static str = "Method2";

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub from: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub message: String,
}

#[async_trait]
pub trait Call {
    fn contract_id() -> &'static str {
        "wasmcloud:example:cross"
    }
    async fn method1(&self, ctx: &Context, req: &Request) -> RpcResult<Response>;
    async fn method2(&self, ctx: &Context, req: &Request) -> RpcResult<Response>;
}

// This plugs our Call trait into wasmcloud so dispatch can be called
// This results in the first 'Call' that can be wash called
#[async_trait]
pub trait CallReceiver: MessageDispatch + Call {
    async fn dispatch(&self, ctx: &Context, message: Message<'_>) -> Result<Vec<u8>, RpcError> {
        debug!("message {:?}", message);
        // provide default values in case we want to wash call the responder
        let mut req: Request = Request {
            from: "ok".to_string(),
            message: "hello".to_string(),
        };
        req = bincode::deserialize(&message.arg).unwrap_or_else(|_e| req);
        match message.method {
            RESPONDER_METHOD1 => match self.method1(ctx, &req).await {
                Ok(a) => Ok(bincode::serialize(&a).unwrap()),
                Err(e) => Err(RpcError::Other(e.to_string())),
            },
            RESPONDER_METHOD2 => match self.method2(ctx, &req).await {
                Ok(a) => Ok(bincode::serialize(&a).unwrap()),
                Err(e) => Err(RpcError::Other(e.to_string())),
            },
            _ => Err(RpcError::NotImplemented),
        }
    }
}
