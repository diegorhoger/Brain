//! Quality Metrics Calculation for Elite Standards

use super::framework::EliteCodeFramework;
use serde_json::{json, Value};

/// Quality metrics for code analysis
#[derive(Debug, Clone)]
pub struct QualityMetrics {
    pub complexity_score: u32,
    pub maintainability_index: u32,
    pub readability_score: u32,
    pub test_coverage_estimate: f32,
}

/// Quality metrics calculator
pub struct QualityMetricsCalculator {
    framework: EliteCodeFramework,
}

impl QualityMetricsCalculator {
    /// Create a new metrics calculator
    pub fn new(framework: EliteCodeFramework) -> Self {
        Self { framework }
    }

    /// Calculate comprehensive quality metrics for code
    pub fn calculate_metrics(&self, code: &str, language: &str) -> QualityMetrics {
        let complexity_score = self.calculate_complexity(code, language);
        let maintainability_index = self.calculate_maintainability(code, language);
        let readability_score = self.calculate_readability(code, language);
        let test_coverage_estimate = self.estimate_test_coverage(code, language);

        QualityMetrics {
            complexity_score,
            maintainability_index,
            readability_score,
            test_coverage_estimate,
        }
    }

    /// Calculate cyclomatic complexity score
    fn calculate_complexity(&self, code: &str, language: &str) -> u32 {
        let complexity_keywords = match language {
            "rust" => vec!["if", "else", "match", "while", "for", "loop", "?"],
            "javascript" | "typescript" => vec!["if", "else", "switch", "while", "for", "do", "?", "&&", "||"],
            "python" => vec!["if", "elif", "else", "while", "for", "try", "except", "and", "or"],
            _ => vec!["if", "else", "while", "for", "switch", "case"],
        };

        let base_complexity = 1; // Each function starts with complexity 1
        let decision_points: u32 = complexity_keywords.iter()
            .map(|keyword| code.matches(keyword).count() as u32)
            .sum();

        base_complexity + decision_points
    }

    /// Calculate maintainability index
    fn calculate_maintainability(&self, code: &str, language: &str) -> u32 {
        let line_count = code.lines().count() as u32;
        let function_count = self.count_functions(code, language) as u32;
        let comment_ratio = self.calculate_comment_ratio(code, language);
        let complexity = self.calculate_complexity(code, language);

        // Simplified maintainability index calculation
        let avg_function_length = if function_count > 0 {
            line_count / function_count
        } else {
            line_count
        };

        let base_score = 100u32;
        let complexity_penalty = complexity.saturating_mul(2);
        let length_penalty = avg_function_length.saturating_mul(1);
        let comment_bonus = (comment_ratio * 20.0) as u32;

        base_score
            .saturating_sub(complexity_penalty)
            .saturating_sub(length_penalty)
            .saturating_add(comment_bonus)
            .min(100)
    }

    /// Calculate readability score
    fn calculate_readability(&self, code: &str, language: &str) -> u32 {
        let mut score = 100u32;

        // Line length penalty
        let max_line_length = self.framework.cognitive_code_design.line_length as usize;
        let long_lines = code.lines()
            .filter(|line| line.len() > max_line_length)
            .count() as u32;
        score = score.saturating_sub(long_lines * 2);

        // Nesting depth penalty (simplified)
        let deep_nesting = self.count_deep_nesting(code, language);
        score = score.saturating_sub(deep_nesting * 5);

        // Comment ratio bonus
        let comment_ratio = self.calculate_comment_ratio(code, language);
        let comment_bonus = (comment_ratio * 10.0) as u32;
        score = score.saturating_add(comment_bonus).min(100);

        // Naming quality (simplified heuristic)
        let naming_penalty = self.calculate_naming_penalty(code, language);
        score = score.saturating_sub(naming_penalty);

        score
    }

    /// Estimate test coverage based on code structure
    fn estimate_test_coverage(&self, code: &str, _language: &str) -> f32 {
        // Simple heuristic: presence of test patterns
        let test_indicators = [
            "test", "spec", "assert", "expect", "should",
            "#[test]", "describe(", "it(", "test_", "Test"
        ];

        let test_count = test_indicators.iter()
            .map(|indicator| code.matches(indicator).count())
            .sum::<usize>();

        let function_count = self.count_functions(code, "generic");
        
        if function_count == 0 {
            return 0.0;
        }

        // Very rough estimation: test indicators vs functions
        let ratio = test_count as f32 / function_count as f32;
        (ratio * 50.0).min(100.0) // Cap at 100%
    }

    /// Count functions in code
    fn count_functions(&self, code: &str, language: &str) -> usize {
        match language {
            "rust" => code.matches("fn ").count(),
            "javascript" | "typescript" => {
                code.matches("function ").count() + 
                code.matches(" => ").count() +
                code.matches("const ").filter(|_| code.contains("=>")).count()
            },
            "python" => code.matches("def ").count(),
            "go" => code.matches("func ").count(),
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
            "rust" | "javascript" | "typescript" | "go" => {
                code.lines().filter(|line| {
                    let trimmed = line.trim();
                    trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with("*")
                }).count()
            },
            "python" => {
                code.lines().filter(|line| {
                    let trimmed = line.trim();
                    trimmed.starts_with("#") || trimmed.starts_with("\"\"\"") || trimmed.starts_with("'''")
                }).count()
            },
            _ => {
                code.lines().filter(|line| {
                    let trimmed = line.trim();
                    trimmed.starts_with("//") || trimmed.starts_with("#")
                }).count()
            }
        };

        comment_lines as f32 / total_lines as f32
    }

