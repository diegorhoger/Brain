//! Integration tests for the Brain Architecture
//! 
//! Tests the integration between Character Predictor and Segment Discovery modules

use brain::{
    CharacterPredictor, CharacterVocab,
    FeedbackBpeSegmenter,
    SegmentAwarePredictor, SegmentProvider, PredictionFeedbackTrait, PerformanceTracker,
    PredictionFeedback, InputType, PredictionMode, IntegrationManager,
    Result
};
use std::time::Instant;

const TEST_TEXT: &str = "the quick brown fox jumps over the lazy dog the cat runs fast";

#[test]
fn test_integration_setup() -> Result<()> {
    // Create vocabulary from test text
    let vocab = CharacterVocab::from_text(TEST_TEXT);
    
    // Create character predictor
    let predictor = CharacterPredictor::new(vocab, None)?;
    
    // Create feedback-enabled segmenter
    let segmenter = FeedbackBpeSegmenter::from_text(TEST_TEXT, None)?;
    
    // Verify basic functionality
    assert!(predictor.get_prediction_mode() == PredictionMode::CharacterOnly);
    assert!(segmenter.get_segmenter().vocab_size() > 0);
    
    println!("✓ Integration setup successful");
    Ok(())
}

#[test]
fn test_segment_provider_interface() -> Result<()> {
    let segmenter = FeedbackBpeSegmenter::from_text(TEST_TEXT, None)?;
    
    // Test SegmentProvider interface
    let segments = segmenter.get_segments();
    assert!(!segments.is_empty(), "Should have discovered segments");
    
    let segmented_text = segmenter.segment_text("the cat");
    assert!(!segmented_text.is_empty(), "Should segment text");
    
    let high_confidence = segmenter.get_high_confidence_segments();
    println!("High confidence segments: {:?}", high_confidence);
    
    println!("✓ SegmentProvider interface working");
    Ok(())
}

#[test]
fn test_segment_aware_prediction() -> Result<()> {
    let vocab = CharacterVocab::from_text(TEST_TEXT);
    let mut predictor = CharacterPredictor::new(vocab, None)?;
    let segmenter = FeedbackBpeSegmenter::from_text(TEST_TEXT, None)?;
    
    // Set the segmenter for segment-aware predictions
    predictor.set_segmenter(Box::new(segmenter));
    predictor.set_prediction_mode(PredictionMode::Hybrid);
    
    // Test character prediction
    let (predicted_char, confidence) = predictor.predict_next_char("th")?;
    println!("Character prediction: '{}' with confidence: {:.3}", predicted_char, confidence);
    assert!(confidence >= 0.0 && confidence <= 1.0, "Confidence should be in [0,1]");
    
    // Test segment prediction
    let segments = vec!["the".to_string(), "quick".to_string()];
    let (predicted_segment, seg_confidence) = predictor.predict_next_segment(&segments)?;
    println!("Segment prediction: '{}' with confidence: {:.3}", predicted_segment, seg_confidence);
    assert!(seg_confidence >= 0.0 && seg_confidence <= 1.0, "Confidence should be in [0,1]");
    
    // Test hybrid prediction
    let (hybrid_pred, hybrid_conf) = predictor.predict_hybrid("th", &segments)?;
    println!("Hybrid prediction: '{}' with confidence: {:.3}", hybrid_pred, hybrid_conf);
    
    println!("✓ Segment-aware prediction working");
    Ok(())
}

#[test]
fn test_prediction_feedback() -> Result<()> {
    let mut segmenter = FeedbackBpeSegmenter::from_text(TEST_TEXT, None)?;
    
    // Create some feedback
    let feedback1 = PredictionFeedback {
        input: "the".to_string(),
        input_type: InputType::Segment,
        predicted: "cat".to_string(),
        actual: "cat".to_string(),
        is_correct: true,
        confidence: 0.85,
        prediction_time_ms: 5,
        timestamp: 1000,
        context_length: 3,
        segment_quality: Some(0.8),
    };
    
    let feedback2 = PredictionFeedback {
        input: "quick".to_string(),
        input_type: InputType::Segment,
        predicted: "brown".to_string(),
        actual: "fox".to_string(),
        is_correct: false,
        confidence: 0.60,
        prediction_time_ms: 8,
        timestamp: 1001,
        context_length: 5,
        segment_quality: Some(0.4),
    };
    
    // Report feedback
    segmenter.report_prediction(feedback1)?;
    segmenter.report_prediction(feedback2)?;
    
    // Check metrics
    let metrics = segmenter.get_performance_metrics();
    assert_eq!(metrics.total_predictions, 2);
    assert_eq!(metrics.correct_predictions, 1);
    assert_eq!(metrics.accuracy, 50.0);
    
    // Test filtering by performance
    let high_performing = segmenter.get_high_performing_segments(60.0);
    let low_performing = segmenter.get_low_performing_segments(60.0);
    
    println!("High performing segments: {:?}", high_performing);
    println!("Low performing segments: {:?}", low_performing);
    
    println!("✓ Prediction feedback system working");
    Ok(())
}

