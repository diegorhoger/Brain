# Quick Start Guide

Get up and running with Brain AI in under 10 minutes! This guide will walk you through the fastest path to experiencing Brain AI's cognitive capabilities.

## Prerequisites

Before you begin, ensure you have:

- **Rust 1.70+** installed ([rustup.rs](https://rustup.rs/))
- **Python 3.8+** (optional, for Python bindings)
- **Docker & Docker Compose** (recommended for easy deployment)
- **Git** for cloning the repository

## Option 1: Docker Deployment (Recommended)

The fastest way to get Brain AI running is with Docker:

### Step 1: Clone and Configure

```bash
# Clone the repository
git clone <repository-url>
cd brain

# Copy environment template
cp env.example .env
```

### Step 2: Configure Environment

Edit `.env` with your preferred settings:

```bash
# Basic configuration
BRAIN_PORT=8080
BRAIN_HOST=0.0.0.0

# Database settings (optional - defaults work fine)
NEO4J_URI=bolt://neo4j:7687
NEO4J_USER=neo4j
NEO4J_PASSWORD=password

# Performance settings
BRAIN_PERFORMANCE_MONITORING=true
BRAIN_LOG_LEVEL=info
```

### Step 3: Deploy

```bash
cd deployment/
docker-compose up -d
```

### Step 4: Verify Installation

```bash
# Check if Brain AI is running
curl http://localhost:8080/health

# Expected response:
# {"status":"healthy","timestamp":"2024-01-01T12:00:00Z"}
```

🎉 **Congratulations!** Brain AI is now running at `http://localhost:8080`

## Option 2: Native Development Setup

For development or if you prefer running natively:

### Step 1: Clone and Build

```bash
# Clone the repository
git clone <repository-url>
cd brain

# Build the Rust components
cargo build --release

# Run comprehensive tests
cargo test
```

### Step 2: Install Python Bindings (Optional)

```bash
# Install maturin for Python bindings
pip install maturin

# Build and install Brain Python module
maturin develop --features python
```

### Step 3: Run Brain AI

```bash
# Start the server
./target/release/brain-server

# Or with custom configuration
BRAIN_CONFIG=scripts/config.toml ./target/release/brain-server
```

## First Steps: Basic Usage

Now that Brain AI is running, let's explore its capabilities:

### 1. Character-Level Learning

```python
from brain import BrainEngine

# Initialize the cognitive system
engine = BrainEngine()

# Start with basic character prediction
text = "The quick brown fox"
predictions = engine.predict_next_chars(text, num_predictions=5)
print(f"Next character predictions: {predictions}")
```

### 2. Pattern Discovery

```python
# Discover meaningful segments in text
text = "The cats are running quickly"
segments = engine.segment(text)
print(f"Discovered segments: {segments}")

# Expected output might be:
# ["The", "cat", "s", "are", "run", "ning", "quick", "ly"]
```

### 3. Memory and Learning

```python
# Teach Brain AI new information
engine.learn("Cats are domestic animals that meow", priority="high")
engine.learn("Dogs are domestic animals that bark", priority="high")

# Query what it learned
memories = engine.query_memory("domestic animals", limit=5)
for memory in memories:
    print(f"Memory: {memory.content} (confidence: {memory.confidence})")
```

### 4. Concept Relationships

```python
# Explore concept relationships
concepts = engine.get_related_concepts("cat", max_depth=2)
print(f"Concepts related to 'cat': {concepts}")

# Expected relationships might include:
# cat -> animal -> pet -> companionship
```

### 5. Simulation and Prediction

```python
# Simulate scenarios
result = engine.simulate(
    scenario="What happens if a cat meets a dog?",
    max_steps=3,
    confidence_threshold=0.3
)

print(f"Simulation outcome: {result.outcome}")
print(f"Confidence: {result.confidence}")
print(f"Steps taken: {len(result.steps)}")
```

## REST API Usage

Brain AI also provides a comprehensive REST API:

### Authentication

```bash
# Get an API token (if authentication is enabled)
curl -X POST http://localhost:8080/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "password"}'
```

### Basic API Calls

```bash
# Learn new information
curl -X POST http://localhost:8080/api/learn \
  -H "Content-Type: application/json" \
  -d '{"text": "Python is a programming language", "priority": "high"}'

# Segment text
curl -X POST http://localhost:8080/api/segment \
  -H "Content-Type: application/json" \
  -d '{"text": "The quick brown fox jumps"}'

# Query memories
curl -X GET "http://localhost:8080/api/memory/search?query=programming&limit=5"

# Simulate scenarios
curl -X POST http://localhost:8080/api/simulate \
  -H "Content-Type: application/json" \
  -d '{"scenario": "Learning a new programming language", "max_steps": 5}'
```

## Web Dashboard

Brain AI includes interactive visualizations accessible at:

- **Main Dashboard**: `http://localhost:8080/dashboard`
- **Concept Graph**: `http://localhost:8080/concepts`
- **Memory Timeline**: `http://localhost:8080/memory`
- **Simulation Explorer**: `http://localhost:8080/simulations`

## Example Workflows

### Workflow 1: Text Analysis

```python
# Analyze a document
document = """
Machine learning is a subset of artificial intelligence that enables 
computers to learn and improve from experience without being explicitly 
programmed. It focuses on developing algorithms that can access data 
and use it to learn for themselves.
"""

# 1. Learn from the document
engine.learn(document, priority="medium")

# 2. Discover key segments
segments = engine.segment(document)

# 3. Extract insights
insights = engine.extract_insights(document)

# 4. Explore related concepts
concepts = engine.get_related_concepts("machine learning")

print(f"Key segments: {segments[:10]}")  # First 10 segments
print(f"Insights: {insights}")
print(f"Related concepts: {concepts}")
```

### Workflow 2: Knowledge Building

```python
# Build a knowledge base about programming languages
languages = [
    "Python is a high-level programming language known for simplicity",
    "JavaScript is used for web development and runs in browsers",
    "Rust is a systems programming language focused on safety and performance",
    "Go is designed for concurrent programming and cloud services"
]

# Learn each fact
for fact in languages:
    engine.learn(fact, priority="high")

# Query the knowledge base
results = engine.query_memory("programming language", limit=10)
for result in results:
    print(f"Knowledge: {result.content}")

# Explore concept relationships
programming_concepts = engine.get_related_concepts("programming", max_depth=3)
print(f"Programming-related concepts: {programming_concepts}")
```

## Performance Monitoring

Brain AI includes built-in performance monitoring:

```python
# Get system performance metrics
metrics = engine.get_performance_metrics()
print(f"Memory usage: {metrics.memory_usage_mb} MB")
print(f"Processing speed: {metrics.operations_per_second} ops/sec")
print(f"Active concepts: {metrics.active_concepts}")

# Identify bottlenecks
bottlenecks = engine.identify_bottlenecks()
for bottleneck in bottlenecks:
    print(f"Bottleneck: {bottleneck.component} - {bottleneck.description}")
```

## Next Steps

Now that you have Brain AI running, explore these areas:

1. **[System Architecture](../architecture/system-architecture.md)** - Understand how Brain AI works internally
2. **[Core Components](../components/character-ingestion.md)** - Deep dive into each cognitive component
3. **[API Reference](../api/overview.md)** - Complete API documentation
4. **[Python Bindings](../python/overview.md)** - Advanced Python usage patterns
5. **[Examples](../examples/basic-examples.md)** - More comprehensive examples and use cases

## Troubleshooting

### Common Issues

**Docker containers won't start:**
```bash
# Check Docker status
docker-compose ps

# View logs
docker-compose logs brain-ai
```

**Python bindings import error:**
```bash
# Reinstall bindings
pip uninstall brain
maturin develop --features python --release
```

**Performance issues:**
```bash
# Enable performance monitoring
export BRAIN_PERFORMANCE_MONITORING=true
./target/release/brain-server
```

**Port already in use:**
```bash
# Change port in .env file
echo "BRAIN_PORT=8081" >> .env
docker-compose up -d
```

For more troubleshooting help, see the [Troubleshooting Guide](../deployment/troubleshooting.md).

---

**🚀 You're ready to explore Brain AI!** The system is now learning and growing with each interaction. Try the examples above and watch as it develops understanding over time. 