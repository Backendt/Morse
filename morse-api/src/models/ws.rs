use serde::{Serialize, Deserialize};
use tokio::sync::{
    mpsc::UnboundedSender,
    RwLock
};
use std::collections::HashMap;
use futures::stream::SplitSink;
use warp::ws::{WebSocket, Message};
use crate::models::errors::{
    RequestResult,
    RequestError::InvalidRequest
};

pub type WsSink = SplitSink<WebSocket, Message>;
pub type UsersChannels = RwLock<HashMap<String, UnboundedSender<Message>>>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    CreateRoom,
    Invite,
    Join,
    Leave,
    Message,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Success,
    Error
}

pub trait Messageable: Serialize {
    fn as_message(&self) -> Message {
        let as_text = serde_json::to_string(self)
            .expect("Could not serialize message");
        Message::text(as_text)
    }
}

#[derive(Debug, Serialize)]
pub struct ChatMessage {
    pub sender: String,
    pub room: String,
    pub content: String
}
impl Messageable for ChatMessage {}

#[derive(Debug, Serialize)]
pub struct Response {
    status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Action>,
    message: String
}
impl Response {
    pub fn err(message: &str) -> Self {
        Self {
            status: Status::Error,
            action: None,
            message: message.to_owned()
        }
    }

    pub fn action_err(action: &Action, message: &str) -> Self {
        Self {
            status: Status::Error,
            action: Some(action.clone()),
            message: message.to_owned()
        }
    }

    pub fn success(action: &Action, message: &str) -> Self {
        Self {
            status: Status::Success,
            action: Some(action.clone()),
            message: message.to_owned()
        }
    }
}
impl Messageable for Response {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Request {
    pub action: Action,
    pub target: Option<String>,
    pub body: Option<String>
}
impl Messageable for Request {}
impl Request {
    pub fn new(action: Action, target: String, body: String) -> Self {
        Self {
            action,
            target: Some(target),
            body: Some(body)
        }
    }

    pub fn body(&self) -> RequestResult<String> {
        self.body.clone()
            .ok_or_else(||
                InvalidRequest("The body is required.".to_owned())
            )
    }

    pub fn target(&self) -> RequestResult<String> {
        self.target.clone()
            .ok_or_else(||
                InvalidRequest("The target is required.".to_owned())
            )
    }
}
