//! Specialized Query Functions - Task 7.2: Common Query Operations
//!
//! This module provides high-level, specialized query functions that implement
//! common operations for the Brain AI system. These functions build on the
//! foundation query language to provide convenient, domain-specific interfaces.

use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::error::{BrainError, Result};
use crate::query_language::{QueryEngine, QueryResult, ConceptQueryResult, RuleQueryResult};

/// Specialized query functions for common operations
#[derive(Debug, Clone)]
pub struct SpecializedQueryEngine {
    /// Underlying query engine
    query_engine: QueryEngine,
    /// Query execution statistics
    specialized_stats: SpecializedStats,
}

/// Statistics for specialized queries
#[derive(Debug, Clone, Default)]
pub struct SpecializedStats {
    pub concept_relationship_queries: usize,
    pub rule_chain_queries: usize,
    pub temporal_queries: usize,
    pub similarity_queries: usize,
}

/// Related concepts result with relationship information
#[derive(Debug, Clone)]
pub struct RelatedConceptsResult {
    /// Original concept
    pub source_concept: ConceptQueryResult,
    /// Related concepts with relationship metadata
    pub related_concepts: Vec<ConceptRelationship>,
    /// Query execution metadata
    pub query_metadata: QueryMetadata,
}

/// Concept relationship information
#[derive(Debug, Clone)]
pub struct ConceptRelationship {
    /// Related concept
    pub concept: ConceptQueryResult,
    /// Relationship type
    pub relationship_type: String,
    /// Relationship strength (0.0 to 1.0)
    pub strength: f64,
    /// Distance from source (1 = direct, 2 = one step removed, etc.)
    pub distance: usize,
}

/// Rule chain result showing connected rules
#[derive(Debug, Clone)]
pub struct RuleChainResult {
    /// Starting rule
    pub starting_rule: RuleQueryResult,
    /// Chain of connected rules
    pub rule_chain: Vec<RuleConnection>,
    /// Chain statistics
    pub chain_stats: ChainStatistics,
    /// Query execution metadata
    pub query_metadata: QueryMetadata,
}

/// Connection between rules in a chain
#[derive(Debug, Clone)]
pub struct RuleConnection {
    /// The rule in the chain
    pub rule: RuleQueryResult,
    /// How this rule connects to the previous one
    pub connection_type: String,
    /// Confidence in this connection
    pub connection_confidence: f64,
    /// Position in the chain (0-based)
    pub position: usize,
}

/// Statistics about a rule chain
#[derive(Debug, Clone)]
pub struct ChainStatistics {
    /// Total rules in chain
    pub total_rules: usize,
    /// Average confidence across chain
    pub average_confidence: f64,
    /// Weakest link confidence
    pub weakest_link_confidence: f64,
    /// Overall chain confidence
    pub chain_confidence: f64,
}

/// Query execution metadata
#[derive(Debug, Clone)]
pub struct QueryMetadata {
    /// Query execution time in milliseconds
    pub execution_time_ms: f64,
    /// Number of intermediate queries executed
    pub query_count: usize,
    /// Query timestamp
    pub timestamp: DateTime<Utc>,
}

/// Temporal query configuration
#[derive(Debug, Clone)]
pub struct TemporalQueryConfig {
    /// Start time for temporal filtering
    pub start_time: Option<DateTime<Utc>>,
    /// End time for temporal filtering
    pub end_time: Option<DateTime<Utc>>,
    /// Time window size for grouping
    pub time_window_hours: Option<u32>,
    /// Include trend analysis
    pub include_trends: bool,
}

/// Similarity search configuration
#[derive(Debug, Clone)]
pub struct SimilarityConfig {
    /// Similarity threshold (0.0 to 1.0)
    pub threshold: f64,
    /// Maximum number of results
    pub max_results: usize,
    /// Include semantic similarity
    pub semantic_similarity: bool,
    /// Include usage pattern similarity
    pub usage_similarity: bool,
}

impl SpecializedQueryEngine {
    /// Create a new specialized query engine
    pub fn new() -> Self {
        Self {
            query_engine: QueryEngine::new(),
            specialized_stats: SpecializedStats::default(),
        }
    }

