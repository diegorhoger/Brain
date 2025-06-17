//! Export System Module - Task 7.2: Export Functionality
//!
//! This module provides comprehensive export capabilities for the Brain AI system,
//! supporting multiple output formats including JSON graph dumps for network
//! visualization and CSV rule tables for analysis in spreadsheet applications.
//!
//! ## Export Formats
//!
//! ### JSON Graph Export
//! - Complete graph structure with nodes and edges
//! - Network visualization compatibility (D3.js, Cytoscape, etc.)
//! - Metadata preservation and system versioning
//! - Configurable depth and filtering options
//!
//! ### CSV Rule Export
//! - Tabular format for spreadsheet analysis
//! - Rule patterns, outcomes, and confidence scores
//! - Usage statistics and validation status
//! - Temporal information and rule evolution

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::error::Result;
use crate::query_language::{QueryResult, ConceptQueryResult, MemoryQueryResult, RuleQueryResult};

/// Export system for generating data exports in various formats
#[derive(Debug, Clone)]
pub struct ExportSystem {
    /// System metadata for exports
    metadata: ExportMetadata,
    /// Export configuration options
    config: ExportConfig,
    /// Export statistics
    stats: ExportStats,
}

/// Metadata included in all exports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportMetadata {
    /// System version
    pub system_version: String,
    /// Export timestamp
    pub export_timestamp: DateTime<Utc>,
    /// Export format version
    pub format_version: String,
    /// Query parameters used for this export
    pub query_parameters: Option<String>,
    /// Total items in export
    pub total_items: usize,
    /// Export type
    pub export_type: String,
    /// Additional custom metadata
    pub custom_metadata: HashMap<String, String>,
}

/// Configuration options for exports
#[derive(Debug, Clone)]
pub struct ExportConfig {
    /// Include system metadata in exports
    pub include_metadata: bool,
    /// Pretty-print JSON exports
    pub pretty_json: bool,
    /// CSV delimiter character
    pub csv_delimiter: char,
    /// Include headers in CSV exports
    pub csv_headers: bool,
    /// Maximum items per export file
    pub max_items_per_file: Option<usize>,
    /// Custom field mappings for exports
    pub field_mappings: HashMap<String, String>,
}

/// Export execution statistics
#[derive(Debug, Clone, Default)]
pub struct ExportStats {
    pub total_exports: usize,
    pub successful_exports: usize,
    pub failed_exports: usize,
    pub total_items_exported: usize,
    pub average_export_time_ms: f64,
    pub last_export_time: Option<DateTime<Utc>>,
}

/// JSON graph export structure for network visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonGraphExport {
    /// Export metadata
    pub metadata: ExportMetadata,
    /// Graph nodes (concepts, memories, rules)
    pub nodes: Vec<GraphNode>,
    /// Graph edges (relationships)
    pub edges: Vec<GraphEdge>,
    /// Graph-level statistics
    pub graph_stats: GraphStats,
}

/// Graph node representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    /// Unique node identifier
    pub id: String,
    /// Node label for display
    pub label: String,
    /// Node type (concept, memory, rule)
    pub node_type: String,
    /// Node size hint for visualization
    pub size: f64,
    /// Node color category
    pub color: String,
    /// Additional node properties
    pub properties: HashMap<String, serde_json::Value>,
}

/// Graph edge representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    /// Source node ID
    pub source: String,
    /// Target node ID
    pub target: String,
    /// Edge type/relationship
    pub edge_type: String,
    /// Edge weight/strength
    pub weight: f64,
    /// Edge color for visualization
    pub color: String,
    /// Additional edge properties
    pub properties: HashMap<String, serde_json::Value>,
}

/// Graph-level statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphStats {
    pub node_count: usize,
    pub edge_count: usize,
    pub average_degree: f64,
    pub density: f64,
}

/// CSV export structure
#[derive(Debug, Clone, Serialize)]
pub struct CsvExport {
    /// Export metadata
    pub metadata: ExportMetadata,
    /// CSV rows as key-value maps
    pub rows: Vec<HashMap<String, String>>,
    /// Column headers
    pub headers: Vec<String>,
}

impl ExportSystem {
    /// Create a new export system
    pub fn new() -> Self {
        Self {
            metadata: ExportMetadata {
                system_version: env!("CARGO_PKG_VERSION").to_string(),
                export_timestamp: Utc::now(),
                format_version: "1.0".to_string(),
                query_parameters: None,
                total_items: 0,
                export_type: "mixed".to_string(),
                custom_metadata: HashMap::new(),
            },
            config: ExportConfig::default(),
            stats: ExportStats::default(),
        }
    }

