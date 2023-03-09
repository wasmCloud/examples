use crate::handler::EventParam;
use crate::{consumer::Consumer, producer::Producer};

wit_bindgen::generate!({
    path: "../wit",
    world: "adapter",
});

/*
#[link(wasm_import_module = "downstream")]
extern "C" {
    #[link_name = "on-receive"]
    fn on_receive(arg0: i32);
}
*/

//wit_import(wit_bindgen::rt::as_i32(b));

#[derive(Clone)]
struct MyAdapter;

//struct MyError;

impl Producer for MyAdapter {
    fn publish(
        //&self,
        _: u32,
        _: messaging_types::ChannelResult,
        _: messaging_types::EventResult,
    ) -> std::result::Result<(), u32> {
        println!(">>> called publish");
        Ok(())
    }
}

impl Consumer for MyAdapter {
    fn subscribe(
        //&self,
        _: u32,
        _: messaging_types::ChannelResult,
    ) -> std::result::Result<std::string::String, u32> {
        println!(">>> called subscribe");
        Ok("".to_string())
    }

    fn unsubscribe(
        //&self,
        _: u32,
        _: std::string::String,
    ) -> std::result::Result<(), u32> {
        println!(">>> called unsubscribe");
        Ok(())
    }
}

#[allow(dead_code)]
fn run() -> anyhow::Result<()> {
    let producer = MyAdapter;
    let _consumer = producer.clone();

    /*
    let component = Component::from_file(&engine, "guest.component.wasm")?;
    let (messaging, _) = Messaging::instantiate_async(&mut store, &component, &linker).await?;
    */

    let _new_event = EventParam {
        data: Some("fizz".as_bytes()),
        id: "123",
        source: "rust",
        specversion: "1.0",
        ty: "com.my-messaing.rust.fizzbuzz",
        datacontenttype: None,
        dataschema: None,
        subject: None,
        time: None,
        extensions: None,
    };

    // this part doesn't work yet: need a handle on the sub-module
    //let res = unsafe { on_receive(0) };
    //let res = handler::on_receive(_new_event);
    //let res2 = downstream::handler::on_receive(new_event);

    //println!(">>> called on_receive: {:#?}", res);

    Ok(())
}

export_adapter!(MyAdapter);
