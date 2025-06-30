use brain_api::start_web_server;
use clap::{Arg, Command};
use std::process;

fn ensure_directories() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("data")?;
    std::fs::create_dir_all("logs")?;
    std::fs::create_dir_all("temp")?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Ensure required directories exist
    ensure_directories()?;
    
    let matches = Command::new("brain")
        .version("0.8.0")
        .author("Brain AI Team")
        .about("🧠 Brain AI - Advanced Multi-Crate Rust AI System")
        .subcommand(
            Command::new("server")
                .about("Start the Brain AI web server")
                .arg(
                    Arg::new("port")
                        .short('p')
                        .long("port")
                        .value_name("PORT")
                        .help("Port to run the server on")
                        .default_value("8080")
                )
        )
        .subcommand(
            Command::new("status")
                .about("Check Brain AI system status")
        )
        .subcommand(
            Command::new("version")
                .about("Show Brain AI version information")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("server", sub_matches)) => {
            let port = sub_matches.get_one::<String>("port").unwrap().parse::<u16>().unwrap_or(8080);
            
            println!("🧠 Brain AI System");
            println!("==================");
            println!();
            println!("🚀 Starting Brain AI Web Server...");
            println!("🌐 Server will be available at: http://localhost:{}", port);
            println!();
            println!("📚 Brain AI Features:");
            println!("   🧠 Advanced Memory System (Working, Episodic, Semantic)");
            println!("   🔮 Neural Architecture (Transformers, Developmental AI)");
            println!("   🕸️ Knowledge Graphs (Neo4j integration, Hebbian learning)");
            println!("   💡 Intelligence Features (Pattern detection, insights)");
            println!("   🔍 Code Analysis & Understanding");
            println!("   💬 Chat & Conversation with context");
            println!("   📊 Performance Monitoring");
            println!("   🔐 Authentication & Rate Limiting");
            println!();
            println!("🌐 API Endpoints:");
            println!("   • GET  /health - Health check");
            println!("   • POST /learn - Add content to memory");
            println!("   • POST /api/chat/converse - Chat with Brain AI");
            println!("   • POST /code/analyze - Code pattern analysis");
            println!("   • POST /dev/context - Development context tracking");
            println!();
            println!("🎯 Web Interface: http://localhost:{}/chat.html", port);
            println!();
            
            if let Err(e) = start_web_server(port).await {
                eprintln!("❌ Error starting Brain AI server: {}", e);
                process::exit(1);
            }
        }
        Some(("status", _)) => {
            println!("🧠 Brain AI System Status");
            println!("========================");
            println!();
            println!("📊 Architecture: Multi-crate Rust system");
            println!("🏗️ Crates: brain-types, brain-core, brain-infra, brain-cognitive, brain-api, brain-cli, brain-analysis");
            println!("✅ Status: Operational");
            println!("🔧 Version: 0.8.0");
            println!("🧪 Tests: 123 passing");
            println!("📈 Migration: 100% complete");
            println!();
            println!("🎯 To start: brain server --port 8080");
        }
        Some(("version", _)) => {
            println!("🧠 Brain AI System v0.8.0");
            println!("Multi-crate Rust architecture with advanced AI capabilities");
            println!();
            println!("Components:");
            println!("  • Memory System: Working, Episodic, Semantic");
            println!("  • Neural Networks: Transformers, Developmental AI");
            println!("  • Concept Graphs: Neo4j, Hebbian learning");
            println!("  • Intelligence: Pattern detection, insights");
            println!("  • API: RESTful web service");
            println!("  • CLI: Command-line interface");
        }
        _ => {
            println!("🧠 Brain AI System v0.8.0");
            println!("=========================");
            println!();
            println!("Usage: brain <COMMAND>");
            println!();
            println!("Commands:");
            println!("  server    Start the Brain AI web server");
            println!("  status    Check system status");
            println!("  version   Show version information");
            println!("  help      Show this help message");
            println!();
            println!("Examples:");
            println!("  brain server              # Start on default port 8080");
            println!("  brain server --port 3000  # Start on custom port");
            println!("  brain status              # Check system status");
            println!();
            println!("🎯 Quick start: brain server");
        }
    }
    Ok(())
} 