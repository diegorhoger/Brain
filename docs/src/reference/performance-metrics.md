# Performance Metrics

Comprehensive reference for all Brain AI performance metrics, monitoring capabilities, and optimization guidelines.

## Metrics Overview

Brain AI provides extensive performance metrics across all system components to enable monitoring, alerting, and optimization:

- **System Metrics**: CPU, memory, disk, network utilization
- **Application Metrics**: Request rates, response times, throughput
- **Component Metrics**: Memory operations, learning pipeline, concept graph
- **Business Metrics**: Knowledge acquisition, insight generation, user engagement

## Metrics Collection

### Prometheus Integration

Brain AI exposes metrics in Prometheus format at `/metrics` endpoint:

```bash
# Access metrics endpoint
curl http://localhost:8080/metrics

# Example metrics output
# HELP brain_ai_requests_total Total number of requests
# TYPE brain_ai_requests_total counter
brain_ai_requests_total{method="GET",endpoint="/api/v1/memory/search",status="200"} 1234

# HELP brain_ai_response_time_seconds Response time in seconds
# TYPE brain_ai_response_time_seconds histogram
brain_ai_response_time_seconds_bucket{le="0.005"} 100
brain_ai_response_time_seconds_bucket{le="0.01"} 200
brain_ai_response_time_seconds_bucket{le="0.025"} 500
```

### Metrics Configuration

```toml
# config/metrics.toml
[metrics]
enabled = true
format = "prometheus"
endpoint = "/metrics"
collection_interval = 30
retention_days = 30

[metrics.labels]
service = "brain-ai"
version = "1.0.0"
environment = "production"
datacenter = "us-east-1"

[metrics.collectors]
system_metrics = true
application_metrics = true
custom_metrics = true
```

## System Metrics

### CPU Metrics

| Metric | Type | Description | Labels |
|--------|------|-------------|--------|
| `brain_ai_cpu_usage_percent` | Gauge | Current CPU usage percentage | `core` |
| `brain_ai_cpu_load_average` | Gauge | System load average | `period` (1m, 5m, 15m) |
| `brain_ai_cpu_context_switches_total` | Counter | Total context switches | - |
| `brain_ai_cpu_interrupts_total` | Counter | Total CPU interrupts | - |

```prometheus
# Example CPU metrics
brain_ai_cpu_usage_percent{core="0"} 45.2
brain_ai_cpu_usage_percent{core="1"} 52.1
brain_ai_cpu_load_average{period="1m"} 2.1
brain_ai_cpu_load_average{period="5m"} 1.8
```

### Memory Metrics

| Metric | Type | Description | Labels |
|--------|------|-------------|--------|
| `brain_ai_memory_usage_bytes` | Gauge | Current memory usage in bytes | `type` (heap, stack, cache) |
| `brain_ai_memory_usage_percent` | Gauge | Memory usage percentage | - |
| `brain_ai_memory_available_bytes` | Gauge | Available memory in bytes | - |
| `brain_ai_memory_allocations_total` | Counter | Total memory allocations | - |
| `brain_ai_memory_deallocations_total` | Counter | Total memory deallocations | - |
| `brain_ai_memory_gc_runs_total` | Counter | Total garbage collection runs | - |
| `brain_ai_memory_gc_duration_seconds` | Histogram | Garbage collection duration | - |

```prometheus
# Example memory metrics
brain_ai_memory_usage_bytes{type="heap"} 1073741824
brain_ai_memory_usage_bytes{type="cache"} 268435456
brain_ai_memory_usage_percent 65.5
brain_ai_memory_gc_duration_seconds_bucket{le="0.001"} 50
```

### Disk Metrics

| Metric | Type | Description | Labels |
|--------|------|-------------|--------|
| `brain_ai_disk_usage_bytes` | Gauge | Disk usage in bytes | `device`, `mount_point` |
| `brain_ai_disk_usage_percent` | Gauge | Disk usage percentage | `device` |
| `brain_ai_disk_io_operations_total` | Counter | Total disk I/O operations | `device`, `operation` (read, write) |
| `brain_ai_disk_io_bytes_total` | Counter | Total disk I/O bytes | `device`, `operation` |
| `brain_ai_disk_io_duration_seconds` | Histogram | Disk I/O operation duration | `device`, `operation` |

### Network Metrics

| Metric | Type | Description | Labels |
|--------|------|-------------|--------|
| `brain_ai_network_bytes_total` | Counter | Total network bytes | `interface`, `direction` (rx, tx) |
| `brain_ai_network_packets_total` | Counter | Total network packets | `interface`, `direction` |
| `brain_ai_network_errors_total` | Counter | Total network errors | `interface`, `type` |
| `brain_ai_network_connections_active` | Gauge | Active network connections | `protocol` |

