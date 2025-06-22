# Python API Overview

Brain AI provides comprehensive Python bindings that allow developers to integrate cognitive AI capabilities directly into Python applications. The Python API offers both synchronous and asynchronous interfaces, comprehensive type hints, and seamless integration with popular Python data science libraries.

## Features

The Python API provides access to all Brain AI capabilities:

- **Character-level Processing**: Advanced text ingestion and character prediction
- **Segment Discovery**: Intelligent text segmentation using adaptive algorithms
- **Memory System**: Multi-layered memory storage and retrieval
- **Concept Graph**: Dynamic knowledge representation and relationship discovery
- **Insight Extraction**: Pattern analysis and insight generation
- **Learning System**: Continuous learning from text input
- **Performance Monitoring**: Built-in metrics and performance tracking

## Architecture

The Python API is built using PyO3 and provides:

- **Native Performance**: Rust-based core with Python bindings for optimal speed
- **Type Safety**: Comprehensive type hints for better development experience
- **Async Support**: Both sync and async interfaces for different use cases
- **Memory Efficiency**: Efficient memory management with automatic cleanup
- **Error Handling**: Comprehensive error handling with detailed error messages

## Quick Start

### Installation

```bash
# Install from PyPI (when available)
pip install brain-ai

# Or install from source
pip install git+https://github.com/your-org/brain-ai.git

# For development
git clone https://github.com/your-org/brain-ai.git
cd brain-ai
pip install -e .
```

### Basic Usage

```python
from brain_ai import BrainAI, BrainConfig
import asyncio

# Initialize Brain AI
config = BrainConfig(
    memory_capacity=10000,
    enable_performance_monitoring=True,
    log_level="INFO"
)

brain = BrainAI(config)

# Learn from text
result = brain.learn("Python is a versatile programming language")
print(f"Learned successfully: {result.success}")
print(f"Segments discovered: {result.segments_discovered}")

# Query memory
memories = brain.search_memory("programming language", limit=5)
for memory in memories:
    print(f"Memory: {memory.content} (confidence: {memory.confidence})")

# Get related concepts
concepts = brain.get_related_concepts("python", depth=2)
for concept in concepts:
    print(f"Related: {concept.name} (strength: {concept.relationship_strength})")
```

### Async Usage

```python
import asyncio
from brain_ai import AsyncBrainAI, BrainConfig

async def main():
    config = BrainConfig(memory_capacity=10000)
    brain = AsyncBrainAI(config)
    
    # Async learning
    result = await brain.learn("Async programming enables concurrent execution")
    print(f"Learning result: {result}")
    
    # Async memory search
    memories = await brain.search_memory("async programming")
    print(f"Found {len(memories)} memories")
    
    # Async concept discovery
    concepts = await brain.get_related_concepts("programming")
    print(f"Found {len(concepts)} related concepts")

# Run async example
asyncio.run(main())
```

## Core Classes

### BrainAI

The main synchronous interface to Brain AI functionality.

```python
class BrainAI:
    def __init__(self, config: BrainConfig) -> None: ...
    def learn(self, text: str, priority: str = "medium") -> LearningResult: ...
    def search_memory(self, query: str, limit: int = 10) -> List[Memory]: ...
    def get_related_concepts(self, concept: str, depth: int = 2) -> List[RelatedConcept]: ...
    def generate_insights(self, query: str) -> List[Insight]: ...
    def get_performance_metrics(self) -> PerformanceMetrics: ...
```

### AsyncBrainAI

The asynchronous interface for non-blocking operations.

```python
class AsyncBrainAI:
    def __init__(self, config: BrainConfig) -> None: ...
    async def learn(self, text: str, priority: str = "medium") -> LearningResult: ...
    async def search_memory(self, query: str, limit: int = 10) -> List[Memory]: ...
    async def get_related_concepts(self, concept: str, depth: int = 2) -> List[RelatedConcept]: ...
    async def generate_insights(self, query: str) -> List[Insight]: ...
    async def get_performance_metrics(self) -> PerformanceMetrics: ...
```

