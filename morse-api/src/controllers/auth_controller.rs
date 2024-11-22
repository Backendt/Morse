use warp::{
    Reply,
    http::StatusCode,
    reply::{
        json,
        with_status
    }
};
use sqlx::MySqlPool;

use crate::{
    controllers::WebResult,
    models::{
        auth::{User, JwtResponse},
        APIMessage,
        errors::RequestError::*
    },
    services::{user_service, jwt_service}
};

const BEARER: &str = "Bearer ";

pub async fn login(user_request: User, database: MySqlPool) -> WebResult<impl Reply> {
    let is_valid = user_service::validate_login(&user_request, &database).await?;
    if !is_valid {
        return InvalidRequest("Invalid credentials".to_owned()).into();
    }

    match jwt_service::create_jwt(&user_request) {
        Ok(token) => {
            let response = JwtResponse { token };
            Ok(with_status(json(&response), StatusCode::OK))
        },
        Err(err) => InternalError(
            format!("Could not create jwt for user. {err:?}")
        ).into()
    }
}

pub async fn register(user_request: User, database: MySqlPool) -> WebResult<impl Reply> {
    user_service::register_user(&user_request, &database).await?;
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
    UnauthorizedUser.into()
}
