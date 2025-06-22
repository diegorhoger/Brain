# REST API Overview

Brain AI provides a comprehensive RESTful API that exposes all cognitive capabilities through HTTP endpoints. This API is designed for integration with external applications, web frontends, and automated systems.

## Base URL and Versioning

```
Base URL: http://localhost:8080/api/v1
```

All API endpoints are versioned and follow RESTful conventions. The current API version is `v1`.

## Authentication

Brain AI uses JWT (JSON Web Token) based authentication for secure API access.

### Getting an Access Token

```bash
POST /auth/login
Content-Type: application/json

{
  "username": "your_username",
  "password": "your_password"
}
```

**Response:**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "expires_in": 3600,
  "token_type": "Bearer"
}
```

### Using the Token

Include the token in the Authorization header for all subsequent requests:

```bash
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

## Rate Limiting

The API implements rate limiting to ensure fair usage and system stability:

- **Default Limit**: 100 requests per minute per user
- **Burst Limit**: 20 requests per 10 seconds
- **Headers**: Rate limit information is included in response headers

**Rate Limit Headers:**
```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1640995200
```

## Core API Endpoints

### 1. Learning Endpoints

#### Learn from Text
```bash
POST /api/v1/learn
Content-Type: application/json
Authorization: Bearer {token}

{
  "text": "Python is a programming language known for its simplicity",
  "priority": "high",
  "context": {
    "source": "documentation",
    "domain": "programming"
  }
}
```

**Response:**
```json
{
  "success": true,
  "message": "Text learned successfully",
  "learning_id": "550e8400-e29b-41d4-a716-446655440000",
  "segments_discovered": 8,
  "concepts_updated": 3,
  "processing_time_ms": 45
}
```

#### Batch Learning
```bash
POST /api/v1/learn/batch
Content-Type: application/json

{
  "texts": [
    {
      "text": "First piece of information",
      "priority": "high"
    },
    {
      "text": "Second piece of information",
      "priority": "medium"
    }
  ]
}
```

### 2. Segmentation Endpoints

#### Segment Text
```bash
POST /api/v1/segment
Content-Type: application/json

{
  "text": "The quick brown fox jumps over the lazy dog",
  "algorithm": "bpe",
  "options": {
    "max_segments": 20,
    "min_segment_length": 1
  }
}
```

**Response:**
```json
{
  "segments": [
    {"text": "The", "start": 0, "end": 3, "confidence": 0.95},
    {"text": "quick", "start": 4, "end": 9, "confidence": 0.92},
    {"text": "brown", "start": 10, "end": 15, "confidence": 0.89}
  ],
  "total_segments": 9,
  "processing_time_ms": 12
}
```

### 3. Memory Endpoints

#### Query Memory
```bash
GET /api/v1/memory/search?query=programming&limit=10&type=semantic
```

**Response:**
```json
{
  "results": [
    {
      "id": "mem_123",
      "content": "Python is a programming language",
      "type": "semantic",
      "confidence": 0.92,
      "timestamp": "2024-01-01T12:00:00Z",
      "related_concepts": ["python", "programming", "language"]
    }
  ],
  "total_results": 25,
  "page": 1,
  "limit": 10
}
```

#### Store Memory
```bash
POST /api/v1/memory
Content-Type: application/json

{
  "content": "Important information to remember",
  "type": "episodic",
  "priority": "high",
  "context": {
    "timestamp": "2024-01-01T12:00:00Z",
    "source": "user_input"
  }
}
```

### 4. Concept Graph Endpoints

#### Get Related Concepts
```bash
GET /api/v1/concepts/cat/related?depth=2&limit=10
```

**Response:**
```json
{
  "concept": "cat",
  "related_concepts": [
    {
      "concept": "animal",
      "relationship": "is_a",
      "strength": 0.95,
      "distance": 1
    },
    {
      "concept": "pet",
      "relationship": "can_be",
      "strength": 0.88,
      "distance": 1
    }
  ],
  "total_relationships": 15
}
```

#### Create Concept Relationship
```bash
POST /api/v1/concepts/relationships
Content-Type: application/json

{
  "from_concept": "dog",
  "to_concept": "animal",
  "relationship_type": "is_a",
  "strength": 0.9
}
```

### 5. Simulation Endpoints

#### Run Simulation
```bash
POST /api/v1/simulate
Content-Type: application/json

{
  "scenario": "What happens if a cat meets a dog?",
  "max_steps": 5,
  "confidence_threshold": 0.3,
  "constraints": [
    {
      "type": "avoid",
      "condition": "aggressive_behavior"
    }
  ]
}
```

**Response:**
```json
{
  "simulation_id": "sim_456",
  "scenario": "What happens if a cat meets a dog?",
  "outcome": "The cat and dog cautiously approach each other",
  "confidence": 0.75,
  "steps": [
    {
      "step": 1,
      "action": "Initial approach",
      "confidence": 0.85,
      "branches_explored": 3
    }
  ],
  "total_branches": 12,
  "pruned_branches": 7,
  "processing_time_ms": 234
}
```

### 6. Insight Extraction Endpoints

#### Extract Insights
```bash
POST /api/v1/insights/extract
Content-Type: application/json

{
  "text": "Cats usually sleep 12-16 hours per day. Dogs sleep 8-12 hours per day.",
  "insight_types": ["patterns", "rules", "relationships"]
}
```

**Response:**
```json
{
  "insights": [
    {
      "type": "pattern",
      "description": "Sleep duration varies by animal type",
      "confidence": 0.87,
      "evidence": ["cats: 12-16 hours", "dogs: 8-12 hours"]
    },
    {
      "type": "rule",
      "condition": "if animal is cat",
      "conclusion": "then sleep duration is 12-16 hours",
      "confidence": 0.92
    }
  ],
  "processing_time_ms": 156
}
```

