use tokio::sync::mpsc::{
    UnboundedReceiver,
    UnboundedSender,
    unbounded_channel,
};
use warp::ws::Message;
use futures::SinkExt;
use std::sync::Arc;

use crate::models::ws::{
    WsSink,
    WsMessage,
    WsStatus,
    UsersChannels,
    Messageable
};

pub async fn add_client(username: &String, users: &Arc<UsersChannels>) -> Result<(UnboundedSender<Message>, UnboundedReceiver<Message>), WsStatus> {
    let already_connected = users.read().await.get(username).is_some();
    if already_connected {
        return WsStatus::err("You are already connected");
    }

    println!("[+] '{username}' connected to websocket.");
    let (user_sender, user_receiver) = unbounded_channel();
    users.write().await.insert(username.clone(), user_sender.clone());
    Ok((user_sender, user_receiver))    
}

pub async fn remove_client(username: &String, users: &Arc<UsersChannels>) {
    println!("[-] '{username}' disconnected from websocket.");
    users.write().await.remove(username);
}

pub async fn start_forwarding(mut receiver: UnboundedReceiver<Message>, mut sink: WsSink) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        while let Some(message) = receiver.recv().await {
            if let Err(err) = sink.send(message).await {
                eprintln!("Could not forward message. {err:?}");
                break;
            }
        }
    })
}

pub fn parse_message(raw_message: Message) -> Result<WsMessage, WsStatus> {
    let content = raw_message.as_bytes(); 
    serde_json::from_slice::<WsMessage>(content)
        .map_err(|_| WsStatus::new("error", "Invalid json."))
}

pub async fn send_message(username: &String, message: WsMessage, user_channel: UnboundedSender<Message>, users: &Arc<UsersChannels>) {
    if message.target == *username {
        return;
    }
    
    let users_channels = users.read().await;
    let Some(target_user) = users_channels.get(&message.target) else {
        let response = WsStatus::new("error", "User not found"); // TODO Temporary
        let _ = user_channel.send(response.as_message());
        return;
    };

    let response = WsMessage {
        action: String::from("receive"),
        target: String::from(username),
        content: message.content.clone()
    };

    let _ = target_user.send(response.as_message());
    let _ = user_channel.send(response.as_message());
}
