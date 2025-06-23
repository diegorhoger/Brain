# Troubleshooting

Comprehensive troubleshooting guide for diagnosing and resolving common issues in Brain AI production deployments.

## Overview

This guide covers systematic approaches to identifying, diagnosing, and resolving issues across all Brain AI components:

- **System-Level Issues**: Infrastructure, networking, storage
- **Application Issues**: Memory system, concept graph, learning pipeline
- **Performance Issues**: Latency, throughput, resource utilization
- **Data Issues**: Corruption, consistency, backup/recovery
- **Integration Issues**: API, authentication, external services

## Diagnostic Tools

### Health Check System

```bash
# Comprehensive system health check
./scripts/health-check.sh --comprehensive

# Component-specific checks
./scripts/health-check.sh --component memory
./scripts/health-check.sh --component concepts
./scripts/health-check.sh --component learning
./scripts/health-check.sh --component api

# Output format:
# ✓ Memory System: Healthy (response time: 12ms)
# ✓ Concept Graph: Healthy (nodes: 45,231, edges: 128,445)
# ✗ Learning Pipeline: Degraded (queue depth: 15,000/10,000)
# ✓ API Endpoints: Healthy (avg response: 23ms)
```

### Log Analysis

```bash
# Real-time log monitoring
tail -f /var/log/brain-ai/application.log | grep -E "(ERROR|WARN|PANIC)"

# Structured log analysis
./scripts/analyze-logs.sh --since "1 hour ago" --level error

# Common log patterns
./scripts/find-log-patterns.sh --pattern "memory_allocation_failed"
./scripts/find-log-patterns.sh --pattern "database_connection_timeout"
```

### Performance Monitoring

```bash
# Real-time performance dashboard
./scripts/performance-dashboard.sh

# Resource utilization
htop
iotop
nethogs

# Application-specific metrics
./scripts/brain-ai-metrics.sh --component all --interval 5
```

## Common Issues and Solutions

### 1. Service Startup Issues

#### Issue: Brain AI fails to start

**Symptoms:**
```bash
# Service status shows failed
systemctl status brain-ai
● brain-ai.service - Brain AI Cognitive System
   Loaded: loaded (/etc/systemd/system/brain-ai.service; enabled)
   Active: failed (Result: exit-code) since Mon 2024-01-01 10:00:00 UTC
```

**Diagnosis:**
```bash
# Check service logs
journalctl -u brain-ai.service -n 50

# Check configuration
./scripts/validate-config.sh

# Check dependencies
./scripts/check-dependencies.sh
```

**Solutions:**

1. **Configuration Issues:**
```bash
# Validate configuration file
./scripts/validate-config.sh
# Fix: Correct invalid configuration parameters

# Check environment variables
env | grep BRAIN_AI
# Fix: Set required environment variables
export ANTHROPIC_API_KEY=your_key_here
```

2. **Database Connection Issues:**
```bash
# Test database connectivity
./scripts/test-database-connection.sh
# Fix: Verify database is running and accessible
systemctl start postgresql
./scripts/create-database.sh
```

3. **Port Conflicts:**
```bash
# Check port availability
netstat -tlnp | grep 8080
# Fix: Kill conflicting process or change port
sudo kill -9 <pid>
# or update config to use different port
```

### 2. Memory System Issues

#### Issue: High memory usage or memory leaks

**Symptoms:**
```bash
# High memory usage
free -h
              total        used        free      shared  buff/cache   available
Mem:           8.0G        7.2G        100M         0B        700M        600M

# Memory allocation errors in logs
grep "memory allocation failed" /var/log/brain-ai/application.log
```

**Diagnosis:**
```bash
# Memory usage analysis
./scripts/analyze-memory-usage.sh

# Check for memory leaks
valgrind --leak-check=full ./target/release/brain-ai

# Monitor memory patterns
./scripts/monitor-memory-patterns.sh --duration 300
```

**Solutions:**

1. **Increase Memory Limits:**
```toml
# config/brain.toml
[memory]
capacity = 2000000  # Increase from 1000000
cache_size_mb = 1024  # Increase cache
```

2. **Enable Memory Compression:**
```toml
[performance]
memory_compression = true
compression_algorithm = "lz4"
```

3. **Implement Memory Cleanup:**
```bash
# Manual memory cleanup
curl -X POST http://localhost:8080/api/v1/admin/cleanup/memory

# Automated cleanup configuration
[memory.cleanup]
enabled = true
interval_minutes = 30
threshold_percent = 80
```

### 3. Database Issues

#### Issue: Database connection timeouts

**Symptoms:**
```bash
# Connection timeout errors
grep "database connection timeout" /var/log/brain-ai/application.log
2024-01-01T10:00:00Z ERROR database connection timeout after 30s
```

**Diagnosis:**
```bash
# Check database status
systemctl status postgresql

# Test connection
psql -h localhost -U brain_ai -d brain_ai -c "SELECT 1;"

# Check connection pool
./scripts/check-connection-pool.sh
```

