use serde::{Serialize, Deserialize};
use tokio::sync::{
    mpsc::UnboundedSender,
    RwLock
};
use std::collections::HashMap;
use futures::stream::SplitSink;
use warp::ws::{WebSocket, Message};

pub type WsSink = SplitSink<WebSocket, Message>;
pub type UsersChannels = RwLock<HashMap<String, UnboundedSender<Message>>>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Invite,
    Accept,
    Refuse,
    Established,
    Message,
    Close
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Success,
    Error
}

#[derive(Debug, Serialize)]
pub struct Response {
    status: Status,
    message: String
}
impl Response {
    pub fn err(message: &str) -> Self {
        Self {
            status: Status::Error,
            message: message.to_string()
        }
    }

    pub fn success(message: &str) -> Self {
        Self {
            status: Status::Success,
            message: message.to_string()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub action: Action,
    pub target: String,
    pub body: Option<String>
}
impl Request {
    pub fn new(action: Action, target: &String) -> Self {
        Self {
            action,
            target: target.clone(),
            body: None
        }
    }

    pub fn body(action: Action, target: &String, body: String) -> Self {
        Self {
            action,
            target: target.clone(),
            body: Some(body)
        }
    }
}

pub trait Messageable: Serialize {
    fn as_message(&self) -> Message {
        let as_text = serde_json::to_string(self)
            .expect("Could not serialize message");
        Message::text(as_text)
    }
}
impl Messageable for Request {}
impl Messageable for Response {}
