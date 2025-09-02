//! Configuration management for HAL 9000
//!
//! This module provides configuration structures that can be populated from
//! command line arguments and environment variables using the `clap` crate.

use clap::Parser;
use rmcp::schemars;
use serde::{Deserialize, Serialize};

/// HAL 9000 configuration
///
/// Configuration can be provided via command line arguments or environment variables.
/// Environment variables take precedence over default values but are overridden by
/// explicit command line arguments.
#[derive(Parser, Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Elasticsearch cluster URL
    #[arg(
        long,
        default_value = "http://localhost:9200",
        env = "ES_URL",
        hide_env_values = true,
        help = "Elasticsearch cluster URL"
    )]
    pub es_url: String,

    /// Elasticsearch API key for authentication
    #[arg(
        long,
        default_value = "",
        env = "ES_APIKEY",
        hide_env_values = true,
        help = "Elasticsearch API key for authentication"
    )]
    pub es_api_key: String,
}
