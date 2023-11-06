use axum::Server;
use dotenvy::{dotenv, var};
use std::net::TcpListener;

mod config;
mod routes;
mod handlers;

#[tokio::main]
async fn main() {
    dotenv().ok();
    config::tracing::initialize_tracing_subscriber();

    let incoming = TcpListener::bind(var("HOST").unwrap()).unwrap();

    Server::from_tcp(incoming)
        .unwrap()
        .serve(routes::service_routes().await)
        .await
        .unwrap();
}