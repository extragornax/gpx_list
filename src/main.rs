use axum::{routing::get, Router};
use tower_http::services::ServeDir;
use std::net::SocketAddr;
use tokio::net::TcpListener;


#[tokio::main]
async fn main() {
    let static_service = ServeDir::new("static");

    let app = Router::new()
        .nest_service("/static", static_service.clone()) // Serve /static directory properly
        .fallback_service(ServeDir::new("static")); // Fallback for root

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Server running on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}