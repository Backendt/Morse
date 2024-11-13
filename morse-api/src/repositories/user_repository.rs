use warp::reject::Rejection;
use sqlx::MySqlPool;

use crate::models::{
    auth::User,
    errors::{InternalError, InvalidRequest}
};

pub async fn get_user(username: &str, database: &MySqlPool) -> Result<User, Rejection> {
    let query = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
        .bind(username)
        .fetch_optional(database)
        .await;

    match query {
        Ok(opt_user) => {
            match opt_user {
                Some(user) => Ok(user),
                None => Err(InvalidRequest::new("User not found"))
            }
        },
        Err(err) => Err(InternalError::new(
            format!("Could not query user from username. {err:?}").as_str()
        ))
    }
}

pub async fn create_user<'a>(user: &'a User, database: &MySqlPool) -> Result<&'a User, Rejection> {
    let query = sqlx::query("INSERT INTO users(username, password) VALUES (?, ?)")
        .bind(&user.username)
        .bind(&user.password)
        .execute(database)
        .await;

    match query {
        Ok(_) => Ok(user),
        Err(err) => Err(InternalError::new(
            format!("Could not insert new user. {err:?}").as_str()
        ))
    }
}

pub async fn exists(username: &str, database: &MySqlPool) -> Result<bool, Rejection> {
    let query = sqlx::query("SELECT 1 FROM users WHERE username = ?")
        .bind(username)
        .fetch_optional(database)
        .await;

    match query {
        Ok(result) => Ok(result.is_some()),
        Err(err) => Err(InternalError::new(
            format!("Could not check if user exists. {err:?}").as_str()
        ))
    }
}
