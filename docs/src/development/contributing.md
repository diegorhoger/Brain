# Contributing to Brain AI

Welcome to Brain AI! We're excited that you're interested in contributing to this post-transformer developmental AI architecture. This guide will help you get started with development, testing, and contributing to the project.

## Development Philosophy

Brain AI follows these core development principles:

- **Developmental Learning**: Code should reflect the gradual learning approach
- **Modularity**: Components should be loosely coupled and highly cohesive
- **Performance**: Optimize for learning efficiency and real-time responsiveness
- **Maintainability**: Write clear, documented, and testable code
- **Safety**: Memory safety and error handling are paramount

## Getting Started

### Development Environment Setup

#### Prerequisites

- **Rust 1.70+** with `rustfmt` and `clippy`
- **Git** with proper configuration
- **IDE**: VS Code with Rust Analyzer (recommended) or similar
- **Docker** for integration testing

#### Initial Setup

```bash
# Clone the repository
git clone https://github.com/your-org/brain-ai.git
cd brain-ai

# Install development dependencies
rustup component add rustfmt clippy
cargo install cargo-watch cargo-audit cargo-tarpaulin

# Set up pre-commit hooks
cp scripts/pre-commit .git/hooks/
chmod +x .git/hooks/pre-commit

# Build in development mode
cargo build

# Run tests to verify setup
cargo test
```

#### IDE Configuration

**VS Code (recommended):**

Create `.vscode/settings.json`:
```json
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.cargo.features": "all",
    "rust-analyzer.procMacro.enable": true,
    "editor.formatOnSave": true,
    "files.exclude": {
        "**/target": true,
        "**/.DS_Store": true
    }
}
```

Create `.vscode/extensions.json`:
```json
{
    "recommendations": [
        "rust-lang.rust-analyzer",
        "vadimcn.vscode-lldb",
        "serayuzgur.crates",
        "tamasfe.even-better-toml"
    ]
}
```

### Project Structure

```
brain-ai/
├── src/                    # Core Rust source code
│   ├── character_ingestion/    # Character-level learning
│   ├── segment_discovery/      # Pattern discovery
│   ├── memory/                 # Memory systems
│   ├── concept_graph/          # Knowledge representation
│   ├── simulation/             # Scenario modeling
│   ├── insight_extraction/     # Rule learning
│   ├── system_integration/     # System coordination
│   └── lib.rs                 # Library entry point
├── examples/               # Usage examples
├── tests/                  # Integration tests
├── web/                    # Web dashboard
├── python/                 # Python bindings
├── docs/                   # Documentation source
├── scripts/                # Utility scripts
├── deployment/             # Deployment configurations
└── data/                   # Runtime data
```

## Development Workflow

### 1. Feature Development

#### Branch Naming Convention

```bash
# Feature branches
git checkout -b feature/character-prediction-improvements

# Bug fixes
git checkout -b fix/memory-leak-in-segment-discovery

# Documentation
git checkout -b docs/api-reference-updates

# Refactoring
git checkout -b refactor/concept-graph-optimization
```

#### Development Process

```bash
# 1. Create feature branch
git checkout -b feature/new-learning-algorithm

# 2. Implement changes with tests
cargo watch -x test -x clippy

# 3. Format code
cargo fmt

# 4. Run comprehensive tests
cargo test --all-features
cargo clippy -- -D warnings
cargo audit

# 5. Commit changes
git add .
git commit -m "feat: implement adaptive learning rate algorithm

- Add dynamic learning rate adjustment based on convergence
- Implement convergence detection using loss variance
- Add comprehensive tests for learning rate adaptation
- Update documentation with new algorithm details

Closes #123"
```

### 2. Code Standards

#### Rust Code Style

Follow the official Rust style guide with these additions:

```rust
// ✅ Good: Clear, documented function
/// Processes character sequences to build vocabulary.
///
/// # Arguments
/// * `text` - Input text to process
/// * `config` - Character ingestion configuration
///
/// # Returns
/// Result containing vocabulary statistics or error
///
/// # Examples
/// ```
/// let config = CharacterConfig::default();
/// let result = process_characters("hello world", &config)?;
/// assert!(result.vocab_size > 0);
/// ```
pub fn process_characters(
    text: &str,
    config: &CharacterConfig,
) -> Result<VocabStats, BrainError> {
    // Implementation
}

// ❌ Bad: No documentation, unclear naming
pub fn proc_chars(txt: &str, cfg: &CharacterConfig) -> Result<VocabStats, BrainError> {
    // Implementation
}
```

#### Error Handling

```rust
// ✅ Good: Structured error handling
#[derive(Debug, thiserror::Error)]
pub enum BrainError {
    #[error("Character ingestion failed: {reason}")]
    CharacterIngestionError { reason: String },
    
    #[error("Memory operation failed: {operation}")]
    MemoryError { operation: String },
    
