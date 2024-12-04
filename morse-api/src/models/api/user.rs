use serde::{self, Serialize};
use super::{MessageBody, MessageType};

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum UserRequestType {
    Invite
}

#[derive(Debug, Serialize, Clone)]
pub struct UserBody {
    pub r#type: UserRequestType,
    pub from: String,
    pub content: String
}
impl MessageBody for UserBody {
    fn get_type(&self) -> MessageType {
        MessageType::User
    }
}
