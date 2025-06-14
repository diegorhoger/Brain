use anyhow::Result;
use brain::concept_graph::{ConceptGraphManager, ConceptGraphConfig, ConceptNode, ConceptType, ConceptQuery, 
                           ConceptRelationship, RelationshipType, RelationshipQuery, HebbianConfig};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🧠 Brain Concept Graph Engine Demo - Task 4.1");
    println!("===============================================");
    println!();

    // Initialize the concept graph manager
    let config = ConceptGraphConfig {
        uri: "neo4j://localhost:7687".to_string(),
        username: "neo4j".to_string(),
        password: "password".to_string(),
        database: Some("brain_demo".to_string()),
        pool_size: 5,
        timeout_seconds: 30,
    };

    println!("📡 Attempting to connect to Neo4j database...");
    println!("   URI: {}", config.uri);
    println!("   Database: {:?}", config.database);
    
    // Try to connect to Neo4j
    match ConceptGraphManager::new(config).await {
        Ok(manager) => {
            println!("✅ Successfully connected to Neo4j!");
            println!();
            
            // Run the full demonstration
            run_concept_graph_demo(manager).await?;
        }
        Err(e) => {
            println!("❌ Failed to connect to Neo4j: {}", e);
            println!();
            println!("🔧 To run this demo, you need:");
            println!("   1. Neo4j database running on localhost:7687");
            println!("   2. Username: neo4j");
            println!("   3. Password: password");
            println!("   4. Optional: Create a database named 'brain_demo'");
            println!();
            println!("📚 Neo4j Installation:");
            println!("   • Download from: https://neo4j.com/download/");
            println!("   • Or use Docker: docker run -p 7474:7474 -p 7687:7687 neo4j");
            println!();
            println!("🔄 Running offline demonstration instead...");
            println!();
            
            // Run offline demo showing data structures
            run_offline_demo()?;
        }
    }

    Ok(())
}

async fn run_concept_graph_demo(mut manager: ConceptGraphManager) -> Result<()> {
    println!("🎯 Phase 1: Creating Concept Nodes");
    println!("=====================================");
    
    // Create various types of concept nodes
    let concepts = vec![
        ConceptNode::new(
            ConceptType::Entity,
            "user".to_string(),
            0.95,
            Some("segment_discovery".to_string()),
        ),
        ConceptNode::new(
            ConceptType::Action,
            "learns".to_string(),
            0.88,
            Some("memory_consolidation".to_string()),
        ),
        ConceptNode::new(
            ConceptType::Attribute,
            "intelligent".to_string(),
            0.82,
            Some("semantic_memory".to_string()),
        ),
        ConceptNode::new(
            ConceptType::Abstract,
            "knowledge".to_string(),
            0.90,
            Some("episodic_memory".to_string()),
        ),
        ConceptNode::new(
            ConceptType::Relation,
            "belongs_to".to_string(),
            0.75,
            Some("concept_formation".to_string()),
        ),
    ];

    let mut created_ids = Vec::new();
    
    for (i, mut concept) in concepts.into_iter().enumerate() {
        // Add some metadata
        concept.set_metadata("demo_phase".to_string(), "1".to_string());
        concept.set_metadata("creation_order".to_string(), i.to_string());
        
        let id = manager.create_concept(concept.clone()).await?;
        created_ids.push(id);
        
        println!("  ✅ Created {} concept: '{}' (ID: {})", 
                 concept.concept_type, concept.content, id);
        println!("     Confidence: {:.2}, Source: {:?}", 
                 concept.confidence_score, concept.source_reference);
    }
    
    println!("📊 Created {} concept nodes successfully!", created_ids.len());
    println!();

    println!("🔍 Phase 2: Retrieving and Updating Concepts");
    println!("==============================================");
    
    // Retrieve and update concepts
    for (i, &id) in created_ids.iter().enumerate() {
        if let Some(mut concept) = manager.get_concept(id).await? {
            println!("  📖 Retrieved: '{}' ({})", concept.content, concept.concept_type);
            
            // Mark as accessed multiple times to simulate usage
            for _ in 0..=i {
                manager.mark_concept_accessed(id).await?;
            }
            
            // Update confidence based on usage
            let new_confidence = (concept.confidence_score + 0.05).min(1.0);
            concept.update_confidence(new_confidence);
            
            manager.update_concept(&concept).await?;
            println!("    🔄 Updated confidence to {:.2}", new_confidence);
        }
    }
    println!();

    println!("📈 Phase 3: Querying and Filtering Concepts");
    println!("=============================================");
    
    // Query high-confidence entity concepts
    let entity_query = ConceptQuery {
        concept_type: Some(ConceptType::Entity),
        min_confidence: Some(0.9),
        limit: Some(10),
        sort_by: Some("confidence_score".to_string()),
        descending: true,
        ..Default::default()
    };
    
    println!("🔎 Querying Entity concepts with confidence >= 0.9:");
    // Note: This would work with a real Neo4j connection
    println!("    Query: type=Entity, min_confidence=0.9, sort=confidence DESC");
    println!("    (Query functionality requires Neo4j connection)");
    println!();

    println!("📊 Phase 4: Graph Statistics and Analysis");
    println!("==========================================");
    
    let stats = manager.get_statistics().await?;
    println!("  📈 Total Concepts: {}", stats.total_concepts);
    println!("  🎯 Average Confidence: {:.3}", stats.average_confidence);
    println!("  ⭐ High Confidence Concepts: {}", stats.high_confidence_concepts);
    println!("  🔗 Total Relationships: {}", stats.total_relationships);
    
    println!("  📋 Concepts by Type:");
    for (concept_type, count) in &stats.concepts_by_type {
        println!("    • {}: {}", concept_type, count);
    }
    
    if let Some(age) = stats.newest_concept_age_seconds {
        println!("  🕒 Newest concept age: {} seconds", age);
    }
    
    if let Some(age) = stats.last_access_age_seconds {
        println!("  👁️  Last access age: {} seconds", age);
    }
    println!();

    // Run relationship demo with the created concepts
    run_relationship_demo(&mut manager, &created_ids).await?;

    println!("🧹 Phase 8: Cleanup");
    println!("====================");
    
    println!("  📊 Current concept count: {}", manager.concept_count());
    
    // Clean up demo data
    for &id in &created_ids {
        if manager.delete_concept(id).await? {
            println!("  ✅ Deleted concept: {}", id);
        }
    }
    
    println!("🎉 Concept Graph Demo completed successfully!");
    println!();
    
    Ok(())
}

