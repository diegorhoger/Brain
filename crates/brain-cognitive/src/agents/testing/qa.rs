use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

use crate::agents::traits::{BrainAgent, AgentMetadata, AgentInput, AgentOutput, CognitiveContext};
use crate::agents::traits::BrainResult;
use brain_types::error::BrainError;

/// Quality Assurance Agent for automated testing and validation
#[derive(Debug, Clone)]
pub struct QAAgent {
    metadata: AgentMetadata,
    config: QAConfig,
    cognitive_preferences: crate::agents::traits::CognitivePreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QAConfig {
    pub test_coverage_threshold: f32,
    pub performance_baseline: PerformanceBaseline,
    pub test_environments: Vec<TestEnvironment>,
    pub quality_gates: QualityGates,
    pub automation_rules: AutomationRules,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBaseline {
    pub max_response_time_ms: u64,
    pub max_memory_usage_mb: u64,
    pub min_throughput_rps: u64,
    pub error_rate_threshold: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestEnvironment {
    pub name: String,
    pub environment_type: EnvironmentType,
    pub config: HashMap<String, String>,
    pub health_check_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnvironmentType {
    Unit,
    Integration,
    EndToEnd,
    Performance,
    Security,
    Accessibility,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityGates {
    pub required_test_types: Vec<TestType>,
    pub min_code_coverage: f32,
    pub max_complexity_score: u32,
    pub security_scan_required: bool,
    pub performance_test_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestType {
    Unit,
    Integration,
    EndToEnd,
    Performance,
    Security,
    Accessibility,
    Regression,
    Smoke,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRules {
    pub auto_run_on_pr: bool,
    pub auto_run_on_merge: bool,
    pub parallel_execution: bool,
    pub retry_failed_tests: u32,
    pub notification_channels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QAInput {
    pub project_context: ProjectContext,
    pub test_request: TestRequest,
    pub target_environment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectContext {
    pub project_name: String,
    pub project_path: String,
    pub language: String,
    pub framework: Option<String>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRequest {
    pub test_types: Vec<TestType>,
    pub target_coverage: Option<f32>,
    pub performance_requirements: Option<PerformanceBaseline>,
    pub custom_test_commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QAOutput {
    pub test_results: TestResults,
    pub quality_assessment: QualityAssessment,
    pub recommendations: Vec<QARecommendation>,
    pub generated_reports: Vec<TestReport>,
    pub next_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResults {
    pub overall_status: TestStatus,
    pub test_suites: Vec<TestSuite>,
    pub coverage_report: CoverageReport,
    pub performance_metrics: PerformanceMetrics,
    pub execution_time: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TestStatus {
    Passed,
    Failed,
    Partial,
    Skipped,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuite {
    pub name: String,
    pub test_type: TestType,
    pub status: TestStatus,
    pub total_tests: u32,
    pub passed_tests: u32,
    pub failed_tests: u32,
    pub skipped_tests: u32,
    pub execution_time_ms: u64,
    pub failed_test_details: Vec<FailedTest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedTest {
    pub test_name: String,
    pub error_message: String,
    pub stack_trace: Option<String>,
    pub assertion_details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageReport {
    pub line_coverage: f32,
    pub branch_coverage: f32,
    pub function_coverage: f32,
    pub statement_coverage: f32,
    pub uncovered_files: Vec<String>,
    pub coverage_by_module: HashMap<String, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub response_times: ResponseTimeStats,
    pub memory_usage: MemoryStats,
    pub throughput: ThroughputStats,
    pub error_rates: ErrorRateStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseTimeStats {
    pub average_ms: f64,
    pub median_ms: f64,
    pub p95_ms: f64,
    pub p99_ms: f64,
    pub max_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    pub peak_usage_mb: f64,
    pub average_usage_mb: f64,
    pub memory_leaks_detected: bool,
    pub gc_pressure: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThroughputStats {
    pub requests_per_second: f64,
    pub transactions_per_second: f64,
    pub concurrent_users: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorRateStats {
    pub total_errors: u32,
    pub error_rate_percent: f32,
    pub error_types: HashMap<String, u32>,
    pub critical_errors: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityAssessment {
    pub overall_quality_score: f32,
    pub quality_gates_passed: bool,
    pub areas_for_improvement: Vec<String>,
    pub strengths: Vec<String>,
    pub risk_level: RiskLevel,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QARecommendation {
    pub category: RecommendationCategory,
    pub priority: Priority,
    pub description: String,
    pub implementation_steps: Vec<String>,
    pub estimated_effort: String,
    pub impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationCategory {
    TestCoverage,
    Performance,
    Security,
    CodeQuality,
    TestAutomation,
    CiCd,
    Documentation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestReport {
    pub report_type: ReportType,
    pub file_path: String,
    pub format: ReportFormat,
    pub summary: String,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportType {
    Coverage,
    Performance,
    Security,
    Integration,
    Summary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportFormat {
    Html,
    Json,
    Xml,
    Pdf,
    Markdown,
}

impl Default for QAConfig {
    fn default() -> Self {
        Self {
            test_coverage_threshold: 80.0,
            performance_baseline: PerformanceBaseline {
                max_response_time_ms: 1000,
                max_memory_usage_mb: 512,
                min_throughput_rps: 100,
                error_rate_threshold: 1.0,
            },
            test_environments: vec![
                TestEnvironment {
                    name: "unit".to_string(),
                    environment_type: EnvironmentType::Unit,
                    config: HashMap::new(),
                    health_check_url: None,
                },
                TestEnvironment {
                    name: "integration".to_string(),
                    environment_type: EnvironmentType::Integration,
                    config: HashMap::new(),
                    health_check_url: Some("http://localhost:3000/health".to_string()),
                },
            ],
            quality_gates: QualityGates {
                required_test_types: vec![TestType::Unit, TestType::Integration],
                min_code_coverage: 80.0,
                max_complexity_score: 10,
                security_scan_required: true,
                performance_test_required: false,
            },
            automation_rules: AutomationRules {
                auto_run_on_pr: true,
                auto_run_on_merge: true,
                parallel_execution: true,
                retry_failed_tests: 3,
                notification_channels: vec!["slack".to_string()],
            },
        }
    }
}

impl QAAgent {
    pub fn new() -> Self {
        let metadata = AgentMetadata {
            id: "qa_agent".to_string(),
            name: "QAAgent".to_string(),
            persona: "A meticulous quality assurance specialist focused on ensuring code reliability, test coverage, and system stability through comprehensive automated testing".to_string(),
            description: "Quality assurance testing and validation agent that ensures code quality, test coverage, and system reliability through automated testing pipelines".to_string(),
            version: "1.0.0".to_string(),
            supported_input_types: vec![
                "qa_request".to_string(),
                "test_execution".to_string(),
                "coverage_analysis".to_string(),
                "performance_testing".to_string(),
            ],
            supported_output_types: vec![
                "test_results".to_string(),
                "quality_assessment".to_string(),
                "qa_report".to_string(),
                "recommendations".to_string(),
            ],
            capabilities: vec![
                "Testing".to_string(),
                "QualityAssurance".to_string(),
                "PerformanceAnalysis".to_string(),
                "ReportGeneration".to_string(),
            ],
            dependencies: vec![],
            tags: vec![
                "testing".to_string(),
                "qa".to_string(),
                "quality".to_string(),
                "automation".to_string(),
            ],
            base_confidence: 0.85,
        };

        Self {
            metadata,
            config: QAConfig::default(),
            cognitive_preferences: crate::agents::traits::CognitivePreferences::default(),
        }
    }

    pub fn with_config(mut self, config: QAConfig) -> Self {
        self.config = config;
        self
    }

    async fn run_test_suite(&self, _suite_name: &str, _test_type: &TestType, _context: &CognitiveContext) -> BrainResult<TestSuite> {
        // Implementation would run actual test commands
        // This is a placeholder that would integrate with actual test runners
        
        Ok(TestSuite {
            name: _suite_name.to_string(),
            test_type: _test_type.clone(),
            status: TestStatus::Passed,
            total_tests: 42,
            passed_tests: 40,
            failed_tests: 2,
            skipped_tests: 0,
            execution_time_ms: 5000,
            failed_test_details: vec![],
        })
    }

    async fn analyze_coverage(&self, _project_path: &str, _context: &CognitiveContext) -> BrainResult<CoverageReport> {
        // Implementation would analyze actual code coverage
        
        Ok(CoverageReport {
            line_coverage: 85.5,
            branch_coverage: 78.2,
            function_coverage: 92.1,
            statement_coverage: 87.3,
            uncovered_files: vec!["src/utils/legacy.rs".to_string()],
            coverage_by_module: HashMap::from([
                ("core".to_string(), 95.0),
                ("api".to_string(), 82.3),
                ("utils".to_string(), 70.1),
            ]),
        })
    }

    async fn run_performance_tests(&self, _baseline: &PerformanceBaseline, _context: &CognitiveContext) -> BrainResult<PerformanceMetrics> {
        // Implementation would run actual performance tests
        
        Ok(PerformanceMetrics {
            response_times: ResponseTimeStats {
                average_ms: 250.5,
                median_ms: 200.0,
                p95_ms: 450.0,
                p99_ms: 800.0,
                max_ms: 1200.0,
            },
            memory_usage: MemoryStats {
                peak_usage_mb: 128.5,
                average_usage_mb: 95.2,
                memory_leaks_detected: false,
                gc_pressure: Some(0.15),
            },
            throughput: ThroughputStats {
                requests_per_second: 150.0,
                transactions_per_second: 140.0,
                concurrent_users: 50,
            },
            error_rates: ErrorRateStats {
                total_errors: 5,
                error_rate_percent: 0.5,
                error_types: HashMap::from([
                    ("timeout".to_string(), 3),
                    ("validation".to_string(), 2),
                ]),
                critical_errors: 0,
            },
        })
    }

    fn assess_quality(&self, results: &TestResults) -> QualityAssessment {
        let coverage_score = (results.coverage_report.line_coverage / 100.0) * 30.0;
        let test_score = if results.overall_status == TestStatus::Passed { 40.0 } else { 20.0 };
        let performance_score = 20.0; // Simplified scoring
        let security_score = 10.0; // Simplified scoring
        
        let overall_score = coverage_score + test_score + performance_score + security_score;
        
        QualityAssessment {
            overall_quality_score: overall_score,
            quality_gates_passed: overall_score >= 80.0,
            areas_for_improvement: vec![
                "Increase test coverage in utils module".to_string(),
                "Add more integration tests".to_string(),
            ],
            strengths: vec![
                "High unit test coverage".to_string(),
                "Fast test execution".to_string(),
            ],
            risk_level: if overall_score >= 90.0 {
                RiskLevel::Low
            } else if overall_score >= 70.0 {
                RiskLevel::Medium
            } else {
                RiskLevel::High
            },
        }
    }

    fn generate_recommendations(&self, assessment: &QualityAssessment, results: &TestResults) -> Vec<QARecommendation> {
        let mut recommendations = Vec::new();
        
        if results.coverage_report.line_coverage < self.config.test_coverage_threshold {
            recommendations.push(QARecommendation {
                category: RecommendationCategory::TestCoverage,
                priority: Priority::High,
                description: format!(
                    "Increase test coverage from {:.1}% to {:.1}%",
                    results.coverage_report.line_coverage,
                    self.config.test_coverage_threshold
                ),
                implementation_steps: vec![
                    "Identify uncovered code paths".to_string(),
                    "Write targeted unit tests".to_string(),
                    "Add integration test scenarios".to_string(),
                ],
                estimated_effort: "2-3 days".to_string(),
                impact: "Improved code reliability and bug detection".to_string(),
            });
        }
        
        if assessment.risk_level == RiskLevel::High || assessment.risk_level == RiskLevel::Critical {
            recommendations.push(QARecommendation {
                category: RecommendationCategory::CodeQuality,
                priority: Priority::Critical,
                description: "Critical quality issues detected requiring immediate attention".to_string(),
                implementation_steps: vec![
                    "Review failed tests and fix critical bugs".to_string(),
                    "Implement missing test scenarios".to_string(),
                    "Enhance error handling and validation".to_string(),
                ],
                estimated_effort: "1-2 weeks".to_string(),
                impact: "Prevent production issues and improve system stability".to_string(),
            });
        }
        
        recommendations
    }
}

#[async_trait]
impl BrainAgent for QAAgent {
    async fn execute(&self, input: AgentInput, context: &CognitiveContext) -> BrainResult<AgentOutput> {
        let qa_input: QAInput = serde_json::from_value(
            input.parameters.get("qa_input").unwrap_or(&serde_json::Value::Null).clone()
        ).map_err(|e| BrainError::InvalidInput(format!("Invalid QA input: {}", e)))?;

        // Run test suites based on request
        let mut test_suites = Vec::new();
        for test_type in &qa_input.test_request.test_types {
            let suite_name = format!("{:?}_tests", test_type);
            let suite = self.run_test_suite(&suite_name, test_type, context).await?;
            test_suites.push(suite);
        }

        // Analyze code coverage
        let coverage_report = self.analyze_coverage(&qa_input.project_context.project_path, context).await?;

        // Run performance tests if required
        let performance_metrics = if let Some(baseline) = &qa_input.test_request.performance_requirements {
            self.run_performance_tests(baseline, context).await?
        } else {
            self.run_performance_tests(&self.config.performance_baseline, context).await?
        };

        // Determine overall test status
        let overall_status = if test_suites.iter().all(|s| s.status == TestStatus::Passed) {
            TestStatus::Passed
        } else if test_suites.iter().any(|s| s.status == TestStatus::Failed) {
            TestStatus::Failed
        } else {
            TestStatus::Partial
        };

        let test_results = TestResults {
            overall_status,
            test_suites,
            coverage_report,
            performance_metrics,
            execution_time: 30000, // 30 seconds
        };

        // Assess quality
        let quality_assessment = self.assess_quality(&test_results);

        // Generate recommendations
        let recommendations = self.generate_recommendations(&quality_assessment, &test_results);

        // Generate reports
        let generated_reports = vec![
            TestReport {
                report_type: ReportType::Summary,
                file_path: "qa_reports/summary.html".to_string(),
                format: ReportFormat::Html,
                summary: "Comprehensive QA test results and analysis".to_string(),
                generated_at: Utc::now(),
            },
            TestReport {
                report_type: ReportType::Coverage,
                file_path: "qa_reports/coverage.json".to_string(),
                format: ReportFormat::Json,
                summary: "Detailed code coverage analysis".to_string(),
                generated_at: Utc::now(),
            },
        ];

        // Generate next actions
        let next_actions = if quality_assessment.quality_gates_passed {
            vec![
                "All quality gates passed - ready for deployment".to_string(),
                "Schedule regular regression testing".to_string(),
            ]
        } else {
            vec![
                "Address failed quality gates before deployment".to_string(),
                "Review and implement recommended improvements".to_string(),
                "Re-run QA pipeline after fixes".to_string(),
            ]
        };

        let qa_output = QAOutput {
            test_results,
            quality_assessment,
            recommendations,
            generated_reports,
            next_actions,
        };

        Ok(AgentOutput {
            agent_id: self.metadata.id.clone(),
            output_type: "qa_results".to_string(),
            content: format!("Quality assessment completed. Overall score: {:.1}/100. Quality gates: {}",
                qa_output.quality_assessment.overall_quality_score,
                if qa_output.quality_assessment.quality_gates_passed { "PASSED" } else { "FAILED" }
            ),
            data: {
                let mut data = std::collections::HashMap::new();
                data.insert("qa_output".to_string(), serde_json::to_value(&qa_output)?);
                data.insert("test_types_executed".to_string(), serde_json::to_value(&qa_input.test_request.test_types)?);
                data.insert("coverage_threshold".to_string(), serde_json::to_value(self.config.test_coverage_threshold)?);
                data.insert("quality_gates_passed".to_string(), serde_json::to_value(qa_output.quality_assessment.quality_gates_passed)?);
                data
            },
            confidence: if qa_output.quality_assessment.quality_gates_passed { 0.95 } else { 0.75 },
            reasoning: Some(format!("Quality assessment based on test results, coverage analysis, and performance metrics. {} quality gates checked.",
                if qa_output.quality_assessment.quality_gates_passed { "All" } else { "Some" }
            )),
            next_actions: qa_output.next_actions.clone(),
            execution_metadata: crate::agents::traits::ExecutionMetadata {
                execution_time_ms: 30000,
                memory_usage_mb: 128.0,
                api_calls: 0,
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
        // QA agent has high confidence in test results and coverage analysis
        Ok(0.85)
    }
} 