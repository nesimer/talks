# HAL 9000

An intelligent security analysis tool implemented as a Model Context Protocol (MCP) server in Rust. HAL 9000 analyzes multi-tenant authentication and API logs stored in Elasticsearch to provide comprehensive security insights, metrics calculation, anomaly detection, and contextualization.

## Features

- **Multi-tenant Analysis**: Analyze logs across multiple tenants with tenant isolation
- **Metrics Calculation**: Compute authentication metrics, failure rates, and API performance statistics
- **Spike Detection**: Detect anomalies and spikes by comparing current metrics against historical baselines
- **Security Contextualization**: Identify suspicious IP addresses and analyze API endpoint patterns
- **MCP Integration**: Exposes functionality through Model Context Protocol for use with AI assistants

## Architecture

HAL 9000 is built using:

- **Rust**: High-performance systems programming language
- **Model Context Protocol (MCP)**: Enables integration with AI assistants and tools
- **Elasticsearch**: Backend for log storage and analysis
- **Tokio**: Asynchronous runtime for concurrent operations

## Available Tools

### 1. `fetch_metrics`

Calculates comprehensive metrics for a specific tenant and time range:

- Total authentication attempts
- Success and failure rates  
- 95th percentile API latency
- Requests per second

### 2. `list_tenants`

Discovers all unique tenant IDs present in the Elasticsearch indices.

### 3. `contextualize`

Provides detailed security context including:

- Top IP addresses with failed authentication attempts (overall and time-windowed)
- Most active API endpoints with performance metrics
- Configurable time windows (e.g., 15m, 1h, 2d)

### 4. `detect_spike`

Performs anomaly detection by comparing current metrics against baseline periods:

- Authentication failure rate spikes
- API error rate anomalies
- Latency (p95) spikes
- Request volume anomalies

## Configuration

HAL 9000 can be configured via command line arguments or environment variables:

- `--es-url` / `ES_URL`: Elasticsearch cluster URL (default: `http://localhost:9200`)
- `--es-api-key` / `ES_APIKEY`: Elasticsearch API key for authentication
- `--report-dir`: Directory for generated reports (default: `./reports`)

## Build

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Cross-compile for Windows
cargo build --release --target x86_64-pc-windows-gnu
```

## Usage

HAL 9000 runs as an MCP server using stdio transport:

```bash
./hal9000 --es-url "https://your-elasticsearch-cluster:9200" --es-api-key "your-api-key"
```

## Data Requirements

HAL 9000 expects Elasticsearch indices with the following structure:

### Authentication Logs (`auth-logs`)

- `tenant_id`: Tenant identifier
- `ts`: Timestamp (ISO8601/RFC3339)
- `success`: Boolean indicating authentication success
- `ip`: Source IP address

### API Logs (`api-logs`)

- `tenant_id`: Tenant identifier
- `ts`: Timestamp (ISO8601/RFC3339)
- `latency_ms`: Request latency in milliseconds
- `endpoint`: API endpoint path
- `error`: Boolean indicating if request resulted in error

## License

This project is part of a laboratory/demonstration environment for security analysis tools.