#[test]
fn test_performance_tracking() -> Result<()> {
    let vocab = CharacterVocab::from_text(TEST_TEXT);
    let mut predictor = CharacterPredictor::new(vocab, None)?;
    
    // Create mixed feedback (characters and segments)
    let char_feedback = PredictionFeedback {
        input: "t".to_string(),
        input_type: InputType::Character,
        predicted: "h".to_string(),
        actual: "h".to_string(),
        is_correct: true,
        confidence: 0.90,
        prediction_time_ms: 3,
        timestamp: 1000,
        context_length: 1,
        segment_quality: None,
    };
    
    let seg_feedback = PredictionFeedback {
        input: "the".to_string(),
        input_type: InputType::Segment,
        predicted: "cat".to_string(),
        actual: "quick".to_string(),
        is_correct: false,
        confidence: 0.75,
        prediction_time_ms: 7,
        timestamp: 1001,
        context_length: 3,
        segment_quality: Some(0.6),
    };
    
    predictor.track_prediction(char_feedback)?;
    predictor.track_prediction(seg_feedback)?;
    
    // Test metrics export/import
    let exported = predictor.export_metrics()?;
    let mut new_predictor = CharacterPredictor::new(CharacterVocab::from_text(TEST_TEXT), None)?;
    new_predictor.import_metrics(&exported)?;
    
    let comparison = new_predictor.get_performance_comparison();
    println!("Performance comparison: {:?}", comparison);
    
    assert_eq!(new_predictor.get_metrics().total_predictions, 2);
    
    println!("✓ Performance tracking and export/import working");
    Ok(())
}

#[test]
fn test_integration_manager() -> Result<()> {
    let mut manager = IntegrationManager::new();
    
    // Simulate some predictions with different performance levels
    let good_char_feedback = PredictionFeedback {
        input: "a".to_string(),
        input_type: InputType::Character,
        predicted: "b".to_string(),
        actual: "b".to_string(),
        is_correct: true,
        confidence: 0.95,
        prediction_time_ms: 2,
        timestamp: 1000,
        context_length: 1,
        segment_quality: None,
    };
    
    let bad_seg_feedback = PredictionFeedback {
        input: "slow".to_string(),
        input_type: InputType::Segment,
        predicted: "fast".to_string(),
        actual: "quick".to_string(),
        is_correct: false,
        confidence: 0.40,
        prediction_time_ms: 15,
        timestamp: 1001,
        context_length: 4,
        segment_quality: Some(0.3),
    };
    
    // Add multiple feedbacks to trigger adaptation
    for _ in 0..15 {
        manager.update_with_feedback(good_char_feedback.clone())?;
    }
    
    for _ in 0..15 {
        manager.update_with_feedback(bad_seg_feedback.clone())?;
    }
    
    let optimal_mode = manager.get_optimal_prediction_mode();
    println!("Optimal prediction mode: {:?}", optimal_mode);
    
    let metrics = manager.get_performance_metrics();
    println!("Manager metrics - Total: {}, Accuracy: {:.1}%", 
             metrics.total_predictions, metrics.accuracy);
    
    let selector = manager.get_segment_selector();
    let best_segments = selector.get_best_segments(5);
    println!("Best performing segments: {:?}", best_segments);
    
    println!("✓ Integration manager working");
    Ok(())
}

