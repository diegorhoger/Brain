//! Code Validation Utilities for Elite Standards

use super::framework::EliteCodeFramework;
use serde_json::{json, Value};

/// Code validation results
#[derive(Debug, Clone)]
pub struct ValidationResults {
    pub compliant: bool,
    pub violations: Vec<String>,
    pub score: u32,
}

/// Elite code validator
#[derive(Clone)]
pub struct EliteCodeValidator {
    framework: EliteCodeFramework,
}

impl EliteCodeValidator {
    /// Create a new validator instance
    pub fn new(framework: EliteCodeFramework) -> Self {
        Self { framework }
    }

    /// Validate code against Elite standards
    pub fn validate_code(&self, code: &str, language: &str) -> ValidationResults {
        let mut violations = Vec::new();
        let mut score = 100u32;

        // Line length validation
        if !self.validate_line_length(code) {
            violations.push("Line length exceeds maximum allowed".to_string());
            score -= 10;
        }

        // Function complexity validation
        if !self.validate_function_complexity(code, language) {
            violations.push("Function complexity exceeds recommended limits".to_string());
            score -= 15;
        }

        // Comment ratio validation
        if !self.validate_comment_ratio(code, language) {
            violations.push("Comment ratio not within recommended range".to_string());
            score -= 5;
        }

        ValidationResults {
            compliant: violations.is_empty(),
            violations,
            score,
        }
    }

    /// Validate line length compliance
    fn validate_line_length(&self, code: &str) -> bool {
        let max_length = self.framework.cognitive_code_design.line_length as usize;
        code.lines().all(|line| line.len() <= max_length)
    }

    /// Validate function complexity
    fn validate_function_complexity(&self, code: &str, language: &str) -> bool {
        let max_length = self.framework.quality_metrics_elite.function_length_max as usize;
        let function_count = self.count_functions(code, language);
        
        if function_count == 0 {
            return true;
        }

        let avg_function_length = code.lines().count() / function_count;
        avg_function_length <= max_length
    }

    /// Validate comment ratio
    fn validate_comment_ratio(&self, code: &str, language: &str) -> bool {
        let ratio = self.calculate_comment_ratio(code, language);
        let range = &self.framework.quality_metrics_elite.comment_to_code_ratio_range;
        ratio >= range.0 && ratio <= range.1
    }

    /// Count functions in code
    fn count_functions(&self, code: &str, language: &str) -> usize {
        match language {
            "rust" => code.matches("fn ").count(),
            "javascript" | "typescript" => code.matches("function ").count() + code.matches("=>").count(),
            "python" => code.matches("def ").count(),
            _ => code.matches("function").count().max(1)
        }
    }

    /// Calculate comment ratio
    fn calculate_comment_ratio(&self, code: &str, language: &str) -> f32 {
        let total_lines = code.lines().count();
        if total_lines == 0 {
            return 0.0;
        }

        let comment_lines = match language {
            "rust" | "javascript" | "typescript" => {
                code.lines().filter(|line| line.trim_start().starts_with("//")).count()
            },
            "python" => {
                code.lines().filter(|line| line.trim_start().starts_with("#")).count()
            },
            _ => code.lines().filter(|line| {
                let trimmed = line.trim_start();
                trimmed.starts_with("//") || trimmed.starts_with("#")
            }).count()
        };

        comment_lines as f32 / total_lines as f32
    }

    /// Generate validation report
    pub fn generate_report(&self, results: &ValidationResults) -> Value {
        json!({
            "compliant": results.compliant,
            "score": results.score,
            "violations": results.violations,
            "recommendations": self.generate_recommendations(results)
        })
    }

    /// Generate recommendations
    fn generate_recommendations(&self, results: &ValidationResults) -> Vec<String> {
        if results.compliant {
            vec!["Code meets Elite standards - excellent work!".to_string()]
        } else {
            let mut recommendations = vec![
                "Review and address the identified violations".to_string(),
                "Consider refactoring to improve code quality".to_string(),
            ];
            
            if results.score < 80 {
                recommendations.push("Significant improvements needed for Elite compliance".to_string());
            }
            
            recommendations
        }
    }
} 