    #[error("Configuration error: {0}")]
    ConfigError(#[from] config::ConfigError),
}

// ✅ Good: Proper error propagation
pub fn learn_from_text(text: &str) -> Result<LearningResult, BrainError> {
    let segments = discover_segments(text)
        .map_err(|e| BrainError::CharacterIngestionError { 
            reason: format!("Segmentation failed: {}", e) 
        })?;
    
    store_in_memory(&segments)?;
    Ok(LearningResult::new(segments.len()))
}
```

#### Testing Standards

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn test_character_prediction_accuracy() {
        // ✅ Good: Descriptive test name and clear setup
        let config = CharacterConfig::test_config();
        let predictor = CharacterPredictor::new(config);
        
        let training_text = "the quick brown fox";
        predictor.train(training_text).unwrap();
        
        let prediction = predictor.predict("the quick brown").unwrap();
        assert!(prediction.confidence > 0.8);
        assert_eq!(prediction.character, ' ');
    }

    #[tokio::test]
    async fn test_memory_system_integration() {
        // ✅ Good: Integration test with proper async handling
        let memory_system = MemorySystem::new_test().await;
        
        let info = "cats are mammals";
        memory_system.store_episodic(info).await.unwrap();
        
        let results = memory_system.query("mammals").await.unwrap();
        assert!(!results.is_empty());
        assert!(results[0].content.contains("cats"));
    }
}
```

### 3. Testing Strategy

#### Test Categories

1. **Unit Tests**: Test individual functions and methods
2. **Integration Tests**: Test component interactions
3. **System Tests**: Test complete workflows
4. **Performance Tests**: Benchmark critical paths
5. **Property Tests**: Test invariants with random inputs

#### Running Tests

```bash
# Run all tests
cargo test

# Run specific test module
cargo test character_ingestion

# Run tests with output
cargo test -- --nocapture

# Run tests in parallel
cargo test -- --test-threads=4

# Run performance benchmarks
cargo test --release --features=bench bench_

# Generate coverage report
cargo tarpaulin --out html
```

#### Test Utilities

Create reusable test utilities in `src/test_utils.rs`:

```rust
pub struct TestMemorySystem {
    memory: MemorySystem,
}

impl TestMemorySystem {
    pub async fn new() -> Self {
        let config = MemoryConfig::test_config();
        let memory = MemorySystem::new(config).await.unwrap();
        Self { memory }
    }
    
    pub async fn with_test_data(data: &[&str]) -> Self {
        let system = Self::new().await;
        for item in data {
            system.memory.store_episodic(item).await.unwrap();
        }
        system
    }
}

pub fn sample_training_text() -> &'static str {
    "The quick brown fox jumps over the lazy dog. \
     Programming is the art of telling another human \
     what one wants the computer to do."
}
```

### 4. Documentation Standards

#### Code Documentation

```rust
//! # Character Ingestion Module
//!
//! This module implements character-level learning that forms the foundation
//! of Brain AI's developmental approach. It processes text character by character,
//! building vocabulary and prediction capabilities gradually.
//!
//! ## Architecture
//!
//! The character ingestion system consists of:
//! - Character tokenizer for preprocessing
//! - GRU-based prediction network
//! - Dynamic vocabulary builder
//! - Confidence scoring system
//!
//! ## Examples
//!
//! ```rust
//! use brain_ai::character_ingestion::*;
//!
//! let config = CharacterConfig::default();
//! let mut engine = CharacterIngestionEngine::new(config);
//!
//! // Train on text
//! engine.learn("Hello, world!")?;
//!
//! // Make predictions
//! let prediction = engine.predict("Hello, wor")?;
//! println!("Next character: {}", prediction.character);
//! ```

/// Represents a character prediction with confidence score.
///
/// This struct encapsulates the result of character-level prediction,
/// including the predicted character and the model's confidence in
/// that prediction.
#[derive(Debug, Clone, PartialEq)]
pub struct CharacterPrediction {
    /// The predicted character
    pub character: char,
    /// Confidence score between 0.0 and 1.0
    pub confidence: f32,
    /// Alternative predictions with lower confidence
    pub alternatives: Vec<(char, f32)>,
}
```

#### API Documentation

Document all public APIs with examples:

```rust
impl CharacterIngestionEngine {
    /// Creates a new character ingestion engine with the given configuration.
    ///
    /// # Arguments
    /// * `config` - Configuration parameters for the engine
    ///
    /// # Examples
    /// ```
    /// let config = CharacterConfig {
    ///     vocab_size: 1000,
    ///     sequence_length: 64,
    ///     learning_rate: 0.001,
    /// };
    /// let engine = CharacterIngestionEngine::new(config);
    /// ```
    pub fn new(config: CharacterConfig) -> Self {
        // Implementation
    }
}
```

### 5. Performance Guidelines

#### Benchmarking

```rust
#[cfg(feature = "bench")]
mod benches {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_character_prediction(b: &mut Bencher) {
        let engine = CharacterIngestionEngine::new(CharacterConfig::default());
        let text = "the quick brown fox";
        
        b.iter(|| {
            engine.predict(text).unwrap()
        });
    }
}
```

#### Memory Management

```rust
// ✅ Good: Efficient memory usage
pub struct MemoryEfficientProcessor {
    buffer: Vec<u8>,
}

impl MemoryEfficientProcessor {
    pub fn process_stream<R: Read>(&mut self, reader: R) -> Result<(), BrainError> {
        self.buffer.clear(); // Reuse existing allocation
        
        for chunk in reader.bytes().chunks(4096) {
            self.buffer.extend(chunk);
            self.process_chunk(&self.buffer)?;
            self.buffer.clear();
        }
        Ok(())
    }
}
```

## Contribution Process

### 1. Issue Reporting

Before contributing, check if an issue already exists:

```bash
# Search existing issues
gh issue list --search "memory leak"

# Create new issue
gh issue create --title "Memory leak in segment discovery" \
                --body "Description of the issue..."
```