# Configuration

Brain AI provides extensive configuration options to tune its behavior for different use cases, from research and development to production deployment. This guide covers all configuration methods and options.

## Configuration Methods

Brain AI supports multiple configuration approaches that can be combined:

1. **Environment Variables** - For deployment and secrets
2. **Configuration Files** - For structured settings (TOML format)
3. **Command Line Arguments** - For runtime overrides
4. **API Configuration** - For dynamic runtime changes

### Priority Order

When the same setting is specified in multiple places, Brain AI uses this priority order (highest to lowest):

1. Command line arguments
2. Environment variables
3. Configuration file
4. Default values

## Environment Variables

### Core System Settings

```bash
# Server Configuration
BRAIN_HOST=0.0.0.0                    # Host to bind to
BRAIN_PORT=8080                       # Port to listen on
BRAIN_WORKERS=4                       # Number of worker threads

# Logging and Monitoring
BRAIN_LOG_LEVEL=info                  # Log level: trace, debug, info, warn, error
BRAIN_LOG_FORMAT=json                 # Log format: json, pretty, compact
BRAIN_PERFORMANCE_MONITORING=true     # Enable performance monitoring
BRAIN_METRICS_PORT=9090               # Metrics endpoint port

# Data Persistence
BRAIN_DATA_DIR=./data                 # Data storage directory
BRAIN_BACKUP_ENABLED=true             # Enable automatic backups
BRAIN_BACKUP_INTERVAL=3600            # Backup interval in seconds
```

### Database Connections

```bash
# Neo4j (Concept Graph)
NEO4J_URI=bolt://localhost:7687       # Neo4j connection URI
NEO4J_USER=neo4j                      # Neo4j username
NEO4J_PASSWORD=password               # Neo4j password
NEO4J_DATABASE=brain                  # Database name
NEO4J_MAX_CONNECTIONS=10              # Connection pool size

# SQLite (Memory System)
SQLITE_PATH=./data/brain_memory.db    # SQLite database path
SQLITE_CACHE_SIZE=64MB                # SQLite cache size
SQLITE_WAL_MODE=true                  # Enable WAL mode for performance

# Redis (Optional - for caching and sessions)
REDIS_URL=redis://localhost:6379     # Redis connection URL
REDIS_DB=0                           # Redis database number
REDIS_POOL_SIZE=10                   # Connection pool size
```

### Component Configuration

```bash
# Character Ingestion
CHAR_MODEL_SIZE=small                 # Model size: tiny, small, medium, large
CHAR_SEQUENCE_LENGTH=512              # Maximum sequence length
CHAR_BATCH_SIZE=32                    # Training batch size
CHAR_LEARNING_RATE=0.001              # Learning rate

# Segment Discovery
SEGMENT_MIN_FREQUENCY=5               # Minimum frequency for segment creation
SEGMENT_MAX_LENGTH=20                 # Maximum segment length
SEGMENT_ENTROPY_THRESHOLD=0.5         # Entropy threshold for boundaries
SEGMENT_PRUNING_INTERVAL=3600         # Pruning interval in seconds

# Memory System
MEMORY_WORKING_CAPACITY=1000          # Working memory capacity
MEMORY_CONSOLIDATION_THRESHOLD=0.7    # Consolidation confidence threshold
MEMORY_DECAY_RATE=0.001               # Memory decay rate per hour
MEMORY_SEMANTIC_DIMENSIONS=384        # Semantic embedding dimensions

# Concept Graph
CONCEPT_FORMATION_THRESHOLD=0.6       # Minimum confidence for concept formation
CONCEPT_HEBBIAN_LEARNING_RATE=0.01    # Hebbian learning rate
CONCEPT_PRUNING_THRESHOLD=0.1         # Minimum weight for relationship pruning
CONCEPT_MAX_CONNECTIONS=50            # Maximum connections per concept

# Simulation Engine
SIMULATION_MAX_STEPS=10               # Maximum simulation steps
SIMULATION_CONFIDENCE_THRESHOLD=0.3   # Minimum confidence for actions
SIMULATION_BRANCHING_FACTOR=3         # Number of branches to explore
SIMULATION_TIMEOUT=30                 # Simulation timeout in seconds
```

