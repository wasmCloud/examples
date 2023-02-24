use wasmbus_rpc::actor::prelude::*;

mod joker;
use joker::*;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, Joker)]
struct ParserActor {}

#[async_trait]
impl Joker for ParserActor {
    async fn joke_msg_handler(&self, _ctx: &Context, arg: &Vec<u8>) -> RpcResult<JokeMsg> {
        let joke: JokeMsg = serde_json::from_slice(arg).unwrap_or_else(|_| JokeMsg::default());
        Ok(joke)
    }
}
