//! Inter-Agent Communication System
//! 
//! Comprehensive communication infrastructure for orchestrated agents including
//! message routing, request-response patterns, event-driven communication,
//! delivery confirmation, and communication metrics.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{RwLock, broadcast, oneshot, Mutex};
use tokio::time::{timeout, Instant};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use brain_types::error::BrainError;
use crate::meta::{MetaMemoryService, KnowledgeType};

/// Enhanced communication bus for agent messaging with comprehensive features
pub struct AgentCommunicationBus {
    /// Message channels organized by topic
    channels: Arc<RwLock<HashMap<String, broadcast::Sender<AgentMessage>>>>,
    
    /// Message routing table for direct agent-to-agent communication
    routing_table: Arc<RwLock<HashMap<String, String>>>, // agent_id -> topic
    
    /// Pending request-response pairs
    pending_requests: Arc<Mutex<HashMap<String, oneshot::Sender<AgentMessage>>>>,
    
    /// Message persistence store for replay capability
    message_store: Arc<RwLock<Vec<StoredMessage>>>,
    
    /// Communication metrics
    metrics: Arc<RwLock<CommunicationMetrics>>,
    
    /// Integration with MetaMemory for tracking communication patterns
    meta_memory: Option<Arc<MetaMemoryService>>,
    
    /// Configuration
    config: CommunicationConfig,
}

/// Configuration for the communication system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationConfig {
    /// Maximum number of messages to store for replay
    pub max_stored_messages: usize,
    
    /// Default timeout for request-response operations
    pub default_timeout_ms: u64,
    
    /// Enable message persistence
    pub enable_persistence: bool,
    
    /// Enable delivery confirmation
    pub enable_delivery_confirmation: bool,
    
    /// Enable MetaMemory integration
    pub enable_meta_memory_tracking: bool,
    
    /// Maximum channel capacity
    pub max_channel_capacity: usize,
}

impl Default for CommunicationConfig {
    fn default() -> Self {
        Self {
            max_stored_messages: 1000,
            default_timeout_ms: 5000,
            enable_persistence: true,
            enable_delivery_confirmation: true,
            enable_meta_memory_tracking: true,
            max_channel_capacity: 1000,
        }
    }
}

/// Communication metrics for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationMetrics {
    pub total_messages_sent: u64,
    pub total_messages_received: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub timeouts: u64,
    pub active_channels: usize,
    pub active_agents: usize,
    pub average_response_time_ms: f64,
    pub message_types_sent: HashMap<String, u64>,
    pub agent_communication_matrix: HashMap<String, HashMap<String, u64>>, // from -> to -> count
}

impl Default for CommunicationMetrics {
    fn default() -> Self {
        Self {
            total_messages_sent: 0,
            total_messages_received: 0,
            successful_requests: 0,
            failed_requests: 0,
            timeouts: 0,
            active_channels: 0,
            active_agents: 0,
            average_response_time_ms: 0.0,
            message_types_sent: HashMap::new(),
            agent_communication_matrix: HashMap::new(),
        }
    }
}

/// Stored message for persistence and replay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredMessage {
    pub message: AgentMessage,
    pub topic: String,
    pub stored_at: DateTime<Utc>,
    pub delivery_status: DeliveryStatus,
}

/// Message delivery status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeliveryStatus {
    Pending,
    Delivered,
    Failed(String),
    Timeout,
}

impl AgentCommunicationBus {
    /// Create a new communication bus with default configuration
    pub fn new() -> Self {
        Self::with_config(CommunicationConfig::default())
    }
    
