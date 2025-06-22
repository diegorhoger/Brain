# Core Endpoints

This document provides detailed information about Brain AI's core API endpoints, including request/response formats, parameters, and usage examples.

## Learning Endpoints

### Learn from Text

Process and learn from text input, updating the AI's knowledge base.

**Endpoint:** `POST /api/v1/learn`

**Request Headers:**
```http
Content-Type: application/json
Authorization: Bearer {token}
```

**Request Body:**
```json
{
  "text": "Machine learning is a subset of artificial intelligence that focuses on algorithms",
  "priority": "high",
  "context": {
    "source": "educational_content",
    "domain": "artificial_intelligence",
    "author": "expert",
    "timestamp": "2024-01-01T12:00:00Z"
  },
  "options": {
    "enable_concept_discovery": true,
    "enable_relationship_inference": true,
    "consolidation_threshold": 0.7
  }
}
```

**Parameters:**
- `text` (string, required): The text content to learn from
- `priority` (string, optional): Learning priority - "low", "medium", "high", "critical"
- `context` (object, optional): Additional context information
- `options` (object, optional): Processing options and parameters

**Response (200 OK):**
```json
{
  "success": true,
  "message": "Text learned successfully",
  "learning_id": "550e8400-e29b-41d4-a716-446655440000",
  "results": {
    "segments_discovered": 12,
    "concepts_updated": 5,
    "concepts_created": 2,
    "relationships_formed": 8,
    "memory_items_stored": 3,
    "processing_time_ms": 156
  },
  "insights": [
    {
      "type": "concept_relationship",
      "description": "Discovered strong relationship between 'machine learning' and 'artificial intelligence'",
      "confidence": 0.92
    }
  ]
}
```

### Batch Learning

Process multiple text inputs in a single request for efficient bulk learning.

**Endpoint:** `POST /api/v1/learn/batch`

**Request Body:**
```json
{
  "texts": [
    {
      "text": "Python is a high-level programming language",
      "priority": "high",
      "context": {"domain": "programming"}
    },
    {
      "text": "JavaScript is used for web development",
      "priority": "medium",
      "context": {"domain": "web_development"}
    }
  ],
  "options": {
    "parallel_processing": true,
    "max_concurrent": 5,
    "fail_on_error": false
  }
}
```

**Response (200 OK):**
```json
{
  "success": true,
  "batch_id": "batch_550e8400-e29b-41d4-a716-446655440000",
  "results": [
    {
      "index": 0,
      "success": true,
      "learning_id": "learn_123",
      "segments_discovered": 8,
      "processing_time_ms": 89
    }
  ],
  "summary": {
    "total_items": 2,
    "successful": 2,
    "failed": 0,
    "total_processing_time_ms": 165
  }
}
```

## Memory Endpoints

### Query Memory System

Search and retrieve information from Brain AI's memory system.

**Endpoint:** `GET /api/v1/memory/search`

**Query Parameters:**
- `query` (string, required): Search query
- `type` (string, optional): Memory type - "semantic", "episodic", "procedural", "pattern"
- `limit` (integer, optional): Maximum results (default: 10, max: 100)
- `min_confidence` (float, optional): Minimum confidence threshold (0.0-1.0)

**Example Request:**
```http
GET /api/v1/memory/search?query=machine%20learning&type=semantic&limit=5&min_confidence=0.7
```

**Response (200 OK):**
```json
{
  "results": [
    {
      "id": "mem_550e8400-e29b-41d4-a716-446655440000",
      "content": "Machine learning is a subset of artificial intelligence",
      "type": "semantic",
      "confidence": 0.94,
      "importance": 0.87,
      "created_at": "2024-01-01T12:00:00Z",
      "related_concepts": ["artificial_intelligence", "algorithms", "data_science"]
    }
  ],
  "metadata": {
    "total_results": 47,
    "page": 1,
    "limit": 5,
    "query_time_ms": 12
  }
}
```

### Store Memory

Add new information to the memory system.

**Endpoint:** `POST /api/v1/memory`

**Request Body:**
```json
{
  "content": "Neural networks are inspired by biological neural networks",
  "type": "semantic",
  "importance": 0.8,
  "context": {
    "domain": "machine_learning",
    "source": "research_paper",
    "timestamp": "2024-01-01T12:00:00Z"
  },
  "tags": ["neural_networks", "biology", "machine_learning"]
}
```

