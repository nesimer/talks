//! Spike detection tool implementation
//!
//! This module provides anomaly detection functionality by comparing current
//! metrics against baseline metrics to identify suspicious spikes or anomalies.

use crate::{
    config::Config,
    errors::AgentError,
    es,
    types::{AggregateMetrics, RangeSpec, SpikeResult, SpikeSignal},
};
use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
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

/// Create a spike detection signal
///
/// Compares current value against baseline to determine if there's a suspicious spike.
///
/// # Arguments
///
/// * `metric` - Name of the metric being analyzed
/// * `base` - Baseline value for comparison
/// * `cur` - Current value being evaluated
/// * `th` - Threshold ratio for determining suspiciousness
///
/// # Returns
///
/// * `SpikeSignal` - Analysis result with ratio and suspicion flag
fn mk_signal(metric: &str, base: f64, cur: f64, th: f64) -> SpikeSignal {
    let delta = cur - base;
    let ratio = if base > 0.0 {
        cur / base
    } else if cur > 0.0 {
        f64::INFINITY
    } else {
        1.0
    };
    let suspicious = ratio > th;
    SpikeSignal {
        metric: metric.to_string(),
        baseline: base,
        current: cur,
        delta,
        ratio,
        suspicious,
    }
}

/// Request parameters for spike detection
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct DetectSpikeRequest {
    /// Start time for current period (ISO8601/RFC3339), defaults to now - window
    #[schemars(description = "the start time (ISO8601/RFC3339)")]
    pub from: Option<String>,
    /// End time for current period (ISO8601/RFC3339), defaults to now
    #[schemars(description = "the end time (ISO8601/RFC3339)")]
    pub to: Option<String>,
    /// Time window size (e.g., '15m', '1h'), defaults to '15m'
    #[schemars(description = "the window (e.g., '15m', '1h')")]
    pub window: Option<String>,
    /// General threshold ratio for spike detection, defaults to 1.2
    #[schemars(description = "the spike detection threshold ratio")]
    pub threshold_ratio: Option<f64>,
    /// Threshold for authentication failure rate spikes, defaults to 1.2
    #[schemars(description = "the spike detection threshold fail_rate_ratio")]
    pub threshold_fail_rate_ratio: Option<f64>,
    /// Threshold for latency (p95) spikes, defaults to 1.2
    #[schemars(description = "the spike detection threshold p95_ratio")]
    pub threshold_p95_ratio: Option<f64>,
    /// Threshold for API error rate spikes, defaults to 1.2
    #[schemars(description = "the spike detection threshold error_rate_ratio")]
    pub threshold_error_rate_ratio: Option<f64>,
    /// Threshold for requests per second spikes, defaults to 1.2
    #[schemars(description = "the spike detection threshold rps_ratio")]
    pub threshold_rps_ratio: Option<f64>,
}

/// Response containing spike detection analysis results
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct DetectSpikeResponse {
    /// Complete spike detection analysis results
    #[schemars(description = "the detected spike signals")]
    results: SpikeResult,
}

/// Process a spike detection request
///
/// Performs anomaly detection by comparing metrics from a current time period
/// against metrics from a baseline period (same duration, immediately preceding).
///
/// # Arguments
///
/// * `cfg` - Application configuration containing Elasticsearch settings
/// * `client` - HTTP client for making requests
/// * `input` - Request parameters with time ranges and detection thresholds
///
/// # Returns
///
/// * `Result<DetectSpikeResponse>` - Spike detection results or error
pub async fn process(
    cfg: &Config,
    client: &Client,
    input: DetectSpikeRequest,
) -> Result<DetectSpikeResponse> {
    // Use default window if not provided
    let window = input.window.unwrap_or_else(|| "15m".to_string());

    // Parse window string into milliseconds
    let window_ms = parse_interval_ms(&window).ok_or_else(|| {
        AgentError::Other(format!(
            "Invalid window format (expected e.g. '15m', got '{}')",
            window
        ))
    })?;

    let now = Utc::now();

    // Determine current period time range
    let (cur_from, cur_to) = if let (Some(f), Some(t)) = (input.from.clone(), input.to.clone()) {
        (f, t)
    } else {
        // Default to last 'window' period ending now
        (
            DateTime::from_timestamp_nanos((now.timestamp() - window_ms) * 1_000_000_000)
                .to_rfc3339(),
            now.to_rfc3339(),
        )
    };

    // Parse current period timestamps
    let cur_from_dt = DateTime::parse_from_rfc3339(&cur_from)
        .map(|d| d.with_timezone(&Utc))
        .unwrap_or(now - Duration::minutes(15));
    let cur_to_dt = DateTime::parse_from_rfc3339(&cur_to)
        .map(|d| d.with_timezone(&Utc))
        .unwrap_or(now);

    // Calculate baseline period (same duration, immediately before current)
    let width = cur_to_dt - cur_from_dt;
    let base_from = (cur_from_dt - width).to_rfc3339();
    let base_to = cur_from_dt.to_rfc3339();

    // Fetch current period metrics
    let mc = es::snapshot_metrics(
        client,
        &cfg.es_url,
        &cur_from,
        &cur_to,
        &Some(cfg.es_api_key.to_string()),
    )
    .await
    .unwrap_or(AggregateMetrics {
        total_auth: 0,
        fail_rate: 0.0,
        p95_latency: 0.0,
        api_error_rate: 0.0,
        rps: 0.0,
    });

    // Fetch baseline period metrics
    let mb = es::snapshot_metrics(
        client,
        &cfg.es_url,
        &base_from,
        &base_to,
        &Some(cfg.es_api_key.to_string()),
    )
    .await
    .unwrap_or(AggregateMetrics {
        total_auth: 0,
        fail_rate: 0.0,
        p95_latency: 0.0,
        api_error_rate: 0.0,
        rps: 0.0,
    });

    // Extract threshold values with defaults
    let default_th = input.threshold_ratio.unwrap_or(1.2);
    let th_fail = input.threshold_fail_rate_ratio.unwrap_or(default_th);
    let th_p95 = input.threshold_p95_ratio.unwrap_or(default_th);
    let th_err = input.threshold_error_rate_ratio.unwrap_or(default_th);
    let th_rps = input.threshold_rps_ratio.unwrap_or(default_th);

    // Generate spike signals for each metric
    let signals = vec![
        mk_signal("auth.fail_rate", mb.fail_rate, mc.fail_rate, th_fail),
        mk_signal("api.p95_latency", mb.p95_latency, mc.p95_latency, th_p95),
        mk_signal(
            "api.error_rate",
            mb.api_error_rate,
            mc.api_error_rate,
            th_err,
        ),
        mk_signal("api.rps", mb.rps, mc.rps, th_rps),
    ];

    Ok(DetectSpikeResponse {
        results: SpikeResult {
            current: RangeSpec {
                from: cur_from,
                to: cur_to,
            },
            baseline: RangeSpec {
                from: base_from,
                to: base_to,
            },
            metrics_current: mc,
            metrics_baseline: mb,
            signals,
        },
    })
}
