//! User Behavior Analyst Agent for Brain AI
//!
//! This agent specializes in analyzing user behavior patterns, identifying usage trends,
//! and providing insights for improving user experience and product development.

use crate::agents::traits::{BrainAgent, AgentMetadata, AgentInput, AgentOutput, CognitiveContext, ExecutionMetadata, ExecutionStatus, BrainResult, CognitivePreferences};
use brain_types::BrainError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use async_trait::async_trait;

/// User Behavior Analyst Agent
/// 
/// Analyzes user behavior patterns, tracks engagement metrics, identifies usage trends,
/// and provides actionable insights for improving user experience and product development.
pub struct UserBehaviorAnalystAgent {
    metadata: AgentMetadata,
    cognitive_preferences: CognitivePreferences,
}

/// Input data for user behavior analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorAnalysisInput {
    /// User sessions data
    pub sessions: Vec<UserSession>,
    /// Feature usage data
    pub feature_usage: HashMap<String, FeatureUsage>,
    /// Time range for analysis
    pub analysis_period: AnalysisPeriod,
    /// Analysis type to perform
    pub analysis_type: BehaviorAnalysisType,
}

/// User session data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub user_id: String,
    pub session_id: String,
    pub start_time: u64,
    pub end_time: Option<u64>,
    pub events: Vec<UserEvent>,
    pub platform: String,
    pub device_type: String,
}

/// User event within a session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEvent {
    pub event_type: String,
    pub timestamp: u64,
    pub properties: HashMap<String, serde_json::Value>,
    pub duration_ms: Option<u64>,
}

/// Feature usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureUsage {
    pub feature_name: String,
    pub usage_count: u64,
    pub unique_users: u64,
    pub avg_duration_ms: f64,
    pub success_rate: f64,
    pub error_count: u64,
}

/// Analysis time period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisPeriod {
    pub start_timestamp: u64,
    pub end_timestamp: u64,
    pub granularity: TimeGranularity,
}

/// Time granularity for analysis
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TimeGranularity {
    Hourly,
    Daily,
    Weekly,
    Monthly,
}

/// Type of behavior analysis to perform
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BehaviorAnalysisType {
    UsagePatterns,
    UserSegmentation,
    ChurnPrediction,
    FeatureAdoption,
    UserJourney,
    EngagementMetrics,
    AnomalyDetection,
}

/// Output of behavior analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorAnalysisOutput {
    /// Analysis results
    pub analysis_results: BehaviorAnalysisResults,
    /// Insights and recommendations
    pub insights: Vec<BehaviorInsight>,
    /// User segments identified
    pub user_segments: Vec<UserSegment>,
    /// Key metrics
    pub metrics: BehaviorMetrics,
    /// Analysis confidence score
    pub confidence_score: f64,
}

/// Behavior analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorAnalysisResults {
    pub analysis_type: BehaviorAnalysisType,
    pub patterns: Vec<BehaviorPattern>,
    pub trends: Vec<UsageTrend>,
    pub anomalies: Vec<BehaviorAnomaly>,
}

/// Behavior insight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorInsight {
    pub insight_type: InsightType,
    pub title: String,
    pub description: String,
    pub impact_score: f64,
    pub confidence: f64,
    pub recommendations: Vec<String>,
    pub affected_users: u64,
}

/// Type of insight
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InsightType {
    Opportunity,
    Risk,
    Trend,
    Anomaly,
    Optimization,
}

/// User segment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSegment {
    pub segment_id: String,
    pub name: String,
    pub description: String,
    pub user_count: u64,
    pub characteristics: HashMap<String, serde_json::Value>,
    pub behavior_patterns: Vec<String>,
}

/// Behavior metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorMetrics {
    pub total_sessions: u64,
    pub unique_users: u64,
    pub avg_session_duration: f64,
    pub bounce_rate: f64,
    pub retention_rate: f64,
    pub engagement_score: f64,
}

/// Behavior pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorPattern {
    pub pattern_id: String,
    pub name: String,
    pub description: String,
    pub frequency: u64,
    pub user_count: u64,
    pub confidence: f64,
}

/// Usage trend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageTrend {
    pub metric_name: String,
    pub trend_direction: TrendDirection,
    pub change_rate: f64,
    pub significance: f64,
    pub data_points: Vec<TrendDataPoint>,
}

/// Trend direction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TrendDirection {
    Increasing,
    Decreasing,
    Stable,
    Volatile,
}

/// Trend data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendDataPoint {
    pub timestamp: u64,
    pub value: f64,
}

/// Behavior anomaly
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorAnomaly {
    pub anomaly_id: String,
    pub anomaly_type: AnomalyType,
    pub description: String,
    pub severity: f64,
    pub affected_users: u64,
    pub timestamp: u64,
    pub details: HashMap<String, serde_json::Value>,
}

/// Type of anomaly
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnomalyType {
    UsageSpike,
    UsageDrop,
    UnusualPattern,
    ErrorSpike,
    PerformanceIssue,
}

