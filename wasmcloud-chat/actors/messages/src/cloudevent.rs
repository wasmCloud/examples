use chrono::prelude::*;
use serde::{Deserialize, Serialize};

/// Repesents a cloudevent v1.0 JSON format message.
/// for ease of use, copy this struct into any library that needs
/// to round-trip these structures. Don't create the unnecessary headache
/// of creating coupling just for the use of cloud events.
#[derive(Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct CloudEvent {
    #[serde(rename = "specversion")]
    pub cloud_events_version: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub source: String, // URI
    #[serde(rename = "id")]
    pub event_id: String,
    #[serde(rename = "time")]
    pub event_time: String,
    #[serde(rename = "datacontenttype")]
    pub content_type: String,
    pub data: serde_json::Value,
}

impl CloudEvent {
    pub fn new_json<T>(
        id: &str,
        event_type: &str,
        source: &str,
        timestamp: u64,
        content: &T,
    ) -> CloudEvent
    where
        T: Serialize,
    {
        let dt = Utc.timestamp(timestamp as i64, 0);
        CloudEvent {
            cloud_events_version: "1.0".to_string(),
            event_type: event_type.to_string(),
            source: source.to_string(),
            event_id: id.to_string(),
            event_time: dt.to_rfc2822(),
            content_type: "application/json".to_string(),
            data: serde_json::to_value(content).unwrap(),
        }
    }
}
