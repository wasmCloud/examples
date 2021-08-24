//! example actor demonstrating logging with the builtin logging provider

use wasmbus_rpc::actor::prelude::*;
use wasmcloud_example_runner::*;
use wasmcloud_interface_logging::{debug, error, info, warn};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, Runner)]
struct DemoLogging {}

#[async_trait]
impl Runner for DemoLogging {
    async fn run(&self, _ctx: &Context, _args: &Vec<String>) -> RpcResult<Vec<String>> {
        // the four levels of log macros
        debug!("This is logged at DEBUG level");
        info!("This is logged at INFO level");
        warn!("This is logged at WARN level");
        error!("This is logged at ERROR level");

        // the macros also accept format args
        for i in 3..8 {
            info!("{} factorial is {}", i, factorial(i));
        }

        Ok(Vec::default())
    }
}

fn factorial(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => n * factorial(n - 1),
    }
}
