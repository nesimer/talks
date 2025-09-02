//! List tenants tool implementation
//!
//! This module provides functionality to discover all tenant IDs present
//! in the Elasticsearch indices by performing terms aggregations.

use crate::config::Config;
use anyhow::Result;
use reqwest::Client;
use rmcp::schemars;
use serde::{Deserialize, Serialize};

/// Response containing the list of discovered tenant IDs
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ListTenantsResponse {
    /// List of unique tenant identifiers discovered in Elasticsearch
    #[schemars(description = "the list of tenants")]
    tenants: Vec<String>,
}

/// Process a list tenants request
///
/// Discovers all unique tenant IDs present in the Elasticsearch indices
/// by performing a terms aggregation across all indices.
///
/// # Arguments
///
/// * `cfg` - Application configuration containing Elasticsearch settings
/// * `client` - HTTP client for making requests
///
/// # Returns
///
/// * `Result<ListTenantsResponse>` - List of tenant IDs or error
pub async fn process(cfg: &Config, client: &Client) -> Result<ListTenantsResponse> {
    let tenants = crate::es::fetch_tenant_ids(client, &cfg.es_url, &cfg.es_api_key, None).await?;
    Ok(ListTenantsResponse { tenants })
}
