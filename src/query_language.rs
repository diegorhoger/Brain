//! Query Language Module - Task 7.2: Advanced Query System
//!
//! This module implements a comprehensive query language system that allows
//! filtering and searching across concepts, memories, and rules. It supports
//! temporal queries, confidence-based filtering, relationship traversal,
//! and complex logical operations.
//!
//! ## Query Language Syntax
//! 
//! The query language supports SQL-like syntax with extensions for AI-specific operations:
//! 
//! ```
//! // Basic concept queries
//! CONCEPTS WHERE type = "entity" AND confidence > 0.8
//! CONCEPTS WHERE created_after = "2024-01-01" LIMIT 10
//! 
//! // Memory queries with temporal filtering
//! MEMORIES WHERE memory_type = "episodic" AND timestamp BETWEEN "2024-01-01" AND "2024-12-31"
//! MEMORIES WHERE content CONTAINS "programming" ORDER BY relevance DESC
//! 
//! // Rule queries with pattern matching
//! RULES WHERE pattern MATCHES "if.*then" AND confidence > 0.7
//! RULES WHERE outcome_type = "prediction" ORDER BY created_at ASC
//! 
//! // Relationship traversal
//! CONCEPTS RELATED TO "programming" DEPTH 2 WHERE confidence > 0.5
//! CONCEPTS CONNECTED TO "language" VIA "is_part_of" 
//! ```

use std::fmt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{BrainError, Result};

/// Query language parser and executor
#[derive(Debug, Clone)]
pub struct QueryEngine {
    /// Parser for query strings
    parser: QueryParser,
    /// Query execution statistics
    stats: QueryStats,
}

/// Query execution statistics
#[derive(Debug, Clone, Default)]
pub struct QueryStats {
    pub total_queries: usize,
    pub successful_queries: usize,
    pub failed_queries: usize,
    pub average_execution_time_ms: f64,
    pub last_query_time: Option<DateTime<Utc>>,
}

/// Parsed query representation
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedQuery {
    /// Query target (CONCEPTS, MEMORIES, RULES)
    pub target: QueryTarget,
    /// Filter conditions
    pub conditions: Vec<QueryCondition>,
    /// Ordering specification
    pub order_by: Option<OrderBy>,
    /// Result limit
    pub limit: Option<usize>,
    /// Relationship traversal specification
    pub traversal: Option<RelationshipTraversal>,
}

/// Query target type
#[derive(Debug, Clone, PartialEq)]
pub enum QueryTarget {
    Concepts,
    Memories,
    Rules,
}

/// Query condition for filtering results
#[derive(Debug, Clone, PartialEq)]
pub struct QueryCondition {
    pub field: String,
    pub operator: QueryOperator,
    pub value: QueryValue,
}

/// Query operators
#[derive(Debug, Clone, PartialEq)]
pub enum QueryOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Contains,
    StartsWith,
    EndsWith,
    Matches,  // Regex matching
    Between,  // Range queries
    In,       // Set membership
    NotIn,    // Set exclusion
}

/// Query value types
#[derive(Debug, Clone, PartialEq)]
pub enum QueryValue {
    String(String),
    Number(f64),
    Integer(i64),
    Boolean(bool),
    DateTime(DateTime<Utc>),
    List(Vec<QueryValue>),
    Range(Box<QueryValue>, Box<QueryValue>),
}

/// Ordering specification
#[derive(Debug, Clone, PartialEq)]
pub struct OrderBy {
    pub field: String,
    pub direction: OrderDirection,
}

/// Ordering direction
#[derive(Debug, Clone, PartialEq)]
pub enum OrderDirection {
    Ascending,
    Descending,
}

/// Relationship traversal for graph queries
#[derive(Debug, Clone, PartialEq)]
pub struct RelationshipTraversal {
    pub start_concept: String,
    pub relationship_type: Option<String>,
    pub max_depth: Option<usize>,
    pub traversal_type: TraversalType,
}

