//! Response Quality Assessment
//! 
//! This module provides response quality assessment, safety evaluation, and 
//! source attribution for conversation responses.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::{RetrievedKnowledge, RiskLevel};

/// Comprehensive response quality metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseQuality {
    pub factual_grounding: f64,
    pub coherence: f64,
    pub relevance: f64,
    pub safety_score: f64,
    // Enhanced quality metrics for Task 13.3
    pub source_attribution: f64,
    pub consistency_score: f64,
    pub completeness: f64,
    pub clarity: f64,
    pub toxicity_score: f64,
    pub bias_score: f64,
    pub hallucination_risk: f64,
    pub confidence_calibration: f64,
}

impl Default for ResponseQuality {
    fn default() -> Self {
        Self {
            factual_grounding: 0.0,
            coherence: 0.0,
            relevance: 0.0,
            safety_score: 1.0, // Default to safe
            source_attribution: 0.0,
            consistency_score: 0.0,
            completeness: 0.0,
            clarity: 0.0,
            toxicity_score: 0.0,
            bias_score: 0.0,
            hallucination_risk: 0.0,
            confidence_calibration: 0.0,
        }
    }
}

impl ResponseQuality {
    /// Calculate overall quality score
    pub fn overall_score(&self) -> f64 {
        let positive_factors = self.factual_grounding + self.coherence + self.relevance + 
                              self.safety_score + self.source_attribution + self.consistency_score + 
                              self.completeness + self.clarity + self.confidence_calibration;
        
        let negative_factors = self.toxicity_score + self.bias_score + self.hallucination_risk;
        
        // Weight positive factors and subtract negative factors
        let weighted_positive = positive_factors / 9.0; // 9 positive factors
        let weighted_negative = negative_factors / 3.0; // 3 negative factors
        
        (weighted_positive - weighted_negative * 0.5).clamp(0.0, 1.0)
    }

    /// Check if response meets quality thresholds
    pub fn meets_quality_thresholds(&self) -> bool {
        self.safety_score >= 0.7 &&
        self.factual_grounding >= 0.5 &&
        self.coherence >= 0.6 &&
        self.relevance >= 0.5 &&
        self.toxicity_score <= 0.3 &&
        self.bias_score <= 0.3 &&
        self.hallucination_risk <= 0.4
    }

    /// Get quality assessment summary
    pub fn get_summary(&self) -> String {
        let overall = self.overall_score();
        let quality_level = match overall {
            score if score >= 0.8 => "Excellent",
            score if score >= 0.6 => "Good",
            score if score >= 0.4 => "Acceptable",
            score if score >= 0.2 => "Poor",
            _ => "Very Poor"
        };

        format!("Quality: {} (Score: {:.2})", quality_level, overall)
    }
}

/// Safety assessment flags and warnings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyFlags {
    pub contains_harmful_content: bool,
    pub contains_personal_info: bool,
    pub contains_misinformation: bool,
    pub contains_bias: bool,
    pub contains_inappropriate_language: bool,
    pub risk_level: RiskLevel,
    pub flagged_terms: Vec<String>,
    pub safety_recommendations: Vec<String>,
}

impl Default for SafetyFlags {
    fn default() -> Self {
        Self {
            contains_harmful_content: false,
            contains_personal_info: false,
            contains_misinformation: false,
            contains_bias: false,
            contains_inappropriate_language: false,
            risk_level: RiskLevel::Low,
            flagged_terms: Vec::new(),
            safety_recommendations: Vec::new(),
        }
    }
}

impl SafetyFlags {
    /// Check if any safety flags are raised
    pub fn has_safety_concerns(&self) -> bool {
        self.contains_harmful_content ||
        self.contains_personal_info ||
        self.contains_misinformation ||
        self.contains_bias ||
        self.contains_inappropriate_language ||
        matches!(self.risk_level, RiskLevel::High | RiskLevel::Critical)
    }

