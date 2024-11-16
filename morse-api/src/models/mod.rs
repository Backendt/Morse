pub mod auth;
pub mod errors;
pub mod ws;

use serde::Serialize;

#[derive(Serialize)]
pub struct APIMessage {
    pub status: u16,
    pub message: String
}

impl APIMessage {
    pub fn new(message: &str, status: warp::http::StatusCode) -> Self {
        Self {
            status: status.as_u16(),
            message: message.to_string()
        }
    }

    pub fn as_reply(&self) -> warp::reply::WithStatus<warp::reply::Json> {
        let status = warp::http::StatusCode::from_u16(self.status).expect("Invalid http status code");
        warp::reply::with_status(warp::reply::json(&self), status)
    }
}
