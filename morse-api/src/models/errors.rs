use warp::{
    reject::Reject,
    Rejection
};

#[derive(Debug)]
pub struct UnauthorizedUser;
impl Reject for UnauthorizedUser {}
impl UnauthorizedUser {
    pub fn new() -> Rejection {
        warp::reject::custom(UnauthorizedUser {})
    }
}

#[derive(Debug)]
pub struct InternalError {
    pub message: String
}
impl InternalError {
    pub fn new(message: &str) -> Rejection {
        warp::reject::custom(
            InternalError {
                message: message.to_string()
            }
        )
    }
}
impl Reject for InternalError {}

#[derive(Debug)]
pub struct InvalidRequest {
    pub message: String
}
impl Reject for InvalidRequest {}
impl InvalidRequest {
    pub fn new(message: &str) -> Rejection {
        warp::reject::custom(
            InvalidRequest {
                message: message.to_string()
            }
        )
    }
}
