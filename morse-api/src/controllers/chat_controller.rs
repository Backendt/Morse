use warp::ws::{WebSocket, Message};
use futures::{StreamExt, SinkExt, stream::SplitStream};
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver};

use crate::{
    services::{ws_service, chat_service},
    models::Response,
    models::errors::{RequestResult, RequestError},
    models::ws::{
        WsEnvironment,
        Request,
        Action,
        Messageable
    }
};

pub async fn on_client_connect(mut socket: WebSocket, environment: WsEnvironment) {
    match ws_service::add_client(&environment.username, &environment.users_channels).await {
        Ok((user_sender, user_receiver)) => establish_connection(user_sender, user_receiver, socket, environment).await,
        Err(error_message) => {
            let response = Response::err(&error_message);
            let _ = socket.send(response.as_message()).await
                .inspect_err(|err| eprintln!("Could not send error message to user. {err:?}"));
            let _ = socket.close().await
                .inspect_err(|err| eprintln!("Could not gracefully close duplicate user connection. {err:?}"));
        }
    };
}

async fn establish_connection(user_sender: UnboundedSender<Message>, user_receiver: UnboundedReceiver<Message>, socket: WebSocket, environment: WsEnvironment) {
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
            Err(error_message) => {
                let response = Response::err(&error_message);
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
        RequestError::UnauthorizedUser => "You do not have the permission to do that".to_owned(),
        RequestError::InternalError(err) => {
            eprintln!("An error occured when processing a chat request. {err:?}");
            "An error occured. Try again later.".to_owned()
        },
        RequestError::InvalidRequest(err) => err
    };

    Response::action_err(action, &message)
}
