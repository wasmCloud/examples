use petclinic_interface::{
    AddPetRequest, CreateOwnerReply, Customers, CustomersReceiver, Date, FindOwnerReply,
    FindPetReply, Owner, OwnersList, Pet, PetList, PetType, PetTypeList, UpdateOwnerReply,
};
use wasmbus_rpc::actor::prelude::*;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, Customers)]
struct CustomersActor {}

#[async_trait]
impl Customers for CustomersActor {
    async fn create_owner(&self, _ctx: &Context, _arg: &Owner) -> RpcResult<CreateOwnerReply> {
        Ok(CreateOwnerReply {
            id: 1,
            success: true,
        })
    }

    async fn find_owner(&self, _ctx: &Context, _arg: &u64) -> RpcResult<FindOwnerReply> {
        Ok(FindOwnerReply {
            owner: Some(Owner {
                first_name: "Bob".to_string(),
                last_name: Some("PetLover".to_string()),
                email: "bob@lovespets.com".to_string(),
                id: 1,
                ..Default::default()
            }),
        })
    }

    async fn list_owners(&self, _ctx: &Context) -> RpcResult<OwnersList> {
        Ok(vec![
            Owner {
                first_name: "Bob".to_string(),
                last_name: Some("PetLover".to_string()),
                email: "bob@lovespets.com".to_string(),
                id: 1,
                ..Default::default()
            },
            Owner {
                first_name: "Alice".to_string(),
                last_name: Some("PetLover".to_string()),
                email: "alice@lovespets.com".to_string(),
                id: 2,
                ..Default::default()
            },
        ])
    }

    async fn update_owner(&self, _ctx: &Context, _arg: &Owner) -> RpcResult<UpdateOwnerReply> {
        Ok(UpdateOwnerReply { success: true })
    }

    async fn list_pet_types(&self, _ctx: &Context) -> RpcResult<PetTypeList> {
        Ok(vec![
            PetType {
                id: 1,
                name: "Cat".to_string(),
            },
            PetType {
                id: 2,
                name: "Dog".to_string(),
            },
            PetType {
                id: 3,
                name: "Anaconda".to_string(),
            },
        ])
    }

    async fn add_pet(&self, _ctx: &Context, _arg: &AddPetRequest) -> RpcResult<bool> {
        Ok(true)
    }

    async fn remove_pet(&self, _ctx: &Context, _arg: &u64) -> RpcResult<bool> {
        Ok(true)
    }

    async fn update_pet(&self, _ctx: &Context, _arg: &Pet) -> RpcResult<bool> {
        Ok(true)
    }

    async fn list_pets(&self, _ctx: &Context, _arg: &u64) -> RpcResult<PetList> {
        Ok(vec![
            Pet {
                birthdate: Date {
                    day: 1,
                    month: 1,
                    year: 2010,
                },
                id: 1,
                name: "Pickles".to_string(),
                pet_type: 1,
            },
            Pet {
                birthdate: Date {
                    day: 1,
                    month: 1,
                    year: 2011,
                },
                id: 2,
                name: "Whiskers".to_string(),
                pet_type: 2,
            },
        ])
    }

    async fn find_pet(&self, _ctx: &Context, arg: &u64) -> RpcResult<FindPetReply> {
        Ok(FindPetReply {
            pet: Some(Pet {
                id: *arg,
                name: "Located Pet".to_string(),
                ..Default::default()
            }),
        })
    }
}
