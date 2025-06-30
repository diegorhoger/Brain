//! BPE Segmentation Demo
//!
//! Demonstrates the Brain AI BPE (Byte Pair Encoding) segmentation capabilities

use brain::{Result, segment_discovery::{BpeSegmenter, BpeConfig}};

fn main() -> Result<()> {
    println!("🧠 Brain Project - BPE Segmentation Demo");
    println!("========================================");
    
    // Sample text for demonstration
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox jumps again";
    println!("\n📄 Input text:");
    println!("\"{}\"", text);
    
    // Create BPE segmenter with configuration
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
    println!("   Num merges: {}", config.num_merges);
    println!("   Entropy threshold: {:.1}", config.min_entropy_threshold);
    println!("   Context window: {}", config.context_window_size);
    println!("   Min confidence: {:.1}", config.min_confidence);
    println!("   Advanced heuristics: {}", config.enable_advanced_heuristics);
    
    let mut bpe = BpeSegmenter::new(config.clone());
    
    // Initialize and train
    println!("\n🔍 Initializing and training BPE...");
    bpe.initialize_from_text(text)?;
    
    println!("   Initial vocabulary size: {}", bpe.vocab_size());
    
    bpe.train()?;
    
    // Get final statistics
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
    let all_segments = bpe.get_segments_by_frequency();
    let merged_segments: Vec<_> = all_segments.iter()
        .filter(|s| s.formed_from.is_some())
        .collect();
        
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
    
    // Demonstrate text segmentation
    println!("\n✂️  Text Segmentation:");
    let test_texts = vec![
        "the quick brown",
        "fox jumps over",
        "lazy dog again",
    ];
    
    for test_text in test_texts {
        let segments = bpe.segment_text(test_text);
        println!("   '{}' -> {:?}", test_text, segments);
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
    
    println!("\n✅ BPE Demo completed successfully!");
    
    Ok(())
} 