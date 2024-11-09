use warp::{
    Filter,
    header,
    http::header::AUTHORIZATION
};
use super::controllers::*;

pub fn get_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    login().or(register()).or(me())
}

// Endpoints

fn me() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("me")
        .and(authenticated())
}

fn login() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("login")
        .and(warp::post())
        .and(warp::body::json()) // TODO Set a size limit with a custom json filter
        .and_then(auth_controller::login)
}

fn register() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("register")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(auth_controller::register)
}

// Filters

fn authenticated() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
    header::<String>(AUTHORIZATION.as_str())
        .and_then(auth_controller::get_current_username)
}
