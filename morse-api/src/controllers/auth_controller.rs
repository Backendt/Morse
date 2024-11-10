use warp::{
    Reply,
    http::StatusCode,
    reply::{
        json,
        with_status
    }
};

use crate::{
    models::{
        auth::{User, JwtResponse},
        APIMessage,
        errors::{InvalidRequest, UnauthorizedUser, InternalError}
    },
    services::{user_service, jwt_service}
};

const BEARER: &str = "Bearer ";

// Shortcut to boilerplate  TODO Move somewhere else?
type WebResult<T> = Result<T, warp::Rejection>;

pub async fn login(user_request: User) -> WebResult<impl Reply> {
    let is_valid = user_service::validate_login(&user_request).await?;
    if !is_valid {
        return Err(InvalidRequest::new("Invalid credentials"));
    }

    match jwt_service::create_jwt(&user_request) {
        Ok(token) => {
            let response = JwtResponse { token };
            Ok(with_status(json(&response), StatusCode::OK))
        },
        Err(err) => Err(
            InternalError::new(
                format!("Could not create jwt for user. {err:?}")
                .as_str()
            )
        )
    }
}

pub async fn register(user_request: User) -> WebResult<impl Reply> {
    user_service::register_user(&user_request).await?;
    let response = APIMessage::new("User was created if it didn't already exist", StatusCode::CREATED);
    Ok(response.as_reply())
}

pub async fn get_current_username(auth_header: String) -> WebResult<String> {
    if auth_header.starts_with(BEARER) {
        let jwt = auth_header.trim_start_matches(BEARER);
        if let Some(username) = jwt_service::get_jwt_username(jwt) {
            return Ok(username);
        }
    }
    Err(UnauthorizedUser::new())
}
