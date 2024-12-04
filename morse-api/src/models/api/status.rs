use serde::{self, Serialize};
use super::{MessageBody, MessageType};

#[derive(Debug, Serialize, Clone)]
pub struct StatusBody {
    pub success: bool,
    pub status_code: StatusCode,
    pub message: String
}

impl MessageBody for StatusBody {
    fn get_type(&self) -> MessageType {
        MessageType::Status
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum StatusCode {
    InvalidToken,
    AlreadyConnected,
    InvalidRequest,
    ParseError,
    InternalError,
    Invitation,
    Registration
}
