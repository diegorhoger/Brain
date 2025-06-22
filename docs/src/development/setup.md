# Development Setup

This guide covers setting up a development environment for Brain AI.

## Prerequisites

### System Requirements

- **Operating System**: Linux, macOS, or Windows (with WSL2)
- **RAM**: Minimum 8GB, recommended 16GB+
- **Storage**: 20GB+ free space

### Required Software

1. **Rust Toolchain** (1.75+)
   ```bash
   # Install rustup
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   
   # Verify installation
   rustc --version
   cargo --version
   ```

2. **Python** (3.8+) for Python bindings
   ```bash
   # Using pyenv (recommended)
   curl https://pyenv.run | bash
   pyenv install 3.11.0
   pyenv global 3.11.0
   ```

3. **Git** for version control
   ```bash
   sudo apt install git     # Ubuntu/Debian
   brew install git         # macOS
   ```

## Project Setup

### Clone Repository

```bash
# Clone the repository
git clone https://github.com/your-org/brain-ai.git
cd brain-ai

# Create development branch
git checkout -b feature/your-feature-name
```

### Environment Configuration

1. **Create environment file**:
   ```bash
   cp env.example .env
   ```

2. **Configure development environment**:
   ```bash
   # .env file for development
   ANTHROPIC_API_KEY=your_api_key_here
   LOG_LEVEL=debug
   DEBUG=true
   MEMORY_CAPACITY=10000
   HOST=127.0.0.1
   PORT=8080
   JWT_SECRET=dev-secret-key
   ```

### Build and Run

1. **Install development dependencies**:
   ```bash
   # Install additional Rust components
   rustup component add rustfmt clippy
   
   # Install cargo tools
   cargo install cargo-watch
   cargo install cargo-nextest
   ```

2. **Build the project**:
   ```bash
   # Build in debug mode
   cargo build
   
   # Run tests
   cargo test
   
   # Run with hot reload
   cargo watch -x run
   ```

## Development Tools

### IDE Setup

#### VS Code Configuration

Create `.vscode/settings.json`:

```json
{
  "rust-analyzer.check.command": "clippy",
  "rust-analyzer.cargo.features": "all",
  "editor.formatOnSave": true,
  "python.defaultInterpreterPath": "./venv/bin/python"
}
```

### Code Formatting

```bash
# Format code
cargo fmt

# Run linting
cargo clippy

# Run tests
cargo test
```

## Development Workflow

### Running in Development Mode

```bash
# Start the server
cargo run

# Run with hot reload
cargo watch -x run

# Run examples
cargo run --example memory_demo
cargo run --example system_integration_demo
```

### Testing

```bash
# Run all tests
cargo test

# Run integration tests
cargo test --test integration_tests

# Run with coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out html
```

### Debugging

```bash
# Enable debug logging
export RUST_LOG=brain_ai=debug
cargo run

# Debug specific components
export RUST_LOG=brain_ai::memory=debug,brain_ai::concept_graph=debug
```

## Documentation Development

```bash
# Generate API docs
cargo doc --open

# Build mdBook documentation
cd docs && mdbook serve
```

## Contributing

### Branch Strategy

```bash
# Create feature branch
git checkout -b feature/your-feature-name

# Make changes and commit
git add .
git commit -m "feat: add new feature"

# Push to remote
git push origin feature/your-feature-name
```

### Code Review Checklist

- [ ] Code follows style guidelines
- [ ] Tests are included and passing
- [ ] Documentation is updated
- [ ] No clippy warnings

This development setup guide provides the essentials for Brain AI development.