### Security and Authentication

```bash
# Authentication
BRAIN_AUTH_ENABLED=false              # Enable authentication
BRAIN_JWT_SECRET=your-secret-key      # JWT signing secret
BRAIN_JWT_EXPIRY=86400                # JWT expiry in seconds
BRAIN_API_KEY=your-api-key            # Simple API key (alternative to JWT)

# Rate Limiting
BRAIN_RATE_LIMIT_ENABLED=true         # Enable rate limiting
BRAIN_RATE_LIMIT_REQUESTS=100         # Requests per window
BRAIN_RATE_LIMIT_WINDOW=3600          # Window size in seconds

# CORS Settings
BRAIN_CORS_ENABLED=true               # Enable CORS
BRAIN_CORS_ORIGINS=*                  # Allowed origins (comma-separated)
BRAIN_CORS_METHODS=GET,POST,PUT,DELETE # Allowed methods
```

## Configuration Files

### Main Configuration File

Create a `config.toml` file for structured configuration:

```toml
[server]
host = "0.0.0.0"
port = 8080
workers = 4

[logging]
level = "info"
format = "json"
file = "./logs/brain.log"
rotation = "daily"
max_files = 30

[performance]
monitoring_enabled = true
metrics_port = 9090
profiling_enabled = false
flamegraph_output = "./profiles"

[database]
data_dir = "./data"
backup_enabled = true
backup_interval = 3600
backup_retention_days = 30

[database.neo4j]
uri = "bolt://localhost:7687"
user = "neo4j"
password = "password"
database = "brain"
max_connections = 10
connection_timeout = 30

[database.sqlite]
path = "./data/brain_memory.db"
cache_size = "64MB"
wal_mode = true
journal_mode = "WAL"
synchronous = "NORMAL"

[database.redis]
url = "redis://localhost:6379"
db = 0
pool_size = 10
connection_timeout = 5

[components.character_ingestion]
model_size = "small"
sequence_length = 512
batch_size = 32
learning_rate = 0.001
dropout = 0.1
weight_decay = 0.01

[components.segment_discovery]
min_frequency = 5
max_length = 20
entropy_threshold = 0.5
pruning_interval = 3600
context_window = 5
confidence_threshold = 0.6

[components.memory_system]
working_capacity = 1000
consolidation_threshold = 0.7
decay_rate = 0.001
semantic_dimensions = 384
faiss_index_type = "IVF"
faiss_nlist = 100

[components.concept_graph]
formation_threshold = 0.6
hebbian_learning_rate = 0.01
pruning_threshold = 0.1
max_connections = 50
spreading_activation_decay = 0.9
concept_merge_threshold = 0.8

[components.simulation_engine]
max_steps = 10
confidence_threshold = 0.3
branching_factor = 3
timeout = 30
use_monte_carlo = true
exploration_rate = 0.1

[components.neural_architecture]
attention_heads = 8
transformer_layers = 6
hidden_size = 512
feedforward_size = 2048
developmental_stages = ["embryonic", "infant", "child", "adolescent", "adult", "expert"]
growth_threshold = 0.8

[security]
auth_enabled = false
jwt_secret = "your-secret-key"
jwt_expiry = 86400
api_key = "your-api-key"
rate_limit_enabled = true
rate_limit_requests = 100
rate_limit_window = 3600

[security.cors]
enabled = true
origins = ["*"]
methods = ["GET", "POST", "PUT", "DELETE"]
headers = ["Content-Type", "Authorization"]
credentials = false

[api]
base_path = "/api/v1"
documentation_enabled = true
openapi_path = "/docs"
max_request_size = "10MB"
request_timeout = 60

[advanced]
meta_memory_enabled = true
novelty_detection_enabled = true
curiosity_learning_enabled = true
self_optimization_enabled = false
experimental_features = []
```

