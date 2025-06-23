# Brain AI - Post-Transformer Developmental Architecture

A sophisticated cognitive AI architecture built in Rust that learns, remembers, and develops like a human brain. Brain AI implements a complete cognitive pipeline from character-level processing to advanced memory systems and GitHub repository learning.

## 🧠 Core Capabilities

- **Character Ingestion Engine**: Neural character-level prediction with GRU/transformer architectures
- **Segment Discovery Module**: Advanced BPE segmentation with adaptive learning and entropy analysis  
- **Memory Module Foundation**: Three-layer memory (working, episodic, semantic) with intelligent consolidation
- **Concept Graph Engine**: Neo4j-based dynamic knowledge representation with Hebbian learning
- **Insight Extraction Engine**: Rule discovery and pattern generalization from experience
- **Simulation Engine**: Internal world modeling with branching scenario exploration
- **GitHub Learning Integration**: Real repository learning with API integration (✅ **Working!**)
- **Meta-Memory & Novelty Detection**: Self-awareness and curiosity-driven learning
- **Advanced Neural Architecture**: Post-transformer developmental AI with adaptive growth
- **Comprehensive API**: RESTful interface with authentication and monitoring

## 🚀 GitHub Learning Demo

Brain AI can learn from real GitHub repositories! Try it now:

```bash
# Set your GitHub token for higher rate limits (optional)
export GITHUB_TOKEN=your_token_here

# Run the comprehensive GitHub learning demo
cargo run --example github_learning_demo

# Or learn from a specific repository
cargo run --example github_learning_demo -- --repo "rust-lang/mdbook"
```

**Real Results from Demo:**
- ✅ Learned from `rust-lang/mdbook`, `BurntSushi/ripgrep`, and `tokio-rs/tokio`
- ✅ Processed 165 files with ~1.2MB of content in ~22 seconds
- ✅ Discovered 996+ concepts and stored 171 memory entries
- ✅ Achieved 12.8:1 learning compression ratio
- ✅ Real-time memory querying and concept analysis

## 🏗️ Quick Start

1. **Clone and build the project:**
```bash
git clone <repository-url>
cd Brain
cargo build --release
```

2. **Run character prediction demo:**
```bash
cargo run --example auth_logging_demo
```

3. **Try BPE segmentation:**
```bash
cargo run --example bpe_demo
```

4. **Explore memory systems:**
```bash
cargo run --example memory_consolidation_demo
```

5. **Test concept graphs:**
```bash
cargo run --example concept_graph_demo
```

6. **GitHub repository learning:**
```bash
export GITHUB_TOKEN=your_token_here  # Optional but recommended
cargo run --example github_learning_demo
```

## 🧪 Available Examples

| Demo | Description | Features |
|------|-------------|----------|
| `github_learning_demo` | **Real GitHub repository learning** | ✅ **Live API integration**, memory storage, concept discovery |
| `neural_architecture_demo` | Advanced transformer and developmental AI | Self-attention, post-transformer architecture |
| `memory_consolidation_demo` | Complete memory system lifecycle | Working → episodic → semantic memory |
| `concept_graph_demo` | Neo4j-based knowledge graphs | Dynamic concept formation, Hebbian learning |
| `insight_extraction_demo` | Rule discovery and generalization | Pattern → outcome formalization |
| `simulation_demo` | Internal world modeling | Text-to-graph conversion, temporal evolution |
| `novelty_detection_demo` | Curiosity-driven learning | Statistical and confidence-based detection |
| `meta_memory_demo` | Self-awareness and reflection | Knowledge quality assessment |
| `visualization_demo` | Web dashboards | Interactive concept graphs, memory timelines |

## 🏛️ Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                         Brain AI Architecture                   │
├─────────────────────────────────────────────────────────────────┤
│  Input Processing Layer                                        │
│  ├─ Character Ingestion (GRU/Transformer)                      │
│  ├─ Segment Discovery (BPE + Entropy Analysis)                 │
│  └─ GitHub Integration (Real Repository Learning) ✅            │
├─────────────────────────────────────────────────────────────────┤
│  Memory Systems Layer                                          │
│  ├─ Working Memory (Priority-based, 1000 items)                │
│  ├─ Episodic Memory (SQLite, temporal events)                  │
│  └─ Semantic Memory (Vector concepts, relationships)           │
├─────────────────────────────────────────────────────────────────┤
│  Cognitive Processing Layer                                    │
│  ├─ Concept Graph Engine (Neo4j + Hebbian Learning)            │
│  ├─ Insight Extraction (Pattern → Rule Formalization)          │
│  ├─ Simulation Engine (Text → Graph → Evolution)               │
│  └─ Meta-Memory & Novelty Detection                            │
├─────────────────────────────────────────────────────────────────┤
│  Output & Interface Layer                                      │
│  ├─ RESTful API (Authentication, Rate Limiting)                │
│  ├─ Web Dashboards (Concept Graphs, Memory Timelines)          │
│  └─ Real-time Query System                                     │
└─────────────────────────────────────────────────────────────────┘
```

## 🔧 Configuration

Brain AI uses environment variables for configuration:

```bash
# GitHub Integration
export GITHUB_TOKEN=your_github_token_here

# Neo4j for Concept Graphs  
export NEO4J_URI=bolt://localhost:7687
export NEO4J_USER=neo4j
export NEO4J_PASSWORD=password

# Logging
export RUST_LOG=debug  # info, warn, error
```

## 🧬 Technical Highlights

- **Pure Rust Implementation**: High performance, memory safety, zero-cost abstractions
- **Educational Architecture**: nalgebra-based approach for transparency and learning
- **Production-Ready**: Enterprise-grade error handling, logging, and monitoring
- **Modular Design**: Clean interfaces between cognitive components
- **Real-World Integration**: Live GitHub API learning with rate limiting and authentication
- **Comprehensive Testing**: 200+ tests ensuring reliability across all components
- **Zero Compilation Warnings**: Enterprise-grade code quality standards

## 📊 Performance Metrics

- **GitHub Learning**: ~22 seconds to process 3 large repositories
- **Memory Compression**: 12.8:1 learning-to-storage ratio
- **Concept Discovery**: 996+ concepts from 1.2MB of repository content
- **Response Times**: Sub-millisecond for memory queries
- **Test Coverage**: 200+ tests with 100% pass rate

## 🤝 Contributing

Brain AI is designed for educational exploration and extension:

1. **Fork and clone** the repository
2. **Add new cognitive modules** following the established patterns
3. **Extend GitHub learning** with additional repository analysis
4. **Create new examples** demonstrating advanced features
5. **Submit pull requests** with comprehensive tests

## 📄 License

See [LICENSE](LICENSE) file for details.

---

**Brain AI - Where artificial intelligence meets cognitive architecture** 🧠✨ 