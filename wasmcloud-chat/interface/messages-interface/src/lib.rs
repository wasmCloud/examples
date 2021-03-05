mod generated;

pub use generated::*;

impl ProcessAck {
    pub fn success(msg_id: &str) -> ProcessAck {
        ProcessAck {
            message_id: msg_id.to_string(),
            processed: true,
            error: None,
        }
    }

    pub fn fail(msg_id: &str, reason: &str) -> ProcessAck {
        ProcessAck {
            message_id: msg_id.to_string(),
            processed: false,
            error: Some(reason.to_string()),
        }
    }
}