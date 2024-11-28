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
    UsersChannels,
    Request
};

pub async fn add_client(username: &String, users: &Arc<UsersChannels>) -> Result<(UnboundedSender<Message>, UnboundedReceiver<Message>), String> {
    let already_connected = users.read().await.get(username).is_some();
    if already_connected {
        return Err(
            String::from("You are already connected.")
        );
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

pub fn parse_message(raw_message: Message) -> Result<Request, serde_json::Error> {
    let content = raw_message.as_bytes(); 
    serde_json::from_slice::<Request>(content)
}
