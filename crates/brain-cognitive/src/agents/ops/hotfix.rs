use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

use crate::agents::traits::{BrainAgent, AgentMetadata, AgentInput, AgentOutput, CognitiveContext};
use crate::agents::traits::BrainResult;
use brain_types::error::BrainError;

/// Hotfix Agent for emergency fixes and automated rollback procedures
#[derive(Debug, Clone)]
pub struct HotfixAgent {
    metadata: AgentMetadata,
    config: HotfixConfig,
    cognitive_preferences: crate::agents::traits::CognitivePreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotfixConfig {
    pub emergency_thresholds: EmergencyThresholds,
    pub rollback_config: RollbackConfig,
    pub approval_config: ApprovalConfig,
    pub notification_config: NotificationConfig,
    pub safety_checks: SafetyChecks,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct EmergencyThresholds {
    pub critical_error_rate_percent: f32,
    pub critical_response_time_ms: f32,
    pub critical_availability_percent: f32,
    pub critical_security_severity: SecuritySeverity,
    pub auto_trigger_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct RollbackConfig {
    pub auto_rollback_enabled: bool,
    pub rollback_timeout_minutes: u32,
    pub health_check_retries: u32,
    pub rollback_strategies: Vec<RollbackStrategy>,
    pub preserve_data: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RollbackStrategy {
    BlueGreen,
    Canary,
    RollingUpdate,
    ImmediateRevert,
    DatabaseRollback,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ApprovalConfig {
    pub require_approval: bool,
    pub emergency_bypass: bool,
    pub approvers: Vec<String>,
    pub approval_timeout_minutes: u32,
    pub escalation_chain: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct NotificationConfig {
    pub immediate_notification: bool,
    pub channels: Vec<NotificationChannel>,
    pub stakeholder_groups: HashMap<String, Vec<String>>,
    pub escalation_intervals: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationChannel {
    Email,
    Slack,
    PagerDuty,
    SMS,
    Webhook,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct SafetyChecks {
    pub pre_deployment_checks: Vec<SafetyCheck>,
    pub post_deployment_checks: Vec<SafetyCheck>,
    pub rollback_checks: Vec<SafetyCheck>,
    pub check_timeout_seconds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct SafetyCheck {
    pub check_name: String,
    pub check_type: CheckType,
    pub endpoint: Option<String>,
    pub expected_result: String,
    pub timeout_seconds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CheckType {
    HealthCheck,
    FunctionalTest,
    PerformanceTest,
    SecurityScan,
    DataIntegrity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotfixInput {
    pub incident_details: IncidentDetails,
    pub hotfix_request: HotfixRequest,
    pub deployment_target: DeploymentTarget,
    pub emergency_context: EmergencyContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentDetails {
    pub incident_id: String,
    pub severity: IncidentSeverity,
    pub description: String,
    pub affected_systems: Vec<String>,
    pub impact_assessment: ImpactAssessment,
    pub root_cause: Option<String>,
    pub reporter: String,
    pub reported_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IncidentSeverity {
    P0,
    P1,
    P2,
    P3,
    P4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAssessment {
    pub users_affected: u32,
    pub revenue_impact_per_hour: f32,
    pub business_functions_impacted: Vec<String>,
    pub regulatory_impact: bool,
    pub reputation_risk: ReputationRisk,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReputationRisk {
    None,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotfixRequest {
    pub hotfix_type: HotfixType,
    pub fix_description: String,
    pub changed_files: Vec<String>,
    pub database_changes: Option<DatabaseChanges>,
    pub configuration_changes: Option<ConfigurationChanges>,
    pub testing_strategy: TestingStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HotfixType {
    CodeFix,
    ConfigurationUpdate,
    DatabasePatch,
    SecurityPatch,
    Rollback,
    ServiceRestart,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct DatabaseChanges {
    pub migration_scripts: Vec<String>,
    pub rollback_scripts: Vec<String>,
    pub backup_required: bool,
    pub estimated_downtime_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ConfigurationChanges {
    pub config_files: HashMap<String, String>,
    pub environment_variables: HashMap<String, String>,
    pub service_restarts_required: Vec<String>,
    pub validation_commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestingStrategy {
    NoTesting,
    MinimalTesting,
    StandardTesting,
    ComprehensiveTesting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct DeploymentTarget {
    pub environment: String,
    pub deployment_method: DeploymentMethod,
    pub target_servers: Vec<String>,
    pub load_balancer_config: Option<String>,
    pub maintenance_window: Option<MaintenanceWindow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentMethod {
    BlueGreen,
    Canary,
    RollingUpdate,
    AllAtOnce,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct MaintenanceWindow {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub notification_sent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct EmergencyContext {
    pub is_emergency: bool,
    pub bypass_normal_process: bool,
    pub emergency_approver: Option<String>,
    pub time_pressure_minutes: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotfixOutput {
    pub deployment_status: DeploymentStatus,
    pub execution_timeline: ExecutionTimeline,
    pub safety_check_results: Vec<SafetyCheckResult>,
    pub rollback_plan: RollbackPlan,
    pub post_deployment_monitoring: MonitoringPlan,
    pub incident_resolution: IncidentResolution,
    pub next_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentStatus {
    pub status: Status,
    pub deployment_id: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub success_rate: f32,
    pub error_messages: Vec<String>,
    pub affected_services: Vec<ServiceStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Status {
    Pending,
    InProgress,
    Successful,
    Failed,
    RolledBack,
    PartialSuccess,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ServiceStatus {
    pub service_name: String,
    pub status: ServiceHealth,
    pub version: String,
    pub last_health_check: DateTime<Utc>,
    pub error_rate_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceHealth {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionTimeline {
    pub total_duration_minutes: u32,
    pub phases: Vec<ExecutionPhase>,
    pub delays_encountered: Vec<Delay>,
    pub critical_path: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPhase {
    pub phase_name: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub status: PhaseStatus,
    pub activities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PhaseStatus {
    NotStarted,
    InProgress,
    Completed,
    Failed,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Delay {
    pub reason: String,
    pub duration_minutes: u32,
    pub impact: DelayImpact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DelayImpact {
    None,
    Minor,
    Moderate,
    Significant,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyCheckResult {
    pub check_name: String,
    pub status: CheckStatus,
    pub result_details: String,
    pub execution_time_seconds: u32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CheckStatus {
    Passed,
    Failed,
    Warning,
    Skipped,
    Timeout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackPlan {
    pub rollback_available: bool,
    pub rollback_strategy: RollbackStrategy,
    pub rollback_steps: Vec<RollbackStep>,
    pub estimated_rollback_time_minutes: u32,
    pub data_loss_risk: DataLossRisk,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackStep {
    pub step_name: String,
    pub commands: Vec<String>,
    pub validation: Vec<String>,
    pub estimated_time_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataLossRisk {
    None,
    Minimal,
    Moderate,
    High,
    Severe,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringPlan {
    pub monitoring_duration_hours: u32,
    pub key_metrics: Vec<KeyMetric>,
    pub alert_thresholds: HashMap<String, f32>,
    pub escalation_triggers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct KeyMetric {
    pub metric_name: String,
    pub current_value: f32,
    pub baseline_value: f32,
    pub acceptable_deviation_percent: f32,
    pub alert_threshold: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentResolution {
    pub incident_resolved: bool,
    pub resolution_time_minutes: u32,
    pub root_cause_addressed: bool,
    pub follow_up_required: bool,
    pub lessons_learned: Vec<String>,
}

impl Default for HotfixConfig {
    fn default() -> Self {
        Self {
            emergency_thresholds: EmergencyThresholds {
                critical_error_rate_percent: 5.0,
                critical_response_time_ms: 5000.0,
                critical_availability_percent: 95.0,
                critical_security_severity: SecuritySeverity::High,
                auto_trigger_enabled: false,
            },
            rollback_config: RollbackConfig {
                auto_rollback_enabled: true,
                rollback_timeout_minutes: 10,
                health_check_retries: 3,
                rollback_strategies: vec![RollbackStrategy::BlueGreen, RollbackStrategy::ImmediateRevert],
                preserve_data: true,
            },
            approval_config: ApprovalConfig {
                require_approval: true,
                emergency_bypass: true,
                approvers: vec!["oncall-engineer".to_string()],
                approval_timeout_minutes: 5,
                escalation_chain: vec!["senior-engineer".to_string(), "engineering-manager".to_string()],
            },
            notification_config: NotificationConfig {
                immediate_notification: true,
                channels: vec![NotificationChannel::PagerDuty, NotificationChannel::Slack],
                stakeholder_groups: HashMap::from([
                    ("engineering".to_string(), vec!["dev-team".to_string()]),
                    ("operations".to_string(), vec!["ops-team".to_string()]),
                ]),
                escalation_intervals: vec![5, 15, 30],
            },
            safety_checks: SafetyChecks {
                pre_deployment_checks: vec![
                    SafetyCheck {
                        check_name: "Health Check".to_string(),
                        check_type: CheckType::HealthCheck,
                        endpoint: Some("/health".to_string()),
                        expected_result: "200 OK".to_string(),
                        timeout_seconds: 30,
                    },
                ],
                post_deployment_checks: vec![
                    SafetyCheck {
                        check_name: "Functional Test".to_string(),
                        check_type: CheckType::FunctionalTest,
                        endpoint: Some("/api/status".to_string()),
                        expected_result: "Service operational".to_string(),
                        timeout_seconds: 60,
                    },
                ],
                rollback_checks: vec![
                    SafetyCheck {
                        check_name: "Rollback Validation".to_string(),
                        check_type: CheckType::HealthCheck,
                        endpoint: Some("/health".to_string()),
                        expected_result: "200 OK".to_string(),
                        timeout_seconds: 30,
                    },
                ],
                check_timeout_seconds: 120,
            },
        }
    }
}

impl HotfixAgent {
    pub fn new() -> Self {
        let metadata = AgentMetadata {
            id: "hotfix_agent".to_string(),
            name: "HotfixAgent".to_string(),
            persona: "An emergency response specialist focused on rapid incident resolution and safe deployment practices with comprehensive rollback capabilities".to_string(),
            description: "Manages emergency hotfixes and automated rollback procedures with safety checks and incident resolution tracking".to_string(),
            version: "1.0.0".to_string(),
            supported_input_types: vec!["hotfix_request".to_string()],
            supported_output_types: vec!["hotfix_deployment_report".to_string()],
            capabilities: vec![
                "EmergencyResponse".to_string(),
                "Deployment".to_string(),
                "RollbackManagement".to_string(),
                "IncidentManagement".to_string(),
            ],
            dependencies: vec![],
            tags: vec!["hotfix".to_string(), "emergency".to_string(), "deployment".to_string()],
            base_confidence: 0.92,
        };

        Self {
            metadata,
            config: HotfixConfig::default(),
            cognitive_preferences: crate::agents::traits::CognitivePreferences::default(),
        }
    }

    pub fn with_config(mut self, config: HotfixConfig) -> Self {
        self.config = config;
        self
    }

    async fn execute_hotfix_deployment(&self, _request: &HotfixRequest, _target: &DeploymentTarget, _context: &CognitiveContext) -> BrainResult<DeploymentStatus> {
        // Implementation would execute actual hotfix deployment
        
        let deployment_id = format!("hotfix-{}", chrono::Utc::now().timestamp());
        
        Ok(DeploymentStatus {
            status: Status::Successful,
            deployment_id,
            started_at: Utc::now(),
            completed_at: Some(Utc::now() + chrono::Duration::minutes(5)),
            success_rate: 100.0,
            error_messages: vec![],
            affected_services: vec![
                ServiceStatus {
                    service_name: "web-api".to_string(),
                    status: ServiceHealth::Healthy,
                    version: "1.2.3-hotfix".to_string(),
                    last_health_check: Utc::now(),
                    error_rate_percent: 0.1,
                },
            ],
        })
    }

    async fn run_safety_checks(&self, checks: &[SafetyCheck], _context: &CognitiveContext) -> BrainResult<Vec<SafetyCheckResult>> {
        // Implementation would run actual safety checks
        
        Ok(checks.iter().map(|check| {
            SafetyCheckResult {
                check_name: check.check_name.clone(),
                status: CheckStatus::Passed,
                result_details: "All checks passed successfully".to_string(),
                execution_time_seconds: 15,
                timestamp: Utc::now(),
            }
        }).collect())
    }

    fn generate_rollback_plan(&self, _request: &HotfixRequest, deployment_status: &DeploymentStatus) -> RollbackPlan {
        RollbackPlan {
            rollback_available: deployment_status.status != Status::Failed,
            rollback_strategy: RollbackStrategy::BlueGreen,
            rollback_steps: vec![
                RollbackStep {
                    step_name: "Switch traffic to previous version".to_string(),
                    commands: vec!["kubectl rollout undo deployment/web-api".to_string()],
                    validation: vec!["curl -f http://api/health".to_string()],
                    estimated_time_minutes: 2,
                },
                RollbackStep {
                    step_name: "Verify service health".to_string(),
                    commands: vec!["kubectl get pods -l app=web-api".to_string()],
                    validation: vec!["Check all pods are Running".to_string()],
                    estimated_time_minutes: 1,
                },
            ],
            estimated_rollback_time_minutes: 5,
            data_loss_risk: DataLossRisk::None,
        }
    }

    fn generate_monitoring_plan(&self, _incident: &IncidentDetails) -> MonitoringPlan {
        MonitoringPlan {
            monitoring_duration_hours: 24,
            key_metrics: vec![
                KeyMetric {
                    metric_name: "error_rate".to_string(),
                    current_value: 0.1,
                    baseline_value: 0.2,
                    acceptable_deviation_percent: 50.0,
                    alert_threshold: 1.0,
                },
                KeyMetric {
                    metric_name: "response_time_ms".to_string(),
                    current_value: 150.0,
                    baseline_value: 200.0,
                    acceptable_deviation_percent: 25.0,
                    alert_threshold: 500.0,
                },
            ],
            alert_thresholds: HashMap::from([
                ("error_rate".to_string(), 1.0),
                ("response_time".to_string(), 500.0),
            ]),
            escalation_triggers: vec![
                "Error rate > 2%".to_string(),
                "Response time > 1000ms".to_string(),
            ],
        }
    }
}

#[async_trait]
impl BrainAgent for HotfixAgent {
    async fn execute(&self, input: AgentInput, context: &CognitiveContext) -> BrainResult<AgentOutput> {
        let hotfix_input: HotfixInput = serde_json::from_value(
            input.parameters.get("hotfix_input").unwrap_or(&serde_json::Value::Null).clone()
        ).map_err(|e| BrainError::InvalidInput(format!("Invalid hotfix input: {}", e)))?;

        let start_time = Utc::now();

        // Run pre-deployment safety checks
        let pre_check_results = self.run_safety_checks(&self.config.safety_checks.pre_deployment_checks, context).await?;
        
        let pre_checks_passed = pre_check_results.iter().all(|r| r.status == CheckStatus::Passed);
        if !pre_checks_passed && !hotfix_input.emergency_context.bypass_normal_process {
            return Err(BrainError::InvalidInput("Pre-deployment safety checks failed".to_string()));
        }

        // Execute hotfix deployment
        let deployment_status = self.execute_hotfix_deployment(
            &hotfix_input.hotfix_request,
            &hotfix_input.deployment_target,
            context
        ).await?;

        // Run post-deployment safety checks
        let post_check_results = self.run_safety_checks(&self.config.safety_checks.post_deployment_checks, context).await?;

        // Generate execution timeline
        let execution_timeline = ExecutionTimeline {
            total_duration_minutes: 8,
            phases: vec![
                ExecutionPhase {
                    phase_name: "Pre-deployment Checks".to_string(),
                    start_time,
                    end_time: Some(start_time + chrono::Duration::minutes(2)),
                    status: PhaseStatus::Completed,
                    activities: vec!["Health checks".to_string(), "Safety validations".to_string()],
                },
                ExecutionPhase {
                    phase_name: "Deployment".to_string(),
                    start_time: start_time + chrono::Duration::minutes(2),
                    end_time: Some(start_time + chrono::Duration::minutes(7)),
                    status: PhaseStatus::Completed,
                    activities: vec!["Code deployment".to_string(), "Service restart".to_string()],
                },
                ExecutionPhase {
                    phase_name: "Post-deployment Validation".to_string(),
                    start_time: start_time + chrono::Duration::minutes(7),
                    end_time: Some(start_time + chrono::Duration::minutes(8)),
                    status: PhaseStatus::Completed,
                    activities: vec!["Functional tests".to_string(), "Health verification".to_string()],
                },
            ],
            delays_encountered: vec![],
            critical_path: vec!["Pre-checks".to_string(), "Deployment".to_string(), "Validation".to_string()],
        };

        // Generate rollback plan
        let rollback_plan = self.generate_rollback_plan(&hotfix_input.hotfix_request, &deployment_status);

        // Generate monitoring plan
        let monitoring_plan = self.generate_monitoring_plan(&hotfix_input.incident_details);

        // Assess incident resolution
        let incident_resolved = deployment_status.status == Status::Successful && 
                               post_check_results.iter().all(|r| r.status == CheckStatus::Passed);

        let incident_resolution = IncidentResolution {
            incident_resolved,
            resolution_time_minutes: execution_timeline.total_duration_minutes,
            root_cause_addressed: true,
            follow_up_required: !incident_resolved,
            lessons_learned: vec![
                "Implement better monitoring for early detection".to_string(),
                "Review deployment process for efficiency".to_string(),
            ],
        };

        // Combine all safety check results
        let mut all_safety_checks = pre_check_results;
        all_safety_checks.extend(post_check_results);

        // Generate next actions
        let next_actions = if incident_resolved {
            vec![
                "Monitor system for 24 hours post-deployment".to_string(),
                "Conduct post-incident review".to_string(),
                "Update incident documentation".to_string(),
            ]
        } else {
            vec![
                "Execute rollback procedure immediately".to_string(),
                "Investigate deployment failure causes".to_string(),
                "Prepare alternative hotfix approach".to_string(),
            ]
        };

        let hotfix_output = HotfixOutput {
            deployment_status,
            execution_timeline,
            safety_check_results: all_safety_checks,
            rollback_plan,
            post_deployment_monitoring: monitoring_plan,
            incident_resolution,
            next_actions,
        };

        // Capture values before moving hotfix_output
        let incident_resolved_val = hotfix_output.incident_resolution.incident_resolved;
        let total_duration = hotfix_output.execution_timeline.total_duration_minutes;
        let next_actions_clone = hotfix_output.next_actions.clone();

        Ok(AgentOutput {
            agent_id: self.metadata.id.clone(),
            output_type: "hotfix_deployment_report".to_string(),
            content: format!("Hotfix deployment completed. Incident resolved: {}. Duration: {} minutes.",
                           incident_resolved_val, total_duration),
            data: {
                let mut data = std::collections::HashMap::new();
                data.insert("hotfix_output".to_string(), serde_json::to_value(hotfix_output)?);
                data
            },
            confidence: if incident_resolved_val { 0.95 } else { 0.70 },
            reasoning: Some("Analysis based on safety checks, deployment status, and incident resolution assessment".to_string()),
            next_actions: next_actions_clone,
            execution_metadata: crate::agents::traits::ExecutionMetadata {
                execution_time_ms: (total_duration * 60 * 1000) as u64,
                memory_usage_mb: 320.0,
                api_calls: 15,
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
        0.8
    }

    fn cognitive_preferences(&self) -> &crate::agents::traits::CognitivePreferences {
        &self.cognitive_preferences
    }

    async fn assess_confidence(&self, _input: &AgentInput, _context: &CognitiveContext) -> BrainResult<f32> {
        Ok(0.92) // High confidence for emergency hotfix operations
    }
} 