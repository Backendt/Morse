pub mod auth;
pub mod errors;
pub mod ws;

use serde::{self, Serialize};
use crate::models::ws::Action;

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Success,
    Error
}

#[derive(Debug, Serialize)]
pub struct Response {
    status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Action>,
    message: String
}
impl Response {
    pub fn err(message: &str) -> Self {
        Self {
            status: Status::Error,
            action: None,
            message: message.to_owned()
        }
    }

    pub fn action_err(action: &Action, message: &str) -> Self {
        Self {
            status: Status::Error,
            action: Some(action.clone()),
            message: message.to_owned()
        }
    }

    pub fn action_success(action: &Action, message: &str) -> Self {
        Self {
            status: Status::Success,
            action: Some(action.clone()),
            message: message.to_owned()
        }
    }

    pub fn success(message: &str) -> Self {
        Self {
            status: Status::Success,
            action: None,
            message: message.to_owned()
        }
    }
}
