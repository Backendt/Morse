use uuid::Uuid;
use redis::{AsyncCommands, RedisResult};
use futures::future::TryFutureExt;
use crate::database::RedisCon;

const ROOM_KEY_PREFIX: &str = "room:";
const USER_KEY_PREFIX: &str = "user:";

/*
*   Room -> Users
*/

pub async fn create_room(username: &str, redis: RedisCon) -> RedisResult<String> {
    let room_id = Uuid::new_v4().simple().to_string();
    let room_key = format!("{ROOM_KEY_PREFIX}{room_id}");
    redis.clone().lpush(room_key.clone(), username)
        .and_then(|_: ()| add_room_to_user(username, &room_key, redis)).await
        .inspect_err(|err| eprintln!("Could not create room: {err:?}"))
        .map(|_| room_id)
}

pub async fn get_users_in_room(room_id: &str, mut redis: RedisCon) -> RedisResult<Vec<String>> {
    let room_key = format!("{ROOM_KEY_PREFIX}{room_id}");
    redis.lrange(room_key, 0, -1).await
        .inspect_err(|err| eprintln!("Could not get users in given room: {err:?}"))
}

pub async fn remove_user(username: &str, room_id: &str, redis: RedisCon) -> RedisResult<bool> {
    let room_key = format!("{ROOM_KEY_PREFIX}{room_id}");
    redis.clone().lrem(room_key.clone(), 1, username)
        .and_then(|was_removed: bool| async move {
            if was_removed {
                remove_room_from_user(&username, &room_key, redis).await
            } else {
                Ok(was_removed)
            }
        }).await
        .inspect_err(|err| eprintln!("Could not remove user from room: {err:?}"))
}

pub async fn add_user(username: &str, room_id: &str, redis: RedisCon) -> RedisResult<bool> {
    let room_key = format!("{ROOM_KEY_PREFIX}{room_id}"); 
    redis.clone().lpush_exists(room_key.clone(), username)
        .and_then(|was_added: bool| async move {
            if was_added {
                add_room_to_user(&username, &room_key, redis).await
            } else {
                Ok(was_added)
            }
        }).await
        .inspect_err(|err| eprintln!("Could not add user to room: {err:?}"))
}

/*
*   User -> Rooms
*/

pub async fn get_user_rooms(username: &str, mut redis: RedisCon) -> RedisResult<Vec<String>> {
    let user_key = format!("{USER_KEY_PREFIX}{username}");
    redis.lrange(user_key, 0, -1)
        .map_ok(|mut room_keys: Vec<String>| {
            room_keys.iter_mut()
                .map(|key|
                    key.strip_prefix(ROOM_KEY_PREFIX)
                        .expect("The room key does not start with expected prefix")
                        .to_owned()
                ).collect()
        }).await
        .inspect_err(|err| eprintln!("Could not get rooms for the given user: {err:?}"))
}

async fn add_room_to_user(username: &str, room_key: &str, mut redis: RedisCon) -> RedisResult<bool> {
    let user_key = format!("{USER_KEY_PREFIX}{username}");
    redis.lpush(user_key, room_key).await
        .inspect_err(|err| eprintln!("Could not add room to user: {err:?}"))
}

async fn remove_room_from_user(username: &str, room_key: &str, mut redis: RedisCon) -> RedisResult<bool> {
    let user_key = format!("{USER_KEY_PREFIX}{username}");
    redis.lrem(user_key, 1, room_key).await
        .inspect_err(|err| eprintln!("Could not remove room from user: {err:?}"))
}
