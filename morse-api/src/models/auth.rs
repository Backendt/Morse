use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use regex::Regex;
use crate::models::errors::RequestError::{self, InvalidRequest};

const USERNAME_REGEX: &str = r"^[a-zA-Z0-9._\- ]{5,20}$";
const USERNAME_ERROR: &str = "The username should be between 5 and 20 characters, and should not contain special characters other than: ._-";
const PASSWORD_SIZE_RESTRICTIONS: (usize, usize) = (8, 30);

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub username: String,
    pub password: String,
}
impl User {
    pub fn validated(self) -> Result<Self, RequestError> { // TODO Trim username
        let name_regex = Regex::new(USERNAME_REGEX).expect("Username regex is invalid");
        if !name_regex.is_match(&self.username) {
            return Err(InvalidRequest(USERNAME_ERROR.to_owned()));
        }

        let (min_password_size, max_password_size) = PASSWORD_SIZE_RESTRICTIONS.clone();
        let password_size = self.password.len();
        if password_size > max_password_size || password_size < min_password_size {
            let error_message = format!("The password should be between {min_password_size} and {max_password_size} characters.").to_owned();
            return Err(InvalidRequest(error_message));
        }

        Ok(self)
    }
}

#[derive(Serialize, Deserialize)]
pub struct JwtClaims {
    pub exp: usize, // Expiration timestamp
    pub sub: String // Subject (username)
}