## Application Metrics

### HTTP Request Metrics

| Metric | Type | Description | Labels |
|--------|------|-------------|--------|
| `brain_ai_http_requests_total` | Counter | Total HTTP requests | `method`, `endpoint`, `status` |
| `brain_ai_http_request_duration_seconds` | Histogram | HTTP request duration | `method`, `endpoint` |
| `brain_ai_http_request_size_bytes` | Histogram | HTTP request size | `method`, `endpoint` |
| `brain_ai_http_response_size_bytes` | Histogram | HTTP response size | `method`, `endpoint` |
| `brain_ai_http_concurrent_requests` | Gauge | Concurrent HTTP requests | - |

```prometheus
# Example HTTP metrics
brain_ai_http_requests_total{method="POST",endpoint="/api/v1/learn",status="200"} 5432
brain_ai_http_request_duration_seconds_bucket{method="GET",endpoint="/api/v1/memory/search",le="0.05"} 1000
```

### Authentication Metrics

| Metric | Type | Description | Labels |
|--------|------|-------------|--------|
| `brain_ai_auth_attempts_total` | Counter | Total authentication attempts | `result` (success, failure) |
| `brain_ai_auth_token_validations_total` | Counter | Total token validations | `result` |
| `brain_ai_auth_active_sessions` | Gauge | Active user sessions | - |
| `brain_ai_auth_session_duration_seconds` | Histogram | User session duration | - |

### Rate Limiting Metrics

| Metric | Type | Description | Labels |
|--------|------|-------------|--------|
| `brain_ai_rate_limit_requests_total` | Counter | Total rate limited requests | `limit_type` |
| `brain_ai_rate_limit_current_usage` | Gauge | Current rate limit usage | `user_id`, `limit_type` |
| `brain_ai_rate_limit_resets_total` | Counter | Total rate limit resets | `limit_type` |

## Component Metrics

### Memory System Metrics

| Metric | Type | Description | Labels |
|--------|------|-------------|--------|
| `brain_ai_memories_total` | Gauge | Total number of memories | `type` (semantic, episodic, procedural) |
| `brain_ai_memory_operations_total` | Counter | Total memory operations | `operation` (store, retrieve, update, delete) |
| `brain_ai_memory_operation_duration_seconds` | Histogram | Memory operation duration | `operation` |
| `brain_ai_memory_search_queries_total` | Counter | Total memory search queries | `type` |
| `brain_ai_memory_search_duration_seconds` | Histogram | Memory search duration | - |
| `brain_ai_memory_search_results_count` | Histogram | Number of search results | - |
| `brain_ai_memory_consolidation_runs_total` | Counter | Total consolidation runs | - |
| `brain_ai_memory_consolidation_duration_seconds` | Histogram | Consolidation duration | - |
| `brain_ai_memory_cache_hits_total` | Counter | Memory cache hits | - |
| `brain_ai_memory_cache_misses_total` | Counter | Memory cache misses | - |

```prometheus
# Example memory system metrics
brain_ai_memories_total{type="semantic"} 45231
brain_ai_memories_total{type="episodic"} 12456
brain_ai_memory_operations_total{operation="store"} 98765
brain_ai_memory_search_duration_seconds_bucket{le="0.01"} 500
```

### Learning Pipeline Metrics

| Metric | Type | Description | Labels |
|--------|------|-------------|--------|
| `brain_ai_learning_queue_size` | Gauge | Learning queue current size | - |
| `brain_ai_learning_queue_max_size` | Gauge | Learning queue maximum size | - |
| `brain_ai_learning_tasks_total` | Counter | Total learning tasks processed | `status` (success, failure) |
| `brain_ai_learning_task_duration_seconds` | Histogram | Learning task duration | `type` |
| `brain_ai_learning_throughput_items_per_second` | Gauge | Learning throughput | - |
| `brain_ai_learning_workers_active` | Gauge | Active learning workers | - |
| `brain_ai_learning_errors_total` | Counter | Total learning errors | `error_type` |

### Segment Discovery Metrics

| Metric | Type | Description | Labels |
|--------|------|-------------|--------|
| `brain_ai_segments_discovered_total` | Counter | Total segments discovered | - |
| `brain_ai_segment_vocabulary_size` | Gauge | Current vocabulary size | - |
| `brain_ai_segment_discovery_duration_seconds` | Histogram | Segment discovery duration | - |
| `brain_ai_bpe_merges_total` | Counter | Total BPE merges performed | - |
| `brain_ai_segment_frequency_distribution` | Histogram | Segment frequency distribution | - |