### Environment-Specific Configurations

#### Development Configuration (`config.dev.toml`)

```toml
[logging]
level = "debug"
format = "pretty"

[performance]
monitoring_enabled = true
profiling_enabled = true

[security]
auth_enabled = false
rate_limit_enabled = false

[components.character_ingestion]
model_size = "tiny"
batch_size = 8

[advanced]
experimental_features = ["enhanced_debugging", "verbose_logging"]
```

#### Production Configuration (`config.prod.toml`)

```toml
[server]
workers = 8

[logging]
level = "warn"
format = "json"
file = "/var/log/brain/brain.log"

[security]
auth_enabled = true
rate_limit_enabled = true

[database]
backup_enabled = true
backup_interval = 1800  # 30 minutes

[components.character_ingestion]
model_size = "large"
batch_size = 64

[performance]
monitoring_enabled = true
metrics_port = 9090

[advanced]
experimental_features = []
```

## Command Line Arguments

Override any configuration setting via command line:

```bash
# Basic usage
brain-server --config config.toml

# Override specific settings
brain-server \
  --host 0.0.0.0 \
  --port 9000 \
  --log-level debug \
  --workers 8

# Database overrides
brain-server \
  --neo4j-uri bolt://neo4j-server:7687 \
  --neo4j-user admin \
  --neo4j-password secret

# Component configuration
brain-server \
  --char-model-size medium \
  --memory-capacity 2000 \
  --concept-threshold 0.8

# Performance tuning
brain-server \
  --enable-monitoring \
  --metrics-port 9090 \
  --enable-profiling
```

## Dynamic Configuration

Some settings can be changed at runtime via the API:

### Update Component Settings

```bash
# Update character ingestion settings
curl -X PUT http://localhost:8080/api/config/character_ingestion \
  -H "Content-Type: application/json" \
  -d '{
    "learning_rate": 0.002,
    "batch_size": 64
  }'

# Update memory system settings
curl -X PUT http://localhost:8080/api/config/memory_system \
  -H "Content-Type: application/json" \
  -d '{
    "working_capacity": 1500,
    "decay_rate": 0.0005
  }'
```

### Get Current Configuration

```bash
# Get all configuration
curl http://localhost:8080/api/config

# Get specific component configuration
curl http://localhost:8080/api/config/concept_graph
```

## Configuration Validation

Brain AI validates configuration on startup and provides detailed error messages:

```bash
# Example validation error
Error: Invalid configuration
  - components.character_ingestion.learning_rate: must be between 0.0001 and 0.1
  - database.neo4j.uri: invalid URI format
  - components.memory_system.working_capacity: must be positive integer
```

### Configuration Schema

Brain AI uses a strict schema for validation. Key constraints:

- **Learning rates**: 0.0001 ≤ value ≤ 0.1
- **Thresholds**: 0.0 ≤ value ≤ 1.0
- **Capacities**: Must be positive integers
- **Timeouts**: Must be positive numbers (seconds)
- **Ports**: 1024 ≤ value ≤ 65535

## Performance Tuning

### Memory-Optimized Configuration

For systems with limited RAM:

```toml
[components.memory_system]
working_capacity = 500
semantic_dimensions = 256

[components.concept_graph]
max_connections = 25

[database.sqlite]
cache_size = "32MB"

[components.character_ingestion]
batch_size = 16
sequence_length = 256
```

### CPU-Optimized Configuration

For systems with limited CPU:

```toml
[server]
workers = 2

[components.character_ingestion]
model_size = "tiny"

[components.simulation_engine]
max_steps = 5
branching_factor = 2

[performance]
monitoring_enabled = false
```

