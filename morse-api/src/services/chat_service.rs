use std::sync::Arc;
use futures::future::TryFutureExt;

use crate::{
    repositories::room_repository,
    models::ws::{
        Action,
        Request,
        UsersChannels,
        Messageable
    }
};
// TODO Improve error handling
pub async fn create_room(username: &str) -> Result<String, String> {
    room_repository::create_room(username).await
        .map(|room_id| room_id.to_string())
        .map_err(|_| String::from("Could not create room"))
}

pub async fn invite_in_room(username: &String, request: &Request, users: &Arc<UsersChannels>) -> Result<String, String> {
    if *username == request.target {
        return Err(String::from("Cannot invite yourself"));
    }

    let room_id = &request.get_body()?;
    let _ = get_usernames_in_same_room(username, &request.target).await?;

    let users = users.read().await;
    if let Some(target) = users.get(&request.target) {
        let request = Request::body(
            Action::Invite,
            &request.target,
            room_id.to_string()
        );
        let _ = target.send(request.as_message());
    };
    Ok(String::from("Invitation sent"))
}

pub async fn join_room(username: &String, request: &Request, users: &Arc<UsersChannels>) -> Result<String, String> {
    let room_id = &request.target;

    let room_users = room_repository::get_users_in_room(room_id).await?;
    if room_users.contains(username) {
        return Err(String::from("You're already in the room"));
    }

    let request = Request::body(
        Action::Join,
        room_id,
        username.to_string()
    );
    send_to_users(request, room_users, users).await;

    let was_added = room_repository::add_user(username, room_id).await?;
    if !was_added {
        return Err(String::from("Could not join the room."));
    }

    Ok(String::from("You joined the room"))
}

pub async fn leave_room(username: &String, request: &Request, users: &Arc<UsersChannels>) -> Result<String, String> {
    let room_id = &request.target;
    
    let was_removed = room_repository::remove_user(username, room_id).await
        .map_err(|_| String::from("An error occured. Try again later"))?;
    if !was_removed {
        return Err(String::from("You are not in the given room."));
    }

    let request = Request::body(
        Action::Leave,
        room_id,
        username.to_string()
    );

    send_to_room(request, room_id, users).await?;
    Ok(String::from("You left the room"))
}

pub async fn leave_all(username: &String, users: &Arc<UsersChannels>) -> Result<(), String> {
    let room_ids = room_repository::get_user_rooms(username).await?;

    for room_id in room_ids.iter() {
        let leave_message = Request::body(
            Action::Leave,
            room_id,
            username.to_string()
        );
        let _ = room_repository::remove_user(username, room_id)
            .and_then(|_| send_to_room(leave_message, room_id, users))
            .await; // TODO put future in list and join_all
    }
    Ok(())
}

pub async fn send_message(username: &String, request: &Request, users: &Arc<UsersChannels>) -> Result<String, String> {
    let message = &request.get_body()?;
    let room_users = get_usernames_in_same_room(username, &request.target).await?;

    let request = Request::body(
        Action::Message,
        username,
        message.to_string()
    );

    send_to_users(request, room_users, users).await;
    Ok(String::from("Message has been sent"))
}

async fn get_usernames_in_same_room(username: &String, room_id: &str) -> Result<Vec<String>, String> {
    let room_users = room_repository::get_users_in_room(room_id).await?;
    if !room_users.contains(username) {
        return Err(String::from("You are not in the given room."))
    }
    Ok(room_users)
}

async fn send_to_room(request: Request, room_id: &str, channels: &Arc<UsersChannels>) -> Result<(), String> {
    let room_users = room_repository::get_users_in_room(room_id).await?;
    send_to_users(request, room_users, channels).await;
    Ok(())
}

async fn send_to_users(request: Request, room_users: Vec<String>, channels: &Arc<UsersChannels>) {
    let users_channels = channels.read().await;
    for room_user in room_users.iter() {
        if let Some(user_channel) = users_channels.get(room_user) {
            let _ = user_channel.send(request.as_message());
        }
    }
}
