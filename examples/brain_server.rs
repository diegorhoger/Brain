#![recursion_limit = "1024"]

use brain_api::start_web_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧠 Brain AI Server - Starting...");
    println!("=========================================");
    println!("");
    println!("🚀 Initializing Brain AI API Server...");
    println!("🌐 Server will be available at: http://localhost:8080");
    println!("");
    println!("📚 Available API Endpoints:");
    println!("   ┌─ Health & Status");
    println!("   │  • GET  /health      - Health check");
    println!("   │  • GET  /status      - System status");
    println!("   │  • GET  /stats       - Performance statistics");
    println!("   │");
    println!("   ┌─ Memory & Learning");
    println!("   │  • POST /learn       - Add content to memory");
    println!("   │  • POST /query       - Query memory system");
    println!("   │");
    println!("   ┌─ Chat & Conversation");
    println!("   │  • POST /chat        - Chat with AI");
    println!("   │  • POST /chat/learn  - Simple chat learning");
    println!("   │  • POST /chat/converse - Simple conversation");
    println!("   │");
    println!("   ┌─ Code Analysis");
    println!("   │  • POST /code/analyze - Code pattern analysis");
    println!("   │");
    println!("   └─ Development Context");
    println!("      • POST /development/context     - Create dev context");
    println!("      • GET  /development/context/:id - Get dev context");
    println!("");
    println!("🔧 Example API Usage:");
    println!("   curl http://localhost:8080/health");
    println!("   curl -X POST http://localhost:8080/learn \\");
    println!("        -H 'Content-Type: application/json' \\");
    println!("        -d '{{\"text\": \"Hello Brain AI!\"}}'");
    println!("");
    println!("📖 For web interface, open: web/brain-interface.html");
    println!("");
    println!("🎯 Press Ctrl+C to stop the server");
    println!("=========================================");
    println!("");
    
    match start_web_server(8080).await {
        Ok(_) => {
            println!("✅ Server started successfully!");
        }
        Err(e) => {
            eprintln!("❌ Failed to start server: {}", e);
            std::process::exit(1);
        }
    }
    
    Ok(())
} 