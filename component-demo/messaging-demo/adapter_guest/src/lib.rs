use actor::Actor;

wit_bindgen::generate!({
    path: "../wit",
    world: "adapter-guest",
    serialize: "serde::Serialize",
    deserialize: "serde::Deserialize",
});

#[derive(Default)]
struct MyAdapter;

/// Actor.guest_call: accept incoming wasmbus_rpc message and dispatch
/// Expected methods:
///     `Messaging.Handler.on_receive` - subscription callback.
impl Actor for MyAdapter {
    fn guest_call(operation: String, payload: Vec<u8>) -> Result<Vec<u8>, String> {
        match operation.as_ref() {
            // op name is world.interface.method
            "Messaging.Handler.on_receive" => {
                // TODO: error handling
                let event = rmp_serde::from_slice::<handler::EventResult>(&payload).unwrap();
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

/// Handle subscription callbacks (CloudEvent delivery on subscribed channel)
impl MyAdapter {
    /// Receive from host, convert wasmbus-rpc to wit object, forward to downstream
    fn on_receive(e: handler::EventResult) -> Result<(), u32> {
        println!(">>> Received: {:#?}", e);
        // we should codegen the transformation from EventResult to EventParam
        // (or allow on_handle to accept EventResult). At least this is fast.
        // at least it's fast and requires no heap allocs
        let extensions = e.extensions.as_ref().map(|vec| {
            vec.iter()
                .map(|(a, b)| (a.as_str(), b.as_str()))
                .collect::<Vec<_>>()
        });
        let param = handler::EventParam {
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
        // there's no 'self' param so we just send to static function export
        let res = handler::on_receive(param);
        if let Err(e) = res {
            println!("error forwarding event: {}", e.to_string());
            Err(e)
        } else {
            Ok(())
        }
    }
}

// generate export functions needed by host and by downstream copmonent
export_adapter_guest!(MyAdapter);
