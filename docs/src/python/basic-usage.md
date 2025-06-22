# Basic Usage

This guide covers the fundamental operations and common usage patterns for the Brain AI Python API. Whether you're building a simple application or integrating AI capabilities into an existing system, these examples will get you started quickly.

## Installation and Setup

### Prerequisites

```bash
# Python 3.8 or higher required
python --version

# Install Brain AI
pip install brain-ai

# Optional: Install additional dependencies for enhanced features
pip install pandas numpy matplotlib jupyter
```

### Initial Setup

```python
from brain_ai import BrainAI, BrainConfig
import logging

# Configure logging (optional)
logging.basicConfig(level=logging.INFO)

# Create configuration
config = BrainConfig(
    memory_capacity=10000,
    enable_performance_monitoring=True,
    log_level="INFO"
)

# Initialize Brain AI
brain = BrainAI(config)
print("Brain AI initialized successfully!")
```

## Core Operations

### 1. Learning from Text

The most fundamental operation is teaching Brain AI new information:

```python
# Basic learning
result = brain.learn("Python is a powerful programming language used for data science")

print(f"Learning successful: {result.success}")
print(f"Segments discovered: {result.segments_discovered}")
print(f"Concepts created: {result.concepts_created}")
print(f"Processing time: {result.processing_time_ms}ms")

# Learning with priority
high_priority_result = brain.learn(
    "Machine learning algorithms require large datasets for training",
    priority="high"
)

# Learning with context
contextual_result = brain.learn(
    "Neural networks mimic the human brain's structure",
    priority="medium",
    context={
        "domain": "artificial_intelligence",
        "source": "educational_content",
        "author": "expert"
    }
)
```

### 2. Memory Search and Retrieval

Search through learned information:

```python
# Basic memory search
memories = brain.search_memory("machine learning", limit=5)

print(f"Found {len(memories)} memories about machine learning:")
for memory in memories:
    print(f"- {memory.content}")
    print(f"  Confidence: {memory.confidence:.2f}")
    print(f"  Type: {memory.memory_type}")
    print()

# Advanced memory search with filters
filtered_memories = brain.search_memory(
    query="programming",
    memory_type="semantic",
    min_confidence=0.8,
    limit=10
)

# Search by specific criteria
recent_memories = brain.search_memory(
    query="python",
    sort_by="recency",
    limit=3
)
```

### 3. Concept Graph Exploration

Discover relationships between concepts:

```python
# Find related concepts
related_concepts = brain.get_related_concepts("python", depth=2)

print("Concepts related to 'python':")
for concept in related_concepts:
    print(f"- {concept.name}")
    print(f"  Relationship: {concept.relationship_type}")
    print(f"  Strength: {concept.relationship_strength:.2f}")
    print(f"  Distance: {concept.distance}")
    print()

# Explore concept relationships in depth
deep_concepts = brain.get_related_concepts(
    concept="artificial_intelligence",
    depth=3,
    min_strength=0.5,
    max_results=15
)

# Get concept details
concept_details = brain.get_concept_details("machine_learning")
print(f"Concept: {concept_details.name}")
print(f"Definition: {concept_details.definition}")
print(f"Importance: {concept_details.importance_score}")
```

### 4. Insight Generation

Extract insights from learned information:

```python
# Generate insights about a topic
insights = brain.generate_insights("trends in machine learning")

print("Generated insights:")
for insight in insights:
    print(f"- {insight.title}")
    print(f"  Type: {insight.insight_type}")
    print(f"  Confidence: {insight.confidence:.2f}")
    print(f"  Description: {insight.description}")
    print(f"  Evidence: {', '.join(insight.evidence[:2])}")
    print()

# Generate insights with specific parameters
focused_insights = brain.generate_insights(
    query="python programming best practices",
    insight_types=["explanatory", "predictive"],
    min_confidence=0.7,
    max_insights=5
)
```

## Working with Text Collections

### Processing Multiple Texts

```python
# Learning from multiple texts
texts = [
    "Python supports multiple programming paradigms",
    "Object-oriented programming is a key feature of Python",
    "Python's syntax emphasizes code readability",
    "The Python community is large and supportive",
    "Python is widely used in web development and data science"
]

# Sequential learning
results = []
for text in texts:
    result = brain.learn(text)
    results.append(result)
    print(f"Learned: {text[:50]}... ({result.segments_discovered} segments)")

# Batch learning (more efficient for large datasets)
batch_results = brain.learn_batch(texts, parallel=True)
print(f"Batch learning completed: {len(batch_results)} texts processed")
```

### Learning from Files

