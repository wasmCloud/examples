#[macro_use]
extern crate wascc_codec as codec;

#[macro_use]
extern crate log;

mod kv;

use crate::kv::KeyValueStore;
use codec::capabilities::{
    CapabilityDescriptor, CapabilityProvider, Dispatcher, NullDispatcher, OperationDirection,
    OP_GET_CAPABILITY_DESCRIPTOR,
};
use codec::core::{OP_BIND_ACTOR, OP_REMOVE_ACTOR};
use codec::keyvalue;
use codec::keyvalue::*;
use wascc_codec::core::CapabilityConfiguration;
use wascc_codec::{deserialize, serialize};

use std::error::Error;
use std::sync::RwLock;

#[cfg(not(feature = "static_plugin"))]
capability_provider!(KeyvalueProvider, KeyvalueProvider::new);

const CAPABILITY_ID: &str = "wascc:keyvalue";
const SYSTEM_ACTOR: &str = "system";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const REVISION: u32 = 0; // Increment for each crates publish

pub struct KeyvalueProvider {
    dispatcher: RwLock<Box<dyn Dispatcher>>,
    store: RwLock<KeyValueStore>,
}

impl Default for KeyvalueProvider {
    fn default() -> Self {
        match env_logger::try_init() {
            Ok(_) => {}
            Err(_) => {}
        };
        KeyvalueProvider {
            dispatcher: RwLock::new(Box::new(NullDispatcher::new())),
            store: RwLock::new(KeyValueStore::new()),
        }
    }
}

impl KeyvalueProvider {
    pub fn new() -> Self {
        Self::default()
    }

    fn configure(&self, _config: CapabilityConfiguration) -> Result<Vec<u8>, Box<dyn Error>> {
        // Do nothing here
        Ok(vec![])
    }

    fn remove_actor(&self, _config: CapabilityConfiguration) -> Result<Vec<u8>, Box<dyn Error>> {
        // Do nothing here
        Ok(vec![])
    }

    fn add(&self, _actor: &str, req: AddRequest) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut store = self.store.write().unwrap();
        let res: i32 = store.incr(&req.key, req.value)?;
        let resp = AddResponse { value: res };

        Ok(serialize(resp)?)
    }

    fn del(&self, _actor: &str, req: DelRequest) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut store = self.store.write().unwrap();
        store.del(&req.key)?;
        let resp = DelResponse { key: req.key };

        Ok(serialize(resp)?)
    }

    fn get(&self, _actor: &str, req: GetRequest) -> Result<Vec<u8>, Box<dyn Error>> {
        let store = self.store.read().unwrap();
        if !store.exists(&req.key)? {
            Ok(serialize(GetResponse {
                value: String::from(""),
                exists: false,
            })?)
        } else {
            let v = store.get(&req.key);
            Ok(serialize(match v {
                Ok(s) => GetResponse {
                    value: s,
                    exists: true,
                },
                Err(e) => {
                    eprint!("GET for {} failed: {}", &req.key, e);
                    GetResponse {
                        value: "".to_string(),
                        exists: false,
                    }
                }
            })?)
        }
    }

    fn list_clear(&self, actor: &str, req: ListClearRequest) -> Result<Vec<u8>, Box<dyn Error>> {
        self.del(actor, DelRequest { key: req.key })
    }

    fn list_range(&self, _actor: &str, req: ListRangeRequest) -> Result<Vec<u8>, Box<dyn Error>> {
        let store = self.store.read().unwrap();
        let result: Vec<String> = store.lrange(&req.key, req.start as _, req.stop as _)?;
        Ok(serialize(ListRangeResponse { values: result })?)
    }

    fn list_push(&self, _actor: &str, req: ListPushRequest) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut store = self.store.write().unwrap();
        let result: i32 = store.lpush(&req.key, req.value)?;
        Ok(serialize(ListResponse { new_count: result })?)
    }

    fn set(&self, _actor: &str, req: SetRequest) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut store = self.store.write().unwrap();
        store.set(&req.key, req.value.clone())?;
        Ok(serialize(SetResponse { value: req.value })?)
    }

    fn list_del_item(
        &self,
        _actor: &str,
        req: ListDelItemRequest,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut store = self.store.write().unwrap();
        let result: i32 = store.lrem(&req.key, req.value)?;
        Ok(serialize(ListResponse { new_count: result })?)
    }

    fn set_add(&self, _actor: &str, req: SetAddRequest) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut store = self.store.write().unwrap();
        let result: i32 = store.sadd(&req.key, req.value)?;
        Ok(serialize(SetOperationResponse { new_count: result })?)
    }

    fn set_remove(&self, _actor: &str, req: SetRemoveRequest) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut store = self.store.write().unwrap();
        let result: i32 = store.srem(&req.key, req.value)?;
        Ok(serialize(SetOperationResponse { new_count: result })?)
    }

    fn set_union(&self, _actor: &str, req: SetUnionRequest) -> Result<Vec<u8>, Box<dyn Error>> {
        let store = self.store.read().unwrap();
        let result: Vec<String> = store.sunion(req.keys)?;
        Ok(serialize(SetQueryResponse { values: result })?)
    }

    fn set_intersect(
        &self,
        _actor: &str,
        req: SetIntersectionRequest,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let store = self.store.read().unwrap();
        let result: Vec<String> = store.sinter(req.keys)?;
        Ok(serialize(SetQueryResponse { values: result })?)
    }

    fn set_query(&self, _actor: &str, req: SetQueryRequest) -> Result<Vec<u8>, Box<dyn Error>> {
        let store = self.store.read().unwrap();
        let result: Vec<String> = store.smembers(req.key)?;
        Ok(serialize(SetQueryResponse { values: result })?)
    }

    fn exists(&self, _actor: &str, req: KeyExistsQuery) -> Result<Vec<u8>, Box<dyn Error>> {
        let store = self.store.read().unwrap();
        let result: bool = store.exists(&req.key)?;
        Ok(serialize(GetResponse {
            value: "".to_string(),
            exists: result,
        })?)
    }

    fn get_descriptor(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        use OperationDirection::ToProvider;
        Ok(serialize(
            CapabilityDescriptor::builder()
                .id(CAPABILITY_ID)
                .name("waSCC Default Key-Value Provider (In-Memory)")
                .long_description(
                    "A key-value store capability provider built on in-process hash maps",
                )
                .version(VERSION)
                .revision(REVISION)
                .with_operation(OP_ADD, ToProvider, "Performs an atomic addition operation")
                .with_operation(OP_DEL, ToProvider, "Deletes a key from the store")
                .with_operation(OP_GET, ToProvider, "Gets the raw value for a key")
                .with_operation(OP_CLEAR, ToProvider, "Clears a list")
                .with_operation(
                    OP_RANGE,
                    ToProvider,
                    "Selects items from a list within a range",
                )
                .with_operation(OP_PUSH, ToProvider, "Pushes a new item onto a list")
                .with_operation(OP_SET, ToProvider, "Sets the value of a key")
                .with_operation(OP_LIST_DEL, ToProvider, "Deletes an item from a list")
                .with_operation(OP_SET_ADD, ToProvider, "Adds an item to a set")
                .with_operation(OP_SET_REMOVE, ToProvider, "Remove an item from a set")
                .with_operation(
                    OP_SET_UNION,
                    ToProvider,
                    "Returns the union of multiple sets",
                )
                .with_operation(
                    OP_SET_INTERSECT,
                    ToProvider,
                    "Returns the intersection of multiple sets",
                )
                .with_operation(OP_SET_QUERY, ToProvider, "Queries a set")
                .with_operation(
                    OP_KEY_EXISTS,
                    ToProvider,
                    "Returns a boolean indicating if a key exists",
                )
                .build(),
        )?)
    }
}