**Solutions:**

1. **Increase Connection Pool:**
```toml
[database]
connection_pool_size = 50  # Increase from 20
connection_timeout_seconds = 60  # Increase timeout
```

2. **Database Optimization:**
```sql
-- Increase PostgreSQL connection limits
ALTER SYSTEM SET max_connections = 200;
SELECT pg_reload_conf();

-- Optimize queries
ANALYZE;
REINDEX DATABASE brain_ai;
```

3. **Connection Pool Monitoring:**
```bash
# Monitor connection pool health
./scripts/monitor-connection-pool.sh --continuous
```

### 4. API Issues

#### Issue: High API response times

**Symptoms:**
```bash
# Slow API responses
curl -w "@curl-format.txt" -s -o /dev/null http://localhost:8080/api/v1/health
     time_namelookup:  0.001
        time_connect:  0.002
     time_appconnect:  0.000
    time_pretransfer:  0.002
       time_redirect:  0.000
  time_starttransfer:  2.456  # High response time
          time_total:  2.456
```

**Diagnosis:**
```bash
# API performance analysis
./scripts/analyze-api-performance.sh --endpoint /api/v1/memory/search

# Check request queue depth
./scripts/check-request-queue.sh

# Monitor database query performance
./scripts/monitor-slow-queries.sh
```

**Solutions:**

1. **Increase Worker Threads:**
```toml
[system]
worker_threads = 16  # Increase from 8
max_concurrent_requests = 200  # Increase from 100
```

2. **Enable Caching:**
```toml
[cache]
enabled = true
size_mb = 512
ttl_seconds = 300
```

3. **Database Query Optimization:**
```sql
-- Add missing indexes
CREATE INDEX CONCURRENTLY idx_memories_content_gin ON memories USING gin(to_tsvector('english', content));
CREATE INDEX CONCURRENTLY idx_concepts_name ON concepts(name);
```

### 5. Learning Pipeline Issues

#### Issue: Learning pipeline stalled or slow

**Symptoms:**
```bash
# Learning queue backing up
./scripts/check-learning-queue.sh
Learning queue depth: 15,000 items (threshold: 10,000)
Average processing time: 45s per item (expected: 5s)
```

**Diagnosis:**
```bash
# Learning pipeline analysis
./scripts/analyze-learning-pipeline.sh

# Check resource bottlenecks
./scripts/check-learning-resources.sh

# Monitor learning throughput
./scripts/monitor-learning-throughput.sh --duration 300
```

**Solutions:**

1. **Increase Learning Workers:**
```toml
[learning]
worker_count = 16  # Increase from 8
batch_size = 1000  # Increase batch size
parallel_processing = true
```

2. **Optimize Learning Parameters:**
```toml
[learning.segment_discovery]
min_frequency = 3  # Increase threshold
max_segment_length = 20  # Reduce complexity

[learning.concept_extraction]
similarity_threshold = 0.8  # Increase threshold
```

3. **Resource Allocation:**
```toml
[system]
learning_priority = "high"
learning_cpu_affinity = [0, 1, 2, 3]  # Dedicate CPU cores
```

## Error Code Reference

### System Errors (1000-1999)

| Code | Error | Cause | Solution |
|------|-------|-------|----------|
| 1001 | Configuration Invalid | Missing or invalid config | Validate config file |
| 1002 | Database Connection Failed | DB unavailable | Check DB status and connection |
| 1003 | Memory Allocation Failed | Insufficient memory | Increase memory or reduce usage |
| 1004 | File Permission Denied | Incorrect permissions | Fix file permissions |
| 1005 | Port Already In Use | Port conflict | Change port or kill conflicting process |

### Memory Errors (2000-2999)

| Code | Error | Cause | Solution |
|------|-------|-------|----------|
| 2001 | Memory Capacity Exceeded | Too many memories | Increase capacity or cleanup |
| 2002 | Memory Corruption Detected | Data integrity issue | Restore from backup |
| 2003 | Working Memory Full | High processing load | Increase working memory size |
| 2004 | Memory Consolidation Failed | Background process error | Check consolidation settings |

### Learning Errors (3000-3999)

| Code | Error | Cause | Solution |
|------|-------|-------|----------|
| 3001 | Segment Discovery Failed | Algorithm error | Check input data quality |
| 3002 | Concept Extraction Timeout | Processing too slow | Optimize parameters |
| 3003 | Learning Queue Overflow | High input rate | Increase queue size or workers |
| 3004 | Model Update Failed | Concurrent modification | Implement proper locking |

### API Errors (4000-4999)

| Code | Error | Cause | Solution |
|------|-------|-------|----------|
| 4001 | Authentication Failed | Invalid credentials | Check API key |
| 4002 | Rate Limit Exceeded | Too many requests | Implement backoff |
| 4003 | Request Timeout | Slow processing | Optimize query or increase timeout |
| 4004 | Invalid Request Format | Malformed JSON | Validate request format |

