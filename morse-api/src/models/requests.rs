use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String, // TODO Validation?
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenRequest {
    pub token: String,
}
