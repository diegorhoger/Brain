//! Brain AI Orchestrator Test - CURRENTLY DISABLED
//!
//! This example demonstrates Brain AI orchestration capabilities but is currently
//! disabled due to API compatibility issues between the example and the updated
//! conversation module.
//!
//! ISSUES TO RESOLVE:
//! 1. Conversation API changed from 4 arguments to 3 arguments
//! 2. Service type mismatches (MemorySystem vs MemoryService, ConceptGraphManager vs ConceptGraphService)
//! 3. PatternDetector integration needs updating
//!
//! For working conversation examples, see:
//! - simple_pocketflow_chat.rs
//! - openai_brain_test.rs
//! - independent_intelligence_demo.rs

/*
// Original code disabled - API compatibility issues

use brain::conversation::{RagOrchestrator, RagRequest};
use brain::memory::MemorySystem;
use brain::concept_graph::{ConceptGraphManager, ConceptGraphConfig};
use brain::insight_extraction::PatternDetector;
use brain::error::BrainError;
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<(), BrainError> {
    println!("🧠 Brain AI Orchestrator Test - True AI Delegation");
    println!("=================================================");
    
    // This example needs updating for new conversation API
    println!("❌ This example is disabled due to API compatibility issues");
    println!("   Please see working examples:");
    println!("   - cargo run --example simple_pocketflow_chat");
    println!("   - cargo run --example openai_brain_test");
    println!("   - cargo run --example independent_intelligence_demo");
    
    Ok(())
}

*/

fn main() {
    println!("🧠 Brain AI Orchestrator Test - DISABLED");
    println!("=========================================");
    println!();
    println!("❌ This example is currently disabled due to API compatibility issues.");
    println!("   The conversation module was updated but this example wasn't migrated.");
    println!();
    println!("✅ Working conversation examples available:");
    println!("   • cargo run --example simple_pocketflow_chat");
    println!("   • cargo run --example openai_brain_test"); 
    println!("   • cargo run --example independent_intelligence_demo");
    println!();
    println!("🔧 Issues to resolve:");
    println!("   1. Conversation API changed from 4 to 3 arguments");
    println!("   2. Service type mismatches (MemorySystem vs MemoryService)");
    println!("   3. PatternDetector integration needs updating");
} 