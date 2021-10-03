use petclinic_interface::{
    ListVisitsRequest, RecordVisitRequest, VisitList, Visits, VisitsReceiver,
};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::error;
use wasmcloud_interface_sqldb::SqlDbSender;

pub(crate) type Db = SqlDbSender<WasmHost>;

mod db;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, Visits)]
struct VisitsActor {}

#[async_trait]
impl Visits for VisitsActor {
    async fn list_visits(&self, ctx: &Context, arg: &ListVisitsRequest) -> RpcResult<VisitList> {
        let db = SqlDbSender::new();
        let arg = arg.clone();
        let visits = db::list_visits_by_owner_and_pet(ctx, &db, arg.owner_id, arg.pet_ids).await?;
        Ok(visits.iter().cloned().map(|v| v.into()).collect())
    }

    async fn record_visit(&self, ctx: &Context, arg: &RecordVisitRequest) -> RpcResult<bool> {
        let db = SqlDbSender::new();

        Ok(
            match db::record_visit(ctx, &db, arg.owner_id, arg.visit.clone()).await {
                Ok(_) => true,
                Err(e) => {
                    error!("Failed to record visit: {}", e);
                    false
                }
            },
        )
    }
}
