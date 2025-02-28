use std::env;
use std::path::PathBuf;

pub fn get_csv_file_path() -> PathBuf {
    PathBuf::from(env::var("CSV_FILE_PATH").unwrap_or_else(|_| "data/sample.csv".to_string()))
}

pub fn get_parquet_file_path() -> PathBuf {
    PathBuf::from(env::var("PARQUET_FILE_PATH").unwrap_or_else(|_| "data/sample.parquet".to_string()))
}

pub fn get_kafka_broker() -> String {
    env::var("KAFKA_BROKER").unwrap_or_else(|_| "kafka:9092".to_string())
}

pub fn get_kafka_topic() -> String {
    env::var("KAFKA_TOPIC").unwrap_or_else(|_| "transactions".to_string())
}

pub fn get_api_port() -> u16 {
    env::var("API_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000)
}

// Initialize environment from .env file if it exists
pub fn init() {
    dotenv::dotenv().ok();
}
