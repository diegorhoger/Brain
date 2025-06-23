# Error Codes

Comprehensive reference for all Brain AI error codes, their meanings, causes, and recommended solutions.

## Error Code Format

Brain AI uses a structured error code system:

```
BRAIN-{CATEGORY}-{SEVERITY}-{CODE}
```

- **CATEGORY**: Component or subsystem (4 chars)
- **SEVERITY**: Error severity level (1 char)
- **CODE**: Specific error identifier (3 digits)

**Severity Levels:**
- `1` - Info/Warning
- `2` - Error (recoverable)
- `3` - Critical (service degraded)
- `4` - Fatal (service unavailable)

## System Errors (SYST)

### Configuration Errors (SYST-2-001 to SYST-2-099)

| Code | Error | Description | Cause | Solution |
|------|-------|-------------|-------|----------|
| SYST-2-001 | Configuration File Not Found | Cannot locate configuration file | Missing config file | Ensure config file exists at specified path |
| SYST-2-002 | Invalid Configuration Format | Configuration file format is invalid | Malformed TOML/JSON | Validate configuration file syntax |
| SYST-2-003 | Missing Required Configuration | Required configuration parameter missing | Incomplete config | Add missing required parameters |
| SYST-2-004 | Invalid Configuration Value | Configuration value is invalid | Wrong data type/range | Correct configuration value |
| SYST-2-005 | Environment Variable Missing | Required environment variable not set | Missing env var | Set required environment variable |
| SYST-2-006 | Configuration Validation Failed | Configuration failed validation rules | Invalid config values | Fix configuration according to schema |
| SYST-2-007 | Configuration Encryption Failed | Cannot decrypt encrypted configuration | Wrong key/corrupted file | Check encryption key and file integrity |

### System Resource Errors (SYST-3-100 to SYST-3-199)

| Code | Error | Description | Cause | Solution |
|------|-------|-------------|-------|----------|
| SYST-3-100 | Insufficient Memory | System running out of memory | High memory usage | Increase memory or reduce usage |
| SYST-3-101 | Memory Allocation Failed | Cannot allocate required memory | Memory exhaustion | Restart service or increase memory |
| SYST-3-102 | Disk Space Exhausted | Insufficient disk space | Full disk | Free disk space or add storage |
| SYST-3-103 | File Permission Denied | Cannot access required file | Wrong permissions | Fix file permissions |
| SYST-3-104 | Port Already In Use | Cannot bind to specified port | Port conflict | Change port or kill conflicting process |
| SYST-3-105 | Network Connection Failed | Cannot establish network connection | Network issue | Check network connectivity |
| SYST-3-106 | CPU Limit Exceeded | CPU usage exceeds configured limits | High CPU usage | Scale up or optimize performance |

### Service Lifecycle Errors (SYST-4-200 to SYST-4-299)

| Code | Error | Description | Cause | Solution |
|------|-------|-------------|-------|----------|
| SYST-4-200 | Service Startup Failed | Service failed to start | Various startup issues | Check logs for specific cause |
| SYST-4-201 | Service Shutdown Timeout | Service failed to shutdown gracefully | Hanging processes | Force kill or investigate hanging operations |
| SYST-4-202 | Dependency Service Unavailable | Required dependency service is down | External service failure | Check and restart dependency services |
| SYST-4-203 | Health Check Failed | Service health check is failing | Service degradation | Investigate service health issues |
| SYST-4-204 | Graceful Shutdown Failed | Cannot perform graceful shutdown | Resource cleanup issues | Force shutdown and investigate |

## Database Errors (DBASE)

### Connection Errors (DBASE-2-001 to DBASE-2-099)

