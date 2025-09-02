//! Elasticsearch integration module
//!
//! This module provides functions for querying Elasticsearch to retrieve
//! authentication logs, API logs, and compute various aggregations and metrics.
//!
//! The module supports:
//! - Document counting with filtering
//! - Percentile calculations
//! - Terms aggregations for tenant discovery
//! - Time-windowed analysis
//! - API endpoint statistics

use crate::{
    errors::AgentError,
    types::{AggregateMetrics, CountsAndIps, EndpointStat, TimeWindowIps, TopCount},
};
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde_json::json;
use std::collections::BTreeSet;

// Performance constants
/// Default constants for Elasticsearch operations
pub const DEFAULT_QUERY_SIZE: usize = 0; // No documents, only aggregations
pub const DEFAULT_TOP_N: usize = 10;
pub const DEFAULT_PERCENTILES: [f64; 1] = [95.0];
pub const AUTH_LOGS_INDEX: &str = "auth-logs";
pub const API_LOGS_INDEX: &str = "api-logs";
const TENANT_FIELD: &str = "tenant_id.keyword";
const TIME_FIELD: &str = "ts";

/// Send a JSON payload to Elasticsearch
///
/// This is a helper function that handles HTTP requests to Elasticsearch
/// with proper authentication and error handling.
///
/// # Arguments
///
/// * `client` - HTTP client for making requests
/// * `url` - Elasticsearch endpoint URL
/// * `body` - JSON body to send with the request
/// * `auth` - Optional API key for authentication
///
/// # Returns
///
/// * `Result<serde_json::Value, AgentError>` - Elasticsearch response or error
async fn json_post(
    client: &Client,
    url: &str,
    body: &serde_json::Value,
    auth: &Option<String>,
) -> Result<serde_json::Value, AgentError> {
    let mut req = client.post(url).json(body);
    if let Some(h) = auth {
        req = req.header("Authorization", format!("ApiKey {}", h));
    }
    let resp = req
        .send()
        .await?
        .error_for_status()? // Throws for 4xx/5xx status codes
        .json::<serde_json::Value>()
        .await?;
    Ok(resp)
}

/// Check the health of the Elasticsearch cluster
///
/// This function verifies connectivity to the Elasticsearch cluster by
/// attempting to resolve the authentication and API log indices.
/// # Arguments
/// * `client` - HTTP client for making requests
/// * `es_url` - Elasticsearch cluster URL
/// * `es_api_key` - API key for authentication
/// # Returns
/// * `Result<bool, AgentError>` - True if both indices are reachable, else false
pub async fn check_health(
    client: &Client,
    es_url: &str,
    es_api_key: &str,
) -> Result<bool, AgentError> {
    let auth_status = client
        .get(format!("{}/_resolve/index/{}", es_url, AUTH_LOGS_INDEX))
        .header("Authorization", format!("Apikey {}", es_api_key))
        .send()
        .await
        .is_ok();
    let api_status = client
        .get(format!("{}/_resolve/index/{}", es_url, API_LOGS_INDEX))
        .header("Authorization", format!("Apikey {}", es_api_key))
        .send()
        .await
        .is_ok();
    Ok(auth_status && api_status)
}

/// Count documents by tenant in Elasticsearch
///
/// Counts authentication log documents for a specific tenant within a time range.
/// Can optionally filter to count only failed authentication attempts.
///
/// # Arguments
///
/// * `client` - HTTP client for making requests
/// * `es_url` - Elasticsearch cluster URL
/// * `tenant` - Tenant ID to filter by
/// * `from` - Start time in ISO8601/RFC3339 format
/// * `to` - End time in ISO8601/RFC3339 format  
/// * `only_failures` - If true, only count failed authentication attempts
/// * `auth` - Optional API key for authentication
///
/// # Returns
///
/// * `Result<u64, AgentError>` - Document count or error
pub async fn count_by_tenant(
    client: &Client,
    es_url: &str,
    tenant: &str,
    from: &str,
    to: &str,
    only_failures: bool,
    auth: &Option<String>,
) -> Result<u64, AgentError> {
    let mut filters = vec![
        json!({ "term": { TENANT_FIELD: tenant } }),
        json!({ "range": { "ts": { "gte": from, "lt": to } } }),
    ];
    if only_failures {
        filters.push(json!({ "term": { "success": false } }));
    }

    let body = json!({
        "query": { "bool": { "filter": filters } }
    });

    let url = format!(
        "{}/{}/_count",
        es_url.trim_end_matches('/'),
        AUTH_LOGS_INDEX
    );
    let resp = json_post(client, &url, &body, auth).await?;
    Ok(resp.get("count").and_then(|c| c.as_u64()).unwrap_or(0))
}

