use clap::Parser;
use hal9000::{config::Config, server};
use rmcp::{ServiceExt, transport::stdio};
use tracing_subscriber::{self, EnvFilter};

/// Main entry point for HAL 9000 MCP server
///
/// HAL 9000 is an intelligent security analysis tool that provides
/// multi-tenant log analysis, metrics calculation, spike detection,
/// and contextualization capabilities via the Model Context Protocol (MCP).
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize structured logging with stderr output and debug level
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting HAL 9000 MCP server...");

    // Parse configuration from command line arguments and environment variables
    let cfg = Config::parse();

    // Create and start the MCP server with stdio transport
    let server = server::HAL9000::new(cfg)
        .serve(stdio())
        .await
        .inspect_err(|e| tracing::error!("Failed to start server: {:?}", e))?;

    // Block until the server shuts down
    server.waiting().await?;

    tracing::info!("HAL 9000 MCP server shut down successfully");
    Ok(())
}
