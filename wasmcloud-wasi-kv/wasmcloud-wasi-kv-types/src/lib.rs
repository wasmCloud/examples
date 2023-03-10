//! WASI keyvalue types interface implementation
//!
//! This crate provides usable guest (WASM component) support for the types
//! interface which is used by the WASI keyvalue contract (https://github.com/WebAssembly/wasi-keyvalue/blob/main/wit/types.wit)
//!
//! Proper use of this module also relies on the streams interface (https://github.com/WebAssembly/wasi-keyvalue/blob/main/wit/deps/io/streams.wit), which is normally provided by the host.

wit_bindgen::generate!({
    path: "../wit",
    world: "types",
});

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Mutex, RwLock};

use wasi_cloud_error::Error;

use crate::types::{Bucket, IncomingValue, InputStream, OutgoingValue, OutputStream};

// /////////////////
// // Static Data //
// /////////////////

static NEXT_OUTGOING_VALUE: Lazy<Mutex<OutgoingValue>> = Lazy::new(|| Mutex::new(0));
static OUTGOING_VALUES: Lazy<RwLock<HashMap<OutgoingValue, Vec<u8>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

/////////////////
// Static Data //
/////////////////

/// A map incoming values, managed by handles (similar to file descriptors)
/// IncomingValues are used by external modules to pass information *into* this module
/// by calling streams#write
static INCOMING_VALUES: Lazy<RwLock<HashMap<IncomingValue, Vec<u8>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

/// A map of input streams, handles (similar to file descriptors) that control the data flowing in
/// from the outside world. An input stream is reserved from this HashMap,
/// then used by an external entity for writing data that will be read by this module.
static INPUT_STREAMS: Lazy<RwLock<HashMap<InputStream, IncomingValue>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));
static NEXT_INPUT_STREAM: Lazy<Mutex<InputStream>> = Lazy::new(|| Mutex::new(0));

/// A map of output streams, handles (similar to file descriptors) that control the data flowing out
/// from this component. As an output stream is reserved from this HashMap,
/// it is normally filled with content that needs to be ready by an external module
static OUTPUT_STREAMS: Lazy<RwLock<HashMap<OutputStream, OutgoingValue>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));
static NEXT_OUTPUT_STREAM: Lazy<Mutex<OutputStream>> = Lazy::new(|| Mutex::new(0));

/// A map of bucket handles which act similarly to file handles.
/// NOTE: It is possible for the same bucket to be used by two handles.
static BUCKETS: Lazy<RwLock<HashMap<Bucket, String>>> = Lazy::new(|| RwLock::new(HashMap::new()));
static NEXT_BUCKET: Lazy<Mutex<Bucket>> = Lazy::new(|| Mutex::new(0));

////////////
// Errors //
////////////

/// Error thrown when an invalid incoming value is specified
const ERROR_INVALID_INCOMING_VALUE: Error = 400;
/// Generic internal error
const ERROR_INTERNAL: Error = 500;
/// Error thrown when there is no more space in a given map
const ERROR_INTERNAL_MAP_SPACE_EXHAUSTED: Error = 201;
/// Error that is thrown when an internal memory access error occurs (failure to acquire a lock, for instance)
const ERROR_INTERNAL_MEMORY_ACCESS_ERROR: Error = 207;

/// Struct on which implementations will go
struct WasmcloudWasiTypes {}

/// Helper for incrementing an existing thread local
fn get_next_available_key_u32<V>(
    map: &'static RwLock<HashMap<u32, V>>,
    start: &'static Mutex<u32>,
) -> Result<u32, Error> {
    let map = map.read().map_err(|_| ERROR_INTERNAL)?;
    let mut start = start.lock().map_err(|_| ERROR_INTERNAL)?;

    let mut idx = *start + 1;
    while idx != *start {
        match map.contains_key(&idx) {
            true => {
                idx = (idx % u32::MAX) + 1;
            }
            false => {
                *start = idx;
                return Ok(idx);
            }
        }
    }
    Err(ERROR_INTERNAL_MAP_SPACE_EXHAUSTED)
}

