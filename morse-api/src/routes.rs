use warp::{
    http::{header::AUTHORIZATION, StatusCode},
    reply::{json, with_status},
    Filter, Reply,
    reject::Rejection,
    ws::Ws
};
use std::{
    error::Error,
    convert::Infallible,
    sync::Arc
};
use sqlx::MySqlPool;

use super::{
    controllers::*,
    models::{
        Response,
        ws::UsersChannels,
        errors::RequestError::{self, *}
    },
    database::RedisCon
};

const JSON_BYTES_SIZE_LIMIT: u64 = 150;

pub fn get_routes(redis: RedisCon, mysql: MySqlPool, users: &Arc<UsersChannels>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    login(&mysql)
        .or(anonymous_login())
        .or(register(&mysql))
        .or(websocket(&redis, &users))
        .recover(handle_rejection)
        .boxed().with(cors())
}

// Endpoints

fn websocket(redis: &RedisCon, users: &Arc<UsersChannels>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("stream")
        .and(with_users(users.clone()))
        .and(with_redis(redis.clone()))
        .and(warp::ws())
        .map(|users_sockets: Arc<UsersChannels>, redis: RedisCon, request: Ws|
            request.on_upgrade(|socket| chat_controller::on_client_connect(socket, users_sockets, redis))
        )
}

fn login(database: &MySqlPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("login")
        .and(warp::post())
        .and(warp::body::content_length_limit(JSON_BYTES_SIZE_LIMIT))
        .and(warp::body::json())
        .and(with_mysql(database.clone()))
        .and_then(auth_controller::login)
}

fn anonymous_login() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("anonymous")
        .and(warp::get())
        .and_then(auth_controller::anonymous_login)
}

fn register(database: &MySqlPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("register")
        .and(warp::post())
        .and(warp::body::content_length_limit(JSON_BYTES_SIZE_LIMIT))
        .and(warp::body::json())
        .and(with_mysql(database.clone()))
        .and_then(auth_controller::register)
}

// Filters

fn cors() -> warp::cors::Builder {
    warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST"])
        .allow_header("content-type")
}

fn with_mysql(database: MySqlPool) -> impl Filter<Extract = (MySqlPool,), Error = Infallible> + Clone {
    warp::any().map(move || database.clone())
}

fn with_redis(redis: RedisCon) -> impl Filter<Extract = (RedisCon,), Error = Infallible> + Clone { 
    warp::any().map(move || redis.clone())
}

fn with_users(users: Arc<UsersChannels>) -> impl Filter<Extract = (Arc<UsersChannels>,), Error = Infallible> + Clone {
    warp::any().map(move || users.clone())
}

// Error handling

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    let message: String;
    let status: StatusCode;
    // Custom HTTP Errors
    if let Some(http_rejection) = err.find::<RequestError>() {
        (message, status) = handle_custom_rejection(http_rejection);

    // Deserialization Error
    } else if let Some(bad_request) = err.find::<warp::filters::body::BodyDeserializeError>() {
        message = bad_request.source().map_or_else(|| bad_request.to_string(), |source| source.to_string());
        status = StatusCode::BAD_REQUEST;

    // Missing Authorization header
    } else if let Some(missing_header) = err.find::<warp::reject::MissingHeader>() {
        if missing_header.to_string().contains(AUTHORIZATION.as_str()) {
            message = "You need authentication".to_owned();
            status = StatusCode::UNAUTHORIZED;
        } else {
            return Err(err);
        }
    // Request too large
    } else if let Some(_) = err.find::<warp::reject::PayloadTooLarge>() {
        message = "Your request is too big".to_owned();
        status = StatusCode::PAYLOAD_TOO_LARGE;
    } else {
        return Err(err);
    }

    let response = Response::err(&message);
    return Ok(with_status(json(&response), status));
}

fn handle_custom_rejection(rejection: &RequestError) -> (String, StatusCode) {
    let response = match rejection {
        InternalError(message) => {
            eprintln!("An unexpected error occured: {message}");
            "An unexpected error occured. Please try again later"
        },
        InvalidRequest(message) => message
    };
    (response.to_owned(), rejection.status_code())
}
