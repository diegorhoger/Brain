//! DAG Execution Engine

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{RwLock, Semaphore};
use tokio::time::timeout;
use futures::future::try_join_all;
use serde::{Deserialize, Serialize};
use brain_types::error::BrainError;
use crate::agents::traits::{BrainAgent, CognitiveContext, AgentInput, AgentOutput, BrainResult};
use super::dag::{ExecutionPlan, ExecutionWave, AgentDAG};

/// Main executor for DAG-based agent workflows
#[derive(Debug)]
pub struct DAGExecutor {
    metrics: Arc<RwLock<ExecutionMetrics>>,
    semaphore: Arc<Semaphore>,
    max_execution_time: Duration,
    retry_policy: RetryPolicy,
    confidence_threshold: f32,
    error_handler: Arc<ExecutionErrorHandler>,
}

/// Enhanced error handling for agent execution
#[derive(Debug)]
pub struct ExecutionErrorHandler {
    error_classification: ErrorClassifier,
    recovery_strategies: RecoveryStrategyManager,
}

impl ExecutionErrorHandler {
    pub fn new() -> Self {
        Self {
            error_classification: ErrorClassifier::new(),
            recovery_strategies: RecoveryStrategyManager::new(),
        }
    }
    
    pub async fn handle_error(
        &self,
        error: &BrainError,
        context: &ExecutionContext,
        agent_metadata: &crate::agents::traits::AgentMetadata,
    ) -> ErrorHandlingDecision {
        let error_type = self.error_classification.classify_error(error);
        self.recovery_strategies.get_recovery_strategy(error_type, context, agent_metadata).await
    }
}

/// Error classification system
#[derive(Debug)]
pub struct ErrorClassifier {
    classification_rules: Vec<ErrorClassificationRule>,
}

impl ErrorClassifier {
    pub fn new() -> Self {
        let mut rules = Vec::new();
        
        // Add standard classification rules
        rules.push(ErrorClassificationRule {
            pattern: "timeout".to_string(),
            error_type: ExecutionErrorType::Timeout,
            severity: ErrorSeverity::Recoverable,
        });
        
        rules.push(ErrorClassificationRule {
            pattern: "confidence".to_string(),
            error_type: ExecutionErrorType::LowConfidence,
            severity: ErrorSeverity::Warning,
        });
        
        rules.push(ErrorClassificationRule {
            pattern: "validation".to_string(),
            error_type: ExecutionErrorType::InputValidation,
            severity: ErrorSeverity::Critical,
        });
        
        Self {
            classification_rules: rules,
        }
    }
    
    pub fn classify_error(&self, error: &BrainError) -> ExecutionErrorType {
        let error_message = format!("{:?}", error).to_lowercase();
        
        for rule in &self.classification_rules {
            if error_message.contains(&rule.pattern) {
                return rule.error_type.clone();
            }
        }
        
        ExecutionErrorType::Unknown
    }
}

/// Recovery strategy management
#[derive(Debug)]
pub struct RecoveryStrategyManager {
    strategies: std::collections::HashMap<ExecutionErrorType, RecoveryStrategy>,
}

impl RecoveryStrategyManager {
    pub fn new() -> Self {
        let mut strategies = std::collections::HashMap::new();
        
        strategies.insert(ExecutionErrorType::Timeout, RecoveryStrategy::Retry { max_attempts: 2 });
        strategies.insert(ExecutionErrorType::LowConfidence, RecoveryStrategy::SkipAndContinue);
        strategies.insert(ExecutionErrorType::InputValidation, RecoveryStrategy::Fail);
        strategies.insert(ExecutionErrorType::Unknown, RecoveryStrategy::Retry { max_attempts: 1 });
        
        Self { strategies }
    }
    
    pub async fn get_recovery_strategy(
        &self,
        error_type: ExecutionErrorType,
        _context: &ExecutionContext,
        _agent_metadata: &crate::agents::traits::AgentMetadata,
    ) -> ErrorHandlingDecision {
        match self.strategies.get(&error_type) {
            Some(RecoveryStrategy::Retry { max_attempts }) => {
                ErrorHandlingDecision::Retry { 
                    max_attempts: *max_attempts,
                    delay: Duration::from_millis(1000),
                }
            }
            Some(RecoveryStrategy::SkipAndContinue) => ErrorHandlingDecision::Skip,
            Some(RecoveryStrategy::Fail) => ErrorHandlingDecision::Fail,
            None => ErrorHandlingDecision::Fail,
        }
    }
}

