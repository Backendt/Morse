use warp::{
    Filter,
    header,
    http::{
        header::AUTHORIZATION,
        StatusCode
    },
    Reply,
    reject::Rejection
};
use std::error::Error;
use super::{
    controllers::*,
    models::{APIMessage, errors::*}
};

pub fn get_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    login()
        .or(register())
        .or(me())
        .recover(handle_rejection)
}

// Endpoints

fn me() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("me")
        .and(authenticated())
}

fn login() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("login")
        .and(warp::post())
        .and(warp::body::json()) // TODO Set a size limit with a custom json filter
        .and_then(auth_controller::login)
}

fn register() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("register")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(auth_controller::register)
}

// Filters

fn authenticated() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    header::<String>(AUTHORIZATION.as_str())
        .and_then(auth_controller::get_current_username)
}

async fn handle_rejection(err: warp::reject::Rejection) -> Result<impl Reply, Rejection> {
    let message: String;
    let status: StatusCode;

    // Invalid Request
    if let Some(invalid_request) = err.find::<InvalidRequest>() {
        message = invalid_request.message.clone();
        status = StatusCode::BAD_REQUEST;

    // Deserialization Error
    } else if let Some(bad_request) = err.find::<warp::filters::body::BodyDeserializeError>() {
        message = bad_request.source().map_or_else(|| bad_request.to_string(), |source| source.to_string());
        status = StatusCode::BAD_REQUEST;

    // Missing Authorization header
    } else if let Some(missing_header) = err.find::<warp::reject::MissingHeader>() {
        if missing_header.to_string().contains(AUTHORIZATION.as_str()) {
            message = "You need authentication".to_string();
            status = StatusCode::UNAUTHORIZED;
        } else {
            return Err(err);
        }

    // User denied
    } else if let Some(_) = err.find::<UnauthorizedUser>() {
        message = "You are not allowed".to_string();
        status = StatusCode::FORBIDDEN;

    // Internal Errors
    } else if let Some(internal_error) = err.find::<InternalError>() {
        eprintln!("An error occured: {}", internal_error.message);
        message = "An unexpected error occured. Please try again later.".to_string();
        status = StatusCode::INTERNAL_SERVER_ERROR;

    } else {
        return Err(err);
    }

    let response = APIMessage::new(&message, status);
    return Ok(response.as_reply())
}
