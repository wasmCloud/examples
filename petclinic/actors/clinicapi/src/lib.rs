use petclinic_interface::{
    AddPetRequest, Customers, CustomersSender, Date, ListVisitsRequest, Owner, Pet, PetType,
    RecordVisitRequest, Vets, VetsSender, Visit, Visits, VisitsSender,
};
use serde::{Deserialize, Serialize};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_logging::debug;

const VETS_ACTOR: &str = "petclinic/vets";
const CUSTOMERS_ACTOR: &str = "petclinic/customers";
const VISITS_ACTOR: &str = "petclinic/visits";

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct ClinicapiActor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for ClinicapiActor {
    async fn handle_request(
        &self,
        ctx: &Context,
        req: &HttpRequest,
    ) -> std::result::Result<HttpResponse, RpcError> {
        debug!("API request: {:?}", req);

        let path = &req.path[1..req.path.len()];
        let segments: Vec<&str> = path.trim_end_matches('/').split('/').collect();
        debug!("Segments: {:?}", segments);
        match (req.method.as_ref(), segments.as_slice()) {
            ("GET", ["pettypes"]) => get_pet_types(ctx).await,
            ("GET", ["vets"]) => get_vets(ctx).await,

            // Owners (Customers)
            ("POST", ["owners"]) => create_owner(ctx, deser(&req.body)?).await,
            ("GET", ["owners"]) => get_owners(ctx).await,
            ("PUT", ["owners", owner_id]) => update_owner(ctx, owner_id, deser(&req.body)?).await,
            ("GET", ["owners", owner_id]) => get_owner(ctx, owner_id).await,
            // Pets (Customers)
            ("GET", ["owners", owner_id, "pets"]) => get_pets(ctx, owner_id).await,
            ("POST", ["owners", owner_id, "pets"]) => {
                create_pet(ctx, owner_id, deser(&req.body)?).await
            }
            ("DELETE", ["owners", owner_id, "pets", pet_id]) => {
                delete_pet(ctx, owner_id, pet_id).await
            }
            ("PUT", ["owners", owner_id, "pets", pet_id]) => {
                update_pet(ctx, owner_id, pet_id, deser(&req.body)?).await
            }
            ("GET", ["owners", owner_id, "pets", pet_id]) => get_pet(ctx, owner_id, pet_id).await,
            // Visits
            ("GET", ["owners", owner_id, "pets", pet_id, "visits"]) => {
                get_pet_visits(ctx, owner_id, pet_id).await
            }
            ("POST", ["owners", owner_id, "pets", pet_id, "visits"]) => {
                record_visit(ctx, owner_id, pet_id, deser(&req.body)?).await
            }
            (_, _) => Ok(HttpResponse::not_found()),
        }
    }
}

async fn record_visit(
    ctx: &Context,
    owner_id: &str,
    _pet_id: &str,
    visit: Visit,
) -> RpcResult<HttpResponse> {
    let oid: u64 = owner_id.parse().unwrap_or(0);
    if VisitsSender::to_actor(VISITS_ACTOR)
        .record_visit(
            ctx,
            &RecordVisitRequest {
                owner_id: oid,
                visit,
            },
        )
        .await?
    {
        Ok(HttpResponse::default())
    } else {
        Ok(HttpResponse::internal_server_error(
            "Failed to record visit",
        ))
    }
}

async fn get_pet_visits(ctx: &Context, owner_id: &str, pet_id: &str) -> RpcResult<HttpResponse> {
    let petid: u64 = pet_id.parse().unwrap_or(0);
    let oid: u64 = owner_id.parse().unwrap_or(0);
    let visits = VisitsSender::to_actor(VISITS_ACTOR)
        .list_visits(
            ctx,
            &ListVisitsRequest {
                owner_id: oid,
                pet_ids: Some(vec![petid]),
            },
        )
        .await?;
    HttpResponse::json(visits, 200)
}

async fn get_pet(ctx: &Context, _owner_id: &str, pet_id: &str) -> RpcResult<HttpResponse> {
    let petid: u64 = pet_id.parse().unwrap_or(0);
    if let Some(p) = CustomersSender::to_actor(CUSTOMERS_ACTOR)
        .find_pet(ctx, &petid)
        .await?
        .pet
    {
        let pts = CustomersSender::to_actor(CUSTOMERS_ACTOR)
            .list_pet_types(ctx)
            .await?;
        let ap = AugmentedPet {
            birthdate: p.birthdate,
            id: p.id,
            name: p.name,
            pet_type: get_pet_type(&pts, p.pet_type),
        };

        HttpResponse::json(ap, 200)
    } else {
        Ok(HttpResponse::not_found())
    }
}

