use warp::ws::{WebSocket, Message};
use futures::{StreamExt, SinkExt, stream::SplitStream};
use tokio::sync::mpsc::UnboundedSender;
use std::sync::Arc;

use crate::{
    services::{
        ws_service,
        chat_service
    },
    models::errors::{RequestResult, RequestError},
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
            let _ = chat_service::leave_all(&username, &users, redis.clone()).await
                .inspect_err(|err| eprintln!("Could not leave all rooms when disconnecting. {err:?}"));
        },
        Err(error_message) => {
            let response = Response::err(&error_message);
            let _ = sender.send(response.as_message()).await
                .inspect_err(|err| eprintln!("Could not send error message to user. {err:?}"));
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
                let _ = user_channel.send(response.as_message())
                    .inspect_err(|err| eprintln!("Could not send parse error message to user. {err:?}"));
            }
        };
    }
}

async fn on_request(username: &str, request: Request, user_channel: UnboundedSender<Message>, users: &Arc<UsersChannels>, redis: RedisCon) {
    let result = handle_request(username, &request, users, redis).await;
    let response = result.map_or_else(
        |error_message| handle_error(&request.action, error_message),
        |response_message| Response::success(&request.action, &response_message)
    );

    let _ = user_channel.send(response.as_message())
        .inspect_err(|err| eprintln!("Could not send response to request. {err:?}"));
}

async fn handle_request(username: &str, request: &Request, users: &Arc<UsersChannels>, redis: RedisCon) -> RequestResult<String> {
    let target = request.target();
    let body = request.body();
    match request.action {
        Action::CreateRoom => chat_service::create_room(username, redis).await,
        Action::Invite => chat_service::invite_in_room(username, &target?, &body?, users, redis).await,
        Action::Join => chat_service::join_room(username, &target?, users, redis).await,
        Action::Message => chat_service::send_message(username, &target?, &body?, users, redis).await,
        Action::Leave => chat_service::leave_room(username, &target?, users, redis).await,
    }
}

fn handle_error(action: &Action, error: RequestError) -> Response {
    let message = match error {
        RequestError::UnauthorizedUser => "You do not have the permission to do that".to_owned(),
        RequestError::InternalError(err) => {
            eprintln!("An error occured when processing a chat request. {err:?}");
            "An error occured. Try again later.".to_owned()
        },
        RequestError::InvalidRequest(err) => err
    };

    Response::action_err(action, &message)
}