    /// Get safety summary
    pub fn get_safety_summary(&self) -> String {
        if !self.has_safety_concerns() {
            return "No safety concerns detected".to_string();
        }

        let mut concerns = Vec::new();
        
        if self.contains_harmful_content {
            concerns.push("harmful content");
        }
        if self.contains_personal_info {
            concerns.push("personal information");
        }
        if self.contains_misinformation {
            concerns.push("potential misinformation");
        }
        if self.contains_bias {
            concerns.push("bias detected");
        }
        if self.contains_inappropriate_language {
            concerns.push("inappropriate language");
        }

        format!("Safety concerns: {} (Risk: {:?})", concerns.join(", "), self.risk_level)
    }

    /// Add a safety recommendation
    pub fn add_recommendation(&mut self, recommendation: String) {
        self.safety_recommendations.push(recommendation);
    }
}

/// Source attribution for response transparency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceAttribution {
    pub knowledge_sources: Vec<AttributedSource>,
    pub confidence_per_source: HashMap<String, f64>,
    pub source_reliability: HashMap<String, f64>,
    pub citation_completeness: f64,
}

impl Default for SourceAttribution {
    fn default() -> Self {
        Self {
            knowledge_sources: Vec::new(),
            confidence_per_source: HashMap::new(),
            source_reliability: HashMap::new(),
            citation_completeness: 0.0,
        }
    }
}

impl SourceAttribution {
    /// Add a source with attribution details
    pub fn add_source(&mut self, source: AttributedSource) {
        self.confidence_per_source.insert(
            source.source_id.clone(),
            source.relevance_to_response
        );
        self.source_reliability.insert(
            source.source_id.clone(),
            source.reliability_score
        );
        self.knowledge_sources.push(source);
    }

    /// Calculate overall source reliability
    pub fn overall_reliability(&self) -> f64 {
        if self.source_reliability.is_empty() {
            return 0.0;
        }

        let total: f64 = self.source_reliability.values().sum();
        total / self.source_reliability.len() as f64
    }

    /// Get source summary
    pub fn get_source_summary(&self) -> String {
        if self.knowledge_sources.is_empty() {
            return "No sources attributed".to_string();
        }

        format!("{} sources (Reliability: {:.2}, Completeness: {:.2})",
                self.knowledge_sources.len(),
                self.overall_reliability(),
                self.citation_completeness)
    }
}

/// Individual attributed source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributedSource {
    pub source_id: String,
    pub source_type: String,
    pub content: String,
    pub relevance_to_response: f64,
    pub reliability_score: f64,
    pub timestamp: DateTime<Utc>,
    pub used_in_response: Vec<String>, // Portions of response that use this source
}

impl AttributedSource {
    /// Create a new attributed source
    pub fn new(
        source_id: String,
        source_type: String,
        content: String,
        relevance: f64,
        reliability: f64,
    ) -> Self {
        Self {
            source_id,
            source_type,
            content,
            relevance_to_response: relevance.clamp(0.0, 1.0),
            reliability_score: reliability.clamp(0.0, 1.0),
            timestamp: Utc::now(),
            used_in_response: Vec::new(),
        }
    }

    /// Add a portion of response that uses this source
    pub fn add_usage(&mut self, response_portion: String) {
        self.used_in_response.push(response_portion);
    }

    /// Calculate usage coverage
    pub fn usage_coverage(&self, total_response_length: usize) -> f64 {
        if total_response_length == 0 {
            return 0.0;
        }

        let used_length: usize = self.used_in_response
            .iter()
            .map(|portion| portion.len())
            .sum();

        used_length as f64 / total_response_length as f64
    }
}

/// Quality assessment utilities
pub struct QualityAssessmentUtils;

