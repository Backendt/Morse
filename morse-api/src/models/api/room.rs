use serde::{self, Serialize};
use super::{MessageType, MessageBody};

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum RoomEvent {
    Leave,
    Join,
}

#[derive(Debug, Serialize, Clone)]
pub struct RoomBody {
    pub event: RoomEvent,
    pub event_user: String,
    pub room: String,
    pub users: Vec<String>
}

impl MessageBody for RoomBody {
    fn get_type(&self) -> MessageType {
        MessageType::Room
    }
}
