use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_cron::*;
use wasmcloud_interface_logging::info;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, Cron)]
struct CronActor {}

#[async_trait]
impl Cron for CronActor {
    async fn timed_invoke(&self, _ctx: &Context, req: &u64) -> RpcResult<()> {
        info!("Timed invoke at {req}");
        Ok(())
    }
}
