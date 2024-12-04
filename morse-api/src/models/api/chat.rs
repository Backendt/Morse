use serde::{self, Serialize};
use super::{MessageBody, MessageType};

#[derive(Debug, Serialize, Clone)]
pub struct ChatMessage {
    pub sender: String,
    pub room: String,
    pub content: String
}
impl MessageBody for ChatMessage {
    fn get_type(&self) -> MessageType {
        MessageType::Chat
    }
}
