# Monitoring & Logging

This guide covers comprehensive monitoring, logging, and observability setup for Brain AI in production environments.

## Performance Monitoring

### Built-in Performance Monitor

Brain AI includes a comprehensive performance monitoring system:

```rust
// Enable performance monitoring in configuration
[performance]
enable_monitoring = true
metrics_interval = 60
alert_thresholds = { cpu = 0.8, memory = 0.9, disk = 0.95 }
profiling_enabled = false
```

### System Metrics Collection

The performance monitor automatically collects:

- **System Resources**: CPU usage, memory consumption, disk I/O
- **Component Performance**: Processing times, throughput rates
- **Memory System**: Memory formation rate, consolidation efficiency
- **Concept Graph**: Node creation rate, relationship discovery
- **API Performance**: Request latency, error rates

### Performance API Endpoints

```bash
# Get current performance snapshot
curl http://localhost:8080/api/v1/performance/snapshot \
  -H "Authorization: Bearer $TOKEN"

# Get performance history
curl http://localhost:8080/api/v1/performance/history?duration=1h \
  -H "Authorization: Bearer $TOKEN"

# Identify bottlenecks
curl http://localhost:8080/api/v1/performance/bottlenecks \
  -H "Authorization: Bearer $TOKEN"

# Get optimization recommendations
curl http://localhost:8080/api/v1/performance/recommendations \
  -H "Authorization: Bearer $TOKEN"
```

## Logging System

### Log Levels and Configuration

```toml
[logging]
level = "info"                    # debug, info, warn, error
format = "json"                   # json, pretty
output = "both"                   # console, file, both

[logging.file]
path = "/app/logs/brain-ai.log"
max_size = "100MB"
max_files = 10
compress = true

[logging.structured]
include_timestamp = true
include_level = true
include_target = true
include_span_info = true
```

### Log Categories

Brain AI logs are categorized by component:

```bash
# System-level logs
2024-01-15T10:30:00Z INFO [brain_ai::system] System startup completed
2024-01-15T10:30:01Z INFO [brain_ai::api] Server listening on 0.0.0.0:8080

# Memory system logs
2024-01-15T10:30:15Z DEBUG [brain_ai::memory] Memory consolidation started
2024-01-15T10:30:16Z INFO [brain_ai::memory] Consolidated 150 memories

# Learning logs
2024-01-15T10:30:30Z INFO [brain_ai::learning] Pattern discovered: text_structure
2024-01-15T10:30:31Z DEBUG [brain_ai::concept_graph] New concept added: programming

# Performance logs
2024-01-15T10:30:45Z WARN [brain_ai::performance] High CPU usage detected: 85%
2024-01-15T10:30:46Z INFO [brain_ai::performance] Performance alert triggered
```

### Structured Logging

Example structured log entry:

```json
{
  "timestamp": "2024-01-15T10:30:00Z",
  "level": "INFO",
  "target": "brain_ai::memory",
  "message": "Memory formation completed",
  "fields": {
    "memory_id": "mem_12345",
    "content_type": "text",
    "processing_time_ms": 45,
    "confidence": 0.87,
    "session_id": "sess_67890"
  },
  "span": {
    "name": "process_input",
    "id": "span_abc123"
  }
}
```

## Health Checks

### Built-in Health Endpoints

```bash
# Basic health check
curl http://localhost:8080/api/v1/health

# Detailed health check
curl http://localhost:8080/api/v1/health/detailed

# Component-specific health
curl http://localhost:8080/api/v1/health/memory
curl http://localhost:8080/api/v1/health/concept-graph
```

### Health Check Response

```json
{
  "status": "healthy",
  "timestamp": "2024-01-15T10:30:00Z",
  "uptime": "2h 15m 30s",
  "version": "1.0.0",
  "components": {
    "memory_system": {
      "status": "healthy",
      "memory_count": 12450,
      "last_consolidation": "2024-01-15T10:15:00Z"
    },
    "concept_graph": {
      "status": "healthy",
      "node_count": 1250,
      "edge_count": 3200
    },
    "character_ingestion": {
      "status": "healthy",
      "processed_chars": 1500000
    },
    "performance_monitor": {
      "status": "healthy",
      "cpu_usage": 0.45,
      "memory_usage": 0.67
    }
  }
}
```

