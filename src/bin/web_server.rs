use brain::{Result, docs_server::{DocsServer, DocsConfig}};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let port = 3030;

    println!("🧠 Starting Brain AI Documentation Server on port {}", port);
    println!("📚 Phase 6: Application Integration - Alternative Web Server");
    
    let mut config = DocsConfig::default();
    config.port = port;
    let docs_server = DocsServer::new(config);
    docs_server.start().await?;
    
    Ok(())
} 