    /// Export query results as JSON graph format
    pub fn export_json_graph<P: AsRef<Path>>(
        &mut self,
        results: &QueryResult,
        file_path: P,
        query_params: Option<&str>,
    ) -> Result<JsonGraphExport> {
        let start_time = std::time::Instant::now();

        // Create graph structure from query results
        let graph = self.create_graph_from_results(results, query_params)?;

        // Write to file
        let file = File::create(file_path)?;
        if self.config.pretty_json {
            serde_json::to_writer_pretty(file, &graph)?;
        } else {
            serde_json::to_writer(file, &graph)?;
        }

        self.update_export_stats(start_time, graph.nodes.len());
        Ok(graph)
    }

    /// Export query results as CSV format
    pub fn export_csv<P: AsRef<Path>>(
        &mut self,
        results: &QueryResult,
        file_path: P,
        query_params: Option<&str>,
    ) -> Result<CsvExport> {
        let start_time = std::time::Instant::now();

        // Create CSV structure from query results
        let csv_export = self.create_csv_from_results(results, query_params)?;

        // Write CSV file
        self.write_csv_file(&csv_export, file_path)?;

        self.update_export_stats(start_time, csv_export.rows.len());
        Ok(csv_export)
    }

    /// Export multiple formats simultaneously
    pub fn export_multi_format<P: AsRef<Path>>(
        &mut self,
        results: &QueryResult,
        base_path: P,
        query_params: Option<&str>,
    ) -> Result<(JsonGraphExport, CsvExport)> {
        let base_path = base_path.as_ref();
        
        // Generate file paths
        let json_path = base_path.with_extension("json");
        let csv_path = base_path.with_extension("csv");

        // Export both formats
        let json_export = self.export_json_graph(results, json_path, query_params)?;
        let csv_export = self.export_csv(results, csv_path, query_params)?;

        Ok((json_export, csv_export))
    }

    /// Get export statistics
    pub fn get_stats(&self) -> &ExportStats {
        &self.stats
    }

    /// Update export configuration
    pub fn update_config(&mut self, config: ExportConfig) {
        self.config = config;
    }

    // Private implementation methods

    fn create_graph_from_results(
        &mut self,
        results: &QueryResult,
        query_params: Option<&str>,
    ) -> Result<JsonGraphExport> {
        let mut nodes = Vec::new();
        let edges = Vec::new(); // Will be populated when relationship data is available

        match results {
            QueryResult::Concepts(concepts) => {
                for concept in concepts {
                    nodes.push(self.concept_to_graph_node(concept));
                }
            }
            QueryResult::Memories(memories) => {
                for memory in memories {
                    nodes.push(self.memory_to_graph_node(memory));
                }
            }
            QueryResult::Rules(rules) => {
                for rule in rules {
                    nodes.push(self.rule_to_graph_node(rule));
                }
            }
        }

        // Calculate graph statistics
        let graph_stats = self.calculate_graph_stats(&nodes, &edges);

        // Create metadata
        let metadata = ExportMetadata {
            system_version: self.metadata.system_version.clone(),
            export_timestamp: Utc::now(),
            format_version: "1.0".to_string(),
            query_parameters: query_params.map(|s| s.to_string()),
            total_items: nodes.len(),
            export_type: "json_graph".to_string(),
            custom_metadata: HashMap::new(),
        };

        Ok(JsonGraphExport {
            metadata,
            nodes,
            edges,
            graph_stats,
        })
    }

