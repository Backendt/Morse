mod routes;
mod controllers;
mod services;
mod models;
mod repositories;
mod database;

use std::{
    net::SocketAddr,
    collections::HashMap,
    sync::Arc
};
use tokio::sync::RwLock;

use crate::models::ws::UsersChannels;

#[tokio::main]
async fn main() {
    let socket: SocketAddr = std::env::var("LISTENING_SOCKET")
        .unwrap_or_else(|_| String::from("0.0.0.0:8080"))
        .parse().expect("Cannot parse the listening socket. Check your LISTENING_SOCKET environment variable");

    let (mysql, redis) = database::get_connections().await;

    let users: Arc<UsersChannels> = Arc::new(RwLock::new(HashMap::new()));
    let api_routes = routes::get_routes(redis, mysql, &users);

    println!("Running API on {}:{}..", socket.ip(), socket.port());
    warp::serve(api_routes)
        .run(socket)
        .await;
}
