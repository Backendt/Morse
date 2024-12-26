use warp::ws::{WebSocket, Message};
use futures::{StreamExt, SinkExt, stream::SplitStream};
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver};
use std::sync::Arc;
use crate::{
    services::{ws_service, chat_service, jwt_service},
    database::RedisCon,
    models::ws::*,
    models::errors::{RequestResult, RequestError},
    models::api::{
        status::{StatusBody, StatusCode},
        MessageBody,
        Request,
        Action
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

async fn authenticate_client(mut socket: &mut WebSocket) -> Option<String> {
    while let Some(received) = socket.next().await {
        let Ok(message) = received else { break; };
        if let Ok(token) = message.to_str() {
            if let Some(username) = jwt_service::get_jwt_username(token.trim()) {
                return Some(username);
            }
        }

        send_socket_error(&mut socket, StatusCode::InvalidToken, "The authentication token is invalid or expired.").await;
    }
    None
}

async fn handle_user_connection(mut socket: WebSocket, environment: WsEnvironment) {
    match ws_service::add_client(&environment.username, &environment.users_channels).await {
        Ok((user_sender, user_receiver)) => establish_chat(user_sender, user_receiver, socket, environment).await,
        Err(error_message) => {
            send_socket_error(&mut socket, StatusCode::AlreadyConnected, &error_message).await;
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
    // TODO drop(sender) to stop forwarding task?
}

async fn receive_messages(mut receiver: SplitStream<WebSocket>, sender: UnboundedSender<Message>, environment: &WsEnvironment) {
    while let Some(received) = receiver.next().await {
        let Ok(raw_message) = received else { break; };

        match ws_service::parse_message(raw_message.clone()) {
            Ok(message) => on_request(message, &sender, environment).await,
            Err(err) => {
                eprintln!("Error when parsing message: {err:?}"); // TODO Temporary
                eprintln!("Tried parsing message: {raw_message:?}");
                eprintln!("Message as bytes: {:?}", raw_message.as_bytes());
                send_response(&sender, StatusBody {
                    success: false,
                    status_code: StatusCode::ParseError,
                    message: err.to_string()
                }.as_message());
            }
        };
    }
}

async fn on_request(request: Request, sender: &UnboundedSender<Message>, environment: &WsEnvironment) {
    let opt_message = handle_request(&request, environment).await
        .unwrap_or_else(|err| Some(get_error_status(err).as_message()));
    if let Some(message) = opt_message {
        send_response(sender, message);
    }
}

async fn handle_request(request: &Request, environment: &WsEnvironment) -> RequestResult<Option<Message>> {
    let target = request.target();
    let body = request.body();
    let message = match request.action {
        Action::CreateRoom => chat_service::create_room(&environment.username, environment.redis()).await?.as_message(),
        Action::Invite => chat_service::invite_in_room(&target?, &body?, environment).await?.as_message(),
        Action::Join => chat_service::join_room(&target?, environment).await?.as_message(),
        Action::Leave => chat_service::leave_room(&target?, environment).await?.as_message(),
        Action::Message => {
            chat_service::send_message(&target?, &body?, environment).await?;
            return Ok(None);
        }
    };
    Ok(Some(message))
}

fn get_error_status(error: RequestError) -> StatusBody {
    let status_code;
    let message = match error {
        RequestError::InternalError(err) => {
            eprintln!("An error occured when processing a request. {err:?}");
            status_code = StatusCode::InternalError;
            "An error occured. Try again later.".to_owned()
        },
        RequestError::InvalidRequest(error_message) => {
            status_code = StatusCode::InvalidRequest;
            error_message
        }
    };

    StatusBody {
        success: false,
        status_code,
        message
    }
}

fn send_response(sender: &UnboundedSender<Message>, message: Message) {
    let _ = sender.send(message)
        .inspect_err(|err| eprintln!("Could not send response to user. {err:?}"));
}

async fn send_socket_error(socket: &mut WebSocket, status_code: StatusCode, message: &str) {
    let body = StatusBody {
        success: false,
        status_code,
        message: message.to_owned()
    };
    let _ = socket.send(body.as_message()).await
        .inspect_err(|err| eprintln!("Could not send response to user. {err:?}"));
}
