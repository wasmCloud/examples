#[macro_use]
extern crate wascc_codec as codec;

#[macro_use]
extern crate log;

use codec::capabilities::{CapabilityProvider, Dispatcher, NullDispatcher};
use codec::core::OP_CONFIGURE;
use codec::eventstreams::{self, Event, StreamQuery, StreamResults, WriteResponse};
use wascc_codec::core::CapabilityConfiguration;
use wascc_codec::{deserialize, serialize};

use std::error::Error;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::{SystemTime, UNIX_EPOCH},
};

fn since_the_epoch() -> std::time::Duration {
    let start = SystemTime::now();
    start
        .duration_since(UNIX_EPOCH)
        .expect("A timey wimey problem has occurred!")
}

struct EventWrapper {
    event: Event,
    timestamp: u64,
}

capability_provider!(TestStreamsProvider, TestStreamsProvider::new);

const CAPABILITY_ID: &str = "wascc:eventstreams";

pub struct TestStreamsProvider {
    dispatcher: RwLock<Box<dyn Dispatcher>>,
    streams: Arc<RwLock<HashMap<String, Vec<EventWrapper>>>>,
}

impl Default for TestStreamsProvider {
    fn default() -> Self {
        match env_logger::try_init() {
            Ok(_) => {}
            Err(_) => {}
        };

        TestStreamsProvider {
            dispatcher: RwLock::new(Box::new(NullDispatcher::new())),
            streams: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl TestStreamsProvider {
    pub fn new() -> Self {
        Self::default()
    }

    fn configure(&self, config: CapabilityConfiguration) -> Result<Vec<u8>, Box<dyn Error>> {
        self.streams.write().unwrap().insert(config.module, vec![]);
        Ok(vec![])
    }

    fn write_event(&self, actor: &str, event: Event) -> Result<Vec<u8>, Box<dyn Error>> {
        let event_id = self.gen_id(actor);
        let evt = Event {
            event_id: event_id.clone(),
            stream: event.stream,
            values: event.values,
        };
        let wrapper = EventWrapper {
            event: evt,
            timestamp: since_the_epoch().as_secs(),
        };
        self.streams
            .write()
            .unwrap()
            .entry(actor.to_string())
            .and_modify(|e| e.push(wrapper));

        Ok(serialize(WriteResponse { event_id })?)
    }

    fn gen_id(&self, actor: &str) -> String {
        format!("event-{}", self.streams.read().unwrap()[actor].len())
    }

    fn query_stream(&self, actor: &str, query: StreamQuery) -> Result<Vec<u8>, Box<dyn Error>> {
        let sid = query.stream_id.to_string();
        let lock = self.streams.read().unwrap();
        let iter = lock[actor].iter().filter(|e| e.event.stream == sid);
        let items: Vec<_> = if let Some(time_range) = query.range {
            if query.count > 0 {
                iter.filter(|w| {
                    w.timestamp >= time_range.min_time && w.timestamp <= time_range.max_time
                })
                .take(query.count as usize)
                .collect()
            } else {
                iter.filter(|w| {
                    w.timestamp >= time_range.min_time && w.timestamp <= time_range.max_time
                })
                .collect()
            }
        } else {
            if query.count > 0 {
                iter.take(query.count as usize).collect()
            } else {
                iter.collect()
            }
        };
        let mut events = Vec::new();
        for evt in items {
            events.push(evt.event.clone());
        }

        Ok(serialize(StreamResults { events })?)
    }
}

impl CapabilityProvider for TestStreamsProvider {
    fn capability_id(&self) -> &'static str {
        CAPABILITY_ID
    }

    // Invoked by the runtime host to give this provider plugin the ability to communicate
    // with actors
    fn configure_dispatch(&self, dispatcher: Box<dyn Dispatcher>) -> Result<(), Box<dyn Error>> {
        trace!("Dispatcher received.");
        let mut lock = self.dispatcher.write().unwrap();
        *lock = dispatcher;

        Ok(())
    }

    fn name(&self) -> &'static str {
        "waSCC Event Streams Provider (Redis)"
    }

    // Invoked by host runtime to allow an actor to make use of the capability
    // All providers MUST handle the "configure" message, even if no work will be done
    fn handle_call(&self, actor: &str, op: &str, msg: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        trace!("Received host call from {}, operation - {}", actor, op);

        match op {
            OP_CONFIGURE if actor == "system" => self.configure(deserialize(msg)?),
            eventstreams::OP_WRITE_EVENT => self.write_event(actor, deserialize(msg)?),
            eventstreams::OP_QUERY_STREAM => self.query_stream(actor, deserialize(msg)?),
            _ => Err("bad dispatch".into()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;
    // **==- REQUIRES A RUNNING REDIS INSTANCE ON LOCALHOST -==**

    #[test]
    fn round_trip() {
        let prov = TestStreamsProvider::new();
        let config = CapabilityConfiguration {
            module: "testing-actor".to_string(),
            values: gen_config(),
        };

        prov.configure(config).unwrap();

        for _ in 0..6 {
            let ev = Event {
                event_id: "".to_string(),
                stream: "my-stream".to_string(),
                values: gen_values(),
            };
            let buf = serialize(&ev).unwrap();
            let _res = prov
                .handle_call("testing-actor", eventstreams::OP_WRITE_EVENT, &buf)
                .unwrap();
        }

        let query = StreamQuery {
            count: 0,
            range: None,
            stream_id: "my-stream".to_string(),
        };
        let buf = serialize(&query).unwrap();
        let res = prov
            .handle_call("testing-actor", eventstreams::OP_QUERY_STREAM, &buf)
            .unwrap();
        let query_res = deserialize::<StreamResults>(res.as_ref()).unwrap();
        assert_eq!(6, query_res.events.len());
        assert_eq!(query_res.events[0].values["scruffy-looking"], "nerf-herder");
    }

    fn gen_config() -> HashMap<String, String> {
        let mut h = HashMap::new();
        h.insert("URL".to_string(), "redis://0.0.0.0:6379/".to_string());
        h
    }

    fn gen_values() -> HashMap<String, String> {
        let mut h = HashMap::new();
        h.insert("test".to_string(), "ok".to_string());
        h.insert("scruffy-looking".to_string(), "nerf-herder".to_string());
        h
    }
}