/// Types of execution errors
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExecutionErrorType {
    Timeout,
    LowConfidence,
    InputValidation,
    ResourceExhausted,
    DependencyFailure,
    AgentUnavailable,
    NetworkError,
    ConfigurationError,
    Unknown,
}

/// Error severity levels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ErrorSeverity {
    Warning,     // Can continue execution
    Recoverable, // Should retry
    Critical,    // Must stop execution
}

/// Error classification rules
#[derive(Debug, Clone)]
pub struct ErrorClassificationRule {
    pub pattern: String,
    pub error_type: ExecutionErrorType,
    pub severity: ErrorSeverity,
}

/// Recovery strategies for different error types
#[derive(Debug, Clone, PartialEq)]
pub enum RecoveryStrategy {
    Retry { max_attempts: u32 },
    SkipAndContinue,
    Fail,
}

/// Error handling decisions
#[derive(Debug, Clone)]
pub enum ErrorHandlingDecision {
    Retry { max_attempts: u32, delay: Duration },
    Skip,
    Fail,
}

/// Enhanced confidence checking
#[derive(Debug)]
pub struct ConfidenceChecker {
    global_threshold: f32,
    agent_specific_thresholds: std::collections::HashMap<String, f32>,
}

impl ConfidenceChecker {
    pub fn new(global_threshold: f32) -> Self {
        Self {
            global_threshold,
            agent_specific_thresholds: std::collections::HashMap::new(),
        }
    }
    
    pub fn set_agent_threshold(&mut self, agent_id: String, threshold: f32) {
        self.agent_specific_thresholds.insert(agent_id, threshold);
    }
    
    pub async fn check_confidence(
        &self,
        agent: &Arc<dyn BrainAgent>,
        input: &AgentInput,
        context: &CognitiveContext,
    ) -> BrainResult<ConfidenceCheckResult> {
        let agent_threshold = self.agent_specific_thresholds
            .get(&agent.metadata().id)
            .copied()
            .unwrap_or(self.global_threshold);
        
        let confidence = agent.assess_confidence(input, context).await?;
        
        Ok(ConfidenceCheckResult {
            confidence,
            threshold: agent_threshold,
            passes: confidence >= agent_threshold,
            recommendation: if confidence >= agent_threshold {
                ConfidenceRecommendation::Proceed
            } else if confidence >= agent_threshold * 0.8 {
                ConfidenceRecommendation::ProceedWithCaution
            } else {
                ConfidenceRecommendation::Skip
            },
        })
    }
}

/// Result of confidence checking
#[derive(Debug, Clone)]
pub struct ConfidenceCheckResult {
    pub confidence: f32,
    pub threshold: f32,
    pub passes: bool,
    pub recommendation: ConfidenceRecommendation,
}

/// Confidence-based recommendations
#[derive(Debug, Clone, PartialEq)]
pub enum ConfidenceRecommendation {
    Proceed,
    ProceedWithCaution,
    Skip,
}

/// Overall execution metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionMetrics {
    pub total_executions: u64,
    pub successful_executions: u64,
    pub failed_executions: u64,
    pub skipped_executions: u64,
    pub total_execution_time_ms: u64,
    pub wave_timings: Vec<WaveTiming>,
    pub error_counts: std::collections::HashMap<ExecutionErrorType, u64>,
    pub confidence_stats: ConfidenceStatistics,
}

/// Confidence statistics tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceStatistics {
    pub average_confidence: f32,
    pub confidence_distribution: std::collections::HashMap<String, u64>, // Confidence ranges
    pub threshold_violations: u64,
    pub confidence_improvements: u64,
}

impl Default for ConfidenceStatistics {
    fn default() -> Self {
        Self {
            average_confidence: 0.0,
            confidence_distribution: std::collections::HashMap::new(),
            threshold_violations: 0,
            confidence_improvements: 0,
        }
    }
}

/// Timing information for execution waves
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaveTiming {
    pub wave_number: usize,
    pub start_time_ms: u64,
    pub duration_ms: u64,
    pub agent_count: usize,
    pub successful_agents: usize,
    pub failed_agents: usize,
    pub skipped_agents: usize,
    pub average_confidence: f32,
}