### 7. Performance Monitoring Endpoints

#### Get System Metrics
```bash
GET /api/v1/performance/metrics
```

**Response:**
```json
{
  "system_metrics": {
    "cpu_usage_percent": 45.2,
    "memory_usage_mb": 512,
    "disk_usage_percent": 23.1
  },
  "component_metrics": {
    "character_ingestion": {
      "operations_per_second": 1250,
      "average_latency_ms": 2.3
    },
    "memory_system": {
      "working_memory_size": 1024,
      "episodic_memories": 5432,
      "semantic_concepts": 1876
    }
  },
  "timestamp": "2024-01-01T12:00:00Z"
}
```

#### Get Performance Bottlenecks
```bash
GET /api/v1/performance/bottlenecks
```

**Response:**
```json
{
  "bottlenecks": [
    {
      "component": "concept_graph",
      "severity": "medium",
      "description": "Neo4j query response time above threshold",
      "current_value": 150,
      "threshold": 100,
      "recommendations": [
        "Add database indexes",
        "Optimize query patterns"
      ]
    }
  ],
  "overall_health": "good",
  "timestamp": "2024-01-01T12:00:00Z"
}
```

## Error Handling

The API uses standard HTTP status codes and provides detailed error information:

### Error Response Format

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid input provided",
    "details": {
      "field": "text",
      "reason": "Text cannot be empty"
    },
    "request_id": "req_789",
    "timestamp": "2024-01-01T12:00:00Z"
  }
}
```

### Common HTTP Status Codes

- **200 OK**: Request successful
- **201 Created**: Resource created successfully
- **400 Bad Request**: Invalid request format or parameters
- **401 Unauthorized**: Authentication required or invalid token
- **403 Forbidden**: Insufficient permissions
- **404 Not Found**: Resource not found
- **429 Too Many Requests**: Rate limit exceeded
- **500 Internal Server Error**: Server error occurred

### Error Codes

| Code | Description |
|------|-------------|
| `VALIDATION_ERROR` | Input validation failed |
| `AUTHENTICATION_ERROR` | Authentication failed |
| `AUTHORIZATION_ERROR` | Insufficient permissions |
| `RATE_LIMIT_EXCEEDED` | Too many requests |
| `RESOURCE_NOT_FOUND` | Requested resource not found |
| `PROCESSING_ERROR` | Error during cognitive processing |
| `STORAGE_ERROR` | Database or storage error |
| `CONFIGURATION_ERROR` | System configuration error |

## Response Formats

### Success Response Structure

```json
{
  "success": true,
  "data": {
    // Response data
  },
  "metadata": {
    "request_id": "req_123",
    "processing_time_ms": 45,
    "timestamp": "2024-01-01T12:00:00Z"
  }
}
```

### Pagination

For endpoints that return lists, pagination is supported:

```json
{
  "data": [...],
  "pagination": {
    "page": 1,
    "limit": 10,
    "total": 150,
    "has_next": true,
    "has_previous": false
  }
}
```

## WebSocket API

For real-time updates and streaming responses, Brain AI provides WebSocket endpoints:

### Connection
```javascript
const ws = new WebSocket('ws://localhost:8080/ws');
ws.onopen = function() {
    // Send authentication
    ws.send(JSON.stringify({
        type: 'auth',
        token: 'your_jwt_token'
    }));
};
```

### Real-time Learning Updates
```javascript
ws.send(JSON.stringify({
    type: 'subscribe',
    channel: 'learning_updates'
}));

ws.onmessage = function(event) {
    const data = JSON.parse(event.data);
    if (data.type === 'learning_update') {
        console.log('New learning event:', data.payload);
    }
};
```

## SDK and Client Libraries

Brain AI provides official client libraries for popular programming languages:

### Python SDK
```python
from brain_ai import BrainClient

client = BrainClient(
    base_url="http://localhost:8080",
    api_key="your_api_key"
)

# Learn from text
result = client.learn("Python is a programming language")

# Query memory
memories = client.query_memory("programming")

# Run simulation
simulation = client.simulate("What if I learn Rust?")
```

### JavaScript SDK
```javascript
import { BrainClient } from '@brain-ai/client';

const client = new BrainClient({
    baseUrl: 'http://localhost:8080',
    apiKey: 'your_api_key'
});

// Learn from text
const result = await client.learn('Python is a programming language');

// Query memory
const memories = await client.queryMemory('programming');

// Run simulation
const simulation = await client.simulate('What if I learn Rust?');
```

## API Versioning and Compatibility

Brain AI follows semantic versioning for API compatibility:

- **Major version**: Breaking changes (e.g., v1 → v2)
- **Minor version**: New features, backward compatible
- **Patch version**: Bug fixes, backward compatible

### Version Headers

Include version preferences in requests:

```bash
API-Version: v1
Accept-Version: v1.2
```

## Next Steps

- **[Authentication Guide](./authentication.md)**: Detailed authentication setup
- **[Core Endpoints](./core-endpoints.md)**: Complete endpoint reference
- **[Error Handling](./error-handling.md)**: Comprehensive error handling guide
- **[Python Bindings](../python/overview.md)**: Python-specific API usage

---

The Brain AI REST API provides comprehensive access to all cognitive capabilities with enterprise-grade security, performance monitoring, and error handling. Start with the [Quick Start Guide](../getting-started/quick-start.md) to begin integrating Brain AI into your applications. 