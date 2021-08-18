mod generated;

pub use generated::*;

impl RoomAck {
    pub fn success() -> RoomAck {
        RoomAck {
            success: true,
            error: None,
        }
    }

    pub fn fail(reason: &str) -> RoomAck {
        RoomAck {
            success: false,
            error: Some(reason.to_string()),
        }
    }
}
