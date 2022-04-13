use log::debug;
use minicbor_ser::{from_slice, to_vec};
use pyprov::Service;
use serde_value::Value;

/// Test service.invoke. This is an internal test - not how an actor would invoke the service.
#[tokio::test]
async fn call_service() {
    env_logger::init();
    let service = Service::try_init(None).await.expect("init");
    debug!("config: {:?}", &service.0);

    let n = Value::I32(10);
    let buf = to_vec(&n).unwrap();
    let res: i32 = from_slice(&service.invoke("f.factorial", &buf).await.unwrap()).unwrap();
    assert_eq!(res, 3628800, "10!");

    let buf = to_vec("Sam").unwrap();
    let res: String = from_slice(&service.invoke("h.hello", &buf).await.unwrap()).unwrap();
    assert_eq!(res.as_str(), "Hello Sam!");

    Service::shutdown().await;
}