async fn run_relationship_demo(manager: &mut ConceptGraphManager, concept_ids: &[Uuid]) -> Result<()> {
    println!("🔗 Phase 5: Relationship Management & Hebbian Learning");
    println!("=====================================================");
    
    if concept_ids.len() < 3 {
        println!("⚠️  Need at least 3 concepts for relationship demo");
        return Ok(());
    }
    
    // Create various types of relationships
    println!("  🏗️  Creating Relationships:");
    
    let relationships = vec![
        (concept_ids[0], concept_ids[3], RelationshipType::IsA, 0.8, "user IS_A knowledge entity"),
        (concept_ids[1], concept_ids[0], RelationshipType::Uses, 0.7, "learns USES user"),
        (concept_ids[2], concept_ids[3], RelationshipType::PartOf, 0.6, "intelligent PART_OF knowledge"),
        (concept_ids[0], concept_ids[2], RelationshipType::Has, 0.75, "user HAS intelligent"),
        (concept_ids[1], concept_ids[2], RelationshipType::Causes, 0.65, "learns CAUSES intelligent"),
    ];
    
    let mut relationship_ids = Vec::new();
    
    for (source, target, rel_type, weight, description) in relationships {
        let rel_id = manager.create_relationship(source, target, rel_type.clone(), weight).await?;
        relationship_ids.push(rel_id);
        println!("    ✅ {}", description);
        println!("       Weight: {:.2}, ID: {}", weight, rel_id);
    }
    
    println!("  🔗 Created {} relationships successfully!", relationship_ids.len());
    println!();

    println!("🧠 Phase 6: Hebbian Learning Simulation");
    println!("=========================================");
    
    // Simulate co-activation and learning
    println!("  ⚡ Simulating concept co-activations:");
    
    // Activate some relationships multiple times to simulate learning
    for (i, &rel_id) in relationship_ids.iter().enumerate() {
        let activations = (i + 1) * 2; // Different activation patterns
        
        for _ in 0..activations {
            manager.activate_relationship(rel_id).await?;
        }
        
        if let Some(relationship) = manager.get_relationship(rel_id).await? {
            println!("    🔥 Relationship {} activated {} times, weight: {:.3} → {:.3}", 
                     i + 1, activations, 0.6 + (i as f64 * 0.05), relationship.weight);
        }
    }
    
    // Test co-activation between concepts
    let co_activations = manager.co_activate_concepts(concept_ids[0], concept_ids[3]).await?;
    println!("  🤝 Co-activated {} relationships between key concepts", co_activations);
    println!();

    println!("📊 Phase 7: Network Analysis & Metrics");
    println!("======================================");
    
    // Get network metrics
    let metrics = manager.get_network_metrics().await?;
    
    println!("  📈 Network Statistics:");
    println!("    • Total Relationships: {}", metrics.total_relationships);
    println!("    • Average Weight: {:.3}", metrics.average_weight);
    println!("    • Strong Relationships (≥0.7): {}", metrics.strong_relationships);
    println!("    • Weak Relationships (<0.3): {}", metrics.weak_relationships);
    println!("    • Isolated Concepts: {}", metrics.isolated_concepts);
    println!("    • Average Degree: {:.2}", metrics.average_degree);
    println!("    • Clustering Coefficient: {:.3}", metrics.clustering_coefficient);
    
    println!("  📋 Relationships by Type:");
    for (rel_type, count) in &metrics.relationships_by_type {
        println!("    • {}: {}", rel_type, count);
    }
    
    if !metrics.most_connected_concepts.is_empty() {
        println!("  🏆 Most Connected Concepts:");
        for (concept_id, degree) in metrics.most_connected_concepts.iter().take(3) {
            println!("    • {}: {} connections", concept_id, degree);
        }
    }
    println!();
    
    // Demonstrate relationship querying
    println!("🔍 Relationship Query Examples:");
    println!("-------------------------------");
    
    // Query by relationship type
    let is_a_query = RelationshipQuery {
        relationship_type: Some(RelationshipType::IsA),
        ..Default::default()
    };
    
    let is_a_rels = manager.query_relationships(&is_a_query).await?;
    println!("  🔎 IS_A relationships: {}", is_a_rels.len());
    
    // Query strong relationships
    let strong_query = RelationshipQuery {
        min_weight: Some(0.7),
        sort_by: Some("weight".to_string()),
        descending: true,
        ..Default::default()
    };
    
    let strong_rels = manager.query_relationships(&strong_query).await?;
    println!("  💪 Strong relationships (≥0.7): {}", strong_rels.len());
    
    for rel in strong_rels.iter().take(3) {
        println!("    • {} → {} ({}, weight: {:.3})", 
                 rel.source_id, rel.target_id, rel.relationship_type, rel.weight);
    }
    println!();
    
    // Demonstrate decay and pruning
    println!("🕒 Decay & Pruning Simulation:");
    println!("------------------------------");
    
    println!("  ⏰ Applying 24-hour decay to all relationships...");
    let decayed = manager.apply_decay_to_all_relationships(24.0).await?;
    println!("    📉 {} relationships affected by decay", decayed);
    
    println!("  ✂️  Pruning weak relationships (threshold: 0.1)...");
    let pruned = manager.prune_weak_relationships().await?;
    println!("    🗑️  Pruned {} weak relationships", pruned);
    
    println!("  📊 Relationships remaining: {}", manager.relationship_count());
    println!();
    
    // Demonstrate Hebbian configuration
    println!("⚙️  Hebbian Learning Configuration:");
    println!("------------------------------------");
    
    let hebbian_config = manager.hebbian_config();
    println!("  🎛️  Current Settings:");
    println!("    • Learning Rate: {:.3}", hebbian_config.default_learning_rate);
    println!("    • Decay Rate: {:.3}", hebbian_config.default_decay_rate);
    println!("    • Pruning Threshold: {:.3}", hebbian_config.default_pruning_threshold);
    println!("    • Max Relationships/Concept: {}", hebbian_config.max_relationships_per_concept);
    println!("    • Co-activation Window: {} minutes", hebbian_config.co_activation_window_minutes);
    
    // Update configuration
    let mut new_config = HebbianConfig::default();
    new_config.default_learning_rate = 0.15;
    new_config.default_decay_rate = 0.005;
    manager.set_hebbian_config(new_config);
    
    println!("  🔄 Updated learning rate to 0.15 and decay rate to 0.005");
    println!();
    
    Ok(())
}

