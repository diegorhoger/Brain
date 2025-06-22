# Testing Strategy

This guide covers the comprehensive testing strategy for Brain AI, including unit tests, integration tests, performance tests, and testing best practices.

## Testing Philosophy

Brain AI follows a multi-layered testing approach:

1. **Unit Tests**: Test individual components in isolation
2. **Integration Tests**: Test component interactions
3. **System Tests**: Test the complete system end-to-end
4. **Performance Tests**: Validate performance characteristics
5. **Property Tests**: Test with generated inputs

## Test Organization

### Test Structure

```
tests/
├── unit/                    # Unit tests (also in src/ modules)
├── integration/             # Integration tests
├── system/                  # System-level tests
├── performance/             # Performance benchmarks
├── fixtures/                # Test data and fixtures
└── common/                  # Shared test utilities
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test categories
cargo test --test unit_tests
cargo test --test integration_tests
cargo test --test system_integration_tests

# Run with nextest (faster)
cargo nextest run

# Run with coverage
cargo tarpaulin --out html
```

## Unit Testing

### Component-Level Tests

Each component has comprehensive unit tests:

```rust
// Example: Memory system unit tests
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_formation() {
        let mut memory_system = MemorySystem::new(1000);
        let content = "test content";
        
        let memory_id = memory_system.form_memory(content, 0.8).unwrap();
        assert!(memory_system.get_memory(&memory_id).is_some());
    }
    
    #[test]
    fn test_memory_consolidation() {
        let mut memory_system = MemorySystem::new(1000);
        
        // Add multiple memories
        for i in 0..10 {
            memory_system.form_memory(&format!("content {}", i), 0.8).unwrap();
        }
        
        // Trigger consolidation
        memory_system.consolidate_memories();
        
        // Verify consolidation occurred
        assert!(memory_system.get_consolidation_count() > 0);
    }
    
    #[tokio::test]
    async fn test_async_memory_operations() {
        let memory_system = Arc::new(Mutex::new(MemorySystem::new(1000)));
        
        // Test concurrent memory formation
        let handles: Vec<_> = (0..10).map(|i| {
            let memory_system = Arc::clone(&memory_system);
            tokio::spawn(async move {
                let content = format!("async content {}", i);
                memory_system.lock().await.form_memory(&content, 0.8)
            })
        }).collect();
        
        // Wait for all operations
        for handle in handles {
            handle.await.unwrap().unwrap();
        }
        
        assert_eq!(memory_system.lock().await.memory_count(), 10);
    }
}
```

### Testing Utilities

```rust
// tests/common/mod.rs
pub fn create_test_memory_system() -> MemorySystem {
    MemorySystem::new(1000)
}

pub fn create_test_concept_graph() -> ConceptGraph {
    ConceptGraph::new()
}

pub fn load_test_data(filename: &str) -> String {
    std::fs::read_to_string(format!("tests/fixtures/{}", filename))
        .expect("Failed to load test data")
}

pub async fn wait_for_condition<F>(mut condition: F, timeout: Duration) -> bool
where
    F: FnMut() -> bool,
{
    let start = Instant::now();
    while start.elapsed() < timeout {
        if condition() {
            return true;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    false
}
```

## Integration Testing

### Component Integration Tests

```rust
// tests/integration_tests.rs
use brain_ai::*;
use tokio;

#[tokio::test]
async fn test_memory_concept_integration() {
    let mut system = BrainSystem::new().await.unwrap();
    
    // Form memory
    let input = "The cat sat on the mat";
    let result = system.process_input(input).await.unwrap();
    
    // Verify memory formation
    assert!(result.memory_formed);
    
    // Verify concept extraction
    let concepts = system.get_concepts().await.unwrap();
    assert!(concepts.iter().any(|c| c.name.contains("cat")));
    assert!(concepts.iter().any(|c| c.name.contains("mat")));
    
    // Verify concept relationships
    let relationships = system.get_concept_relationships().await.unwrap();
    assert!(!relationships.is_empty());
}

#[tokio::test]
async fn test_learning_pipeline() {
    let mut system = BrainSystem::new().await.unwrap();
    
    // Process multiple related inputs
    let inputs = vec![
        "Cats are animals",
        "Dogs are animals", 
        "Animals need food",
        "Food provides energy"
    ];
    
    for input in inputs {
        system.process_input(input).await.unwrap();
    }
    
    // Verify learning occurred
    let insights = system.get_insights().await.unwrap();
    assert!(!insights.is_empty());
    
    // Verify concept graph structure
    let graph = system.get_concept_graph().await.unwrap();
    assert!(graph.node_count() > 4);
    assert!(graph.edge_count() > 0);
}
```

