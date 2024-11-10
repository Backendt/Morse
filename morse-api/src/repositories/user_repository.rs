use warp::reject::Rejection;
use crate::models::{
    auth::User,
    errors::{InternalError, InvalidRequest}
};

pub async fn get_user(username: &str) -> Result<User, Rejection> {
    // TODO Test implementation
    use argon2::{
        Argon2,
        password_hash::{
            SaltString, PasswordHasher,
            rand_core::OsRng
        }
    };

    let password = b"1234";
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default().hash_password(password, &salt).unwrap().to_string();

    let test_user = User {
        username: username.to_string(),
        password: hash
    };

    // Testing
    if username == "err" {
        Err(InternalError::new("I don't like this username"))
    } else if username != "bob" {
        Err(InvalidRequest::new("Who's this?"))
    } else {
        Ok(test_user)
    }
}

pub async fn create_user(user: &User) -> Result<&User, Rejection> {
    println!("Creating user {} with password {}", &user.username, &user.password);
    Ok(user)
}

pub async fn exists(username: &str) -> Result<bool, Rejection> {
    Ok(username == "bob")
}
