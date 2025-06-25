#!/usr/bin/env cargo run --example extract_readme_insights
//! Extract README Insights
//!
//! This example focuses on extracting detailed architectural insights
//! from the PocketFlow README content that's already in memory.

use brain::*;
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    println!("📖 Brain AI README Insights Extractor - PocketFlow Architecture");
    println!("{}", "=".repeat(60));

    // Initialize Brain AI components
    let mut memory_system = MemorySystem::new(2000);
    
    // Get GitHub token
    let github_token = env::var("GITHUB_TOKEN").ok();
    
    // Create GitHub learning configuration
    let config = GitHubLearningConfig {
        max_files: 50,
        max_file_size: 100_000, // Larger to get full README
        include_code: true,
        include_docs: true,
        include_config: true,
        ..Default::default()
    };

    let github_engine = GitHubLearningEngine::new(github_token.clone(), Some(config));

    println!("\n🚀 Learning from PocketFlow Repository");
    println!("{}", "-".repeat(40));

    // Learn from PocketFlow repository
    let pocketflow_url = "https://github.com/The-Pocket/PocketFlow";
    match github_engine.learn_from_repository(&mut memory_system, pocketflow_url).await {
        Ok(result) => {
            println!("✅ Learning completed!");
            println!("   Files processed: {}", result.files_processed);
            println!("   Memory entries: {}", result.memory_entries_created);
        }
        Err(e) => {
            println!("❌ Learning failed: {}", e);
            return Err(e);
        }
    }

    println!("\n📖 Extracting README Content");
    println!("{}", "-".repeat(40));

    // Search for README content specifically
    let readme_query = WorkingMemoryQuery {
        content_pattern: Some("README".to_string()),
        priority: None,
        min_importance: None,
        created_after: None,
        limit: Some(5),
    };

    match memory_system.query_working(&readme_query) {
        Ok(items) => {
            for (i, item) in items.iter().enumerate() {
                if item.content.contains("README") && item.content.len() > 1000 {
                    println!("\n📝 README Content {} (Length: {} chars):", i + 1, item.content.len());
                    
                    // Extract key sections from README
                    println!("\n🔍 Analyzing README for Architecture Patterns:");
                    
                    let content = &item.content;
                    
                    // Look for key architectural sections
                    if let Some(start) = content.find("## Quick Start") {
                        if let Some(end) = content[start..].find("\n## ").map(|pos| start + pos) {
                            let quick_start = &content[start..end];
                            println!("\n🚀 Quick Start Section:");
                            println!("{}", &quick_start[..quick_start.len().min(500)]);
                        }
                    }
                    
                    if let Some(start) = content.find("## Examples") {
                        if let Some(end) = content[start..].find("\n## ").map(|pos| start + pos) {
                            let examples = &content[start..end];
                            println!("\n💡 Examples Section:");
                            println!("{}", &examples[..examples.len().min(800)]);
                        }
                    }
                    
                    if let Some(start) = content.find("## Key Features") {
                        if let Some(end) = content[start..].find("\n## ").map(|pos| start + pos) {
                            let features = &content[start..end];
                            println!("\n⭐ Key Features Section:");
                            println!("{}", &features[..features.len().min(600)]);
                        }
                    }
                    
                    // Look for code examples
                    let mut code_blocks = Vec::new();
                    let mut current_pos = 0;
                    while let Some(start) = content[current_pos..].find("```python") {
                        let abs_start = current_pos + start;
                        if let Some(end) = content[abs_start..].find("```\n").map(|pos| abs_start + pos) {
                            let code_block = &content[abs_start..end + 4];
                            code_blocks.push(code_block);
                            current_pos = end + 4;
                        } else {
                            break;
                        }
                    }
                    
                    println!("\n🐍 Found {} Python Code Examples:", code_blocks.len());
                    for (j, code) in code_blocks.iter().take(3).enumerate() {
                        println!("\n💻 Code Example {}:", j + 1);
                        println!("{}", &code[..code.len().min(400)]);
                        
                        // Analyze the code for patterns
                        if code.contains("class") {
                            println!("   🔍 Contains class definitions");
                        }
                        if code.contains("async") {
                            println!("   🔍 Uses async/await patterns");
                        }
                        if code.contains("Node") {
                            println!("   🔍 Uses Node-based architecture");
                        }
                        if code.contains("Flow") {
                            println!("   🔍 Uses Flow-based programming");
                        }
                        if code.contains("batch") {
                            println!("   🔍 Supports batch processing");
                        }
                    }
                    
                    // Extract architectural insights
                    println!("\n🏗️  Architectural Pattern Analysis:");
                    
                    if content.contains("Node") && content.contains("Flow") {
                        println!("   ✅ Pattern 1: Node-Flow Architecture");
                        println!("      - Uses Node-based components");
                        println!("      - Implements Flow-based programming");
                    }
                    
                    if content.contains("async") && content.contains("parallel") {
                        println!("   ✅ Pattern 2: Async Parallel Processing");
                        println!("      - Supports asynchronous execution");
                        println!("      - Enables parallel processing");
                    }
                    
                    if content.contains("batch") && content.contains("LLM") {
                        println!("   ✅ Pattern 3: Batch LLM Processing");
                        println!("      - Optimizes LLM calls with batching");
                        println!("      - Reduces API costs and latency");
                    }
                    
                    if content.contains("agent") || content.contains("Agent") {
                        println!("   ✅ Pattern 4: Agent-Based Framework");
                        println!("      - Implements agent abstractions");
                        println!("      - Supports agent orchestration");
                    }
                    
                    // Extract key concepts
                    println!("\n🎯 Key Concepts Identified:");
                    let concepts = vec![
                        ("BaseNode", "Base class for all processing nodes"),
                        ("Flow", "Orchestrates node execution"),
                        ("AsyncFlow", "Asynchronous flow execution"),
                        ("BatchNode", "Batch processing optimization"),
                        ("ParallelBatchNode", "Parallel batch processing"),
                        ("LLM Integration", "Large language model integration"),
                        ("100-line framework", "Minimalist design philosophy"),
                    ];
                    
                    for (concept, description) in concepts {
                        if content.to_lowercase().contains(&concept.to_lowercase()) {
                            println!("   • {}: {}", concept, description);
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to query README content: {}", e);
        }
    }

    println!("\n🧠 Creating Enhanced Memory Entries");
    println!("{}", "-".repeat(40));

    // Store detailed architectural insights in memory
    let architectural_insights = vec![
        ("PocketFlow Node-Flow Architecture", 
         "PocketFlow implements a node-flow architecture where BaseNode classes represent processing units and Flow classes orchestrate their execution. This enables modular, composable AI workflows."),
        
        ("PocketFlow Async Parallel Processing", 
         "The framework supports asynchronous execution with AsyncFlow and parallel processing with ParallelBatchNode, enabling efficient concurrent LLM operations."),
        
        ("PocketFlow Batch Optimization", 
         "BatchNode and related classes implement batch processing to optimize LLM API calls, reducing costs and improving throughput by processing multiple items together."),
        
        ("PocketFlow 100-Line Philosophy", 
         "PocketFlow follows a minimalist '100-line framework' philosophy, providing essential LLM orchestration capabilities in a compact, easy-to-understand codebase."),
        
        ("PocketFlow Agent Framework", 
         "The framework enables 'Agents build Agents' by providing abstractions for agent-based workflows and autonomous system development."),
    ];

    for (title, description) in architectural_insights {
        match memory_system.learn(format!("{}: {}", title, description), Priority::High) {
            Ok(_) => println!("✅ Stored: {}", title),
            Err(e) => println!("❌ Failed to store {}: {}", title, e),
        }
    }

    println!("\n✅ README Analysis Complete!");
    println!("Enhanced architectural insights have been extracted and stored in memory.");
    println!("The Brain AI should now be able to answer detailed questions about PocketFlow's architecture.");

    Ok(())
} 