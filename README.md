# Brain: Character Ingestion Engine

A post-transformer developmental AI architecture that learns from scratch, starting at the character level. This is **Task 01** of the Brain project - implementing the foundational character-level predictor.

## Overview

The Character Ingestion Engine is the foundational layer of the Brain architecture. It implements a GRU-based neural network that:

- Reads raw character streams and predicts the next character
- Learns patterns at the most fundamental level of text
- Forms the base for higher-level pattern recognition and abstraction
- Runs efficiently on CPU (Mac Mini M1/M2) with GPU acceleration support

## Features

### ✅ Implemented (Task 01 - Character Ingestion Engine)

- **Character-Level Model Architecture**: GRU-based predictor with configurable layers
- **Data Processing Pipeline**: Efficient character tokenization and batching  
- **Training System**: Complete training loop with metrics tracking
- **Vocabulary Management**: Dynamic character vocabulary with save/load capabilities
- **Text Generation**: Sampling-based text generation with temperature control
- **CLI Interface**: Command-line tool for training and text generation
- **CPU Optimized**: Designed for Mac Mini M1/M2 performance

### 🔄 In Development (Future Tasks)

- Segment Discovery Module (Task 02)
- Concept Graph Engine (Task 03) 
- Insight Extraction Engine (Task 04)
- Simulation Engine (Task 05)
- Memory Architecture (Task 06)

## Quick Start

### Prerequisites

- Rust 1.70+ 
- Cargo package manager

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd brain
```

2. Build the project:
```bash
cargo build --release
```

3. Run tests:
```bash
cargo test
```

### Training a Model

Train the character predictor on sample text:

```bash
cargo run -- train -i examples/sample_text.txt -e 20 -b 16 -s 30
```

Options:
- `-i, --input <FILE>`: Input text file for training
- `-e, --epochs <NUM>`: Number of training epochs (default: 10)
- `-b, --batch-size <SIZE>`: Training batch size (default: 32)
- `-s, --seq-len <LENGTH>`: Sequence length for training (default: 50)

### Generating Text (Placeholder)

Generate text using a trained model:

```bash
cargo run -- generate -s "The quick" -l 100 -t 0.8
```

Options:
- `-s, --seed <TEXT>`: Seed text for generation (default: "The")
- `-l, --length <NUM>`: Length of text to generate (default: 100)
- `-t, --temperature <TEMP>`: Sampling temperature (default: 0.8)

*Note: Text generation currently shows placeholder output. Full implementation requires model persistence.*

## Architecture

### Core Components

1. **CharacterVocab**: Maps characters to indices and handles encoding/decoding
2. **CharacterPredictor**: GRU-based neural network for character prediction
3. **CharacterDataset**: Efficient data loading and batching for training
4. **CharacterTrainer**: Training loop with metrics tracking and optimization

### Model Architecture

```
Input Characters → Embedding Layer → GRU Layers → Linear Output → Character Probabilities
```

- **Embedding Dimension**: 128 (configurable)
- **Hidden Dimension**: 256 (configurable)  
- **GRU Layers**: 2 (configurable)
- **Dropout**: 0.1 (configurable)

### Key Design Decisions

- **GRU over LSTM**: Simpler architecture, better CPU performance
- **Character-level tokens**: Enables learning from scratch without pre-tokenization
- **Candle framework**: Pure Rust ML library for better integration
- **CPU-first design**: Optimized for Mac Mini M1/M2 development environment

## Performance Characteristics

- **Memory efficient**: Sliding window approach for long sequences
- **CPU optimized**: Designed for Mac Mini M1/M2 performance
- **Scalable**: Architecture supports GPU acceleration when available
- **Lightweight**: Small model size suitable for edge deployment

## Development Status

This project implements **Task 01** from the Brain development roadmap:

- ✅ **Task 01.1**: Character-Level Model Architecture
- ✅ **Task 01.2**: Data Processing and Batching Pipeline  
- ✅ **Task 01.3**: Training Loop and Evaluation Metrics

### Next Steps (Task 02)

The next phase will implement the Segment Discovery Module:
- Dynamic segmentation of character patterns into proto-words
- Entropy and frequency-based boundary detection
- Integration with the character predictor

## Testing

Run the complete test suite:

```bash
cargo test
```

Run specific test modules:

```bash
cargo test character_vocab
cargo test character_dataset
```

Run benchmarks:

```bash
cargo bench
```

## Contributing

This project follows the Brain development workflow:

1. Tasks are managed using Task Master
2. Each task has detailed implementation requirements
3. Code changes should include appropriate tests
4. Performance considerations are important for CPU deployment

## Technical Implementation Notes

### Character Vocabulary

- Uses `\0` for padding and `?` for unknown characters
- Automatically builds vocabulary from training text
- Supports save/load for model persistence
- Handles Unicode characters properly

### Training Metrics

- **Loss**: Cross-entropy loss for character prediction
- **Accuracy**: Character-level prediction accuracy
- **Perplexity**: Measure of prediction uncertainty

### Memory Management

- Sliding window for long sequences (max 100 characters)
- Batch processing for efficient GPU utilization
- Configurable sequence lengths for different use cases

## License

[License information to be added]

## References

- [Candle ML Framework](https://github.com/huggingface/candle)
- [GRU: Gated Recurrent Unit](https://arxiv.org/abs/1412.3555)
- Original Brain architecture design: `prd.txt` 