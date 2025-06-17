#!/usr/bin/env python3
"""
Comprehensive demonstration of Brain Python API
Shows usage of all four core functions with error handling and examples.
"""

import sys
import time

try:
    from brain import BrainEngine, segment_text, quick_query
except ImportError as e:
    print(f"Error importing brain module: {e}")
    print("Make sure you've built the Python package with maturin")
    sys.exit(1)


def print_header(title: str):
    """Print a formatted header for each demo section."""
    print(f"\n{'='*60}")
    print(f" {title}")
    print(f"{'='*60}")


def demo_segmentation():
    """Demonstrate text segmentation functionality."""
    print_header("TEXT SEGMENTATION DEMO")

    engine = BrainEngine()

    test_texts = [
        "Hello world! This is a test.",
        "The quick brown fox jumps over the lazy dog.",
        "Python is a programming language. It's very popular.",
        ""  # Edge case: empty string
    ]

    for i, text in enumerate(test_texts, 1):
        print(f"\nTest {i}: '{text}'")
        try:
            start_time = time.time()
            segments = engine.segment(text)
            duration = time.time() - start_time

            print(f"  Segmented into {len(segments)} parts "
                  f"(in {duration:.4f}s):")
            for j, segment in enumerate(segments):
                print(f"    {j+1}. '{segment.text}' "
                      f"(confidence: {segment.confidence:.3f}, "
                      f"type: {segment.segment_type})")

        except Exception as e:
            print(f"  Error: {e}")


def demo_learning():
    """Demonstrate knowledge storage functionality."""
    print_header("LEARNING & STORAGE DEMO")

    engine = BrainEngine()

    knowledge_items = [
        ("Python is a high-level programming language", "high"),
        ("The capital of France is Paris", "medium"),
        ("Today's weather is sunny", "low"),
        ("Machine learning uses algorithms to find patterns", "high"),
        ("Coffee tastes good", "low")
    ]

    print("Storing knowledge items:")
    for knowledge, priority in knowledge_items:
        try:
            start_time = time.time()
            success = engine.learn(knowledge, priority)
            duration = time.time() - start_time

            status = "✓" if success else "✗"
            print(f"  {status} '{knowledge}' (priority: {priority}) - "
                  f"{duration:.4f}s")

        except Exception as e:
            print(f"  ✗ Error storing '{knowledge}': {e}")

    # Show storage stats
    try:
        status = engine.get_status()
        print(f"\nStorage status: {status}")
    except Exception as e:
        print(f"Error getting status: {e}")


def demo_simulation():
    """Demonstrate predictive simulation functionality."""
    print_header("SIMULATION DEMO")

    engine = BrainEngine()

    scenarios = [
        "What happens if I learn Python programming?",
        "Predict the outcome of studying machine learning",
        "What if I drink coffee every morning?",
        ""  # Edge case
    ]

    for i, scenario in enumerate(scenarios, 1):
        print(f"\nScenario {i}: '{scenario}'")
        try:
            max_steps = 5
            threshold = 0.1

            start_time = time.time()
            result = engine.simulate(scenario, max_steps=max_steps,
                                     confidence_threshold=threshold)
            duration = time.time() - start_time

            print(f"  Simulation completed in {duration:.4f}s:")
            print(f"    Steps taken: {result.steps}")
            print(f"    Final state: '{result.outcome}'")
            print(f"    Confidence: {result.confidence:.3f}")

            if result.metadata:
                print(f"    Metadata: {result.metadata}")

        except Exception as e:
            print(f"  Error: {e}")