| Code | Error | Description | Cause | Solution |
|------|-------|-------------|-------|----------|
| DBASE-2-001 | Connection Failed | Cannot connect to database | DB down/network issue | Check database status and connectivity |
| DBASE-2-002 | Connection Timeout | Database connection timed out | Network latency/DB load | Increase timeout or check DB performance |
| DBASE-2-003 | Authentication Failed | Database authentication failed | Wrong credentials | Verify database credentials |
| DBASE-2-004 | Connection Pool Exhausted | No available connections in pool | High concurrent usage | Increase pool size or optimize queries |
| DBASE-2-005 | Connection Lost | Lost connection to database | Network interruption | Implement connection retry logic |
| DBASE-2-006 | SSL Connection Failed | Cannot establish SSL connection | SSL configuration issue | Check SSL certificates and configuration |

### Query Errors (DBASE-2-100 to DBASE-2-199)

| Code | Error | Description | Cause | Solution |
|------|-------|-------------|-------|----------|
| DBASE-2-100 | Query Syntax Error | SQL query has syntax error | Malformed SQL | Fix SQL syntax |
| DBASE-2-101 | Query Timeout | Query execution timed out | Slow query/DB load | Optimize query or increase timeout |
| DBASE-2-102 | Constraint Violation | Database constraint violated | Data integrity issue | Fix data or adjust constraints |
| DBASE-2-103 | Table Not Found | Referenced table does not exist | Missing table/schema | Create table or fix table name |
| DBASE-2-104 | Column Not Found | Referenced column does not exist | Missing column | Add column or fix column name |
| DBASE-2-105 | Deadlock Detected | Database deadlock occurred | Concurrent transactions | Retry transaction with backoff |
| DBASE-2-106 | Transaction Rollback | Transaction was rolled back | Error during transaction | Check transaction logic and retry |

### Data Integrity Errors (DBASE-3-200 to DBASE-3-299)

| Code | Error | Description | Cause | Solution |
|------|-------|-------------|-------|----------|
| DBASE-3-200 | Data Corruption Detected | Database data corruption found | Hardware/software issue | Restore from backup and investigate |
| DBASE-3-201 | Index Corruption | Database index is corrupted | Index damage | Rebuild indexes |
| DBASE-3-202 | Foreign Key Violation | Foreign key constraint violated | Referential integrity issue | Fix data relationships |
| DBASE-3-203 | Unique Constraint Violation | Unique constraint violated | Duplicate data | Remove duplicates or adjust constraints |
| DBASE-3-204 | Check Constraint Violation | Check constraint violated | Invalid data values | Fix data values |

## Memory System Errors (MEMSYS)

### Memory Management Errors (MEMSYS-2-001 to MEMSYS-2-099)

| Code | Error | Description | Cause | Solution |
|------|-------|-------------|-------|----------|
| MEMSYS-2-001 | Memory Capacity Exceeded | Memory system at capacity | Too many memories stored | Increase capacity or cleanup old memories |
| MEMSYS-2-002 | Working Memory Full | Working memory is full | High processing load | Increase working memory size |
| MEMSYS-2-003 | Memory Allocation Failed | Cannot allocate memory object | System resource issue | Check system memory and restart |
| MEMSYS-2-004 | Memory Serialization Failed | Cannot serialize memory object | Data format issue | Check memory data format |
| MEMSYS-2-005 | Memory Deserialization Failed | Cannot deserialize memory object | Corrupted data | Restore from backup |
| MEMSYS-2-006 | Invalid Memory Type | Unknown memory type specified | Programming error | Use valid memory type |

### Memory Operations Errors (MEMSYS-2-100 to MEMSYS-2-199)

| Code | Error | Description | Cause | Solution |
|------|-------|-------------|-------|----------|
| MEMSYS-2-100 | Memory Not Found | Requested memory does not exist | Invalid memory ID | Verify memory ID exists |
| MEMSYS-2-101 | Memory Access Denied | Cannot access requested memory | Permission issue | Check memory access permissions |
| MEMSYS-2-102 | Memory Update Failed | Cannot update memory | Concurrent modification | Retry with proper locking |
| MEMSYS-2-103 | Memory Delete Failed | Cannot delete memory | Reference constraints | Remove references before deletion |
| MEMSYS-2-104 | Memory Search Failed | Memory search operation failed | Index/query issue | Check search parameters and indexes |
| MEMSYS-2-105 | Memory Consolidation Failed | Memory consolidation process failed | Background process error | Check consolidation configuration |

