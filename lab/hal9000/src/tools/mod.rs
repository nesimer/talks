//! Tool implementations for HAL 9000
//!
//! This module contains all the individual tool implementations that power
//! HAL 9000's analysis capabilities. Each tool module provides specific
//! functionality for security analysis and metrics computation.

/// Contextualization tool for detailed security analysis
pub mod contextualize;

/// Spike detection tool for anomaly analysis  
pub mod detect_spike;

/// Metrics fetching tool for performance analysis
pub mod fetch_metrics;

/// Tenant listing tool for multi-tenant discovery
pub mod list_tenants;
