use uuid::{
    Uuid,
    fmt::Simple
};

pub async fn create_room(username: &str) -> Result<Simple, String> {
    let room_id = Uuid::new_v4().simple();
    let room_key = format!("room:{}", room_id);
    println!("creating room for {username}: {room_key}");
    // TODO Save in db
    Ok(room_id)
}

pub async fn get_users_in_room(room_id: &str) -> Result<Vec<String>, String> {
    let room_key = format!("room:{}", room_id);
    println!("Getting which users are in: {room_key}");
    Ok(vec![String::from("bob")])
}

pub async fn get_user_rooms(username: &str) -> Result<Vec<String>, String> {
    println!("Getting all rooms for user: {username}");
    Ok(vec![String::from("1234"), String::from("4321")])
}

pub async fn remove_user(username: &str, room_id: &str) -> Result<bool, String> {
    let room_key = format!("room:{}", room_id);
    println!("Removing user {username} from room {room_key}");
    // TODO Also check if room is now empty
    Ok(*username == *"bob")
}

pub async fn add_user(username: &str, room_id: &str) -> Result<bool, String> {
    let room_key = format!("room:{}", room_id);
    println!("Adding user {username} to room {room_key}");
    Ok(true) // Use lpushx
}
