// This file is @generated by wasmcloud/weld-codegen 0.5.0.
// It is not intended for manual editing.
// namespace: org.wasmcloud.example.runner

#[allow(unused_imports)]
use async_trait::async_trait;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::{borrow::Borrow, borrow::Cow, io::Write, string::ToString};
#[allow(unused_imports)]
use wasmbus_rpc::{
    cbor::*,
    common::{
        deserialize, message_format, serialize, Context, Message, MessageDispatch, MessageFormat,
        SendOpts, Transport,
    },
    error::{RpcError, RpcResult},
    Timestamp,
};

#[allow(dead_code)]
pub const SMITHY_VERSION: &str = "1.0";

pub type StringList = Vec<String>;

// Encode StringList as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_string_list<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &StringList,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.array(val.len() as u64)?;
    for item in val.iter() {
        e.str(item)?;
    }
    Ok(())
}

// Decode StringList from cbor input stream
#[doc(hidden)]
pub fn decode_string_list(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<StringList, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<String> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(d.str()?.to_string())
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<String> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => arr.push(d.str()?.to_string()),
                }
            }
            arr
        }
    };
    Ok(__result)
}
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
    async fn dispatch(&self, ctx: &Context, message: Message<'_>) -> Result<Vec<u8>, RpcError> {
        match message.method {
            "Run" => {
                let value: StringList = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'StringList': {}", e)))?;

                let resp = Runner::run(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;

                Ok(buf)
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

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
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
        let buf = wasmbus_rpc::common::serialize(arg)?;

        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Runner.Run",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: StringList = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': StringList", e)))?;
        Ok(value)
    }
}
