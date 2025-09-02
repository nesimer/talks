//! Error handling for HAL 9000
//!
//! This module defines custom error types used throughout the HAL 9000 application.
//! All errors implement the standard library's `Error` trait through the `thiserror` crate.

use thiserror::Error;

/// Custom error types for HAL 9000 operations
///
/// This enum encompasses all possible error conditions that can occur during
/// HAL 9000's operation, providing structured error information for better
/// error handling and user feedback.
#[derive(Error, Debug)]
pub enum AgentError {
    /// I/O operation failed
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization/deserialization failed
    #[error("JSON serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    /// HTTP request failed
    #[error("HTTP request error: {0}")]
    Reqwest(#[from] reqwest::Error),

    /// Elasticsearch operation failed
    #[error("Elasticsearch error: {0}")]
    Elasticsearch(String),

    /// Invalid time format or range
    #[error("Time parsing error: {0}")]
    TimeFormat(String),

    /// Configuration validation error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Generic error for other cases
    #[error("Operation failed: {0}")]
    Other(String),
}
