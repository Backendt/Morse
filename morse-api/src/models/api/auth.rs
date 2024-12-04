use serde::{self, Serialize};
use crate::models::api::{MessageBody, MessageType};

#[derive(Debug, Serialize)]
pub struct TokenBody {
    pub username: String,
    pub token: String
}
impl MessageBody for TokenBody {
    fn get_type(&self) -> MessageType {
        MessageType::Token
    }
}
