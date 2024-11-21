use sqlx::MySqlPool;
use std::env;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

pub type RedisCon = redis::aio::MultiplexedConnection;

pub async fn get_connections() -> (MySqlPool, RedisCon) {
    let mysql = get_mysql_connection().await.expect("Could not connect to MySQL database.");
    let redis = get_redis_connection().await.expect("Could not connect to Redis database.");
    (mysql, redis)
}

async fn get_mysql_connection() -> Result<MySqlPool, sqlx::Error> {
    let path = get_mysql_path();
    MySqlPool::connect(&path.as_str()).await
}

async fn get_redis_connection() -> redis::RedisResult<RedisCon> {
    let path = get_redis_path();
    let client = redis::Client::open(path).expect("Invalid Redis URL");
    client.get_multiplexed_tokio_connection().await
}

fn get_mysql_path() -> String {
    let user = env::var("MYSQL_USER").expect("The MYSQL_USER env is required.");
    let password = env::var("MYSQL_PASSWORD").expect("The MYSQL_PASSWORD env is required.");
    let host = env::var("MYSQL_HOST").unwrap_or_else(|_| String::from("127.0.0.1"));
    let port = env::var("MYSQL_PORT").unwrap_or_else(|_| String::from("3306"));
    let database = env::var("MYSQL_DATABASE").unwrap_or_else(|_| String::from("morse"));
    format!("mysql://{user}:{password}@{host}:{port}/{database}")
}

fn get_redis_path() -> String {
    let host = env::var("REDIS_HOST").unwrap_or_else(|_| String::from("127.0.0.1"));
    let port = env::var("REDIS_PORT").unwrap_or_else(|_| String::from("6379"));
    let password = env::var("REDIS_PASSWORD").expect("The REDIS_PASSWORD env is required.");
    let encoded_password = percent_encode(password.as_bytes(), NON_ALPHANUMERIC).to_string();
    format!("redis://:{encoded_password}@{host}:{port}/")
}
