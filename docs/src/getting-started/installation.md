# Installation Guide

This guide provides detailed instructions for installing Brain AI in different environments, from development setup to production deployment.

## System Requirements

### Minimum Requirements
- **CPU**: 2 cores, 2.0 GHz
- **RAM**: 4 GB
- **Storage**: 2 GB free space
- **OS**: Linux, macOS, or Windows 10+

### Recommended Requirements
- **CPU**: 4+ cores, 3.0 GHz
- **RAM**: 8+ GB
- **Storage**: 10+ GB SSD
- **OS**: Ubuntu 20.04+, macOS 12+, Windows 11

### Dependencies

#### Required
- **Rust 1.70+** ([rustup.rs](https://rustup.rs/))
- **Git** for version control

#### Optional (for full features)
- **Python 3.8+** (for Python bindings)
- **Docker & Docker Compose** (for containerized deployment)
- **Neo4j 4.4+** (for concept graph storage)
- **Redis 6.0+** (for caching and session management)

## Installation Methods

### Method 1: Docker Deployment (Recommended)

Docker provides the easiest and most reliable way to run Brain AI with all dependencies.

#### Step 1: Install Docker

**Linux (Ubuntu/Debian):**
```bash
# Update package index
sudo apt update

# Install Docker
sudo apt install docker.io docker-compose

# Add user to docker group
sudo usermod -aG docker $USER

# Log out and back in, then test
docker --version
```

**macOS:**
```bash
# Using Homebrew
brew install --cask docker

# Or download Docker Desktop from docker.com
```

**Windows:**
Download and install Docker Desktop from [docker.com](https://docker.com)

#### Step 2: Clone and Deploy

```bash
# Clone the repository
git clone https://github.com/your-org/brain-ai.git
cd brain-ai

# Copy environment template
cp env.example .env

# Configure environment (edit .env as needed)
nano .env

# Deploy with Docker Compose
cd deployment/
docker-compose up -d

# Verify installation
curl http://localhost:8080/health
```

#### Step 3: Access the System

- **API**: `http://localhost:8080/api/v1`
- **Dashboard**: `http://localhost:8080/dashboard`
- **Documentation**: `http://localhost:8080/docs`

### Method 2: Native Installation

For development or custom deployments, install Brain AI natively.

#### Step 1: Install Rust

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Reload shell configuration
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version
```

#### Step 2: Install System Dependencies

**Linux (Ubuntu/Debian):**
```bash
sudo apt update
sudo apt install build-essential pkg-config libssl-dev libsqlite3-dev
```

**macOS:**
```bash
# Using Homebrew
brew install openssl sqlite

# You may need to set environment variables
export OPENSSL_DIR=$(brew --prefix openssl)
export SQLITE3_LIB_DIR=$(brew --prefix sqlite)/lib
```

**Windows:**
```powershell
# Install Visual Studio Build Tools
# Download from: https://visualstudio.microsoft.com/downloads/

# Install vcpkg for C++ dependencies
git clone https://github.com/Microsoft/vcpkg.git
cd vcpkg
.\bootstrap-vcpkg.bat
.\vcpkg integrate install
.\vcpkg install openssl sqlite3
```

#### Step 3: Clone and Build

```bash
# Clone the repository
git clone https://github.com/your-org/brain-ai.git
cd brain-ai

# Build the project
cargo build --release

# Run tests to verify installation
cargo test

# Install the binary (optional)
cargo install --path .
```

#### Step 4: Configure and Run

```bash
# Copy configuration template
cp scripts/config.toml.example scripts/config.toml

# Edit configuration as needed
nano scripts/config.toml

# Run Brain AI
./target/release/brain-server

# Or if installed globally
brain-server
```

### Method 3: Python Package Installation

Install Brain AI as a Python package for integration with existing Python projects.

#### Step 1: Install Python Dependencies

```bash
# Create virtual environment
python -m venv brain-ai-env
source brain-ai-env/bin/activate  # Linux/macOS
# brain-ai-env\Scripts\activate    # Windows

# Upgrade pip
pip install --upgrade pip
```

#### Step 2: Install Brain AI

```bash
# Install from PyPI (when available)
pip install brain-ai

# Or install from source
git clone https://github.com/your-org/brain-ai.git
cd brain-ai
pip install maturin
maturin develop --features python
```

#### Step 3: Verify Installation

```python
import brain_ai

# Test basic functionality
engine = brain_ai.BrainEngine()
result = engine.learn("Test text for learning")
print(f"Learning successful: {result.success}")
```

## Database Setup

### SQLite (Default)

SQLite is included by default and requires no additional setup:

```bash
# SQLite databases are created automatically
# Default location: ./data/
```

### Neo4j (Optional, for Concept Graph)

#### Docker Neo4j
```bash
# Run Neo4j in Docker
docker run -d \
  --name neo4j \
  -p 7474:7474 -p 7687:7687 \
  -e NEO4J_AUTH=neo4j/password \
  neo4j:4.4

# Verify connection
curl http://localhost:7474
```

#### Native Neo4j Installation

**Linux:**
```bash
# Add Neo4j repository
wget -O - https://debian.neo4j.com/neotechnology.gpg.key | sudo apt-key add -
echo 'deb https://debian.neo4j.com stable 4.4' | sudo tee /etc/apt/sources.list.d/neo4j.list

# Install Neo4j
sudo apt update
sudo apt install neo4j

# Start service
sudo systemctl start neo4j
sudo systemctl enable neo4j
```

**macOS:**
```bash
# Using Homebrew
brew install neo4j

# Start Neo4j
brew services start neo4j
```

#### Configure Neo4j Connection

Edit your configuration file:

```toml
[database.neo4j]
uri = "bolt://localhost:7687"
username = "neo4j"
password = "your_password"
database = "neo4j"
```

### Redis (Optional, for Caching)

#### Docker Redis
```bash
# Run Redis in Docker
docker run -d \
  --name redis \
  -p 6379:6379 \
  redis:7-alpine

# Test connection
redis-cli ping
```

#### Native Redis Installation

**Linux:**
```bash
sudo apt update
sudo apt install redis-server

# Start Redis
sudo systemctl start redis-server
sudo systemctl enable redis-server
```

**macOS:**
```bash
brew install redis
brew services start redis
```

## Configuration

### Environment Variables

Create a `.env` file in the project root:

```bash
# Core Configuration
BRAIN_HOST=0.0.0.0
BRAIN_PORT=8080
BRAIN_LOG_LEVEL=info

# Database Configuration
NEO4J_URI=bolt://localhost:7687
NEO4J_USER=neo4j
NEO4J_PASSWORD=password

REDIS_URL=redis://localhost:6379

# Security Configuration
JWT_SECRET=your-secret-key-here
BCRYPT_COST=12

# Performance Configuration
BRAIN_PERFORMANCE_MONITORING=true
BRAIN_MAX_MEMORY_MB=1024
BRAIN_WORKER_THREADS=4

# Feature Flags
ENABLE_PYTHON_BINDINGS=true
ENABLE_WEB_DASHBOARD=true
ENABLE_METRICS_EXPORT=true
```

### Configuration File

Create `scripts/config.toml`:

```toml
[server]
host = "0.0.0.0"
port = 8080
workers = 4

[logging]
level = "info"
file = "logs/brain-ai.log"
rotation = "daily"

[memory]
working_memory_size = 1000
episodic_memory_retention_days = 30
semantic_memory_threshold = 0.8

[character_ingestion]
vocab_size = 10000
sequence_length = 256
learning_rate = 0.001

[segment_discovery]
max_segments = 1000
min_frequency = 2
entropy_threshold = 0.5

[concept_graph]
max_connections = 100
decay_rate = 0.01
reinforcement_factor = 1.1

[simulation]
max_steps = 10
confidence_threshold = 0.3
branching_factor = 3

[performance]
enable_monitoring = true
metrics_interval_seconds = 60
alert_thresholds = { cpu = 80.0, memory = 85.0 }
```

## Verification and Testing

### Health Check

```bash
# Check system health
curl http://localhost:8080/health

# Expected response:
# {
#   "status": "healthy",
#   "timestamp": "2024-01-01T12:00:00Z",
#   "components": {
#     "database": "connected",
#     "memory": "ready",
#     "api": "operational"
#   }
# }
```

### Run Test Suite

```bash
# Run all tests
cargo test

# Run specific test categories
cargo test --test integration_tests
cargo test --test memory_tests
cargo test --test api_tests

# Run with verbose output
cargo test -- --nocapture
```

### Basic Functionality Test

```bash
# Test learning endpoint
curl -X POST http://localhost:8080/api/v1/learn \
  -H "Content-Type: application/json" \
  -d '{"text": "The quick brown fox jumps", "priority": "high"}'

# Test segmentation
curl -X POST http://localhost:8080/api/v1/segment \
  -H "Content-Type: application/json" \
  -d '{"text": "Hello world programming"}'

# Test memory query
curl "http://localhost:8080/api/v1/memory/search?query=fox&limit=5"
```

## Performance Optimization

### System Tuning

**Linux:**
```bash
# Increase file descriptor limits
echo "* soft nofile 65536" | sudo tee -a /etc/security/limits.conf
echo "* hard nofile 65536" | sudo tee -a /etc/security/limits.conf

# Optimize kernel parameters
echo "net.core.somaxconn = 1024" | sudo tee -a /etc/sysctl.conf
echo "vm.swappiness = 10" | sudo tee -a /etc/sysctl.conf
sudo sysctl -p
```

**Memory Configuration:**
```toml
[memory]
# Adjust based on available RAM
working_memory_size = 2000      # For 8GB+ RAM
episodic_memory_cache = 5000    # For 16GB+ RAM
```

### Monitoring Setup

Enable comprehensive monitoring:

```bash
# Install monitoring tools
docker-compose -f deployment/docker-compose.monitoring.yml up -d

# Access dashboards
# Prometheus: http://localhost:9090
# Grafana: http://localhost:3000 (admin/admin)
```

## Troubleshooting

### Common Issues

#### Rust Compilation Errors
```bash
# Update Rust toolchain
rustup update

# Clear cache and rebuild
cargo clean
cargo build --release
```

#### Database Connection Issues
```bash
# Check Neo4j status
docker logs neo4j

# Test connection
echo "RETURN 'Hello World'" | cypher-shell -u neo4j -p password
```

#### Permission Errors (Linux)
```bash
# Fix file permissions
sudo chown -R $USER:$USER /path/to/brain-ai
chmod +x target/release/brain-server
```

#### Port Already in Use
```bash
# Find process using port
sudo lsof -i :8080

# Kill process
sudo kill -9 <PID>

# Or use different port
export BRAIN_PORT=8081
```

### Log Analysis

```bash
# View application logs
tail -f logs/brain-ai.log

# View Docker logs
docker-compose logs -f brain-ai

# Check system resources
htop
df -h
```

### Getting Help

- **Documentation**: Complete guides in this documentation system
- **Issues**: Report bugs on GitHub Issues
- **Discussions**: Community discussions on GitHub Discussions
- **Support**: Enterprise support available

## Next Steps

After successful installation:

1. **[Configuration Guide](./configuration.md)**: Detailed configuration options
2. **[First Steps](./first-steps.md)**: Basic usage examples
3. **[API Reference](../api/overview.md)**: Complete API documentation
4. **[Deployment Guide](../deployment/docker.md)**: Production deployment

---

**Installation Complete!** Brain AI is now ready to learn and grow. Start with the [First Steps Guide](./first-steps.md) to begin exploring its cognitive capabilities.