**Response (201 Created):**
```json
{
  "success": true,
  "memory_id": "mem_550e8400-e29b-41d4-a716-446655440000",
  "message": "Memory stored successfully",
  "consolidation_status": "pending",
  "related_memories_updated": 3
}
```

## Concept Graph Endpoints

### Get Related Concepts

Discover concepts related to a given concept through the knowledge graph.

**Endpoint:** `GET /api/v1/concepts/{concept}/related`

**Query Parameters:**
- `depth` (integer, optional): Traversal depth (default: 2, max: 5)
- `limit` (integer, optional): Maximum results (default: 10, max: 50)
- `min_strength` (float, optional): Minimum relationship strength (0.0-1.0)

**Example Request:**
```http
GET /api/v1/concepts/machine_learning/related?depth=2&limit=10&min_strength=0.5
```

**Response (200 OK):**
```json
{
  "concept": "machine_learning",
  "related_concepts": [
    {
      "concept": "artificial_intelligence",
      "relationship_type": "subset_of",
      "strength": 0.92,
      "distance": 1
    },
    {
      "concept": "neural_networks",
      "relationship_type": "uses",
      "strength": 0.87,
      "distance": 1
    }
  ],
  "metadata": {
    "total_related": 23,
    "search_depth": 2,
    "query_time_ms": 18
  }
}
```

## System Status Endpoints

### Health Check

Check the overall health and status of the Brain AI system.

**Endpoint:** `GET /api/v1/health`

**Response (200 OK):**
```json
{
  "status": "healthy",
  "timestamp": "2024-01-02T10:30:00Z",
  "version": "1.0.0",
  "components": {
    "character_ingestion": {
      "status": "healthy",
      "metrics": {
        "characters_processed": 1234567,
        "average_processing_time_ms": 12
      }
    },
    "memory_system": {
      "status": "healthy",
      "metrics": {
        "total_memories": 5678,
        "working_memory_usage": 0.67
      }
    },
    "concept_graph": {
      "status": "healthy",
      "metrics": {
        "total_concepts": 2345,
        "total_relationships": 8901
      }
    }
  }
}
```

This document provides detailed information about Brain AI's core API endpoints, including request/response formats, parameters, and usage examples.

## Learning Endpoints

### Learn from Text

Process and learn from text input, updating the AI's knowledge base.

**Endpoint:** `POST /api/v1/learn`

**Request Headers:**
```http
Content-Type: application/json
Authorization: Bearer {token}
```

**Request Body:**
```json
{
  "text": "Machine learning is a subset of artificial intelligence that focuses on algorithms",
  "priority": "high",
  "context": {
    "source": "educational_content",
    "domain": "artificial_intelligence",
    "author": "expert",
    "timestamp": "2024-01-01T12:00:00Z"
  },
  "options": {
    "enable_concept_discovery": true,
    "enable_relationship_inference": true,
    "consolidation_threshold": 0.7
  }
}
```

**Parameters:**
- `text` (string, required): The text content to learn from
- `priority` (string, optional): Learning priority - "low", "medium", "high", "critical"
- `context` (object, optional): Additional context information
- `options` (object, optional): Processing options and parameters

**Response (200 OK):**
```json
{
  "success": true,
  "message": "Text learned successfully",
  "learning_id": "550e8400-e29b-41d4-a716-446655440000",
  "results": {
    "segments_discovered": 12,
    "concepts_updated": 5,
    "concepts_created": 2,
    "relationships_formed": 8,
    "memory_items_stored": 3,
    "processing_time_ms": 156
  },
  "insights": [
    {
      "type": "concept_relationship",
      "description": "Discovered strong relationship between 'machine learning' and 'artificial intelligence'",
      "confidence": 0.92
    }
  ]
}
```

**Error Responses:**
```json
// 400 Bad Request
{
  "error": "invalid_request",
  "message": "Text content is required",
  "details": {
    "field": "text",
    "code": "MISSING_REQUIRED_FIELD"
  }
}

// 413 Payload Too Large
{
  "error": "payload_too_large",
  "message": "Text content exceeds maximum size limit",
  "details": {
    "max_size": "1MB",
    "provided_size": "1.5MB"
  }
}
```

### Batch Learning

Process multiple text inputs in a single request for efficient bulk learning.

