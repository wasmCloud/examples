// This file is generated automatically using wasmcloud-weld and smithy model definitions
//

#![allow(clippy::ptr_arg)]
#[allow(unused_imports)]
use async_trait::async_trait;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::{borrow::Cow, string::ToString};
#[allow(unused_imports)]
use wasmbus_rpc::{
    deserialize, serialize, Context, Message, MessageDispatch, RpcError, RpcResult, SendOpts,
    Transport,
};

pub const SMITHY_VERSION: &str = "1.0";

pub type StringList = Vec<String>;

/// The Runner interface has a single Run method
/// wasmbus.contractId: wasmcloud:example:runner
/// wasmbus.actorReceive
#[async_trait]
pub trait Runner {
    /// returns the capability contract id for this interface
    fn contract_id() -> &'static str {
        "wasmcloud:example:runner"
    }
    /// The Run operation takes an array of strings and returns an array of strings.
    /// The interpretation of the inputs, and the meaning of the outputs,
    /// is dependent on the implementation.
    /// Either input or output arrays may be empty.
    async fn run(&self, ctx: &Context, arg: &StringList) -> RpcResult<StringList>;
}

/// RunnerReceiver receives messages defined in the Runner service trait
/// The Runner interface has a single Run method
#[doc(hidden)]
#[async_trait]
pub trait RunnerReceiver: MessageDispatch + Runner {
    async fn dispatch(&self, ctx: &Context, message: &Message<'_>) -> RpcResult<Message<'_>> {
        match message.method {
            "Run" => {
                let value: StringList = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = Runner::run(self, ctx, &value).await?;
                let buf = Cow::Owned(serialize(&resp)?);
                Ok(Message {
                    method: "Runner.Run",
                    arg: buf,
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "Runner::{}",
                message.method
            ))),
        }
    }
}

/// RunnerSender sends messages to a Runner service
/// The Runner interface has a single Run method
/// client for sending Runner messages
#[derive(Debug)]
pub struct RunnerSender<T: Transport> {
    transport: T,
}

impl<T: Transport> RunnerSender<T> {
    /// Constructs a RunnerSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> RunnerSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl RunnerSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> Runner for RunnerSender<T> {
    #[allow(unused)]
    /// The Run operation takes an array of strings and returns an array of strings.
    /// The interpretation of the inputs, and the meaning of the outputs,
    /// is dependent on the implementation.
    /// Either input or output arrays may be empty.
    async fn run(&self, ctx: &Context, arg: &StringList) -> RpcResult<StringList> {
        let arg = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Runner.Run",
                    arg: Cow::Borrowed(&arg),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "Run", e)))?;
        Ok(value)
    }
}