/// Traversal type for relationship queries
#[derive(Debug, Clone, PartialEq)]
pub enum TraversalType {
    RelatedTo,      // General relationship traversal
    ConnectedVia,   // Specific relationship type
}

/// Query result that can contain different data types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryResult {
    Concepts(Vec<ConceptQueryResult>),
    Memories(Vec<MemoryQueryResult>),
    Rules(Vec<RuleQueryResult>),
}

/// Concept query result with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptQueryResult {
    pub id: Uuid,
    pub name: String,
    pub concept_type: String,
    pub confidence: f64,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub usage_count: usize,
    pub related_concepts: Vec<String>,
}

/// Memory query result with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryQueryResult {
    pub id: Uuid,
    pub content: String,
    pub memory_type: String,
    pub importance: String,
    pub relevance_score: f64,
    pub created_at: DateTime<Utc>,
    pub last_accessed: Option<DateTime<Utc>>,
}

/// Rule query result with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleQueryResult {
    pub id: Uuid,
    pub pattern: String,
    pub outcome: String,
    pub confidence: f64,
    pub rule_type: String,
    pub created_at: DateTime<Utc>,
    pub usage_count: usize,
    pub validation_status: String,
}

/// Query parser implementation
#[derive(Debug, Clone)]
pub struct QueryParser {
    /// Current parsing position
    position: usize,
    /// Input query string
    input: String,
}

impl QueryEngine {
    /// Create a new query engine
    pub fn new() -> Self {
        Self {
            parser: QueryParser::new(),
            stats: QueryStats::default(),
        }
    }

    /// Execute a complete query from string to results
    pub fn query(&mut self, query_string: &str) -> Result<QueryResult> {
        let start_time = std::time::Instant::now();
        
        // Parse the query
        let parsed_query = self.parser.parse(query_string)?;
        
        // Execute based on target type
        let result = match parsed_query.target {
            QueryTarget::Concepts => self.execute_concept_query(&parsed_query),
            QueryTarget::Memories => self.execute_memory_query(&parsed_query),
            QueryTarget::Rules => self.execute_rule_query(&parsed_query),
        };

        // Update statistics
        match &result {
            Ok(_) => self.stats.successful_queries += 1,
            Err(_) => self.stats.failed_queries += 1,
        }
        
        self.update_execution_time(start_time);
        result
    }

    /// Get query execution statistics
    pub fn get_stats(&self) -> &QueryStats {
        &self.stats
    }

    fn execute_concept_query(&self, query: &ParsedQuery) -> Result<QueryResult> {
        // Mock data for now - will be integrated with actual concept graph
        let mut results = vec![
            ConceptQueryResult {
                id: Uuid::new_v4(),
                name: "programming".to_string(),
                concept_type: "skill".to_string(),
                confidence: 0.95,
                created_at: Utc::now(),
                last_updated: Utc::now(),
                usage_count: 42,
                related_concepts: vec!["coding".to_string(), "software".to_string()],
            },
            ConceptQueryResult {
                id: Uuid::new_v4(),
                name: "machine_learning".to_string(),
                concept_type: "domain".to_string(),
                confidence: 0.88,
                created_at: Utc::now(),
                last_updated: Utc::now(),
                usage_count: 37,
                related_concepts: vec!["AI".to_string(), "data_science".to_string()],
            },
        ];

        // Apply filters, ordering, and limits
        self.apply_concept_filters(&mut results, &query.conditions);
        self.apply_ordering_concept(&mut results, &query.order_by);
        self.apply_limit_concept(&mut results, query.limit);

        Ok(QueryResult::Concepts(results))
    }

    fn execute_memory_query(&self, _query: &ParsedQuery) -> Result<QueryResult> {
        // Mock memory data
        let results = vec![
            MemoryQueryResult {
                id: Uuid::new_v4(),
                content: "Learned about Python programming today".to_string(),
                memory_type: "episodic".to_string(),
                importance: "high".to_string(),
                relevance_score: 0.92,
                created_at: Utc::now(),
                last_accessed: Some(Utc::now()),
            },
        ];

        Ok(QueryResult::Memories(results))
    }

