mod sensor;
mod handlers;

use axum::{
    http::Request, routing::{delete, get, post, put}, Router
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Build our application with a route
    let app = Router::new()
        .route("/sensors", get(handlers::list_sensors));

        
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