/// Fetch API percentiles and count from Elasticsearch
///
/// Retrieves the 95th percentile latency for API calls and the total count
/// of API requests for a specific tenant within a time range.
///
/// # Arguments
///
/// * `client` - HTTP client for making requests
/// * `es_url` - Elasticsearch cluster URL
/// * `tenant` - Tenant ID to filter by
/// * `from` - Start time in ISO8601/RFC3339 format
/// * `to` - End time in ISO8601/RFC3339 format
/// * `auth` - Optional API key for authentication
///
/// # Returns
///
/// * `Result<(f64, u64), AgentError>` - Tuple of (p95_latency, count) or error
pub async fn api_percentiles_and_count(
    client: &Client,
    es_url: &str,
    tenant: &str,
    from: &str,
    to: &str,
    auth: &Option<String>,
) -> Result<(f64, u64), AgentError> {
    let body = json!({
        "size": DEFAULT_QUERY_SIZE,
        "track_total_hits": true,
        "query": {
            "bool": {
                "filter": [
                    { "term": { "tenant_id": tenant } },
                    { "range": { TIME_FIELD: { "gte": from, "lt": to } } }
                ]
            }
        },
        "aggs": {
            "percentiles_agg": { "percentiles": { "field": "latency_ms", "percents": DEFAULT_PERCENTILES } }
        }
    });

    let url = format!(
        "{}/{}/_search",
        es_url.trim_end_matches('/'),
        API_LOGS_INDEX
    );
    let resp = json_post(client, &url, &body, auth).await?;

    let count = resp
        .get("hits")
        .and_then(|h| h.get("total"))
        .and_then(|t| t.get("value"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    let p95_latency = resp
        .get("aggregations")
        .and_then(|a| a.get("percentiles_agg"))
        .and_then(|p| p.get("values"))
        .and_then(|v| v.get("95.0"))
        .and_then(|x| x.as_f64())
        .unwrap_or(0.0);

    Ok((p95_latency, count))
}

/// Fetch all unique tenant IDs from Elasticsearch
///
/// This function retrieves a list of all unique tenant IDs from the Elasticsearch cluster
/// by performing a terms aggregation on the tenant_id field. Optionally
/// filters by time range if provided.
///
/// # Arguments
///
/// * `client` - HTTP client for making requests
/// * `es_url` - Elasticsearch cluster URL
/// * `es_api_key` - API key for authentication
/// * `time_range` - Optional time range filter (from, to)
///
/// # Returns
///
/// * `Result<Vec<String>, AgentError>` - List of unique tenant IDs or error
pub async fn fetch_tenant_ids(
    client: &Client,
    es_url: &str,
    es_api_key: &str,
    time_range: Option<(&str, &str)>,
) -> Result<Vec<String>, AgentError> {
    let mut query = json!({ "match_all": {} });
    if let Some((from_time, to_time)) = time_range {
        query = json!({
            "range": { TIME_FIELD: { "gte": from_time, "lt": to_time } }
        });
    }

    let body = json!({
        "size": DEFAULT_QUERY_SIZE,
        "query": query,
        "aggs": {
            "tenant_aggregation": {
                "terms": {
                    "field": TENANT_FIELD,
                    "size": 1000
                }
            }
        }
    });

    let url = format!("{}/{}/_search", es_url, "*");
    let api_key_option = Some(es_api_key.to_string());
    let response = json_post(client, &url, &body, &api_key_option).await?;

    let tenant_buckets = response
        .get("aggregations")
        .and_then(|aggregations| aggregations.get("tenant_aggregation"))
        .and_then(|tenant_agg| tenant_agg.get("buckets"))
        .and_then(|buckets| buckets.as_array())
        .ok_or_else(|| {
            AgentError::Other("missing aggregations.tenant_aggregation.buckets".into())
        })?;

    let mut unique_tenants = BTreeSet::new();
    for bucket in tenant_buckets {
        if let Some(tenant_key) = bucket.get("key").and_then(|k| k.as_str()) {
            unique_tenants.insert(tenant_key.to_string());
        }
    }

    Ok(unique_tenants.into_iter().collect())
}

/// Configuration for fetching authentication failed IPs with windowing
#[derive(Debug, Clone)]
pub struct AuthFailedIpsConfig<'a> {
    pub es_url: &'a str,
    pub tenant: &'a str,
    pub from: &'a str,
    pub to: &'a str,
    pub window: &'a str,
    pub window_ms: i64,
    pub top_n: usize,
    pub auth: &'a Option<String>,
}

/// Fetch authentication failed IPs with windowing
///
/// Analyzes authentication failures by IP address both overall and within
/// time windows. This provides insights into potential security threats
/// and attack patterns.
///
/// # Arguments
///
/// * `client` - HTTP client for making requests
/// * `config` - Configuration containing all query parameters
///
/// # Returns
///
/// * `Result<CountsAndIps, AgentError>` - Overall and windowed IP stats
pub async fn fetch_auth_failed_ips_windowed(
    client: &Client,
    config: AuthFailedIpsConfig<'_>,
) -> Result<CountsAndIps, AgentError> {
    let body = json!({
        "size": DEFAULT_QUERY_SIZE,
        "query": {
            "bool": {
                "filter": [
                    { "term": { TENANT_FIELD: config.tenant } },
                    { "range": { TIME_FIELD: { "gte": config.from, "lt": config.to } } }
                ]
            }
        },
        "aggs": {
            "overall_failed_ips": {
                "filter": { "term": { "success": false } },
                "aggs": {
                    "top_ips": { "terms": { "field": "ip", "size": config.top_n } }
                }
            },
            "time_buckets": {
                "date_histogram": {
                    "field": TIME_FIELD,
                    "fixed_interval": config.window,
                    "min_doc_count": 0,
                    "format": "strict_date_time"
                },
                "aggs": {
                    "failed_in_window": {
                        "filter": { "term": { "success": false } },
                        "aggs": {
                            "top_ips": { "terms": { "field": "ip", "size": config.top_n } }
                        }
                    }
                }
            }
        }
    });

    let url = format!(
        "{}/{}/_search",
        config.es_url.trim_end_matches('/'),
        AUTH_LOGS_INDEX
    );
    let response = json_post(client, &url, &body, config.auth).await?;

    // Extract overall failed IPs
    let overall_ips: Vec<TopCount> = extract_top_ips_from_response(
        &response,
        &["aggregations", "overall_failed_ips", "top_ips", "buckets"],
    );

    // Extract time-bucketed failed IPs
    let time_buckets = response
        .get("aggregations")
        .and_then(|aggs| aggs.get("time_buckets"))
        .and_then(|time_agg| time_agg.get("buckets"))
        .and_then(|buckets| buckets.as_array())
        .cloned()
        .unwrap_or_default();

    let windowed_ips = process_time_buckets(time_buckets, config.window_ms, config.window);

    Ok((overall_ips, windowed_ips))
}

/// Helper function to extract top IPs from Elasticsearch response
fn extract_top_ips_from_response(response: &serde_json::Value, path: &[&str]) -> Vec<TopCount> {
    let mut current = response;
    for segment in path {
        current = match current.get(segment) {
            Some(value) => value,
            None => return Vec::new(),
        };
    }

    current
        .as_array()
        .map(|buckets| {
            buckets
                .iter()
                .filter_map(|bucket| {
                    Some(TopCount {
                        key: bucket.get("key")?.as_str()?.to_string(),
                        count: bucket.get("doc_count")?.as_u64()?,
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Helper function to process time buckets into windowed IP statistics
fn process_time_buckets(
    buckets: Vec<serde_json::Value>,
    window_ms: i64,
    window_str: &str,
) -> Vec<TimeWindowIps> {
    let mut windowed_results = Vec::with_capacity(buckets.len());

    for bucket in buckets {
        let start_timestamp_ms = bucket.get("key").and_then(|k| k.as_i64()).unwrap_or(0);
        let start_time_str = bucket
            .get("key_as_string")
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string();

        let end_time_str =
            format_end_time(start_timestamp_ms, window_ms, &start_time_str, window_str);

        let failed_ips =
            extract_top_ips_from_response(&bucket, &["failed_in_window", "top_ips", "buckets"]);

        windowed_results.push(TimeWindowIps {
            start: start_time_str,
            end: end_time_str,
            top_failed_ips: failed_ips,
        });
    }

    windowed_results
}

/// Helper function to format end time for time windows
fn format_end_time(start_ms: i64, window_ms: i64, start_str: &str, window_str: &str) -> String {
    let end_ms = start_ms + window_ms;
    if end_ms > 0 {
        // Simple display format - could be improved with proper time formatting
        format!("{} + {}", start_str, window_str)
    } else {
        start_str.to_string()
    }
}

/// Fetch the top API endpoints with their failure statistics.
pub async fn fetch_api_endpoints(
    client: &Client,
    es_url: &str,
    tenant: &str,
    from: &str,
    to: &str,
    top_n: usize,
    auth: &Option<String>,
) -> Result<Vec<EndpointStat>, AgentError> {
    let body = json!({
        "size": DEFAULT_QUERY_SIZE,
        "query": {
            "bool": {
                "filter": [
                    { "term": { "tenant_id": tenant } },
                    { "range": { TIME_FIELD: { "gte": from, "lt": to } } }
                ]
            }
        },
        "aggs": {
            "by_endpoint": {
                "terms": { "field": "endpoint", "size": top_n },
                "aggs": {
                    "percentiles": { "percentiles": { "field": "latency_ms", "percents": DEFAULT_PERCENTILES } },
                    "error_count": { "filter": { "terms": { "status_code": [429, 500, 502] } } }
                }
            }
        }
    });

    let url = format!(
        "{}/{}/_search",
        es_url.trim_end_matches('/'),
        API_LOGS_INDEX
    );
    let response = json_post(client, &url, &body, auth).await?;

    let endpoint_buckets = response
        .get("aggregations")
        .and_then(|aggs| aggs.get("by_endpoint"))
        .and_then(|endpoint_agg| endpoint_agg.get("buckets"))
        .and_then(|buckets| buckets.as_array())
        .cloned()
        .unwrap_or_default();

    let mut endpoint_stats = Vec::with_capacity(endpoint_buckets.len());
    for bucket in endpoint_buckets {
        let endpoint = bucket
            .get("key")
            .and_then(|k| k.as_str())
            .unwrap_or("")
            .to_string();
        let total_requests = bucket
            .get("doc_count")
            .and_then(|c| c.as_u64())
            .unwrap_or(0);
        let p95_latency = bucket
            .get("percentiles")
            .and_then(|p| p.get("values"))
            .and_then(|vals| vals.get("95.0"))
            .and_then(|x| x.as_f64())
            .unwrap_or(0.0);
        let error_count = bucket
            .get("error_count")
            .and_then(|e| e.get("doc_count"))
            .and_then(|c| c.as_u64())
            .unwrap_or(0);
        let error_rate = if total_requests > 0 {
            (error_count as f64) / (total_requests as f64)
        } else {
            0.0
        };
        endpoint_stats.push(EndpointStat {
            endpoint,
            count: total_requests,
            p95_latency,
            error_rate,
        });
    }
    Ok(endpoint_stats)
}

/// Count all documents in Elasticsearch
pub async fn count(
    client: &Client,
    es_url: &str,
    from: &str,
    to: &str,
    add_failure_term: bool,
    auth: &Option<String>,
) -> Result<u64, AgentError> {
    let mut filters = vec![json!({ "range": { TIME_FIELD: { "gte": from, "lt": to } } })];
    if add_failure_term {
        filters.push(json!({ "term": { "success": false } }));
    }
    let body = json!({ "query": { "bool": { "filter": filters } } });
    let url = format!(
        "{}/{}/_count",
        es_url.trim_end_matches('/'),
        AUTH_LOGS_INDEX
    );
    let response = json_post(client, &url, &body, auth).await?;
    Ok(response.get("count").and_then(|c| c.as_u64()).unwrap_or(0))
}

/// Fetch API statistics from Elasticsearch
pub async fn fetch_api_stats(
    client: &Client,
    es_url: &str,
    from: &str,
    to: &str,
    auth: &Option<String>,
) -> Result<(f64, u64, u64), AgentError> {
    let body = json!({
        "size": DEFAULT_QUERY_SIZE,
        "track_total_hits": true,
        "query": { "bool": { "filter": [
            { "range": { TIME_FIELD: { "gte": from, "lt": to } } }
        ]}},
        "aggs": {
            "latency_percentiles": { "percentiles": { "field": "latency_ms", "percents": DEFAULT_PERCENTILES } },
            "error_count": { "filter": { "terms": { "status_code": [429, 500, 502] } } }
        }
    });
    let url = format!(
        "{}/{}/_search",
        es_url.trim_end_matches('/'),
        API_LOGS_INDEX
    );
    let response = json_post(client, &url, &body, auth).await?;

    let total_requests = response
        .get("hits")
        .and_then(|h| h.get("total"))
        .and_then(|t| t.get("value"))
        .and_then(|x| x.as_u64())
        .unwrap_or(0);

    let p95_latency = response
        .get("aggregations")
        .and_then(|aggs| aggs.get("latency_percentiles"))
        .and_then(|p| p.get("values"))
        .and_then(|vals| vals.get("95.0"))
        .and_then(|x| x.as_f64())
        .unwrap_or(0.0);

    let error_requests = response
        .get("aggregations")
        .and_then(|aggs| aggs.get("error_count"))
        .and_then(|e| e.get("doc_count"))
        .and_then(|c| c.as_u64())
        .unwrap_or(0);

    Ok((p95_latency, total_requests, error_requests))
}

/// Fetch a snapshot of metrics from Elasticsearch
pub async fn snapshot_metrics(
    client: &Client,
    es_url: &str,
    from: &str,
    to: &str,
    auth: &Option<String>,
) -> Result<AggregateMetrics, AgentError> {
    let total_auth_requests = count(client, es_url, from, to, false, auth).await?;
    let total_failed_requests = count(client, es_url, from, to, true, auth).await?;
    let auth_failure_rate = if total_auth_requests > 0 {
        total_failed_requests as f64 / total_auth_requests as f64
    } else {
        0.0
    };

    let (p95_latency, total_api_requests, api_error_count) =
        fetch_api_stats(client, es_url, from, to, auth).await?;
    let api_error_rate = if total_api_requests > 0 {
        api_error_count as f64 / total_api_requests as f64
    } else {
        0.0
    };

    let from_datetime = DateTime::parse_from_rfc3339(from)
        .map(|d| d.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now());
    let to_datetime = DateTime::parse_from_rfc3339(to)
        .map(|d| d.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now());
    let duration_seconds = (to_datetime - from_datetime).num_seconds().max(1) as f64;
    let requests_per_second = (total_api_requests as f64) / duration_seconds;

    Ok(AggregateMetrics {
        total_auth: total_auth_requests,
        fail_rate: auth_failure_rate,
        p95_latency,
        api_error_rate,
        rps: requests_per_second,
    })
}
