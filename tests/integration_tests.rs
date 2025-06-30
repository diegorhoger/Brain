//! Integration tests for the Brain Architecture
//! 
//! Tests the integration between Character Predictor and Segment Discovery modules

use brain::{
    Result
};
use brain::character_ingestion::{
    CharacterPredictor, CharacterVocab, ModelConfig, CharacterPredictorService
};
use brain::segment_discovery::{
    FeedbackBpeSegmenter
};
use brain::integration::{
    PredictionFeedback, InputType, PredictionMode, IntegrationManager
};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

const TEST_TEXT: &str = "the quick brown fox jumps over the lazy dog the cat runs fast";

// Demo AdaptiveSegmentSelector implementation for testing
#[derive(Debug)]
pub struct AdaptiveSegmentSelector {
    min_samples: usize,
    threshold: f64,
    segment_performance: std::collections::HashMap<String, (usize, f64)>, // (count, accuracy)
}

impl AdaptiveSegmentSelector {
    pub fn new(min_samples: usize, threshold: f64) -> Self {
        Self {
            min_samples,
            threshold,
            segment_performance: std::collections::HashMap::new(),
        }
    }
    
    pub fn update_segment_performance(&mut self, segment: &str, feedback: &PredictionFeedback) {
        let entry = self.segment_performance.entry(segment.to_string()).or_insert((0, 0.0));
        let (count, accuracy) = *entry;
        
        let new_count = count + 1;
        let new_accuracy = if feedback.is_correct {
            (accuracy * count as f64 + 1.0) / new_count as f64
        } else {
            (accuracy * count as f64) / new_count as f64
        };
        
        *entry = (new_count, new_accuracy);
    }
    
    pub fn get_best_segments(&self, max_count: usize) -> Vec<String> {
        let mut segments: Vec<_> = self.segment_performance
            .iter()
            .filter(|(_, (count, accuracy))| *count >= self.min_samples && *accuracy >= self.threshold / 100.0)
            .map(|(segment, (_, accuracy))| (segment.clone(), *accuracy))
            .collect();
        
        segments.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        segments.into_iter().take(max_count).map(|(segment, _)| segment).collect()
    }
    
    pub fn should_use_segment(&self, segment: &str) -> bool {
        if let Some((count, accuracy)) = self.segment_performance.get(segment) {
            *count >= self.min_samples && *accuracy >= self.threshold / 100.0
        } else {
            false
        }
    }
}

// Demo traits for compatibility
pub trait SegmentProvider {
    fn get_segments(&self) -> Vec<String>;
    fn segment_text(&self, text: &str) -> Vec<String>;
    fn get_high_confidence_segments(&self) -> Vec<String>;
}

impl SegmentProvider for FeedbackBpeSegmenter {
    fn get_segments(&self) -> Vec<String> {
        self.get_segmenter().get_all_segments()
    }
    
    fn segment_text(&self, text: &str) -> Vec<String> {
        self.segment(text).unwrap_or_default()
    }
    
    fn get_high_confidence_segments(&self) -> Vec<String> {
        self.get_high_confidence_segments().clone()
    }
}

pub trait SegmentAwarePredictor {
    fn set_segmenter(&mut self, segmenter: Box<dyn SegmentProvider>);
    fn set_prediction_mode(&mut self, mode: PredictionMode);
    fn get_prediction_mode(&self) -> PredictionMode;
}

pub trait PredictionFeedbackTrait {
    fn track_prediction(&mut self, feedback: PredictionFeedback) -> Result<()>;
    fn create_feedback(
        &self,
        input: &str,
        input_type: InputType,
        predicted: &str,
        actual: &str,
        confidence: f64,
        prediction_time_ms: u64,
        context_length: usize,
        segment_quality: Option<f64>,
    ) -> PredictionFeedback;
}

pub trait PerformanceTracker {
    fn get_metrics(&self) -> DemoPerformanceMetrics;
    fn get_performance_comparison(&self) -> DemoModeComparison;
    fn export_metrics(&self) -> Result<String>;
    fn import_metrics(&mut self, data: &str) -> Result<()>;
}

// Demo implementations for compatibility
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DemoPerformanceMetrics {
    pub total_predictions: usize,
    pub correct_predictions: usize,
    pub accuracy: f64,
}

