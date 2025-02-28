use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::BorrowedMessage;
use tokio_stream::StreamExt;
use std::error::Error;
use rdkafka::Message;
use crate::config::{get_kafka_broker, get_kafka_topic};

pub async fn start_kafka_consumer() -> Result<(), Box<dyn Error>> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "rust-consumer")
        .set("bootstrap.servers", get_kafka_broker())
        .create()?;
    consumer.subscribe(&[&get_kafka_topic()])?;

    println!("Listening for new transactions...");
    while let Some(message) = consumer.stream().next().await {
        match message {
            Ok(msg) => handle_kafka_message(&msg).await?,
            Err(err) => eprintln!("Kafka error: {}", err),
        }
    }
    Ok(())
}

async fn handle_kafka_message(msg: &BorrowedMessage<'_>) -> Result<(), Box<dyn Error>> {
    let payload = match msg.payload_view::<str>() {
        Some(Ok(text)) => text,
        Some(Err(_)) | None => return Ok(()),
    };
    println!("Received Kafka message: {}", payload);
    Ok(())
}
