use warp::ws::{WebSocket, Message};
use futures::{StreamExt, SinkExt, stream::SplitStream};
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver};
use std::sync::Arc;
use crate::{
    services::{ws_service, chat_service, jwt_service},
    database::RedisCon,
    models::{
        Response,
        errors::{RequestResult, RequestError},
        ws::{WsEnvironment, Request, Action,Messageable, UsersChannels},
    }
};

pub async fn on_client_connect(mut socket: WebSocket, users_channels: Arc<UsersChannels>, redis: RedisCon) {
    let Some(username) = authenticate_client(&mut socket).await else { return; };
    let environment = WsEnvironment {
        username,
        users_channels: users_channels.clone(),
        redis: redis.clone()
    };
    
    handle_user_connection(socket, environment).await;
}

async fn authenticate_client(socket: &mut WebSocket) -> Option<String> {
    while let Some(received) = socket.next().await {
        let Ok(message) = received else { break; };
        let Ok(token) = message.to_str() else { break; };
        if let Some(username) = jwt_service::get_jwt_username(token.trim()) {
            return Some(username);
        }

        let response = Response::err("The given JWT is invalid or expired.");
        let _ = socket.send(response.as_message()).await
            .inspect_err(|err| eprintln!("Could not send authentication error message to user. {err:?}"));
    }

    None
}

async fn handle_user_connection(mut socket: WebSocket, environment: WsEnvironment) {
    match ws_service::add_client(&environment.username, &environment.users_channels).await {
        Ok((user_sender, user_receiver)) => establish_chat(user_sender, user_receiver, socket, environment).await,
        Err(error_message) => {
            let response = Response::err(&error_message);
            let _ = socket.send(response.as_message()).await
                .inspect_err(|err| eprintln!("Could not send error message to user. {err:?}"));
            let _ = socket.close().await
                .inspect_err(|err| eprintln!("Could not gracefully close duplicate user connection. {err:?}"));
        }
    };
}

async fn establish_chat(user_sender: UnboundedSender<Message>, user_receiver: UnboundedReceiver<Message>, socket: WebSocket, environment: WsEnvironment) {
    let (sender, receiver) = socket.split();
    ws_service::start_forwarding(user_receiver, sender).await;
    
    // Start listening
    receive_messages(receiver, user_sender, &environment).await;

    // On disconnect
    ws_service::remove_client(&environment.username, &environment.users_channels).await;
    let _ = chat_service::leave_all(&environment).await
        .inspect_err(|err| eprintln!("Could not leave all rooms when disconnecting. {err:?}"));
}

async fn receive_messages(mut receiver: SplitStream<WebSocket>, user_channel: UnboundedSender<Message>, environment: &WsEnvironment) {
    while let Some(received) = receiver.next().await {
        let Ok(raw_message) = received else { break; };

        match ws_service::parse_message(raw_message) {
            Ok(message) => on_request(message, user_channel.clone(), environment).await,
            Err(_) => {
                let response = Response::err("Could not parse message.");
                let _ = user_channel.send(response.as_message())
                    .inspect_err(|err| eprintln!("Could not send parse error message to user. {err:?}"));
            }
        };
    }
}

async fn on_request(request: Request, user_channel: UnboundedSender<Message>, environment: &WsEnvironment) {
    let result = handle_request(&request, environment).await;
    let response = result.map_or_else(
        |error_message| handle_error(&request.action, error_message),
        |response_message| Response::action_success(&request.action, &response_message)
    );

    let _ = user_channel.send(response.as_message())
        .inspect_err(|err| eprintln!("Could not send response to request. {err:?}"));
}

async fn handle_request(request: &Request, environment: &WsEnvironment) -> RequestResult<String> {
    let target = request.target();
    let body = request.body();
    match request.action {
        Action::CreateRoom => chat_service::create_room(&environment.username, environment.redis()).await,
        Action::Invite => chat_service::invite_in_room(&target?, &body?, environment).await,
        Action::Join => chat_service::join_room(&target?, environment).await,
        Action::Message => chat_service::send_message(&target?, &body?, environment).await,
        Action::Leave => chat_service::leave_room(&target?, environment).await,
    }
}

fn handle_error(action: &Action, error: RequestError) -> Response {
    let message = match error {
        RequestError::InternalError(err) => {
            eprintln!("An error occured when processing a chat request. {err:?}");
            "An error occured. Try again later.".to_owned()
        },
        RequestError::InvalidRequest(err) => err
    };

    Response::action_err(action, &message)
}