// TODO(SECURITY): we need security here -- some way to encode the invocation that was related to a given u32 with
// the rest of it's calls -- you could technically GUESS at incoming/outgoing value handles and maybe hit something.
// We could split a u32 (ex. u8 + u24) in order ot keep track of which invocation thread was using the handle.

impl crate::types::Types for WasmcloudWasiTypes {
    ///////////////////////
    // Bucket Management //
    ///////////////////////

    fn drop_bucket(b: Bucket) {
        println!("[debug][kv-provider] droping bucket...");
        if let Ok(mut buckets) = BUCKETS.write() {
            buckets.remove(&b);
        }
    }

    fn open_bucket(bucket_name: String) -> Result<Bucket, Error> {
        //print("[debug][kv-provider] opening bucket...");
        println!("[debug][kv-provider] opening bucket...");
        let next_b = get_next_available_key_u32(&BUCKETS, &NEXT_BUCKET)?;

        let mut buckets = BUCKETS
            .write()
            .map_err(|_| ERROR_INTERNAL_MEMORY_ACCESS_ERROR)?;
        let _ = buckets.insert(next_b, bucket_name);

        Ok(next_b)
    }

    ////////////////////
    // Outgoing Value //
    ////////////////////

    fn drop_outgoing_value(ov: OutgoingValue) {
        if let Ok(mut outgoing_values) = OUTGOING_VALUES.write() {
            outgoing_values.remove(&ov);
        }
    }

    fn new_outgoing_value() -> OutgoingValue {
        let next_ov = match get_next_available_key_u32(&OUTGOING_VALUES, &NEXT_OUTGOING_VALUE) {
            Ok(v) => v,
            Err(_) => panic!("allocating outgoing value handle failed"),
        };

        match OUTGOING_VALUES.write() {
            Ok(mut outgoing_values) => outgoing_values.insert(next_ov, Vec::new()),
            Err(_) => panic!("saving new outgoing value failed"),
        };

        next_ov
    }

    /// Allocate a new output stream to be used in conjunction with a given outgoing value
    /// as the stream receives values, it will update the outgoing value
    fn outgoing_value_write_body(ov: OutgoingValue) -> Result<OutputStream, ()> {
        let next_os =
            get_next_available_key_u32(&OUTPUT_STREAMS, &NEXT_OUTPUT_STREAM).map_err(|_| ())?;

        let mut output_streams = OUTPUT_STREAMS.write().map_err(|_| ())?;
        output_streams.insert(next_os, ov).ok_or(())
    }

    /////////////////////
    // Incoming Values //
    /////////////////////

    /// Get the size of an incoming value
    fn size(iv: IncomingValue) -> u64 {
        println!("[debug][kv-provider] getting size of incoming value...");
        match INCOMING_VALUES.read() {
            Ok(ivs) => match ivs.get(&iv) {
                Some(bytes) => bytes.len().try_into().unwrap_or(0),
                None => 0,
            },
            Err(_) => 0,
        }
    }

    /// Drop an incoming value that was being taken
    fn drop_incoming_value(iv: IncomingValue) {
        if let Ok(mut map) = INCOMING_VALUES.write() {
            map.remove(&iv);
        }
    }

    /// Consume an incoming value synchronously
    fn incoming_value_consume_sync(iv: IncomingValue) -> Result<Vec<u8>, Error> {
        let mut incoming_values = INCOMING_VALUES
            .write()
            .map_err(|_| ERROR_INTERNAL_MEMORY_ACCESS_ERROR)?;
        incoming_values
            .remove(&iv)
            .ok_or(ERROR_INVALID_INCOMING_VALUE)
    }

    /// Provide an input stream that can be used to read an incoming value
    fn incoming_value_consume_async(iv: IncomingValue) -> Result<InputStream, Error> {
        // Allocate an input stream for reading the given incoming value
        let next_is = get_next_available_key_u32(&INPUT_STREAMS, &NEXT_INPUT_STREAM)?;

        // Add the input stream to the list
        let mut input_streams = INPUT_STREAMS
            .write()
            .map_err(|_| ERROR_INTERNAL_MEMORY_ACCESS_ERROR)?;
        input_streams.insert(next_is, iv);

        Ok(next_is)
    }
}

export_wasmcloud_types!(WasmcloudWasiTypes);
