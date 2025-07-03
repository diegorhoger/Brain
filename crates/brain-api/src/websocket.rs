use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex, RwLock};
use uuid::Uuid;
use warp::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::agents::{AgentStatus, SystemHealth};

/// Types of WebSocket messages for real-time updates
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum WebSocketMessage {
    /// Agent execution started
    AgentExecutionStarted {
        execution_id: String,
        agent_name: String,
        started_at: DateTime<Utc>,
        user_id: Option<String>,
    },
    /// Agent execution completed
    AgentExecutionCompleted {
        execution_id: String,
        agent_name: String,
        completed_at: DateTime<Utc>,
        success: bool,
        result: Option<serde_json::Value>,
        error: Option<String>,
    },
    /// Agent execution progress update
    AgentExecutionProgress {
        execution_id: String,
        agent_name: String,
        progress: f64, // 0.0 to 1.0
        stage: String,
        message: Option<String>,
    },
    /// Agent status changed
    AgentStatusChanged {
        agent_name: String,
        status: AgentStatus,
        timestamp: DateTime<Utc>,
    },
    /// System health update
    SystemHealthUpdate {
        health: SystemHealth,
        timestamp: DateTime<Utc>,
    },
    /// Workflow execution update
    WorkflowExecutionUpdate {
        workflow_id: String,
        stage: String,
        agents_completed: Vec<String>,
        agents_pending: Vec<String>,
        overall_progress: f64,
    },
    /// CPP configuration changed
    ProfileConfigurationChanged {
        user_id: String,
        profile_name: String,
        changes: serde_json::Value,
        timestamp: DateTime<Utc>,
    },
    /// Resource usage alert
    ResourceUsageAlert {
        resource_type: String, // "cpu", "memory", "api_calls"
        current_usage: f64,
        threshold: f64,
        severity: String, // "warning", "critical"
        timestamp: DateTime<Utc>,
    },
    /// General notification
    Notification {
        level: String, // "info", "warning", "error"
        title: String,
        message: String,
        timestamp: DateTime<Utc>,
    },
    /// Connection acknowledgment
    Connected {
        client_id: String,
        server_time: DateTime<Utc>,
    },
    /// Heartbeat/ping message
    Heartbeat {
        timestamp: DateTime<Utc>,
    },
}

/// Client subscription preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionRequest {
    pub agent_names: Option<Vec<String>>, // Subscribe to specific agents only
    pub message_types: Option<Vec<String>>, // Subscribe to specific message types
    pub user_id: Option<String>, // Filter by user ID
    pub include_system_health: bool,
    pub include_resource_alerts: bool,
}

/// WebSocket client information
#[derive(Debug)]
pub struct WebSocketClient {
    pub id: String,
    pub sender: tokio::sync::mpsc::UnboundedSender<Message>,
    pub subscriptions: SubscriptionRequest,
    pub connected_at: DateTime<Utc>,
    pub last_heartbeat: DateTime<Utc>,
}

/// WebSocket manager for handling real-time communication
pub struct WebSocketManager {
    /// Connected clients
    clients: Arc<RwLock<HashMap<String, WebSocketClient>>>,
    /// Broadcast channel for sending messages to all clients
    broadcast_tx: broadcast::Sender<WebSocketMessage>,
    /// Background task handles
    _handles: Arc<Mutex<Vec<tokio::task::JoinHandle<()>>>>,
}