```python
# Learn from text file
def learn_from_file(brain: BrainAI, filepath: str):
    with open(filepath, 'r', encoding='utf-8') as file:
        content = file.read()
    
    # Split into chunks for better processing
    chunks = content.split('\n\n')  # Split by paragraphs
    
    results = []
    for chunk in chunks:
        if chunk.strip():  # Skip empty chunks
            result = brain.learn(chunk.strip())
            results.append(result)
    
    return results

# Usage
results = learn_from_file(brain, "knowledge_base.txt")
print(f"Learned from file: {len(results)} chunks processed")
```

### Learning from Structured Data

```python
import pandas as pd

# Learn from CSV data
def learn_from_csv(brain: BrainAI, csv_path: str, text_column: str):
    df = pd.read_csv(csv_path)
    results = []
    
    for _, row in df.iterrows():
        text = str(row[text_column])
        if pd.notna(text) and text.strip():
            # Add context from other columns
            context = {col: str(row[col]) for col in df.columns if col != text_column}
            result = brain.learn(text, context=context)
            results.append(result)
    
    return results

# Learn from JSON data
import json

def learn_from_json(brain: BrainAI, json_path: str, text_field: str):
    with open(json_path, 'r') as file:
        data = json.load(file)
    
    results = []
    for item in data:
        if text_field in item:
            text = item[text_field]
            # Use other fields as context
            context = {k: v for k, v in item.items() if k != text_field}
            result = brain.learn(text, context=context)
            results.append(result)
    
    return results
```

## Memory Management and Optimization

### Memory Usage Monitoring

```python
# Check memory usage
memory_stats = brain.get_memory_statistics()
print(f"Total memories: {memory_stats.total_memories}")
print(f"Memory usage: {memory_stats.usage_percentage:.1f}%")
print(f"Available capacity: {memory_stats.available_capacity}")

# Performance metrics
perf_metrics = brain.get_performance_metrics()
print(f"Average learning time: {perf_metrics.avg_learning_time_ms}ms")
print(f"Average search time: {perf_metrics.avg_search_time_ms}ms")
print(f"Total operations: {perf_metrics.total_operations}")
```

### Memory Consolidation

```python
# Trigger memory consolidation
consolidation_result = brain.consolidate_memories()
print(f"Consolidated {consolidation_result.memories_consolidated} memories")
print(f"Storage saved: {consolidation_result.storage_saved_bytes} bytes")

# Automatic consolidation settings
brain.configure_auto_consolidation(
    threshold=0.8,  # Consolidate when 80% full
    frequency="daily",
    keep_recent_days=7
)
```

### Memory Cleanup

```python
# Remove low-importance memories
cleanup_result = brain.cleanup_memories(
    min_importance=0.3,
    max_age_days=30,
    keep_accessed_recently=True
)
print(f"Cleaned up {cleanup_result.memories_removed} memories")

# Clear specific types of memories
brain.clear_memories(memory_type="temporary")
```

## Error Handling and Debugging

### Comprehensive Error Handling

```python
from brain_ai import BrainError, ConfigurationError, MemoryError, LearningError

def safe_learning(brain: BrainAI, text: str):
    try:
        result = brain.learn(text)
        return result
    except LearningError as e:
        print(f"Learning failed: {e}")
        print(f"Error details: {e.details}")
        return None
    except MemoryError as e:
        print(f"Memory system error: {e}")
        # Maybe try consolidation
        brain.consolidate_memories()
        return None
    except BrainError as e:
        print(f"General Brain AI error: {e}")
        return None
    except Exception as e:
        print(f"Unexpected error: {e}")
        return None

# Usage with error handling
texts = ["Valid text", "", "Another valid text", None]
for text in texts:
    if text:
        result = safe_learning(brain, text)
        if result:
            print(f"Successfully learned: {text[:30]}...")
```

### Debugging and Logging

```python
import logging

# Enable detailed logging
logging.basicConfig(level=logging.DEBUG)
brain_logger = logging.getLogger('brain_ai')

# Custom logging handler
class BrainAIHandler(logging.Handler):
    def emit(self, record):
        if record.levelno >= logging.WARNING:
            print(f"⚠️  Brain AI Warning: {record.getMessage()}")

brain_logger.addHandler(BrainAIHandler())

# Debug learning process
result = brain.learn("Debug this learning process", debug=True)
print(f"Debug info: {result.debug_info}")
```

## Configuration and Customization

### Dynamic Configuration

