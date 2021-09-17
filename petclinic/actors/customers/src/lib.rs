use petclinic_interface::{
    AddPetRequest, CreateOwnerReply, Customers, CustomersReceiver, FindOwnerReply, FindPetReply,
    Owner, OwnersList, Pet, PetList, PetTypeList, UpdateOwnerReply,
};
use wasmbus_rpc::actor::prelude::*;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, Customers)]
struct CustomersActor {}

#[async_trait]
impl Customers for CustomersActor {
    async fn create_owner(&self, ctx: &Context, arg: &Owner) -> RpcResult<CreateOwnerReply> {
        todo!()
    }

    async fn find_owner(&self, ctx: &Context, arg: &u64) -> RpcResult<FindOwnerReply> {
        todo!()
    }

    async fn list_owners(&self, ctx: &Context) -> RpcResult<OwnersList> {
        todo!()
    }

    async fn update_owner(&self, ctx: &Context, arg: &Owner) -> RpcResult<UpdateOwnerReply> {
        todo!()
    }

    async fn list_pet_types(&self, ctx: &Context) -> RpcResult<PetTypeList> {
        todo!()
    }

    async fn add_pet(&self, ctx: &Context, arg: &AddPetRequest) -> RpcResult<bool> {
        todo!()
    }

    async fn remove_pet(&self, ctx: &Context, arg: &u64) -> RpcResult<bool> {
        todo!()
    }

    async fn update_pet(&self, ctx: &Context, arg: &Pet) -> RpcResult<bool> {
        todo!()
    }

    async fn list_pets(&self, ctx: &Context, arg: &u64) -> RpcResult<PetList> {
        todo!()
    }

    async fn find_pet(&self, ctx: &Context, arg: &u64) -> RpcResult<FindPetReply> {
        todo!()
    }
}
