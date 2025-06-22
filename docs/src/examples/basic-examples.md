# Basic Examples

This guide provides practical examples for getting started with Brain AI.

## Memory Formation Examples

### Simple Text Learning

```rust
use brain_ai::BrainSystem;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Brain AI system
    let mut brain = BrainSystem::new().await?;
    
    // Form a simple memory
    let result = brain.process_input("The cat sat on the mat").await?;
    
    println!("Memory formed: {}", result.memory_formed);
    println!("Memory ID: {:?}", result.memory_id);
    
    Ok(())
}
```

### Batch Learning

```rust
use brain_ai::BrainSystem;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut brain = BrainSystem::new().await?;
    
    let texts = vec![
        "Rust is a systems programming language",
        "Rust focuses on safety and performance", 
        "Rust has zero-cost abstractions"
    ];
    
    // Process multiple texts
    for text in texts {
        let result = brain.process_input(text).await?;
        println!("Processed: {} -> {}", text, result.memory_formed);
    }
    
    Ok(())
}
```

## Memory Retrieval Examples

### Basic Memory Search

```rust
use brain_ai::BrainSystem;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut brain = BrainSystem::new().await?;
    
    // Add some memories
    brain.process_input("Python is a programming language").await?;
    brain.process_input("JavaScript runs in browsers").await?;
    brain.process_input("Rust is fast and safe").await?;
    
    // Search for memories
    let results = brain.search_memories("programming").await?;
    
    for memory in results {
        println!("Found: {} (confidence: {:.2})", 
                 memory.content, memory.confidence);
    }
    
    Ok(())
}
```

## Concept Graph Examples

### Basic Concept Extraction

```rust
use brain_ai::BrainSystem;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut brain = BrainSystem::new().await?;
    
    // Process text to extract concepts
    brain.process_input("Machine learning algorithms analyze data patterns").await?;
    
    // Get extracted concepts
    let concepts = brain.get_concepts().await?;
    
    for concept in concepts {
        println!("Concept: {} (confidence: {:.2})", 
                 concept.name, concept.confidence);
    }
    
    Ok(())
}
```

## Python Integration Examples

### Basic Python Usage

```python
import brain_ai

# Initialize Brain AI
brain = brain_ai.BrainSystem()

# Learn something
result = brain.process_input("Python integration example")
print(f"Memory formed: {result.memory_formed}")

# Search memories
memories = brain.search_memories("Python")
for memory in memories:
    print(f"Found: {memory.content} (confidence: {memory.confidence:.2f})")
```

### Async Python Usage

```python
import asyncio
import brain_ai

async def main():
    # Initialize async Brain AI
    brain = await brain_ai.AsyncBrainSystem.new()
    
    # Process input
    result = await brain.process_input("Async example")
    print(f"Memory formed: {result.memory_formed}")
    
    # Get insights
    insights = await brain.extract_insights()
    for insight in insights:
        print(f"Insight: {insight.description}")

if __name__ == "__main__":
    asyncio.run(main())
```

## API Usage Examples

### REST API Client

```rust
use reqwest;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    // Learn via API
    let response = client
        .post("http://localhost:8080/api/v1/learn")
        .json(&json!({
            "content": "API learning example",
            "priority": "medium"
        }))
        .send()
        .await?;
    
    let result: serde_json::Value = response.json().await?;
    println!("API Response: {}", result);
    
    Ok(())
}
```

## Configuration Examples

### Custom Configuration

```rust
use brain_ai::{BrainSystem, BrainConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Custom configuration
    let config = BrainConfig::builder()
        .memory_capacity(50000)
        .consolidation_threshold(0.9)
        .concept_discovery_enabled(true)
        .build();
    
    // Initialize with custom config
    let mut brain = BrainSystem::with_config(config).await?;
    
    let result = brain.process_input("Custom configuration example").await?;
    println!("Memory formed: {}", result.memory_formed);
    
    Ok(())
}
```

## Error Handling Examples

### Graceful Error Handling

```rust
use brain_ai::{BrainSystem, BrainError};

#[tokio::main]
async fn main() {
    let mut brain = match BrainSystem::new().await {
        Ok(brain) => brain,
        Err(e) => {
            eprintln!("Failed to initialize Brain AI: {}", e);
            return;
        }
    };
    
    // Handle different error types
    match brain.process_input("").await {
        Ok(result) => println!("Success: {:?}", result),
        Err(BrainError::InvalidInput(msg)) => {
            println!("Invalid input: {}", msg);
        },
        Err(e) => {
            println!("Other error: {}", e);
        }
    }
}
```

These basic examples provide a foundation for working with Brain AI across different use cases.
