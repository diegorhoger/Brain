# First Steps

Welcome to Brain AI! This guide will walk you through your first interactions with the system and help you understand its core capabilities. By the end of this guide, you'll have hands-on experience with Brain AI's cognitive architecture.

## Understanding Brain AI

Brain AI is a developmental cognitive architecture that learns and evolves through interaction. Unlike traditional AI systems that are pre-trained on fixed datasets, Brain AI:

- **Learns continuously** from character-level input
- **Discovers patterns** and segments automatically
- **Forms concepts** and relationships dynamically
- **Simulates scenarios** based on learned knowledge
- **Adapts its architecture** as it grows

## Your First Session

### 1. Verify Your Installation

First, make sure Brain AI is running properly:

```bash
# Check system health
curl http://localhost:8080/health

# Expected response:
{
  "status": "healthy",
  "timestamp": "2024-01-01T12:00:00Z",
  "components": {
    "character_ingestion": "ready",
    "segment_discovery": "ready",
    "memory_system": "ready",
    "concept_graph": "ready",
    "simulation_engine": "ready"
  }
}
```

### 2. Start with Character Prediction

Brain AI begins by learning at the character level. Let's see it in action:

```python
from brain import BrainEngine

# Initialize the system
brain = BrainEngine()

# Start with simple character prediction
text = "Hello wor"
next_chars = brain.predict_next_chars(text, num_predictions=3)
print(f"Predicted next characters: {next_chars}")
# Output: ['l', 'd', ' '] (with confidence scores)
```

### 3. Discover Patterns and Segments

As Brain AI processes text, it automatically discovers meaningful patterns:

```python
# Feed it some text to learn from
brain.learn("The quick brown fox jumps over the lazy dog")
brain.learn("The cat sat on the mat")
brain.learn("The dog ran in the park")

# See what segments it discovered
segments = brain.get_discovered_segments()
print("Discovered segments:")
for segment, stats in segments.items():
    print(f"  '{segment}': frequency={stats.frequency}, confidence={stats.confidence}")

# Expected output might include:
#   'the': frequency=4, confidence=0.95
#   'cat': frequency=1, confidence=0.78
#   'dog': frequency=2, confidence=0.82
```

### 4. Explore Memory Formation

Brain AI has three types of memory working together:

```python
# Working memory (temporary, high-priority information)
brain.add_to_working_memory("Current task: learning about animals", priority=0.9)

# Episodic memory (events and experiences)
brain.store_episode("I saw a cat chase a mouse", timestamp="now", importance=0.7)

# Semantic memory (abstract knowledge)
brain.store_semantic("Cats are predators that hunt small animals")

# Query memories
memories = brain.query_memory("cat", memory_type="all")
for memory in memories:
    print(f"{memory.type}: {memory.content} (confidence: {memory.confidence})")
```

### 5. Watch Concepts Form

As Brain AI learns, it forms abstract concepts and relationships:

```python
# Add related information
brain.learn("Dogs bark loudly")
brain.learn("Cats meow softly")
brain.learn("Both dogs and cats are pets")
brain.learn("Pets live with humans")

# Explore the concept graph
concepts = brain.get_concepts_near("cat", radius=2)
print("Concepts related to 'cat':")
for concept in concepts:
    print(f"  {concept.name}: {concept.concept_type} (strength: {concept.activation})")

# Get relationships
relationships = brain.get_concept_relationships("cat")
for rel in relationships:
    print(f"  {rel.source} --{rel.relationship_type}--> {rel.target} (weight: {rel.weight})")
```

### 6. Try Simple Simulation

Brain AI can simulate scenarios based on what it has learned:

```python
# Run a simple simulation
result = brain.simulate(
    scenario="A cat sees a mouse",
    max_steps=3,
    confidence_threshold=0.3
)

print(f"Simulation result: {result.outcome}")
print(f"Confidence: {result.confidence}")
print("Steps taken:")
for i, step in enumerate(result.steps):
    print(f"  {i+1}. {step.action} -> {step.result}")

# Example output:
# 1. cat notices mouse -> cat becomes alert
# 2. cat stalks mouse -> cat moves closer
# 3. cat pounces -> mouse tries to escape
```

## Understanding the Learning Process

### Character-Level Foundation

Brain AI starts by learning character-by-character patterns. This foundational layer allows it to:

