#![allow(unused_macros)]
use crate::actor::Actor;
use crate::consumer::Consumer;
use crate::handler::Handler;
use crate::producer::Producer;

wit_bindgen::generate!({
    path: "../wit",
    world: "adapter",
    serialize: "serde::Serialize",
    deserialize: "serde::Deserialize",
});

#[derive(Default)]
struct MyAdapter;

/// Actor.guest_call: accept incoming wasmbus_rpc message and dispatch
/// Expected methods:
///     `Messaging.Handler.on_receive` - subscription callback.
impl Actor for MyAdapter {
    fn guest_call(
        operation: wit_bindgen::rt::string::String,
        payload: wit_bindgen::rt::vec::Vec<u8>,
    ) -> Result<wit_bindgen::rt::vec::Vec<u8>, wit_bindgen::rt::string::String> {
        match operation.as_ref() {
            // op name is world.interface.method
            "Messaging.Handler.on_receive" => {
                // TODO: error handling
                let event =
                    rmp_serde::from_slice::<messaging_types::EventResult>(&payload).unwrap();
                let res = MyAdapter::on_receive(event);
                if let Err(e) = res {
                    Err(format!("on_receive returned {e}"))
                } else {
                    Ok(Vec::new())
                }
            }
            _ => {
                let msg = "invalid invocation on adapter. Expecting Handler.on_receive";
                println!("{msg}");
                Err(msg.to_string())
            }
        }
    }

    /// handle host check of api version
    fn wasmbus_rpc_version() -> u32 {
        2
    }
}

/// messaging-types "static" functions that may be used by downstream component.
/// These are all stubbed out to do nothing, since we don't need refernce handles for this interface
impl MyAdapter {
    fn open_broker(_: String) -> Result<u32, u32> {
        println!(">>> called open_broker");
        Ok(0)
    }

    fn drop_broker(_: u32) {
        println!(">>> called drop_broker");
    }

    fn drop_error(_: u32) {
        println!(">>> called drop_error");
    }

    fn trace(_: u32) -> String {
        println!(">>> called trace");
        String::new()
    }
}

/// low-level exported functions from messaging-types
/// We should have been able to declare these as exported in adapter world file,
/// but that got a conflict because the name is used by producer and consumer in this package.
/// So, these had to be created by hand, with lifting and lowering also by hand.
/// Fortunately, lifting and lowering is simple for these because this capability
/// implementation doesn't need resources, so we pass back dummy values for open, and drop is a no-op.
#[doc(hidden)]
#[export_name = "messaging-types#open-broker"]
#[allow(non_snake_case)]
unsafe extern "C" fn __export_messaging_types_open_broker(_arg0: i32, _arg1: i32) -> i32 {
    let _ = MyAdapter::open_broker(String::new());
    0
}

#[doc(hidden)]
#[export_name = "messaging-types#drop-broker"]
#[allow(non_snake_case)]
unsafe extern "C" fn __export_messaging_types_drop_broker(arg0: i32) {
    MyAdapter::drop_broker(arg0 as u32)
}

#[doc(hidden)]
#[export_name = "messaging-types#drop-error"]
#[allow(non_snake_case)]
unsafe extern "C" fn __export_messaging_types_drop_error(arg0: i32) {
    MyAdapter::drop_error(arg0 as u32)
}

#[doc(hidden)]
#[export_name = "messaging-types#trace"]
#[allow(non_snake_case)]
unsafe extern "C" fn __export_messaging_types_trace(arg0: i32) -> i32 {
    let _s = MyAdapter::trace(arg0 as u32);
    // TODO: incomplete
    0
}

/// Publisher interface
impl Producer for MyAdapter {
    /// Publish:  publish() from downstream crate coverted to wasmbus-rpc, forwarded to host
    fn publish(
        _: u32,
        _channel: crate::producer::ChannelResult,
        event: crate::producer::EventResult,
    ) -> Result<(), u32> {
        println!(">>> called publish");
        let vec = rmp_serde::to_vec(&event).unwrap();
        crate::host::host_call(
            //wasmbus_rpc::actor::prelude::host_call(
            "default",                 // link name
            "wasmcloud:wasi:messaging", // contract_id
            "Messaging.Producer.publish",        // method
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
}

impl Consumer for MyAdapter {
    /// subscribe() from downstream crate coverted to wasmbus-rpc, forwarded to host
    fn subscribe(_: u32, channel: crate::consumer::ChannelResult) -> Result<String, u32> {
        println!(">>> called subscribe, channel: {:?}", &channel);
        let vec1 = rmp_serde::to_vec(&channel).unwrap();
        println!("subscribe: serde encode: {}", vec1.len(),);
        let ret = crate::host::host_call(
            //let ret = wasmbus_rpc::actor::prelude::host_call(
            "default",                 // link name
            "wasmcloud:wasi:messaging", // contract_id
            "Messaging.Consumer.subscribe",      // method
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
        let _ = crate::host::host_call(
            "default",                 // link name
            "wasmcloud:wasi:messaging", // contract_id
            "Messaging.Consumer.unsubscribe",    // method
            &vec1,
        )
        .map_err(|e| {
            println!("unsubscribe error: {e}");
            0u32
        })?;
        Ok(())
    }
}

/// Handle subscription callbacks (CloudEvent delivery on subscribed channel)
impl handler::Handler for MyAdapter {
    /// Receive from host, convert wasmbus-rpc to wit object, forward to downstream
    fn on_receive(e: messaging_types::EventResult) -> Result<(), u32> {
        println!(">>> Received: {:#?}", e);
        // we should codegen the transformation from EventResult to EventParam
        // (or allow on_handle to accept EventResult). At least this is fast.
        // at least it's fast and requires no heap allocs
        let extensions = e.extensions.as_ref().map(|vec| {
            vec.iter()
                .map(|(a, b)| (a.as_str(), b.as_str()))
                .collect::<Vec<_>>()
        });
        let param = messaging_types::EventParam {
            specversion: &e.specversion,
            ty: &e.ty,
            source: &e.source,
            id: &e.id,
            data: e.data.as_deref(),
            datacontenttype: e.datacontenttype.as_deref(),
            dataschema: e.dataschema.as_deref(),
            subject: e.subject.as_deref(),
            time: e.time.as_deref(),
            extensions: extensions.as_deref(),
        };
        // forward to downstream
        let res = downstream::on_receive(param);
        if let Err(e) = res {
            println!("error forwarding event: {e}");
            Err(e)
        } else {
            Ok(())
        }
    }
}

// generate export functions needed by host and by downstream copmonent
export_adapter!(MyAdapter);
