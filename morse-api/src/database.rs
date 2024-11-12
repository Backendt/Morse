use sqlx::MySqlPool;
use std::env;

pub async fn get_connection() -> Result<MySqlPool, sqlx::Error> {
    let database_path = get_mysql_path();
    let connection = MySqlPool::connect(&database_path.as_str()).await?;
    Ok(connection)
}

fn get_mysql_path() -> String {
    let user = env::var("MYSQL_USER").expect("The MYSQL_USER env must be set.");
    let password = env::var("MYSQL_PASSWORD").expect("The MYSQL_PASSWORD env must be set.");
    let host = env::var("MYSQL_HOST").unwrap_or_else(|_| String::from("127.0.0.1"));
    let database = env::var("MYSQL_DATABASE").unwrap_or_else(|_| String::from("morse"));
    
    format!("mysql://{user}:{password}@{host}/{database}")
}
