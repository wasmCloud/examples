// This file is generated automatically using wasmcloud/weld-codegen and smithy model definitions
//

#![allow(unused_imports, clippy::ptr_arg, clippy::needless_lifetimes)]
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, io::Write, string::ToString};
use wasmbus_rpc::{
    deserialize, serialize, Context, Message, MessageDispatch, RpcError, RpcResult, SendOpts,
    Timestamp, Transport,
};

pub const SMITHY_VERSION: &str = "1.0";

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct AddPetRequest {
    #[serde(rename = "ownerId")]
    pub owner_id: u64,
    pub pet: Pet,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CreateOwnerReply {
    pub id: u64,
    #[serde(default)]
    pub success: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct FindOwnerReply {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<Owner>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct FindPetReply {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pet: Option<Pet>,
}

/// Request to list visits
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ListVisitsRequest {
    #[serde(rename = "ownerId")]
    pub owner_id: u64,
    #[serde(rename = "petIds")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pet_ids: Option<PetIdList>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Owner {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default)]
    pub email: String,
    #[serde(rename = "firstName")]
    #[serde(default)]
    pub first_name: String,
    pub id: u64,
    #[serde(rename = "lastName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,
}

pub type OwnersList = Vec<Owner>;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Pet {
    pub birthdate: Date,
    pub id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "petType")]
    pub pet_type: u64,
}

pub type PetIdList = Vec<u64>;

pub type PetList = Vec<Pet>;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PetType {
    pub id: u64,
    #[serde(default)]
    pub name: String,
}

pub type PetTypeList = Vec<PetType>;

/// Request to record a visit
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct RecordVisitRequest {
    #[serde(rename = "ownerId")]
    pub owner_id: u64,
    pub visit: Visit,
}

pub type SpecialtyList = Vec<String>;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct UpdateOwnerReply {
    #[serde(default)]
    pub success: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Vet {
    #[serde(rename = "firstName")]
    #[serde(default)]
    pub first_name: String,
    pub id: u64,
    #[serde(rename = "lastName")]
    #[serde(default)]
    pub last_name: String,
    pub specialties: SpecialtyList,
}

pub type VetList = Vec<Vet>;

/// The core metadata for a veterinarian visit
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Visit {
    /// The date the visit occurred
    pub date: Date,
    /// Description of the visit
    #[serde(default)]
    pub description: String,
    /// The ID of the owner for this visit
    #[serde(rename = "ownerId")]
    pub owner_id: u64,
    /// The ID of the pet involved in the visit
    #[serde(rename = "petId")]
    pub pet_id: u64,
    /// The time the visit occurred
    pub time: Time,
    /// ID of the veterinarian who saw the given pet on this visit
    #[serde(rename = "vetId")]
    pub vet_id: u64,
}

pub type VisitList = Vec<Visit>;

/// wasmbus.actorReceive
#[async_trait]
pub trait Visits {
    /// Retrieve a list of visits for a given owner and an optional
    /// list of pet IDs
    async fn list_visits(&self, ctx: &Context, arg: &ListVisitsRequest) -> RpcResult<VisitList>;
    /// Records a new visit
    async fn record_visit(&self, ctx: &Context, arg: &RecordVisitRequest) -> RpcResult<bool>;
}

/// VisitsReceiver receives messages defined in the Visits service trait
#[doc(hidden)]
#[async_trait]
pub trait VisitsReceiver: MessageDispatch + Visits {
    async fn dispatch(&self, ctx: &Context, message: &Message<'_>) -> RpcResult<Message<'_>> {
        match message.method {
            "ListVisits" => {
                let value: ListVisitsRequest = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = Visits::list_visits(self, ctx, &value).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "Visits.ListVisits",
                    arg: Cow::Owned(buf),
                })
            }
            "RecordVisit" => {
                let value: RecordVisitRequest = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = Visits::record_visit(self, ctx, &value).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "Visits.RecordVisit",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "Visits::{}",
                message.method
            ))),
        }
    }
}

/// VisitsSender sends messages to a Visits service
/// client for sending Visits messages
#[derive(Debug)]
pub struct VisitsSender<T: Transport> {
    transport: T,
}