impl Default for ExecutionMetrics {
    fn default() -> Self {
        Self {
            total_executions: 0,
            successful_executions: 0,
            failed_executions: 0,
            skipped_executions: 0,
            total_execution_time_ms: 0,
            wave_timings: Vec::new(),
            error_counts: std::collections::HashMap::new(),
            confidence_stats: ConfidenceStatistics::default(),
        }
    }
}

impl DAGExecutor {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(ExecutionMetrics::default())),
            semaphore: Arc::new(Semaphore::new(10)), // Max 10 concurrent agents
            max_execution_time: Duration::from_secs(300), // 5 minute timeout
            retry_policy: RetryPolicy::default(),
            confidence_threshold: 0.7, // Default 70% confidence threshold
            error_handler: Arc::new(ExecutionErrorHandler::new()),
        }
    }
    
    pub fn with_concurrency(mut self, max_concurrent: usize) -> Self {
        self.semaphore = Arc::new(Semaphore::new(max_concurrent));
        self
    }
    
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.max_execution_time = timeout;
        self
    }
    
    pub fn with_retry_policy(mut self, policy: RetryPolicy) -> Self {
        self.retry_policy = policy;
        self
    }
    
    pub fn with_confidence_threshold(mut self, threshold: f32) -> Self {
        self.confidence_threshold = threshold;
        self
    }
    
    /// Execute a complete DAG execution plan with enhanced error handling
    /// 
    /// This method implements:
    /// - Confidence threshold enforcement before execution
    /// - Comprehensive error classification and handling
    /// - Sophisticated retry strategies based on error types
    /// - Detailed metrics tracking for monitoring and optimization
    pub async fn execute_plan(
        &self,
        plan: ExecutionPlan,
        dag: &mut AgentDAG,
        context: &CognitiveContext,
    ) -> BrainResult<Vec<AgentOutput>> {
        let execution_start = Instant::now();
        let mut all_outputs = Vec::new();
        let confidence_checker = ConfidenceChecker::new(self.confidence_threshold);
        
        let mut metrics_guard = self.metrics.write().await;
        metrics_guard.total_executions += 1;
        metrics_guard.wave_timings.clear();
        drop(metrics_guard);
        
        // Execute waves sequentially with enhanced error handling
        for (wave_idx, wave) in plan.execution_waves.iter().enumerate() {
            let wave_start = Instant::now();
            
            match self.execute_wave_with_confidence_checks(wave, dag, context, &confidence_checker).await {
                Ok(wave_result) => {
                    all_outputs.extend(wave_result.outputs);
                    
                    // Record successful wave timing with confidence stats
                    let wave_duration = wave_start.elapsed();
                    let mut metrics_guard = self.metrics.write().await;
                    metrics_guard.wave_timings.push(WaveTiming {
                        wave_number: wave_idx,
                        start_time_ms: wave_start.elapsed().as_millis() as u64,
                        duration_ms: wave_duration.as_millis() as u64,
                        agent_count: wave.node_ids.len(),
                        successful_agents: wave_result.successful_count,
                        failed_agents: wave_result.failed_count,
                        skipped_agents: wave_result.skipped_count,
                        average_confidence: wave_result.average_confidence,
                    });
                    
                    // Update confidence statistics
                    metrics_guard.confidence_stats.average_confidence = 
                        (metrics_guard.confidence_stats.average_confidence + wave_result.average_confidence) / 2.0;
                    
                    drop(metrics_guard);
                }
                Err(e) => {
                    // Enhanced error handling with classification
                    let _execution_context = ExecutionContext::new(format!("wave_{}", wave_idx));
                    let error_type = self.error_handler.error_classification.classify_error(&e);
                    
                    let wave_duration = wave_start.elapsed();
                    let mut metrics_guard = self.metrics.write().await;
                    metrics_guard.failed_executions += 1;
                    
                    // Update error counts
                    *metrics_guard.error_counts.entry(error_type.clone()).or_insert(0) += 1;
                    
                    metrics_guard.wave_timings.push(WaveTiming {
                        wave_number: wave_idx,
                        start_time_ms: wave_start.elapsed().as_millis() as u64,
                        duration_ms: wave_duration.as_millis() as u64,
                        agent_count: wave.node_ids.len(),
                        successful_agents: 0,
                        failed_agents: wave.node_ids.len(),
                        skipped_agents: 0,
                        average_confidence: 0.0,
                    });
                    drop(metrics_guard);
                    
                    return Err(BrainError::ExecutionError(
                        format!("Wave {} failed with error type {:?}: {}", wave_idx, error_type, e)
                    ));
                }
            }
        }
        
        // Update final metrics
        let total_execution_time = execution_start.elapsed();
        let mut metrics_guard = self.metrics.write().await;
        metrics_guard.successful_executions += 1;
        metrics_guard.total_execution_time_ms += total_execution_time.as_millis() as u64;
        drop(metrics_guard);
        
        Ok(all_outputs)
    }
    
    /// Execute a wave with confidence checks and enhanced error handling
    async fn execute_wave_with_confidence_checks(
        &self,
        wave: &ExecutionWave,
        dag: &AgentDAG,
        context: &CognitiveContext,
        confidence_checker: &ConfidenceChecker,
    ) -> BrainResult<WaveExecutionResult> {
        if wave.node_ids.is_empty() {
            return Ok(WaveExecutionResult {
                outputs: Vec::new(),
                successful_count: 0,
                failed_count: 0,
                skipped_count: 0,
                average_confidence: 0.0,
            });
        }
        
        let mut outputs = Vec::new();
        let mut successful_count = 0;
        let mut failed_count = 0;
        let mut skipped_count = 0;
        let mut total_confidence = 0.0;
        let mut confidence_count = 0;
        
        // Create futures for all agents in this wave with confidence checks
        let agent_futures: Vec<_> = wave.node_ids.iter().map(|node_id| {
            self.execute_agent_with_confidence_and_error_handling(
                node_id.clone(), 
                dag, 
                context, 
                confidence_checker
            )
        }).collect();
        
        // Execute all agents in parallel and collect results
        let results = futures::future::join_all(agent_futures).await;
        
        for result in results {
            match result {
                Ok(agent_result) => {
                    match agent_result {
                        AgentExecutionResult::Success { output, confidence } => {
                            outputs.push(output);
                            successful_count += 1;
                            total_confidence += confidence;
                            confidence_count += 1;
                        }
                        AgentExecutionResult::Skipped { confidence } => {
                            skipped_count += 1;
                            total_confidence += confidence;
                            confidence_count += 1;
                        }
                    }
                }
                Err(_) => {
                    failed_count += 1;
                }
            }
        }
        
        let average_confidence = if confidence_count > 0 {
            total_confidence / confidence_count as f32
        } else {
            0.0
        };
        
        Ok(WaveExecutionResult {
            outputs,
            successful_count,
            failed_count,
            skipped_count,
            average_confidence,
        })
    }
    
    /// Execute agent with confidence checking and comprehensive error handling
    async fn execute_agent_with_confidence_and_error_handling(
        &self,
        node_id: String,
        dag: &AgentDAG,
        context: &CognitiveContext,
        confidence_checker: &ConfidenceChecker,
    ) -> BrainResult<AgentExecutionResult> {
        let node = dag.nodes.get(&node_id)
            .ok_or_else(|| BrainError::ExecutionError(
                format!("Node {} not found in DAG", node_id)
            ))?;
        
        // Check confidence before execution
        let confidence_result = confidence_checker
            .check_confidence(&node.agent, &node.input, context)
            .await?;
        
        if !confidence_result.passes {
            // Record confidence threshold violation
            let mut metrics_guard = self.metrics.write().await;
            metrics_guard.confidence_stats.threshold_violations += 1;
            metrics_guard.skipped_executions += 1;
            drop(metrics_guard);
            
            return Ok(AgentExecutionResult::Skipped { 
                confidence: confidence_result.confidence 
            });
        }
        
        // Proceed with execution with retry logic
        let execution_context = ExecutionContext::new(node_id.clone());
        let mut last_error = None;
        
        for attempt in 1..=self.retry_policy.max_attempts {
            // Acquire semaphore permit for concurrency control
            let _permit = self.semaphore.acquire().await
                .map_err(|_| BrainError::ExecutionError(
                    "Failed to acquire execution permit".to_string()
                ))?;
            
            match self.execute_single_agent_with_timeout(node, context).await {
                Ok(output) => {
                    return Ok(AgentExecutionResult::Success { 
                        output, 
                        confidence: confidence_result.confidence 
                    });
                }
                Err(e) => {
                    // Enhanced error handling
                    let error_decision = self.error_handler
                        .handle_error(&e, &execution_context, node.agent.metadata())
                        .await;
                    
                    match error_decision {
                        ErrorHandlingDecision::Retry { max_attempts, delay } => {
                            if attempt < max_attempts.min(self.retry_policy.max_attempts) {
                                tokio::time::sleep(delay).await;
                                last_error = Some(e);
                                continue;
                            }
                        }
                        ErrorHandlingDecision::Skip => {
                            let mut metrics_guard = self.metrics.write().await;
                            metrics_guard.skipped_executions += 1;
                            drop(metrics_guard);
                            
                            return Ok(AgentExecutionResult::Skipped { 
                                confidence: confidence_result.confidence 
                            });
                        }
                        ErrorHandlingDecision::Fail => {
                            return Err(e);
                        }
                    }
                    
                    last_error = Some(e);
                }
            }
        }
        
        // All retry attempts failed
        Err(last_error.unwrap_or_else(|| BrainError::ExecutionError(
            format!("Agent {} failed after {} attempts", node_id, self.retry_policy.max_attempts)
        )))
    }
    
    /// Execute a single agent with timeout (internal helper)
    async fn execute_single_agent_with_timeout(
        &self,
        node: &super::dag::AgentNode,
        context: &CognitiveContext,
    ) -> BrainResult<AgentOutput> {
        // Apply timeout to agent execution
        let execution_future = node.agent.execute(node.input.clone(), context);
        
        match timeout(self.max_execution_time, execution_future).await {
            Ok(result) => result,
            Err(_) => Err(BrainError::ExecutionError(
                format!("Agent {} execution timed out after {:?}", 
                       node.id, self.max_execution_time)
            )),
        }
    }
    
    pub async fn total_executions(&self) -> u64 {
        let metrics = self.metrics.read().await;
        metrics.total_executions
    }
    
    pub async fn successful_executions(&self) -> u64 {
        let metrics = self.metrics.read().await;
        metrics.successful_executions
    }
    
    pub async fn failed_executions(&self) -> u64 {
        let metrics = self.metrics.read().await;
        metrics.failed_executions
    }
    
    pub async fn skipped_executions(&self) -> u64 {
        let metrics = self.metrics.read().await;
        metrics.skipped_executions
    }
    
    pub async fn average_execution_time(&self) -> f64 {
        let metrics = self.metrics.read().await;
        if metrics.total_executions == 0 {
            0.0
        } else {
            metrics.total_execution_time_ms as f64 / metrics.total_executions as f64
        }
    }
    
    pub async fn active_agents(&self) -> usize {
        self.semaphore.available_permits()
    }
    
    pub async fn get_wave_timings(&self) -> Vec<WaveTiming> {
        let metrics = self.metrics.read().await;
        metrics.wave_timings.clone()
    }
    
    pub async fn get_error_statistics(&self) -> std::collections::HashMap<ExecutionErrorType, u64> {
        let metrics = self.metrics.read().await;
        metrics.error_counts.clone()
    }
    
    pub async fn get_confidence_statistics(&self) -> ConfidenceStatistics {
        let metrics = self.metrics.read().await;
        metrics.confidence_stats.clone()
    }
    
    /// Get current execution metrics (added for orchestrator integration)
    pub async fn get_metrics(&self) -> BrainResult<ExecutionMetrics> {
        let metrics = self.metrics.read().await;
        Ok(metrics.clone())
    }
}