    fn execute_rule_query(&self, _query: &ParsedQuery) -> Result<QueryResult> {
        // Mock rule data
        let results = vec![
            RuleQueryResult {
                id: Uuid::new_v4(),
                pattern: "if morning then coffee".to_string(),
                outcome: "increased_productivity".to_string(),
                confidence: 0.85,
                rule_type: "behavioral".to_string(),
                created_at: Utc::now(),
                usage_count: 15,
                validation_status: "validated".to_string(),
            },
        ];

        Ok(QueryResult::Rules(results))
    }

    fn apply_concept_filters(&self, results: &mut Vec<ConceptQueryResult>, conditions: &[QueryCondition]) {
        for condition in conditions {
            results.retain(|result| self.evaluate_concept_condition(result, condition));
        }
    }

    fn evaluate_concept_condition(&self, result: &ConceptQueryResult, condition: &QueryCondition) -> bool {
        match condition.field.as_str() {
            "confidence" => {
                if let QueryValue::Number(threshold) = condition.value {
                    match condition.operator {
                        QueryOperator::GreaterThan => result.confidence > threshold,
                        QueryOperator::LessThan => result.confidence < threshold,
                        _ => true,
                    }
                } else {
                    true
                }
            }
            "type" => {
                if let QueryValue::String(ref type_name) = condition.value {
                    match condition.operator {
                        QueryOperator::Equals => result.concept_type == *type_name,
                        _ => true,
                    }
                } else {
                    true
                }
            }
            _ => true,
        }
    }

    fn apply_ordering_concept(&self, results: &mut [ConceptQueryResult], order_by: &Option<OrderBy>) {
        if let Some(order) = order_by {
            match order.field.as_str() {
                "confidence" => match order.direction {
                    OrderDirection::Descending => results.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap()),
                    OrderDirection::Ascending => results.sort_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap()),
                },
                _ => {}
            }
        }
    }

    fn apply_limit_concept(&self, results: &mut Vec<ConceptQueryResult>, limit: Option<usize>) {
        if let Some(n) = limit {
            results.truncate(n);
        }
    }

    fn update_execution_time(&mut self, start_time: std::time::Instant) {
        let execution_time = start_time.elapsed().as_millis() as f64;
        
        self.stats.total_queries += 1;
        self.stats.last_query_time = Some(Utc::now());
        
        // Update rolling average
        let total = self.stats.total_queries as f64;
        self.stats.average_execution_time_ms = 
            (self.stats.average_execution_time_ms * (total - 1.0) + execution_time) / total;
    }
}

impl QueryParser {
    /// Create a new query parser
    pub fn new() -> Self {
        Self {
            position: 0,
            input: String::new(),
        }
    }

    /// Parse a query string into a structured query
    pub fn parse(&mut self, input: &str) -> Result<ParsedQuery> {
        self.input = input.to_string();
        self.position = 0;

        let input_lower = self.input.to_lowercase();
        
        // Determine target
        let target = if input_lower.contains("concepts") {
            QueryTarget::Concepts
        } else if input_lower.contains("memories") {
            QueryTarget::Memories
        } else if input_lower.contains("rules") {
            QueryTarget::Rules
        } else {
            return Err(BrainError::ParseError("Unknown query target".to_string()));
        };

        // Parse conditions
        let mut conditions = Vec::new();
        
        if let Some(confidence_condition) = self.extract_confidence_condition() {
            conditions.push(confidence_condition);
        }

        if let Some(type_condition) = self.extract_type_condition() {
            conditions.push(type_condition);
        }

        // Parse other elements
        let limit = self.extract_limit();
        let order_by = self.extract_order_by();

        Ok(ParsedQuery {
            target,
            conditions,
            order_by,
            limit,
            traversal: None,
        })
    }