impl<T: Transport> VisitsSender<T> {
    /// Constructs a VisitsSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> VisitsSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl VisitsSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> Visits for VisitsSender<T> {
    #[allow(unused)]
    /// Retrieve a list of visits for a given owner and an optional
    /// list of pet IDs
    async fn list_visits(&self, ctx: &Context, arg: &ListVisitsRequest) -> RpcResult<VisitList> {
        let buf = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Visits.ListVisits",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "ListVisits", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// Records a new visit
    async fn record_visit(&self, ctx: &Context, arg: &RecordVisitRequest) -> RpcResult<bool> {
        let buf = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Visits.RecordVisit",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "RecordVisit", e)))?;
        Ok(value)
    }
}

/// wasmbus.actorReceive
#[async_trait]
pub trait Vets {
    async fn list_vets(&self, ctx: &Context) -> RpcResult<VetList>;
}

/// VetsReceiver receives messages defined in the Vets service trait
#[doc(hidden)]
#[async_trait]
pub trait VetsReceiver: MessageDispatch + Vets {
    async fn dispatch(&self, ctx: &Context, message: &Message<'_>) -> RpcResult<Message<'_>> {
        match message.method {
            "ListVets" => {
                let resp = Vets::list_vets(self, ctx).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "Vets.ListVets",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "Vets::{}",
                message.method
            ))),
        }
    }
}

/// VetsSender sends messages to a Vets service
/// client for sending Vets messages
#[derive(Debug)]
pub struct VetsSender<T: Transport> {
    transport: T,
}

impl<T: Transport> VetsSender<T> {
    /// Constructs a VetsSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> VetsSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl VetsSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> Vets for VetsSender<T> {
    #[allow(unused)]
    async fn list_vets(&self, ctx: &Context) -> RpcResult<VetList> {
        let buf = *b"";
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Vets.ListVets",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "ListVets", e)))?;
        Ok(value)
    }
}

/// Description of Petclinic service
/// wasmbus.actorReceive
#[async_trait]
pub trait Petclinic {
    /// Converts the input string to a result
    async fn convert<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<String>;
}

/// PetclinicReceiver receives messages defined in the Petclinic service trait
/// Description of Petclinic service
#[doc(hidden)]
#[async_trait]
pub trait PetclinicReceiver: MessageDispatch + Petclinic {
    async fn dispatch(&self, ctx: &Context, message: &Message<'_>) -> RpcResult<Message<'_>> {
        match message.method {
            "Convert" => {
                let value: String = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = Petclinic::convert(self, ctx, &value).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "Petclinic.Convert",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "Petclinic::{}",
                message.method
            ))),
        }
    }
}

/// PetclinicSender sends messages to a Petclinic service
/// Description of Petclinic service
/// client for sending Petclinic messages
#[derive(Debug)]
pub struct PetclinicSender<T: Transport> {
    transport: T,
}

impl<T: Transport> PetclinicSender<T> {
    /// Constructs a PetclinicSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> PetclinicSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl PetclinicSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> Petclinic for PetclinicSender<T> {
    #[allow(unused)]
    /// Converts the input string to a result
    async fn convert<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<String> {
        let buf = serialize(&arg.to_string())?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Petclinic.Convert",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "Convert", e)))?;
        Ok(value)
    }
}

/// wasmbus.actorReceive
#[async_trait]
pub trait Customers {
    async fn create_owner(&self, ctx: &Context, arg: &Owner) -> RpcResult<CreateOwnerReply>;
    async fn find_owner(&self, ctx: &Context, arg: &u64) -> RpcResult<FindOwnerReply>;
    async fn list_owners(&self, ctx: &Context) -> RpcResult<OwnersList>;
    async fn update_owner(&self, ctx: &Context, arg: &Owner) -> RpcResult<UpdateOwnerReply>;
    async fn list_pet_types(&self, ctx: &Context) -> RpcResult<PetTypeList>;
    async fn add_pet(&self, ctx: &Context, arg: &AddPetRequest) -> RpcResult<bool>;
    async fn remove_pet(&self, ctx: &Context, arg: &u64) -> RpcResult<bool>;
    async fn update_pet(&self, ctx: &Context, arg: &Pet) -> RpcResult<bool>;
    async fn list_pets(&self, ctx: &Context, arg: &u64) -> RpcResult<PetList>;
    async fn find_pet(&self, ctx: &Context, arg: &u64) -> RpcResult<FindPetReply>;
}