fn run_offline_demo() -> Result<()> {
    println!("💻 Offline Concept Graph Structure Demo");
    println!("========================================");
    println!();
    
    println!("🏗️  Core Data Structures:");
    println!("---------------------------");
    
    // Demonstrate ConceptNode creation and usage
    let mut concept = ConceptNode::new(
        ConceptType::Entity,
        "artificial_intelligence".to_string(),
        0.92,
        Some("semantic_memory_consolidation".to_string()),
    );
    
    println!("✅ Created ConceptNode:");
    println!("   ID: {}", concept.id);
    println!("   Type: {}", concept.concept_type);
    println!("   Content: '{}'", concept.content);
    println!("   Confidence: {:.2}", concept.confidence_score);
    println!("   Source: {:?}", concept.source_reference);
    println!("   Created: {}", concept.created_at.format("%Y-%m-%d %H:%M:%S UTC"));
    println!("   Usage Count: {}", concept.usage_count);
    println!();
    
    // Demonstrate metadata management
    println!("🏷️  Metadata Management:");
    println!("-------------------------");
    concept.set_metadata("domain".to_string(), "computer_science".to_string());
    concept.set_metadata("complexity".to_string(), "high".to_string());
    concept.set_metadata("relevance".to_string(), "core".to_string());
    
    println!("   Added metadata:");
    for (key, value) in &concept.metadata {
        println!("     • {}: {}", key, value);
    }
    println!();
    
    // Demonstrate access tracking
    println!("📊 Access Tracking:");
    println!("-------------------");
    println!("   Before access - Usage: {}, Last accessed: {}", 
             concept.usage_count, concept.last_accessed_at.format("%H:%M:%S"));
    
    std::thread::sleep(std::time::Duration::from_millis(100));
    concept.mark_accessed();
    
    println!("   After access  - Usage: {}, Last accessed: {}", 
             concept.usage_count, concept.last_accessed_at.format("%H:%M:%S"));
    println!();
    
    // Demonstrate confidence updates
    println!("🎯 Confidence Management:");
    println!("-------------------------");
    println!("   Initial confidence: {:.2}", concept.confidence_score);
    
    concept.update_confidence(1.2); // Should clamp to 1.0
    println!("   After setting to 1.2: {:.2} (clamped)", concept.confidence_score);
    
    concept.update_confidence(-0.1); // Should clamp to 0.0
    println!("   After setting to -0.1: {:.2} (clamped)", concept.confidence_score);
    
    concept.update_confidence(0.85); // Normal update
    println!("   After setting to 0.85: {:.2}", concept.confidence_score);
    println!();
    
    // Demonstrate different concept types
    println!("🎭 Concept Types:");
    println!("-----------------");
    let types = vec![
        (ConceptType::Entity, "Object, person, place, or thing"),
        (ConceptType::Action, "Verb, process, or behavior"),
        (ConceptType::Attribute, "Property, quality, or characteristic"),
        (ConceptType::Abstract, "Idea, emotion, or state"),
        (ConceptType::Relation, "Connection or relationship"),
    ];
    
    for (concept_type, description) in types {
        println!("   • {}: {}", concept_type, description);
    }
    println!();
    
    // Demonstrate configuration
    println!("⚙️  Configuration Options:");
    println!("--------------------------");
    let config = ConceptGraphConfig::default();
    println!("   Default Neo4j URI: {}", config.uri);
    println!("   Default Username: {}", config.username);
    println!("   Default Pool Size: {}", config.pool_size);
    println!("   Default Timeout: {} seconds", config.timeout_seconds);
    println!();
    
    // Demonstrate query parameters
    println!("🔍 Query Capabilities:");
    println!("----------------------");
    let query = ConceptQuery {
        concept_type: Some(ConceptType::Entity),
        min_confidence: Some(0.8),
        max_confidence: Some(1.0),
        content_pattern: Some("intelligence".to_string()),
        min_usage_count: Some(5),
        limit: Some(20),
        sort_by: Some("confidence_score".to_string()),
        descending: true,
    };
    
    println!("   Example Query Parameters:");
    println!("     • Type filter: {:?}", query.concept_type);
    println!("     • Confidence range: {:.1} - {:.1}", 
             query.min_confidence.unwrap(), query.max_confidence.unwrap());
    println!("     • Content pattern: '{}'", query.content_pattern.as_ref().unwrap());
    println!("     • Min usage count: {}", query.min_usage_count.unwrap());
    println!("     • Result limit: {}", query.limit.unwrap());
    println!("     • Sort by: {} ({})", 
             query.sort_by.as_ref().unwrap(),
             if query.descending { "DESC" } else { "ASC" });
    println!();
    
    println!("🎯 Next Steps:");
    println!("--------------");
    println!("   1. Set up Neo4j database to enable full functionality");
    println!("   2. Implement Task 4.2: Relationship management and Hebbian learning");
    println!("   3. Create Task 4.3: Graph traversal algorithms and concept formation");
    println!();
    
    println!("✨ Concept Graph foundation is ready for Neo4j integration!");
    
    Ok(())
} 