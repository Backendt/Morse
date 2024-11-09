use warp::{
    Filter,
    header,
    http::header::AUTHORIZATION
};
use super::services::*;

pub fn get_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    login().or(register()).or(me())
}

// TODO Used to test
fn me() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("me")
        .and(authenticated())
}

fn authenticated() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
    header::<String>(AUTHORIZATION.as_str())
        .and_then(authorize)
}

async fn authorize(auth_header: String) -> Result<String, warp::Rejection> {
    if auth_header.starts_with("Bearer ") {
        let jwt = auth_header.trim_start_matches("Bearer ");
        if let Some(username) = authentication::get_jwt_username(jwt) {
            return Ok(username);
        }
    }
    Err(warp::reject())
}

fn login() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("login")
        .and(warp::post())
        .and(warp::body::json()) // TODO Set a size limit with a custom json filter
        .and_then(authentication::login)
}

fn register() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("register")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(authentication::register)
}