### API Integration Tests

```rust
// tests/api_tests.rs
use axum::http::StatusCode;
use axum_test::TestServer;
use brain_ai::create_app;

#[tokio::test]
async fn test_learn_endpoint() {
    let app = create_app().await;
    let server = TestServer::new(app).unwrap();
    
    // Test learning endpoint
    let response = server
        .post("/api/v1/learn")
        .json(&serde_json::json!({
            "content": "Test learning content",
            "priority": "high"
        }))
        .await;
    
    assert_eq!(response.status_code(), StatusCode::OK);
    
    let body: serde_json::Value = response.json();
    assert!(body["memory_formed"].as_bool().unwrap());
    assert!(body["memory_id"].as_str().is_some());
}

#[tokio::test]
async fn test_query_endpoint() {
    let app = create_app().await;
    let server = TestServer::new(app).unwrap();
    
    // First, add some content
    server
        .post("/api/v1/learn")
        .json(&serde_json::json!({
            "content": "Rust is a systems programming language"
        }))
        .await;
    
    // Then query for it
    let response = server
        .post("/api/v1/query")
        .json(&serde_json::json!({
            "query": "programming language",
            "limit": 10
        }))
        .await;
    
    assert_eq!(response.status_code(), StatusCode::OK);
    
    let body: serde_json::Value = response.json();
    assert!(!body["results"].as_array().unwrap().is_empty());
}
```

## System Testing

### End-to-End System Tests

```rust
// tests/system_integration_tests.rs
use brain_ai::*;
use std::time::Duration;
use tokio::time::timeout;

#[tokio::test]
async fn test_complete_learning_cycle() {
    let mut system = BrainSystem::new().await.unwrap();
    
    // Phase 1: Character-level learning
    let text = "The quick brown fox jumps over the lazy dog";
    for chunk in text.chars().collect::<Vec<_>>().chunks(5) {
        let chunk_str: String = chunk.iter().collect();
        system.process_input(&chunk_str).await.unwrap();
    }
    
    // Phase 2: Pattern discovery
    system.trigger_pattern_discovery().await.unwrap();
    
    // Phase 3: Concept formation
    system.trigger_concept_formation().await.unwrap();
    
    // Phase 4: Insight extraction
    let insights = system.extract_insights().await.unwrap();
    
    // Verify complete pipeline
    assert!(system.get_memory_count().await.unwrap() > 0);
    assert!(system.get_concept_count().await.unwrap() > 0);
    assert!(!insights.is_empty());
}

#[tokio::test]
async fn test_system_resilience() {
    let mut system = BrainSystem::new().await.unwrap();
    
    // Test with invalid inputs
    let invalid_inputs = vec![
        "",           // Empty string
        " ",          // Whitespace only
        "a".repeat(1000000), // Very long string
        "\0\0\0",     // Null bytes
        "🦀🦀🦀",    // Unicode
    ];
    
    for input in invalid_inputs {
        let result = system.process_input(&input).await;
        // Should handle gracefully without panicking
        assert!(result.is_ok() || result.is_err());
    }
    
    // System should still be functional
    let result = system.process_input("normal input").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_concurrent_operations() {
    let system = Arc::new(Mutex::new(BrainSystem::new().await.unwrap()));
    
    // Spawn multiple concurrent operations
    let handles: Vec<_> = (0..10).map(|i| {
        let system = Arc::clone(&system);
        tokio::spawn(async move {
            let input = format!("concurrent input {}", i);
            system.lock().await.process_input(&input).await
        })
    }).collect();
    
    // Wait for all operations with timeout
    let results = timeout(Duration::from_secs(30), async {
        let mut results = Vec::new();
        for handle in handles {
            results.push(handle.await.unwrap());
        }
        results
    }).await.unwrap();
    
    // Verify all operations completed successfully
    for result in results {
        assert!(result.is_ok());
    }
}
```