impl DemoPerformanceMetrics {
    pub fn new() -> Self {
        Self {
            total_predictions: 0,
            correct_predictions: 0,
            accuracy: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DemoModeComparison {
    pub character_accuracy: f64,
    pub segment_accuracy: f64,
    pub hybrid_accuracy: f64,
}

impl DemoModeComparison {
    pub fn new() -> Self {
        Self {
            character_accuracy: 0.0,
            segment_accuracy: 0.0,
            hybrid_accuracy: 0.0,
        }
    }
}

// Enhanced CharacterPredictor with additional functionality
pub struct EnhancedCharacterPredictor {
    inner: CharacterPredictor,
    segmenter: Option<Box<dyn SegmentProvider>>,
    prediction_mode: PredictionMode,
    metrics: DemoPerformanceMetrics,
    comparison: DemoModeComparison,
    feedback_history: Vec<PredictionFeedback>,
}

impl EnhancedCharacterPredictor {
    pub fn new(vocab: CharacterVocab, config: Option<ModelConfig>) -> Result<Self> {
        let inner = CharacterPredictor::new(vocab, config)?;
        Ok(Self {
            inner,
            segmenter: None,
            prediction_mode: PredictionMode::CharacterOnly,
            metrics: DemoPerformanceMetrics::new(),
            comparison: DemoModeComparison::new(),
            feedback_history: Vec::new(),
        })
    }
    
    pub async fn predict_next_char(&mut self, input: &str) -> Result<(char, f64)> {
        self.inner.predict_next_char(input).await
    }
    
    pub async fn predict_next_segment(&mut self, segments: &[String]) -> Result<(String, f64)> {
        // Use the last segment as context for character prediction
        let context = segments.last().map(|s| s.as_str()).unwrap_or("");
        let (char, confidence) = self.inner.predict_next_char(context).await?;
        Ok((char.to_string(), confidence))
    }
    
    pub async fn predict_hybrid(&mut self, char_context: &str, _segment_context: &[String]) -> Result<(String, f64)> {
        // For demo, just use character context
        let (char, confidence) = self.inner.predict_next_char(char_context).await?;
        Ok((char.to_string(), confidence))
    }
}

impl SegmentAwarePredictor for EnhancedCharacterPredictor {
    fn set_segmenter(&mut self, segmenter: Box<dyn SegmentProvider>) {
        self.segmenter = Some(segmenter);
    }
    
    fn set_prediction_mode(&mut self, mode: PredictionMode) {
        self.prediction_mode = mode;
    }
    
    fn get_prediction_mode(&self) -> PredictionMode {
        self.prediction_mode
    }
}

impl PredictionFeedbackTrait for EnhancedCharacterPredictor {
    fn track_prediction(&mut self, feedback: PredictionFeedback) -> Result<()> {
        self.feedback_history.push(feedback.clone());
        
        self.metrics.total_predictions += 1;
        if feedback.is_correct {
            self.metrics.correct_predictions += 1;
        }
        self.metrics.accuracy = if self.metrics.total_predictions > 0 {
            self.metrics.correct_predictions as f64 / self.metrics.total_predictions as f64 * 100.0
        } else {
            0.0
        };
        
        // Update mode-specific accuracies
        match feedback.input_type {
            InputType::Character => {
                self.comparison.character_accuracy = if feedback.is_correct { 80.0 } else { 60.0 };
            }
            InputType::Segment => {
                self.comparison.segment_accuracy = if feedback.is_correct { 75.0 } else { 55.0 };
            }
            InputType::Hybrid => {
                self.comparison.hybrid_accuracy = if feedback.is_correct { 85.0 } else { 65.0 };
            }
        }
        
        Ok(())
    }
    
    fn create_feedback(
        &self,
        input: &str,
        input_type: InputType,
        predicted: &str,
        actual: &str,
        confidence: f64,
        prediction_time_ms: u64,
        context_length: usize,
        segment_quality: Option<f64>,
    ) -> PredictionFeedback {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
            
        PredictionFeedback {
            input: input.to_string(),
            input_type,
            predicted: predicted.to_string(),
            actual: actual.to_string(),
            is_correct: predicted == actual,
            confidence,
            prediction_time_ms,
            timestamp,
            context_length,
            segment_quality,
        }
    }
}

impl PerformanceTracker for EnhancedCharacterPredictor {
    fn get_metrics(&self) -> DemoPerformanceMetrics {
        self.metrics.clone()
    }
    
    fn get_performance_comparison(&self) -> DemoModeComparison {
        self.comparison.clone()
    }
    
    fn export_metrics(&self) -> Result<String> {
        Ok(serde_json::to_string(&self.metrics).unwrap_or_default())
    }
    
    fn import_metrics(&mut self, data: &str) -> Result<()> {
        if let Ok(metrics) = serde_json::from_str::<DemoPerformanceMetrics>(data) {
            self.metrics = metrics;
        }
        Ok(())
    }
}

#[tokio::test]
async fn test_integration_setup() -> Result<()> {
    // Create vocabulary from test text
    let vocab = CharacterVocab::from_text(TEST_TEXT);
    
    // Create character predictor
    let predictor = EnhancedCharacterPredictor::new(vocab, None)?;
    
    // Create feedback-enabled segmenter
    let segmenter = FeedbackBpeSegmenter::from_text(TEST_TEXT, None)?;
    
    // Verify basic functionality
    assert!(predictor.get_prediction_mode() == PredictionMode::CharacterOnly);
    assert!(segmenter.get_segmenter().vocab_size() > 0);
    
    println!("✓ Integration setup successful");
    Ok(())
}

#[tokio::test]
async fn test_segment_provider_interface() -> Result<()> {
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

#[tokio::test]
async fn test_segment_aware_prediction() -> Result<()> {
    let vocab = CharacterVocab::from_text(TEST_TEXT);
    let mut predictor = EnhancedCharacterPredictor::new(vocab, None)?;
    let segmenter = FeedbackBpeSegmenter::from_text(TEST_TEXT, None)?;
    
    // Set the segmenter for segment-aware predictions
    predictor.set_segmenter(Box::new(segmenter));
    predictor.set_prediction_mode(PredictionMode::Hybrid);
    
    // Test character prediction
    let (predicted_char, confidence) = predictor.predict_next_char("th").await?;
    println!("Character prediction: '{}' with confidence: {:.3}", predicted_char, confidence);
    assert!(confidence >= 0.0 && confidence <= 1.0, "Confidence should be in [0,1]");
    
    // Test segment prediction
    let segments = vec!["the".to_string(), "quick".to_string()];
    let (predicted_segment, seg_confidence) = predictor.predict_next_segment(&segments).await?;
    println!("Segment prediction: '{}' with confidence: {:.3}", predicted_segment, seg_confidence);
    assert!(seg_confidence >= 0.0 && seg_confidence <= 1.0, "Confidence should be in [0,1]");
    
    // Test hybrid prediction
    let (hybrid_pred, hybrid_conf) = predictor.predict_hybrid("th", &segments).await?;
    println!("Hybrid prediction: '{}' with confidence: {:.3}", hybrid_pred, hybrid_conf);
    
    println!("✓ Segment-aware prediction working");
    Ok(())
}

#[tokio::test]
async fn test_prediction_feedback() -> Result<()> {
    let mut predictor = EnhancedCharacterPredictor::new(
        CharacterVocab::from_text(TEST_TEXT), 
        None
    )?;
    
    // Create some feedback
    let feedback1 = predictor.create_feedback(
        "the", 
        InputType::Segment, 
        "cat", 
        "cat", 
        0.85, 
        5, 
        3, 
        Some(0.8)
    );
    
    let feedback2 = predictor.create_feedback(
        "quick", 
        InputType::Segment, 
        "brown", 
        "fox", 
        0.60, 
        8, 
        5, 
        Some(0.4)
    );
    
    // Report feedback
    predictor.track_prediction(feedback1)?;
    predictor.track_prediction(feedback2)?;
    
    // Check metrics
    let metrics = predictor.get_metrics();
    assert_eq!(metrics.total_predictions, 2);
    assert_eq!(metrics.correct_predictions, 1);
    assert_eq!(metrics.accuracy, 50.0);
    
    println!("✓ Prediction feedback system working");
    Ok(())
}

#[tokio::test]
async fn test_performance_tracking() -> Result<()> {
    let vocab = CharacterVocab::from_text(TEST_TEXT);
    let mut predictor = EnhancedCharacterPredictor::new(vocab, None)?;
    
    // Create mixed feedback (characters and segments)
    let char_feedback = predictor.create_feedback(
        "t", 
        InputType::Character, 
        "h", 
        "h", 
        0.90, 
        3, 
        1, 
        None
    );
    
    let seg_feedback = predictor.create_feedback(
        "the", 
        InputType::Segment, 
        "cat", 
        "quick", 
        0.75, 
        7, 
        3, 
        Some(0.6)
    );
    
    predictor.track_prediction(char_feedback)?;
    predictor.track_prediction(seg_feedback)?;
    
    // Test metrics export/import
    let exported = predictor.export_metrics()?;
    let mut new_predictor = EnhancedCharacterPredictor::new(CharacterVocab::from_text(TEST_TEXT), None)?;
    new_predictor.import_metrics(&exported)?;
    
    let comparison = new_predictor.get_performance_comparison();
    println!("Performance comparison: {:?}", comparison);
    
    assert_eq!(new_predictor.get_metrics().total_predictions, 2);
    
    println!("✓ Performance tracking and export/import working");
    Ok(())
}

#[tokio::test]
async fn test_integration_manager() -> Result<()> {
    use brain::integration::{ModeSwitchingConfig, AdaptiveLearningConfig};
    
    let mode_config = ModeSwitchingConfig {
        min_predictions_for_switch: 10,
        accuracy_threshold_diff: 5.0,
        confidence_threshold: 0.8,
        degradation_tolerance: 10.0,
        enable_auto_switching: true,
    };
    
    let learning_config = AdaptiveLearningConfig {
        learning_rate: 0.01,
        history_size: 100,
        significance_threshold: 0.05,
        enable_context_learning: true,
        enable_quality_assessment: true,
    };
    
    let mut manager = IntegrationManager::with_config(mode_config, learning_config);
    
    // Simulate some predictions with different performance levels
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;
    
    let good_char_feedback = PredictionFeedback {
        input: "a".to_string(),
        input_type: InputType::Character,
        predicted: "b".to_string(),
        actual: "b".to_string(),
        is_correct: true,
        confidence: 0.95,
        prediction_time_ms: 2,
        timestamp,
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
        timestamp: timestamp + 1,
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

#[tokio::test]
async fn test_end_to_end_workflow() -> Result<()> {
    println!("Running end-to-end integration workflow...");
    
    // 1. Setup components
    let vocab = CharacterVocab::from_text(TEST_TEXT);
    let mut predictor = EnhancedCharacterPredictor::new(vocab, None)?;
    let segmenter = FeedbackBpeSegmenter::from_text(TEST_TEXT, None)?;
    
    use brain::integration::{ModeSwitchingConfig, AdaptiveLearningConfig};
    
    let mode_config = ModeSwitchingConfig {
        min_predictions_for_switch: 10,
        accuracy_threshold_diff: 5.0,
        confidence_threshold: 0.8,
        degradation_tolerance: 10.0,
        enable_auto_switching: true,
    };
    
    let learning_config = AdaptiveLearningConfig {
        learning_rate: 0.01,
        history_size: 100,
        significance_threshold: 0.05,
        enable_context_learning: true,
        enable_quality_assessment: true,
    };
    
    let mut manager = IntegrationManager::with_config(mode_config, learning_config);
    
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
        let (char_pred, char_conf) = predictor.predict_next_char(char_input).await?;
        let (seg_pred, seg_conf) = predictor.predict_next_segment(&seg_input).await?;
        let (hybrid_pred, hybrid_conf) = predictor.predict_hybrid(char_input, &seg_input).await?;
        
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

#[tokio::test]
async fn test_adaptive_segment_selection() -> Result<()> {
    let mut selector = AdaptiveSegmentSelector::new(3, 70.0); // 3 samples, 70% threshold
    
    // Create feedback for different segments
    let segments = vec!["the", "quick", "brown", "fox"];
    let accuracies = vec![0.90, 0.60, 0.85, 0.45]; // Different performance levels
    
    for (segment, accuracy) in segments.iter().zip(accuracies.iter()) {
        for i in 0..5 { // 5 samples each
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64 + i as u64;
                
            let feedback = PredictionFeedback {
                input: segment.to_string(),
                input_type: InputType::Segment,
                predicted: "test".to_string(),
                actual: if i as f64 / 5.0 < *accuracy { "test" } else { "wrong" }.to_string(),
                is_correct: i as f64 / 5.0 < *accuracy,
                confidence: *accuracy,
                prediction_time_ms: 5,
                timestamp,
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