- Handle any language or writing system
- Discover natural word boundaries
- Learn spelling and grammar implicitly
- Adapt to new vocabularies automatically

### Pattern Discovery

As it processes text, Brain AI automatically:

- Identifies frequently occurring character sequences
- Forms proto-words and morphemes
- Builds a dynamic vocabulary
- Tracks usage statistics and contexts

### Concept Formation

From patterns and segments, Brain AI forms abstract concepts:

- Groups related segments into concept nodes
- Creates weighted relationships between concepts
- Strengthens connections through repeated use (Hebbian learning)
- Prunes weak or unused connections

### Memory Consolidation

Information flows between memory types:

- **Working Memory** → **Episodic Memory** (important experiences)
- **Episodic Memory** → **Semantic Memory** (repeated patterns)
- **Semantic Memory** → **Concept Graph** (abstract relationships)

## Monitoring Learning Progress

### Check Learning Statistics

```python
# Get overall system statistics
stats = brain.get_learning_stats()
print(f"Characters processed: {stats.characters_processed}")
print(f"Segments discovered: {stats.segments_discovered}")
print(f"Concepts formed: {stats.concepts_formed}")
print(f"Memory entries: {stats.memory_entries}")
```

### Visualize Progress

```python
# Generate learning progress visualization
brain.generate_learning_report("learning_progress.html")
# Opens a web page showing:
# - Character prediction accuracy over time
# - Segment discovery timeline
# - Concept graph growth
# - Memory consolidation patterns
```

### Performance Metrics

```python
# Check prediction performance
performance = brain.get_prediction_performance()
print(f"Character accuracy: {performance.character_accuracy:.2%}")
print(f"Segment accuracy: {performance.segment_accuracy:.2%}")
print(f"Average confidence: {performance.avg_confidence:.2f}")
```

## Common First-Session Patterns

### 1. Start Simple

Begin with basic, clear text:
```python
brain.learn("The sun is bright")
brain.learn("The moon is dim")
brain.learn("Stars shine at night")
```

### 2. Build Gradually

Add related concepts:
```python
brain.learn("The sun gives light during the day")
brain.learn("The moon appears at night")
brain.learn("Stars are distant suns")
```

### 3. Explore Relationships

```python
# See how concepts connect
sun_concepts = brain.get_related_concepts("sun")
# Might show: light, bright, day, star, heat, solar
```

### 4. Test Understanding

```python
# Ask questions through simulation
result = brain.simulate("What happens when the sun sets?")
# Brain AI will use learned relationships to predict outcomes
```

## Next Steps

Once you're comfortable with these basics:

1. **Explore the [Architecture Guide](../architecture/system-architecture.md)** to understand how components work together
2. **Try the [Python Examples](../examples/basic-examples.md)** for more complex use cases
3. **Read about [Component Deep-Dives](../components/)** for detailed technical information
4. **Check out [Advanced Features](../advanced/)** like meta-memory and novelty detection

## Troubleshooting First Steps

### Brain AI Seems Slow

- Check if you're running in debug mode (use `--release` for faster performance)
- Ensure adequate RAM (4GB minimum, 8GB recommended)
- Consider enabling performance monitoring to identify bottlenecks

### Predictions Seem Random

- Brain AI needs time to learn patterns (try feeding it more text)
- Check if the text domain is consistent (mixing languages/styles can confuse early learning)
- Verify character encoding is correct (UTF-8 recommended)

### Memory Not Persisting

- Ensure database connections are configured correctly
- Check file permissions for data directories
- Verify disk space is available for database growth

### Concepts Not Forming

- Feed Brain AI more structured, related text
- Check that concept formation thresholds aren't too high
- Allow more time for pattern discovery (concepts emerge gradually)

## Getting Help

- **Documentation**: Browse the full documentation for detailed guides
- **Examples**: Check the `examples/` directory for working code samples
- **Issues**: Report problems on the project's issue tracker
- **Community**: Join discussions in the project forums

Remember: Brain AI learns and grows over time. Don't expect perfect performance immediately—like a developing mind, it needs experience to become capable and nuanced in its understanding.

## What's Next?

Now that you've taken your first steps with Brain AI, you're ready to:

- **Dive deeper** into specific components
- **Integrate** Brain AI into your own projects
- **Experiment** with different learning scenarios
- **Contribute** to the Brain AI ecosystem

Welcome to the journey of developmental AI! 🧠✨
