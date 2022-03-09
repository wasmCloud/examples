// This file is generated automatically using wasmcloud/weld-codegen 0.4.2

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

pub const SMITHY_VERSION: &str = "1.0";

/// Union of various data types
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum AnyValue {
    ValU8(u8),
    ValU16(u16),
    ValU32(u32),
    ValU64(u64),
    ValStr(String),
    ValF64(f64),
    ValBin(Vec<u8>),
}

// Encode AnyValue as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_any_value<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &AnyValue,
) -> RpcResult<()> {
    // encoding union AnyValue
    e.array(2)?;
    match val {
        AnyValue::ValU8(v) => {
            e.u16(0)?;
            e.u8(*v)?;
        }
        AnyValue::ValU16(v) => {
            e.u16(1)?;
            e.u16(*v)?;
        }
        AnyValue::ValU32(v) => {
            e.u16(2)?;
            e.u32(*v)?;
        }
        AnyValue::ValU64(v) => {
            e.u16(3)?;
            e.u64(*v)?;
        }
        AnyValue::ValStr(v) => {
            e.u16(4)?;
            e.str(v)?;
        }
        AnyValue::ValF64(v) => {
            e.u16(5)?;
            e.f64(*v)?;
        }
        AnyValue::ValBin(v) => {
            e.u16(6)?;
            e.bytes(v)?;
        }
    }
    Ok(())
}

// Decode AnyValue from cbor input stream
#[doc(hidden)]
pub fn decode_any_value(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<AnyValue, RpcError> {
    let __result = {
        // decoding union AnyValue
        let len = d.array()?.ok_or_else(|| {
            RpcError::Deser("decoding union 'AnyValue': indefinite array not supported".to_string())
        })?;
        if len != 2 {
            return Err(RpcError::Deser(
                "decoding union 'AnyValue': expected 2-array".to_string(),
            ));
        }
        match d.u16()? {
            0 => {
                let val = d.u8()?;
                AnyValue::ValU8(val)
            }

            1 => {
                let val = d.u16()?;
                AnyValue::ValU16(val)
            }

            2 => {
                let val = d.u32()?;
                AnyValue::ValU32(val)
            }

            3 => {
                let val = d.u64()?;
                AnyValue::ValU64(val)
            }

            4 => {
                let val = d.str()?.to_string();
                AnyValue::ValStr(val)
            }

            5 => {
                let val = d.f64()?;
                AnyValue::ValF64(val)
            }

            6 => {
                let val = d.bytes()?.to_vec();
                AnyValue::ValBin(val)
            }

            n => {
                return Err(RpcError::Deser(format!(
                    "invalid field number for union 'org.wasmcloud.example.union_demo#AnyValue':{}",
                    n
                )));
            }
        }
    };
    Ok(__result)
}
/// An error response contains an error message and optional stack trace
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ErrorResponse {
    #[serde(default)]
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stacktrace: Option<String>,
}

// Encode ErrorResponse as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_error_response<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ErrorResponse,
) -> RpcResult<()> {
    e.array(2)?;
    e.str(&val.message)?;
    if let Some(val) = val.stacktrace.as_ref() {
        e.str(val)?;
    } else {
        e.null()?;
    }
    Ok(())
}

// Decode ErrorResponse from cbor input stream
#[doc(hidden)]
pub fn decode_error_response(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ErrorResponse, RpcError> {
    let __result = {
        let mut message: Option<String> = None;
        let mut stacktrace: Option<Option<String>> = Some(None);

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct ErrorResponse, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct ErrorResponse: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => message = Some(d.str()?.to_string()),
                    1 => {
                        stacktrace = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }

                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct ErrorResponse: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "message" => message = Some(d.str()?.to_string()),
                    "stacktrace" => {
                        stacktrace = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    _ => d.skip()?,
                }
            }
        }
        ErrorResponse {
            message: if let Some(__x) = message {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ErrorResponse.message (#0)".to_string(),
                ));
            },
            stacktrace: stacktrace.unwrap(),
        }
    };
    Ok(__result)
}
/// response contains either a map, for success, or error, for failure
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Response {
    Values(ValueMap),
    Error(ErrorResponse),
}

// Encode Response as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_response<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &Response,
) -> RpcResult<()> {
    // encoding union Response
    e.array(2)?;
    match val {
        Response::Values(v) => {
            e.u16(0)?;
            encode_value_map(e, v)?;
        }
        Response::Error(v) => {
            e.u16(1)?;
            encode_error_response(e, v)?;
        }
    }
    Ok(())
}