### Concept Graph Metrics

| Metric | Type | Description | Labels |
|--------|------|-------------|--------|
| `brain_ai_concept_nodes_total` | Gauge | Total concept nodes | - |
| `brain_ai_concept_edges_total` | Gauge | Total concept edges | - |
| `brain_ai_concept_operations_total` | Counter | Total concept operations | `operation` (create, update, delete, query) |
| `brain_ai_concept_query_duration_seconds` | Histogram | Concept query duration | `query_type` |
| `brain_ai_concept_clustering_runs_total` | Counter | Total clustering runs | - |
| `brain_ai_concept_clustering_duration_seconds` | Histogram | Clustering duration | - |
| `brain_ai_concept_relationships_strength` | Histogram | Relationship strength distribution | - |

```prometheus
# Example concept graph metrics
brain_ai_concept_nodes_total 128445
brain_ai_concept_edges_total 456789
brain_ai_concept_operations_total{operation="query"} 23456
```

### Simulation Engine Metrics

| Metric | Type | Description | Labels |
|--------|------|-------------|--------|
| `brain_ai_simulations_total` | Counter | Total simulations run | `status` (success, failure, timeout) |
| `brain_ai_simulation_duration_seconds` | Histogram | Simulation duration | - |
| `brain_ai_simulation_steps_total` | Histogram | Number of simulation steps | - |
| `brain_ai_simulation_queue_size` | Gauge | Simulation queue size | - |
| `brain_ai_simulation_accuracy_score` | Histogram | Simulation accuracy scores | - |
| `brain_ai_simulation_confidence_score` | Histogram | Simulation confidence scores | - |

## Database Metrics

### Connection Pool Metrics

| Metric | Type | Description | Labels |
|--------|------|-------------|--------|
| `brain_ai_db_connections_active` | Gauge | Active database connections | `database` |
| `brain_ai_db_connections_idle` | Gauge | Idle database connections | `database` |
| `brain_ai_db_connections_max` | Gauge | Maximum database connections | `database` |
| `brain_ai_db_connection_wait_duration_seconds` | Histogram | Connection wait duration | `database` |
| `brain_ai_db_connection_lifetime_seconds` | Histogram | Connection lifetime | `database` |

### Query Performance Metrics

| Metric | Type | Description | Labels |
|--------|------|-------------|--------|
| `brain_ai_db_queries_total` | Counter | Total database queries | `database`, `operation` (select, insert, update, delete) |
| `brain_ai_db_query_duration_seconds` | Histogram | Database query duration | `database`, `operation` |
| `brain_ai_db_slow_queries_total` | Counter | Total slow queries | `database` |
| `brain_ai_db_query_errors_total` | Counter | Total query errors | `database`, `error_type` |
| `brain_ai_db_transactions_total` | Counter | Total database transactions | `database`, `status` |

```prometheus
# Example database metrics
brain_ai_db_connections_active{database="brain_ai"} 15
brain_ai_db_connections_idle{database="brain_ai"} 5
brain_ai_db_queries_total{database="brain_ai",operation="select"} 98765
```

## Business Metrics

### Knowledge Acquisition Metrics

| Metric | Type | Description | Labels |
|--------|------|-------------|--------|
| `brain_ai_knowledge_items_total` | Gauge | Total knowledge items | `type` |
| `brain_ai_knowledge_growth_rate` | Gauge | Knowledge growth rate per hour | - |
| `brain_ai_learning_effectiveness_score` | Gauge | Learning effectiveness score | - |
| `brain_ai_knowledge_retention_rate` | Gauge | Knowledge retention rate | - |

### Insight Generation Metrics

| Metric | Type | Description | Labels |
|--------|------|-------------|--------|
| `brain_ai_insights_generated_total` | Counter | Total insights generated | `type`, `confidence_level` |
| `brain_ai_insight_quality_score` | Histogram | Insight quality scores | - |
| `brain_ai_insight_generation_duration_seconds` | Histogram | Insight generation duration | - |
| `brain_ai_insights_validated_total` | Counter | Total insights validated | `result` |

### User Engagement Metrics

| Metric | Type | Description | Labels |
|--------|------|-------------|--------|
| `brain_ai_active_users` | Gauge | Currently active users | - |
| `brain_ai_user_sessions_total` | Counter | Total user sessions | - |
| `brain_ai_user_queries_total` | Counter | Total user queries | `user_type` |
| `brain_ai_user_satisfaction_score` | Histogram | User satisfaction scores | - |

## Performance Benchmarks

### Baseline Performance Targets

