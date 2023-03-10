//! WASI keyvalue interface implementation for wasmCloud
//!
//! This crate provides a usable guest (WASM component) implementation of the
//! WASI keyvalue interface (https://github.com/WebAssembly/wasi-keyvalue)
//!
//! This component expects to be provided with implementation for wasmcloud interaction
//! and fine-grained streams/memory management (via types). The default world is as follows:
//!
//! ```
//! default world keyvalue {
//!   import host: pkg.wasmcloud.host
//!   import types: pkg.types
//!   export keyvalue: self.wasmcloud-kv
//! }
//! ```

wit_bindgen::generate!({
    path: "../wit",
    world: "keyvalue",
});

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::hash::Hash;
use std::str::from_utf8;
use std::sync::Mutex;
use std::sync::RwLock;

// use crate::keyvalue::{Bucket, Error};
//use crate::types::{incoming_value_consume_sync, String, String};

use crate::host::host_call as wasmcloud_host_call;

use wasmcloud_interface_keyvalue::{GetResponse, SetRequest};

/////////////////
// Static Data //
/////////////////

const WASMCLOUD_DEFAULT_BINDING: &'static str = "default";
const WASMCLOUD_KV_NAMESPACE: &'static str = "wasmcloud:keyvalue";

/// The named key could not be found
const ERROR_NO_SUCH_KEY: &str = "400";

/// Unexpected Internal Error
const ERROR_INTERNAL: &str = "500";
/// Internal map space has been exhausted
const ERROR_INTERNAL_MAP_SPACE_EXHAUSTED: &str = "501";
/// An Error occurred while trying to access internal memory
const ERROR_INTERNAL_MEMORY_ACCESS_ERROR: &str = "502";
/// An error occurred wiht the upstream keyvalue store
const ERROR_INTERNAL_UPSTREAM_OPERATION_FAILURE: &str = "503";
const ERROR_INTERNAL_PROVIDER_REQUEST_SERIALIZATION_FAILURE: &str = "505";
const ERROR_INTERNAL_PROVIDER_RESPONSE_DESERIALIZATION_FAILURE: &str = "506";

/// This struct implements WASI interfaces
struct KvProvider {}

///////////////////////////////////////
// KeyValue Interface Implementation //
///////////////////////////////////////

impl keyvalue::Keyvalue for KvProvider {
    /// Get a value from the key value store
    fn get(key: String) -> Result<String, String> {
        println!("[debug][kv-provider] performing get...");

        let payload = wasmbus_rpc::common::serialize(&key)
            .map_err(|_| String::from(ERROR_INTERNAL_PROVIDER_REQUEST_SERIALIZATION_FAILURE))?;

        match wasmcloud_host_call(
            WASMCLOUD_DEFAULT_BINDING, // "default"
            WASMCLOUD_KV_NAMESPACE, // "wasmcloud:keyvalue"
            "KeyValue.Get",
            Some(&payload),
        ) {
            Ok(Some(resp_bytes)) => {

                // Parse the get response
                let resp = wasmbus_rpc::common::deserialize(&resp_bytes)
                    .map_err(|_| String::from(ERROR_INTERNAL_PROVIDER_RESPONSE_DESERIALIZATION_FAILURE))?;

                match resp {
                    // TODO(QUESTION) what do they expect?
                    GetResponse { exists: false, .. } => Err(String::from(ERROR_NO_SUCH_KEY)),
                    GetResponse {
                        exists: true,
                        value,
                    } => {
                        // let iv = create_incoming_value(value.into())?;
                        // Ok(iv)
                        Ok(value)
                    }
                }
            }
            _ => Err(ERROR_INTERNAL_UPSTREAM_OPERATION_FAILURE.into()),
        }
    }

