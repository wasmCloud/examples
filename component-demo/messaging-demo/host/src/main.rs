wasmtime::component::bindgen!({
    path: "../wit",
    world: "messaging",
    async: true,
});

use host::{add_to_linker, WasiCtx};
use messaging_types::MessagingTypes;
use wasi_cap_std_sync::WasiCtxBuilder;
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};

use crate::{consumer::Consumer, handler::EventParam, producer::Producer};

struct MyProducer;
struct MyConsumer;

struct MyError;

#[async_trait::async_trait]
impl MessagingTypes for MyError {
    async fn open_broker(
        &mut self,
        _: std::string::String,
    ) -> std::result::Result<std::result::Result<u32, u32>, anyhow::Error>
    {
        println!(">>> called open_broker");
        Ok(Ok(0))
    }

    async fn drop_broker(
        &mut self,
        _: u32,
    ) -> std::result::Result<(), anyhow::Error>
    {
        println!(">>> called drop_broker");
        Ok(())
    }

    async fn drop_error(
        &mut self,
        _: u32,
    ) -> std::result::Result<(), anyhow::Error>
    {
        println!(">>> called drop_error");
        Ok(())
    }

    async fn trace(
        &mut self,
        _: u32,
    ) -> std::result::Result<std::string::String, anyhow::Error>
    {
        println!(">>> called trace");
        Ok("".to_string())
    }
}

#[async_trait::async_trait]
impl Producer for MyProducer {
    async fn publish(
        &mut self,
        _: u32,
        _: messaging_types::Channel,
        _: messaging_types::EventResult,
    ) -> std::result::Result<std::result::Result<(), u32>, anyhow::Error> {
        println!(">>> called publish");
        Ok(Ok(()))
    }
}

#[async_trait::async_trait]
impl Consumer for MyConsumer {
    async fn subscribe(
        &mut self,
        _: u32,
        _: messaging_types::Channel,
    ) -> std::result::Result<std::result::Result<std::string::String, u32>, anyhow::Error> {
        println!(">>> called subscribe");
        Ok(Ok("".to_string()))
    }

    async fn unsubscribe(
        &mut self,
        _: u32,
        _: std::string::String,
    ) -> std::result::Result<std::result::Result<(), u32>, anyhow::Error> {
        println!(">>> called unsubscribe");
        Ok(Ok(()))
    }
}

pub struct Ctx {
    producer: MyProducer,
    consumer: MyConsumer,
    types: MyError,
    wasi: WasiCtx,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let producer = MyProducer;
    let consumer = MyConsumer;
    let types = MyError;

    let wasi = WasiCtxBuilder::new().build();

    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let engine = Engine::new(&config)?;

    let mut store = Store::new(
        &engine,
        Ctx {
            producer,
            consumer,
            types,
            wasi,
        },
    );

    let mut linker = Linker::new(&engine);
    producer::add_to_linker(&mut linker, |ctx: &mut Ctx| &mut ctx.producer)?;
    consumer::add_to_linker(&mut linker, |ctx: &mut Ctx| &mut ctx.consumer)?;
    messaging_types::add_to_linker(&mut linker, |ctx: &mut Ctx| &mut ctx.types)?;

    add_to_linker(&mut linker, |ctx: &mut Ctx| &mut ctx.wasi)?;

    let component = Component::from_file(&engine, "guest.component.wasm")?;
    let (messaging, _) = Messaging::instantiate_async(&mut store, &component, &linker).await?;

    let new_event = EventParam {
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

    let res = messaging
        .handler
        .call_on_receive(&mut store, new_event)
        .await?;

    println!(">>> called on_receive: {:#?}", res);

    Ok(())
}