impl QualityAssessmentUtils {
    /// Assess factual grounding based on knowledge sources
    pub fn assess_factual_grounding(
        response: &str,
        knowledge: &[RetrievedKnowledge],
    ) -> f64 {
        if knowledge.is_empty() {
            return 0.0;
        }

        // Simple heuristic: calculate how much of the response can be attributed to knowledge
        let total_knowledge_content: String = knowledge
            .iter()
            .map(|k| k.content.clone())
            .collect::<Vec<_>>()
            .join(" ");

        Self::calculate_content_overlap(response, &total_knowledge_content)
    }

    /// Assess response coherence
    pub fn assess_coherence(response: &str) -> f64 {
        if response.is_empty() {
            return 0.0;
        }

        let sentences: Vec<&str> = response.split('.').filter(|s| !s.trim().is_empty()).collect();
        
        if sentences.len() <= 1 {
            return 1.0; // Single sentence is coherent by definition
        }

        // Simple coherence check based on sentence transitions
        let mut coherence_score = 0.0;
        for i in 1..sentences.len() {
            let current = sentences[i].trim();
            let previous = sentences[i-1].trim();
            
            // Check for transition words and topic continuity
            if Self::has_good_transition(previous, current) {
                coherence_score += 1.0;
            }
        }

        coherence_score / (sentences.len() - 1) as f64
    }

    /// Assess response relevance to query
    pub fn assess_relevance(response: &str, query: &str) -> f64 {
        Self::calculate_content_overlap(response, query)
    }

    /// Calculate content overlap between two texts
    fn calculate_content_overlap(text1: &str, text2: &str) -> f64 {
        // Create owned strings for comparison
        let text1_lower = text1.to_lowercase();
        let text2_lower = text2.to_lowercase();
        
        // Create hash sets from the owned strings
        let words1: std::collections::HashSet<&str> = text1_lower
            .split_whitespace()
            .collect();
        let words2: std::collections::HashSet<&str> = text2_lower
            .split_whitespace()
            .collect();

        if words1.is_empty() || words2.is_empty() {
            return 0.0;
        }

        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();

        intersection as f64 / union as f64
    }

    /// Check for good sentence transitions
    fn has_good_transition(_previous: &str, current: &str) -> bool {
        let transition_words = [
            "however", "therefore", "furthermore", "additionally", "moreover",
            "consequently", "nevertheless", "similarly", "likewise", "in contrast",
            "on the other hand", "for example", "specifically", "in particular"
        ];

        let current_lower = current.to_lowercase();
        transition_words.iter().any(|&word| current_lower.contains(word))
    }

    /// Assess safety concerns in text
    pub fn assess_safety(text: &str) -> SafetyFlags {
        let mut flags = SafetyFlags::default();
        let text_lower = text.to_lowercase();

        // Check for harmful content patterns
        let harmful_patterns = [
            "violence", "harm", "attack", "kill", "destroy", "dangerous",
            "illegal", "criminal", "fraud", "scam"
        ];

        // Check for personal information patterns
        let personal_info_patterns = [
            "ssn", "social security", "credit card", "password", "phone number",
            "address", "email", "personal data"
        ];

        // Check for bias indicators
        let bias_patterns = [
            "all women", "all men", "all people from", "typical of", "always",
            "never", "inferior", "superior"
        ];

        for pattern in &harmful_patterns {
            if text_lower.contains(pattern) {
                flags.contains_harmful_content = true;
                flags.flagged_terms.push(pattern.to_string());
            }
        }

        for pattern in &personal_info_patterns {
            if text_lower.contains(pattern) {
                flags.contains_personal_info = true;
                flags.flagged_terms.push(pattern.to_string());
            }
        }

        for pattern in &bias_patterns {
            if text_lower.contains(pattern) {
                flags.contains_bias = true;
                flags.flagged_terms.push(pattern.to_string());
            }
        }

        // Set risk level based on flags
        flags.risk_level = if flags.contains_harmful_content {
            RiskLevel::High
        } else if flags.contains_personal_info || flags.contains_bias {
            RiskLevel::Medium
        } else {
            RiskLevel::Low
        };

        flags
    }
} 