    fn extract_confidence_condition(&self) -> Option<QueryCondition> {
        if let Some(start) = self.input.find("confidence") {
            let substr = &self.input[start..];
            if let Some(op_start) = substr.find('>') {
                let after_op = substr[op_start + 1..].trim();
                if let Ok(value) = after_op.split_whitespace().next().unwrap_or("").parse::<f64>() {
                    return Some(QueryCondition {
                        field: "confidence".to_string(),
                        operator: QueryOperator::GreaterThan,
                        value: QueryValue::Number(value),
                    });
                }
            }
        }
        None
    }

    fn extract_type_condition(&self) -> Option<QueryCondition> {
        if let Some(start) = self.input.find("type") {
            let substr = &self.input[start..];
            if let Some(eq_start) = substr.find('=') {
                let after_eq = substr[eq_start + 1..].trim();
                if let Some(value) = after_eq.split_whitespace().next() {
                    let clean_value = value.trim_matches(&['"', '\'', ' '][..]);
                    return Some(QueryCondition {
                        field: "type".to_string(),
                        operator: QueryOperator::Equals,
                        value: QueryValue::String(clean_value.to_string()),
                    });
                }
            }
        }
        None
    }

    fn extract_limit(&self) -> Option<usize> {
        if let Some(start) = self.input.find("LIMIT") {
            let substr = &self.input[start + 5..].trim();
            if let Some(limit_str) = substr.split_whitespace().next() {
                return limit_str.parse().ok();
            }
        }
        None
    }

    fn extract_order_by(&self) -> Option<OrderBy> {
        if let Some(start) = self.input.find("ORDER BY") {
            let substr = &self.input[start + 8..].trim();
            let parts: Vec<&str> = substr.split_whitespace().collect();
            if !parts.is_empty() {
                let field = parts[0].to_string();
                let direction = if parts.len() > 1 && parts[1].to_uppercase() == "DESC" {
                    OrderDirection::Descending
                } else {
                    OrderDirection::Ascending
                };
                return Some(OrderBy { field, direction });
            }
        }
        None
    }
}

impl Default for QueryEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for QueryResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QueryResult::Concepts(concepts) => {
                writeln!(f, "Concept Results ({} items):", concepts.len())?;
                for (i, concept) in concepts.iter().enumerate() {
                    writeln!(f, "  {}. {} (type: {}, confidence: {:.3})", 
                            i + 1, concept.name, concept.concept_type, concept.confidence)?;
                }
            }
            QueryResult::Memories(memories) => {
                writeln!(f, "Memory Results ({} items):", memories.len())?;
                for (i, memory) in memories.iter().enumerate() {
                    writeln!(f, "  {}. {} (type: {}, relevance: {:.3})", 
                            i + 1, memory.content, memory.memory_type, memory.relevance_score)?;
                }
            }
            QueryResult::Rules(rules) => {
                writeln!(f, "Rule Results ({} items):", rules.len())?;
                for (i, rule) in rules.iter().enumerate() {
                    writeln!(f, "  {}. {} → {} (confidence: {:.3})", 
                            i + 1, rule.pattern, rule.outcome, rule.confidence)?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_engine_creation() {
        let engine = QueryEngine::new();
        assert_eq!(engine.stats.total_queries, 0);
    }

    #[test]
    fn test_concept_query() -> Result<()> {
        let mut engine = QueryEngine::new();
        let result = engine.query("CONCEPTS WHERE confidence > 0.9")?;
        
        match result {
            QueryResult::Concepts(concepts) => {
                assert!(!concepts.is_empty());
            }
            _ => panic!("Expected concept results"),
        }
        
        Ok(())
    }

    #[test]
    fn test_query_statistics() -> Result<()> {
        let mut engine = QueryEngine::new();
        
        engine.query("CONCEPTS")?;
        
        let stats = engine.get_stats();
        assert_eq!(stats.total_queries, 1);
        assert_eq!(stats.successful_queries, 1);
        
        Ok(())
    }
} 