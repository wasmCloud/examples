use keyvalue::*;
use serde::{Deserialize, Serialize};
use wasmbus_sender as wasmbus;

wit_bindgen_rust::export!("../keyvalue.wit");
wit_bindgen_rust::import!("../wasmbus-sender.wit");

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
struct IncrementRequest {
    /// name of value to increment
    #[serde(default)]
    pub key: String,
    /// amount to add to value
    #[serde(default)]
    pub value: i32,
}

#[derive(Default, Clone)]
pub struct Keyvalue;

impl keyvalue::Keyvalue for Keyvalue {
    fn increment(key: String, value: i32) -> Result<i32, RpcError> {
        let payload = serde_json::to_vec(&IncrementRequest { key, value })
            .map_err(|e| RpcError::Ser(e.to_string()))?;
        let resp = wasmbus::send(
            wasmbus::Message {
                method: "KeyValue.Increment",
                arg: &payload,
            },
            "wasmcloud:keyvalue",
            None,
        )
        .map_err(wasmbus_to_keyvalue_error)?;
        serde_json::from_slice(&resp).map_err(|e| RpcError::Deser(e.to_string()))
    }
}

fn wasmbus_to_keyvalue_error(e: wasmbus::RpcError) -> RpcError {
    match e {
        wasmbus::RpcError::ActorHandler(m) => RpcError::ActorHandler(m),
        wasmbus::RpcError::DeadlineExceeded(m) => RpcError::DeadlineExceeded(m),
        wasmbus::RpcError::Deser(m) => RpcError::Deser(m),
        wasmbus::RpcError::HostError(m) => RpcError::HostError(m),
        wasmbus::RpcError::InvalidParameter(m) => RpcError::InvalidParameter(m),
        wasmbus::RpcError::MethodNotHandled(m) => RpcError::MethodNotHandled(m),
        wasmbus::RpcError::Nats(m) => RpcError::Nats(m),
        wasmbus::RpcError::NotImplemented => RpcError::NotImplemented,
        wasmbus::RpcError::NotInitialized(m) => RpcError::NotInitialized(m),
        wasmbus::RpcError::Other(m) => RpcError::Other(m),
        wasmbus::RpcError::ProviderInit(m) => RpcError::ProviderInit(m),
        wasmbus::RpcError::Rpc(m) => RpcError::Rpc(m),
        wasmbus::RpcError::Ser(m) => RpcError::Ser(m),
        wasmbus::RpcError::Timeout(m) => RpcError::Timeout(m),
    }
}
