mod config;
mod query;
mod udf;
mod kafka;
mod observability;
mod api;

use std::error::Error;
use query::execute_query;
use kafka::start_kafka_consumer;
use api::start_api_server;
use observability::{init_monitoring, serve_metrics};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    config::init();
    init_monitoring();
    
    let api_handle = tokio::spawn(start_api_server());
    let metrics_handle = tokio::spawn(serve_metrics());
    
    execute_query().await?;
    start_kafka_consumer().await?;
    
    if let Err(e) = api_handle.await? {
        eprintln!("API server error: {}", e);
    }
    if let Err(e) = metrics_handle.await? {
        eprintln!("Metrics server error: {}", e);
    }
    Ok(())
}
