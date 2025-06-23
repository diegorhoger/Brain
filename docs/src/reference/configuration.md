# Configuration Reference

Complete reference for all Brain AI configuration options, environment variables, and settings across all components and deployment scenarios.

## Configuration Hierarchy

Brain AI uses a layered configuration system with the following precedence (highest to lowest):

1. **Command-line arguments** (highest priority)
2. **Environment variables**
3. **Configuration files** (TOML format)
4. **Default values** (lowest priority)

## Environment Variables

### Core System Configuration

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `ANTHROPIC_API_KEY` | string | *required* | Anthropic API key for Claude integration |
| `PERPLEXITY_API_KEY` | string | *optional* | Perplexity API key for research features |
| `LOG_LEVEL` | string | `info` | Logging level: `debug`, `info`, `warn`, `error` |
| `DEBUG` | boolean | `false` | Enable debug mode with verbose logging |
| `CONFIG_FILE` | string | `config/brain.toml` | Path to main configuration file |

### Server Configuration

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `HOST` | string | `0.0.0.0` | Server bind address |
| `PORT` | integer | `8080` | Server port number |
| `WORKERS` | integer | `auto` | Number of worker threads (auto = CPU cores) |
| `MAX_CONNECTIONS` | integer | `1000` | Maximum concurrent connections |
| `REQUEST_TIMEOUT` | integer | `30` | Request timeout in seconds |

### Database Configuration

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `DATABASE_URL` | string | `sqlite:data/brain.db` | Database connection string |
| `DATABASE_POOL_SIZE` | integer | `20` | Connection pool size |
| `DATABASE_TIMEOUT` | integer | `30` | Connection timeout in seconds |
| `DATABASE_MAX_LIFETIME` | integer | `3600` | Connection max lifetime in seconds |

### Memory System Configuration

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `MEMORY_CAPACITY` | integer | `1000000` | Maximum number of memories |
| `WORKING_MEMORY_SIZE` | integer | `1000` | Working memory capacity |
| `MEMORY_CACHE_SIZE_MB` | integer | `256` | Memory cache size in MB |
| `CONSOLIDATION_THRESHOLD` | float | `0.8` | Memory consolidation threshold |

### Learning Configuration

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `LEARNING_WORKERS` | integer | `4` | Number of learning worker threads |
| `BATCH_SIZE` | integer | `100` | Learning batch size |
| `SEGMENT_MIN_FREQUENCY` | integer | `2` | Minimum frequency for segment discovery |
| `CONCEPT_SIMILARITY_THRESHOLD` | float | `0.7` | Concept similarity threshold |

### Performance Configuration

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `ENABLE_MONITORING` | boolean | `true` | Enable performance monitoring |
| `METRICS_INTERVAL` | integer | `60` | Metrics collection interval in seconds |
| `CACHE_ENABLED` | boolean | `true` | Enable application caching |
| `COMPRESSION_ENABLED` | boolean | `false` | Enable data compression |

## TOML Configuration Files

### Main Configuration File

```toml
# config/brain.toml - Main configuration file

[system]
project_name = "brain-ai"
version = "1.0.0"
log_level = "info"
debug = false
config_validation = true

[server]
host = "0.0.0.0"
port = 8080
workers = 8
max_connections = 1000
request_timeout = 30
keep_alive_timeout = 75
graceful_shutdown_timeout = 30

[database]
url = "postgresql://user:password@localhost:5432/brain_ai"
pool_size = 20
timeout = 30
max_lifetime = 3600
idle_timeout = 600
test_on_borrow = true
migration_auto = true

[memory]
capacity = 1000000
working_memory_size = 1000
cache_size_mb = 256
consolidation_threshold = 0.8
cleanup_interval = 3600
compression = false
encryption = false

[learning]
workers = 4
batch_size = 100
queue_size = 10000
timeout = 300
parallel_processing = true
adaptive_thresholds = true

[learning.segment_discovery]
algorithm = "adaptive_bpe"
min_frequency = 2
max_segment_length = 50
vocabulary_size = 50000
merge_threshold = 0.5

[learning.concept_extraction]
similarity_threshold = 0.7
max_concepts_per_memory = 10
relationship_threshold = 0.6
clustering_algorithm = "hierarchical"

[performance]
monitoring_enabled = true
metrics_interval = 60
cache_enabled = true
cache_size_mb = 128
cache_ttl = 300
compression_enabled = false
compression_algorithm = "lz4"

[security]
auth_enabled = true
jwt_secret = "${JWT_SECRET}"
jwt_expires_in = 3600
bcrypt_rounds = 12
rate_limiting_enabled = true
max_requests_per_minute = 1000

[logging]
level = "info"
format = "json"
output = "stdout"
file_path = "/var/log/brain-ai/application.log"
max_file_size_mb = 100
max_files = 10
compress_rotated = true
```

