pub mod auth_controller;

type WebResult<T> = Result<T, warp::Rejection>;
