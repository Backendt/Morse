use warp::ws::{WebSocket, Message};
use futures::StreamExt;
use tokio::sync::mpsc::{
    unbounded_channel,
    UnboundedReceiver,
    UnboundedSender
};
use std::sync::Arc;

use crate::{
    services::ws_service,
    models::ws::UsersChannels
};

pub async fn on_client_connect(username: String, socket: WebSocket, users: Arc<UsersChannels>) {
    let (sender, mut receiver) = socket.split();

    let (user_sender, user_receiver) = add_client(&username, &users).await; 
    ws_service::start_forwarding(user_receiver, sender).await;

    while let Some(content) = receiver.next().await {
        match content {
            Ok(msg) => on_message(&username, msg, user_sender.clone(), &users).await,
            Err(err) => { // Protocol(ResetWithoutClosingHandshake) when client closes
                eprintln!("Could not read message from {username}: {err:?}");
                break; 
            }
        };
    }

    remove_client(&username, &users).await;
}

async fn add_client(username: &String, users: &Arc<UsersChannels>) -> (UnboundedSender<Message>, UnboundedReceiver<Message>) {
    // TODO Check if already connected
    println!("[+] '{username}' connected to websocket.");
    let (user_sender, user_receiver) = unbounded_channel();
    users.write().await.insert(username.clone(), user_sender.clone());
    (user_sender, user_receiver)
}

async fn remove_client(username: &String, users: &Arc<UsersChannels>) {
    println!("[-] '{username}' disconnected from websocket.");
    users.write().await.remove(username);
}

// TODO Impl made for testing
async fn on_message(username: &String, message: Message, user_sink: UnboundedSender<Message>, users: &Arc<UsersChannels>) {
    let (target, text) = message.to_str().unwrap().split_once("@").expect("Specify target");

    let response = Message::text(
        format!("[{username} -> {target}]): {text}")
    );
    let _ = user_sink.send(response.clone());

    let sinks = users.read().await;
    let target_sink = sinks.get(target).expect("User not found");
    let _ = target_sink.send(response);
}