### Memory Consistency Errors (MEMSYS-3-200 to MEMSYS-3-299)

| Code | Error | Description | Cause | Solution |
|------|-------|-------------|-------|----------|
| MEMSYS-3-200 | Memory Inconsistency Detected | Memory state inconsistency found | Data corruption | Run consistency check and repair |
| MEMSYS-3-201 | Memory Index Mismatch | Memory index out of sync | Index corruption | Rebuild memory indexes |
| MEMSYS-3-202 | Memory Relationship Broken | Memory relationship integrity violated | Broken references | Fix memory relationships |

## Learning System Errors (LEARN)

### Learning Pipeline Errors (LEARN-2-001 to LEARN-2-099)

| Code | Error | Description | Cause | Solution |
|------|-------|-------------|-------|----------|
| LEARN-2-001 | Learning Queue Full | Learning queue is at capacity | High input rate | Increase queue size or processing speed |
| LEARN-2-002 | Learning Worker Failed | Learning worker process failed | Worker crash/error | Restart learning workers |
| LEARN-2-003 | Learning Timeout | Learning operation timed out | Complex processing | Increase timeout or optimize algorithm |
| LEARN-2-004 | Invalid Input Format | Learning input format is invalid | Malformed data | Validate and fix input format |
| LEARN-2-005 | Learning Model Error | Learning model encountered error | Model corruption | Reload or retrain model |
| LEARN-2-006 | Batch Processing Failed | Batch learning operation failed | Data/resource issue | Check batch data and resources |

### Segment Discovery Errors (LEARN-2-100 to LEARN-2-199)

| Code | Error | Description | Cause | Solution |
|------|-------|-------------|-------|----------|
| LEARN-2-100 | Segment Discovery Failed | Segment discovery algorithm failed | Algorithm error | Check input data quality |
| LEARN-2-101 | Vocabulary Size Exceeded | Vocabulary size limit exceeded | Too many unique segments | Increase limit or adjust pruning |
| LEARN-2-102 | Segment Frequency Too Low | Segment frequency below threshold | Sparse data | Lower threshold or provide more data |
| LEARN-2-103 | BPE Merge Failed | Byte-pair encoding merge failed | Algorithm state issue | Reset BPE state |
| LEARN-2-104 | Segment Validation Failed | Discovered segment failed validation | Invalid segment pattern | Adjust validation rules |

### Concept Extraction Errors (LEARN-2-200 to LEARN-2-299)

| Code | Error | Description | Cause | Solution |
|------|-------|-------------|-------|----------|
| LEARN-2-200 | Concept Extraction Failed | Concept extraction process failed | Processing error | Check extraction parameters |
| LEARN-2-201 | Concept Similarity Error | Concept similarity calculation failed | Algorithm error | Verify similarity algorithm |
| LEARN-2-202 | Concept Clustering Failed | Concept clustering algorithm failed | Data/parameter issue | Adjust clustering parameters |
| LEARN-2-203 | Concept Relationship Error | Cannot establish concept relationship | Relationship logic error | Check relationship rules |
| LEARN-2-204 | Concept Graph Full | Concept graph at capacity | Too many concepts | Increase capacity or prune concepts |

## API Errors (APIER)

### Authentication Errors (APIER-2-001 to APIER-2-099)

| Code | Error | Description | Cause | Solution |
|------|-------|-------------|-------|----------|
| APIER-2-001 | Authentication Required | Request requires authentication | Missing auth header | Provide authentication credentials |
| APIER-2-002 | Invalid Credentials | Authentication credentials are invalid | Wrong username/password | Use correct credentials |
| APIER-2-003 | Token Expired | Authentication token has expired | Expired JWT/session | Refresh or obtain new token |
| APIER-2-004 | Token Invalid | Authentication token is invalid | Malformed/corrupted token | Obtain new valid token |
| APIER-2-005 | Insufficient Permissions | User lacks required permissions | Authorization issue | Grant required permissions |
| APIER-2-006 | Account Locked | User account is locked | Security lockout | Unlock account or contact admin |

