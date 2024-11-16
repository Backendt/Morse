use serde::{Serialize, Deserialize};
use tokio::sync::{
    mpsc,
    RwLock
};
use std::collections::HashMap;
use futures::stream::SplitSink;
use warp::ws::{WebSocket, Message};

pub type WsSink = SplitSink<WebSocket, Message>;
pub type UsersChannels = RwLock<HashMap<String, mpsc::UnboundedSender<Message>>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct WsMessage {
    pub action: String,
    pub target: String,
    pub content: String
}
impl Messageable for WsMessage {}

#[derive(Debug, Serialize)]
pub struct WsStatus {
    pub status: String,
    pub message: String
}
impl Messageable for WsStatus {}

impl WsStatus {
    pub fn new(status: &str, message: &str) -> Self {
        Self {
            status: String::from(status),
            message: String::from(message)
        }
    }

    pub fn err<T>(message: &str) -> Result<T, Self> {
        Err(Self::new("error", message))
    }
}

pub trait Messageable: Serialize {
    fn as_message(&self) -> warp::ws::Message {
        let as_text = serde_json::to_string(self)
            .expect("Could not serialize message");
        Message::text(as_text)
    }
}

