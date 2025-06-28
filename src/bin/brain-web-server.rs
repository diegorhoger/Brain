#![recursion_limit = "1024"]

use brain::{Result, docs_server::{DocsServer, DocsConfig}};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize environment logger
    env_logger::init();

    // Get port from environment or use default
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3030".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    println!("🧠 Starting Brain AI Documentation Server on port {}", port);
    println!("📚 Phase 6: Application Integration - Web Server Available");

    // Create and start the documentation server
    let mut config = DocsConfig::default();
    config.port = port;
    let docs_server = DocsServer::new(config);
    docs_server.start().await?;

    Ok(())
} 