use petclinic_interface::{
    Date, ListVisitsRequest, RecordVisitRequest, Time, Visit, VisitList, Visits, VisitsReceiver,
};
use wasmbus_rpc::actor::prelude::*;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, Visits)]
struct VisitsActor {}

#[async_trait]
impl Visits for VisitsActor {
    async fn list_visits(&self, _ctx: &Context, _arg: &ListVisitsRequest) -> RpcResult<VisitList> {
        Ok(vec![
            Visit {
                date: Date {
                    day: 11,
                    month: 1,
                    year: 2021,
                },
                description: "Pickles had a rash".to_string(),
                pet_id: 1,
                time: Time {
                    hour: 12,
                    minute: 0,
                },
                vet_id: 1,
            },
            Visit {
                date: Date {
                    day: 5,
                    month: 5,
                    year: 2021,
                },
                description: "Whiskers was missing a tooth".to_string(),
                pet_id: 2,
                time: Time {
                    hour: 12,
                    minute: 0,
                },
                vet_id: 2,
            },
        ])
    }

    async fn record_visit(&self, _ctx: &Context, _arg: &RecordVisitRequest) -> RpcResult<bool> {
        Ok(true)
    }
}
