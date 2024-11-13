mod routes;
mod controllers;
mod services;
mod models;
mod repositories;
mod database;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let database = database::get_connection().await
        .expect("Could not connect to database.");

    let socket: SocketAddr = std::env::var("LISTENING_SOCKET")
        .unwrap_or_else(|_| String::from("0.0.0.0:8080"))
        .parse().expect("Cannot parse the listening socket. Check your LISTENING_SOCKET environment variable");

    let api_routes = routes::get_routes(database);

    println!("Running API on {}:{}..", socket.ip(), socket.port());
    warp::serve(api_routes)
        .run(socket)
        .await;
}
