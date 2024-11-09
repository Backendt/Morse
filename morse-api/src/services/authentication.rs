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
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

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
    if !is_password_valid {
        return Ok(refuse_response.as_reply());
    }

    create_jwt(&user).map_or_else(
        |err| { // TODO Return Err instead
            eprintln!("Could not create jwt for user. {:?}", err);
            let response = APIMessage::new("An unexpected error occured. Try again later.", StatusCode::INTERNAL_SERVER_ERROR);
            Ok(response.as_reply())
        },
        |token| {
            let response = JwtResponse { token };
            Ok(with_status(json(&response), StatusCode::OK))
        }
    )
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

// JWT

fn create_jwt(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(10))
        .expect("Could not add 10 hours to current time")
        .timestamp();

    let claims = JwtClaims {
        exp: expiration as usize,
        sub: user.username.to_owned()
    };

    let header = Header::new(Algorithm::HS512);
    let secret = std::env::var("JWT_SECRET").expect("The JWT_SECRET environment variable is not set.");
    
    encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

pub fn get_jwt_username(token: &str) -> Option<String> {
    let secret = std::env::var("JWT_SECRET").expect("The JWT_SECRET environment variable is not set.");
    let decoded_jwt = decode::<JwtClaims>(&token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::new(Algorithm::HS512));

    match decoded_jwt {
        Ok(jwt) => Some(jwt.claims.sub),
        _ => None
    }
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
