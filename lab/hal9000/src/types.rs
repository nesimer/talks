//! Common type definitions for HAL 9000
//!
//! This module contains all the data structures used throughout the HAL 9000
//! application for representing metrics, analysis results, and API requests/responses.

use rmcp::schemars;
use serde::{Deserialize, Serialize};

/// Metrics for a specific tenant and time window
///
/// Contains authentication metrics including success/failure rates,
/// latency information, and request rates.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema, Clone)]
pub struct Metrics {
    /// The tenant identifier
    pub tenant: String,
    /// Time window description (e.g., "2025-01-01T00:00:00Z..2025-01-01T01:00:00Z")
    pub window: String,
    /// Total number of authentication attempts
    pub total_auth: u64,
    /// Failure rate as a ratio (0.0 to 1.0)
    pub fail_rate: f64,
    /// Success rate as a ratio (0.0 to 1.0)
    pub success_rate: f64,
    /// 95th percentile latency in milliseconds
    pub p95_latency: f64,
    /// Requests per second
    pub rps: f64,
}

/// Time range specification
///
/// Represents a time range with start and end timestamps in ISO8601/RFC3339 format.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema, Clone)]
pub struct RangeSpec {
    /// Start time (ISO8601/RFC3339)
    pub from: String,
    /// End time (ISO8601/RFC3339)
    pub to: String,
}

/// Top item count pair
///
/// Represents a key-value pair where the key is typically an identifier
/// (like an IP address or endpoint) and the count is the number of occurrences.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema, Clone)]
pub struct TopCount {
    /// The identifier (e.g., IP address, endpoint)
    pub key: String,
    /// Number of occurrences
    pub count: u64,
}

/// Endpoint statistics
///
/// Contains performance and error metrics for a specific API endpoint.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema, Clone)]
pub struct EndpointStat {
    /// API endpoint path
    pub endpoint: String,
    /// Total number of requests to this endpoint
    pub count: u64,
    /// 95th percentile latency in milliseconds
    pub p95_latency: f64,
    /// Error rate as a ratio (0.0 to 1.0)
    pub error_rate: f64,
}

/// Time-windowed IP addresses with failed authentication attempts
///
/// Contains information about IP addresses that had the most failed
/// authentication attempts within a specific time window.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema, Clone)]
pub struct TimeWindowIps {
    /// Window start time (ISO8601/RFC3339)
    pub start: String,
    /// Window end time (ISO8601/RFC3339)
    pub end: String,
    /// Top IP addresses with failed authentication attempts
    pub top_failed_ips: Vec<TopCount>,
}

/// Contextualization data
///
/// Provides comprehensive context about authentication failures and API usage
/// patterns for a specific tenant and time range.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema, Clone)]
pub struct Context {
    /// The tenant identifier
    pub tenant: String,
    /// Time range for the analysis
    pub range: RangeSpec,
    /// Window size used for time-based analysis (e.g., "15m", "1h")
    pub window: String,
    /// Overall top IP addresses with failed authentication attempts
    pub top_failed_ips_overall: Vec<TopCount>,
    /// Time-windowed analysis of failed authentication attempts by IP
    pub top_failed_ips_windowed: Vec<TimeWindowIps>,
    /// Most active API endpoints with performance metrics
    pub hot_endpoints: Vec<EndpointStat>,
}

/// Combined counts and IP information
///
/// Return type for authentication failed IPs analysis, containing both
/// overall and time-windowed results.
pub type CountsAndIps = (Vec<TopCount>, Vec<TimeWindowIps>);

/// Spike detection threshold configuration
///
/// Contains optional thresholds for various metrics used in spike detection.
/// If a threshold is None, the default threshold will be used.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema, Clone, Default)]
pub struct SpikeThresholds {
    /// General ratio threshold for spike detection
    pub ratio: Option<f64>,
    /// Threshold for authentication failure rate spikes
    pub fail_rate_ratio: Option<f64>,
    /// Threshold for latency (p95) spikes
    pub p95_ratio: Option<f64>,
    /// Threshold for API error rate spikes
    pub error_rate_ratio: Option<f64>,
    /// Threshold for requests per second spikes
    pub rps_ratio: Option<f64>,
}

/// Aggregated metrics across multiple data sources
///
/// Contains key performance indicators aggregated from both authentication
/// and API logs for comprehensive analysis.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema, Clone)]
pub struct AggregateMetrics {
    /// Total authentication attempts
    pub total_auth: u64,
    /// Authentication failure rate (0.0 to 1.0)
    pub fail_rate: f64,
    /// 95th percentile latency in milliseconds
    pub p95_latency: f64,
    /// API error rate (0.0 to 1.0)
    pub api_error_rate: f64,
    /// Requests per second
    pub rps: f64,
}

/// Individual spike detection signal
///
/// Represents a single metric's spike detection analysis, comparing
/// current values against baseline values.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema, Clone)]
pub struct SpikeSignal {
    /// Name of the metric being analyzed
    pub metric: String,
    /// Baseline value for comparison
    pub baseline: f64,
    /// Current value being evaluated
    pub current: f64,
    /// Absolute difference (current - baseline)
    pub delta: f64,
    /// Ratio of current to baseline (current / baseline)
    pub ratio: f64,
    /// Whether this signal indicates a suspicious spike
    pub suspicious: bool,
}

/// Complete spike detection result
///
/// Contains the full analysis of spike detection including time ranges,
/// metrics comparison, and individual spike signals.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema, Clone)]
pub struct SpikeResult {
    /// Time range for current metrics
    pub current: RangeSpec,
    /// Time range for baseline metrics
    pub baseline: RangeSpec,
    /// Current period metrics
    pub metrics_current: AggregateMetrics,
    /// Baseline period metrics
    pub metrics_baseline: AggregateMetrics,
    /// Individual spike detection signals
    pub signals: Vec<SpikeSignal>,
}