    /// Set a value from the key value store
    fn set(key: String, outgoing_value: String) -> Result<(), String> {
        println!("[debug][kv-provider] performing set...");

        // Read the value from the outgoing value handle
        //
        // TODO(QUESTION): It's not strictly necessary to drop an OV after it's used...
        // It is also a valid use-case to submit the same OV for multiple keys...
        // Can we trust consumers to requst cleanup of values?
        // let ov_bytes = clone_hashmap_value(&OUTGOING_VALUES, outgoing_value)?.unwrap_or_default();

        let payload = wasmbus_rpc::common::serialize(&SetRequest {
            key,
            value: outgoing_value,
            expires: 0, // TODO(ISSUE): keyvalue contract does not specify expiration
        })
        .map_err(|_| String::from(ERROR_INTERNAL_PROVIDER_REQUEST_SERIALIZATION_FAILURE))?;

        match wasmcloud_host_call(
            WASMCLOUD_DEFAULT_BINDING,
            WASMCLOUD_KV_NAMESPACE,
            "KeyValue.Set",
            Some(&payload),
        ) {
            Ok(Some(_)) => Ok(()),
            _ => Err(ERROR_INTERNAL_UPSTREAM_OPERATION_FAILURE.into()),
        }
    }

    fn delete(key: String) -> Result<(), String> {
        println!("[debug][kv-provider] performing delete...");

        // Build payload
        let payload = wasmbus_rpc::common::serialize(&key)
            .map_err(|_| String::from(ERROR_INTERNAL_PROVIDER_REQUEST_SERIALIZATION_FAILURE))?;

        match wasmcloud_host_call(
            WASMCLOUD_DEFAULT_BINDING,
            WASMCLOUD_KV_NAMESPACE,
            "KeyValue.Del",
            Some(&payload),
        ) {
            Ok(Some(_)) => Ok(()),
            _ => Err(String::from(ERROR_INTERNAL_UPSTREAM_OPERATION_FAILURE)),
        }
    }

    fn exists(key: String) -> Result<bool, String> {
        println!("[debug][kv-provider] performing exists...");

        // Build payload
        let payload = wasmbus_rpc::common::serialize(&key)
            .map_err(|_| String::from(ERROR_INTERNAL_PROVIDER_REQUEST_SERIALIZATION_FAILURE))?;

        match wasmcloud_host_call(
            WASMCLOUD_DEFAULT_BINDING,
            WASMCLOUD_KV_NAMESPACE,
            "KeyValue.Contains",
            Some(&payload),
        ) {
            Ok(Some(resp_bytes)) => {
                let resp: bool = wasmbus_rpc::common::deserialize(&resp_bytes)
                    .map_err(|_| String::from(ERROR_INTERNAL_PROVIDER_RESPONSE_DESERIALIZATION_FAILURE))?;
                Ok(resp)
            }
            _ => Err(String::from(ERROR_INTERNAL_UPSTREAM_OPERATION_FAILURE)),
        }
    }

    fn guest_call(op: String, payload: Option<Vec<u8>>) -> Result<Option<Vec<u8>>, String> {
        match op.as_str() {
            "KeyValue.Get" => {
                let key: String = match payload {
                    Some(bytes) => wasmbus_rpc::common::deserialize(&bytes),
                    None => Err("Empty payload".into()),
                }
                .map_err(|e| format!("Request parsing failed: {e}"))?;

                let incoming_value = KvProvider::get(key)
                    .map_err(|e| format!("Get operation failed: {e}"))?;

                // // Read the incoming value
                // let bytes = incoming_value_consume_sync(incoming_value)
                //     .map_err(|e| format!("Failed to read incoming falue, error code: {e}"))?;


                let response = GetResponse {
                    exists: true,
                    // TODO(ISSUE): string <-> byte stream problem (see above)
                    value: incoming_value
                };

                let serialized = wasmbus_rpc::common::serialize(&response)
                    .map_err(|e| format!("response serialization failure: {e}"))?;
                Ok(Some(serialized))
            }

            _ => Err(format!("Operation [{op}] not supported")),
        }
    }
}

export_keyvalue!(KvProvider);
