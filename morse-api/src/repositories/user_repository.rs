use std::convert::Infallible;

use crate::models::auth::User;

pub async fn get_user(username: &str) -> Result<User, Infallible> {
    // TODO Test implementation
    use argon2::{
        Argon2,
        password_hash::{
            SaltString, PasswordHasher,
            rand_core::OsRng
        }
    };

    let password = b"1234";
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default().hash_password(password, &salt).unwrap().to_string();

    let test_user = User {
        username: username.to_string(),
        password: hash
    };
    Ok(test_user)
}

pub async fn create_user(user: &User) -> Result<&User, Infallible> {
    println!("Creating user {} with password {}", &user.username, &user.password);
    Ok(user)
}

pub async fn exists(username: &str) -> Result<bool, Infallible> {
    Ok(username == "test")
}
