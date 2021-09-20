use petclinic_interface::{Vet, VetList, Vets, VetsReceiver};
use wasmbus_rpc::actor::prelude::*;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, Vets)]
struct VetsActor {}

#[async_trait]
impl Vets for VetsActor {
    async fn list_vets(&self, _ctx: &Context) -> RpcResult<VetList> {
        Ok(vec![
            Vet {
                first_name: "Alice".to_string(),
                id: 1,
                last_name: "Vetsworth".to_string(),
                specialties: vec![
                    "Cats".to_string(),
                    "Dogs".to_string(),
                    "Cryptography".to_string(),
                ],
            },
            Vet {
                first_name: "Bob".to_string(),
                id: 2,
                last_name: "Vetsworth".to_string(),
                specialties: vec!["Dogs".to_string(), "Cryptography".to_string()],
            },
        ])
    }
}