## Performance Testing

### Benchmarks

```rust
// benches/memory_benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use brain_ai::memory::MemorySystem;

fn memory_formation_benchmark(c: &mut Criterion) {
    let mut memory_system = MemorySystem::new(10000);
    
    c.bench_function("memory_formation", |b| {
        b.iter(|| {
            let content = format!("benchmark content {}", black_box(rand::random::<u32>()));
            memory_system.form_memory(black_box(&content), black_box(0.8))
        })
    });
}

fn memory_retrieval_benchmark(c: &mut Criterion) {
    let mut memory_system = MemorySystem::new(10000);
    
    // Pre-populate with memories
    let memory_ids: Vec<_> = (0..1000).map(|i| {
        memory_system.form_memory(&format!("content {}", i), 0.8).unwrap()
    }).collect();
    
    c.bench_function("memory_retrieval", |b| {
        b.iter(|| {
            let id = &memory_ids[black_box(rand::random::<usize>() % memory_ids.len())];
            memory_system.get_memory(black_box(id))
        })
    });
}

criterion_group!(benches, memory_formation_benchmark, memory_retrieval_benchmark);
criterion_main!(benches);
```

### Load Testing

```rust
// tests/load_tests.rs
use brain_ai::*;
use std::time::{Duration, Instant};
use tokio::time::timeout;

#[tokio::test]
async fn test_memory_system_load() {
    let mut system = BrainSystem::new().await.unwrap();
    let start = Instant::now();
    
    // Process 1000 memories
    for i in 0..1000 {
        let content = format!("load test content {}", i);
        system.process_input(&content).await.unwrap();
    }
    
    let duration = start.elapsed();
    println!("Processed 1000 memories in {:?}", duration);
    
    // Verify performance requirements
    assert!(duration < Duration::from_secs(60)); // Should complete within 1 minute
    assert!(system.get_memory_count().await.unwrap() == 1000);
}

#[tokio::test]
async fn test_concurrent_load() {
    let system = Arc::new(Mutex::new(BrainSystem::new().await.unwrap()));
    let start = Instant::now();
    
    // Spawn 100 concurrent tasks, each processing 10 memories
    let handles: Vec<_> = (0..100).map(|task_id| {
        let system = Arc::clone(&system);
        tokio::spawn(async move {
            for i in 0..10 {
                let content = format!("concurrent load test {} {}", task_id, i);
                system.lock().await.process_input(&content).await.unwrap();
            }
        })
    }).collect();
    
    // Wait for all tasks
    for handle in handles {
        handle.await.unwrap();
    }
    
    let duration = start.elapsed();
    println!("Processed 1000 memories concurrently in {:?}", duration);
    
    // Verify all memories were processed
    assert!(system.lock().await.get_memory_count().await.unwrap() == 1000);
}
```

## Property Testing

```rust
// tests/property_tests.rs
use proptest::prelude::*;
use brain_ai::memory::MemorySystem;

proptest! {
    #[test]
    fn test_memory_formation_properties(
        content in "\\PC*",  // Any string
        confidence in 0.0f64..1.0f64
    ) {
        let mut memory_system = MemorySystem::new(1000);
        
        if !content.is_empty() {
            let result = memory_system.form_memory(&content, confidence);
            prop_assert!(result.is_ok());
            
            let memory_id = result.unwrap();
            let retrieved = memory_system.get_memory(&memory_id);
            prop_assert!(retrieved.is_some());
            prop_assert_eq!(retrieved.unwrap().content, content);
        }
    }
    
    #[test]
    fn test_concept_extraction_properties(
        words in prop::collection::vec("[a-zA-Z]+", 1..20)
    ) {
        let content = words.join(" ");
        let mut system = futures::executor::block_on(BrainSystem::new()).unwrap();
        
        let result = futures::executor::block_on(system.process_input(&content));
        prop_assert!(result.is_ok());
        
        // Properties that should always hold
        let concepts = futures::executor::block_on(system.get_concepts()).unwrap();
        prop_assert!(concepts.len() <= words.len()); // Can't have more concepts than words
    }
}
```