    /// Create a new communication bus with custom configuration
    pub fn with_config(config: CommunicationConfig) -> Self {
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
            routing_table: Arc::new(RwLock::new(HashMap::new())),
            pending_requests: Arc::new(Mutex::new(HashMap::new())),
            message_store: Arc::new(RwLock::new(Vec::new())),
            metrics: Arc::new(RwLock::new(CommunicationMetrics::default())),
            meta_memory: None,
            config,
        }
    }
    
    /// Create communication bus with MetaMemory integration
    pub fn with_meta_memory(
        config: CommunicationConfig,
        meta_memory: Arc<MetaMemoryService>,
    ) -> Self {
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
            routing_table: Arc::new(RwLock::new(HashMap::new())),
            pending_requests: Arc::new(Mutex::new(HashMap::new())),
            message_store: Arc::new(RwLock::new(Vec::new())),
            metrics: Arc::new(RwLock::new(CommunicationMetrics::default())),
            meta_memory: Some(meta_memory),
            config,
        }
    }
    
    /// Register an agent for communication
    pub async fn register_agent(&self, agent_id: &str) -> Result<String, BrainError> {
        let topic = format!("agent.{}", agent_id);
        
        // Create dedicated channel for the agent
        let mut channels = self.channels.write().await;
        let (sender, _receiver) = broadcast::channel(self.config.max_channel_capacity);
        channels.insert(topic.clone(), sender);
        
        // Add to routing table
        let mut routing_table = self.routing_table.write().await;
        routing_table.insert(agent_id.to_string(), topic.clone());
        
        // Update metrics
        let mut metrics = self.metrics.write().await;
        metrics.active_agents += 1;
        metrics.active_channels = channels.len();
        
        // Track in MetaMemory
        if self.config.enable_meta_memory_tracking {
            if let Some(meta_memory) = &self.meta_memory {
                let agent_uuid = Uuid::new_v4();
                let _ = meta_memory.track_component(
                    agent_uuid,
                    KnowledgeType::OrchestrationNamespace,
                    0.9,
                    format!("Agent {} registered for communication", agent_id),
                ).await;
            }
        }
        
        Ok(topic)
    }
    
    /// Send a message to a specific agent
    pub async fn send_message(
        &self,
        message: AgentMessage,
    ) -> Result<(), BrainError> {
        let start_time = Instant::now();
        
        // Determine routing
        let topic = if let Some(ref to_agent) = message.to_agent {
            let routing_table = self.routing_table.read().await;
            routing_table.get(to_agent).cloned()
                .unwrap_or_else(|| format!("agent.{}", to_agent))
        } else {
            "broadcast".to_string()
        };
        
        // Send message
        let channels = self.channels.read().await;
        if let Some(sender) = channels.get(&topic) {
            match sender.send(message.clone()) {
                Ok(_) => {
                    // Update metrics
                    self.update_send_metrics(&message, start_time.elapsed()).await;
                    
                    // Store message if persistence is enabled
                    if self.config.enable_persistence {
                        self.store_message(message.clone(), topic, DeliveryStatus::Delivered).await;
                    }
                    
                    Ok(())
                }
                Err(_) => {
                    let error_msg = format!("Failed to send message to topic: {}", topic);
                    
                    // Store failure
                    if self.config.enable_persistence {
                        self.store_message(message, topic, DeliveryStatus::Failed(error_msg.clone())).await;
                    }
                    
                    Err(BrainError::Other(error_msg))
                }
            }
        } else {
            Err(BrainError::Other(format!("Topic not found: {}", topic)))
        }
    }
    
    /// Send a request and wait for response
    pub async fn send_request(
        &self,
        mut request: AgentMessage,
        timeout_ms: Option<u64>,
    ) -> Result<AgentMessage, BrainError> {
        let timeout_duration = Duration::from_millis(
            timeout_ms.unwrap_or(self.config.default_timeout_ms)
        );
        
        // Set up response channel
        let (response_sender, response_receiver) = oneshot::channel();
        let request_id = request.id.clone();
        
        // Register pending request
        {
            let mut pending = self.pending_requests.lock().await;
            pending.insert(request_id.clone(), response_sender);
        }
        
        // Set message type to request
        request.message_type = MessageType::Request;
        request.reply_to = Some(request_id.clone());
        
        // Send the request
        self.send_message(request).await?;
        
        // Wait for response with timeout
        match timeout(timeout_duration, response_receiver).await {
            Ok(Ok(response)) => {
                // Update success metrics
                let mut metrics = self.metrics.write().await;
                metrics.successful_requests += 1;
                Ok(response)
            }
            Ok(Err(_)) => {
                // Cleanup pending request
                let mut pending = self.pending_requests.lock().await;
                pending.remove(&request_id);
                
                let mut metrics = self.metrics.write().await;
                metrics.failed_requests += 1;
                
                Err(BrainError::Other("Request cancelled".to_string()))
            }
            Err(_) => {
                // Timeout occurred
                let mut pending = self.pending_requests.lock().await;
                pending.remove(&request_id);
                
                let mut metrics = self.metrics.write().await;
                metrics.timeouts += 1;
                
                Err(BrainError::Other("Request timeout".to_string()))
            }
        }
    }
    
    /// Send a response to a request
    pub async fn send_response(
        &self,
        request_id: &str,
        response_payload: serde_json::Value,
        from_agent: &str,
    ) -> Result<(), BrainError> {
        // Check if there's a pending request
        let response_sender = {
            let mut pending = self.pending_requests.lock().await;
            pending.remove(request_id)
        };
        
        if let Some(sender) = response_sender {
            let response = AgentMessage {
                id: Uuid::new_v4().to_string(),
                from_agent: from_agent.to_string(),
                to_agent: None,
                message_type: MessageType::Response,
                payload: response_payload,
                timestamp: Utc::now(),
                reply_to: Some(request_id.to_string()),
                correlation_id: Some(request_id.to_string()),
            };
            
            // Send response through the oneshot channel
            sender.send(response).map_err(|_| {
                BrainError::Other("Failed to send response".to_string())
            })?;
            
            Ok(())
        } else {
            Err(BrainError::Other(format!("No pending request found for ID: {}", request_id)))
        }
    }
    
    /// Subscribe to messages for an agent
    pub async fn subscribe_agent(&self, agent_id: &str) -> Result<broadcast::Receiver<AgentMessage>, BrainError> {
        let topic = format!("agent.{}", agent_id);
        let channels = self.channels.read().await;
        
        if let Some(sender) = channels.get(&topic) {
            Ok(sender.subscribe())
        } else {
            Err(BrainError::Other(format!("Agent not registered: {}", agent_id)))
        }
    }
    
    /// Broadcast message to all agents
    pub async fn broadcast_message(&self, message: AgentMessage) -> Result<usize, BrainError> {
        let channels = self.channels.read().await;
        let mut sent_count = 0;
        
        for (topic, sender) in channels.iter() {
            if topic != "broadcast" { // Avoid infinite loop
                if sender.send(message.clone()).is_ok() {
                    sent_count += 1;
                }
            }
        }
        
        // Update metrics
        self.update_send_metrics(&message, Duration::from_millis(0)).await;
        
        Ok(sent_count)
    }
    
    /// Get communication metrics
    pub async fn get_metrics(&self) -> CommunicationMetrics {
        let metrics = self.metrics.read().await;
        metrics.clone()
    }
    
    /// Get message history for replay
    pub async fn get_message_history(
        &self,
        agent_id: Option<&str>,
        limit: Option<usize>,
    ) -> Vec<StoredMessage> {
        let messages = self.message_store.read().await;
        let filtered: Vec<_> = if let Some(agent_id) = agent_id {
            messages.iter()
                .filter(|msg| {
                    msg.message.from_agent == agent_id || 
                    msg.message.to_agent.as_ref() == Some(&agent_id.to_string())
                })
                .cloned()
                .collect()
        } else {
            messages.clone()
        };
        
        let limit = limit.unwrap_or(100);
        if filtered.len() > limit {
            filtered[filtered.len() - limit..].to_vec()
        } else {
            filtered
        }
    }
    
    /// Helper method to update send metrics
    async fn update_send_metrics(&self, message: &AgentMessage, duration: Duration) {
        let mut metrics = self.metrics.write().await;
        metrics.total_messages_sent += 1;
        
        // Update message type statistics
        let msg_type = format!("{:?}", message.message_type);
        *metrics.message_types_sent.entry(msg_type).or_insert(0) += 1;
        
        // Update communication matrix
        if let Some(ref to_agent) = message.to_agent {
            let from_agent = message.from_agent.clone();
            let to_count = metrics.agent_communication_matrix
                .entry(from_agent)
                .or_insert_with(HashMap::new);
            *to_count.entry(to_agent.clone()).or_insert(0) += 1;
        }
        
        // Update average response time
        let duration_ms = duration.as_millis() as f64;
        metrics.average_response_time_ms = (metrics.average_response_time_ms + duration_ms) / 2.0;
    }
    
    /// Helper method to store messages
    async fn store_message(&self, message: AgentMessage, topic: String, status: DeliveryStatus) {
        let mut store = self.message_store.write().await;
        
        let stored_message = StoredMessage {
            message,
            topic,
            stored_at: Utc::now(),
            delivery_status: status,
        };
        
        store.push(stored_message);
        
        // Trim to max size
        if store.len() > self.config.max_stored_messages {
            store.remove(0);
        }
    }
    
    pub async fn create_channel(&self, topic: &str) -> broadcast::Receiver<AgentMessage> {
        let mut channels = self.channels.write().await;
        let (sender, receiver) = broadcast::channel(self.config.max_channel_capacity);
        channels.insert(topic.to_string(), sender);
        receiver
    }
    
    pub async fn get_channel_count(&self) -> usize {
        let channels = self.channels.read().await;
        channels.len()
    }
    
    pub async fn cleanup_unused_channels(&self) {
        let mut channels = self.channels.write().await;
        channels.retain(|_, sender| sender.receiver_count() > 0);
        
        // Update metrics
        let mut metrics = self.metrics.write().await;
        metrics.active_channels = channels.len();
    }
}