**Endpoint:** `POST /api/v1/learn/batch`

**Request Body:**
```json
{
  "texts": [
    {
      "text": "Python is a high-level programming language",
      "priority": "high",
      "context": {"domain": "programming"}
    },
    {
      "text": "JavaScript is used for web development",
      "priority": "medium",
      "context": {"domain": "web_development"}
    }
  ],
  "options": {
    "parallel_processing": true,
    "max_concurrent": 5,
    "fail_on_error": false
  }
}
```

**Response (200 OK):**
```json
{
  "success": true,
  "batch_id": "batch_550e8400-e29b-41d4-a716-446655440000",
  "results": [
    {
      "index": 0,
      "success": true,
      "learning_id": "learn_123",
      "segments_discovered": 8,
      "processing_time_ms": 89
    },
    {
      "index": 1,
      "success": true,
      "learning_id": "learn_124",
      "segments_discovered": 7,
      "processing_time_ms": 76
    }
  ],
  "summary": {
    "total_items": 2,
    "successful": 2,
    "failed": 0,
    "total_processing_time_ms": 165
  }
}
```

## Segmentation Endpoints

### Segment Text

Break down text into meaningful segments using Brain AI's advanced segmentation algorithms.

**Endpoint:** `POST /api/v1/segment`

**Request Body:**
```json
{
  "text": "The quick brown fox jumps over the lazy dog",
  "algorithm": "adaptive_bpe",
  "options": {
    "max_segments": 20,
    "min_segment_length": 1,
    "context_window": 5,
    "confidence_threshold": 0.6,
    "enable_validation": true
  }
}
```

**Parameters:**
- `text` (string, required): Text to segment
- `algorithm` (string, optional): Segmentation algorithm - "bpe", "adaptive_bpe", "feedback_bpe"
- `options` (object, optional): Algorithm-specific options

**Response (200 OK):**
```json
{
  "segments": [
    {
      "text": "The",
      "start_index": 0,
      "end_index": 3,
      "confidence": 0.95,
      "segment_type": "word",
      "boundary_strength": 0.89
    },
    {
      "text": "quick",
      "start_index": 4,
      "end_index": 9,
      "confidence": 0.92,
      "segment_type": "word",
      "boundary_strength": 0.87
    },
    {
      "text": "brown fox",
      "start_index": 10,
      "end_index": 19,
      "confidence": 0.88,
      "segment_type": "compound",
      "boundary_strength": 0.82
    }
  ],
  "metadata": {
    "total_segments": 8,
    "average_confidence": 0.91,
    "algorithm_used": "adaptive_bpe",
    "processing_time_ms": 23,
    "validation_passed": true
  }
}
```

### Get Segmentation Quality

Analyze the quality of segmentation results.

**Endpoint:** `POST /api/v1/segment/analyze`

**Request Body:**
```json
{
  "text": "Original text",
  "segments": [
    {"text": "Original", "start_index": 0, "end_index": 8},
    {"text": "text", "start_index": 9, "end_index": 13}
  ]
}
```

**Response (200 OK):**
```json
{
  "quality_score": 0.87,
  "analysis": {
    "boundary_accuracy": 0.92,
    "semantic_coherence": 0.84,
    "consistency_score": 0.89,
    "length_distribution": "optimal"
  },
  "recommendations": [
    "Consider merging segments with high semantic similarity",
    "Boundary at index 8 could be refined"
  ]
}
```

## Memory Endpoints

### Query Memory System

Search and retrieve information from Brain AI's memory system.

**Endpoint:** `GET /api/v1/memory/search`

**Query Parameters:**
- `query` (string, required): Search query
- `type` (string, optional): Memory type - "semantic", "episodic", "procedural", "pattern"
- `limit` (integer, optional): Maximum results (default: 10, max: 100)
- `offset` (integer, optional): Pagination offset (default: 0)
- `min_confidence` (float, optional): Minimum confidence threshold (0.0-1.0)
- `sort_by` (string, optional): Sort criteria - "relevance", "confidence", "recency"
- `include_associations` (boolean, optional): Include associated memories

**Example Request:**
```http
GET /api/v1/memory/search?query=machine%20learning&type=semantic&limit=5&min_confidence=0.7&include_associations=true
```

