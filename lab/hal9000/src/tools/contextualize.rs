//! Contextualization tool implementation
//!
//! This module provides detailed security context analysis including
//! failed authentication attempts by IP address and API endpoint statistics.

use crate::{
    config::Config,
    errors::AgentError,
    es,
    types::{Context, RangeSpec},
};
use anyhow::Result;
use reqwest::Client;
use rmcp::schemars;
use serde::{Deserialize, Serialize};

/// Parse string intervals into milliseconds
///
/// Converts time interval strings like "15m", "1h", "2d" into milliseconds.
///
/// # Arguments
///
/// * `s` - Interval string (e.g., "15m", "1h", "2d")
///
/// # Returns
///
/// * `Option<i64>` - Interval in milliseconds or None if invalid format
fn parse_interval_ms(s: &str) -> Option<i64> {
    if s.is_empty() {
        return None;
    }
    let (num, unit) = s.split_at(s.len().saturating_sub(1));
    let n: i64 = num.parse().ok()?;
    match unit {
        "s" => Some(n * 1000),                // seconds
        "m" => Some(n * 60 * 1000),           // minutes
        "h" => Some(n * 60 * 60 * 1000),      // hours
        "d" => Some(n * 24 * 60 * 60 * 1000), // days
        _ => None,
    }
}

/// Request parameters for contextualization analysis
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ContextualizeRequest {
    /// The tenant identifier to analyze
    #[schemars(description = "the tenant to fetch metrics for")]
    pub tenant: String,
    /// Start time in ISO8601/RFC3339 format
    #[schemars(description = "the start time (ISO8601/RFC3339)")]
    pub from: String,
    /// End time in ISO8601/RFC3339 format
    #[schemars(description = "the end time (ISO8601/RFC3339)")]
    pub to: String,
    /// Time window for analysis (e.g., '15m', '1h', '2d')
    #[schemars(description = "the window (e.g., '15m', '1h')")]
    pub window: Option<String>,
}

/// Response containing contextualization analysis results
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ContextualizeResponse {
    /// Detailed security context and analysis results
    #[schemars(description = "the context of metrics")]
    pub contextualization: Context,
}

/// Process a contextualization request
///
/// Provides comprehensive security context including:
/// - Top IP addresses with failed authentication attempts (overall and windowed)
/// - Most active API endpoints with performance metrics
///
/// # Arguments
///
/// * `cfg` - Application configuration containing Elasticsearch settings
/// * `client` - HTTP client for making requests
/// * `input` - Request parameters with tenant ID, time range, and window
///
/// # Returns
///
/// * `Result<ContextualizeResponse>` - Contextualization results or error
pub async fn process(
    cfg: &Config,
    client: &Client,
    input: ContextualizeRequest,
) -> Result<ContextualizeResponse> {
    // Use default window if not provided
    let window = input.window.unwrap_or_else(|| "15m".to_string());

    // Parse window string into milliseconds
    let window_ms = parse_interval_ms(&window).ok_or_else(|| {
        AgentError::Other(format!(
            "Invalid window format (expected e.g. '15m', got '{}')",
            window
        ))
    })?;

    // Number of top results to return (reduced from default for focused results)
    let top_n = 5; // Using 5 instead of es::DEFAULT_TOP_N (10) for more focused results

    // Fetch failed authentication IPs with time windowing
    let auth_config = es::AuthFailedIpsConfig {
        es_url: &cfg.es_url,
        tenant: &input.tenant,
        from: &input.from,
        to: &input.to,
        window: &window,
        window_ms,
        top_n,
        auth: &Some(cfg.es_api_key.to_string()),
    };

    let (overall_ips, by_window) = es::fetch_auth_failed_ips_windowed(client, auth_config)
        .await
        .unwrap_or_default();

    // Fetch hot API endpoints with performance metrics
    let hot_endpoints = es::fetch_api_endpoints(
        client,
        &cfg.es_url,
        &input.tenant,
        &input.from,
        &input.to,
        top_n,
        &Some(cfg.es_api_key.to_string()),
    )
    .await
    .unwrap_or_default();

    Ok(ContextualizeResponse {
        contextualization: Context {
            tenant: input.tenant,
            range: RangeSpec {
                from: input.from,
                to: input.to,
            },
            window,
            top_failed_ips_overall: overall_ips,
            top_failed_ips_windowed: by_window,
            hot_endpoints,
        },
    })
}
