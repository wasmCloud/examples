//! Factorial example capability provider
//!
//!
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use wasmbus_rpc::provider::prelude::*;
use wasmcloud_interface_factorial::{Factorial, FactorialReceiver};

// main (via provider_main) initializes the threaded tokio executor,
// listens to lattice rpcs, handles actor links,
// and returns only when it receives a shutdown message
//
fn main() -> Result<(), Box<dyn std::error::Error>> {
    provider_main(FactorialProvider::default())?;

    eprintln!("Factorial provider exiting");
    Ok(())
}

/// Factorial capability provider implementation
#[derive(Default, Clone)]
struct FactorialProvider {}

/// use default implementations of provider message handlers
impl ProviderDispatch for FactorialProvider {}
impl FactorialReceiver for FactorialProvider {}
impl ProviderHandler for FactorialProvider {}

/// Handle Factorial methods
#[async_trait]
impl Factorial for FactorialProvider {
    /// accepts a number and calculates its factorial
    async fn calculate(&self, _ctx: &Context, req: &u32) -> RpcResult<u64> {
        Ok(n_factorial(*req))
    }
}

/// calculate n factorial
fn n_factorial(n: u32) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        _ => {
            let mut result = 1u64;
            // add 1 because rust ranges exclude upper bound
            for v in 2..(n + 1) {
                result = result * (v as u64);
            }
            result
        }
    }
}

/// Handle incoming rpc messages and dispatch to applicable trait handler.
#[async_trait]
impl MessageDispatch for FactorialProvider {
    async fn dispatch(&self, ctx: &Context, message: Message<'_>) -> RpcResult<Message<'_>> {
        let op = match message.method.split_once('.') {
            Some((cls, op)) if cls == "Factorial" => op,
            None => message.method,
            _ => {
                return Err(RpcError::MethodNotHandled(message.method.to_string()));
            }
        };
        FactorialReceiver::dispatch(
            self,
            ctx,
            &Message {
                method: op,
                arg: message.arg,
            },
        )
        .await
    }
}
