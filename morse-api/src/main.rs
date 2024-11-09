mod routes;
mod controllers;
mod services;
mod models;
mod repositories;

#[tokio::main]
async fn main() {
    let api_routes = routes::get_routes();

    println!("Running API on 127.0.0.1:8080..");
    warp::serve(api_routes)
        .run(([127, 0, 0, 1], 8080)) // TODO Configure listening host and port from env variables
        .await;
}