**Response (200 OK):**
```json
{
  "results": [
    {
      "id": "mem_550e8400-e29b-41d4-a716-446655440000",
      "content": "Machine learning is a subset of artificial intelligence",
      "type": "semantic",
      "confidence": 0.94,
      "importance": 0.87,
      "created_at": "2024-01-01T12:00:00Z",
      "last_accessed": "2024-01-02T08:30:00Z",
      "access_count": 15,
      "context": {
        "domain": "artificial_intelligence",
        "source": "educational_content"
      },
      "related_concepts": ["artificial_intelligence", "algorithms", "data_science"],
      "associations": [
        {
          "memory_id": "mem_another_id",
          "relationship_type": "related_to",
          "strength": 0.82
        }
      ]
    }
  ],
  "metadata": {
    "total_results": 47,
    "page": 1,
    "limit": 5,
    "query_time_ms": 12,
    "search_strategy": "semantic_similarity"
  }
}
```

### Store Memory

Add new information to the memory system.

**Endpoint:** `POST /api/v1/memory`

**Request Body:**
```json
{
  "content": "Neural networks are inspired by biological neural networks",
  "type": "semantic",
  "importance": 0.8,
  "context": {
    "domain": "machine_learning",
    "source": "research_paper",
    "author": "expert",
    "timestamp": "2024-01-01T12:00:00Z"
  },
  "tags": ["neural_networks", "biology", "machine_learning"],
  "associations": [
    {
      "memory_id": "existing_memory_id",
      "relationship_type": "builds_upon",
      "strength": 0.75
    }
  ]
}
```

**Response (201 Created):**
```json
{
  "success": true,
  "memory_id": "mem_550e8400-e29b-41d4-a716-446655440000",
  "message": "Memory stored successfully",
  "consolidation_status": "pending",
  "related_memories_updated": 3
}
```

### Retrieve Memory by ID

Get specific memory item by its unique identifier.

**Endpoint:** `GET /api/v1/memory/{memory_id}`

**Response (200 OK):**
```json
{
  "id": "mem_550e8400-e29b-41d4-a716-446655440000",
  "content": "Neural networks are inspired by biological neural networks",
  "type": "semantic",
  "confidence": 0.92,
  "importance": 0.8,
  "created_at": "2024-01-01T12:00:00Z",
  "last_accessed": "2024-01-02T08:30:00Z",
  "access_count": 8,
  "context": {
    "domain": "machine_learning",
    "source": "research_paper"
  },
  "tags": ["neural_networks", "biology", "machine_learning"],
  "associations": [
    {
      "memory_id": "mem_related_id",
      "relationship_type": "builds_upon",
      "strength": 0.75,
      "created_at": "2024-01-01T12:05:00Z"
    }
  ],
  "access_history": [
    {
      "timestamp": "2024-01-02T08:30:00Z",
      "context": "user_query"
    }
  ]
}
```

## Concept Graph Endpoints

### Get Related Concepts

Discover concepts related to a given concept through the knowledge graph.

**Endpoint:** `GET /api/v1/concepts/{concept}/related`

**Query Parameters:**
- `depth` (integer, optional): Traversal depth (default: 2, max: 5)
- `limit` (integer, optional): Maximum results (default: 10, max: 50)
- `min_strength` (float, optional): Minimum relationship strength (0.0-1.0)
- `relationship_types` (string, optional): Comma-separated relationship types
- `include_path` (boolean, optional): Include relationship path information

**Example Request:**
```http
GET /api/v1/concepts/machine_learning/related?depth=2&limit=10&min_strength=0.5&include_path=true
```

**Response (200 OK):**
```json
{
  "concept": "machine_learning",
  "related_concepts": [
    {
      "concept": "artificial_intelligence",
      "relationship_type": "subset_of",
      "strength": 0.92,
      "distance": 1,
      "path": [
        {
          "from": "machine_learning",
          "to": "artificial_intelligence",
          "relationship": "subset_of",
          "strength": 0.92
        }
      ]
    },
    {
      "concept": "neural_networks",
      "relationship_type": "uses",
      "strength": 0.87,
      "distance": 1,
      "path": [
        {
          "from": "machine_learning",
          "to": "neural_networks",
          "relationship": "uses",
          "strength": 0.87
        }
      ]
    }
  ],
  "metadata": {
    "total_related": 23,
    "search_depth": 2,
    "query_time_ms": 18
  }
}
```