    /// Count deep nesting occurrences
    fn count_deep_nesting(&self, code: &str, language: &str) -> u32 {
        let max_depth = self.framework.quality_metrics_elite.nesting_depth_max as usize;
        let mut violations = 0u32;

        for line in code.lines() {
            let depth = match language {
                "python" => {
                    // Count leading whitespace
                    line.len() - line.trim_start().len()
                },
                _ => {
                    // Count opening braces (simplified)
                    line.matches('{').count()
                }
            };

            if depth > max_depth {
                violations += 1;
            }
        }

        violations
    }

    /// Calculate naming quality penalty
    fn calculate_naming_penalty(&self, code: &str, language: &str) -> u32 {
        let poor_naming_patterns = match language {
            "rust" => vec!["fn a(", "fn b(", "fn c(", "let x =", "let y =", "let z ="],
            "javascript" | "typescript" => vec!["function a(", "function b(", "var x =", "let y =", "const z ="],
            "python" => vec!["def a(", "def b(", "x =", "y =", "z ="],
            _ => vec!["a(", "b(", "x =", "y ="],
        };

        poor_naming_patterns.iter()
            .map(|pattern| code.matches(pattern).count() as u32)
            .sum::<u32>() * 2 // 2 points penalty per poor name
    }

    /// Generate metrics report
    pub fn generate_metrics_report(&self, metrics: &QualityMetrics) -> Value {
        let framework_targets = &self.framework.quality_metrics_elite;
        
        json!({
            "metrics": {
                "complexity_score": metrics.complexity_score,
                "maintainability_index": metrics.maintainability_index,
                "readability_score": metrics.readability_score,
                "test_coverage_estimate": metrics.test_coverage_estimate
            },
            "thresholds": {
                "max_complexity": framework_targets.cyclomatic_complexity_max,
                "min_maintainability": framework_targets.maintainability_index_min,
                "min_test_coverage": self.framework.testing_excellence.coverage_targets.unit_test_coverage_min
            },
            "compliance": {
                "complexity_compliant": metrics.complexity_score <= framework_targets.cyclomatic_complexity_max,
                "maintainability_compliant": metrics.maintainability_index >= framework_targets.maintainability_index_min,
                "readability_compliant": metrics.readability_score >= 80,
                "test_coverage_compliant": metrics.test_coverage_estimate >= self.framework.testing_excellence.coverage_targets.unit_test_coverage_min as f32
            },
            "recommendations": self.generate_improvement_recommendations(metrics)
        })
    }

    /// Generate improvement recommendations
    fn generate_improvement_recommendations(&self, metrics: &QualityMetrics) -> Vec<String> {
        let mut recommendations = Vec::new();
        let framework_targets = &self.framework.quality_metrics_elite;

        if metrics.complexity_score > framework_targets.cyclomatic_complexity_max {
            recommendations.push(format!(
                "Reduce complexity from {} to {} or below by breaking down complex functions",
                metrics.complexity_score,
                framework_targets.cyclomatic_complexity_max
            ));
        }

        if metrics.maintainability_index < framework_targets.maintainability_index_min {
            recommendations.push(format!(
                "Improve maintainability from {} to {} or above by simplifying code structure",
                metrics.maintainability_index,
                framework_targets.maintainability_index_min
            ));
        }

        if metrics.readability_score < 80 {
            recommendations.push(format!(
                "Enhance readability from {} by improving naming, reducing line length, and adding comments",
                metrics.readability_score
            ));
        }

        if metrics.test_coverage_estimate < self.framework.testing_excellence.coverage_targets.unit_test_coverage_min as f32 {
            recommendations.push(format!(
                "Increase test coverage from {:.1}% to {}% minimum",
                metrics.test_coverage_estimate,
                self.framework.testing_excellence.coverage_targets.unit_test_coverage_min
            ));
        }

        if recommendations.is_empty() {
            recommendations.push("Excellent! Code meets all Elite quality metrics.".to_string());
        }

        recommendations
    }

    /// Calculate overall quality score
    pub fn calculate_overall_quality_score(&self, metrics: &QualityMetrics) -> u32 {
        let complexity_weight = 0.3;
        let maintainability_weight = 0.3;
        let readability_weight = 0.2;
        let test_coverage_weight = 0.2;

        let complexity_score = if metrics.complexity_score <= self.framework.quality_metrics_elite.cyclomatic_complexity_max {
            100
        } else {
            std::cmp::max(0, 100 - (metrics.complexity_score * 5)) as u32
        };

        let maintainability_score = metrics.maintainability_index;
        let readability_score = metrics.readability_score;
        let test_coverage_score = (metrics.test_coverage_estimate as u32).min(100);

        let weighted_score = 
            (complexity_score as f32 * complexity_weight) +
            (maintainability_score as f32 * maintainability_weight) +
            (readability_score as f32 * readability_weight) +
            (test_coverage_score as f32 * test_coverage_weight);

        weighted_score as u32
    }
}