impl WebSocketManager {
    /// Create a new WebSocket manager
    pub fn new() -> Self {
        let (broadcast_tx, _) = broadcast::channel(1000);
        
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
            broadcast_tx,
            _handles: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Add a new WebSocket client
    pub async fn add_client(&self, ws: WebSocket) -> String {
        let client_id = Uuid::new_v4().to_string();
        let (mut ws_tx, mut ws_rx) = ws.split();
        let (client_tx, mut client_rx) = tokio::sync::mpsc::unbounded_channel();

        // Send connection acknowledgment
        let connect_msg = WebSocketMessage::Connected {
            client_id: client_id.clone(),
            server_time: Utc::now(),
        };
        
        if let Ok(msg_json) = serde_json::to_string(&connect_msg) {
            let _ = client_tx.send(Message::text(msg_json));
        }

        // Create client info with default subscriptions
        let client = WebSocketClient {
            id: client_id.clone(),
            sender: client_tx,
            subscriptions: SubscriptionRequest {
                agent_names: None,
                message_types: None,
                user_id: None,
                include_system_health: true,
                include_resource_alerts: true,
            },
            connected_at: Utc::now(),
            last_heartbeat: Utc::now(),
        };

        // Add client to the manager
        {
            let mut clients = self.clients.write().await;
            clients.insert(client_id.clone(), client);
        }

        let clients_for_cleanup = self.clients.clone();
        let client_id_for_cleanup = client_id.clone();

        // Task to send messages to this client
        let send_task = tokio::spawn(async move {
            while let Some(message) = client_rx.recv().await {
                if ws_tx.send(message).await.is_err() {
                    break;
                }
            }
        });

        // Task to handle incoming messages from this client
        let _broadcast_tx = self.broadcast_tx.clone();
        let clients_for_rx = self.clients.clone();
        let client_id_for_rx = client_id.clone();
        
        let receive_task = tokio::spawn(async move {
            while let Some(result) = ws_rx.next().await {
                match result {
                    Ok(msg) => {
                        if let Ok(text) = msg.to_str() {
                            // Handle subscription updates or heartbeat responses
                            if let Ok(sub_req) = serde_json::from_str::<SubscriptionRequest>(text) {
                                let mut clients = clients_for_rx.write().await;
                                if let Some(client) = clients.get_mut(&client_id_for_rx) {
                                    client.subscriptions = sub_req;
                                    client.last_heartbeat = Utc::now();
                                }
                            }
                        }
                    }
                    Err(_) => break,
                }
            }

            // Cleanup on disconnect
            let mut clients = clients_for_cleanup.write().await;
            clients.remove(&client_id_for_cleanup);
        });

        // Store task handles for cleanup
        {
            let mut handles = self._handles.lock().await;
            handles.push(send_task);
            handles.push(receive_task);
        }

        // Start heartbeat for this client
        self.start_heartbeat(&client_id).await;

        client_id
    }

    /// Broadcast a message to all subscribed clients
    pub async fn broadcast(&self, message: WebSocketMessage) {
        let clients = self.clients.read().await;
        
        for client in clients.values() {
            if self.should_send_to_client(client, &message) {
                let msg_json = match serde_json::to_string(&message) {
                    Ok(json) => json,
                    Err(_) => continue,
                };
                
                let _ = client.sender.send(Message::text(msg_json));
            }
        }
    }

    /// Send message to specific client
    pub async fn send_to_client(&self, client_id: &str, message: WebSocketMessage) {
        let clients = self.clients.read().await;
        if let Some(client) = clients.get(client_id) {
            if let Ok(msg_json) = serde_json::to_string(&message) {
                let _ = client.sender.send(Message::text(msg_json));
            }
        }
    }

    /// Get number of connected clients
    pub async fn client_count(&self) -> usize {
        let clients = self.clients.read().await;
        clients.len()
    }

    /// Get client information
    pub async fn get_client_info(&self, client_id: &str) -> Option<(DateTime<Utc>, SubscriptionRequest)> {
        let clients = self.clients.read().await;
        clients.get(client_id).map(|client| (client.connected_at, client.subscriptions.clone()))
    }

    /// Start periodic heartbeat for a client
    async fn start_heartbeat(&self, client_id: &str) {
        let clients = self.clients.clone();
        let client_id = client_id.to_string();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
            
            loop {
                interval.tick().await;
                
                let heartbeat_msg = WebSocketMessage::Heartbeat {
                    timestamp: Utc::now(),
                };
                
                let clients_guard = clients.read().await;
                if let Some(client) = clients_guard.get(&client_id) {
                    if let Ok(msg_json) = serde_json::to_string(&heartbeat_msg) {
                        if client.sender.send(Message::text(msg_json)).is_err() {
                            break; // Client disconnected
                        }
                    }
                } else {
                    break; // Client no longer exists
                }
            }
        });
    }

    /// Check if message should be sent to specific client based on subscriptions
    fn should_send_to_client(&self, client: &WebSocketClient, message: &WebSocketMessage) -> bool {
        match message {
            WebSocketMessage::AgentExecutionStarted { agent_name, user_id, .. } => {
                // Check agent name filter
                if let Some(ref subscribed_agents) = client.subscriptions.agent_names {
                    if !subscribed_agents.contains(agent_name) {
                        return false;
                    }
                }
                
                // Check user ID filter
                if let Some(ref client_user_id) = client.subscriptions.user_id {
                    if let Some(ref msg_user_id) = user_id {
                        if client_user_id != msg_user_id {
                            return false;
                        }
                    }
                }
                
                true
            }
            WebSocketMessage::AgentExecutionCompleted { agent_name, .. } |
            WebSocketMessage::AgentExecutionProgress { agent_name, .. } |
            WebSocketMessage::AgentStatusChanged { agent_name, .. } => {
                // Check agent name filter
                if let Some(ref subscribed_agents) = client.subscriptions.agent_names {
                    if !subscribed_agents.contains(agent_name) {
                        return false;
                    }
                }
                
                true
            }
            WebSocketMessage::SystemHealthUpdate { .. } => {
                client.subscriptions.include_system_health
            }
            WebSocketMessage::ResourceUsageAlert { .. } => {
                client.subscriptions.include_resource_alerts
            }
            WebSocketMessage::ProfileConfigurationChanged { user_id, .. } => {
                if let Some(ref client_user_id) = client.subscriptions.user_id {
                    client_user_id == user_id
                } else {
                    true
                }
            }
            // Always send system messages
            WebSocketMessage::Connected { .. } |
            WebSocketMessage::Heartbeat { .. } |
            WebSocketMessage::Notification { .. } => true,
            _ => true,
        }
    }
}

