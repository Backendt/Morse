use std::sync::Arc;
use futures::future::{TryFutureExt, join_all};

use crate::{
    repositories::room_repository,
    models::errors::{
        RequestResult,
        RequestError::{InternalError, InvalidRequest}
    },
    models::ws::{
        Action,
        Request,
        UsersChannels,
        ChatMessage,
        Messageable,
        WsEnvironment
    },
    database::RedisCon
};

pub async fn create_room(username: &str, redis: RedisCon) -> RequestResult<String> {
    room_repository::create_room(username, redis).await
        .map(|room_id| room_id.to_string())
        .map_err(|err| InternalError(format!("Could not create room. {err:?}")))
}

pub async fn invite_in_room(target_username: &str, room_id: &str, environment: &WsEnvironment) -> RequestResult<String> {
    let username = &environment.username;
    if *username == *target_username {
        return Err(InvalidRequest("Cannot invite yourself".to_owned()));
    }

    if !is_user_in_room(username, room_id, environment.redis()).await? {
        return Err(InvalidRequest("You are not in the given room".to_owned()));
    }

    let user_channels = environment.users_channels.read().await;
    if let Some(target) = user_channels.get(target_username) {
        let request = Request::new(
            Action::Invite,
            target_username.to_owned(),
            room_id.to_owned()
        );
        let _ = target.send(request.as_message())
            .map_err(|err| InternalError(format!("Could not send invitation: {err:?}")))?;
    };

    return Ok("Invitation sent.".to_owned());
}

pub async fn join_room(room_id: &str, environment: &WsEnvironment) -> RequestResult<String> {
    let username = &environment.username;

    let room_users = room_repository::get_users_in_room(room_id, environment.redis()).await
        .map_err(|err| InternalError(format!("Could not get users in room. {err:?}")))?;
    if room_users.contains(&username.to_owned()) {
        return Err(InvalidRequest("You're already in the room".to_owned()));
    }

    let was_added = room_repository::add_user(username, room_id, environment.redis()).await
        .map_err(|err| InternalError(format!("Could not add user to room. {err:?}")))?;
    if !was_added {
        return Err(InvalidRequest("The room does not exist.".to_owned()));
    }

    let request = Request::new(
        Action::Join,
        room_id.to_owned(),
        username.to_owned()
    );
    let _ = send_to_users(request, room_users, &environment.users_channels).await
        .inspect_err(|err| eprintln!("Could not send join message to users in room. {err:?}"));
    Ok("You joined the room".to_owned())
}

pub async fn leave_room(room_id: &str, environment: &WsEnvironment) -> RequestResult<String> {
    let was_removed = room_repository::remove_user(&environment.username, room_id, environment.redis()).await
        .map_err(|err| InternalError(format!("Could not leave room. {err:?}")))?;
    if !was_removed {
        return Err(InvalidRequest("You are not in the given room.".to_owned()));
    }

    let request = Request::new(
        Action::Leave,
        room_id.to_owned(),
        environment.username.to_owned()
    );

    let _ = send_to_room(request, room_id, &environment.users_channels, environment.redis()).await
        .inspect_err(|errs| eprintln!("Could not send leave message to everyone in room. {errs:?}"));
    Ok("You left the room.".to_owned())
}

pub async fn leave_all(environment: &WsEnvironment) -> RequestResult<()> {
    let room_ids = room_repository::get_user_rooms(&environment.username, environment.redis()).await
        .map_err(|err| InternalError(format!("Could not get user rooms to leave them. {err:?}")))?;

    let mut futures = Vec::with_capacity(room_ids.len());
    for room_id in room_ids.iter() {
        let future = leave_room(room_id, environment);
        futures.push(future);
    }

    let results = join_all(futures).await;
    for result in results.iter() {
        if let Err(err) = result {
            eprintln!("An error occured when leaving all rooms: {err:?}");
        }
    }
    Ok(())
}

pub async fn send_message(room_id: &str, message: &str, environment: &WsEnvironment) -> RequestResult<String> {
    let room_users = get_usernames_in_same_room(&environment.username, room_id, environment.redis()).await?;

    let request = ChatMessage {
        sender: environment.username.to_owned(),
        room: room_id.to_owned(),
        content: message.to_owned()
    };

    send_to_users(request, room_users, &environment.users_channels).await
        .map(|_| "Message has been sent".to_owned())
        .or_else(|err| {
            eprintln!("Could not send chat message to users in room. {err:?}");
            Ok("Message has been sent, but one or multiple users might have not received it".to_owned())
        })
}

async fn get_usernames_in_same_room(username: &str, room_id: &str, redis: RedisCon) -> RequestResult<Vec<String>> {
    let room_users = room_repository::get_users_in_room(room_id, redis).await
            .map_err(|err| InternalError(format!("Could not get users in room. {err:?}")))?;
    if !room_users.contains(&username.to_owned()) {
        return Err(InvalidRequest("You are not in the given room.".to_owned()));
    }
    Ok(room_users)
}

async fn send_to_room(request: Request, room_id: &str, channels: &Arc<UsersChannels>, redis: RedisCon) -> RequestResult<()> {
    let room_users = room_repository::get_users_in_room(room_id, redis).await
        .map_err(|err| InternalError(format!("Could not get users in room to send them a request. {err:?}")))?;
    
    send_to_users(request, room_users, channels).await
}

async fn send_to_users(request: impl Messageable, room_users: Vec<String>, channels: &Arc<UsersChannels>) -> RequestResult<()> {
    let mut errors = Vec::with_capacity(room_users.len());
    let users_channels = channels.read().await;
    for room_user in room_users.iter() {
        if let Some(user_channel) = users_channels.get(room_user) {
            if let Err(err) = user_channel.send(request.as_message()) {
                errors.push(err);
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(InternalError(format!("Could not send request to all users. {errors:?}")))
    }
}

async fn is_user_in_room(username: &str, room_id: &str, redis: RedisCon) -> RequestResult<bool> {
    room_repository::get_user_rooms(username, redis)
        .map_ok(|rooms| rooms.contains(&room_id.to_owned()))
        .map_err(|err| InternalError(format!("Could not get user rooms. {err:?}")))
        .await
}
