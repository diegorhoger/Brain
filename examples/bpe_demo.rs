use brain::segment_discovery::{BpeSegmenter, BpeConfig};
use brain::Result;

fn main() -> Result<()> {
    println!("🧠 Brain Project - Advanced BPE Segmentation Demo");
    println!("================================================");
    
    // Sample text for demonstration
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox jumps again";
    println!("\n📄 Input text:");
    println!("\"{}\"", text);
    
    // Create BPE segmenter with advanced heuristics enabled
    let config = BpeConfig {
        min_frequency: 2,
        max_vocab_size: 50,
        num_merges: 10,
        include_chars: true,
        min_entropy_threshold: 0.3,
        context_window_size: 4,
        min_confidence: 0.2,
        enable_advanced_heuristics: true,
    };
    
    println!("\n⚙️  Configuration:");
    println!("   Min frequency: {}", config.min_frequency);
    println!("   Max vocab size: {}", config.max_vocab_size);
    println!("   Entropy threshold: {:.1}", config.min_entropy_threshold);
    println!("   Context window: {}", config.context_window_size);
    println!("   Min confidence: {:.1}", config.min_confidence);
    println!("   Advanced heuristics: {}", config.enable_advanced_heuristics);
    
    let mut bpe = BpeSegmenter::new(config.clone());
    
    // Initialize and train
    println!("\n🔍 Initializing and training BPE...");
    bpe.initialize_from_text(text)?;
    
    let initial_stats = bpe.get_stats();
    println!("   Initial vocabulary size: {}", initial_stats.total_segments);
    println!("   Context observations: {}", initial_stats.context_observations);
    
    bpe.train()?;
    
    // Get final statistics with advanced metrics
    let stats = bpe.get_stats();
    println!("\n📊 Training Results:");
    println!("   Final vocabulary size: {}", stats.total_segments);
    println!("   Character segments: {}", stats.character_segments);
    println!("   Merged segments: {}", stats.merged_segments);
    println!("   Merges performed: {}", stats.merges_performed);
    println!("   Max segment length: {}", stats.max_segment_length);
    println!("   High confidence segments: {}", stats.high_confidence_segments);
    println!("   Average confidence: {:.3}", stats.average_confidence);
    println!("   Average entropy: {:.3}", stats.average_entropy);
    println!("   Context observations: {}", stats.context_observations);
    
    // Display discovered segments by frequency
    println!("\n🔤 Discovered Segments (by frequency):");
    let segments_by_freq = bpe.get_segments_by_frequency();
    for (i, segment) in segments_by_freq.iter().take(15).enumerate() {
        println!("   {}: '{}' (freq: {}, len: {}, conf: {:.2}, ent: {:.2}, stab: {:.2})", 
                 i + 1, 
                 segment.segment, 
                 segment.frequency, 
                 segment.length,
                 segment.confidence,
                 segment.entropy,
                 segment.context_stability);
    }
    
    // Display high-confidence segments
    println!("\n⭐ High-Confidence Segments:");
    let high_conf_segments = bpe.get_high_confidence_segments();
    if high_conf_segments.is_empty() {
        println!("   (No segments above confidence threshold)");
    } else {
        for segment in &high_conf_segments {
            println!("   '{}' (conf: {:.3}, freq: {}, ent: {:.2})", 
                     segment.segment, 
                     segment.confidence,
                     segment.frequency,
                     segment.entropy);
        }
    }
    
    // Display segments by confidence
    println!("\n🎯 Segments by Confidence Score:");
    let segments_by_conf = bpe.get_segments_by_confidence();
    for segment in segments_by_conf.iter().take(10) {
        if segment.confidence > 0.0 {
            println!("   '{}': {:.3} (freq: {}, len: {})", 
                     segment.segment, 
                     segment.confidence,
                     segment.frequency,
                     segment.length);
        }
    }
    
    // Show merged segments with formation history
    println!("\n🔗 Merged Segments (formation history):");
    let merged_segments = bpe.get_merged_segments();
    for segment in &merged_segments {
        if let Some(ref pair) = segment.formed_from {
            println!("   '{}' <- '{}' + '{}' (step: {}, conf: {:.2})", 
                     segment.segment,
                     pair.left,
                     pair.right,
                     segment.merge_step.unwrap_or(0),
                     segment.confidence);
        }
    }
    
    // Demonstrate co-occurrence analysis
    println!("\n🤝 Context Co-occurrence Analysis:");
    let test_pairs = vec![
        ("t", "h"),
        ("h", "e"),
        ("o", "x"),
        ("q", "u"),
        ("r", "o"),
    ];
    
    for (seg1, seg2) in test_pairs {
        let strength = bpe.get_co_occurrence_strength(seg1, seg2);
        println!("   '{}' ↔ '{}': {:.3}", seg1, seg2, strength);
    }
    
    // Compare with basic BPE (without advanced heuristics)
    println!("\n📈 Comparison with Basic BPE:");
    let basic_config = BpeConfig {
        enable_advanced_heuristics: false,
        ..config
    };
    
    let mut basic_bpe = BpeSegmenter::new(basic_config);
    basic_bpe.initialize_from_text(text)?;
    basic_bpe.train()?;
    
    let basic_stats = basic_bpe.get_stats();
    
    println!("   Advanced BPE:");
    println!("     Vocabulary: {}, Merges: {}, High-conf: {}", 
             stats.total_segments, stats.merges_performed, stats.high_confidence_segments);
    println!("     Avg confidence: {:.3}, Avg entropy: {:.3}", 
             stats.average_confidence, stats.average_entropy);
    
    println!("   Basic BPE:");
    println!("     Vocabulary: {}, Merges: {}, High-conf: {}", 
             basic_stats.total_segments, basic_stats.merges_performed, basic_stats.high_confidence_segments);
    println!("     Avg confidence: {:.3}, Avg entropy: {:.3}", 
             basic_stats.average_confidence, basic_stats.average_entropy);
    
    println!("\n✨ Advanced heuristics provide enhanced segmentation quality!");
    println!("   • Entropy analysis identifies natural boundaries");
    println!("   • Confidence scoring ranks segment reliability");
    println!("   • Context tracking captures co-occurrence patterns");
    println!("   • Segment splitting prevents over-segmentation");
    
    // Demonstrate persistent storage and lifecycle management (Task 2.3)
    println!("\n💾 Persistent Storage & Lifecycle Management Demo:");
    println!("==================================================");
    
    use brain::segment_discovery::{StorageConfig, PruningConfig};
    use std::fs;
    
    // Create storage configuration
    let storage_config = StorageConfig {
        segments_path: "demo_segments.json".to_string(),
        archive_path: "demo_segments_archive.json".to_string(),
        context_path: "demo_context_matrix.json".to_string(),
        save_interval_seconds: 0, // Immediate save for demo
        compress_data: false,
        max_backups: 3,
    };
    
    // Create pruning configuration
    let pruning_config = PruningConfig {
        min_confidence_threshold: 0.5, // Higher threshold for demo
        min_age_days: 0,               // Allow immediate pruning
        max_inactive_days: 0,          // No activity required
        min_access_count: 1,           // Low access requirement
        max_segments: 20,              // Small limit for demo
        enable_auto_pruning: true,
    };
    
    // Create segmenter with storage
    let mut persistent_bpe = BpeSegmenter::with_storage(config, storage_config, pruning_config);
    persistent_bpe.initialize_from_text(text)?;
    persistent_bpe.train()?;
    
    println!("   📊 Before persistence:");
    let pre_stats = persistent_bpe.get_stats();
    println!("     Total segments: {}", pre_stats.total_segments);
    println!("     High confidence: {}", pre_stats.high_confidence_segments);
    
    // Demonstrate access tracking
    println!("   🔍 Simulating segment access...");
    let segments_to_access: Vec<String> = persistent_bpe.get_high_confidence_segments()
        .iter()
        .map(|stats| stats.segment.clone())
        .collect();
    
    for segment in segments_to_access {
        persistent_bpe.mark_segment_accessed(&segment);
    }
    
    // Save to storage
    println!("   💾 Saving to persistent storage...");
    persistent_bpe.save_to_storage("demo_segments.json")?;
    
    // Demonstrate pruning
    println!("   ✂️  Pruning low-confidence segments...");
    let pruned = persistent_bpe.prune_segments()?;
    println!("     Pruned {} segments: {:?}", pruned.len(), pruned);
    println!("     Archived segments: {}", persistent_bpe.get_archived_segments().len());
    
    // Load from storage
    println!("   📂 Loading from persistent storage...");
    let loaded_bpe = BpeSegmenter::load_from_storage("demo_segments.json")?;
    let loaded_stats = loaded_bpe.get_stats();
    
    println!("   📊 After loading:");
    println!("     Total segments: {}", loaded_stats.total_segments);
    println!("     High confidence: {}", loaded_stats.high_confidence_segments);
    println!("     Archived segments: {}", loaded_bpe.get_archived_segments().len());
    
    // Show lifecycle information
    println!("   ⏰ Segment lifecycle information:");
    let segments = loaded_bpe.get_segments_by_confidence();
    for (i, segment) in segments.iter().take(5).enumerate() {
        println!("     {}: '{}' (accessed: {} times, conf: {:.2})", 
                 i + 1, 
                 segment.segment, 
                 segment.access_count,
                 segment.confidence);
    }
    
    // Cleanup demo files
    let demo_files = ["demo_segments.json", "demo_segments_archive.json", "demo_context_matrix.json"];
    for file in &demo_files {
        if let Err(_) = fs::remove_file(file) {
            // Ignore errors - file might not exist
        }
    }
    
    println!("\n🎯 Persistent Storage Features Demonstrated:");
    println!("   • Lifecycle tracking with timestamps and access counts");
    println!("   • Configurable pruning based on confidence and usage");
    println!("   • Archive system for segment preservation");
    println!("   • Automatic backup creation and rotation");
    println!("   • Multi-file storage architecture for organization");
    println!("   • Complete serialization/deserialization integrity");
    
    Ok(())
} 