/// Result of wave execution
#[derive(Debug)]
struct WaveExecutionResult {
    outputs: Vec<AgentOutput>,
    successful_count: usize,
    failed_count: usize,
    skipped_count: usize,
    average_confidence: f32,
}

/// Result of individual agent execution
#[derive(Debug)]
enum AgentExecutionResult {
    Success { output: AgentOutput, confidence: f32 },
    Skipped { confidence: f32 },
}

/// Execution engine trait for pluggable execution strategies
pub trait ExecutionEngine: Send + Sync {
    fn execute_agent(
        &self,
        agent: Arc<dyn BrainAgent>,
        input: AgentInput,
        context: &CognitiveContext,
    ) -> impl std::future::Future<Output = BrainResult<AgentOutput>> + Send;
    
    fn execute_batch(
        &self,
        agents: Vec<(Arc<dyn BrainAgent>, AgentInput)>,
        context: &CognitiveContext,
    ) -> impl std::future::Future<Output = BrainResult<Vec<AgentOutput>>> + Send;
}

/// Standard execution engine implementation
pub struct StandardExecutionEngine {
    concurrency_limit: usize,
    timeout: Duration,
}

impl StandardExecutionEngine {
    pub fn new(concurrency_limit: usize, timeout: Duration) -> Self {
        Self {
            concurrency_limit,
            timeout,
        }
    }
}

