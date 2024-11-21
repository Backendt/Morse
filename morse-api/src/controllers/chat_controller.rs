use warp::ws::{WebSocket, Message};
use futures::{StreamExt, SinkExt, stream::SplitStream};
use tokio::sync::mpsc::UnboundedSender;
use std::sync::Arc;

use crate::{
    services::{
        ws_service,
        chat_service
    },
    models::ws::{
        UsersChannels,
        Response,
        Request,
        Action,
        Messageable
    },
    database::RedisCon
};

pub async fn on_client_connect(username: String, socket: WebSocket, users: Arc<UsersChannels>, redis: RedisCon) {
    let (mut sender, receiver) = socket.split();
    
    match ws_service::add_client(&username, &users).await {
        Ok((user_sender, user_receiver)) => {
            ws_service::start_forwarding(user_receiver, sender).await;
            receive_messages(&username, receiver, user_sender, &users, &redis).await;
            ws_service::remove_client(&username, &users).await;
            let _ = chat_service::leave_all(&username, &users, redis.clone()).await;
        },
        Err(error_message) => {
            let response = Response::err(&error_message);
            let _ = sender.send(response.as_message());
        }
    };
}

async fn receive_messages(username: &String, mut receiver: SplitStream<WebSocket>, user_channel: UnboundedSender<Message>, users: &Arc<UsersChannels>, redis: &RedisCon) {
    while let Some(received) = receiver.next().await {
        let Ok(raw_message) = received else { break; };

        match ws_service::parse_message(raw_message) {
            Ok(message) => on_request(&username, message, user_channel.clone(), &users, redis.clone()).await,
            Err(error_message) => {
                let response = Response::err(&error_message);
                let _ = user_channel.send(response.as_message());
            }
        };
    }
}

async fn on_request(username: &String, request: Request, user_channel: UnboundedSender<Message>, users: &Arc<UsersChannels>, redis: RedisCon) {
    let result: Result<String, String> = match request.action {
        Action::CreateRoom => chat_service::create_room(username, redis).await,
        Action::Invite => chat_service::invite_in_room(username, &request, users, redis).await,
        Action::Join => chat_service::join_room(username, &request, users, redis).await,
        Action::Message => chat_service::send_message(username, &request, users, redis).await,
        Action::Leave => chat_service::leave_room(username, &request, users, redis).await,
    };

    let response = result.map_or_else(
        |error_message| Response::err(&error_message),
        |response_message| Response::success(&response_message)
    );

    let _ = user_channel.send(response.as_message());
}
