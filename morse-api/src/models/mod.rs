use warp::{
    reply::{json, Json, WithStatus, with_status},
    http::StatusCode
};

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
    pub fn new(message: &str, status: StatusCode) -> Self {
        Self {
            status: status.as_u16(),
            message: message.to_owned()
        }
    }

    pub fn as_reply(&self) -> WithStatus<Json> {
        let status = StatusCode::from_u16(self.status)
            .expect("Invalid http status code");
        with_status(json(&self), status)
    }
}
