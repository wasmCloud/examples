#[macro_use]
extern crate wascc_codec as codec;
#[macro_use]
extern crate log;

mod kv;
use crate::kv::KeyValueStore;
use codec::capabilities::{CapabilityProvider, Dispatcher, NullDispatcher};
use codec::core::{OP_BIND_ACTOR, OP_REMOVE_ACTOR};
use std::error::Error;
use std::sync::{Arc, RwLock};
use wascc_codec::{deserialize, serialize};
use wasmcloud_actor_core::CapabilityConfiguration;
use wasmcloud_actor_keyvalue::*;

#[cfg(not(feature = "static_plugin"))]
capability_provider!(KeyvalueProvider, KeyvalueProvider::new);

#[allow(unused)]
const CAPABILITY_ID: &str = "wasmcloud:keyvalue";
const SYSTEM_ACTOR: &str = "system";

#[derive(Clone)]
pub struct KeyvalueProvider {
    dispatcher: Arc<RwLock<Box<dyn Dispatcher>>>,
    store: Arc<RwLock<KeyValueStore>>,
}

impl Default for KeyvalueProvider {
    fn default() -> Self {
        if env_logger::try_init().is_ok() {};
        KeyvalueProvider {
            dispatcher: Arc::new(RwLock::new(Box::new(NullDispatcher::new()))),
            store: Arc::new(RwLock::new(KeyValueStore::new())),
        }
    }
}

impl KeyvalueProvider {
    pub fn new() -> Self {
        Self::default()
    }

    fn configure(
        &self,
        _config: CapabilityConfiguration,
    ) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        // Do nothing here
        Ok(vec![])
    }

    fn remove_actor(
        &self,
        _config: CapabilityConfiguration,
    ) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        // Do nothing here
        Ok(vec![])
    }

    fn add(&self, _actor: &str, req: AddArgs) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        let mut store = self.store.write().unwrap();
        let res: i32 = store.incr(&req.key, req.value)?;
        let resp = AddResponse { value: res };

        Ok(serialize(resp)?)
    }

    fn del(&self, _actor: &str, req: DelArgs) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        let mut store = self.store.write().unwrap();
        store.del(&req.key)?;
        let resp = DelResponse { key: req.key };

        Ok(serialize(resp)?)
    }

    fn get(&self, _actor: &str, req: GetArgs) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
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

    fn list_clear(
        &self,
        actor: &str,
        req: ClearArgs,
    ) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        self.del(actor, DelArgs { key: req.key })
    }

    fn list_range(
        &self,
        _actor: &str,
        req: RangeArgs,
    ) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        let store = self.store.read().unwrap();
        let result: Vec<String> = store.lrange(&req.key, req.start as _, req.stop as _)?;
        Ok(serialize(ListRangeResponse { values: result })?)
    }

    fn list_push(
        &self,
        _actor: &str,
        req: PushArgs,
    ) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        let mut store = self.store.write().unwrap();
        let result: i32 = store.lpush(&req.key, req.value)?;
        Ok(serialize(ListResponse { new_count: result })?)
    }

    fn set(&self, _actor: &str, req: SetArgs) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        let mut store = self.store.write().unwrap();
        store.set(&req.key, req.value.clone())?;
        Ok(serialize(SetResponse { value: req.value })?)
    }

    fn list_del_item(
        &self,
        _actor: &str,
        req: ListItemDeleteArgs,
    ) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        let mut store = self.store.write().unwrap();
        let result: i32 = store.lrem(&req.key, req.value)?;
        Ok(serialize(ListResponse { new_count: result })?)
    }

    fn set_add(
        &self,
        _actor: &str,
        req: SetAddArgs,
    ) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        let mut store = self.store.write().unwrap();
        let result: i32 = store.sadd(&req.key, req.value)?;
        Ok(serialize(SetOperationResponse { new_count: result })?)
    }

    fn set_remove(
        &self,
        _actor: &str,
        req: SetRemoveArgs,
    ) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        let mut store = self.store.write().unwrap();
        let result: i32 = store.srem(&req.key, req.value)?;
        Ok(serialize(SetOperationResponse { new_count: result })?)
    }

    fn set_union(
        &self,
        _actor: &str,
        req: SetUnionArgs,
    ) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        let store = self.store.read().unwrap();
        let result: Vec<String> = store.sunion(req.keys)?;
        Ok(serialize(SetQueryResponse { values: result })?)
    }

    fn set_intersect(
        &self,
        _actor: &str,
        req: SetIntersectionArgs,
    ) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        let store = self.store.read().unwrap();
        let result: Vec<String> = store.sinter(req.keys)?;
        Ok(serialize(SetQueryResponse { values: result })?)
    }

    fn set_query(
        &self,
        _actor: &str,
        req: SetQueryArgs,
    ) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        let store = self.store.read().unwrap();
        let result: Vec<String> = store.smembers(req.key)?;
        Ok(serialize(SetQueryResponse { values: result })?)
    }

    fn exists(
        &self,
        _actor: &str,
        req: KeyExistsArgs,
    ) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        let store = self.store.read().unwrap();
        let result: bool = store.exists(&req.key)?;
        Ok(serialize(GetResponse {
            value: "".to_string(),
            exists: result,
        })?)
    }
}

impl CapabilityProvider for KeyvalueProvider {
    // Invoked by the runtime host to give this provider plugin the ability to communicate
    // with actors
    fn configure_dispatch(
        &self,
        dispatcher: Box<dyn Dispatcher>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        trace!("Dispatcher received.");
        let mut lock = self.dispatcher.write().unwrap();
        *lock = dispatcher;

        Ok(())
    }

    // Invoked by host runtime to allow an actor to make use of the capability
    // All providers MUST handle the "configure" message, even if no work will be done
    fn handle_call(
        &self,
        actor: &str,
        op: &str,
        msg: &[u8],
    ) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        trace!("Received host call from {}, operation - {}", actor, op);

        match op {
            OP_BIND_ACTOR if actor == SYSTEM_ACTOR => self.configure(deserialize(msg)?),
            OP_REMOVE_ACTOR if actor == SYSTEM_ACTOR => self.remove_actor(deserialize(msg)?),
            OP_ADD => self.add(actor, deserialize(msg)?),
            OP_DEL => self.del(actor, deserialize(msg)?),
            OP_GET => self.get(actor, deserialize(msg)?),
            OP_CLEAR => self.list_clear(actor, deserialize(msg)?),
            OP_RANGE => self.list_range(actor, deserialize(msg)?),
            OP_PUSH => self.list_push(actor, deserialize(msg)?),
            OP_SET => self.set(actor, deserialize(msg)?),
            OP_LIST_DEL => self.list_del_item(actor, deserialize(msg)?),
            OP_SET_ADD => self.set_add(actor, deserialize(msg)?),
            OP_SET_REMOVE => self.set_remove(actor, deserialize(msg)?),
            OP_SET_UNION => self.set_union(actor, deserialize(msg)?),
            OP_SET_INTERSECT => self.set_intersect(actor, deserialize(msg)?),
            OP_SET_QUERY => self.set_query(actor, deserialize(msg)?),
            OP_KEY_EXISTS => self.exists(actor, deserialize(msg)?),
            _ => Err("bad dispatch".into()),
        }
    }

    /// No cleanup needed
    fn stop(&self) {}
}
