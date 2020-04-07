use redis_provider::RedisKVProvider;
use wascc_httpsrv::HttpServerProvider;
fn main() {
    //let host = WasccHost::new();
    let _kv = RedisKVProvider::new();
    let _http = HttpServerProvider::new();

    println!("Hello, world!");
}