### Request Errors (APIER-2-100 to APIER-2-199)

| Code | Error | Description | Cause | Solution |
|------|-------|-------------|-------|----------|
| APIER-2-100 | Invalid Request Format | Request format is invalid | Malformed JSON/XML | Fix request format |
| APIER-2-101 | Missing Required Parameter | Required parameter is missing | Incomplete request | Add required parameter |
| APIER-2-102 | Invalid Parameter Value | Parameter value is invalid | Wrong data type/range | Correct parameter value |
| APIER-2-103 | Request Too Large | Request payload too large | Oversized request | Reduce request size |
| APIER-2-104 | Unsupported Media Type | Request media type not supported | Wrong Content-Type | Use supported media type |
| APIER-2-105 | Method Not Allowed | HTTP method not allowed | Wrong HTTP method | Use correct HTTP method |

### Rate Limiting Errors (APIER-2-200 to APIER-2-299)

| Code | Error | Description | Cause | Solution |
|------|-------|-------------|-------|----------|
| APIER-2-200 | Rate Limit Exceeded | API rate limit exceeded | Too many requests | Implement request throttling |
| APIER-2-201 | Quota Exceeded | API quota exceeded | Usage limit reached | Upgrade quota or wait for reset |
| APIER-2-202 | Concurrent Limit Exceeded | Too many concurrent requests | High concurrency | Reduce concurrent requests |

### Response Errors (APIER-3-300 to APIER-3-399)

| Code | Error | Description | Cause | Solution |
|------|-------|-------------|-------|----------|
| APIER-3-300 | Internal Server Error | Unexpected server error | Server-side issue | Check server logs and restart if needed |
| APIER-3-301 | Service Unavailable | Service temporarily unavailable | Maintenance/overload | Wait and retry later |
| APIER-3-302 | Gateway Timeout | Gateway timeout occurred | Upstream service slow | Check upstream services |
| APIER-3-303 | Response Serialization Failed | Cannot serialize response | Data format issue | Check response data format |

## Simulation Engine Errors (SIMUL)

### Simulation Execution Errors (SIMUL-2-001 to SIMUL-2-099)

| Code | Error | Description | Cause | Solution |
|------|-------|-------------|-------|----------|
| SIMUL-2-001 | Simulation Failed | Simulation execution failed | Various issues | Check simulation parameters and data |
| SIMUL-2-002 | Simulation Timeout | Simulation exceeded time limit | Complex scenario | Increase timeout or simplify scenario |
| SIMUL-2-003 | Invalid Scenario | Simulation scenario is invalid | Malformed scenario | Validate scenario format |
| SIMUL-2-004 | Simulation Queue Full | Simulation queue at capacity | High simulation load | Increase queue size or processing |
| SIMUL-2-005 | Insufficient Knowledge | Not enough knowledge for simulation | Limited training data | Provide more training data |
| SIMUL-2-006 | Simulation Convergence Failed | Simulation failed to converge | Algorithm issue | Adjust simulation parameters |

### Scenario Generation Errors (SIMUL-2-100 to SIMUL-2-199)

| Code | Error | Description | Cause | Solution |
|------|-------|-------------|-------|----------|
| SIMUL-2-100 | Scenario Generation Failed | Cannot generate scenario | Algorithm error | Check generation parameters |
| SIMUL-2-101 | Scenario Validation Failed | Generated scenario is invalid | Validation rules | Adjust generation or validation rules |
| SIMUL-2-102 | Scenario Complexity Too High | Scenario too complex to process | Resource limitations | Simplify scenario or increase resources |

## Concept Graph Errors (GRAPH)

### Graph Operations Errors (GRAPH-2-001 to GRAPH-2-099)