### Add Concept Relationship

Create or strengthen a relationship between two concepts.

**Endpoint:** `POST /api/v1/concepts/relationships`

**Request Body:**
```json
{
  "source_concept": "deep_learning",
  "target_concept": "machine_learning",
  "relationship_type": "subset_of",
  "strength": 0.89,
  "evidence": [
    "Deep learning is a subset of machine learning",
    "Multiple authoritative sources confirm this relationship"
  ],
  "context": {
    "source": "expert_knowledge",
    "confidence": 0.95
  }
}
```

**Response (201 Created):**
```json
{
  "success": true,
  "relationship_id": "rel_550e8400-e29b-41d4-a716-446655440000",
  "message": "Relationship created successfully",
  "existing_relationship_updated": false,
  "graph_updates": {
    "concepts_affected": 2,
    "indirect_relationships_updated": 5
  }
}
```

### Get Concept Details

Retrieve detailed information about a specific concept.

**Endpoint:** `GET /api/v1/concepts/{concept}`

**Response (200 OK):**
```json
{
  "concept": "machine_learning",
  "definition": "A method of data analysis that automates analytical model building",
  "aliases": ["ML", "statistical_learning"],
  "concept_type": "abstract",
  "importance_score": 0.94,
  "creation_date": "2024-01-01T10:00:00Z",
  "last_updated": "2024-01-02T15:30:00Z",
  "usage_frequency": 156,
  "relationships": {
    "outgoing": [
      {
        "target": "artificial_intelligence",
        "type": "subset_of",
        "strength": 0.92
      }
    ],
    "incoming": [
      {
        "source": "deep_learning",
        "type": "subset_of",
        "strength": 0.89
      }
    ]
  },
  "related_memories": [
    {
      "memory_id": "mem_123",
      "relevance": 0.87,
      "content_preview": "Machine learning algorithms can learn from data..."
    }
  ]
}
```

## Insight Endpoints

### Generate Insights

Extract insights from processed data or specific queries.

**Endpoint:** `POST /api/v1/insights/generate`

**Request Body:**
```json
{
  "data_source": "memory",
  "query": "trends in machine learning adoption",
  "insight_types": ["predictive", "explanatory"],
  "options": {
    "min_confidence": 0.7,
    "max_insights": 10,
    "include_evidence": true,
    "time_range": {
      "start": "2024-01-01T00:00:00Z",
      "end": "2024-01-31T23:59:59Z"
    }
  }
}
```

**Response (200 OK):**
```json
{
  "insights": [
    {
      "id": "insight_550e8400-e29b-41d4-a716-446655440000",
      "type": "predictive",
      "title": "Increasing Trend in Machine Learning Adoption",
      "description": "Based on processed data, machine learning adoption shows a 23% increase trend",
      "confidence": 0.87,
      "importance": 0.82,
      "evidence": [
        "15 mentions of 'machine learning implementation' in recent content",
        "Strong correlation with 'automation' and 'efficiency' concepts"
      ],
      "actionable_recommendations": [
        "Consider expanding machine learning resources",
        "Monitor emerging ML technologies"
      ],
      "generated_at": "2024-01-02T10:30:00Z"
    }
  ],
  "metadata": {
    "total_insights": 7,
    "processing_time_ms": 234,
    "data_points_analyzed": 1247
  }
}
```

### Get Historical Insights

Retrieve previously generated insights with filtering options.

**Endpoint:** `GET /api/v1/insights`

**Query Parameters:**
- `type` (string, optional): Insight type filter
- `min_confidence` (float, optional): Minimum confidence threshold
- `date_from` (string, optional): ISO date string
- `date_to` (string, optional): ISO date string
- `limit` (integer, optional): Maximum results
- `sort_by` (string, optional): Sort criteria

**Response (200 OK):**
```json
{
  "insights": [
    {
      "id": "insight_123",
      "type": "explanatory",
      "title": "Correlation Between Learning Rate and Performance",
      "confidence": 0.91,
      "importance": 0.78,
      "generated_at": "2024-01-01T14:20:00Z",
      "summary": "Analysis reveals strong correlation between learning frequency and system performance"
    }
  ],
  "pagination": {
    "total": 45,
    "page": 1,
    "limit": 10,
    "has_next": true
  }
}
```