| Component | Metric | Target | Acceptable | Critical |
|-----------|--------|--------|------------|----------|
| API Response Time | 95th percentile | <50ms | <100ms | >500ms |
| Memory Operations | Throughput | >5000/sec | >1000/sec | <500/sec |
| Learning Pipeline | Latency | <5sec | <10sec | >30sec |
| Concept Queries | Response Time | <10ms | <50ms | >200ms |
| Database Queries | 95th percentile | <10ms | <50ms | >200ms |

### Scalability Benchmarks

```prometheus
# Load test results metrics
brain_ai_load_test_requests_per_second 1000
brain_ai_load_test_response_time_p95_ms 45
brain_ai_load_test_error_rate_percent 0.1
brain_ai_load_test_cpu_usage_percent 70
brain_ai_load_test_memory_usage_percent 65
```

## Alerting Rules

### Critical Alerts

```yaml
# prometheus/alerts.yml
groups:
- name: brain-ai-critical
  rules:
  - alert: HighResponseTime
    expr: histogram_quantile(0.95, brain_ai_http_request_duration_seconds_bucket) > 0.5
    for: 2m
    annotations:
      summary: "High API response times detected"
      
  - alert: HighErrorRate
    expr: rate(brain_ai_http_requests_total{status=~"5.."}[5m]) > 0.05
    for: 1m
    annotations:
      summary: "High error rate detected"
      
  - alert: MemorySystemOverload
    expr: brain_ai_memory_operations_total / brain_ai_memory_operations_capacity > 0.9
    for: 2m
    annotations:
      summary: "Memory system approaching capacity"
      
  - alert: LearningPipelineStalled
    expr: rate(brain_ai_learning_tasks_total[5m]) == 0
    for: 5m
    annotations:
      summary: "Learning pipeline has stalled"
```

### Warning Alerts

```yaml
- name: brain-ai-warnings
  rules:
  - alert: HighCPUUsage
    expr: brain_ai_cpu_usage_percent > 80
    for: 5m
    annotations:
      summary: "High CPU usage detected"
      
  - alert: HighMemoryUsage
    expr: brain_ai_memory_usage_percent > 85
    for: 5m
    annotations:
      summary: "High memory usage detected"
      
  - alert: DatabaseSlowQueries
    expr: rate(brain_ai_db_slow_queries_total[5m]) > 0.1
    for: 2m
    annotations:
      summary: "Database slow queries detected"
```

## Monitoring Dashboards

### System Overview Dashboard

```json
{
  "dashboard": {
    "title": "Brain AI System Overview",
    "panels": [
      {
        "title": "Request Rate",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(brain_ai_http_requests_total[5m])",
            "legendFormat": "{{method}} {{endpoint}}"
          }
        ]
      },
      {
        "title": "Response Time",
        "type": "graph",
        "targets": [
          {
            "expr": "histogram_quantile(0.95, brain_ai_http_request_duration_seconds_bucket)",
            "legendFormat": "95th percentile"
          }
        ]
      },
      {
        "title": "System Resources",
        "type": "graph",
        "targets": [
          {
            "expr": "brain_ai_cpu_usage_percent",
            "legendFormat": "CPU Usage %"
          },
          {
            "expr": "brain_ai_memory_usage_percent",
            "legendFormat": "Memory Usage %"
          }
        ]
      }
    ]
  }
}
```

### Component-Specific Dashboards

#### Memory System Dashboard

- Memory operations throughput
- Memory search performance
- Memory consolidation metrics
- Cache hit/miss ratios

#### Learning Pipeline Dashboard

- Learning queue depth
- Task processing rates
- Worker utilization
- Error rates by component

#### Concept Graph Dashboard

- Graph size and growth
- Query performance
- Clustering effectiveness
- Relationship quality metrics

## Performance Optimization

### Metrics-Driven Optimization

1. **Identify Bottlenecks**: Use metrics to identify performance bottlenecks
2. **Set Baselines**: Establish performance baselines for comparison
3. **Monitor Trends**: Track performance trends over time
4. **Capacity Planning**: Use metrics for capacity planning
5. **Optimization Validation**: Validate optimizations with metrics

### Key Performance Indicators (KPIs)

```prometheus
# Primary KPIs for Brain AI
brain_ai_kpi_system_availability_percent 99.9
brain_ai_kpi_api_response_time_p95_ms 45
brain_ai_kpi_learning_throughput_items_per_hour 360000
brain_ai_kpi_memory_search_accuracy_percent 95.5
brain_ai_kpi_concept_discovery_rate_per_hour 150
brain_ai_kpi_user_satisfaction_score 4.2
```

This comprehensive metrics reference enables effective monitoring, alerting, and optimization of Brain AI performance across all system components.
