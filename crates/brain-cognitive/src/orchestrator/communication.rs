//! Inter-Agent Communication System

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Communication bus for agent messaging
#[derive(Debug)]
pub struct AgentCommunicationBus {
    channels: Arc<RwLock<HashMap<String, broadcast::Sender<AgentMessage>>>>,
}

impl AgentCommunicationBus {
    pub fn new() -> Self {
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub async fn create_channel(&self, topic: &str) -> broadcast::Receiver<AgentMessage> {
        let mut channels = self.channels.write().await;
        let (sender, receiver) = broadcast::channel(100);
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
            id: uuid::Uuid::new_v4().to_string(),
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