## Test Data Management

### Fixtures

```rust
// tests/fixtures/mod.rs
use std::path::Path;

pub struct TestFixtures;

impl TestFixtures {
    pub fn load_sample_text() -> String {
        std::fs::read_to_string("tests/fixtures/sample_text.txt")
            .expect("Failed to load sample text")
    }
    
    pub fn load_concept_data() -> Vec<String> {
        std::fs::read_to_string("tests/fixtures/concepts.json")
            .map(|s| serde_json::from_str(&s).expect("Invalid JSON"))
            .expect("Failed to load concept data")
    }
    
    pub fn create_temp_db() -> tempfile::TempDir {
        tempfile::tempdir().expect("Failed to create temp directory")
    }
}
```

### Mock Services

```rust
// tests/mocks/mod.rs
use brain_ai::*;
use mockall::mock;

mock! {
    ExternalService {}
    
    #[async_trait]
    impl ExternalServiceTrait for ExternalService {
        async fn fetch_data(&self, query: &str) -> Result<String, BrainError>;
        async fn validate_content(&self, content: &str) -> Result<bool, BrainError>;
    }
}

// Usage in tests
#[tokio::test]
async fn test_with_mock_service() {
    let mut mock_service = MockExternalService::new();
    mock_service
        .expect_fetch_data()
        .with(eq("test query"))
        .times(1)
        .returning(|_| Ok("mock response".to_string()));
    
    // Use mock in test
    let result = mock_service.fetch_data("test query").await;
    assert_eq!(result.unwrap(), "mock response");
}
```

## Continuous Integration

### GitHub Actions Workflow

```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        components: rustfmt, clippy
    
    - name: Cache dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Run tests
      run: |
        cargo test --all-features
        cargo test --release --all-features
    
    - name: Run clippy
      run: cargo clippy --all-features -- -D warnings
    
    - name: Check formatting
      run: cargo fmt -- --check
    
    - name: Generate coverage
      run: |
        cargo install cargo-tarpaulin
        cargo tarpaulin --out xml
    
    - name: Upload coverage
      uses: codecov/codecov-action@v3
```

## Testing Best Practices

### Test Organization

1. **Arrange-Act-Assert**: Structure tests clearly
2. **Single Responsibility**: One test per behavior
3. **Descriptive Names**: Test names should describe the scenario
4. **Independent Tests**: Tests should not depend on each other

### Performance Considerations

```rust
// Use lazy_static for expensive setup
use lazy_static::lazy_static;

lazy_static! {
    static ref TEST_SYSTEM: Mutex<BrainSystem> = {
        Mutex::new(futures::executor::block_on(BrainSystem::new()).unwrap())
    };
}

#[tokio::test]
async fn test_with_shared_system() {
    let system = TEST_SYSTEM.lock().await;
    // Use shared system for faster tests
}
```

### Error Testing

```rust
#[tokio::test]
async fn test_error_conditions() {
    let mut system = BrainSystem::new().await.unwrap();
    
    // Test various error conditions
    let result = system.process_input("").await;
    assert!(matches!(result, Err(BrainError::InvalidInput(_))));
    
    let result = system.get_memory(&"invalid_id".to_string()).await;
    assert!(matches!(result, Err(BrainError::MemoryNotFound(_))));
}
```

This comprehensive testing strategy ensures Brain AI maintains high quality and reliability across all components and use cases.