### Environment-Specific Configurations

#### Development Configuration

```toml
# config/development.toml
[system]
debug = true
log_level = "debug"

[database]
url = "sqlite:data/brain_dev.db"
pool_size = 5

[memory]
capacity = 10000
cache_size_mb = 64

[security]
auth_enabled = false
jwt_expires_in = 86400  # 24 hours

[performance]
monitoring_enabled = true
cache_enabled = false  # Disable for development
```

#### Production Configuration

```toml
# config/production.toml
[system]
debug = false
log_level = "info"

[server]
workers = 16
max_connections = 2000

[database]
url = "${DATABASE_URL}"
pool_size = 50
timeout = 60

[memory]
capacity = 10000000
cache_size_mb = 1024
compression = true
encryption = true

[security]
auth_enabled = true
rate_limiting_enabled = true
max_requests_per_minute = 10000

[performance]
monitoring_enabled = true
cache_enabled = true
cache_size_mb = 512
compression_enabled = true
```

#### Testing Configuration

```toml
# config/test.toml
[system]
debug = true
log_level = "warn"

[database]
url = ":memory:"  # In-memory SQLite for tests
pool_size = 1

[memory]
capacity = 1000
cache_size_mb = 16

[security]
auth_enabled = false

[performance]
monitoring_enabled = false
cache_enabled = false
```

## Component-Specific Configuration

### Character Ingestion Configuration

```toml
[character_ingestion]
enabled = true
buffer_size = 1024
encoding = "utf-8"
normalization = "nfc"
filter_control_chars = true
max_input_size_mb = 10

[character_ingestion.prediction]
model_type = "lstm"
context_window = 256
prediction_threshold = 0.5
top_k = 5
temperature = 0.8
```

### Segment Discovery Configuration

```toml
[segment_discovery]
enabled = true
algorithm = "adaptive_bpe"
min_frequency = 2
max_segment_length = 50
vocabulary_size = 50000
merge_threshold = 0.5
update_frequency = 1000

[segment_discovery.adaptive_bpe]
initial_vocab_size = 1000
growth_factor = 1.5
pruning_threshold = 0.1
recompute_interval = 10000
```

### Memory System Configuration

```toml
[memory_system]
enabled = true
storage_backend = "postgresql"  # or "sqlite"
index_type = "gin"  # PostgreSQL full-text search
search_algorithm = "bm25"

[memory_system.types]
semantic_enabled = true
episodic_enabled = true
procedural_enabled = true
pattern_enabled = true

[memory_system.consolidation]
enabled = true
algorithm = "importance_based"
threshold = 0.8
batch_size = 1000
interval = 3600
```

### Concept Graph Configuration

```toml
[concept_graph]
enabled = true
storage_backend = "neo4j"  # or "postgresql"
max_nodes = 1000000
max_edges = 10000000
clustering_enabled = true

[concept_graph.algorithms]
similarity_algorithm = "cosine"
clustering_algorithm = "hierarchical"
community_detection = "louvain"
centrality_algorithm = "pagerank"

[concept_graph.pruning]
enabled = true
min_edge_weight = 0.1
max_node_degree = 1000
pruning_interval = 86400
```

### Simulation Engine Configuration

```toml
[simulation_engine]
enabled = true
max_concurrent_simulations = 10
max_simulation_steps = 1000
timeout_seconds = 300
branching_factor = 3

[simulation_engine.algorithms]
scenario_generation = "monte_carlo"
outcome_prediction = "bayesian"
confidence_calculation = "ensemble"
```

## API Configuration

### REST API Configuration

```toml
[api]
enabled = true
version = "v1"
base_path = "/api"
cors_enabled = true
cors_origins = ["*"]
cors_methods = ["GET", "POST", "PUT", "DELETE"]
cors_headers = ["Content-Type", "Authorization"]

[api.endpoints]
health_check = "/health"
metrics = "/metrics"
documentation = "/docs"
openapi_spec = "/openapi.json"

[api.rate_limiting]
enabled = true
requests_per_minute = 1000
burst_size = 100
cleanup_interval = 60
```

### Authentication Configuration

