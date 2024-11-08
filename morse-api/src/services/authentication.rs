use warp::{
    Reply,
    http::StatusCode,
    reply::{
        json,
        with_status
    }
};
use argon2::{
    Argon2,
    password_hash::{
        self,
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    }
};

use crate::models::*;
use crate::repositories::user_repository;

// Shortcut to boilerplate  TODO Move somewhere else?
type WebResult<T> = Result<T, warp::Rejection>;

pub async fn login(user_request: User) -> WebResult<impl Reply> {
    let refuse_response = APIMessage::new("Invalid credentials", StatusCode::UNAUTHORIZED);

    let Ok(user) = user_repository::get_user(&user_request.username).await else {
        // TODO Vulnerable to timing attack
        return Ok(refuse_response.as_reply());
    };
    let is_password_valid = compare_to_hash(&user_request.password, &user.password).unwrap_or_else(
        |err| {
            eprintln!("User has an invalid hash stored as password. {:?}", err);
            false
        }
    );

    // Create JWT if successful
    if is_password_valid {
        let token = create_token(&user);
        let body = json(&token);
        return Ok(with_status(body, StatusCode::OK));
    }

    Ok(refuse_response.as_reply())
}

pub async fn register(user_request: User) -> WebResult<impl Reply> {
    let user_exists = user_repository::exists(&user_request.username).await?;

    let Ok(hashed_password) = hash(&user_request.password) else {
        let response = APIMessage::new("Could not hash the given password", StatusCode::BAD_REQUEST);
        return Ok(response.as_reply());
    };

    let hashed_user = User {
        username: user_request.username,
        password: hashed_password
    };

    if !user_exists {
        let _ = user_repository::create_user(&hashed_user).await; // TODO don't block current thread
    }

    let response = APIMessage::new("User was created if it didn't already exist", StatusCode::CREATED);
    Ok(response.as_reply())
}


fn create_token(user: &User) -> TokenRequest {
    let token = format!("this is jwt for {}", user.username);
    TokenRequest { token }
}

// Hashing

fn hash(password: &str) -> Result<String, password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
}

fn compare_to_hash(password: &str, hash: &str) -> Result<bool, password_hash::Error> { 
    let parsed_hash = PasswordHash::new(&hash)?;
    let is_equal = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();
    Ok(is_equal)
}
