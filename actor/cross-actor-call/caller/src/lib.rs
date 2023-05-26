use wasmbus_rpc::actor::prelude::*;
use wasmcloud_example_runner::*;
use wasmcloud_interface_logging::debug;

use std::borrow::Cow;

use types::Response;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, Runner)]
struct Caller {}

#[async_trait]
impl Runner for Caller {
    async fn run(&self, ctx: &Context, args: &Vec<String>) -> RpcResult<Vec<String>> {
        // ask runtime for link to actor by alias
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(types::RESPONDER_ALIAS).unwrap();

        let mut msg: Message = Message {
            method: "",
            arg: Cow::Owned(vec![]),
        };

        // to call method1: wash call $ID '[]'
        // to call method2: wash call $ID '["example"]'
        if args.len() == 0 {
            msg.method = types::XRESPONDER_METHOD1;
        } else {
            msg.method = types::XRESPONDER_METHOD2;
        }

        // execute cross actor call
        let resp = transport.send(ctx, msg, None).await?;
        debug!("got response: {:?}", resp);

        // deserialize binary response
        let resp: Response = bincode::deserialize(&resp[..]).unwrap();
        return RpcResult::Ok(vec![resp.message.clone()]);
    }
}
