use wascc_host::{WasccHost, NativeCapability};
use wascc_redis::RedisKVProvider;
use wascc_httpsrv::HttpServerProvider;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = WasccHost::new();
    let kv = RedisKVProvider::new();
    let http = HttpServerProvider::new();

    host.add_native_capability(NativeCapability::from_instance(kv, None)?)?;
    host.add_native_capability(NativeCapability::from_instance(http, None)?)?;

    println!("Hello, world!");
    Ok(())
}
