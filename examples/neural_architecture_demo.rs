//! Neural Architecture Demo - Task 3.1
//! 
//! This example demonstrates the advanced neural architecture features including:
//! - Self-attention and multi-head attention mechanisms
//! - Transformer encoder architecture with layer normalization
//! - Post-transformer developmental AI with adaptive growth
//! - Integration with existing character prediction and segmentation

use brain::neural_architecture::{
    SelfAttention, AttentionConfig, TransformerPredictor, TransformerConfig,
    DevelopmentalPredictor, GrowthConfig, SelfAttentionService, TransformerPredictorService,
    DevelopmentalPredictorService
};
use brain::character_ingestion::{CharacterVocab, CharacterPredictor, ModelConfig};
use brain::Result;
use nalgebra::DMatrix;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🧠 Brain Neural Architecture - Advanced Features Demo (Task 3.1)");
    println!("================================================================");
    
    // Ensure data and logs directories exist
    std::fs::create_dir_all("data").map_err(|e| brain::BrainError::Io { source: e })?;
    std::fs::create_dir_all("logs").map_err(|e| brain::BrainError::Io { source: e })?;
    std::fs::create_dir_all("temp").map_err(|e| brain::BrainError::Io { source: e })?;
    
    // 1. Demonstrate Self-Attention Mechanism
    println!("\n⚡ Self-Attention Mechanism Demo");
    println!("===============================");
    
    let attention_config = AttentionConfig {
        model_dim: 64,
        num_heads: 4,
        head_dim: 16,
        dropout_rate: 0.1,
        use_scaling: true,
    };
    
    let mut attention = SelfAttention::new(attention_config.clone())?;
    println!("✅ Self-attention layer created:");
    println!("   - Model dimension: {}", attention_config.model_dim);
    println!("   - Number of heads: {}", attention_config.num_heads);
    println!("   - Head dimension: {}", attention_config.head_dim);
    
    // Create sample input sequences
    let seq_len = 8;
    let input = DMatrix::from_fn(seq_len, attention_config.model_dim, |i, j| {
        // Create a pattern that changes over positions
        ((i as f64 + 1.0) * 0.1 + (j as f64) * 0.01).sin()
    });
    
    println!("\n📊 Processing sequence of length {}...", seq_len);
    let attention_output = attention.forward(&input).await?;
    println!("✅ Self-attention forward pass completed");
    println!("   - Input shape: {}x{}", input.nrows(), input.ncols());
    println!("   - Output shape: {}x{}", attention_output.nrows(), attention_output.ncols());
    
    // Analyze attention weights
    if let Some(weights) = attention.get_attention_weights().await {
        println!("🔍 Attention weights analysis:");
        println!("   - Attention matrix shape: {}x{}", weights.nrows(), weights.ncols());
        
        // Find strongest attention connections
        let mut max_attention = 0.0;
        let mut max_pos = (0, 0);
        for i in 0..weights.nrows() {
            for j in 0..weights.ncols() {
                if weights[(i, j)] > max_attention {
                    max_attention = weights[(i, j)];
                    max_pos = (i, j);
                }
            }
        }
        println!("   - Strongest attention: {:.4} at position ({}, {})", max_attention, max_pos.0, max_pos.1);
    }
    
    // 2. Demonstrate Transformer Architecture
    println!("\n🏗️ Transformer Architecture Demo");
    println!("===============================");
    
    let vocab_size = 100;
    let transformer_config = TransformerConfig {
        model_dim: 128,
        num_layers: 3,
        num_heads: 4,
        ff_hidden_dim: 256,
        max_seq_len: 32,
        dropout_rate: 0.1,
    };
    
    let mut transformer = TransformerPredictor::new(vocab_size, Some(transformer_config.clone()))?;
    println!("✅ Transformer predictor created:");
    println!("   - Vocabulary size: {}", vocab_size);
    println!("   - Number of layers: {}", transformer_config.num_layers);
    println!("   - Model dimension: {}", transformer_config.model_dim);
    println!("   - Feed-forward hidden: {}", transformer_config.ff_hidden_dim);
    
    // Test transformer prediction
    let input_sequence = vec![1, 15, 23, 42, 7, 89, 34];
    println!("\n🔮 Testing transformer prediction...");
    println!("Input sequence: {:?}", input_sequence);
    
    let predictions = transformer.predict_next(&input_sequence).await?;
    println!("✅ Prediction completed, output dimension: {}", predictions.len());
    
    // Analyze predictions
    let mut top_predictions = Vec::new();
    for (i, &prob) in predictions.iter().enumerate() {
        top_predictions.push((i, prob));
    }
    top_predictions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    println!("🎯 Top 5 predictions:");
    for (i, (token_id, prob)) in top_predictions.iter().take(5).enumerate() {
        println!("   {}. Token {}: {:.4}", i + 1, token_id, prob);
    }
    
    // Analyze attention maps
    let attention_maps = transformer.get_attention_maps().await;
    println!("\n🗺️ Attention maps analysis:");
    for (layer_idx, attention_map) in attention_maps.iter().enumerate() {
        if let Some(map) = attention_map {
            println!("   - Layer {}: {}x{} attention matrix", layer_idx + 1, map.nrows(), map.ncols());
        } else {
            println!("   - Layer {}: No attention weights available", layer_idx + 1);
        }
    }
    
    // 3. Demonstrate Developmental AI
    println!("\n🌱 Developmental AI Demo");
    println!("========================");
    
    let growth_config = GrowthConfig {
        initial_scale: 0.3,
        growth_rate: 1.8,
        max_scale: 3.0,
        complexity_threshold: 0.7,
        enable_meta_learning: true,
    };
    
    let mut dev_predictor = DevelopmentalPredictor::new(vocab_size, Some(transformer_config), Some(growth_config.clone()))?;
    println!("✅ Developmental predictor created:");
    println!("   - Initial stage: {:?}", dev_predictor.get_developmental_stage().await);
    println!("   - Initial scale: {:.1}x", growth_config.initial_scale);
    println!("   - Growth rate: {:.1}x", growth_config.growth_rate);
    println!("   - Meta-learning: {}", growth_config.enable_meta_learning);
    
    // Simulate developmental learning
    println!("\n📈 Simulating developmental learning sessions...");
    
    let learning_contexts = vec![
        "Character sequence learning",
        "Pattern recognition training", 
        "Context understanding",
        "Complex reasoning",
        "Abstract concept formation",
    ];
    
    for (session, context) in learning_contexts.iter().enumerate() {
        println!("\n--- Learning Session {} ---", session + 1);
        println!("Context: {}", context);
        
        // Generate different input patterns for each session
        let input_ids: Vec<usize> = (0..=session).map(|i| i * 7 % vocab_size).collect();
        let output = dev_predictor.developmental_forward(&input_ids, context).await?;
        
        println!("   - Input length: {}", input_ids.len());
        println!("   - Developmental stage: {:?}", dev_predictor.get_developmental_stage().await);
        
        let capacity = dev_predictor.get_capacity_metrics().await;
        println!("   - Current complexity: {:.3}", capacity.current_complexity);
        println!("   - Utilization: {:.3}", capacity.utilization);
        println!("   - Growth pressure: {:.3}", capacity.growth_pressure);
        
        // Show top prediction
        let (max_idx, max_prob) = output.iter().enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap();
        println!("   - Top prediction: Token {} (prob: {:.4})", max_idx, max_prob);
    }
    
    // 4. Learning History Analysis
    println!("\n📚 Learning History Analysis");
    println!("============================");
    
    let learning_history = dev_predictor.get_learning_history().await;
    println!("Total learning events: {}", learning_history.len());
    
    for (i, event) in learning_history.iter().enumerate() {
        println!("Event {}: {:?}", i + 1, event.learning_type);
        println!("   - Performance change: {:.3} -> {:.3}", event.performance_before, event.performance_after);
        println!("   - Context: {}", event.context);
        println!("   - Timestamp: {}", event.timestamp);
    }
    
    // 5. Export Developmental State
    println!("\n💾 Exporting Developmental State");
    println!("===============================");
    
    match dev_predictor.export_developmental_state().await {
        Ok(state_json) => {
            println!("✅ Developmental state exported ({} bytes)", state_json.len());
            
            // Save to data directory
            std::fs::write("data/developmental_state.json", &state_json)
                .map_err(|e| brain::BrainError::Io { source: e })?;
            println!("📁 State saved to 'data/developmental_state.json'");
            
            // Show summary
            println!("\n🔍 State Summary:");
            println!("   - Current stage: {:?}", dev_predictor.get_developmental_stage().await);
            println!("   - Learning events: {}", learning_history.len());
            let final_capacity = dev_predictor.get_capacity_metrics().await;
            println!("   - Capacity complexity: {:.3}", final_capacity.current_complexity);
        }
        Err(e) => {
            println!("❌ Failed to export state: {}", e);
        }
    }
    
    // 6. Integration with Character Predictor
    println!("\n🔗 Integration with Character Predictor");
    println!("======================================");
    
    // Create a traditional character predictor for comparison
    let training_text = "the quick brown fox jumps over the lazy dog";
    let vocab = CharacterVocab::from_text(training_text);
    let vocab_size_display = vocab.vocab_size();
    let model_config = ModelConfig {
        vocab_size: vocab.vocab_size(),
        embedding_dim: 32,
        hidden_dim: 64,
        learning_rate: 0.01,
        sequence_length: 8,
    };
    
    let _char_predictor = CharacterPredictor::new(vocab, Some(model_config))?;
    println!("✅ Character predictor created for comparison");
    println!("   - Traditional architecture: Feedforward");
    println!("   - Vocab size: {}", vocab_size_display);
    
    // Compare architectural complexity
    println!("\n⚖️ Architecture Comparison:");
    println!("Traditional Character Predictor:");
    println!("   - Type: Simple feedforward");
    println!("   - Parameters: ~few thousand");
    println!("   - Capabilities: Basic character prediction");
    
    println!("\nAdvanced Transformer Predictor:");
    println!("   - Type: Multi-layer transformer");
    println!("   - Parameters: ~hundreds of thousands");
    println!("   - Capabilities: Attention-based sequence modeling");
    
    println!("\nDevelopmental AI Predictor:");
    println!("   - Type: Adaptive transformer with growth");
    println!("   - Parameters: Dynamic (grows over time)");
    println!("   - Capabilities: Meta-learning, developmental adaptation");
    
    // 7. Performance Insights
    println!("\n🎯 Performance Insights & Next Steps");
    println!("====================================");
    
    println!("✅ Task 3.1 Features Demonstrated:");
    println!("   ⚡ Self-attention mechanisms with multi-head support");
    println!("   🏗️ Transformer encoder architecture");
    println!("   🌱 Developmental AI with adaptive growth");
    println!("   📊 Advanced attention analysis and visualization");
    println!("   🔗 Integration with existing prediction systems");
    println!("   📚 Meta-learning and developmental tracking");
    
    println!("\n🚀 Ready for Task 3.2: Advanced Neural Features");
    println!("   - Cross-attention between character and segment representations");
    println!("   - Encoder-decoder architectures");
    println!("   - Advanced positional encodings");
    println!("   - Neural architecture search capabilities");
    println!("   - Continual learning mechanisms");
    
    println!("\n🎉 Neural Architecture Demo Complete!");
    println!("=====================================");
    println!("The Brain project now features cutting-edge transformer");
    println!("architectures with post-transformer developmental AI!");
    
    Ok(())
} 