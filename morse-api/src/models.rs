use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
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
