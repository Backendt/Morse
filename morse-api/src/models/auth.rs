use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub username: String, // TODO Validation?
    pub password: String,
}

#[derive(Serialize)]
pub struct JwtResponse {
    pub token: String
}

#[derive(Serialize, Deserialize)]
pub struct JwtClaims {
    pub exp: usize, // Expiration timestamp
    pub sub: String // Subject (username)
}

