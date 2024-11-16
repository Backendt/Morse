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

#[derive(Serialize, Deserialize)]
pub struct WsMessage {
    pub action: String,
    pub target: String,
    pub content: Vec<u8>
}