impl ExecutionEngine for StandardExecutionEngine {
    fn execute_agent(
        &self,
        agent: Arc<dyn BrainAgent>,
        input: AgentInput,
        context: &CognitiveContext,
    ) -> impl std::future::Future<Output = BrainResult<AgentOutput>> + Send {
        let timeout_duration = self.timeout;
        let context = context.clone();
        async move {
            match timeout(timeout_duration, agent.execute(input, &context)).await {
                Ok(result) => result,
                Err(_) => Err(BrainError::ExecutionError(
                    format!("Agent execution timed out after {:?}", timeout_duration)
                )),
            }
        }
    }
    
    fn execute_batch(
        &self,
        agents: Vec<(Arc<dyn BrainAgent>, AgentInput)>,
        context: &CognitiveContext,
    ) -> impl std::future::Future<Output = BrainResult<Vec<AgentOutput>>> + Send {
        let concurrency_limit = self.concurrency_limit;
        let timeout_duration = self.timeout;
        let context = context.clone();
        async move {
            let semaphore = Arc::new(Semaphore::new(concurrency_limit));
            let futures: Vec<_> = agents.into_iter().map(|(agent, input)| {
                let semaphore = Arc::clone(&semaphore);
                let context = context.clone();
                async move {
                    let _permit = semaphore.acquire().await
                        .map_err(|_| BrainError::ExecutionError(
                            "Failed to acquire execution permit".to_string()
                        ))?;
                    match timeout(timeout_duration, agent.execute(input, &context)).await {
                        Ok(result) => result,
                        Err(_) => Err(BrainError::ExecutionError(
                            format!("Agent execution timed out after {:?}", timeout_duration)
                        )),
                    }
                }
            }).collect();
            
            try_join_all(futures).await
        }
    }
}

