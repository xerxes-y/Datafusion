use opentelemetry::global;
use opentelemetry_sdk::trace::{self, TracerProvider};
use prometheus::{Encoder, Registry};
use std::sync::Arc;
use axum::{routing::get, Router, serve};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use std::error::Error;

lazy_static::lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
}

pub fn init_monitoring() {
    let config = trace::config().with_sampler(trace::Sampler::AlwaysOn);
    let provider = TracerProvider::builder()
        .with_config(config)
        .build();
    global::set_tracer_provider(provider);

    let _exporter = opentelemetry_prometheus::exporter()
        .with_registry(REGISTRY.clone())
        .build()
        .unwrap();
}

async fn metrics_handler() -> Result<String, Box<dyn Error + Send + Sync>> {
    let encoder = prometheus::TextEncoder::new();
    let metric_families = REGISTRY.gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer)
        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
    Ok(String::from_utf8(buffer)
        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?)
}

pub async fn serve_metrics() -> Result<(), Box<dyn Error + Send + Sync>> {
    let app = Router::new()
        .route("/metrics", get(|| async { 
            metrics_handler()
                .await
                .unwrap_or_else(|e| format!("Error gathering metrics: {}", e))
        }));
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 9000));
    let listener = TcpListener::bind(addr).await?;
    println!("Metrics server listening on port 9000");
    serve(listener, app).await?;
    Ok(())
}