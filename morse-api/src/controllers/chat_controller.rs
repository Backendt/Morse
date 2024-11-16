use warp::ws::{WebSocket, Message};
use futures::{StreamExt, SinkExt};
use tokio::sync::mpsc::UnboundedSender;
use std::sync::Arc;

use crate::{
    services::ws_service,
    models::ws::{
        UsersChannels,
        WsMessage,
        WsStatus,
        Messageable
    }
};

pub async fn on_client_connect(username: String, socket: WebSocket, users: Arc<UsersChannels>) {
    let (mut sender, mut receiver) = socket.split();
    
    match ws_service::add_client(&username, &users).await {
        Err(error_message) => {
            let _ = sender.send(error_message.as_message());
        },
        Ok((user_sender, user_receiver)) => {
            ws_service::start_forwarding(user_receiver, sender).await;

            while let Some(raw_message) = receiver.next().await { // TODO Put in its own function
                let parsed_message = match raw_message {
                    Ok(message) => ws_service::parse_message(message),
                    Err(err) => { // TODO Handle Protocol(ResetWithoutClosingHandshake) when client closes
                        eprintln!("Could not receive message from {username}: {err:?}");
                        break;
                    }
                };

                match parsed_message {
                    Ok(message) => on_message(&username, message, user_sender.clone(), &users).await,
                    Err(error_message) => { 
                        let _ = user_sender.send(error_message.as_message());
                    }
                };
            }
            ws_service::remove_client(&username, &users).await;
        }
    };
}

async fn on_message(username: &String, message: WsMessage, user_channel: UnboundedSender<Message>, users: &Arc<UsersChannels>) {
    println!("Received from {username}: {message:?}");
    match message.action.as_str() {
        "send_message" => ws_service::send_message(username, message, user_channel, users).await,
        _ => {
            let error_message = WsStatus::new("error", "Invalid action").as_message();
            let _ = user_channel.send(error_message);
        }
    }
}
