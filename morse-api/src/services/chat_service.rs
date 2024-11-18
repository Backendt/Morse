use std::sync::Arc;

use crate::models::ws::{
    Action,
    Request,
    UsersChannels,
    Messageable
};

pub async fn invite_to_chat() {

}

pub async fn refuse_invitation() {

}

pub async fn accept_invitation() {

}

pub async fn send_message(username: &String, request: &Request, users: &Arc<UsersChannels>) -> Result<String, String> {
    let Some(body) = &request.body else {
        return Err(String::from("The body is required."));
    };

    if request.target == *username {
        return Err(String::from("Can't send messages to yourself."));
    }
    // TODO Check for invitation

    let users_channels = users.read().await;
    let Some(target_user) = users_channels.get(&request.target) else {
        return Err(String::from("User not found")); // TODO Temporary
    };

    let message = Request::body(
        Action::Message,
        username,
        body.to_string()
    );

    let _ = target_user.send(message.as_message());
    Ok(String::from("Message has been sent"))
}