### High-Performance Configuration

For powerful systems:

```toml
[server]
workers = 16

[components.character_ingestion]
model_size = "large"
batch_size = 128
sequence_length = 1024

[components.memory_system]
working_capacity = 5000
semantic_dimensions = 768

[components.concept_graph]
max_connections = 100

[database.neo4j]
max_connections = 20
```

## Environment-Specific Setup

### Docker Environment

Use environment variables in `docker-compose.yml`:

```yaml
services:
  brain-ai:
    environment:
      - BRAIN_HOST=0.0.0.0
      - BRAIN_PORT=8080
      - NEO4J_URI=bolt://neo4j:7687
      - REDIS_URL=redis://redis:6379
      - BRAIN_LOG_LEVEL=info
```

### Kubernetes Environment

Use ConfigMaps and Secrets:

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: brain-config
data:
  BRAIN_HOST: "0.0.0.0"
  BRAIN_PORT: "8080"
  BRAIN_LOG_LEVEL: "info"
  
---
apiVersion: v1
kind: Secret
metadata:
  name: brain-secrets
data:
  NEO4J_PASSWORD: <base64-encoded-password>
  BRAIN_JWT_SECRET: <base64-encoded-secret>
```

### Development Environment

Use `.env` file for local development:

```bash
# .env file
BRAIN_HOST=localhost
BRAIN_PORT=8080
BRAIN_LOG_LEVEL=debug
NEO4J_URI=bolt://localhost:7687
NEO4J_PASSWORD=development
BRAIN_AUTH_ENABLED=false
```

## Configuration Best Practices

### 1. Use Environment-Specific Configs

```bash
# Development
brain-server --config config.dev.toml

# Staging
brain-server --config config.staging.toml

# Production
brain-server --config config.prod.toml
```

### 2. Secure Sensitive Data

- Use environment variables for passwords and secrets
- Never commit secrets to version control
- Use proper secret management in production

### 3. Monitor Configuration Changes

- Log configuration changes
- Use configuration versioning
- Test configuration changes in staging first

### 4. Validate Before Deployment

```bash
# Validate configuration without starting server
brain-server --config config.toml --validate-only
```

### 5. Document Custom Settings

Maintain a `CONFIG.md` file documenting your specific configuration choices and rationale.

## Troubleshooting Configuration

### Common Issues

#### Database Connection Failures

```bash
# Check connectivity
telnet localhost 7687  # Neo4j
redis-cli ping         # Redis

# Verify credentials
brain-server --config config.toml --test-connections
```

#### Performance Issues

```bash
# Enable performance monitoring
BRAIN_PERFORMANCE_MONITORING=true brain-server

# Check metrics
curl http://localhost:9090/metrics
```

#### Memory Issues

```bash
# Reduce memory usage
MEMORY_WORKING_CAPACITY=500 \
CHAR_BATCH_SIZE=16 \
brain-server
```

### Configuration Debugging

Enable debug logging to see configuration loading:

```bash
BRAIN_LOG_LEVEL=debug brain-server --config config.toml
```

This will show:
- Which configuration files are loaded
- Environment variable overrides
- Final resolved configuration
- Validation results

## Migration Between Versions

When upgrading Brain AI, configuration may need migration:

```bash
# Check for configuration compatibility
brain-server --config config.toml --check-compatibility

# Migrate configuration to new format
brain-server --migrate-config config.toml --output config.new.toml
```

## Configuration Templates

Brain AI provides configuration templates for common scenarios:

```bash
# Generate configuration template
brain-server --generate-config development > config.dev.toml
brain-server --generate-config production > config.prod.toml
brain-server --generate-config research > config.research.toml
```

Each template is optimized for its intended use case with appropriate defaults and documentation.

Remember: Configuration is key to optimal Brain AI performance. Start with templates, customize for your use case, and monitor performance to fine-tune settings over time.