### BrainConfig

Configuration class for customizing Brain AI behavior.

```python
@dataclass
class BrainConfig:
    memory_capacity: int = 10000
    enable_performance_monitoring: bool = False
    log_level: str = "INFO"
    character_prediction_enabled: bool = True
    segment_discovery_algorithm: str = "adaptive_bpe"
    concept_graph_max_depth: int = 5
    insight_confidence_threshold: float = 0.7
    consolidation_threshold: float = 0.8
```

## Data Types

### Learning Result

```python
@dataclass
class LearningResult:
    success: bool
    learning_id: str
    segments_discovered: int
    concepts_updated: int
    concepts_created: int
    relationships_formed: int
    processing_time_ms: int
    insights: List[str]
```

### Memory

```python
@dataclass
class Memory:
    id: str
    content: str
    memory_type: str
    confidence: float
    importance: float
    created_at: datetime
    last_accessed: datetime
    access_count: int
    related_concepts: List[str]
    context: Dict[str, Any]
```

### Related Concept

```python
@dataclass
class RelatedConcept:
    name: str
    relationship_type: str
    relationship_strength: float
    distance: int
    path: List[ConceptRelationship]
```

### Insight

```python
@dataclass
class Insight:
    id: str
    insight_type: str
    title: str
    description: str
    confidence: float
    importance: float
    evidence: List[str]
    recommendations: List[str]
    generated_at: datetime
```

## Integration Patterns

### Jupyter Notebook Integration

```python
# Enable rich display in Jupyter
%load_ext brain_ai.jupyter

from brain_ai import BrainAI, BrainConfig
import pandas as pd

# Initialize
brain = BrainAI(BrainConfig(memory_capacity=5000))

# Learn from data
texts = ["Machine learning is powerful", "AI transforms industries"]
results = [brain.learn(text) for text in texts]

# Display results as DataFrame
df = pd.DataFrame([{
    'text': text,
    'segments': result.segments_discovered,
    'concepts': result.concepts_created,
    'processing_time': result.processing_time_ms
} for text, result in zip(texts, results)])

display(df)
```

### Pandas Integration

```python
import pandas as pd
from brain_ai import BrainAI

# Learn from DataFrame
def learn_from_dataframe(brain: BrainAI, df: pd.DataFrame, text_column: str):
    results = []
    for text in df[text_column]:
        if pd.notna(text):
            result = brain.learn(str(text))
            results.append(result)
    return results

# Search memories as DataFrame
def memories_to_dataframe(memories: List[Memory]) -> pd.DataFrame:
    return pd.DataFrame([{
        'id': mem.id,
        'content': mem.content,
        'type': mem.memory_type,
        'confidence': mem.confidence,
        'importance': mem.importance,
        'created_at': mem.created_at
    } for mem in memories])
```

### NumPy Integration

```python
import numpy as np
from brain_ai import BrainAI

def analyze_concept_similarities(brain: BrainAI, concepts: List[str]) -> np.ndarray:
    """Create similarity matrix for concepts"""
    n = len(concepts)
    similarity_matrix = np.zeros((n, n))
    
    for i, concept_a in enumerate(concepts):
        related = brain.get_related_concepts(concept_a, depth=1)
        for j, concept_b in enumerate(concepts):
            if i != j:
                # Find relationship strength
                for rel in related:
                    if rel.name == concept_b:
                        similarity_matrix[i][j] = rel.relationship_strength
                        break
    
    return similarity_matrix
```

## Error Handling

The Python API provides comprehensive error handling:

```python
from brain_ai import BrainAI, BrainError, ConfigurationError, MemoryError

try:
    brain = BrainAI(config)
    result = brain.learn("Some text")
except ConfigurationError as e:
    print(f"Configuration error: {e}")
except MemoryError as e:
    print(f"Memory system error: {e}")
except BrainError as e:
    print(f"General Brain AI error: {e}")
except Exception as e:
    print(f"Unexpected error: {e}")
```

