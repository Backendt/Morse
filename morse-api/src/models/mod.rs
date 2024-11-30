pub mod auth;
pub mod errors;
pub mod ws;

use serde::{self, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Success,
    Error
}

#[derive(Debug, Serialize)]
pub struct Response {
    status: Status,
    code: String,
    message: String
}
impl Response {
    pub fn err(error_code: &str, message: &str) -> Self {
        Self {
            status: Status::Error,
            code: error_code.to_owned(),
            message: message.to_owned()
        }
    }

    pub fn success(status_code: &str, message: &str) -> Self {
        Self {
            status: Status::Success,
            code: status_code.to_owned(),
            message: message.to_owned()
        }
    }
}
