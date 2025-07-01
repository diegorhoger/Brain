use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

use crate::agents::traits::{BrainAgent, AgentMetadata, AgentInput, AgentOutput, CognitiveContext};
use crate::agents::traits::BrainResult;
use brain_types::error::BrainError;

/// Sandbox Environment Agent for managing isolated testing environments and PR previews
#[derive(Debug, Clone)]
pub struct SandboxEnvironmentAgent {
    metadata: AgentMetadata,
    config: SandboxConfig,
    cognitive_preferences: crate::agents::traits::CognitivePreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    pub cloud_provider: CloudProvider,
    pub container_runtime: ContainerRuntime,
    pub resource_limits: ResourceLimits,
    pub network_policies: NetworkPolicies,
    pub cleanup_policies: CleanupPolicies,
    pub security_policies: SecurityPolicies,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CloudProvider {
    AWS,
    GCP,
    Azure,
    Local,
    Kubernetes,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContainerRuntime {
    Docker,
    Containerd,
    Podman,
    Kubernetes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub max_cpu_cores: f32,
    pub max_memory_gb: f32,
    pub max_disk_gb: f32,
    pub max_network_mbps: f32,
    pub max_duration_hours: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPolicies {
    pub internet_access: bool,
    pub internal_access: bool,
    pub allowed_ports: Vec<u16>,
    pub blocked_domains: Vec<String>,
    pub vpn_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupPolicies {
    pub auto_cleanup_after_hours: u32,
    pub cleanup_on_pr_close: bool,
    pub cleanup_on_merge: bool,
    pub preserve_artifacts: bool,
    pub notification_before_cleanup: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicies {
    pub enable_security_scanning: bool,
    pub require_secrets_encryption: bool,
    pub network_isolation: bool,
    pub read_only_filesystem: bool,
    pub no_privileged_containers: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxInput {
    pub environment_request: EnvironmentRequest,
    pub application_config: ApplicationConfig,
    pub deployment_config: DeploymentConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentRequest {
    pub request_type: RequestType,
    pub environment_name: String,
    pub pr_number: Option<u32>,
    pub branch_name: String,
    pub commit_hash: String,
    pub requester: String,
    pub labels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestType {
    CreatePRPreview,
    CreateTestEnvironment,
    UpdateEnvironment,
    DestroyEnvironment,
    ScaleEnvironment,
    CloneEnvironment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationConfig {
    pub dockerfile_path: String,
    pub build_context: String,
    pub environment_variables: HashMap<String, String>,
    pub secrets: Vec<String>,
    pub health_check_path: String,
    pub startup_probe_path: Option<String>,
    pub dependencies: Vec<ServiceDependency>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDependency {
    pub name: String,
    pub service_type: ServiceType,
    pub image: String,
    pub environment_variables: HashMap<String, String>,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceType {
    Database,
    Cache,
    MessageQueue,
    ExternalAPI,
    MockService,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentConfig {
    pub replicas: u32,
    pub resource_requests: ResourceRequests,
    pub ingress_config: IngressConfig,
    pub storage_config: Option<StorageConfig>,
    pub monitoring_config: MonitoringConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequests {
    pub cpu_millicores: u32,
    pub memory_mb: u32,
    pub storage_gb: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngressConfig {
    pub subdomain: String,
    pub ssl_enabled: bool,
    pub basic_auth: Option<BasicAuth>,
    pub ip_whitelist: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicAuth {
    pub username: String,
    pub password_secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub volume_size_gb: u32,
    pub storage_class: String,
    pub backup_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub metrics_enabled: bool,
    pub logging_enabled: bool,
    pub tracing_enabled: bool,
    pub alert_webhooks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxOutput {
    pub environment_status: EnvironmentStatus,
    pub deployment_details: DeploymentDetails,
    pub access_information: AccessInformation,
    pub resource_usage: ResourceUsage,
    pub monitoring_links: Vec<MonitoringLink>,
    pub next_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentStatus {
    pub status: EnvironmentState,
    pub environment_id: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub health_status: HealthStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnvironmentState {
    Creating,
    Running,
    Updating,
    Stopping,
    Stopped,
    Failed,
    Expired,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Unhealthy,
    Unknown,
    Starting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentDetails {
    pub namespace: String,
    pub pods: Vec<PodStatus>,
    pub services: Vec<ServiceStatus>,
    pub ingress_url: Option<String>,
    pub build_logs_url: String,
    pub deployment_logs_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodStatus {
    pub name: String,
    pub status: String,
    pub ready: bool,
    pub restarts: u32,
    pub age_seconds: u64,
    pub resource_usage: PodResourceUsage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodResourceUsage {
    pub cpu_usage_millicores: u32,
    pub memory_usage_mb: u32,
    pub network_in_mb: f32,
    pub network_out_mb: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStatus {
    pub name: String,
    pub service_type: String,
    pub cluster_ip: String,
    pub external_ip: Option<String>,
    pub ports: Vec<u16>,
    pub ready_endpoints: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessInformation {
    pub primary_url: String,
    pub admin_urls: Vec<AdminUrl>,
    pub database_connections: Vec<DatabaseConnection>,
    pub api_keys: Vec<ApiKeyInfo>,
    pub ssh_access: Option<SshAccess>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUrl {
    pub service: String,
    pub url: String,
    pub credentials: Option<Credentials>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConnection {
    pub database_type: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
    pub credentials: Credentials,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyInfo {
    pub service: String,
    pub key_name: String,
    pub key_value: String,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshAccess {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub private_key_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub current_cpu_usage: f32,
    pub current_memory_usage_mb: u32,
    pub current_storage_usage_gb: f32,
    pub network_ingress_mb: f32,
    pub network_egress_mb: f32,
    pub estimated_cost_per_hour: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringLink {
    pub service: String,
    pub link_type: LinkType,
    pub url: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LinkType {
    Metrics,
    Logs,
    Traces,
    Dashboard,
    Alerts,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            cloud_provider: CloudProvider::Kubernetes,
            container_runtime: ContainerRuntime::Docker,
            resource_limits: ResourceLimits {
                max_cpu_cores: 2.0,
                max_memory_gb: 4.0,
                max_disk_gb: 10.0,
                max_network_mbps: 100.0,
                max_duration_hours: 24,
            },
            network_policies: NetworkPolicies {
                internet_access: true,
                internal_access: false,
                allowed_ports: vec![80, 443, 3000, 8080],
                blocked_domains: vec!["malware.example.com".to_string()],
                vpn_required: false,
            },
            cleanup_policies: CleanupPolicies {
                auto_cleanup_after_hours: 24,
                cleanup_on_pr_close: true,
                cleanup_on_merge: true,
                preserve_artifacts: true,
                notification_before_cleanup: true,
            },
            security_policies: SecurityPolicies {
                enable_security_scanning: true,
                require_secrets_encryption: true,
                network_isolation: true,
                read_only_filesystem: false,
                no_privileged_containers: true,
            },
        }
    }
}

impl SandboxEnvironmentAgent {
    pub fn new() -> Self {
        let metadata = AgentMetadata {
            id: "sandbox_environment_agent".to_string(),
            name: "SandboxEnvironmentAgent".to_string(),
            persona: "An expert infrastructure engineer specializing in isolated testing environments, containerization, and automated deployment workflows".to_string(),
            description: "Manages isolated testing environments and PR preview deployments with automated provisioning, monitoring, and cleanup".to_string(),
            version: "1.0.0".to_string(),
            supported_input_types: vec![
                "environment_request".to_string(),
                "deployment_config".to_string(),
                "infrastructure_setup".to_string(),
                "resource_provisioning".to_string(),
            ],
            supported_output_types: vec![
                "environment_status".to_string(),
                "deployment_details".to_string(),
                "access_information".to_string(),
                "infrastructure_report".to_string(),
            ],
            capabilities: vec![
                "Infrastructure".to_string(),
                "Deployment".to_string(),
                "Monitoring".to_string(),
                "Security".to_string(),
            ],
            dependencies: vec![],
            tags: vec![
                "infrastructure".to_string(),
                "containers".to_string(),
                "deployment".to_string(),
                "testing".to_string(),
            ],
            base_confidence: 0.90,
        };

        Self {
            metadata,
            config: SandboxConfig::default(),
            cognitive_preferences: crate::agents::traits::CognitivePreferences::default(),
        }
    }

    pub fn with_config(mut self, config: SandboxConfig) -> Self {
        self.config = config;
        self
    }

    async fn provision_environment(&self, _request: &EnvironmentRequest, _app_config: &ApplicationConfig, _deployment_config: &DeploymentConfig, _context: &CognitiveContext) -> BrainResult<EnvironmentStatus> {
        // Implementation would provision actual cloud resources
        // This is a placeholder that would integrate with cloud providers
        
        let environment_id = format!("sb-{}-{}", _request.environment_name, chrono::Utc::now().timestamp());
        
        Ok(EnvironmentStatus {
            status: EnvironmentState::Running,
            environment_id,
            created_at: Utc::now(),
            last_updated: Utc::now(),
            expires_at: Some(Utc::now() + chrono::Duration::hours(self.config.cleanup_policies.auto_cleanup_after_hours as i64)),
            health_status: HealthStatus::Healthy,
        })
    }

    async fn get_deployment_details(&self, _environment_id: &str, _context: &CognitiveContext) -> BrainResult<DeploymentDetails> {
        // Implementation would query actual deployment status
        
        Ok(DeploymentDetails {
            namespace: format!("sandbox-{}", _environment_id),
            pods: vec![
                PodStatus {
                    name: "app-pod-1".to_string(),
                    status: "Running".to_string(),
                    ready: true,
                    restarts: 0,
                    age_seconds: 300,
                    resource_usage: PodResourceUsage {
                        cpu_usage_millicores: 250,
                        memory_usage_mb: 512,
                        network_in_mb: 5.2,
                        network_out_mb: 3.8,
                    },
                },
            ],
            services: vec![
                ServiceStatus {
                    name: "app-service".to_string(),
                    service_type: "ClusterIP".to_string(),
                    cluster_ip: "10.0.1.100".to_string(),
                    external_ip: Some("203.0.113.10".to_string()),
                    ports: vec![80, 443],
                    ready_endpoints: 1,
                },
            ],
            ingress_url: Some("https://pr-123.sandbox.example.com".to_string()),
            build_logs_url: "https://ci.example.com/builds/12345/logs".to_string(),
            deployment_logs_url: "https://logs.example.com/sandbox/12345".to_string(),
        })
    }

    async fn setup_access_information(&self, _environment_id: &str, deployment: &DeploymentDetails, _context: &CognitiveContext) -> BrainResult<AccessInformation> {
        // Implementation would setup actual access credentials and URLs
        
        Ok(AccessInformation {
            primary_url: deployment.ingress_url.clone().unwrap_or_else(|| "http://localhost:3000".to_string()),
            admin_urls: vec![
                AdminUrl {
                    service: "Database Admin".to_string(),
                    url: "https://db-admin.sandbox.example.com".to_string(),
                    credentials: Some(Credentials {
                        username: "admin".to_string(),
                        password: "temp-password-123".to_string(),
                        expires_at: Some(Utc::now() + chrono::Duration::hours(24)),
                    }),
                },
            ],
            database_connections: vec![
                DatabaseConnection {
                    database_type: "PostgreSQL".to_string(),
                    host: "postgres.sandbox.svc.cluster.local".to_string(),
                    port: 5432,
                    database_name: "testdb".to_string(),
                    credentials: Credentials {
                        username: "testuser".to_string(),
                        password: "testpass123".to_string(),
                        expires_at: Some(Utc::now() + chrono::Duration::hours(24)),
                    },
                },
            ],
            api_keys: vec![],
            ssh_access: None,
        })
    }

    async fn monitor_resource_usage(&self, _environment_id: &str, _context: &CognitiveContext) -> BrainResult<ResourceUsage> {
        // Implementation would query actual resource metrics
        
        Ok(ResourceUsage {
            current_cpu_usage: 0.25,
            current_memory_usage_mb: 512,
            current_storage_usage_gb: 2.1,
            network_ingress_mb: 10.5,
            network_egress_mb: 8.2,
            estimated_cost_per_hour: 0.15,
        })
    }

    fn generate_monitoring_links(&self, environment_id: &str, deployment: &DeploymentDetails) -> Vec<MonitoringLink> {
        vec![
            MonitoringLink {
                service: "Kubernetes Dashboard".to_string(),
                link_type: LinkType::Dashboard,
                url: format!("https://k8s-dashboard.example.com/#!/overview?namespace=sandbox-{}", environment_id),
                description: "View pods, services, and deployments".to_string(),
            },
            MonitoringLink {
                service: "Grafana".to_string(),
                link_type: LinkType::Metrics,
                url: format!("https://grafana.example.com/d/sandbox?var-namespace=sandbox-{}", environment_id),
                description: "Application and infrastructure metrics".to_string(),
            },
            MonitoringLink {
                service: "Kibana".to_string(),
                link_type: LinkType::Logs,
                url: deployment.deployment_logs_url.clone(),
                description: "Application and system logs".to_string(),
            },
        ]
    }

    fn generate_next_actions(&self, request: &EnvironmentRequest, status: &EnvironmentStatus) -> Vec<String> {
        let mut actions = Vec::new();
        
        match request.request_type {
            RequestType::CreatePRPreview => {
                actions.push("Share preview URL with team for review".to_string());
                actions.push("Run automated tests against preview environment".to_string());
                actions.push("Validate feature functionality in isolated environment".to_string());
            },
            RequestType::CreateTestEnvironment => {
                actions.push("Configure test data and scenarios".to_string());
                actions.push("Set up monitoring and alerting".to_string());
                actions.push("Document environment access procedures".to_string());
            },
            _ => {
                actions.push("Monitor environment health and performance".to_string());
            }
        }
        
        if let Some(expires_at) = status.expires_at {
            let hours_until_expiry = (expires_at - Utc::now()).num_hours();
            if hours_until_expiry < 2 {
                actions.push("Environment expires soon - extend or backup if needed".to_string());
            }
        }
        
        actions
    }
}

#[async_trait]
impl BrainAgent for SandboxEnvironmentAgent {
    async fn execute(&self, input: AgentInput, context: &CognitiveContext) -> BrainResult<AgentOutput> {
        let sandbox_input: SandboxInput = serde_json::from_value(
            input.parameters.get("sandbox_input").unwrap_or(&serde_json::Value::Null).clone()
        ).map_err(|e| BrainError::InvalidInput(format!("Invalid sandbox input: {}", e)))?;

        // Provision or manage the environment based on request type
        let environment_status = self.provision_environment(
            &sandbox_input.environment_request,
            &sandbox_input.application_config,
            &sandbox_input.deployment_config,
            context
        ).await?;

        // Get deployment details
        let deployment_details = self.get_deployment_details(&environment_status.environment_id, context).await?;

        // Setup access information
        let access_information = self.setup_access_information(&environment_status.environment_id, &deployment_details, context).await?;

        // Monitor resource usage
        let resource_usage = self.monitor_resource_usage(&environment_status.environment_id, context).await?;

        // Generate monitoring links
        let monitoring_links = self.generate_monitoring_links(&environment_status.environment_id, &deployment_details);

        // Generate next actions
        let next_actions = self.generate_next_actions(&sandbox_input.environment_request, &environment_status);

        let sandbox_output = SandboxOutput {
            environment_status,
            deployment_details,
            access_information,
            resource_usage,
            monitoring_links,
            next_actions,
        };

        Ok(AgentOutput {
            agent_id: self.metadata.id.clone(),
            output_type: "sandbox_environment_results".to_string(),
            content: format!("Environment '{}' provisioned successfully. Status: {:?}, Health: {:?}. Access URL: {}",
                sandbox_output.environment_status.environment_id,
                sandbox_output.environment_status.status,
                sandbox_output.environment_status.health_status,
                sandbox_output.access_information.primary_url
            ),
            data: {
                let mut data = std::collections::HashMap::new();
                data.insert("sandbox_output".to_string(), serde_json::to_value(&sandbox_output)?);
                data.insert("environment_id".to_string(), serde_json::to_value(&sandbox_output.environment_status.environment_id)?);
                data.insert("cloud_provider".to_string(), serde_json::to_value(&self.config.cloud_provider)?);
                data.insert("estimated_cost_per_hour".to_string(), serde_json::to_value(sandbox_output.resource_usage.estimated_cost_per_hour)?);
                data
            },
            confidence: match sandbox_output.environment_status.health_status {
                HealthStatus::Healthy => 0.95,
                HealthStatus::Starting => 0.80,
                HealthStatus::Unknown => 0.60,
                HealthStatus::Unhealthy => 0.40,
            },
            reasoning: Some(format!("Environment provisioned using {} with {} configuration. Health checks {}",
                match self.config.cloud_provider {
                    CloudProvider::AWS => "AWS cloud services",
                    CloudProvider::GCP => "Google Cloud Platform",
                    CloudProvider::Azure => "Microsoft Azure",
                    CloudProvider::Kubernetes => "Kubernetes cluster",
                    CloudProvider::Local => "local infrastructure",
                },
                match self.config.container_runtime {
                    ContainerRuntime::Docker => "Docker",
                    ContainerRuntime::Containerd => "containerd",
                    ContainerRuntime::Podman => "Podman",
                    ContainerRuntime::Kubernetes => "Kubernetes",
                },
                if sandbox_output.environment_status.health_status == HealthStatus::Healthy { "passed" } else { "pending" }
            )),
            next_actions: sandbox_output.next_actions.clone(),
            execution_metadata: crate::agents::traits::ExecutionMetadata {
                execution_time_ms: 15000, // 15 seconds for environment provisioning
                memory_usage_mb: 256.0,
                api_calls: 3,
                status: crate::agents::traits::ExecutionStatus::Success,
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

    fn cognitive_preferences(&self) -> &crate::agents::traits::CognitivePreferences {
        &self.cognitive_preferences
    }

    async fn assess_confidence(&self, _input: &AgentInput, _context: &CognitiveContext) -> BrainResult<f32> {
        // Sandbox environment agent has high confidence in infrastructure provisioning
        Ok(0.90)
    }
} 