async fn update_pet(
    ctx: &Context,
    _owner_id: &str,
    _pet_id: &str,
    pet: Pet,
) -> RpcResult<HttpResponse> {
    if CustomersSender::to_actor(CUSTOMERS_ACTOR)
        .update_pet(ctx, &pet)
        .await?
    {
        Ok(HttpResponse::default())
    } else {
        Ok(HttpResponse::internal_server_error("Failed to update pet"))
    }
}

async fn delete_pet(ctx: &Context, _owner_id: &str, pet_id: &str) -> RpcResult<HttpResponse> {
    let petid: u64 = pet_id.parse().unwrap_or(0);
    if CustomersSender::to_actor(CUSTOMERS_ACTOR)
        .remove_pet(ctx, &petid)
        .await?
    {
        Ok(HttpResponse::default())
    } else {
        Ok(HttpResponse::internal_server_error("Failed to remove pet"))
    }
}

async fn create_pet(ctx: &Context, owner_id: &str, pet: Pet) -> RpcResult<HttpResponse> {
    let oid: u64 = owner_id.parse().unwrap_or(0);
    if CustomersSender::to_actor(CUSTOMERS_ACTOR)
        .add_pet(ctx, &AddPetRequest { owner_id: oid, pet })
        .await?
    {
        Ok(HttpResponse::default())
    } else {
        Ok(HttpResponse::internal_server_error("Failed to create pet"))
    }
}

async fn get_pets(ctx: &Context, owner_id: &str) -> RpcResult<HttpResponse> {
    let oid: u64 = owner_id.parse().unwrap_or(0);
    let x = CustomersSender::to_actor(CUSTOMERS_ACTOR)
        .list_pets(ctx, &oid)
        .await?;
    let pts = CustomersSender::to_actor(CUSTOMERS_ACTOR)
        .list_pet_types(ctx)
        .await?;
    let ax: Vec<_> = x
        .iter()
        .map(|p| AugmentedPet {
            birthdate: p.birthdate.clone(),
            id: p.id,
            name: p.name.to_string(),
            pet_type: get_pet_type(&pts, p.pet_type),
        })
        .collect();
    HttpResponse::json(ax, 200)
}

async fn update_owner(ctx: &Context, _owner_id: &str, owner: Owner) -> RpcResult<HttpResponse> {
    let x = CustomersSender::to_actor(CUSTOMERS_ACTOR)
        .update_owner(ctx, &owner)
        .await?;
    if x.success {
        HttpResponse::json(x, 200)
    } else {
        Ok(HttpResponse::internal_server_error(
            "Failed to update owner",
        ))
    }
}

async fn create_owner(ctx: &Context, owner: Owner) -> RpcResult<HttpResponse> {
    let x = CustomersSender::to_actor(CUSTOMERS_ACTOR)
        .create_owner(ctx, &owner)
        .await?;
    if x.success {
        HttpResponse::json(x, 200)
    } else {
        Ok(HttpResponse::internal_server_error(
            "Failed to create owner",
        ))
    }
}

async fn get_pet_types(ctx: &Context) -> RpcResult<HttpResponse> {
    let pettypes = CustomersSender::to_actor(CUSTOMERS_ACTOR)
        .list_pet_types(ctx)
        .await?;
    HttpResponse::json(pettypes, 200)
}

async fn get_vets(ctx: &Context) -> RpcResult<HttpResponse> {
    let vets = VetsSender::to_actor(VETS_ACTOR).list_vets(ctx).await?;
    HttpResponse::json(vets, 200)
}

async fn get_owners(ctx: &Context) -> RpcResult<HttpResponse> {
    let owners = CustomersSender::to_actor(CUSTOMERS_ACTOR)
        .list_owners(ctx)
        .await?;
    HttpResponse::json(owners, 200)
}

async fn get_owner(ctx: &Context, owner_id: &str) -> RpcResult<HttpResponse> {
    let oid: u64 = owner_id.parse().unwrap_or(0);
    let owner = CustomersSender::to_actor(CUSTOMERS_ACTOR)
        .find_owner(ctx, &oid)
        .await?;
    if let Some(o) = owner.owner {
        HttpResponse::json(o, 200)
    } else {
        Ok(HttpResponse::not_found())
    }
}

fn get_pet_type(pts: &[PetType], id: u64) -> AugmentedPetType {
    pts.iter().find(|p| p.id == id).map_or_else(
        || AugmentedPetType {
            id: 0,
            name: "Unknown".to_string(),
        },
        |p| AugmentedPetType {
            id,
            name: p.name.to_string(),
        },
    )
}

fn deser<'de, T: Deserialize<'de>>(raw: &'de [u8]) -> RpcResult<T> {
    serde_json::from_slice(raw).map_err(|e| RpcError::Deser(format!("{}", e)))
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
struct AugmentedPet {
    pub birthdate: Date,
    pub id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "petType")]
    pub pet_type: AugmentedPetType,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
struct AugmentedPetType {
    pub id: u64,
    pub name: String,
}
