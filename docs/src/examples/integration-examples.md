# Integration Examples

This guide demonstrates how to integrate Brain AI with popular frameworks, databases, and services.

## Web Framework Integration

### Axum (Rust) Integration

```rust
use axum::{
    extract::{State, Json},
    routing::{get, post},
    Router, response::Json as ResponseJson,
};
use brain_ai::BrainSystem;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
struct AppState {
    brain: Arc<Mutex<BrainSystem>>,
}

#[derive(Deserialize)]
struct LearnRequest {
    content: String,
    priority: Option<String>,
}

#[derive(Serialize)]
struct LearnResponse {
    memory_id: String,
    confidence: f64,
    concepts_count: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let brain = BrainSystem::new().await?;
    let state = AppState {
        brain: Arc::new(Mutex::new(brain)),
    };

    let app = Router::new()
        .route("/api/v1/learn", post(learn_handler))
        .route("/api/v1/query", post(query_handler))
        .route("/api/v1/health", get(health_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn learn_handler(
    State(state): State<AppState>,
    Json(request): Json<LearnRequest>,
) -> Result<ResponseJson<LearnResponse>, String> {
    let mut brain = state.brain.lock().await;
    
    let result = brain.process_input(&request.content).await
        .map_err(|e| format!("Learning failed: {}", e))?;
    
    let concepts = brain.get_concepts().await
        .map_err(|e| format!("Failed to get concepts: {}", e))?;
    
    Ok(ResponseJson(LearnResponse {
        memory_id: result.memory_id.unwrap_or_default(),
        confidence: result.confidence,
        concepts_count: concepts.len(),
    }))
}
```

### FastAPI (Python) Integration

```python
from fastapi import FastAPI, HTTPException, BackgroundTasks
from pydantic import BaseModel
import brain_ai
import asyncio
from typing import List, Optional

app = FastAPI(title="Brain AI API", version="1.0.0")

# Global Brain AI instance
brain = None

class LearnRequest(BaseModel):
    content: str
    priority: Optional[str] = "medium"

class QueryRequest(BaseModel):
    query: str
    limit: Optional[int] = 10

class MemoryResponse(BaseModel):
    memory_id: str
    content: str
    confidence: float

@app.on_event("startup")
async def startup_event():
    global brain
    brain = await brain_ai.AsyncBrainSystem.new()

@app.post("/api/v1/learn")
async def learn_endpoint(request: LearnRequest):
    try:
        result = await brain.process_input(request.content)
        return {
            "memory_id": result.memory_id,
            "confidence": result.confidence,
            "success": True
        }
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

@app.post("/api/v1/query")
async def query_endpoint(request: QueryRequest) -> List[MemoryResponse]:
    try:
        memories = await brain.search_memories(request.query, limit=request.limit)
        return [
            MemoryResponse(
                memory_id=memory.id,
                content=memory.content,
                confidence=memory.confidence
            )
            for memory in memories
        ]
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

@app.get("/api/v1/insights")
async def get_insights():
    try:
        insights = await brain.extract_insights()
        return {"insights": [insight.description for insight in insights]}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))
```

## Database Integration

### PostgreSQL Integration

```rust
use brain_ai::BrainSystem;
use sqlx::{PgPool, Row};
use serde_json;

struct DatabaseIntegration {
    brain: BrainSystem,
    db_pool: PgPool,
}

impl DatabaseIntegration {
    async fn new(database_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let brain = BrainSystem::new().await?;
        let db_pool = PgPool::connect(database_url).await?;
        
        Ok(Self { brain, db_pool })
    }
    
    async fn process_database_records(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Query database for unprocessed records
        let rows = sqlx::query(
            "SELECT id, content, created_at FROM documents WHERE processed = false"
        )
        .fetch_all(&self.db_pool)
        .await?;
        
        for row in rows {
            let id: i64 = row.get("id");
            let content: String = row.get("content");
            
            // Process with Brain AI
            let result = self.brain.process_input(&content).await?;
            
            // Store results back to database
            let insights = self.brain.extract_insights().await?;
            let insights_json = serde_json::to_string(&insights)?;
            
            sqlx::query(
                "UPDATE documents SET processed = true, brain_ai_insights = $1, memory_id = $2 WHERE id = $3"
            )
            .bind(&insights_json)
            .bind(&result.memory_id.unwrap_or_default())
            .bind(id)
            .execute(&self.db_pool)
            .await?;
        }
        
        Ok(())
    }
    
    async fn search_enhanced_records(&self, query: &str) -> Result<Vec<EnhancedRecord>, Box<dyn std::error::Error>> {
        // First, search Brain AI memories
        let brain_results = self.brain.search_memories(query).await?;
        
        // Then enhance with database information
        let mut enhanced_records = Vec::new();
        
        for memory in brain_results {
            if let Some(memory_id) = &memory.id {
                let row = sqlx::query(
                    "SELECT id, content, created_at, brain_ai_insights FROM documents WHERE memory_id = $1"
                )
                .bind(memory_id)
                .fetch_optional(&self.db_pool)
                .await?;
                
                if let Some(row) = row {
                    enhanced_records.push(EnhancedRecord {
                        database_id: row.get("id"),
                        content: row.get("content"),
                        brain_ai_memory: memory,
                        insights: row.get("brain_ai_insights"),
                    });
                }
            }
        }
        
        Ok(enhanced_records)
    }
}
```