impl Default for WebSocketManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience functions for sending specific types of messages
impl WebSocketManager {
    /// Send agent execution started notification
    pub async fn notify_agent_execution_started(
        &self,
        execution_id: String,
        agent_name: String,
        user_id: Option<String>,
    ) {
        let message = WebSocketMessage::AgentExecutionStarted {
            execution_id,
            agent_name,
            started_at: Utc::now(),
            user_id,
        };
        self.broadcast(message).await;
    }

    /// Send agent execution completed notification
    pub async fn notify_agent_execution_completed(
        &self,
        execution_id: String,
        agent_name: String,
        success: bool,
        result: Option<serde_json::Value>,
        error: Option<String>,
    ) {
        let message = WebSocketMessage::AgentExecutionCompleted {
            execution_id,
            agent_name,
            completed_at: Utc::now(),
            success,
            result,
            error,
        };
        self.broadcast(message).await;
    }

    /// Send agent execution progress update
    pub async fn notify_agent_execution_progress(
        &self,
        execution_id: String,
        agent_name: String,
        progress: f64,
        stage: String,
        message: Option<String>,
    ) {
        let msg = WebSocketMessage::AgentExecutionProgress {
            execution_id,
            agent_name,
            progress,
            stage,
            message,
        };
        self.broadcast(msg).await;
    }

    /// Send system notification
    pub async fn notify_system(
        &self,
        level: &str,
        title: String,
        message: String,
    ) {
        let notification = WebSocketMessage::Notification {
            level: level.to_string(),
            title,
            message,
            timestamp: Utc::now(),
        };
        self.broadcast(notification).await;
    }
} 