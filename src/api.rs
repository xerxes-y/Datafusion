use axum::{routing::get, Router};
use tokio::net::TcpListener;
use std::error::Error;
use crate::config::get_api_port;

pub async fn start_api_server() -> Result<(), Box<dyn Error + Send + Sync>> {
    let app = Router::new().route("/health", get(|| async { "Service is running" }));

    let listener = TcpListener::bind(format!("127.0.0.1:{}", get_api_port()))
        .await
        .map_err(|e| format!("Failed to bind to port {}: {}", get_api_port(), e))?;
        
    println!("API server listening on port {}", get_api_port());
    axum::serve(listener, app)
        .await
        .map_err(|e| format!("Server error: {}", e))?;
        
    Ok(())
}
