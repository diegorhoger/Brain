use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

use crate::agents::traits::{BrainAgent, AgentMetadata, AgentInput, AgentOutput, CognitiveContext};
use crate::agents::traits::BrainResult;
use brain_types::error::BrainError;

/// Observability Agent for comprehensive system monitoring and performance tracking
#[derive(Debug, Clone)]
pub struct ObservabilityAgent {
    metadata: AgentMetadata,
    config: ObservabilityConfig,
    cognitive_preferences: crate::agents::traits::CognitivePreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityConfig {
    pub monitoring_stack: MonitoringStack,
    pub metrics_config: MetricsConfig,
    pub logging_config: LoggingConfig,
    pub tracing_config: TracingConfig,
    pub alerting_config: AlertingConfig,
    pub dashboards_config: DashboardsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringStack {
    pub metrics_backend: MetricsBackend,
    pub logging_backend: LoggingBackend,
    pub tracing_backend: TracingBackend,
    pub dashboard_backend: DashboardBackend,
    pub alerting_backend: AlertingBackend,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricsBackend {
    Prometheus,
    DataDog,
    NewRelic,
    CloudWatch,
    Grafana,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoggingBackend {
    ElasticSearch,
    Splunk,
    CloudWatch,
    Fluentd,
    Loki,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TracingBackend {
    Jaeger,
    Zipkin,
    DataDog,
    Honeycomb,
    XRay,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DashboardBackend {
    Grafana,
    Kibana,
    DataDog,
    NewRelic,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertingBackend {
    Prometheus,
    PagerDuty,
    Slack,
    Email,
    Webhook,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct MetricsConfig {
    pub collection_interval_seconds: u32,
    pub retention_days: u32,
    pub custom_metrics: Vec<CustomMetric>,
    pub aggregation_rules: Vec<AggregationRule>,
    pub cardinality_limits: CardinalityLimits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct CustomMetric {
    pub name: String,
    pub metric_type: MetricType,
    pub description: String,
    pub labels: Vec<String>,
    pub source: MetricSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct MetricSource {
    pub source_type: SourceType,
    pub endpoint: String,
    pub query: String,
    pub authentication: Option<Authentication>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceType {
    HTTP,
    Database,
    File,
    CloudAPI,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Authentication {
    pub auth_type: AuthType,
    pub credentials: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthType {
    Bearer,
    ApiKey,
    BasicAuth,
    OAuth2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct AggregationRule {
    pub name: String,
    pub source_metrics: Vec<String>,
    pub aggregation_function: AggregationFunction,
    pub time_window_minutes: u32,
    pub output_metric: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AggregationFunction {
    Sum,
    Average,
    Max,
    Min,
    Percentile(f32),
    Count,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct CardinalityLimits {
    pub max_series_per_metric: u32,
    pub max_total_series: u32,
    pub cardinality_enforcement: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct LoggingConfig {
    pub log_levels: Vec<LogLevel>,
    pub retention_days: u32,
    pub structured_logging: bool,
    pub sampling_rate: f32,
    pub log_processors: Vec<LogProcessor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct LogProcessor {
    pub name: String,
    pub processor_type: ProcessorType,
    pub configuration: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessorType {
    Filter,
    Transform,
    Enrich,
    Route,
    Aggregate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct TracingConfig {
    pub sampling_rate: f32,
    pub trace_retention_days: u32,
    pub custom_spans: Vec<CustomSpan>,
    pub baggage_keys: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct CustomSpan {
    pub name: String,
    pub operation: String,
    pub tags: HashMap<String, String>,
    pub auto_instrument: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct AlertingConfig {
    pub alert_rules: Vec<AlertRule>,
    pub notification_channels: Vec<NotificationChannel>,
    pub escalation_policies: Vec<EscalationPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct AlertRule {
    pub name: String,
    pub condition: AlertCondition,
    pub severity: AlertSeverity,
    pub notification_channels: Vec<String>,
    pub cooldown_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct AlertCondition {
    pub metric: String,
    pub operator: ComparisonOperator,
    pub threshold: f64,
    pub duration_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonOperator {
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertSeverity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct NotificationChannel {
    pub name: String,
    pub channel_type: ChannelType,
    pub configuration: HashMap<String, String>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelType {
    Email,
    Slack,
    PagerDuty,
    Webhook,
    SMS,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct EscalationPolicy {
    pub name: String,
    pub escalation_steps: Vec<EscalationStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct EscalationStep {
    pub delay_minutes: u32,
    pub notification_channels: Vec<String>,
    pub auto_resolve: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct DashboardsConfig {
    pub default_dashboards: Vec<DefaultDashboard>,
    pub custom_dashboards: Vec<CustomDashboard>,
    pub dashboard_refresh_interval: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DefaultDashboard {
    SystemOverview,
    ApplicationMetrics,
    InfrastructureHealth,
    ErrorTracking,
    PerformanceAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct CustomDashboard {
    pub name: String,
    pub panels: Vec<DashboardPanel>,
    pub tags: Vec<String>,
    pub auto_refresh: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct DashboardPanel {
    pub title: String,
    pub panel_type: PanelType,
    pub query: String,
    pub time_range: TimeRange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PanelType {
    Graph,
    SingleStat,
    Table,
    Heatmap,
    Logs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct TimeRange {
    pub from: String,
    pub to: String,
    pub refresh_interval: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityInput {
    pub monitoring_request: MonitoringRequest,
    pub target_systems: Vec<TargetSystem>,
    pub monitoring_scope: MonitoringScope,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringRequest {
    pub request_type: MonitoringRequestType,
    pub priority: Priority,
    pub duration_hours: Option<u32>,
    pub specific_metrics: Vec<String>,
    pub alert_thresholds: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MonitoringRequestType {
    SetupMonitoring,
    UpdateConfiguration,
    GenerateReport,
    CreateDashboard,
    ConfigureAlerts,
    PerformanceAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct TargetSystem {
    pub system_name: String,
    pub system_type: SystemType,
    pub endpoints: Vec<String>,
    pub authentication: Option<Authentication>,
    pub health_check_endpoint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemType {
    WebApplication,
    Database,
    MessageQueue,
    Cache,
    LoadBalancer,
    Microservice,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringScope {
    pub include_infrastructure: bool,
    pub include_application: bool,
    pub include_business_metrics: bool,
    pub include_security_metrics: bool,
    pub custom_scopes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityOutput {
    pub monitoring_status: MonitoringStatus,
    pub collected_metrics: CollectedMetrics,
    pub active_alerts: Vec<ActiveAlert>,
    pub dashboard_links: Vec<DashboardLink>,
    pub performance_insights: Vec<PerformanceInsight>,
    pub recommendations: Vec<MonitoringRecommendation>,
    pub next_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringStatus {
    pub overall_health: HealthStatus,
    pub systems_monitored: u32,
    pub metrics_collected: u32,
    pub alerts_configured: u32,
    pub uptime_percentage: f32,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectedMetrics {
    pub system_metrics: SystemMetrics,
    pub application_metrics: ApplicationMetrics,
    pub business_metrics: BusinessMetrics,
    pub custom_metrics: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage_percent: f32,
    pub memory_usage_percent: f32,
    pub disk_usage_percent: f32,
    pub network_throughput_mbps: f32,
    pub load_average: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationMetrics {
    pub response_time_ms: f32,
    pub requests_per_second: f32,
    pub error_rate_percent: f32,
    pub active_connections: u32,
    pub queue_depth: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessMetrics {
    pub user_sessions: u32,
    pub conversion_rate: f32,
    pub revenue_per_hour: f32,
    pub customer_satisfaction_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveAlert {
    pub alert_id: String,
    pub alert_name: String,
    pub severity: AlertSeverity,
    pub description: String,
    pub triggered_at: DateTime<Utc>,
    pub metric_value: f64,
    pub threshold: f64,
    pub status: AlertStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertStatus {
    Firing,
    Resolved,
    Suppressed,
    Acknowledged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardLink {
    pub name: String,
    pub url: String,
    pub dashboard_type: DashboardType,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DashboardType {
    Overview,
    Detailed,
    Alerts,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceInsight {
    pub category: InsightCategory,
    pub description: String,
    pub impact: ImpactLevel,
    pub trend: TrendDirection,
    pub actionable_recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InsightCategory {
    Performance,
    Reliability,
    Cost,
    Security,
    UserExperience,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Improving,
    Stable,
    Degrading,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringRecommendation {
    pub category: RecommendationCategory,
    pub priority: Priority,
    pub description: String,
    pub implementation_steps: Vec<String>,
    pub estimated_impact: String,
    pub cost_estimate: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationCategory {
    ScaleUp,
    ScaleDown,
    OptimizeQuery,
    AddCaching,
    ImproveIndexing,
    ConfigureTuning,
}

impl Default for ObservabilityConfig {
    fn default() -> Self {
        Self {
            monitoring_stack: MonitoringStack {
                metrics_backend: MetricsBackend::Prometheus,
                logging_backend: LoggingBackend::ElasticSearch,
                tracing_backend: TracingBackend::Jaeger,
                dashboard_backend: DashboardBackend::Grafana,
                alerting_backend: AlertingBackend::Prometheus,
            },
            metrics_config: MetricsConfig {
                collection_interval_seconds: 30,
                retention_days: 30,
                custom_metrics: vec![],
                aggregation_rules: vec![],
                cardinality_limits: CardinalityLimits {
                    max_series_per_metric: 10000,
                    max_total_series: 100000,
                    cardinality_enforcement: true,
                },
            },
            logging_config: LoggingConfig {
                log_levels: vec![LogLevel::Error, LogLevel::Warn, LogLevel::Info],
                retention_days: 30,
                structured_logging: true,
                sampling_rate: 1.0,
                log_processors: vec![],
            },
            tracing_config: TracingConfig {
                sampling_rate: 0.1,
                trace_retention_days: 7,
                custom_spans: vec![],
                baggage_keys: vec![],
            },
            alerting_config: AlertingConfig {
                alert_rules: vec![],
                notification_channels: vec![],
                escalation_policies: vec![],
            },
            dashboards_config: DashboardsConfig {
                default_dashboards: vec![
                    DefaultDashboard::SystemOverview,
                    DefaultDashboard::ApplicationMetrics,
                ],
                custom_dashboards: vec![],
                dashboard_refresh_interval: 30,
            },
        }
    }
}

impl ObservabilityAgent {
    pub fn new() -> Self {
        let metadata = AgentMetadata {
            id: "observability_agent".to_string(),
            name: "ObservabilityAgent".to_string(),
            persona: "An expert monitoring and observability engineer specializing in comprehensive system health tracking, performance analysis, and proactive alerting".to_string(),
            description: "Comprehensive system monitoring and observability platform with metrics collection, alerting, dashboards, and performance insights".to_string(),
            version: "1.0.0".to_string(),
            supported_input_types: vec![
                "monitoring_request".to_string(),
                "metrics_collection".to_string(),
                "alert_configuration".to_string(),
                "performance_analysis".to_string(),
            ],
            supported_output_types: vec![
                "monitoring_status".to_string(),
                "metrics_report".to_string(),
                "alert_summary".to_string(),
                "performance_insights".to_string(),
            ],
            capabilities: vec![
                "Monitoring".to_string(),
                "Analytics".to_string(),
                "AlertManagement".to_string(),
                "ReportGeneration".to_string(),
            ],
            dependencies: vec![],
            tags: vec![
                "monitoring".to_string(),
                "observability".to_string(),
                "metrics".to_string(),
                "alerts".to_string(),
            ],
            base_confidence: 0.92,
        };

        Self {
            metadata,
            config: ObservabilityConfig::default(),
            cognitive_preferences: crate::agents::traits::CognitivePreferences::default(),
        }
    }

    pub fn with_config(mut self, config: ObservabilityConfig) -> Self {
        self.config = config;
        self
    }

    async fn collect_metrics(&self, _targets: &[TargetSystem], _context: &CognitiveContext) -> BrainResult<CollectedMetrics> {
        // Implementation would collect actual metrics from monitoring systems
        
        Ok(CollectedMetrics {
            system_metrics: SystemMetrics {
                cpu_usage_percent: 45.2,
                memory_usage_percent: 72.5,
                disk_usage_percent: 35.8,
                network_throughput_mbps: 125.3,
                load_average: 1.2,
            },
            application_metrics: ApplicationMetrics {
                response_time_ms: 180.5,
                requests_per_second: 245.0,
                error_rate_percent: 0.8,
                active_connections: 150,
                queue_depth: 12,
            },
            business_metrics: BusinessMetrics {
                user_sessions: 1250,
                conversion_rate: 3.2,
                revenue_per_hour: 1500.0,
                customer_satisfaction_score: 4.6,
            },
            custom_metrics: HashMap::from([
                ("cache_hit_rate".to_string(), 95.2),
                ("deployment_frequency".to_string(), 4.5),
            ]),
        })
    }

    async fn check_alerts(&self, _metrics: &CollectedMetrics, _context: &CognitiveContext) -> BrainResult<Vec<ActiveAlert>> {
        // Implementation would check actual alert conditions
        
        Ok(vec![
            ActiveAlert {
                alert_id: "alert-001".to_string(),
                alert_name: "High Memory Usage".to_string(),
                severity: AlertSeverity::Medium,
                description: "Memory usage exceeded 70% threshold".to_string(),
                triggered_at: Utc::now() - chrono::Duration::minutes(5),
                metric_value: 72.5,
                threshold: 70.0,
                status: AlertStatus::Firing,
            },
        ])
    }

    fn generate_performance_insights(&self, metrics: &CollectedMetrics, _alerts: &[ActiveAlert]) -> Vec<PerformanceInsight> {
        let mut insights = Vec::new();
        
        if metrics.application_metrics.response_time_ms > 200.0 {
            insights.push(PerformanceInsight {
                category: InsightCategory::Performance,
                description: "Response times are above optimal threshold".to_string(),
                impact: ImpactLevel::Medium,
                trend: TrendDirection::Degrading,
                actionable_recommendations: vec![
                    "Consider adding caching layer".to_string(),
                    "Optimize database queries".to_string(),
                    "Review application profiling data".to_string(),
                ],
            });
        }
        
        if metrics.system_metrics.memory_usage_percent > 70.0 {
            insights.push(PerformanceInsight {
                category: InsightCategory::Reliability,
                description: "Memory usage approaching capacity limits".to_string(),
                impact: ImpactLevel::High,
                trend: TrendDirection::Degrading,
                actionable_recommendations: vec![
                    "Scale up memory resources".to_string(),
                    "Investigate memory leaks".to_string(),
                    "Optimize memory-intensive operations".to_string(),
                ],
            });
        }
        
        insights
    }

    fn generate_monitoring_recommendations(&self, _metrics: &CollectedMetrics, insights: &[PerformanceInsight]) -> Vec<MonitoringRecommendation> {
        let mut recommendations = Vec::new();
        
        for insight in insights {
            match insight.category {
                InsightCategory::Performance => {
                    recommendations.push(MonitoringRecommendation {
                        category: RecommendationCategory::AddCaching,
                        priority: Priority::High,
                        description: "Implement caching to improve response times".to_string(),
                        implementation_steps: vec![
                            "Add Redis caching layer".to_string(),
                            "Implement cache-aside pattern".to_string(),
                            "Monitor cache hit rates".to_string(),
                        ],
                        estimated_impact: "30-50% response time improvement".to_string(),
                        cost_estimate: Some("$50-100/month for Redis instance".to_string()),
                    });
                },
                InsightCategory::Reliability => {
                    recommendations.push(MonitoringRecommendation {
                        category: RecommendationCategory::ScaleUp,
                        priority: Priority::High,
                        description: "Scale up memory resources to prevent OOM issues".to_string(),
                        implementation_steps: vec![
                            "Increase memory allocation".to_string(),
                            "Set up memory monitoring alerts".to_string(),
                            "Implement graceful degradation".to_string(),
                        ],
                        estimated_impact: "Prevent system instability and crashes".to_string(),
                        cost_estimate: Some("$200-500/month for increased resources".to_string()),
                    });
                },
                _ => {}
            }
        }
        
        recommendations
    }
}

#[async_trait]
impl BrainAgent for ObservabilityAgent {
    async fn execute(&self, input: AgentInput, context: &CognitiveContext) -> BrainResult<AgentOutput> {
        let observability_input: ObservabilityInput = serde_json::from_value(
            input.parameters.get("observability_input").unwrap_or(&serde_json::Value::Null).clone()
        ).map_err(|e| BrainError::InvalidInput(format!("Invalid observability input: {}", e)))?;

        // Collect metrics from target systems
        let collected_metrics = self.collect_metrics(&observability_input.target_systems, context).await?;

        // Check for active alerts
        let active_alerts = self.check_alerts(&collected_metrics, context).await?;

        // Generate monitoring status
        let monitoring_status = MonitoringStatus {
            overall_health: if active_alerts.iter().any(|a| matches!(a.severity, AlertSeverity::Critical)) {
                HealthStatus::Critical
            } else if active_alerts.iter().any(|a| matches!(a.severity, AlertSeverity::High | AlertSeverity::Medium)) {
                HealthStatus::Warning
            } else {
                HealthStatus::Healthy
            },
            systems_monitored: observability_input.target_systems.len() as u32,
            metrics_collected: 25, // Simplified count
            alerts_configured: self.config.alerting_config.alert_rules.len() as u32,
            uptime_percentage: 99.8,
            last_updated: Utc::now(),
        };

        // Generate dashboard links
        let dashboard_links = vec![
            DashboardLink {
                name: "System Overview".to_string(),
                url: "https://grafana.example.com/d/system-overview".to_string(),
                dashboard_type: DashboardType::Overview,
                description: "High-level system health and performance metrics".to_string(),
            },
            DashboardLink {
                name: "Application Metrics".to_string(),
                url: "https://grafana.example.com/d/app-metrics".to_string(),
                dashboard_type: DashboardType::Detailed,
                description: "Detailed application performance and business metrics".to_string(),
            },
        ];

        // Generate performance insights
        let performance_insights = self.generate_performance_insights(&collected_metrics, &active_alerts);

        // Generate recommendations
        let recommendations = self.generate_monitoring_recommendations(&collected_metrics, &performance_insights);

        // Generate next actions
        let next_actions = if active_alerts.is_empty() {
            vec![
                "Monitor system health and performance trends".to_string(),
                "Review and update alert thresholds".to_string(),
                "Optimize dashboard configurations".to_string(),
            ]
        } else {
            vec![
                format!("Address {} active alerts", active_alerts.len()),
                "Investigate performance degradation causes".to_string(),
                "Implement recommended optimizations".to_string(),
            ]
        };

        let observability_output = ObservabilityOutput {
            monitoring_status,
            collected_metrics,
            active_alerts,
            dashboard_links,
            performance_insights,
            recommendations,
            next_actions,
        };

        // Capture values before moving observability_output
        let systems_monitored = observability_output.monitoring_status.systems_monitored;
        let overall_health = observability_output.monitoring_status.overall_health.clone();
        let active_alerts_count = observability_output.active_alerts.len();
        let next_actions_clone = observability_output.next_actions.clone();

        Ok(AgentOutput {
            agent_id: self.metadata.id.clone(),
            output_type: "observability_analysis".to_string(),
            content: format!("Observability analysis completed for {} systems. Overall health: {:?}. {} active alerts detected.",
                           systems_monitored,
                           overall_health,
                           active_alerts_count),
            data: {
                let mut data = std::collections::HashMap::new();
                data.insert("observability_output".to_string(), serde_json::to_value(observability_output)?);
                data
            },
            confidence: match overall_health {
                HealthStatus::Healthy => 0.95,
                HealthStatus::Warning => 0.80,
                HealthStatus::Critical => 0.60,
                HealthStatus::Unknown => 0.40,
            },
            reasoning: Some("Analysis based on collected metrics, active alerts, and system health indicators".to_string()),
            next_actions: next_actions_clone,
            execution_metadata: crate::agents::traits::ExecutionMetadata {
                execution_time_ms: 5000, // 5 seconds for metrics collection and analysis
                memory_usage_mb: 128.0,
                api_calls: systems_monitored + 5, // Systems + API calls
                status: crate::agents::traits::ExecutionStatus::Success,
                warnings: Vec::new(),
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

    fn cognitive_preferences(&self) -> &crate::agents::traits::CognitivePreferences {
        &self.cognitive_preferences
    }

    async fn assess_confidence(&self, _input: &AgentInput, _context: &CognitiveContext) -> BrainResult<f32> {
        Ok(0.85) // High confidence for observability analysis
    }
} 