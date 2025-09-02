//! Fetch metrics tool implementation
//!
//! This module provides functionality to fetch and calculate authentication
//! and API metrics for a specific tenant within a given time range.

use crate::{config::Config, errors::AgentError, es, types::Metrics};
use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::Client;
use rmcp::schemars;
use serde::{Deserialize, Serialize};

/// Request parameters for fetching metrics
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct FetchMetricsRequest {
    /// The tenant identifier to fetch metrics for
    #[schemars(description = "the tenant to fetch metrics for")]
    pub tenant: String,
    /// Start time in ISO8601/RFC3339 format
    #[schemars(description = "the start time (ISO8601/RFC3339)")]
    pub from: String,
    /// End time in ISO8601/RFC3339 format  
    #[schemars(description = "the end time (ISO8601/RFC3339)")]
    pub to: String,
}

/// Response containing computed metrics
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct FetchMetricsResponse {
    /// The computed metrics for the requested tenant and time range
    #[schemars(description = "the fetched metrics")]
    pub metrics: Metrics,
}

/// Parse and validate time range
///
/// Helper function to parse time strings and validate the time range.
fn parse_time_range(from: &str, to: &str) -> Result<(DateTime<Utc>, DateTime<Utc>), AgentError> {
    let from_dt: DateTime<Utc> = DateTime::parse_from_rfc3339(from)
        .map_err(|e| AgentError::TimeFormat(format!("invalid from time '{}': {}", from, e)))?
        .with_timezone(&Utc);
    let to_dt: DateTime<Utc> = DateTime::parse_from_rfc3339(to)
        .map_err(|e| AgentError::TimeFormat(format!("invalid to time '{}': {}", to, e)))?
        .with_timezone(&Utc);

    if to_dt <= from_dt {
        return Err(AgentError::TimeFormat(
            "End time must be after start time".to_string(),
        ));
    }

    Ok((from_dt, to_dt))
}

/// Calculate rates safely
///
/// Helper function to calculate failure and success rates with proper zero handling.
fn calculate_rates(total_auth: u64, total_failures: u64) -> (f64, f64) {
    if total_auth == 0 {
        return (0.0, 0.0);
    }

    let fail_rate = (total_failures as f64) / (total_auth as f64);
    let success_rate = 1.0 - fail_rate;

    (fail_rate, success_rate)
}
///
/// Computes various authentication and API metrics by querying Elasticsearch
/// indices for the specified tenant and time range.
///
/// # Arguments
///
/// * `cfg` - Application configuration containing Elasticsearch settings
/// * `client` - HTTP client for making requests
/// * `input` - Request parameters with tenant ID and time range
///
/// # Returns
///
/// * `Result<FetchMetricsResponse>` - Computed metrics or error
pub async fn process(
    cfg: &Config,
    client: &Client,
    input: FetchMetricsRequest,
) -> Result<FetchMetricsResponse> {
    // Parse and validate time range
    let (from_dt, to_dt) = parse_time_range(&input.from, &input.to)?;

    // Calculate window duration in seconds (avoid division by zero)
    let window_secs = (to_dt - from_dt).num_seconds().max(1) as f64;

    // Get total authentication attempts
    let total_auth = es::count_by_tenant(
        client,
        &cfg.es_url,
        &input.tenant,
        &from_dt.to_rfc3339(),
        &to_dt.to_rfc3339(),
        false,
        &Some(cfg.es_api_key.to_string()),
    )
    .await?;

    // Get failed authentication attempts
    let total_failures = es::count_by_tenant(
        client,
        &cfg.es_url,
        &input.tenant,
        &from_dt.to_rfc3339(),
        &to_dt.to_rfc3339(),
        true,
        &Some(cfg.es_api_key.to_string()),
    )
    .await?;

    // Calculate failure and success rates
    let (fail_rate, success_rate) = calculate_rates(total_auth, total_failures);

    // Get API latency percentiles and request count
    let (p95_latency, api_count) = es::api_percentiles_and_count(
        client,
        &cfg.es_url,
        &input.tenant,
        &from_dt.to_rfc3339(),
        &to_dt.to_rfc3339(),
        &Some(cfg.es_api_key.to_string()),
    )
    .await?;

    // Calculate requests per second
    let rps = (api_count as f64) / window_secs;

    // Create window label for display
    let window_label = format!("{}..{}", input.from, input.to);

    Ok(FetchMetricsResponse {
        metrics: Metrics {
            tenant: input.tenant,
            window: window_label,
            total_auth,
            fail_rate,
            success_rate,
            p95_latency,
            rps,
        },
    })
}