## System Status Endpoints

### Health Check

Check the overall health and status of the Brain AI system.

**Endpoint:** `GET /api/v1/health`

**Response (200 OK):**
```json
{
  "status": "healthy",
  "timestamp": "2024-01-02T10:30:00Z",
  "version": "1.0.0",
  "components": {
    "character_ingestion": {
      "status": "healthy",
      "last_check": "2024-01-02T10:29:55Z",
      "metrics": {
        "characters_processed": 1234567,
        "average_processing_time_ms": 12
      }
    },
    "memory_system": {
      "status": "healthy",
      "last_check": "2024-01-02T10:29:55Z",
      "metrics": {
        "total_memories": 5678,
        "working_memory_usage": 0.67,
        "consolidation_queue": 23
      }
    },
    "concept_graph": {
      "status": "healthy",
      "last_check": "2024-01-02T10:29:55Z",
      "metrics": {
        "total_concepts": 2345,
        "total_relationships": 8901,
        "graph_density": 0.34
      }
    }
  },
  "performance": {
    "cpu_usage": 0.45,
    "memory_usage": 0.67,
    "disk_usage": 0.23,
    "response_time_ms": 8
  }
}
```

### System Metrics

Get detailed system performance and usage metrics.

**Endpoint:** `GET /api/v1/metrics`

**Query Parameters:**
- `component` (string, optional): Specific component metrics
- `time_range` (string, optional): Time range for metrics (1h, 24h, 7d, 30d)
- `granularity` (string, optional): Data granularity (minute, hour, day)

**Response (200 OK):**
```json
{
  "metrics": {
    "learning": {
      "texts_processed_total": 12345,
      "average_processing_time_ms": 156,
      "success_rate": 0.987,
      "last_24h": {
        "texts_processed": 234,
        "peak_processing_time": 1200,
        "errors": 2
      }
    },
    "memory": {
      "total_capacity": 1000000,
      "current_usage": 678901,
      "consolidation_rate": 0.23,
      "retrieval_success_rate": 0.994
    },
    "api": {
      "total_requests": 56789,
      "requests_per_minute": 45,
      "average_response_time_ms": 23,
      "error_rate": 0.002
    }
  },
  "timestamp": "2024-01-02T10:30:00Z",
  "time_range": "24h"
}
```

## Error Handling

All endpoints follow consistent error response formats:

### Standard Error Response

```json
{
  "error": "error_code",
  "message": "Human-readable error description",
  "details": {
    "field": "specific_field_if_applicable",
    "code": "DETAILED_ERROR_CODE",
    "suggestions": ["Possible solutions"]
  },
  "request_id": "req_550e8400-e29b-41d4-a716-446655440000",
  "timestamp": "2024-01-02T10:30:00Z"
}
```

### Common HTTP Status Codes

- **200 OK**: Request successful
- **201 Created**: Resource created successfully
- **400 Bad Request**: Invalid request parameters
- **401 Unauthorized**: Authentication required or invalid
- **403 Forbidden**: Insufficient permissions
- **404 Not Found**: Resource not found
- **413 Payload Too Large**: Request body too large
- **429 Too Many Requests**: Rate limit exceeded
- **500 Internal Server Error**: Server error
- **503 Service Unavailable**: Service temporarily unavailable

### Rate Limiting Responses

```json
{
  "error": "rate_limit_exceeded",
  "message": "Too many requests. Please try again later.",
  "details": {
    "limit": 100,
    "window": "1 minute",
    "retry_after": 45
  }
}
```

## Request/Response Examples

### Complete Learning Workflow

```bash
# 1. Authenticate
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "user", "password": "password"}'

# 2. Learn from text
curl -X POST http://localhost:8080/api/v1/learn \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -d '{
    "text": "Artificial intelligence is transforming healthcare",
    "priority": "high",
    "context": {"domain": "healthcare"}
  }'

# 3. Query related concepts
curl -X GET "http://localhost:8080/api/v1/concepts/artificial_intelligence/related?depth=2" \
  -H "Authorization: Bearer YOUR_TOKEN"

# 4. Search memory
curl -X GET "http://localhost:8080/api/v1/memory/search?query=healthcare&limit=5" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

This comprehensive API documentation provides developers with all the information needed to integrate with Brain AI's cognitive capabilities through RESTful endpoints.