impl CapabilityProvider for KeyvalueProvider {
    // Invoked by the runtime host to give this provider plugin the ability to communicate
    // with actors
    fn configure_dispatch(&self, dispatcher: Box<dyn Dispatcher>) -> Result<(), Box<dyn Error>> {
        trace!("Dispatcher received.");
        let mut lock = self.dispatcher.write().unwrap();
        *lock = dispatcher;

        Ok(())
    }

    // Invoked by host runtime to allow an actor to make use of the capability
    // All providers MUST handle the "configure" message, even if no work will be done
    fn handle_call(&self, actor: &str, op: &str, msg: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        trace!("Received host call from {}, operation - {}", actor, op);

        match op {
            OP_BIND_ACTOR if actor == SYSTEM_ACTOR => self.configure(deserialize(msg)?),
            OP_REMOVE_ACTOR if actor == SYSTEM_ACTOR => self.remove_actor(deserialize(msg)?),
            OP_GET_CAPABILITY_DESCRIPTOR if actor == SYSTEM_ACTOR => self.get_descriptor(),
            keyvalue::OP_ADD => self.add(actor, deserialize(msg)?),
            keyvalue::OP_DEL => self.del(actor, deserialize(msg)?),
            keyvalue::OP_GET => self.get(actor, deserialize(msg)?),
            keyvalue::OP_CLEAR => self.list_clear(actor, deserialize(msg)?),
            keyvalue::OP_RANGE => self.list_range(actor, deserialize(msg)?),
            keyvalue::OP_PUSH => self.list_push(actor, deserialize(msg)?),
            keyvalue::OP_SET => self.set(actor, deserialize(msg)?),
            keyvalue::OP_LIST_DEL => self.list_del_item(actor, deserialize(msg)?),
            keyvalue::OP_SET_ADD => self.set_add(actor, deserialize(msg)?),
            keyvalue::OP_SET_REMOVE => self.set_remove(actor, deserialize(msg)?),
            keyvalue::OP_SET_UNION => self.set_union(actor, deserialize(msg)?),
            keyvalue::OP_SET_INTERSECT => self.set_intersect(actor, deserialize(msg)?),
            keyvalue::OP_SET_QUERY => self.set_query(actor, deserialize(msg)?),
            keyvalue::OP_KEY_EXISTS => self.exists(actor, deserialize(msg)?),
            _ => Err("bad dispatch".into()),
        }
    }
}
