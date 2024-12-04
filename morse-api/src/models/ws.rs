use tokio::sync::{
    mpsc::UnboundedSender,
    RwLock
};
use std::collections::HashMap;
use std::sync::Arc;
use futures::stream::SplitSink;
use warp::ws::{WebSocket, Message};
use crate::database::RedisCon;

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
