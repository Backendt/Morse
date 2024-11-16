pub mod auth_controller;
pub mod chat_controller;

type WebResult<T> = Result<T, warp::Rejection>;