/// Execution context for tracking execution state
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub execution_id: String,
    pub start_time: Instant,
    pub max_duration: Duration,
    pub retry_policy: RetryPolicy,
}

impl ExecutionContext {
    pub fn new(execution_id: String) -> Self {
        Self {
            execution_id,
            start_time: Instant::now(),
            max_duration: Duration::from_secs(300),
            retry_policy: RetryPolicy::default(),
        }
    }
    
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }
    
    pub fn is_timeout(&self) -> bool {
        self.elapsed() > self.max_duration
    }
}

/// Execution result with detailed information
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub execution_id: String,
    pub outputs: Vec<AgentOutput>,
    pub execution_time: Duration,
    pub wave_count: usize,
    pub agent_count: usize,
    pub failed_agents: Vec<String>,
    pub skipped_agents: Vec<String>,
    pub confidence_stats: ConfidenceStatistics,
}

/// Enhanced retry policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub retry_delay_ms: u64,
    pub exponential_backoff: bool,
    pub max_delay_ms: u64,
    pub retry_on_low_confidence: bool,
    pub confidence_improvement_threshold: f32,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            retry_delay_ms: 1000,
            exponential_backoff: false,
            max_delay_ms: 10000,
            retry_on_low_confidence: false,
            confidence_improvement_threshold: 0.1,
        }
    }
}

impl RetryPolicy {
    pub fn get_delay(&self, attempt: u32) -> Duration {
        if self.exponential_backoff {
            let delay = self.retry_delay_ms * (2_u64.pow(attempt.saturating_sub(1)));
            Duration::from_millis(delay.min(self.max_delay_ms))
        } else {
            Duration::from_millis(self.retry_delay_ms)
        }
    }
}
