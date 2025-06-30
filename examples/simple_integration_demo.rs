//! Simplified Integration Demo - Predictor-Segmenter Integration
//! 
//! This example demonstrates basic integration between CharacterPredictor 
//! and FeedbackBpeSegmenter with core functionality.

use brain::character_ingestion::{CharacterVocab, CharacterPredictor, ModelConfig, CharacterPredictorService};
use brain::segment_discovery::{BpeConfig, FeedbackBpeSegmenter};
use brain::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🧠 Brain - Integration Demo: Predictor-Segmenter Integration");
    println!("=============================================================");
    
    // Sample training text with rich patterns
    let training_text = "the quick brown fox jumps over the lazy dog. \
                        the fox is quick and the dog is lazy. \
                        brown foxes and lazy dogs are common. \
                        quick movements and lazy afternoons.";
    
    println!("\n📝 Training Text:");
    println!("{}", training_text);
    
    // 1. Initialize Character Predictor
    println!("\n🔤 Initializing Character Predictor...");
    let vocab = CharacterVocab::from_text(training_text);
    let config = ModelConfig {
        vocab_size: vocab.vocab_size(),
        embedding_dim: 64,
        hidden_dim: 128,
        learning_rate: 0.01,
        sequence_length: 16,
    };
    
    let mut predictor = CharacterPredictor::new(vocab.clone(), Some(config))?;
    println!("✅ Character predictor initialized with vocab size: {}", vocab.vocab_size());
    
    // 2. Initialize BPE Segmenter with feedback
    println!("\n🔍 Initializing Feedback BPE Segmenter...");
    let bpe_config = BpeConfig {
        min_frequency: 2,
        max_vocab_size: 100,
        num_merges: 10,
        include_chars: true,
        enable_advanced_heuristics: true,
        min_entropy_threshold: 0.3,
        context_window_size: 3,
        min_confidence: 0.4,
    };
    
    let feedback_segmenter = FeedbackBpeSegmenter::from_text(training_text, Some(bpe_config))?;
    println!("✅ Feedback BPE segmenter initialized and trained");
    
    // Display basic statistics
    let bpe_stats = feedback_segmenter.get_segmenter().get_stats();
    println!("📊 Segmenter Stats:");
    println!("   - Total segments: {}", bpe_stats.total_segments);
    println!("   - Merged segments: {}", bpe_stats.merged_segments);
    println!("   - Average confidence: {:.3}", bpe_stats.average_confidence);
    
    let high_confidence_segments = feedback_segmenter.get_high_confidence_segments();
    println!("🎯 High confidence segments: {} found", high_confidence_segments.len());
    for (i, segment) in high_confidence_segments.iter().take(5).enumerate() {
        println!("   {}. '{}'", i + 1, segment);
    }
    
    // 3. Demonstrate basic prediction functionality
    println!("\n🔮 Testing Basic Prediction Capabilities");
    println!("==========================================");
    
    let test_inputs = vec![
        "the quick",
        "brown fox", 
        "lazy dog",
        "quick brown",
    ];
    
    for (i, input) in test_inputs.iter().enumerate() {
        println!("\n--- Test {} ---", i + 1);
        println!("Input: '{}'", input);
        
        // Character-level prediction
        let (char_pred, char_conf) = predictor.predict_next_char(input).await?;
        println!("Character prediction: '{}' (confidence: {:.3})", char_pred, char_conf);
        
        // Segment the input text
        let segments = feedback_segmenter.segment(input)?;
        println!("Text segmentation: {:?}", segments);
        
        // Segment-aware prediction
        let (seg_pred, seg_conf) = predictor.predict_next_segment(&segments).await?;
        println!("Segment prediction: '{}' (confidence: {:.3})", seg_pred, seg_conf);
        
        // Hybrid prediction combining both approaches
        let (hybrid_pred, hybrid_conf) = predictor.predict_hybrid(input, &segments).await?;
        println!("Hybrid prediction: '{}' (confidence: {:.3})", hybrid_pred, hybrid_conf);
    }
    
    // 4. Performance comparison
    println!("\n📊 Performance Insights");
    println!("========================");
    
    let metrics = predictor.get_metrics();
    println!("Predictor Performance:");
    println!("  - Total predictions: {}", metrics.total_predictions);
    println!("  - Correct predictions: {}", metrics.correct_predictions);
    println!("  - Overall accuracy: {:.2}%", metrics.accuracy() * 100.0);
    println!("  - Character accuracy: {:.2}%", metrics.character_accuracy);
    println!("  - Segment accuracy: {:.2}%", metrics.segment_accuracy);
    println!("  - Hybrid accuracy: {:.2}%", metrics.hybrid_accuracy);
    
    // 5. Text generation demo
    println!("\n🎨 Text Generation Demo");
    println!("========================");
    
    let generation_prefixes = vec!["the", "quick", "fox"];
    
    for prefix in generation_prefixes {
        println!("\nGenerating from prefix: '{}'", prefix);
        let generated = predictor.generate(prefix, 20, 0.8).await?;
        println!("Generated text: '{}'", generated);
    }
    
    // 6. Advanced segmentation analysis
    println!("\n🔬 Advanced Segmentation Analysis");
    println!("==================================");
    
    let analysis_texts = vec![
        "the quick brown fox",
        "jumps over the lazy dog", 
        "foxes and dogs are animals",
    ];
    
    for text in analysis_texts {
        println!("\nAnalyzing: '{}'", text);
        let segments = feedback_segmenter.segment(text)?;
        println!("  Segments: {:?}", segments);
        println!("  Segment count: {}", segments.len());
        println!("  Average segment length: {:.1}", 
                 segments.iter().map(|s| s.len()).sum::<usize>() as f64 / segments.len() as f64);
    }
    
    println!("\n🎉 Integration Demo Complete!");
    println!("==============================");
    println!("✅ Successfully demonstrated:");
    println!("   • Character prediction with confidence scoring");
    println!("   • Advanced BPE segmentation with feedback");
    println!("   • Segment-aware prediction capabilities");
    println!("   • Hybrid prediction combining both approaches");
    println!("   • Text generation from prefixes");
    println!("   • Performance metrics and analysis");
    println!("   • Advanced segmentation analysis");
    println!("\n🚀 The Brain AI system now features robust predictor-segmenter integration!");
    
    Ok(())
} 