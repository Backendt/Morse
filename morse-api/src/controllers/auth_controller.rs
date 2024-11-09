use warp::{
    Reply,
    http::StatusCode,
    reply::{
        json,
        with_status
    }
};

use crate::models::{
    auth::{User, JwtResponse},
    APIMessage
};
use crate::services::{user_service, jwt_service};

const BEARER: &str = "Bearer ";

// Shortcut to boilerplate  TODO Move somewhere else?
type WebResult<T> = Result<T, warp::Rejection>;

pub async fn login(user_request: User) -> WebResult<impl Reply> {
    let is_valid = match user_service::validate_login(&user_request).await {
        Ok(valid) => valid,
        Err(err) => {
            eprintln!("Could not validate login {:?}", err);
            false
        }
    };

    if !is_valid {
        let deny_response = APIMessage::new("Invalid credentials", StatusCode::UNAUTHORIZED);
        return Ok(deny_response.as_reply());
    }

    match jwt_service::create_jwt(&user_request) {
        Ok(token) => {
            let response = JwtResponse { token };
            Ok(with_status(json(&response), StatusCode::OK))
        },
        Err(err) => { // TODO Return Err instead
            eprintln!("Could not create jwt for user. {:?}", err);
            let response = APIMessage::new("An unexpected error occured. Try again later.", StatusCode::INTERNAL_SERVER_ERROR);
            Ok(response.as_reply())
        }
    }
}

pub async fn register(user_request: User) -> WebResult<impl Reply> {
    let register_result = user_service::register_user(&user_request).await;

    match register_result {
        Err(_err) => Ok(APIMessage::new("An unexpected error occured. Try again later.", StatusCode::INTERNAL_SERVER_ERROR).as_reply()), //Err(warp::reject::custom(err)),
        Ok(()) => Ok(APIMessage::new("User was created if it didn't already exist", StatusCode::CREATED).as_reply())
    }
}

pub async fn get_current_username(auth_header: String) -> WebResult<String> {
    if auth_header.starts_with(BEARER) {
        let jwt = auth_header.trim_start_matches(BEARER);
        if let Some(username) = jwt_service::get_jwt_username(jwt) {
            return Ok(username);
        }
    }

    Err(warp::reject()) // Reject with Unauthorized status instead
}
