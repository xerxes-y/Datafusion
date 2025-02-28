# DataFusion Processing System

A high-performance data processing system built with Rust, DataFusion, Kafka, and Apache Arrow.

## Overview

This project implements a scalable data processing pipeline that:
- Processes CSV and Parquet files using DataFusion
- Exposes a REST API for data queries and management
- Consumes messages from Kafka for real-time processing
- Provides comprehensive metrics and monitoring

## Features

- **High-Performance SQL Processing**: Execute SQL queries against CSV and Parquet data
- **Real-time Data Processing**: Consume and process Kafka messages
- **REST API**: Query data and manage the system via HTTP endpoints
- **Metrics & Monitoring**: Track system performance with Prometheus-compatible metrics
- **Scalable Architecture**: Designed for horizontal scaling

## Technology Stack

- **Rust**: Core programming language
- **DataFusion**: SQL query engine based on Apache Arrow
- **Apache Arrow**: In-memory columnar data format
- **Apache Parquet**: Efficient columnar storage format
- **Kafka**: Distributed event streaming platform
- **Docker & Docker Compose**: Containerization and orchestration

## Getting Started

### Prerequisites

- Docker and Docker Compose
- Rust (for development)
- Git

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/xerxes-y/Datafusion.git
   cd Datafusion
   ```

2. Start the services:
   ```bash
   docker compose up -d
   ```

3. Verify the services are running:
   ```bash
   docker compose ps
   ```

### API Endpoints

- **Health Check**: `GET http://localhost:3000/health`
- **Metrics**: `GET http://localhost:9000/metrics`
- **Query Data**: `POST http://localhost:3000/query`
  ```json
  {
    "sql": "SELECT * FROM transactions LIMIT 10"
  }
  ```

## Development

### Building from Source

```bash
# Build the project
cargo build --release

# Run tests
cargo test

# Run locally
cargo run
```

### Project Structure

- `src/main.rs`: Application entry point
- `src/api.rs`: HTTP API server implementation
- `src/kafka.rs`: Kafka consumer implementation
- `src/query.rs`: DataFusion query execution
- `src/udf.rs`: User-defined functions
- `src/observability.rs`: Metrics and monitoring
- `src/config.rs`: Configuration management

## Configuration

The application can be configured using environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `RUST_LOG` | Logging level | `info` |
| `KAFKA_BROKER` | Kafka broker address | `kafka:9092` |
| `KAFKA_TOPIC` | Kafka topic to consume | `transactions` |
| `API_PORT` | API server port | `3000` |

## Performance

The system is designed to handle:
- Up to 100,000 transactions per second for simple queries
- 50,000 TPS for complex aggregations
- Sub-millisecond latency for data operations

## Docker Compose Services

The project includes the following services:

- **datafusion**: The main application
- **kafka**: Message broker for streaming data
- **zookeeper**: Required for Kafka coordination

## Troubleshooting

### Common Issues

1. **Kafka Connection Issues**
   - Ensure Zookeeper is healthy
   - Check Kafka logs: `docker compose logs kafka`

2. **Permission Issues**
   - The containers run as root to avoid volume permission problems
   - Ensure your host system allows this

3. **Performance Issues**
   - Adjust Kafka and JVM settings in docker-compose.yaml
   - Consider increasing container resources

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Apache Arrow](https://arrow.apache.org/)
- [DataFusion](https://github.com/apache/arrow-datafusion)
- [Apache Kafka](https://kafka.apache.org/) 