| Code | Error | Description | Cause | Solution |
|------|-------|-------------|-------|----------|
| GRAPH-2-001 | Node Not Found | Requested graph node not found | Invalid node ID | Verify node exists |
| GRAPH-2-002 | Edge Not Found | Requested graph edge not found | Invalid edge ID | Verify edge exists |
| GRAPH-2-003 | Graph Capacity Exceeded | Graph at maximum capacity | Too many nodes/edges | Increase capacity or prune graph |
| GRAPH-2-004 | Circular Reference Detected | Circular reference in graph | Graph logic error | Remove circular references |
| GRAPH-2-005 | Graph Traversal Failed | Graph traversal operation failed | Algorithm error | Check traversal parameters |
| GRAPH-2-006 | Graph Update Conflict | Concurrent graph update conflict | Race condition | Implement proper locking |

### Graph Analysis Errors (GRAPH-2-100 to GRAPH-2-199)

| Code | Error | Description | Cause | Solution |
|------|-------|-------------|-------|----------|
| GRAPH-2-100 | Clustering Algorithm Failed | Graph clustering failed | Algorithm/data issue | Adjust clustering parameters |
| GRAPH-2-101 | Centrality Calculation Failed | Centrality calculation failed | Algorithm error | Check algorithm parameters |
| GRAPH-2-102 | Community Detection Failed | Community detection failed | Graph structure issue | Use different detection algorithm |
| GRAPH-2-103 | Path Finding Failed | Cannot find path between nodes | Disconnected graph | Check graph connectivity |

## Error Handling Best Practices

### Error Response Format

```json
{
  "error": {
    "code": "MEMSYS-2-001",
    "message": "Memory capacity exceeded",
    "description": "The memory system has reached its configured capacity of 1,000,000 memories",
    "timestamp": "2024-01-01T12:00:00Z",
    "request_id": "req_123456789",
    "details": {
      "current_capacity": 1000000,
      "memory_count": 1000000,
      "suggested_action": "Increase memory capacity or clean up old memories"
    },
    "help_url": "https://docs.brain-ai.com/errors/MEMSYS-2-001"
  }
}
```

### Error Logging Format

```json
{
  "timestamp": "2024-01-01T12:00:00Z",
  "level": "ERROR",
  "logger": "brain_ai::memory",
  "message": "Memory capacity exceeded",
  "error_code": "MEMSYS-2-001",
  "request_id": "req_123456789",
  "user_id": "user_123",
  "context": {
    "memory_count": 1000000,
    "capacity": 1000000,
    "operation": "store_memory"
  },
  "stack_trace": "..."
}
```

### Error Recovery Strategies

#### Automatic Recovery

```rust
// Example automatic recovery for transient errors
match error.code {
    "DBASE-2-002" => {
        // Connection timeout - retry with backoff
        retry_with_exponential_backoff(operation, max_retries: 3)
    },
    "MEMSYS-2-002" => {
        // Working memory full - trigger cleanup
        trigger_memory_cleanup().await?;
        retry_operation(operation)
    },
    _ => return Err(error)
}
```

#### Manual Recovery

```bash
# Example recovery scripts for common errors

# SYST-3-100: Insufficient Memory
./scripts/cleanup-memory.sh
./scripts/restart-service.sh

# DBASE-2-001: Connection Failed
./scripts/check-database-status.sh
./scripts/restart-database.sh

# MEMSYS-3-200: Memory Inconsistency
./scripts/memory-consistency-check.sh
./scripts/repair-memory-indexes.sh
```

### Monitoring and Alerting

```yaml
# Example alert rules for error codes
groups:
- name: brain-ai-errors
  rules:
  - alert: CriticalErrors
    expr: increase(brain_ai_errors_total{severity="3"}[5m]) > 0
    annotations:
      summary: "Critical errors detected in Brain AI"
      
  - alert: FatalErrors
    expr: increase(brain_ai_errors_total{severity="4"}[1m]) > 0
    annotations:
      summary: "Fatal errors detected in Brain AI"
      
  - alert: HighErrorRate
    expr: rate(brain_ai_errors_total[5m]) > 0.1
    annotations:
      summary: "High error rate in Brain AI"
```

This comprehensive error code reference enables quick diagnosis and resolution of issues across all Brain AI components.
