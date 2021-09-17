use petclinic_interface::{
    ListVisitsRequest, RecordVisitRequest, VisitList, Visits, VisitsReceiver,
};
use wasmbus_rpc::actor::prelude::*;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, Visits)]
struct VisitsActor {}

#[async_trait]
impl Visits for VisitsActor {
    async fn list_visits(&self, ctx: &Context, arg: &ListVisitsRequest) -> RpcResult<VisitList> {
        todo!()
    }

    async fn record_visit(&self, ctx: &Context, arg: &RecordVisitRequest) -> RpcResult<bool> {
        todo!()
    }
}
