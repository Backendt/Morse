use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::models::{JwtClaims, User};


pub fn create_jwt(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(get_jwt_expiration_delay()))
        .expect("Could not add the given hours to current time. Check the JWT_EXP_HOURS environment variable.")
        .timestamp();

    let claims = JwtClaims {
        exp: expiration as usize,
        sub: user.username.to_owned()
    };

    let header = Header::new(Algorithm::HS512);
    let secret = EncodingKey::from_secret(get_jwt_secret().as_bytes());
    encode(&header, &claims, &secret)
}

pub fn get_jwt_username(token: &str) -> Option<String> {
    let secret = DecodingKey::from_secret(get_jwt_secret().as_bytes());
    let decoded_jwt = decode::<JwtClaims>(
        &token,
        &secret,
        &Validation::new(Algorithm::HS512)
    );

    match decoded_jwt {
        Ok(jwt) => Some(jwt.claims.sub), // Return the username
        _ => None
    }
}

fn get_jwt_secret() -> String {
    std::env::var("JWT_SECRET")
        .expect("The JWT_SECRET environment variable is not set.")
}

fn get_jwt_expiration_delay() -> i64 {
    std::env::var("JWT_EXP_HOURS").map_or_else(
        |_err| 10,
        |hours| hours.parse().expect("The JWT_EXP_HOURS environment variable should be a i64 number")
    )
}

