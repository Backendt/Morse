use serde::{Serialize, Deserialize};
use tokio::sync::{
    mpsc::UnboundedSender,
    RwLock
};
use std::collections::HashMap;
use std::sync::Arc;
use futures::stream::SplitSink;
use warp::ws::{WebSocket, Message};
use crate::{
    database::RedisCon,
    models::Response,
    models::errors::{
        RequestResult,
        RequestError::InvalidRequest
    }
};

pub type WsSink = SplitSink<WebSocket, Message>;
pub type UsersChannels = RwLock<HashMap<String, UnboundedSender<Message>>>;

#[derive(Debug, Clone)]
pub struct WsEnvironment {
    pub username: String,
    pub users_channels: Arc<UsersChannels>,
    pub redis: RedisCon
}
impl WsEnvironment {
    pub fn redis(&self) -> RedisCon {
        self.redis.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    CreateRoom,
    Invite,
    Join,
    Leave,
    Message,
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