```toml
[auth]
enabled = true
provider = "jwt"  # or "oauth2", "basic"
jwt_secret = "${JWT_SECRET}"
jwt_algorithm = "HS256"
jwt_expires_in = 3600
refresh_token_enabled = true
refresh_token_expires_in = 86400

[auth.oauth2]
provider = "google"  # or "github", "microsoft"
client_id = "${OAUTH2_CLIENT_ID}"
client_secret = "${OAUTH2_CLIENT_SECRET}"
redirect_uri = "${OAUTH2_REDIRECT_URI}"
scopes = ["openid", "email", "profile"]
```

## Monitoring and Observability

### Metrics Configuration

```toml
[metrics]
enabled = true
format = "prometheus"  # or "statsd", "json"
endpoint = "/metrics"
collection_interval = 30
retention_days = 30

[metrics.collectors]
system_metrics = true
application_metrics = true
custom_metrics = true
histogram_buckets = [0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0]
```

### Logging Configuration

```toml
[logging]
level = "info"
format = "json"  # or "text", "structured"
output = "stdout"  # or "file", "syslog"
timestamp_format = "rfc3339"
include_caller = true
include_stacktrace = false

[logging.file]
path = "/var/log/brain-ai/application.log"
max_size_mb = 100
max_files = 10
compress = true
rotation = "daily"  # or "size", "hourly"

[logging.structured]
service_name = "brain-ai"
service_version = "1.0.0"
environment = "production"
additional_fields = { datacenter = "us-east-1", cluster = "main" }
```

### Tracing Configuration

```toml
[tracing]
enabled = true
provider = "jaeger"  # or "zipkin", "otlp"
endpoint = "http://jaeger:14268/api/traces"
sample_rate = 0.1
service_name = "brain-ai"
tags = { version = "1.0.0", environment = "production" }
```

## Deployment Configuration

### Docker Configuration

```toml
[docker]
image = "brain-ai:latest"
registry = "ghcr.io/your-org"
build_context = "."
dockerfile = "Dockerfile"

[docker.resources]
memory_limit = "2g"
cpu_limit = "1000m"
memory_request = "1g"
cpu_request = "500m"
```

### Kubernetes Configuration

```toml
[kubernetes]
namespace = "brain-ai"
deployment_name = "brain-ai"
service_name = "brain-ai-service"
replicas = 3

[kubernetes.resources]
memory_limit = "2Gi"
cpu_limit = "1000m"
memory_request = "1Gi"
cpu_request = "500m"

[kubernetes.autoscaling]
enabled = true
min_replicas = 3
max_replicas = 20
target_cpu_utilization = 70
target_memory_utilization = 80
```

## Configuration Validation

### Schema Validation

```toml
[validation]
enabled = true
strict_mode = false
warn_on_unknown_fields = true
fail_on_missing_required = true

[validation.rules]
memory_capacity_min = 1000
memory_capacity_max = 100000000
port_range_min = 1024
port_range_max = 65535
```

### Environment Variable Substitution

```toml
# Use environment variables in configuration
[database]
url = "${DATABASE_URL}"
password = "${DB_PASSWORD}"

[auth]
jwt_secret = "${JWT_SECRET:-default_secret_for_dev}"  # With default value
```

## Configuration Management

### Configuration Loading

```bash
# Load configuration with environment override
brain-ai --config config/production.toml --env production

# Validate configuration
brain-ai --validate-config --config config/production.toml

# Show effective configuration
brain-ai --show-config --config config/production.toml
```

### Configuration Hot Reload

```toml
[config]
hot_reload_enabled = true
watch_files = ["config/brain.toml", "config/production.toml"]
reload_signal = "SIGHUP"
validation_on_reload = true
```

### Configuration Encryption

```toml
[config.encryption]
enabled = true
key_file = "/etc/brain-ai/config.key"
encrypted_fields = ["database.password", "auth.jwt_secret"]
algorithm = "aes-256-gcm"
```

## Best Practices

### Security Best Practices

1. **Never store secrets in configuration files**
2. **Use environment variables for sensitive data**
3. **Enable configuration validation**
4. **Regularly rotate secrets**
5. **Use encrypted configuration for production**

### Performance Best Practices

1. **Tune memory settings based on workload**
2. **Configure appropriate connection pools**
3. **Enable caching for read-heavy workloads**
4. **Monitor and adjust worker thread counts**
5. **Use compression for large datasets**

### Operational Best Practices

1. **Use environment-specific configurations**
2. **Enable comprehensive monitoring**
3. **Configure appropriate log levels**
4. **Set up configuration validation**
5. **Document configuration changes**

This configuration reference provides complete documentation for all Brain AI settings, enabling precise system tuning for any deployment scenario.
