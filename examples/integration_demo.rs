//! Advanced Integration Demo - Task 2.4
//! 
//! This example demonstrates the enhanced integration between CharacterPredictor 
//! and BpeSegmenter with adaptive learning, intelligent mode switching, and 
//! comprehensive performance analytics.

use brain::character_ingestion::{CharacterVocab, CharacterPredictor, ModelConfig};
use brain::segment_discovery::{BpeConfig, FeedbackBpeSegmenter};
use brain::integration::{
    IntegrationManager, PredictionFeedback, InputType, PredictionMode,
    AdaptiveLearningConfig, ModeSwitchingConfig, SegmentAwarePredictor, SegmentProvider
};
use brain::Result;

fn main() -> Result<()> {
    println!("🧠 Brain Neural Architecture - Advanced Integration Demo (Task 2.4)");
    println!("================================================================");
    
    // Sample training text with rich patterns
    let training_text = "the quick brown fox jumps over the lazy dog. \
                        the fox is quick and the dog is lazy. \
                        brown foxes and lazy dogs are common. \
                        quick movements and lazy afternoons. \
                        the brown quick fox and the lazy sleeping dog.";
    
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
    
    // 2. Initialize BPE Segmenter with advanced heuristics
    println!("\n🔍 Initializing BPE Segmenter with advanced heuristics...");
    let bpe_config = BpeConfig {
        min_frequency: 2,
        max_vocab_size: 1000,
        num_merges: 20,
        include_chars: true,
        enable_advanced_heuristics: true,
        min_entropy_threshold: 0.3,
        context_window_size: 3,
        min_confidence: 0.4,
    };
    
    let feedback_segmenter = FeedbackBpeSegmenter::from_text(training_text, Some(bpe_config))?;
    println!("✅ BPE segmenter initialized and trained");
    
    // Display discovered segments
    let bpe_stats = feedback_segmenter.get_segmenter().get_stats();
    println!("📊 Discovered {} segments ({} merged)", 
             bpe_stats.total_segments, bpe_stats.merged_segments);
    
    let high_confidence_segments = feedback_segmenter.get_high_confidence_segments();
    println!("🎯 High confidence segments: {:?}", 
             high_confidence_segments.iter().take(10).collect::<Vec<_>>());
    
    // 3. Set up Advanced Integration Manager
    println!("\n🤖 Setting up Advanced Integration Manager...");
    let learning_config = AdaptiveLearningConfig {
        learning_rate: 0.15,
        history_size: 500,
        significance_threshold: 2.0,
        enable_context_learning: true,
        enable_quality_assessment: true,
    };
    
    let mode_config = ModeSwitchingConfig {
        min_predictions_for_switch: 20,
        accuracy_threshold_diff: 3.0,
        confidence_threshold: 0.65,
        degradation_tolerance: 8.0,
        enable_auto_switching: true,
    };
    
    let mut integration_manager = IntegrationManager::with_config(mode_config, learning_config);
    println!("✅ Integration manager configured with adaptive learning");
    
    // 4. Connect segmenter to predictor
    predictor.set_segmenter(Box::new(feedback_segmenter));
    predictor.set_prediction_mode(PredictionMode::Adaptive);
    
    // 5. Demonstrate different prediction modes with feedback learning
    println!("\n🔮 Testing Prediction Modes with Adaptive Learning");
    println!("==================================================");
    
    let test_inputs = vec![
        "the quick",
        "brown fox",
        "lazy dog",
        "quick brown",
        "fox jumps",
        "over the",
        "sleeping",
        "afternoon",
    ];
    
    for (i, input) in test_inputs.iter().enumerate() {
        println!("\n--- Test {} ---", i + 1);
        println!("Input: '{}'", input);
        
        // Test character-level prediction
        predictor.set_prediction_mode(PredictionMode::CharacterOnly);
        let (char_pred, char_conf) = predictor.predict_next_char(input)?;
        println!("Character prediction: '{}' (confidence: {:.3})", char_pred, char_conf);
        
        // Test segment-level prediction
        predictor.set_prediction_mode(PredictionMode::SegmentOnly);
        let segments = vec![input.to_string()];
        let (seg_pred, seg_conf) = predictor.predict_next_segment(&segments)?;
        println!("Segment prediction: '{}' (confidence: {:.3})", seg_pred, seg_conf);
        
        // Test hybrid prediction
        predictor.set_prediction_mode(PredictionMode::Hybrid);
        let (hybrid_pred, hybrid_conf) = predictor.predict_hybrid(input, &segments)?;
        println!("Hybrid prediction: '{}' (confidence: {:.3})", hybrid_pred, hybrid_conf);
        
        // Create feedback for learning (simulating actual vs predicted)
        let actual_char = if i % 3 == 0 { char_pred.to_string() } else { "x".to_string() };
        
        let char_feedback = PredictionFeedback {
            input: input.to_string(),
            input_type: InputType::Character,
            predicted: char_pred.to_string(),
            actual: actual_char.clone(),
            is_correct: char_pred.to_string() == actual_char,
            confidence: char_conf,
            prediction_time_ms: 15 + (i as u64 * 2),
            timestamp: 1000000 + (i as u64 * 100),
            context_length: input.len(),
            segment_quality: None,
        };
        
        let seg_feedback = PredictionFeedback {
            input: input.to_string(),
            input_type: InputType::Segment,
            predicted: seg_pred.clone(),
            actual: if i % 2 == 0 { seg_pred.clone() } else { "unknown".to_string() },
            is_correct: i % 2 == 0,
            confidence: seg_conf,
            prediction_time_ms: 8 + (i as u64),
            timestamp: 1000000 + (i as u64 * 100),
            context_length: segments.len(),
            segment_quality: Some(0.7 + (i as f64 * 0.05)),
        };
        
        let hybrid_feedback = PredictionFeedback {
            input: format!("{}|{}", input, segments.join(",")),
            input_type: InputType::Hybrid,
            predicted: hybrid_pred.clone(),
            actual: if i % 4 == 0 { hybrid_pred.clone() } else { "mixed".to_string() },
            is_correct: i % 4 == 0,
            confidence: hybrid_conf,
            prediction_time_ms: 12 + (i as u64),
            timestamp: 1000000 + (i as u64 * 100),
            context_length: input.len() + segments.len(),
            segment_quality: Some(0.8 + (i as f64 * 0.02)),
        };
        
        // Update integration manager with feedback
        integration_manager.update_with_feedback(char_feedback)?;
        integration_manager.update_with_feedback(seg_feedback)?;
        integration_manager.update_with_feedback(hybrid_feedback)?;
        
        // Show current recommended mode
        let analysis = integration_manager.get_integration_analysis();
        println!("🎯 Current mode: {:?} | Recommended: {:?}", 
                 analysis.current_mode, analysis.recommended_mode);
        
        if i >= 3 {
            println!("📈 Learning effectiveness: {:.3}", analysis.learning_effectiveness);
        }
    }
    
    // 6. Display comprehensive performance analytics
    println!("\n📊 Comprehensive Performance Analytics");
    println!("=====================================");
    
    let performance = integration_manager.get_performance_metrics();
    println!("Total predictions: {}", performance.total_predictions);
    println!("Overall accuracy: {:.2}%", performance.accuracy);
    println!("Recent accuracy: {:.2}%", performance.recent_accuracy);
    println!("Improvement rate: {:.2}%", performance.improvement_rate);
    println!("Learning effectiveness: {:.3}", performance.learning_effectiveness);
    
    let comparison = performance.compare_performance();
    println!("\n🔍 Mode Comparison:");
    println!("Character accuracy: {:.2}% ({} predictions)", 
             comparison.character_accuracy, comparison.character_count);
    println!("Segment accuracy: {:.2}% ({} predictions)", 
             comparison.segment_accuracy, comparison.segment_count);
    println!("Hybrid accuracy: {:.2}% ({} predictions)", 
             comparison.hybrid_accuracy, comparison.hybrid_count);
    println!("Segment advantage: {:.2}%", comparison.segment_advantage);
    println!("Hybrid advantage: {:.2}%", comparison.hybrid_advantage);
    println!("Recommended mode: {:?}", comparison.recommended_mode);
    
    // 7. Advanced segment analytics
    println!("\n🎯 Advanced Segment Analytics");
    println!("============================");
    
    let segment_selector = integration_manager.get_segment_selector();
    let best_segments = segment_selector.get_best_segments(8);
    println!("Top performing segments: {:?}", best_segments);
    
    // Analyze individual segments
    for segment in best_segments.iter().take(3) {
        if let Some(analysis) = segment_selector.get_segment_analysis(segment) {
            println!("\n📋 Analysis for segment '{}':", segment);
            println!("  Overall score: {:.3}", analysis.overall_score);
            println!("  Recommendation: {}", analysis.recommendation);
            println!("  Confidence level: {}", analysis.confidence_level);
            println!("  Usage frequency: {}", analysis.quality_metrics.usage_frequency);
            println!("  Prediction accuracy: {:.3}", analysis.quality_metrics.prediction_accuracy);
            println!("  Speed improvement: {:.3}", analysis.quality_metrics.speed_improvement);
        }
    }
    
    // 8. Context-aware recommendations
    println!("\n🧭 Context-Aware Recommendations");
    println!("================================");
    
    for context_len in [3, 5, 8] {
        let context_segments = segment_selector.get_context_segments(context_len);
        println!("Context length {}: {:?}", context_len, 
                 context_segments.iter().take(5).collect::<Vec<_>>());
    }
    
    // 9. Export full analytics
    println!("\n💾 Exporting Full Analytics...");
    match integration_manager.export_full_analytics() {
        Ok(analytics_json) => {
            println!("✅ Analytics exported ({} bytes)", analytics_json.len());
            
            // Save to file for inspection
            std::fs::write("integration_analytics.json", &analytics_json)
                .map_err(|e| brain::error::BrainError::Io { source: e })?;
            println!("📁 Analytics saved to 'integration_analytics.json'");
        }
        Err(e) => println!("❌ Failed to export analytics: {}", e),
    }
    
    // 10. Demonstrate adaptive mode switching
    println!("\n🔄 Demonstrating Adaptive Mode Switching");
    println!("========================================");
    
    let initial_mode = integration_manager.get_optimal_prediction_mode();
    println!("Initial optimal mode: {:?}", initial_mode);
    
    // Simulate more predictions with varying performance to trigger mode switching
    for round in 1..=3 {
        println!("\nRound {} - Simulating {} predictions...", round, 10);
        
        for i in 0..10 {
            let performance_factor = match round {
                1 => 0.9, // High character performance
                2 => 0.6, // Mixed performance
                3 => 0.8, // High segment performance
                _ => 0.7,
            };
            
            let feedback = PredictionFeedback {
                input: format!("test_{}", i),
                input_type: match round {
                    1 => InputType::Character,
                    2 => InputType::Hybrid,
                    3 => InputType::Segment,
                    _ => InputType::Character,
                },
                predicted: "result".to_string(),
                actual: "result".to_string(),
                is_correct: (i as f64 / 10.0) < performance_factor,
                confidence: performance_factor + (i as f64 * 0.01),
                prediction_time_ms: 10 + round,
                timestamp: 2000000 + (round as u64 * 1000) + (i as u64),
                context_length: 5,
                segment_quality: Some(performance_factor),
            };
            
            integration_manager.update_with_feedback(feedback)?;
        }
        
        let current_mode = integration_manager.get_optimal_prediction_mode();
        let analysis = integration_manager.get_integration_analysis();
        
        println!("Current mode: {:?}", current_mode);
        println!("Mode switches so far: {}", analysis.integration_stats.total_mode_switches);
        println!("Adaptation success rate: {:.2}%", analysis.adaptation_success_rate * 100.0);
        
        if current_mode != initial_mode {
            println!("🔄 Mode switch detected! From {:?} to {:?}", initial_mode, current_mode);
        }
    }
    
    println!("\n🎉 Advanced Integration Demo Complete!");
    println!("=====================================");
    
    let final_analysis = integration_manager.get_integration_analysis();
    println!("Final recommended mode: {:?}", final_analysis.recommended_mode);
    println!("Total adaptations: {}", final_analysis.total_adaptations);
    println!("Learning effectiveness: {:.3}", final_analysis.learning_effectiveness);
    println!("Best performing segments: {:?}", 
             final_analysis.best_segments.iter().take(5).collect::<Vec<_>>());
    
    println!("\n📈 Task 2.4 Features Demonstrated:");
    println!("   ✅ Adaptive learning mechanisms");
    println!("   ✅ Intelligent mode switching");
    println!("   ✅ Context-aware segment recommendations");
    println!("   ✅ Advanced performance analytics");
    println!("   ✅ Segment quality assessment");
    println!("   ✅ Comprehensive feedback integration");
    println!("   ✅ Historical performance tracking");
    println!("   ✅ Real-time optimization");
    
    Ok(())
} 