## Metrics Collection

### Prometheus Integration

Brain AI exposes Prometheus-compatible metrics:

```bash
# Metrics endpoint
curl http://localhost:8080/metrics
```

Example metrics:

```prometheus
# System metrics
brain_ai_cpu_usage_percent 45.2
brain_ai_memory_usage_bytes 1073741824
brain_ai_uptime_seconds 8130

# Component metrics
brain_ai_memory_total 12450
brain_ai_memory_formation_rate 2.5
brain_ai_concept_nodes_total 1250
brain_ai_concept_edges_total 3200

# API metrics
brain_ai_http_requests_total{method="GET",status="200"} 1500
brain_ai_http_request_duration_seconds{method="POST"} 0.045
brain_ai_http_errors_total{status="500"} 2

# Performance metrics
brain_ai_processing_time_seconds{component="memory"} 0.023
brain_ai_throughput_operations_per_second{component="concept_graph"} 15.7
```

### Grafana Dashboard

Example Grafana dashboard configuration:

```json
{
  "dashboard": {
    "title": "Brain AI Monitoring",
    "panels": [
      {
        "title": "System Resources",
        "type": "graph",
        "targets": [
          {
            "expr": "brain_ai_cpu_usage_percent",
            "legendFormat": "CPU Usage"
          },
          {
            "expr": "brain_ai_memory_usage_bytes / 1024 / 1024 / 1024",
            "legendFormat": "Memory Usage (GB)"
          }
        ]
      },
      {
        "title": "Memory System",
        "type": "stat",
        "targets": [
          {
            "expr": "brain_ai_memory_total",
            "legendFormat": "Total Memories"
          },
          {
            "expr": "rate(brain_ai_memory_formation_rate[5m])",
            "legendFormat": "Formation Rate"
          }
        ]
      }
    ]
  }
}
```

## Alerting

### Alert Configuration

```toml
[alerts]
enabled = true
webhook_url = "https://hooks.slack.com/your-webhook"
email_recipients = ["admin@yourcompany.com"]

[alerts.thresholds]
cpu_usage = 0.8
memory_usage = 0.9
disk_usage = 0.95
error_rate = 0.05
response_time = 2.0

[alerts.rules]
high_cpu = { threshold = 0.8, duration = "5m", severity = "warning" }
high_memory = { threshold = 0.9, duration = "3m", severity = "critical" }
high_error_rate = { threshold = 0.05, duration = "2m", severity = "critical" }
```

### Alert Examples

```bash
# CPU usage alert
{
  "alert": "high_cpu_usage",
  "severity": "warning",
  "message": "CPU usage is 85% for 5 minutes",
  "timestamp": "2024-01-15T10:30:00Z",
  "metrics": {
    "cpu_usage": 0.85,
    "threshold": 0.8
  }
}

# Memory formation stalled alert
{
  "alert": "memory_formation_stalled",
  "severity": "critical",
  "message": "No new memories formed in 10 minutes",
  "timestamp": "2024-01-15T10:30:00Z",
  "metrics": {
    "last_memory_time": "2024-01-15T10:20:00Z",
    "formation_rate": 0.0
  }
}
```

## Log Aggregation

### ELK Stack Integration

#### Logstash Configuration

```ruby
input {
  file {
    path => "/app/logs/brain-ai.log"
    codec => "json"
    type => "brain-ai"
  }
}

filter {
  if [type] == "brain-ai" {
    date {
      match => [ "timestamp", "ISO8601" ]
    }
    
    mutate {
      add_field => { "service" => "brain-ai" }
    }
  }
}

output {
  elasticsearch {
    hosts => ["elasticsearch:9200"]
    index => "brain-ai-logs-%{+YYYY.MM.dd}"
  }
}
```

#### Elasticsearch Index Template

```json
{
  "index_patterns": ["brain-ai-logs-*"],
  "mappings": {
    "properties": {
      "timestamp": { "type": "date" },
      "level": { "type": "keyword" },
      "target": { "type": "keyword" },
      "message": { "type": "text" },
      "fields": {
        "properties": {
          "memory_id": { "type": "keyword" },
          "processing_time_ms": { "type": "integer" },
          "confidence": { "type": "float" }
        }
      }
    }
  }
}
```