### MongoDB Integration

```rust
use brain_ai::BrainSystem;
use mongodb::{Client, Collection, bson::doc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Document {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<mongodb::bson::oid::ObjectId>,
    content: String,
    processed: bool,
    memory_id: Option<String>,
    insights: Option<Vec<String>>,
}

struct MongoIntegration {
    brain: BrainSystem,
    collection: Collection<Document>,
}

impl MongoIntegration {
    async fn new(connection_string: &str, database: &str, collection: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let brain = BrainSystem::new().await?;
        let client = Client::with_uri_str(connection_string).await?;
        let db = client.database(database);
        let collection = db.collection::<Document>(collection);
        
        Ok(Self { brain, collection })
    }
    
    async fn process_unprocessed_documents(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let filter = doc! { "processed": false };
        let mut cursor = self.collection.find(filter, None).await?;
        
        while let Some(doc) = cursor.next().await {
            let mut document = doc?;
            
            // Process with Brain AI
            let result = self.brain.process_input(&document.content).await?;
            let insights = self.brain.extract_insights().await?;
            
            // Update document
            document.processed = true;
            document.memory_id = result.memory_id;
            document.insights = Some(insights.iter().map(|i| i.description.clone()).collect());
            
            // Save back to MongoDB
            let filter = doc! { "_id": document.id };
            let update = doc! {
                "$set": {
                    "processed": true,
                    "memory_id": &document.memory_id,
                    "insights": &document.insights
                }
            };
            
            self.collection.update_one(filter, update, None).await?;
        }
        
        Ok(())
    }
}
```

## Message Queue Integration

### Apache Kafka Integration

```rust
use brain_ai::BrainSystem;
use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    producer::{FutureProducer, FutureRecord},
    config::ClientConfig,
    message::Message,
};
use tokio_stream::StreamExt;

struct KafkaIntegration {
    brain: BrainSystem,
    consumer: StreamConsumer,
    producer: FutureProducer,
}

impl KafkaIntegration {
    async fn new(brokers: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let brain = BrainSystem::new().await?;
        
        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", "brain-ai-consumer")
            .set("bootstrap.servers", brokers)
            .set("auto.offset.reset", "latest")
            .create()?;
        
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .create()?;
        
        consumer.subscribe(&["brain-ai-input"])?;
        
        Ok(Self { brain, consumer, producer })
    }
    
    async fn start_processing(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut message_stream = self.consumer.stream();
        
        while let Some(message) = message_stream.next().await {
            match message {
                Ok(msg) => {
                    if let Some(payload) = msg.payload_view::<str>() {
                        match payload {
                            Ok(text) => {
                                // Process with Brain AI
                                let result = self.brain.process_input(text).await?;
                                
                                // Publish results
                                let output = serde_json::json!({
                                    "original_text": text,
                                    "memory_id": result.memory_id,
                                    "confidence": result.confidence,
                                    "timestamp": chrono::Utc::now().to_rfc3339()
                                });
                                
                                let record = FutureRecord::to("brain-ai-output")
                                    .payload(&output.to_string())
                                    .key("processed");
                                
                                self.producer.send(record, None).await?;
                            }
                            Err(e) => eprintln!("Error parsing message: {}", e),
                        }
                    }
                }
                Err(e) => eprintln!("Error receiving message: {}", e),
            }
        }
        
        Ok(())
    }
}
```

### Redis Integration

```rust
use brain_ai::BrainSystem;
use redis::{AsyncCommands, Client};
use tokio::time::{interval, Duration};

struct RedisIntegration {
    brain: BrainSystem,
    redis_client: Client,
}

impl RedisIntegration {
    async fn new(redis_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let brain = BrainSystem::new().await?;
        let redis_client = Client::open(redis_url)?;
        
        Ok(Self { brain, redis_client })
    }
    
    async fn start_queue_processing(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut con = self.redis_client.get_async_connection().await?;
        
        loop {
            // Block and wait for items in the queue
            let result: Option<(String, String)> = con.blpop("brain-ai:input", 0).await?;
            
            if let Some((_, content)) = result {
                // Process with Brain AI
                let result = self.brain.process_input(&content).await?;
                
                // Store result in Redis
                let output = serde_json::json!({
                    "memory_id": result.memory_id,
                    "confidence": result.confidence,
                    "processed_at": chrono::Utc::now().to_rfc3339()
                });
                
                con.rpush("brain-ai:output", output.to_string()).await?;
                
                // Also cache in Redis for quick access
                if let Some(memory_id) = &result.memory_id {
                    con.setex(
                        format!("brain-ai:memory:{}", memory_id),
                        3600, // 1 hour TTL
                        &content
                    ).await?;
                }
            }
        }
    }
    
    async fn start_cache_warming(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut interval = interval(Duration::from_secs(300)); // 5 minutes
        let mut con = self.redis_client.get_async_connection().await?;
        
        loop {
            interval.tick().await;
            
            // Get recent insights and cache them
            let insights = self.brain.extract_insights().await?;
            
            for insight in insights {
                let key = format!("brain-ai:insight:{}", insight.id);
                let value = serde_json::to_string(&insight)?;
                con.setex(key, 1800, value).await?; // 30 minutes TTL
            }
        }
    }
}
```

This guide provides practical examples for integrating Brain AI with various frameworks and services commonly used in production environments.
