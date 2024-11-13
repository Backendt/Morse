use argon2::{
    Argon2,
    password_hash::{
        self,
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    }
};
use sqlx::MySqlPool;

use crate::{
    models::{
        auth::User,
        errors::{InternalError, InvalidRequest}
    },
    repositories::user_repository
};

pub async fn validate_login(user_request: &User, database: &MySqlPool) -> Result<bool, warp::reject::Rejection> {
    match user_repository::get_user(&user_request.username, database).await {
        Ok(user) => {
            compare_to_hash(&user_request.password, &user.password).map_err(|err|
                InternalError::new(
                    format!("User has an invalid hash stored as password. {err:?}")
                    .as_str()
                )
            )
        },
        Err(err) => {
            if let Some(_user_not_found) = err.find::<InvalidRequest>() {
                let _ = hash(&user_request.username); // Required to avoid timing attack
                return Ok(false);
            }
            Err(err)
        }
    }
}

pub async fn register_user(user_request: &User, database: &MySqlPool) -> Result<(), warp::reject::Rejection> {
    let user_exists = user_repository::exists(&user_request.username, database).await?;

    let Ok(hashed_password) = hash(&user_request.password) else {
        return Err(InvalidRequest::new("Could not hash the given password"));
    };

    let hashed_user = User {
        username: user_request.username.clone(),
        password: hashed_password
    };

    if !user_exists {
        let created_user = user_repository::create_user(&hashed_user, database).await; // TODO don't block current thread
        if let Err(err) = created_user {
            return Err(
                InternalError::new(
                    format!("Could not save user. {err:?}")
                    .as_str()
                )
            );
        }
    }

    Ok(())
}

// Hashing

fn hash(password: &str) -> Result<String, password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
}

fn compare_to_hash(password: &str, hash: &str) -> Result<bool, password_hash::Error> { 
    let parsed_hash = PasswordHash::new(&hash)?;
    let is_equal = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();
    Ok(is_equal)
}
