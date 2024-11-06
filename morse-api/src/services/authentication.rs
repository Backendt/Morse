use std::convert::Infallible;
//use warp::http::StatusCode;

use crate::models::requests::*;

pub async fn login(login_request: LoginRequest) -> Result<Box<dyn warp::Reply>, Infallible>{
    let username = login_request.username;
    let password = login_request.password;

    let test_token = format!("token:{username}:{password}:token");
    let response = TokenRequest {token: test_token};
    Ok(Box::new(warp::reply::json(&response)))
}
