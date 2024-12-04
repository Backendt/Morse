pub mod room;
pub mod status;
pub mod chat;
pub mod user;
pub mod auth;

use serde::{self, Serialize, Deserialize};
use warp::ws::Message;
use crate::models::errors::*;

// Response from API

#[derive(Debug, Serialize)]
pub struct ApiMessage<T: MessageBody> {
    pub r#type: MessageType,
    pub body: T
}
impl <T: MessageBody>ApiMessage<T> {
    pub fn new(body: T) -> ApiMessage<T> {
        ApiMessage {
            r#type: body.get_type(),
            body
        }
    }

    pub fn as_message(&self) -> Message {
        let as_text = serde_json::to_string(self).expect("Could not serialize ApiMessage.");
        Message::text(as_text)
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    Status, // A response from a request
    Chat, // Message sent in room
    Room, // User leaving/joining room
    User, // User requests
    Token // User logging-in
}

pub trait MessageBody: Serialize {
    fn get_type(&self) -> MessageType;
    fn as_message(self) -> Message where Self: Sized {
        ApiMessage::new(self).as_message()
    }
}

// Request to API

#[derive(Debug, Deserialize, Clone)]
pub struct Request {
    pub action: Action,
    pub target: Option<String>,
    pub body: Option<String>
}
impl Request {
    pub fn body(&self) -> RequestResult<String> {
        self.body.clone()
            .ok_or_else(||
                RequestError::InvalidRequest("The body is required.".to_owned())
            )
    }

    pub fn target(&self) -> RequestResult<String> {
        self.target.clone()
            .ok_or_else(||
                RequestError::InvalidRequest("The target is required.".to_owned())
            )
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    CreateRoom,
    Invite,
    Join,
    Leave,
    Message,
}
