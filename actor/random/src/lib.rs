//! example actor demonstrating random number generation
//! in wasmcloud:builtin:numbergen

use wasmbus_rpc::actor::prelude::*;
use wasmcloud_example_runner::*;
use wasmcloud_interface_numbergen::random_in_range;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, Runner)]
struct DemoRandom {}

#[async_trait]
impl Runner for DemoRandom {
    /// given a list of strings, return a random item from the list
    /// If no list is provided, return a random number from 1 to 6
    async fn run(&self, _ctx: &Context, args: &Vec<String>) -> RpcResult<Vec<String>> {
        let result = if args.is_empty() {
            random_in_range(1, 6).await.unwrap().to_string()
        } else {
            let choice_num = random_in_range(0, args.len() as u32 - 1).await.unwrap();
            args.get(choice_num as usize).unwrap().clone()
        };
        Ok(vec![result])
    }
}
