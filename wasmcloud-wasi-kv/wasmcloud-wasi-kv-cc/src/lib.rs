use bindings::{
    keyvalue::{
        Keyvalue,
        Key,
    },
    readwrite::{
        Bucket,
        Error,
        IncomingValue,
        OutgoingValue,
    },
    // bindings::batch
    // bindings::atomic
};

/// KeyValue Component configured to work with wasmCloud (https://wasmcloud.com)
struct WasmCloudKvComponent;

const ERROR_CODE_NOT_IMPLEMENTED: Error = 1;

/// Implementation of the WASI-KV contract for wasmCloud
impl Keyvalue for WasmCloudKvComponent {

    /// Get a value from the key value store
    fn get(bucket: Bucket, key: Key) -> Result<IncomingValue, Error> {
        Err(ERROR_CODE_NOT_IMPLEMENTED)
    }

    /// Set a value from the key value store
    fn set(bucket: Bucket, key: Key, outgoing_value: OutgoingValue) -> Result<(), Error> {
        Err(ERROR_CODE_NOT_IMPLEMENTED)
    }

    fn delete(bucket: Bucket, key: Key) -> Result<(), Error> {
        Err(ERROR_CODE_NOT_IMPLEMENTED)
    }

    fn exists(bucket: Bucket, key: Key) -> Result<bool, Error> {
        Err(ERROR_CODE_NOT_IMPLEMENTED)
    }
}

bindings::export!(WasmCloudKvComponent);
