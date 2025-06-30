#!/usr/bin/env cargo run --example working_pocketflow_chat
//! Working PocketFlow Chat
//!
//! This example demonstrates a working chat interface that can answer
//! questions about PocketFlow based on stored knowledge.

use brain::*;
use brain_infra::memory::{WorkingMemoryRepository, EpisodicMemoryRepository, SemanticMemoryRepository};
use tokio;
use std::io::{self, Write};
use env_logger;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .init();

    println!("🧠 Working PocketFlow Chat Demo");
    println!("🤖 PocketFlow Knowledge Chat");
    println!("============================");
    println!("Ask me anything about PocketFlow! Type 'quit' to exit.\n");

    // Ensure data directory exists
    std::fs::create_dir_all("data").map_err(|e| BrainError::Io { source: e })?;
    
    // Initialize repositories
    let working_repo = Box::new(WorkingMemoryRepository::new(100));
    let episodic_repo = Box::new(EpisodicMemoryRepository::new("data/pocketflow_chat.db").await?);
    let semantic_repo = Box::new(SemanticMemoryRepository::new());
    
    // Create memory service
    let mut memory_service = MemoryService::new(working_repo, episodic_repo, semantic_repo);

    // Load PocketFlow knowledge
    println!("📚 Loading PocketFlow knowledge...");
    load_pocketflow_knowledge(&mut memory_service).await?;
    println!("✅ Knowledge loaded! Ready to chat.\n");

    // Start chat loop
    loop {
        print!("You: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let question = input.trim();

        if question.is_empty() {
            continue;
        }

        if question.to_lowercase() == "quit" {
            println!("👋 Goodbye!");
            break;
        }

        // Find answer in memory
        match find_answer_in_memory(&memory_service, question).await {
            Some(answer) => {
                println!("🤖 Bot: {}\n", answer);
            }
            None => {
                println!("🤖 Bot: I don't have specific information about that. Could you try rephrasing your question or ask about PocketFlow's architecture, features, or implementation?\n");
            }
        }
    }

    Ok(())
}

async fn load_pocketflow_knowledge(memory_service: &mut MemoryService) -> Result<()> {
    let knowledge_items = vec![
        "PocketFlow is a 100-line AI framework that provides essential LLM orchestration capabilities in a compact, easy-to-understand codebase.",
        "PocketFlow implements three unique architecture patterns: Node-Flow Architecture, Async Parallel Processing, and Batch Optimization Framework.",
        "The Node-Flow pattern in PocketFlow separates processing logic (Nodes) from execution orchestration (Flows). BaseNode is the fundamental abstraction.",
        "PocketFlow supports asynchronous execution with AsyncFlow and parallel processing with ParallelBatchNode for efficient concurrent LLM operations.",
        "BatchNode and ParallelBatchNode in PocketFlow are used to optimize LLM API costs by grouping multiple requests together.",
        "PocketFlow enables agent-based workflows through its 'Agents build Agents' design philosophy, allowing autonomous agents to create and orchestrate other agents.",
        "The key classes in PocketFlow are: BaseNode (base processing unit), Flow (synchronous orchestrator), AsyncFlow (asynchronous orchestrator), BatchNode (batch processor), and ParallelBatchNode (parallel batch processor).",
        "PocketFlow optimizes LLM API costs through BatchNode grouping, ParallelBatchNode concurrent processing, and efficient request management to reduce redundant API calls.",
        "PocketFlow's main use cases include: LLM workflow orchestration, agent-based AI systems, batch processing of AI tasks, parallel LLM operations, cost-optimized AI pipelines, and rapid prototyping of AI agents.",
        "PocketFlow uses Python with async/await patterns for non-blocking operations and leverages asyncio library for asynchronous operations.",
        "PocketFlow implements the observer pattern for flow coordination and uses class inheritance for node specialization.",
        "The framework maintains clean separation of concerns between data processing (nodes) and execution control (flows).",
        "PocketFlow supports error handling, fallback mechanisms, and flexible configuration for both research and production environments.",
        "PocketFlow's minimalist design focuses on core functionality without bloat, making it easy to understand, modify, and extend.",
        "The framework enables recursive and self-improving AI systems where agents can spawn new agents, coordinate multi-agent tasks, and build complex agent hierarchies.",
    ];

    for (i, knowledge) in knowledge_items.iter().enumerate() {
        let priority = if i < 5 { Priority::High } else { Priority::Medium };
        let _id = memory_service.learn(knowledge.to_string(), priority).await?;
    }

    Ok(())
}

async fn find_answer_in_memory(memory_service: &MemoryService, question: &str) -> Option<String> {
    // Extract keywords from the question
    let keywords: Vec<String> = question
        .to_lowercase()
        .split_whitespace()
        .filter(|word| word.len() > 3)
        .map(|word| word.trim_matches(|c: char| !c.is_alphanumeric()))
        .filter(|word| !word.is_empty())
        .map(|word| word.to_string())
        .collect();

    if keywords.is_empty() {
        return None;
    }

    // Create query for working memory with correct field names
    let query = WorkingMemoryQuery {
        content_pattern: Some(keywords[0].clone()),
        priority: None,
        min_importance: None,
        created_after: None,
        limit: Some(5),
    };

    // Search working memory
    if let Ok(results) = memory_service.query_working(&query).await {
        if !results.is_empty() {
            // Return the content of the first relevant result
            return Some(results[0].content.clone());
        }
    }

    // If no results in working memory, try cross-memory search
    if let Ok(cross_results) = memory_service.query_all_memories(&keywords[0]).await {
        if !cross_results.working_results.is_empty() {
            return Some(cross_results.working_results[0].content.clone());
        }
        if !cross_results.episodic_results.is_empty() {
            return Some(cross_results.episodic_results[0].content.clone());
        }
    }

    None
} 