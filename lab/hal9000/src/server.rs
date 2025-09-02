//! HAL 9000 MCP server implementation
//!
//! This module contains the main server implementation that provides Model Context Protocol (MCP)
//! tools for security analysis and metrics computation. The server exposes various tools for
//! analyzing multi-tenant authentication and API logs stored in Elasticsearch.

use crate::config::Config;
use crate::tools::contextualize::{
    ContextualizeRequest, ContextualizeResponse, process as contextualize_process,
};
use crate::tools::detect_spike::{
    DetectSpikeRequest, DetectSpikeResponse, process as detect_spike_process,
};
use crate::tools::fetch_metrics::{
    FetchMetricsRequest, FetchMetricsResponse, process as fetch_metrics_process,
};
use crate::tools::list_tenants::{ListTenantsResponse, process as list_tenants_process};
use anyhow::Context;
use reqwest::Client;
use rmcp::Json;
use rmcp::ServerHandler;
use rmcp::handler::server::{router::tool::ToolRouter, wrapper::Parameters};
use rmcp::model::*;
use rmcp::tool;
use rmcp::{tool_handler, tool_router};

/// HAL 9000 MCP server
///
/// This is the main server struct that implements the MCP protocol for HAL 9000.
/// It provides tools for security analysis, metrics computation, and anomaly detection
/// across multi-tenant environments.
#[derive(Debug, Clone)]
pub struct HAL9000 {
    /// Router for handling tool requests
    tool_router: ToolRouter<Self>,
    /// Server configuration
    cfg: Config,
}

#[tool_router]
impl HAL9000 {
    /// Create a new HAL 9000 server instance
    ///
    /// # Arguments
    ///
    /// * `cfg` - Configuration containing Elasticsearch connection details and other settings
    pub fn new(cfg: Config) -> Self {
        Self {
            tool_router: Self::tool_router(),
            cfg,
        }
    }

    /// Fetch metrics for a specific tenant and time range
    ///
    /// This tool calculates authentication metrics including:
    /// - Total authentication attempts
    /// - Success and failure rates
    /// - 95th percentile latency for API calls
    /// - Requests per second
    ///
    /// # Arguments
    ///
    /// * `request` - Contains tenant ID, start time, and end time for metrics calculation
    ///
    /// # Returns
    ///
    /// * `Result<Json<FetchMetricsResponse>, String>` - Computed metrics or error message
    #[tool(
        name = "fetch_metrics",
        description = "Fetch metrics for a specific tenant and time range"
    )]
    async fn fetch_metrics(
        &self,
        Parameters(request): Parameters<FetchMetricsRequest>,
    ) -> Result<Json<FetchMetricsResponse>, String> {
        let client = Client::new();
        let metrics = fetch_metrics_process(&self.cfg, &client, request)
            .await
            .context("Failed to fetch metrics from Elasticsearch")
            .map_err(|e| e.to_string())?;
        Ok(Json(metrics))
    }

    /// List all tenants discovered in Elasticsearch
    ///
    /// This tool discovers all unique tenant IDs present in the Elasticsearch indices
    /// by performing a terms aggregation on the tenant_id field.
    ///
    /// # Returns
    ///
    /// * `Result<Json<ListTenantsResponse>, String>` - List of tenant IDs or error message
    #[tool(
        name = "list_tenants",
        description = "List tenants discovered in Elasticsearch"
    )]
    async fn list_tenants(&self) -> Result<Json<ListTenantsResponse>, String> {
        let client = Client::new();
        let tenants = list_tenants_process(&self.cfg, &client)
            .await
            .context("Failed to list tenants from Elasticsearch")
            .map_err(|e| e.to_string())?;
        Ok(Json(tenants))
    }

    /// Contextualize metrics for a specific tenant and time range
    ///
    /// This tool provides detailed context about security events including:
    /// - Top IP addresses with failed authentication attempts (overall and windowed)
    /// - Hottest API endpoints with performance metrics
    ///
    /// The analysis can be performed with configurable time windows (e.g., 15m, 1h).
    ///
    /// # Arguments
    ///
    /// * `request` - Contains tenant ID, time range, and optional window size
    ///
    /// # Returns
    ///
    /// * `Result<Json<ContextualizeResponse>, String>` - Contextualization data or error message
    #[tool(
        name = "contextualize",
        description = "Contextualize metrics for a specific tenant and time range"
    )]
    async fn contextualize(
        &self,
        Parameters(request): Parameters<ContextualizeRequest>,
    ) -> Result<Json<ContextualizeResponse>, String> {
        let client = Client::new();
        let response = contextualize_process(&self.cfg, &client, request)
            .await
            .context("Failed to contextualize metrics")
            .map_err(|e| e.to_string())?;
        Ok(Json(response))
    }

    /// Detect spikes in authentication and API metrics
    ///
    /// This tool performs anomaly detection by comparing current metrics against
    /// a baseline period. It analyzes various metrics including:
    /// - Authentication failure rate
    /// - API error rate  
    /// - Request latency (p95)
    /// - Requests per second
    ///
    /// # Arguments
    ///
    /// * `request` - Contains time ranges, window size, and detection thresholds
    ///
    /// # Returns
    ///
    /// * `Result<Json<DetectSpikeResponse>, String>` - Spike detection results or error message
    #[tool(name = "detect_spike", description = "Detect spikes in metrics")]
    async fn detect_spike(
        &self,
        Parameters(request): Parameters<DetectSpikeRequest>,
    ) -> Result<Json<DetectSpikeResponse>, String> {
        let client = Client::new();
        let response = detect_spike_process(&self.cfg, &client, request)
            .await
            .context("Failed to detect spikes in metrics")
            .map_err(|e| e.to_string())?;
        Ok(Json(response))
    }
}

/// MCP server handler implementation
///
/// Implements the ServerHandler trait to provide MCP protocol compliance
/// and server metadata.
#[tool_handler]
impl ServerHandler for HAL9000 {
    /// Get server information and capabilities
    ///
    /// Returns metadata about the HAL 9000 server including its capabilities
    /// and usage instructions.
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            instructions: Some(
                "HAL 9000 - An intelligent security analysis tool for multi-tenant log analysis.\n\
                 Available tools:\n\
                 - fetch_metrics: Calculate authentication and API metrics\n\
                 - list_tenants: Discover available tenants\n\
                 - contextualize: Get detailed security context\n\
                 - detect_spike: Perform anomaly detection\n\
                 "
                .to_string(),
            ),
            ..Default::default()
        }
    }
}