// Decode Response from cbor input stream
#[doc(hidden)]
pub fn decode_response(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<Response, RpcError> {
    let __result = {
        // decoding union Response
        let len = d.array()?.ok_or_else(|| {
            RpcError::Deser("decoding union 'Response': indefinite array not supported".to_string())
        })?;
        if len != 2 {
            return Err(RpcError::Deser(
                "decoding union 'Response': expected 2-array".to_string(),
            ));
        }
        match d.u16()? {
            0 => {
                let val = decode_value_map(d).map_err(|e| format!("decoding 'ValueMap': {}", e))?;
                Response::Values(val)
            }

            1 => {
                let val = decode_error_response(d)
                    .map_err(|e| format!("decoding 'ErrorResponse': {}", e))?;
                Response::Error(val)
            }

            n => {
                return Err(RpcError::Deser(format!(
                    "invalid field number for union 'org.wasmcloud.example.union_demo#Response':{}",
                    n
                )));
            }
        }
    };
    Ok(__result)
}
/// Map a string key to an AnyValue
pub type ValueMap = std::collections::HashMap<String, AnyValue>;

// Encode ValueMap as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_value_map<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ValueMap,
) -> RpcResult<()> {
    e.map(val.len() as u64)?;
    for (k, v) in val {
        e.str(k)?;
        encode_any_value(e, v)?;
    }
    Ok(())
}

// Decode ValueMap from cbor input stream
#[doc(hidden)]
pub fn decode_value_map(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<ValueMap, RpcError> {
    let __result = {
        {
            let mut m: std::collections::HashMap<String, AnyValue> =
                std::collections::HashMap::default();
            if let Some(n) = d.map()? {
                for _ in 0..(n as usize) {
                    let k = d.str()?.to_string();
                    let v =
                        decode_any_value(d).map_err(|e| format!("decoding 'AnyValue': {}", e))?;
                    m.insert(k, v);
                }
            } else {
                return Err(RpcError::Deser("indefinite maps not supported".to_string()));
            }
            m
        }
    };
    Ok(__result)
}
/// The Runner interface has a single Run method
/// wasmbus.contractId: wasmcloud:example:union_demo
/// wasmbus.providerReceive
/// wasmbus.actorReceive
#[async_trait]
pub trait UnionDemo {
    /// returns the capability contract id for this interface
    fn contract_id() -> &'static str {
        "wasmcloud:example:union_demo"
    }
    async fn get<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<Response>;
}

/// UnionDemoReceiver receives messages defined in the UnionDemo service trait
/// The Runner interface has a single Run method
#[doc(hidden)]
#[async_trait]
pub trait UnionDemoReceiver: MessageDispatch + UnionDemo {
    async fn dispatch<'disp__, 'ctx__, 'msg__>(
        &'disp__ self,
        ctx: &'ctx__ Context,
        message: &Message<'msg__>,
    ) -> Result<Message<'msg__>, RpcError> {
        match message.method {
            "Get" => {
                let value: String = wasmbus_rpc::common::decode(&message.arg, &decode_string)
                    .map_err(|e| RpcError::Deser(format!("'String': {}", e)))?;
                let resp = UnionDemo::get(self, ctx, &value).await?;
                let mut e = wasmbus_rpc::cbor::vec_encoder(true);
                encode_response(&mut e, &resp)?;
                let buf = e.into_inner();
                Ok(Message {
                    method: "UnionDemo.Get",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "UnionDemo::{}",
                message.method
            ))),
        }
    }
}

/// UnionDemoSender sends messages to a UnionDemo service
/// The Runner interface has a single Run method
/// client for sending UnionDemo messages
#[derive(Debug)]
pub struct UnionDemoSender<T: Transport> {
    transport: T,
}

impl<T: Transport> UnionDemoSender<T> {
    /// Constructs a UnionDemoSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> UnionDemoSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl UnionDemoSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}

#[cfg(target_arch = "wasm32")]
impl UnionDemoSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for sending to a UnionDemo provider
    /// implementing the 'wasmcloud:example:union_demo' capability contract, with the "default" link
    pub fn new() -> Self {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_provider(
            "wasmcloud:example:union_demo",
            "default",
        )
        .unwrap();
        Self { transport }
    }

    /// Constructs a client for sending to a UnionDemo provider
    /// implementing the 'wasmcloud:example:union_demo' capability contract, with the specified link name
    pub fn new_with_link(link_name: &str) -> wasmbus_rpc::error::RpcResult<Self> {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_provider(
            "wasmcloud:example:union_demo",
            link_name,
        )?;
        Ok(Self { transport })
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> UnionDemo for UnionDemoSender<T> {
    #[allow(unused)]
    async fn get<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<Response> {
        let arg = arg.to_string();
        let mut e = wasmbus_rpc::cbor::vec_encoder(true);
        e.str(arg.as_ref())?;
        let buf = e.into_inner();
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "UnionDemo.Get",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: Response = wasmbus_rpc::common::decode(&resp, &decode_response)
            .map_err(|e| RpcError::Deser(format!("'{}': Response", e)))?;
        Ok(value)
    }
}
