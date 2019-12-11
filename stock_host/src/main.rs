use std::collections::HashMap;
use wascc_host::{host, Actor, Capability};
use wapc::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let stock_actor = Actor::from_file("../stock_actor/stock_actor.wasm")?;
    let stock_cap = Actor::from_file("../stock_capability/stock_capability.wasm")?;

    host::add_actor(stock_actor)?;
    host::add_capability(stock_cap, WasiParams::new(vec![], vec![], vec![], vec![]))?;    

    host::add_native_capability(Capability::from_file(
        "../../wascc-host/examples/.assets/libwascc_httpsrv.so",
    )?)?;

    host::configure(
        "MDHEMFPZ7FQQSG6BYOC3AIY6EZJ2WJ2CD5N4JPGCWHL4GX4GR7EAY6V4",
        "acme:stock",
        HashMap::new(),
    )?;

    host::configure(
        "MDHEMFPZ7FQQSG6BYOC3AIY6EZJ2WJ2CD5N4JPGCWHL4GX4GR7EAY6V4",
        "wascc:http_server",
        generate_port_config(8081),
    )?;


    std::thread::park();

    Ok(())
}

fn generate_port_config(port: u16) -> HashMap<String, String> {
    let mut hm = HashMap::new();
    hm.insert("PORT".to_string(), port.to_string());

    hm
}