    /// Find concepts related to a given concept with relationship information
    pub fn find_related_concepts(
        &mut self,
        concept_name: &str,
        max_depth: Option<usize>,
        _relationship_types: Option<Vec<String>>,
    ) -> Result<RelatedConceptsResult> {
        let start_time = std::time::Instant::now();
        let _max_depth = max_depth.unwrap_or(2);
        
        // Find the source concept
        let concept_query = format!("CONCEPTS WHERE name = '{}'", concept_name);
        let concept_result = self.query_engine.query(&concept_query)?;
        
        let source_concept = match concept_result {
            QueryResult::Concepts(mut concepts) => {
                if concepts.is_empty() {
                    return Err(BrainError::NotFound(format!("Concept '{}' not found", concept_name)));
                }
                concepts.remove(0)
            }
            _ => return Err(BrainError::InvalidQuery("Expected concept result".to_string())),
        };

        // Find related concepts (mock implementation)
        let related_concepts = vec![
            ConceptRelationship {
                concept: ConceptQueryResult {
                    id: Uuid::new_v4(),
                    name: "related_concept_1".to_string(),
                    concept_type: "skill".to_string(),
                    confidence: 0.88,
                    created_at: Utc::now(),
                    last_updated: Utc::now(),
                    usage_count: 25,
                    related_concepts: vec![],
                },
                relationship_type: "similar_to".to_string(),
                strength: 0.85,
                distance: 1,
            }
        ];

        let execution_time = start_time.elapsed().as_millis() as f64;
        self.specialized_stats.concept_relationship_queries += 1;

        let result = RelatedConceptsResult {
            source_concept,
            related_concepts,
            query_metadata: QueryMetadata {
                execution_time_ms: execution_time,
                query_count: 1,
                timestamp: Utc::now(),
            },
        };

        Ok(result)
    }

    /// Get a chain of connected rules starting from a given rule
    pub fn get_rule_chain(
        &mut self,
        starting_rule_pattern: &str,
        max_chain_length: Option<usize>,
        min_confidence: Option<f64>,
    ) -> Result<RuleChainResult> {
        let start_time = std::time::Instant::now();
        let _max_length = max_chain_length.unwrap_or(10);
        let _min_conf = min_confidence.unwrap_or(0.5);

        // Find the starting rule
        let rule_query = format!("RULES WHERE pattern CONTAINS '{}'", starting_rule_pattern);
        let rule_result = self.query_engine.query(&rule_query)?;
        
        let starting_rule = match rule_result {
            QueryResult::Rules(mut rules) => {
                if rules.is_empty() {
                    return Err(BrainError::NotFound(format!("Rule with pattern '{}' not found", starting_rule_pattern)));
                }
                rules.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
                rules.remove(0)
            }
            _ => return Err(BrainError::InvalidQuery("Expected rule result".to_string())),
        };

        // Build the rule chain (mock implementation)
        let rule_chain = vec![
            RuleConnection {
                rule: RuleQueryResult {
                    id: Uuid::new_v4(),
                    pattern: "if study then learn".to_string(),
                    outcome: "knowledge_gain".to_string(),
                    confidence: 0.90,
                    rule_type: "causal".to_string(),
                    created_at: Utc::now(),
                    usage_count: 20,
                    validation_status: "validated".to_string(),
                },
                connection_type: "causal_link".to_string(),
                connection_confidence: 0.85,
                position: 1,
            }
        ];
        
        // Calculate chain statistics
        let chain_stats = self.calculate_chain_statistics(&rule_chain);

        let execution_time = start_time.elapsed().as_millis() as f64;
        self.specialized_stats.rule_chain_queries += 1;

        Ok(RuleChainResult {
            starting_rule,
            rule_chain,
            chain_stats,
            query_metadata: QueryMetadata {
                execution_time_ms: execution_time,
                query_count: 1,
                timestamp: Utc::now(),
            },
        })
    }