## Performance Troubleshooting

### High CPU Usage

**Diagnosis:**
```bash
# Identify CPU-intensive processes
top -p $(pgrep brain-ai)

# CPU profiling
perf record -g ./target/release/brain-ai
perf report
```

**Solutions:**
```bash
# Reduce CPU-intensive operations
[performance]
enable_background_processing = false
reduce_concept_analysis_frequency = true

# Scale horizontally
docker-compose up --scale brain-ai=3
```

### High Memory Usage

**Diagnosis:**
```bash
# Memory profiling
./scripts/memory-profile.sh --duration 300

# Check memory fragmentation
cat /proc/buddyinfo
```

**Solutions:**
```bash
# Memory optimization
[memory]
enable_compression = true
aggressive_cleanup = true
consolidation_frequency = "high"
```

### Disk I/O Issues

**Diagnosis:**
```bash
# Disk I/O monitoring
iotop -o -d 1

# Check disk usage
df -h
du -sh /var/lib/brain-ai/*
```

**Solutions:**
```bash
# I/O optimization
[storage]
use_ssd_optimizations = true
batch_write_size = 1000
fsync_frequency = "low"
```

## Network Troubleshooting

### Connection Issues

**Diagnosis:**
```bash
# Network connectivity
ping <database_host>
telnet <database_host> 5432

# Check firewall rules
iptables -L
ufw status
```

**Solutions:**
```bash
# Firewall configuration
sudo ufw allow 8080/tcp
sudo ufw allow from <trusted_ip> to any port 5432
```

### Load Balancer Issues

**Diagnosis:**
```bash
# Check load balancer status
curl -I http://load-balancer/health

# Backend health
for backend in backend1 backend2 backend3; do
  curl -I http://$backend:8080/health
done
```

**Solutions:**
```bash
# Update load balancer configuration
# Remove unhealthy backends
# Adjust health check parameters
```

## Recovery Procedures

### Automated Recovery

```bash
# Automatic recovery script
./scripts/auto-recovery.sh

# Recovery workflow:
# 1. Detect issue type
# 2. Apply appropriate fix
# 3. Validate recovery
# 4. Alert if manual intervention needed
```

### Manual Recovery Steps

1. **Identify Issue:**
```bash
./scripts/diagnose-issue.sh --comprehensive
```

2. **Stop Services:**
```bash
systemctl stop brain-ai
```

3. **Apply Fix:**
```bash
# Based on diagnosis results
./scripts/apply-fix.sh --issue-type <type>
```

4. **Validate Fix:**
```bash
./scripts/validate-fix.sh
```

5. **Restart Services:**
```bash
systemctl start brain-ai
./scripts/post-recovery-check.sh
```

## Monitoring and Alerting

### Critical Alerts

```yaml
# alertmanager/brain-ai-alerts.yml
groups:
- name: brain-ai-critical
  rules:
  - alert: ServiceDown
    expr: up{job="brain-ai"} == 0
    for: 1m
    annotations:
      summary: "Brain AI service is down"
      
  - alert: HighErrorRate
    expr: rate(http_requests_total{status=~"5.."}[5m]) > 0.1
    for: 2m
    annotations:
      summary: "High error rate detected"
      
  - alert: DatabaseConnectionFailed
    expr: brain_ai_database_connections_failed_total > 10
    for: 1m
    annotations:
      summary: "Database connection failures"
```

### Monitoring Dashboard

```bash
# Start monitoring dashboard
./scripts/start-monitoring-dashboard.sh

# Access at: http://localhost:3000
# Default credentials: admin/admin
```

## Emergency Procedures

### Service Recovery

```bash
# Emergency service restart
sudo systemctl stop brain-ai
sudo systemctl reset-failed brain-ai
sudo systemctl start brain-ai

# If restart fails, restore from backup
./scripts/emergency-restore.sh latest
```

### Data Recovery

```bash
# Emergency data recovery
./scripts/emergency-data-recovery.sh

# Steps:
# 1. Stop all services
# 2. Restore from latest backup
# 3. Verify data integrity
# 4. Restart services
# 5. Run health checks
```

### Escalation Procedures

1. **Level 1**: Automated recovery attempts
2. **Level 2**: On-call engineer notification
3. **Level 3**: Senior engineer escalation
4. **Level 4**: Management and vendor escalation

## Preventive Measures

### Regular Maintenance

```bash
# Weekly maintenance script
./scripts/weekly-maintenance.sh

# Includes:
# - Log rotation
# - Database optimization
# - Memory cleanup
# - Health checks
# - Backup verification
```

### Monitoring Best Practices

1. **Set up comprehensive monitoring**
2. **Configure appropriate alerting thresholds**
3. **Regular backup testing**
4. **Performance baseline establishment**
5. **Incident response documentation**

This troubleshooting guide provides systematic approaches to identify and resolve issues quickly, minimizing downtime and ensuring reliable Brain AI operations.
