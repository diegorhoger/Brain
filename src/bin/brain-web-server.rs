#![recursion_limit = "1024"]

use brain::web_server::WebServer;
use brain::error::Result;
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize environment logger
    env_logger::init();

    // Get port from environment or use default
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3030".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    println!("Starting Brain Web Server on port {}", port);

    // Create and start the web server
    let web_server = WebServer::new();
    web_server.start(port).await?;

    Ok(())
} 