### Fluentd Configuration

```ruby
<source>
  @type tail
  path /app/logs/brain-ai.log
  pos_file /var/log/fluentd/brain-ai.log.pos
  tag brain-ai
  format json
</source>

<match brain-ai>
  @type elasticsearch
  host elasticsearch
  port 9200
  index_name brain-ai-logs
  type_name _doc
</match>
```

## Distributed Tracing

### OpenTelemetry Integration

```rust
// Enable tracing in configuration
[tracing]
enabled = true
service_name = "brain-ai"
endpoint = "http://jaeger:14268/api/traces"
sample_rate = 0.1

[tracing.attributes]
environment = "production"
version = "1.0.0"
```

### Trace Examples

```bash
# Memory formation trace
Trace: memory_formation_flow
├── Span: character_ingestion (2ms)
├── Span: segment_discovery (15ms)
├── Span: pattern_analysis (8ms)
├── Span: memory_creation (5ms)
└── Span: concept_graph_update (3ms)
Total: 33ms

# API request trace
Trace: api_request_/api/v1/learn
├── Span: authentication (1ms)
├── Span: request_validation (2ms)
├── Span: memory_formation_flow (33ms)
├── Span: response_serialization (1ms)
└── Span: logging (0.5ms)
Total: 37.5ms
```

## Docker Monitoring

### Container Monitoring

```yaml
services:
  brain-ai:
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"
    labels:
      - "monitoring.enable=true"
      - "monitoring.port=8080"
```

### Docker Compose with Monitoring Stack

```yaml
version: '3.8'

services:
  brain-ai:
    image: brain-ai:latest
    ports:
      - "8080:8080"
    environment:
      - ENABLE_PERFORMANCE_MONITORING=true
    volumes:
      - brain_logs:/app/logs

  prometheus:
    image: prom/prometheus
    ports:
      - "9090:9090"
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml
      - prometheus_data:/prometheus

  grafana:
    image: grafana/grafana
    ports:
      - "3000:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
    volumes:
      - grafana_data:/var/lib/grafana

  elasticsearch:
    image: docker.elastic.co/elasticsearch/elasticsearch:8.5.0
    environment:
      - discovery.type=single-node
      - "ES_JAVA_OPTS=-Xms1g -Xmx1g"
    volumes:
      - es_data:/usr/share/elasticsearch/data

  kibana:
    image: docker.elastic.co/kibana/kibana:8.5.0
    ports:
      - "5601:5601"
    depends_on:
      - elasticsearch

volumes:
  brain_logs:
  prometheus_data:
  grafana_data:
  es_data:
```

## Troubleshooting

### Common Monitoring Issues

1. **High Memory Usage**:
   ```bash
   # Check memory consumption
   curl http://localhost:8080/api/v1/performance/snapshot | jq '.memory'
   
   # Trigger memory cleanup
   curl -X POST http://localhost:8080/api/v1/system/cleanup
   ```

2. **Performance Degradation**:
   ```bash
   # Get bottleneck analysis
   curl http://localhost:8080/api/v1/performance/bottlenecks
   
   # Check component health
   curl http://localhost:8080/api/v1/health/detailed
   ```

3. **Log Volume Issues**:
   ```bash
   # Rotate logs manually
   docker exec brain-ai logrotate /etc/logrotate.conf
   
   # Compress old logs
   find /app/logs -name "*.log" -mtime +7 -exec gzip {} \;
   ```

### Performance Optimization

Based on monitoring data:

```bash
# Increase memory capacity if formation rate is high
curl -X PUT http://localhost:8080/api/v1/config \
  -d '{"memory": {"capacity": 2000000}}'

# Adjust consolidation threshold if CPU usage is high
curl -X PUT http://localhost:8080/api/v1/config \
  -d '{"memory": {"consolidation_threshold": 0.9}}'

# Enable performance profiling for detailed analysis
curl -X POST http://localhost:8080/api/v1/performance/profiling/start
```

This comprehensive monitoring guide provides all the tools needed to maintain visibility into Brain AI's performance and health in production environments.