    fn create_csv_from_results(
        &mut self,
        results: &QueryResult,
        query_params: Option<&str>,
    ) -> Result<CsvExport> {
        let (headers, rows) = match results {
            QueryResult::Concepts(concepts) => {
                let headers = vec![
                    "id".to_string(),
                    "name".to_string(),
                    "type".to_string(),
                    "confidence".to_string(),
                    "usage_count".to_string(),
                    "created_at".to_string(),
                ];
                
                let rows: Vec<HashMap<String, String>> = concepts
                    .iter()
                    .map(|concept| {
                        let mut row = HashMap::new();
                        row.insert("id".to_string(), concept.id.to_string());
                        row.insert("name".to_string(), concept.name.clone());
                        row.insert("type".to_string(), concept.concept_type.clone());
                        row.insert("confidence".to_string(), concept.confidence.to_string());
                        row.insert("usage_count".to_string(), concept.usage_count.to_string());
                        row.insert("created_at".to_string(), concept.created_at.to_rfc3339());
                        row
                    })
                    .collect();
                
                (headers, rows)
            }
            QueryResult::Memories(memories) => {
                let headers = vec![
                    "id".to_string(),
                    "content".to_string(),
                    "memory_type".to_string(),
                    "importance".to_string(),
                    "relevance_score".to_string(),
                    "created_at".to_string(),
                ];
                
                let rows: Vec<HashMap<String, String>> = memories
                    .iter()
                    .map(|memory| {
                        let mut row = HashMap::new();
                        row.insert("id".to_string(), memory.id.to_string());
                        row.insert("content".to_string(), memory.content.clone());
                        row.insert("memory_type".to_string(), memory.memory_type.clone());
                        row.insert("importance".to_string(), memory.importance.clone());
                        row.insert("relevance_score".to_string(), memory.relevance_score.to_string());
                        row.insert("created_at".to_string(), memory.created_at.to_rfc3339());
                        row
                    })
                    .collect();
                
                (headers, rows)
            }
            QueryResult::Rules(rules) => {
                let headers = vec![
                    "id".to_string(),
                    "pattern".to_string(),
                    "outcome".to_string(),
                    "confidence".to_string(),
                    "rule_type".to_string(),
                    "created_at".to_string(),
                    "usage_count".to_string(),
                ];
                
                let rows: Vec<HashMap<String, String>> = rules
                    .iter()
                    .map(|rule| {
                        let mut row = HashMap::new();
                        row.insert("id".to_string(), rule.id.to_string());
                        row.insert("pattern".to_string(), rule.pattern.clone());
                        row.insert("outcome".to_string(), rule.outcome.clone());
                        row.insert("confidence".to_string(), rule.confidence.to_string());
                        row.insert("rule_type".to_string(), rule.rule_type.clone());
                        row.insert("created_at".to_string(), rule.created_at.to_rfc3339());
                        row.insert("usage_count".to_string(), rule.usage_count.to_string());
                        row
                    })
                    .collect();
                
                (headers, rows)
            }
        };

        let metadata = ExportMetadata {
            system_version: self.metadata.system_version.clone(),
            export_timestamp: Utc::now(),
            format_version: "1.0".to_string(),
            query_parameters: query_params.map(|s| s.to_string()),
            total_items: rows.len(),
            export_type: "csv".to_string(),
            custom_metadata: HashMap::new(),
        };

        Ok(CsvExport {
            metadata,
            rows,
            headers,
        })
    }

    fn concept_to_graph_node(&self, concept: &ConceptQueryResult) -> GraphNode {
        let mut properties = HashMap::new();
        properties.insert("confidence".to_string(), 
                         serde_json::Value::Number(serde_json::Number::from_f64(concept.confidence).unwrap()));
        properties.insert("usage_count".to_string(), 
                         serde_json::Value::Number(serde_json::Number::from(concept.usage_count)));

        GraphNode {
            id: concept.id.to_string(),
            label: concept.name.clone(),
            node_type: "concept".to_string(),
            size: concept.confidence * 20.0 + 5.0,
            color: "#FF6B6B".to_string(),
            properties,
        }
    }

    fn memory_to_graph_node(&self, memory: &MemoryQueryResult) -> GraphNode {
        let mut properties = HashMap::new();
        properties.insert("relevance_score".to_string(), 
                         serde_json::Value::Number(serde_json::Number::from_f64(memory.relevance_score).unwrap()));

        GraphNode {
            id: memory.id.to_string(),
            label: if memory.content.len() > 50 {
                format!("{}...", &memory.content[..47])
            } else {
                memory.content.clone()
            },
            node_type: "memory".to_string(),
            size: memory.relevance_score * 15.0 + 3.0,
            color: "#FFD93D".to_string(),
            properties,
        }
    }

    fn rule_to_graph_node(&self, rule: &RuleQueryResult) -> GraphNode {
        let mut properties = HashMap::new();
        properties.insert("confidence".to_string(), 
                         serde_json::Value::Number(serde_json::Number::from_f64(rule.confidence).unwrap()));

        GraphNode {
            id: rule.id.to_string(),
            label: format!("{} → {}", rule.pattern, rule.outcome),
            node_type: "rule".to_string(),
            size: rule.confidence * 18.0 + 4.0,
            color: "#FF8C42".to_string(),
            properties,
        }
    }