/// Message bus trait for pluggable communication backends
pub trait MessageBus: Send + Sync {
    fn publish(&self, topic: &str, message: AgentMessage) -> impl std::future::Future<Output = ()> + Send;
    fn subscribe(&self, topic: &str) -> impl std::future::Future<Output = broadcast::Receiver<AgentMessage>> + Send;
}

/// Message passed between agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    pub id: String,
    pub from_agent: String,
    pub to_agent: Option<String>,
    pub message_type: MessageType,
    pub payload: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub reply_to: Option<String>,
    pub correlation_id: Option<String>,
}

/// Types of messages that can be sent between agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Request,
    Response,
    Notification,
    Error,
    Heartbeat,
}

/// Communication protocol for agent interactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationProtocol {
    pub protocol_name: String,
    pub version: String,
    pub supported_message_types: Vec<MessageType>,
    pub security_enabled: bool,
}

/// Event trigger for agent activation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTrigger {
    pub trigger_id: String,
    pub event_type: EventType,
    pub target_agents: Vec<String>,
    pub condition: TriggerCondition,
}

/// Types of events that can trigger agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    AgentCompleted,
    AgentFailed,
    DataReceived,
    TimeoutExpired,
    UserRequest,
}

/// Conditions for event triggers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TriggerCondition {
    Always,
    OnSuccess,
    OnFailure,
    OnData(String),
    Custom(serde_json::Value),
}

impl AgentMessage {
    pub fn new(
        from_agent: String,
        message_type: MessageType,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            from_agent,
            to_agent: None,
            message_type,
            payload,
            timestamp: Utc::now(),
            reply_to: None,
            correlation_id: None,
        }
    }
    
    pub fn to_agent(mut self, agent_id: String) -> Self {
        self.to_agent = Some(agent_id);
        self
    }
    
    pub fn with_correlation_id(mut self, correlation_id: String) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }
}

impl MessageBus for AgentCommunicationBus {
    fn publish(&self, topic: &str, message: AgentMessage) -> impl std::future::Future<Output = ()> + Send {
        let channels = Arc::clone(&self.channels);
        async move {
            let channels = channels.read().await;
            if let Some(sender) = channels.get(topic) {
                let _ = sender.send(message);
            }
        }
    }
    
    fn subscribe(&self, topic: &str) -> impl std::future::Future<Output = broadcast::Receiver<AgentMessage>> + Send {
        let channels = Arc::clone(&self.channels);
        let topic = topic.to_string();
        async move {
            let mut channels = channels.write().await;
            let (sender, receiver) = broadcast::channel(100);
            channels.insert(topic, sender);
            receiver
        }
    }
}