```python
# Update configuration at runtime
brain.update_config({
    'insight_confidence_threshold': 0.8,
    'concept_graph_max_depth': 4,
    'enable_performance_monitoring': True
})

# Get current configuration
current_config = brain.get_config()
print(f"Current memory capacity: {current_config.memory_capacity}")
print(f"Current log level: {current_config.log_level}")
```

### Custom Processing Options

```python
# Learning with custom options
custom_result = brain.learn(
    "Custom processing example",
    options={
        'enable_concept_discovery': True,
        'enable_relationship_inference': True,
        'consolidation_threshold': 0.9,
        'segment_validation': True,
        'parallel_processing': False
    }
)

# Search with custom options
custom_search = brain.search_memory(
    query="custom search",
    options={
        'fuzzy_matching': True,
        'include_context': True,
        'boost_recent': True,
        'semantic_similarity_threshold': 0.7
    }
)
```

## Integration Patterns

### Context Managers

```python
from contextlib import contextmanager

@contextmanager
def brain_session(config):
    brain = BrainAI(config)
    try:
        yield brain
    finally:
        # Cleanup operations
        brain.consolidate_memories()
        brain.save_state()

# Usage
with brain_session(config) as brain:
    brain.learn("This will be automatically cleaned up")
    memories = brain.search_memory("cleanup")
```

### Decorator Pattern

```python
def with_brain_ai(func):
    def wrapper(*args, **kwargs):
        brain = BrainAI(BrainConfig())
        try:
            return func(brain, *args, **kwargs)
        finally:
            brain.cleanup()
    return wrapper

@with_brain_ai
def process_text(brain, text):
    result = brain.learn(text)
    insights = brain.generate_insights(text)
    return result, insights

# Usage
result, insights = process_text("Text to process")
```

### Class-based Integration

```python
class IntelligentTextProcessor:
    def __init__(self, config=None):
        self.config = config or BrainConfig()
        self.brain = BrainAI(self.config)
        self.processing_stats = {
            'texts_processed': 0,
            'insights_generated': 0,
            'concepts_discovered': 0
        }
    
    def process_text(self, text, generate_insights=True):
        # Learn from text
        learn_result = self.brain.learn(text)
        self.processing_stats['texts_processed'] += 1
        self.processing_stats['concepts_discovered'] += learn_result.concepts_created
        
        result = {
            'learning_result': learn_result,
            'related_concepts': self.brain.get_related_concepts(text[:20], depth=1)
        }
        
        # Generate insights if requested
        if generate_insights:
            insights = self.brain.generate_insights(text)
            result['insights'] = insights
            self.processing_stats['insights_generated'] += len(insights)
        
        return result
    
    def get_statistics(self):
        return self.processing_stats.copy()
    
    def search_knowledge(self, query):
        return self.brain.search_memory(query)

# Usage
processor = IntelligentTextProcessor()
result = processor.process_text("Machine learning is transforming industries")
print(f"Processing complete: {result['learning_result'].success}")
print(f"Statistics: {processor.get_statistics()}")
```

## Performance Optimization

### Batch Processing

```python
def optimized_batch_learning(brain: BrainAI, texts: list, batch_size: int = 50):
    """Optimized batch processing with memory management"""
    total_results = []
    
    for i in range(0, len(texts), batch_size):
        batch = texts[i:i + batch_size]
        
        # Process batch
        batch_results = brain.learn_batch(batch, parallel=True)
        total_results.extend(batch_results)
        
        # Memory management
        if i % (batch_size * 10) == 0:  # Every 10 batches
            brain.consolidate_memories()
        
        print(f"Processed batch {i//batch_size + 1}/{(len(texts) + batch_size - 1)//batch_size}")
    
    return total_results
```

### Caching and Memoization

```python
from functools import lru_cache

class CachedBrainAI:
    def __init__(self, config):
        self.brain = BrainAI(config)
    
    @lru_cache(maxsize=1000)
    def cached_search(self, query: str, limit: int = 10):
        """Cache search results for repeated queries"""
        return tuple(self.brain.search_memory(query, limit=limit))
    
    @lru_cache(maxsize=500)
    def cached_concepts(self, concept: str, depth: int = 2):
        """Cache concept relationships"""
        return tuple(self.brain.get_related_concepts(concept, depth=depth))
    
    def learn(self, text: str):
        """Learning invalidates relevant caches"""
        result = self.brain.learn(text)
        # Clear caches that might be affected
        self.cached_search.cache_clear()
        self.cached_concepts.cache_clear()
        return result

# Usage
cached_brain = CachedBrainAI(config)
```

This comprehensive guide covers the essential patterns and practices for using Brain AI's Python API effectively. The examples demonstrate both basic usage and advanced integration patterns suitable for production applications.