    fn calculate_graph_stats(&self, nodes: &[GraphNode], edges: &[GraphEdge]) -> GraphStats {
        let node_count = nodes.len();
        let edge_count = edges.len();
        
        let average_degree = if node_count > 0 {
            (edge_count * 2) as f64 / node_count as f64
        } else {
            0.0
        };

        let density = if node_count > 1 {
            edge_count as f64 / ((node_count * (node_count - 1)) / 2) as f64
        } else {
            0.0
        };

        GraphStats {
            node_count,
            edge_count,
            average_degree,
            density,
        }
    }

    fn write_csv_file<P: AsRef<Path>>(&self, csv_export: &CsvExport, file_path: P) -> Result<()> {
        let mut file = File::create(file_path)?;
        
        // Write headers if enabled
        if self.config.csv_headers {
            writeln!(file, "{}", csv_export.headers.join(&self.config.csv_delimiter.to_string()))?;
        }

        // Write data rows
        for row in &csv_export.rows {
            let row_values: Vec<String> = csv_export.headers
                .iter()
                .map(|header| row.get(header).cloned().unwrap_or_default())
                .collect();
            writeln!(file, "{}", row_values.join(&self.config.csv_delimiter.to_string()))?;
        }

        Ok(())
    }

    fn update_export_stats(&mut self, start_time: std::time::Instant, items_exported: usize) {
        let execution_time = start_time.elapsed().as_millis() as f64;
        
        self.stats.total_exports += 1;
        self.stats.successful_exports += 1;
        self.stats.total_items_exported += items_exported;
        self.stats.last_export_time = Some(Utc::now());
        
        // Update rolling average
        let total = self.stats.total_exports as f64;
        self.stats.average_export_time_ms = 
            (self.stats.average_export_time_ms * (total - 1.0) + execution_time) / total;
    }
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            include_metadata: true,
            pretty_json: true,
            csv_delimiter: ',',
            csv_headers: true,
            max_items_per_file: None,
            field_mappings: HashMap::new(),
        }
    }
}

impl Default for ExportSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query_language::{ConceptQueryResult, QueryResult};
    use crate::error::BrainError;
    use uuid::Uuid;
    use tempfile::tempdir;

    #[test]
    fn test_export_system_creation() {
        let export_system = ExportSystem::new();
        assert_eq!(export_system.stats.total_exports, 0);
    }

    #[test]
    fn test_json_graph_export() -> Result<()> {
        let mut export_system = ExportSystem::new();
        
        // Create mock query results
        let concepts = vec![
            ConceptQueryResult {
                id: Uuid::new_v4(),
                name: "test_concept".to_string(),
                concept_type: "skill".to_string(),
                confidence: 0.95,
                created_at: Utc::now(),
                last_updated: Utc::now(),
                usage_count: 10,
                related_concepts: vec!["related1".to_string()],
            }
        ];
        
        let results = QueryResult::Concepts(concepts);
        
        // Export to temporary file
        let temp_dir = tempdir().map_err(|e| BrainError::Other(e.to_string()))?;
        let file_path = temp_dir.path().join("test_export.json");
        
        let graph_export = export_system.export_json_graph(&results, &file_path, Some("test query"))?;
        
        assert_eq!(graph_export.nodes.len(), 1);
        assert_eq!(graph_export.metadata.export_type, "json_graph");
        assert!(file_path.exists());
        
        Ok(())
    }

    #[test]
    fn test_export_statistics() -> Result<()> {
        let mut export_system = ExportSystem::new();
        
        let concepts = vec![
            ConceptQueryResult {
                id: Uuid::new_v4(),
                name: "test".to_string(),
                concept_type: "test".to_string(),
                confidence: 0.5,
                created_at: Utc::now(),
                last_updated: Utc::now(),
                usage_count: 1,
                related_concepts: vec![],
            }
        ];
        
        let results = QueryResult::Concepts(concepts);
        let temp_dir = tempdir().map_err(|e| BrainError::Other(e.to_string()))?;
        let file_path = temp_dir.path().join("stats_test.json");
        
        export_system.export_json_graph(&results, &file_path, None)?;
        
        let stats = export_system.get_stats();
        assert_eq!(stats.total_exports, 1);
        assert_eq!(stats.successful_exports, 1);
        assert_eq!(stats.total_items_exported, 1);
        
        Ok(())
    }
} 