use petclinic_interface::{
    AddPetRequest, CreateOwnerReply, Customers, CustomersReceiver, FindOwnerReply, FindPetReply,
    Owner, OwnersList, Pet, PetList, PetTypeList, UpdateOwnerReply,
};

use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_sqldb::SqlDbSender;

pub(crate) type Db = SqlDbSender<WasmHost>;

mod db;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, Customers)]
struct CustomersActor {}

#[async_trait]
impl Customers for CustomersActor {
    async fn create_owner(&self, ctx: &Context, owner: &Owner) -> RpcResult<CreateOwnerReply> {
        let db = SqlDbSender::new();
        Ok(match db::create_owner(&ctx, &db, owner).await {
            Ok(_) => CreateOwnerReply {
                id: owner.id,
                success: true,
            },
            Err(_e) => CreateOwnerReply {
                id: 0,
                success: false,
            },
        })
    }

    async fn find_owner(&self, ctx: &Context, arg: &u64) -> RpcResult<FindOwnerReply> {
        let db = SqlDbSender::new();
        Ok(FindOwnerReply {
            owner: db::find_owner(&ctx, &db, *arg).await?.map(|o| o.into()),
        })
    }

    async fn list_owners(&self, ctx: &Context) -> RpcResult<OwnersList> {
        let db = SqlDbSender::new();
        let owners = db::list_all_owners(&ctx, &db).await?;
        Ok(owners.iter().cloned().map(|o| o.into()).collect())
    }

    async fn update_owner(&self, ctx: &Context, arg: &Owner) -> RpcResult<UpdateOwnerReply> {
        let db = SqlDbSender::new();
        Ok(match db::update_owner(&ctx, &db, arg).await {
            Ok(_) => UpdateOwnerReply { success: true },
            Err(_e) => UpdateOwnerReply { success: false },
        })
    }

    async fn list_pet_types(&self, ctx: &Context) -> RpcResult<PetTypeList> {
        let db = SqlDbSender::new();
        let pettypes = db::list_all_pet_types(&ctx, &db).await?;
        Ok(pettypes.iter().cloned().map(|o| o.into()).collect())
    }

    async fn add_pet(&self, ctx: &Context, arg: &AddPetRequest) -> RpcResult<bool> {
        let db = SqlDbSender::new();
        Ok(match db::add_pet(&ctx, &db, arg.owner_id, &arg.pet).await {
            Ok(_) => true,
            Err(_e) => false,
        })
    }

    async fn remove_pet(&self, ctx: &Context, arg: &u64) -> RpcResult<bool> {
        let db = SqlDbSender::new();
        Ok(match db::delete_pet(&ctx, &db, *arg).await {
            Ok(_) => true,
            Err(_e) => false,
        })
    }

    async fn update_pet(&self, ctx: &Context, arg: &Pet) -> RpcResult<bool> {
        let db = SqlDbSender::new();
        Ok(match db::update_pet(&ctx, &db, arg).await {
            Ok(_) => true,
            Err(_e) => false,
        })
    }

    async fn list_pets(&self, ctx: &Context, arg: &u64) -> RpcResult<PetList> {
        let db = SqlDbSender::new();
        let pets = db::list_pets_by_owner(&ctx, &db, *arg).await?;
        Ok(pets.iter().cloned().map(|o| o.into()).collect())
    }

    async fn find_pet(&self, ctx: &Context, arg: &u64) -> RpcResult<FindPetReply> {
        let db = SqlDbSender::new();
        Ok(FindPetReply {
            pet: db::find_pet(ctx, &db, *arg).await?.map(|p| p.into()),
        })
    }
}