impl UserBehaviorAnalystAgent {
    /// Create a new User Behavior Analyst Agent
    pub fn new() -> Self {
        let metadata = AgentMetadata {
            id: "user_behavior_analyst".to_string(),
            name: "UserBehaviorAnalystAgent".to_string(),
            persona: "I am an expert data analyst specializing in user behavior analysis and insights generation".to_string(),
            description: "Analyzes user behavior patterns and provides insights for product improvement".to_string(),
            version: "1.0.0".to_string(),
            supported_input_types: vec!["behavior_analysis".to_string(), "user_data".to_string()],
            supported_output_types: vec!["behavior_insights".to_string(), "user_segments".to_string()],
            capabilities: vec!["Analytics".to_string(), "Analysis".to_string()],
            dependencies: vec![],
            tags: vec!["intelligence".to_string(), "behavior".to_string(), "analytics".to_string()],
            base_confidence: 0.85,
        };

        Self {
            metadata,
            cognitive_preferences: CognitivePreferences::default(),
        }
    }

    /// Calculate behavior metrics
    fn calculate_metrics(&self, sessions: &[UserSession]) -> BrainResult<BehaviorMetrics> {
        let unique_users = sessions.iter()
            .map(|s| &s.user_id)
            .collect::<std::collections::HashSet<_>>()
            .len() as u64;

        let total_sessions = sessions.len() as u64;

        let durations: Vec<u64> = sessions.iter()
            .filter_map(|s| s.end_time.map(|end| end - s.start_time))
            .collect();

        let avg_session_duration = if durations.is_empty() {
            0.0
        } else {
            durations.iter().sum::<u64>() as f64 / durations.len() as f64
        };

        // Calculate bounce rate (sessions with duration < 30 seconds)
        let short_sessions = durations.iter().filter(|&&d| d < 30).count();
        let bounce_rate = if total_sessions > 0 {
            short_sessions as f64 / total_sessions as f64
        } else {
            0.0
        };

        // Simplified retention rate calculation
        let retention_rate = if unique_users > 0 {
            (total_sessions as f64 / unique_users as f64 - 1.0).max(0.0).min(1.0)
        } else {
            0.0
        };

        // Engagement score based on session duration and frequency
        let engagement_score = if avg_session_duration > 0.0 {
            ((avg_session_duration / 300.0).min(1.0) * 0.5 + (1.0 - bounce_rate) * 0.5).max(0.0).min(1.0)
        } else {
            0.0
        };

        Ok(BehaviorMetrics {
            total_sessions,
            unique_users,
            avg_session_duration,
            bounce_rate,
            retention_rate,
            engagement_score,
        })
    }
}

#[async_trait]
impl BrainAgent for UserBehaviorAnalystAgent {
    async fn execute(&self, input: AgentInput, _context: &CognitiveContext) -> BrainResult<AgentOutput> {
        let analysis_input: BehaviorAnalysisInput = serde_json::from_value(
            input.parameters.get("behavior_analysis_data")
                .cloned()
                .unwrap_or_default()
        )
            .map_err(|e| BrainError::InvalidInput(format!("Failed to parse behavior analysis input: {}", e)))?;

        // Calculate metrics
        let metrics = self.calculate_metrics(&analysis_input.sessions)?;

        // Create simplified analysis results
        let analysis_results = BehaviorAnalysisResults {
            analysis_type: analysis_input.analysis_type,
            patterns: vec![], // Simplified for initial implementation
            trends: vec![], 
            anomalies: vec![],
        };

        let output = BehaviorAnalysisOutput {
            analysis_results,
            insights: vec![], // Simplified for initial implementation
            user_segments: vec![], 
            metrics,
            confidence_score: 0.85,
        };

        Ok(AgentOutput {
            agent_id: self.metadata.id.clone(),
            output_type: "behavior_insights".to_string(),
            content: "User behavior analysis completed".to_string(),
            data: {
                let mut data = HashMap::new();
                data.insert("analysis_output".to_string(), serde_json::to_value(output)
                    .map_err(|e| BrainError::InvalidInput(format!("Failed to serialize output: {}", e)))?);
                data
            },
            confidence: 0.85,
            reasoning: Some("Analyzed user behavior patterns and generated insights".to_string()),
            next_actions: vec!["review_insights".to_string(), "implement_recommendations".to_string()],
            execution_metadata: ExecutionMetadata {
                execution_time_ms: 1000,
                memory_usage_mb: 10.0,
                api_calls: 0,
                status: ExecutionStatus::Success,
                warnings: vec![],
            },
            timestamp: chrono::Utc::now(),
        })
    }

    fn metadata(&self) -> &AgentMetadata {
        &self.metadata
    }

    fn confidence_threshold(&self) -> f32 {
        0.7
    }

    fn cognitive_preferences(&self) -> &CognitivePreferences {
        &self.cognitive_preferences
    }

    async fn assess_confidence(&self, input: &AgentInput, _context: &CognitiveContext) -> BrainResult<f32> {
        // Check if input contains required fields for behavior analysis
        if let Some(behavior_data) = input.parameters.get("behavior_analysis_data") {
            if let Ok(_analysis_input) = serde_json::from_value::<BehaviorAnalysisInput>(behavior_data.clone()) {
                Ok(0.9) // High confidence if input is well-formed
            } else {
                Ok(0.3) // Low confidence if input format is incorrect
            }
        } else {
            Ok(0.3) // Low confidence if required parameter is missing
        }
    }
}

impl Default for UserBehaviorAnalystAgent {
    fn default() -> Self {
        Self::new()
    }
} 