    /// Find concepts, memories, or rules within a temporal window
    pub fn temporal_query(
        &mut self,
        query_type: &str,
        config: TemporalQueryConfig,
    ) -> Result<QueryResult> {
        let start_time = std::time::Instant::now();
        
        // Build temporal query string
        let mut query_parts = vec![query_type.to_uppercase()];
        
        if let Some(start_time) = config.start_time {
            query_parts.push(format!("WHERE created_at >= '{}'", start_time.to_rfc3339()));
        }
        
        if let Some(end_time) = config.end_time {
            let connector = if query_parts.len() > 1 { "AND" } else { "WHERE" };
            query_parts.push(format!("{} created_at <= '{}'", connector, end_time.to_rfc3339()));
        }
        
        query_parts.push("ORDER BY created_at ASC".to_string());
        
        let query_string = query_parts.join(" ");
        let result = self.query_engine.query(&query_string)?;

        let _execution_time = start_time.elapsed().as_millis() as f64;
        self.specialized_stats.temporal_queries += 1;

        Ok(result)
    }

    /// Find similar concepts based on various similarity metrics
    pub fn find_similar_concepts(
        &mut self,
        target_concept: &str,
        config: SimilarityConfig,
    ) -> Result<Vec<ConceptQueryResult>> {
        let start_time = std::time::Instant::now();
        
        // Get all concepts for similarity comparison
        let all_concepts_result = self.query_engine.query("CONCEPTS")?;
        
        let similar_concepts = match all_concepts_result {
            QueryResult::Concepts(concepts) => {
                let mut similarities = Vec::new();
                
                for concept in concepts {
                    if concept.name == target_concept {
                        continue; // Skip self
                    }
                    
                    let similarity_score = self.calculate_concept_similarity(&concept);
                    
                    if similarity_score >= config.threshold {
                        similarities.push((concept, similarity_score));
                    }
                }
                
                // Sort by similarity score (descending)
                similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                
                // Take top results
                similarities
                    .into_iter()
                    .take(config.max_results)
                    .map(|(concept, _score)| concept)
                    .collect()
            }
            _ => return Err(BrainError::InvalidQuery("Expected concept result".to_string())),
        };

        let _execution_time = start_time.elapsed().as_millis() as f64;
        self.specialized_stats.similarity_queries += 1;

        Ok(similar_concepts)
    }

    /// Get specialized query statistics
    pub fn get_specialized_stats(&self) -> &SpecializedStats {
        &self.specialized_stats
    }

    // Private helper methods

    fn calculate_chain_statistics(&self, chain: &[RuleConnection]) -> ChainStatistics {
        if chain.is_empty() {
            return ChainStatistics {
                total_rules: 0,
                average_confidence: 0.0,
                weakest_link_confidence: 0.0,
                chain_confidence: 0.0,
            };
        }

        let total_rules = chain.len();
        let average_confidence = chain.iter()
            .map(|connection| connection.rule.confidence)
            .sum::<f64>() / total_rules as f64;
        
        let weakest_link_confidence = chain.iter()
            .map(|connection| connection.rule.confidence)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);
        
        // Chain confidence is limited by the weakest link
        let chain_confidence = weakest_link_confidence * 0.8 + average_confidence * 0.2;

        ChainStatistics {
            total_rules,
            average_confidence,
            weakest_link_confidence,
            chain_confidence,
        }
    }

    fn calculate_concept_similarity(&self, candidate_concept: &ConceptQueryResult) -> f64 {
        // Mock similarity calculation
        let confidence_factor = candidate_concept.confidence;
        let usage_factor = (candidate_concept.usage_count as f64 / 100.0).min(1.0);
        
        (confidence_factor + usage_factor) / 2.0
    }
}

impl Default for SpecializedQueryEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specialized_query_engine_creation() {
        let engine = SpecializedQueryEngine::new();
        assert_eq!(engine.specialized_stats.concept_relationship_queries, 0);
    }

    #[test]
    fn test_temporal_query_config() {
        let config = TemporalQueryConfig {
            start_time: Some(Utc::now()),
            end_time: None,
            time_window_hours: Some(24),
            include_trends: true,
        };
        
        assert!(config.start_time.is_some());
        assert_eq!(config.time_window_hours, Some(24));
        assert!(config.include_trends);
    }

    #[test]
    fn test_similarity_config() {
        let config = SimilarityConfig {
            threshold: 0.7,
            max_results: 10,
            semantic_similarity: true,
            usage_similarity: false,
        };
        
        assert!((config.threshold - 0.7).abs() < f64::EPSILON);
        assert_eq!(config.max_results, 10);
        assert!(config.semantic_similarity);
        assert!(!config.usage_similarity);
    }
} 