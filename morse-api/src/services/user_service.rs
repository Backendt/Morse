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
        errors::{
            RequestResult,
            RequestError::{InternalError, InvalidRequest}
        }
    },
    repositories::user_repository
};

pub async fn validate_login(request: &User, database: &MySqlPool) -> RequestResult<bool> {
    let opt_user = user_repository::get_user(&request.username, database).await
        .map_err(|err| InternalError(format!("Could not get user to validate login. {err:?}")))?;
    
    match opt_user {
        Some(user) => {
            compare_to_hash(&request.password, &user.password)
                .map_err(|err| InternalError(
                    format!("Could not compare request password to stored user hash. {err:?}")
                ))
        },
        None => {
            let _ = hash(&request.username); // Required to avoid timing attack
            Ok(false)
        }
    }
}

pub async fn register_user(request: &User, database: &MySqlPool) -> RequestResult<()> {
    let user_exists = user_repository::exists(&request.username, database).await
        .map_err(|err| InternalError(format!("Could not check if user exist. {err:?}")))?;

    let Ok(hashed_password) = hash(&request.password) else {
        return Err(
            InvalidRequest("Could not hash the given password".to_owned())
        );
    };

    let hashed_user = User {
        username: request.username.to_owned(),
        password: hashed_password
    };

    if !user_exists {
        let _ = user_repository::create_user(&hashed_user, database).await
            .map_err(|err| InternalError(format!("Could not save user. {err:?}")))?;
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