### Custom Error Types

```python
class BrainError(Exception):
    """Base exception for Brain AI operations"""
    pass

class ConfigurationError(BrainError):
    """Configuration-related errors"""
    pass

class MemoryError(BrainError):
    """Memory system errors"""
    pass

class LearningError(BrainError):
    """Learning process errors"""
    pass

class ConceptGraphError(BrainError):
    """Concept graph operation errors"""
    pass
```

## Performance Considerations

### Memory Management

```python
# Efficient batch processing
def batch_learn(brain: BrainAI, texts: List[str], batch_size: int = 100):
    results = []
    for i in range(0, len(texts), batch_size):
        batch = texts[i:i + batch_size]
        batch_results = []
        
        for text in batch:
            result = brain.learn(text)
            batch_results.append(result)
        
        results.extend(batch_results)
        
        # Optional: trigger garbage collection
        import gc
        gc.collect()
    
    return results
```

### Async Processing

```python
import asyncio
from brain_ai import AsyncBrainAI

async def concurrent_learning(brain: AsyncBrainAI, texts: List[str], max_concurrent: int = 10):
    semaphore = asyncio.Semaphore(max_concurrent)
    
    async def learn_with_semaphore(text: str):
        async with semaphore:
            return await brain.learn(text)
    
    tasks = [learn_with_semaphore(text) for text in texts]
    results = await asyncio.gather(*tasks, return_exceptions=True)
    
    # Filter out exceptions
    successful_results = [r for r in results if not isinstance(r, Exception)]
    return successful_results
```

## Testing Support

Brain AI provides testing utilities for unit tests:

```python
import unittest
from brain_ai import BrainAI, BrainConfig
from brain_ai.testing import MockBrainAI, create_test_config

class TestMyApplication(unittest.TestCase):
    def setUp(self):
        # Use test configuration
        config = create_test_config()
        self.brain = BrainAI(config)
        
        # Or use mock for unit tests
        self.mock_brain = MockBrainAI()
    
    def test_learning(self):
        result = self.brain.learn("Test text")
        self.assertTrue(result.success)
        self.assertGreater(result.segments_discovered, 0)
    
    def test_memory_search(self):
        # First learn something
        self.brain.learn("Python programming language")
        
        # Then search for it
        memories = self.brain.search_memory("Python")
        self.assertGreater(len(memories), 0)
```

## Configuration Examples

### Development Configuration

```python
from brain_ai import BrainConfig

dev_config = BrainConfig(
    memory_capacity=1000,
    enable_performance_monitoring=True,
    log_level="DEBUG",
    character_prediction_enabled=True,
    segment_discovery_algorithm="adaptive_bpe",
    concept_graph_max_depth=3,
    insight_confidence_threshold=0.6
)
```

### Production Configuration

```python
prod_config = BrainConfig(
    memory_capacity=100000,
    enable_performance_monitoring=True,
    log_level="INFO",
    character_prediction_enabled=True,
    segment_discovery_algorithm="feedback_bpe",
    concept_graph_max_depth=5,
    insight_confidence_threshold=0.8,
    consolidation_threshold=0.9
)
```

### High-Performance Configuration

```python
high_perf_config = BrainConfig(
    memory_capacity=1000000,
    enable_performance_monitoring=False,  # Disable for max speed
    log_level="WARN",
    character_prediction_enabled=False,   # Disable if not needed
    segment_discovery_algorithm="bpe",    # Faster algorithm
    concept_graph_max_depth=3,           # Limit depth for speed
    insight_confidence_threshold=0.9,    # Higher threshold
    parallel_processing=True,
    max_concurrent_operations=8
)
```

This Python API provides a powerful and flexible interface to Brain AI's cognitive capabilities, enabling developers to build sophisticated AI-powered applications with ease.
