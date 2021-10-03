use petclinic_interface::{VetList, Vets, VetsReceiver};
use wasmbus_rpc::actor::prelude::*;

use wasmcloud_interface_sqldb::SqlDbSender;

pub(crate) type Db = SqlDbSender<WasmHost>;

mod db;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, Vets)]
struct VetsActor {}

#[async_trait]
impl Vets for VetsActor {
    async fn list_vets(&self, ctx: &Context) -> RpcResult<VetList> {
        let db = SqlDbSender::new();
        let vets = db::list_vets(ctx, &db).await?;
        Ok(vets.iter().cloned().map(|v| v.into()).collect())
    }
}
