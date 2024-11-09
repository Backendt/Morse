use argon2::{
    Argon2,
    password_hash::{
        self,
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    }
};

use crate::models::auth::User;
use crate::repositories::user_repository;

pub async fn validate_login(user_request: &User) -> Result<bool, Box<dyn std::error::Error>> {
    let Ok(user) = user_repository::get_user(&user_request.username).await else { // TODO Handle database connection error
        let _ = hash(&user_request.username); // Required to avoid timing attack
        return Ok(false);
    };
    
    compare_to_hash(&user_request.password, &user.password).or_else(
        |err| {
            eprintln!("User has an invalid hash stored as password. {:?}", err);
            Ok(false)
        }
    )
}

pub async fn register_user(user_request: &User) -> Result<(), Box<dyn std::error::Error>> {
    let user_exists = user_repository::exists(&user_request.username).await?;

    let hashed_password = hash(&user_request.password).expect("TODO Return error that will be translated to bad request"); // TODO

    let hashed_user = User {
        username: user_request.username.clone(),
        password: hashed_password
    };

    if !user_exists {
        let _ = user_repository::create_user(&hashed_user).await; // TODO don't block current thread
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
