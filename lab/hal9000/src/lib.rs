//! # HAL 9000
//!
//! An intelligent security analysis tool that provides multi-tenant log analysis capabilities.
//!
//! HAL 9000 is a Model Context Protocol (MCP) server implemented in Rust that analyzes
//! authentication and API logs from Elasticsearch. It provides:
//!
//! - **Metrics calculation**: Compute authentication metrics, failure rates, and latency percentiles
//! - **Tenant management**: List and analyze data across multiple tenants
//! - **Contextualization**: Provide detailed context about failed authentication attempts and hot endpoints
//! - **Spike detection**: Detect anomalies and spikes in authentication and API metrics
//!
//! ## Architecture
//!
//! The crate is organized into several modules:
//!
//! - [`config`]: Configuration management with CLI argument parsing
//! - [`server`]: MCP server implementation and tool handlers
//! - [`types`]: Common data structures and type definitions
//! - [`es`]: Elasticsearch integration and query functions
//! - [`tools`]: Individual tool implementations for metrics, analysis, and reporting
//! - [`errors`]: Error handling and custom error types

pub mod config;
pub mod errors;
pub mod es;
pub mod server;
pub mod tools;
pub mod types;
