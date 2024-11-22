use sqlx::{
    MySqlPool,
    Error
};

use crate::models::auth::User;

type SqlResult<T> = Result<T, Error>;

pub async fn get_user(username: &str, database: &MySqlPool) -> SqlResult<Option<User>> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
        .bind(username)
        .fetch_optional(database)
        .await
}

pub async fn create_user<'a>(user: &'a User, database: &MySqlPool) -> SqlResult<&'a User> {
    sqlx::query("INSERT INTO users(username, password) VALUES (?, ?)")
        .bind(&user.username)
        .bind(&user.password)
        .execute(database)
        .await
        .map(|_| user)
}

pub async fn exists(username: &str, database: &MySqlPool) -> SqlResult<bool> {
    sqlx::query("SELECT 1 FROM users WHERE username = ?")
        .bind(username)
        .fetch_optional(database)
        .await
        .map(|opt| opt.is_some())
}
