//! example actor demonstrating random number generation
//! in wasmcloud:builtin:numbergen

use wasmbus_rpc::actor::prelude::*;
use wasmcloud_example_runner::*;
use wasmcloud_interface_logging::debug;

use std::borrow::Cow;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, Runner)]
struct Caller {}

#[async_trait]
impl Runner for Caller {
    async fn run(&self, ctx: &Context, _args: &Vec<String>) -> RpcResult<Vec<String>> {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_actor(types::ALIAS).unwrap();
        let resp = transport
            .send(
                ctx,
                Message {
                    method: "Call.Call",
                    arg: Cow::Owned(vec![]),
                },
                None,
            )
            .await?;
        debug!("got response: {:?}", resp);
        Ok(vec![])
    }
}