/// CustomersReceiver receives messages defined in the Customers service trait
#[doc(hidden)]
#[async_trait]
pub trait CustomersReceiver: MessageDispatch + Customers {
    async fn dispatch(&self, ctx: &Context, message: &Message<'_>) -> RpcResult<Message<'_>> {
        match message.method {
            "CreateOwner" => {
                let value: Owner = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = Customers::create_owner(self, ctx, &value).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "Customers.CreateOwner",
                    arg: Cow::Owned(buf),
                })
            }
            "FindOwner" => {
                let value: u64 = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = Customers::find_owner(self, ctx, &value).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "Customers.FindOwner",
                    arg: Cow::Owned(buf),
                })
            }
            "ListOwners" => {
                let resp = Customers::list_owners(self, ctx).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "Customers.ListOwners",
                    arg: Cow::Owned(buf),
                })
            }
            "UpdateOwner" => {
                let value: Owner = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = Customers::update_owner(self, ctx, &value).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "Customers.UpdateOwner",
                    arg: Cow::Owned(buf),
                })
            }
            "ListPetTypes" => {
                let resp = Customers::list_pet_types(self, ctx).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "Customers.ListPetTypes",
                    arg: Cow::Owned(buf),
                })
            }
            "AddPet" => {
                let value: AddPetRequest = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = Customers::add_pet(self, ctx, &value).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "Customers.AddPet",
                    arg: Cow::Owned(buf),
                })
            }
            "RemovePet" => {
                let value: u64 = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = Customers::remove_pet(self, ctx, &value).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "Customers.RemovePet",
                    arg: Cow::Owned(buf),
                })
            }
            "UpdatePet" => {
                let value: Pet = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = Customers::update_pet(self, ctx, &value).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "Customers.UpdatePet",
                    arg: Cow::Owned(buf),
                })
            }
            "ListPets" => {
                let value: u64 = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = Customers::list_pets(self, ctx, &value).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "Customers.ListPets",
                    arg: Cow::Owned(buf),
                })
            }
            "FindPet" => {
                let value: u64 = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = Customers::find_pet(self, ctx, &value).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "Customers.FindPet",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "Customers::{}",
                message.method
            ))),
        }
    }
}

/// CustomersSender sends messages to a Customers service
/// client for sending Customers messages
#[derive(Debug)]
pub struct CustomersSender<T: Transport> {
    transport: T,
}

impl<T: Transport> CustomersSender<T> {
    /// Constructs a CustomersSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> CustomersSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl CustomersSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> Customers for CustomersSender<T> {
    #[allow(unused)]
    async fn create_owner(&self, ctx: &Context, arg: &Owner) -> RpcResult<CreateOwnerReply> {
        let buf = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Customers.CreateOwner",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "CreateOwner", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn find_owner(&self, ctx: &Context, arg: &u64) -> RpcResult<FindOwnerReply> {
        let buf = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Customers.FindOwner",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "FindOwner", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn list_owners(&self, ctx: &Context) -> RpcResult<OwnersList> {
        let buf = *b"";
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Customers.ListOwners",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "ListOwners", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn update_owner(&self, ctx: &Context, arg: &Owner) -> RpcResult<UpdateOwnerReply> {
        let buf = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Customers.UpdateOwner",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "UpdateOwner", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn list_pet_types(&self, ctx: &Context) -> RpcResult<PetTypeList> {
        let buf = *b"";
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Customers.ListPetTypes",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "ListPetTypes", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn add_pet(&self, ctx: &Context, arg: &AddPetRequest) -> RpcResult<bool> {
        let buf = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Customers.AddPet",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "AddPet", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn remove_pet(&self, ctx: &Context, arg: &u64) -> RpcResult<bool> {
        let buf = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Customers.RemovePet",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "RemovePet", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn update_pet(&self, ctx: &Context, arg: &Pet) -> RpcResult<bool> {
        let buf = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Customers.UpdatePet",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "UpdatePet", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn list_pets(&self, ctx: &Context, arg: &u64) -> RpcResult<PetList> {
        let buf = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Customers.ListPets",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "ListPets", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn find_pet(&self, ctx: &Context, arg: &u64) -> RpcResult<FindPetReply> {
        let buf = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Customers.FindPet",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "FindPet", e)))?;
        Ok(value)
    }
}
