use warp::{
    reject::Reject,
    Rejection,
    http::StatusCode
};

pub type RequestResult<T> = Result<T, RequestError>;

#[derive(Debug)]
pub enum RequestError {
    InternalError(String),
    InvalidRequest(String)
}
impl Reject for RequestError {}
impl <T>Into<Result<T, Rejection>> for RequestError {
    fn into(self) -> Result<T, Rejection> {
        Err(warp::reject::custom(self))
    }
}
impl RequestError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            RequestError::InvalidRequest(_) => StatusCode::BAD_REQUEST,
            RequestError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
