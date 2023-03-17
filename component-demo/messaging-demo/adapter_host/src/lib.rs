#![cfg(target_arch = "wasm32")]
#![allow(unused_macros)]

wit_bindgen::generate!({
    path: "../wit",
    world: "adapter-host",
    serialize: "serde::Serialize",
    deserialize: "serde::Deserialize",
});
use messaging_types::{ChannelResult, EventResult};

//#[used]
//#[link(wasm_import_module = "xyz")]
//extern "C" {
//    static _GUEST_MODULE: i32;
//}

#[derive(Default)]
struct MyAdapter;

/// Publisher interface
impl messaging::Messaging for MyAdapter {
    /// Publish:  publish() from downstream crate coverted to wasmbus-rpc, forwarded to host
    fn publish(
        _broker: u32,
        channel: messaging_types::ChannelResult,
        event: EventResult,
    ) -> Result<(), u32> {
        println!(">>> called publish");
        let message = rmp_serde::to_vec(&event).map_err(|e| {
            println!("serialization of event: {e}");
            1u32
        })?;
        let rpc_message = lib::PublishDataResult {
            subject: match channel {
                ChannelResult::Queue(s) => s,
                ChannelResult::Topic(s) => s,
            },
            message,
        };
        let vec = rmp_serde::to_vec(&rpc_message).unwrap();
        host::host_call(
            "default",                    // link name
            "wasmcloud:wasi:messaging",   // contract_id
            "Messaging.Producer.publish", // method
            &vec,
        )
        .map_err(|e| {
            // TODO: is this number supposed to be a pointer?
            println!("publish error: {e}");
            1u32
        })?;
        println!("publish: len {}", vec.len(),);
        Ok(())
    }

    /// subscribe() from downstream crate coverted to wasmbus-rpc, forwarded to host
    fn subscribe(_: u32, channel: ChannelResult) -> Result<String, u32> {
        println!(">>> called subscribe, channel: {:?}", &channel);
        let vec1 = rmp_serde::to_vec(&channel).unwrap();
        println!("subscribe: serde encode: {}", vec1.len(),);
        let ret = host::host_call(
            //let ret = wasmbus_rpc::actor::prelude::host_call(
            "default",                      // link name
            "wasmcloud:wasi:messaging",     // contract_id
            "Messaging.Consumer.subscribe", // method
            &vec1,
        )
        .map_err(|e| {
            // TODO: what is the subscribe error?
            println!("subscribe error: {e}");
            1u32
        })?;
        // on success, returns a String subscribe-token
        let s = String::from_utf8_lossy(&ret);
        Ok(s.to_string())
    }

    fn unsubscribe(_: u32, channel: String) -> Result<(), u32> {
        println!(">>> called unsubscribe, channel: {:?}", &channel);
        let vec1 = rmp_serde::to_vec(&channel).unwrap();
        println!("unsubscribe: serde encode: {}", vec1.len(),);
        let _ = host::host_call(
            "default",                        // link name
            "wasmcloud:wasi:messaging",       // contract_id
            "Messaging.Consumer.unsubscribe", // method
            &vec1,
        )
        .map_err(|e| {
            println!("unsubscribe error: {e}");
            0u32
        })?;
        Ok(())
    }

    fn drop_broker(_b: messaging::Broker) {
        // nothing to do: no broker
    }

    fn open_broker(_name: String) -> Result<messaging::Broker, messaging::Error> {
        Ok(42)
    }

    fn drop_error(e: messaging::Error) {
        println!("drop error: {e}")
    }

    fn trace(e: messaging::Error) -> String {
        println!("trace: {e}");
        "ok".into()
    }
}

// generate export functions needed by host and by downstream copmonent
export_adapter_host!(MyAdapter);
