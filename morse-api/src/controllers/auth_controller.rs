use warp::{
    Reply,
    http::StatusCode as HttpStatus,
    reply::{
        json,
        with_status
    }
};
use sqlx::MySqlPool;

use crate::{
    controllers::WebResult,
    models::{
        api::ApiMessage,
        api::status::*,
        api::auth::*,
        auth::User,
        errors::RequestError::*
    },
    services::{user_service, jwt_service}
};

const ANONYMOUS_USER_PREFIX: &str = "anon-";

pub async fn login(user_request: User, database: MySqlPool) -> WebResult<impl Reply> {
    let is_valid = user_service::validate_login(&user_request, &database).await?;
    if !is_valid {
        return InvalidRequest("Invalid credentials".to_owned()).into();
    }

    let username = &user_request.username;
    match jwt_service::create_jwt(username) {
        Ok(token) => {
            let body = TokenBody { username: username.to_owned(), token };
            let response = ApiMessage::new(body);
            Ok(with_status(json(&response), HttpStatus::OK))
        },
        Err(err) => InternalError(
            format!("Could not create jwt for user. {err:?}")
        ).into()
    }
}

pub async fn register(raw_user_request: User, database: MySqlPool) -> WebResult<impl Reply> {
    let user_request = raw_user_request.validated()?;
    user_service::register_user(&user_request, &database).await?;
    let body = StatusBody {
        success: true,
        status_code: StatusCode::Registration,
        message: "User was created if it didn't already exist".to_owned()
    };
    let response = ApiMessage::new(body);
    Ok(with_status(json(&response), HttpStatus::CREATED))
}

pub async fn anonymous_login() -> WebResult<impl Reply> {
    let random_id = uuid::Uuid::new_v4().simple().to_string();
    let username = format!("{ANONYMOUS_USER_PREFIX}{random_id}");
    
    match jwt_service::create_jwt(&username) {
        Ok(token) => {
            let body = TokenBody { username, token };
            let response = ApiMessage::new(body);
            Ok(with_status(json(&response), HttpStatus::OK))
        },
        Err(err) => InternalError(
            format!("Could not create anonymous jwt. {err:?}")
        ).into()
    }
}