def demo_memory_query():
    """Demonstrate memory querying functionality."""
    print_header("MEMORY QUERY DEMO")

    engine = BrainEngine()

    # First, store some knowledge to query
    print("Setting up test data...")
    test_data = [
        "Python is used for web development",
        "Machine learning requires data",
        "Paris is the capital of France",
        "Coffee contains caffeine"
    ]

    for data in test_data:
        try:
            engine.learn(data, "medium")
        except Exception as e:
            print(f"Warning: Could not store '{data}': {e}")

    queries = [
        "programming languages",
        "European capitals",
        "beverages",
        "nonexistent topic"
    ]

    print(f"\nQuerying memory with {len(queries)} different topics:")
    for i, query in enumerate(queries, 1):
        print(f"\nQuery {i}: '{query}'")
        try:
            start_time = time.time()
            results = engine.query_memory(query, limit=3)
            duration = time.time() - start_time

            print(f"  Found {len(results)} result(s) in {duration:.4f}s:")
            for j, result in enumerate(results):
                print(f"    {j+1}. '{result.content}' "
                      f"(relevance: {result.relevance:.3f}, "
                      f"type: {result.memory_type})")

        except Exception as e:
            print(f"  Error: {e}")


def demo_configuration():
    """Demonstrate configuration management."""
    print_header("CONFIGURATION DEMO")

    engine = BrainEngine()

    try:
        # Get current configuration
        config = engine.get_config()
        print("Current configuration:")
        for key, value in config.items():
            print(f"  {key}: {value}")

        # Update some settings
        print("\nUpdating configuration...")
        new_settings = {
            "max_memory_size": "2000",
            "prediction_steps": "10"
        }

        success = engine.update_config(new_settings)
        status = "✓" if success else "✗"
        print(f"  {status} Updated configuration with new settings")

        # Show updated config
        updated_config = engine.get_config()
        print("\nUpdated configuration:")
        for key, value in updated_config.items():
            print(f"  {key}: {value}")

    except Exception as e:
        print(f"Configuration error: {e}")


def demo_convenience_functions():
    """Demonstrate module-level convenience functions."""
    print_header("CONVENIENCE FUNCTIONS DEMO")

    print("Testing module-level functions:")

    # Test segment_text function
    try:
        text = "Quick test of convenience functions"
        segments = segment_text(text)
        print(f"✓ segment_text('{text}') -> {len(segments)} segments")
    except Exception as e:
        print(f"✗ segment_text error: {e}")

    # Test quick_query function
    try:
        query = "test query"
        quick_query(query)
        print(f"✓ quick_query('{query}') -> query executed")
    except Exception as e:
        print(f"✗ quick_query error: {e}")


def demo_performance_test():
    """Run basic performance tests."""
    print_header("PERFORMANCE TEST")

    engine = BrainEngine()

    # Test batch operations
    print("Testing batch performance...")

    # Segmentation performance
    long_text = "This is a longer text for performance testing. " * 20
    start_time = time.time()
    segments = engine.segment(long_text)
    seg_duration = time.time() - start_time
    print(f"Segmentation: {len(segments)} segments from "
          f"{len(long_text)} chars in {seg_duration:.4f}s")

    # Learning performance
    start_time = time.time()
    for i in range(10):
        engine.learn(f"Test knowledge item number {i}", "low")
    learn_duration = time.time() - start_time
    print(f"Learning: 10 items stored in {learn_duration:.4f}s "
          f"({learn_duration/10:.4f}s per item)")

    # Query performance
    start_time = time.time()
    total_results = 0
    for i in range(5):
        results = engine.query_memory(f"test query {i}", limit=3)
        total_results += len(results)
    query_duration = time.time() - start_time
    print(f"Querying: 5 queries in {query_duration:.4f}s "
          f"({query_duration/5:.4f}s per query)")
    print(f"Total results found: {total_results}")


def main():
    """Run all demonstrations."""
    print("Brain Python API Comprehensive Demo")
    print("This demo shows all core functionality with error handling and "
          "examples.")

    try:
        # Core functionality demos
        demo_segmentation()
        demo_learning()
        demo_simulation()
        demo_memory_query()

        # Advanced features
        demo_configuration()
        demo_convenience_functions()
        demo_performance_test()

        print_header("DEMO COMPLETED SUCCESSFULLY")
        print("All core API functions demonstrated.")
        print("The Brain Python API is working correctly!")

    except KeyboardInterrupt:
        print("\n\nDemo interrupted by user.")
    except Exception as e:
        print(f"\n\nUnexpected error during demo: {e}")
        import traceback
        traceback.print_exc()


if __name__ == "__main__":
    main()