#[test]
fn test_end_to_end_workflow() -> Result<()> {
    println!("Running end-to-end integration workflow...");
    
    // 1. Setup components
    let vocab = CharacterVocab::from_text(TEST_TEXT);
    let mut predictor = CharacterPredictor::new(vocab, None)?;
    let segmenter = FeedbackBpeSegmenter::from_text(TEST_TEXT, None)?;
    let mut manager = IntegrationManager::new();
    
    println!("  ✓ Components initialized");
    
    // 2. Connect segmenter to predictor
    predictor.set_segmenter(Box::new(segmenter));
    predictor.set_prediction_mode(PredictionMode::Adaptive);
    
    // 3. Simulate a training/prediction loop
    let test_inputs = vec![
        ("th", vec!["the".to_string()]),
        ("qu", vec!["quick".to_string()]),
        ("br", vec!["brown".to_string()]),
    ];
    
    for (char_input, seg_input) in test_inputs {
        let start_time = Instant::now();
        
        // Make predictions
        let (char_pred, char_conf) = predictor.predict_next_char(char_input)?;
        let (seg_pred, seg_conf) = predictor.predict_next_segment(&seg_input)?;
        let (hybrid_pred, hybrid_conf) = predictor.predict_hybrid(char_input, &seg_input)?;
        
        let prediction_time = start_time.elapsed().as_millis() as u64;
        
        // Create feedback (simulate some correct and incorrect predictions)
        let char_feedback = predictor.create_feedback(
            char_input, 
            InputType::Character, 
            &char_pred.to_string(), 
            "e", // Simulated actual
            char_conf, 
            prediction_time,
            1, // context_length
            None, // segment_quality
        );
        
        let seg_feedback = predictor.create_feedback(
            &seg_input[0], 
            InputType::Segment, 
            &seg_pred, 
            "fox", // Simulated actual
            seg_conf, 
            prediction_time,
            seg_input[0].len(), // context_length
            Some(0.7), // segment_quality
        );
        
        // Track feedback
        predictor.track_prediction(char_feedback.clone())?;
        manager.update_with_feedback(char_feedback)?;
        manager.update_with_feedback(seg_feedback)?;
        
        println!("  ✓ Processed input: '{}' -> char: '{}' ({:.3}), seg: '{}' ({:.3}), hybrid: '{}' ({:.3})", 
                 char_input, char_pred, char_conf, seg_pred, seg_conf, hybrid_pred, hybrid_conf);
    }
    
    // 4. Analyze results
    let final_metrics = predictor.get_metrics();
    let comparison = predictor.get_performance_comparison();
    let optimal_mode = manager.get_optimal_prediction_mode();
    
    println!("  ✓ Final Results:");
    println!("    - Total predictions: {}", final_metrics.total_predictions);
    println!("    - Overall accuracy: {:.1}%", final_metrics.accuracy);
    println!("    - Character accuracy: {:.1}%", comparison.character_accuracy);
    println!("    - Segment accuracy: {:.1}%", comparison.segment_accuracy);
    println!("    - Optimal mode: {:?}", optimal_mode);
    
    println!("✓ End-to-end workflow completed successfully");
    Ok(())
}

#[test]
fn test_adaptive_segment_selection() -> Result<()> {
    use brain::AdaptiveSegmentSelector;
    
    let mut selector = AdaptiveSegmentSelector::new(3, 70.0); // 3 samples, 70% threshold
    
    // Create feedback for different segments
    let segments = vec!["the", "quick", "brown", "fox"];
    let accuracies = vec![0.90, 0.60, 0.85, 0.45]; // Different performance levels
    
    for (segment, accuracy) in segments.iter().zip(accuracies.iter()) {
        for i in 0..5 { // 5 samples each
            let feedback = PredictionFeedback {
                input: segment.to_string(),
                input_type: InputType::Segment,
                predicted: "test".to_string(),
                actual: if i as f64 / 5.0 < *accuracy { "test" } else { "wrong" }.to_string(),
                is_correct: i as f64 / 5.0 < *accuracy,
                confidence: *accuracy,
                prediction_time_ms: 5,
                timestamp: 1000 + i as u64,
                context_length: segment.len(),
                segment_quality: Some(if i as f64 / 5.0 < *accuracy { 0.75 } else { 0.25 }),
            };
            selector.update_segment_performance(segment, &feedback);
        }
    }
    
    // Test selection
    let best_segments = selector.get_best_segments(10);
    println!("Best segments: {:?}", best_segments);
    
    // Should include high-performing segments
    assert!(best_segments.contains(&"the".to_string()));
    assert!(best_segments.contains(&"brown".to_string()));
    
    // Should exclude low-performing segments
    assert!(!best_segments.contains(&"fox".to_string()));
    
    // Test individual segment checks
    assert!(selector.should_use_segment("the"));
    assert!(!selector.should_use_segment("fox"));
    
    println!("✓ Adaptive segment selection working");
    Ok(())
} 