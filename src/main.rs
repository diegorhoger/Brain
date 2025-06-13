//! Brain CLI - Character Ingestion Engine Demo
//! 
//! This binary demonstrates the character-level predictor functionality

use brain::character_ingestion::{CharacterPredictor, ModelConfig, CharacterVocab};
use brain::Result;
use clap::{Arg, Command};
use log::info;
use std::fs;

fn main() -> Result<()> {
    env_logger::init();
    
    let matches = Command::new("brain-cli")
        .version("0.1.0")
        .about("Brain Character Ingestion Engine")
        .subcommand(
            Command::new("train")
                .about("Train the character predictor")
                .arg(Arg::new("input")
                    .short('i')
                    .long("input")
                    .value_name("FILE")
                    .help("Training text file")
                    .required(true))
                .arg(Arg::new("output")
                    .short('o')
                    .long("output")
                    .value_name("FILE")
                    .help("Output model file")
                    .default_value("model.json"))
                .arg(Arg::new("epochs")
                    .short('e')
                    .long("epochs")
                    .value_name("NUM")
                    .help("Number of training epochs")
                    .default_value("10"))
                .arg(Arg::new("batch_size")
                    .short('b')
                    .long("batch-size")
                    .value_name("NUM")
                    .help("Batch size for training")
                    .default_value("32"))
                .arg(Arg::new("sequence_length")
                    .short('s')
                    .long("sequence-length")
                    .value_name("NUM")
                    .help("Sequence length for training")
                    .default_value("32"))
        )
        .subcommand(
            Command::new("generate")
                .about("Generate text using trained model")
                .arg(Arg::new("model")
                    .short('m')
                    .long("model")
                    .value_name("FILE")
                    .help("Trained model file")
                    .default_value("model.json"))
                .arg(Arg::new("prefix")
                    .short('p')
                    .long("prefix")
                    .value_name("TEXT")
                    .help("Text prefix to start generation")
                    .default_value("The"))
                .arg(Arg::new("length")
                    .short('l')
                    .long("length")
                    .value_name("NUM")
                    .help("Length of text to generate")
                    .default_value("100"))
                .arg(Arg::new("temperature")
                    .short('t')
                    .long("temperature")
                    .value_name("FLOAT")
                    .help("Temperature for sampling (higher = more random)")
                    .default_value("1.0"))
        )
        .get_matches();

    match matches.subcommand() {
        Some(("train", sub_matches)) => {
            let input_file = sub_matches.get_one::<String>("input").unwrap();
            let output_file = sub_matches.get_one::<String>("output").unwrap();
            let epochs: usize = sub_matches.get_one::<String>("epochs").unwrap().parse()?;
            let batch_size: usize = sub_matches.get_one::<String>("batch_size").unwrap().parse()?;
            let sequence_length: usize = sub_matches.get_one::<String>("sequence_length").unwrap().parse()?;

            info!("Loading training data from: {}", input_file);
            let text = fs::read_to_string(input_file)?;
            
            info!("Creating vocabulary...");
            let vocab = CharacterVocab::from_text(&text);
            info!("Vocabulary size: {}", vocab.vocab_size());

            info!("Creating model...");
            let mut config = ModelConfig::default();
            config.sequence_length = sequence_length;
            let mut model = CharacterPredictor::new(vocab, Some(config))?;

            info!("Training model...");
            let losses = model.train_sequence(&text, batch_size, epochs)?;
            
            info!("Final loss: {:.4}", losses.last().unwrap_or(&0.0));
            
            info!("Saving model to: {}", output_file);
            model.save(output_file)?;
            
            info!("Training completed!");
        }
        Some(("generate", sub_matches)) => {
            let model_file = sub_matches.get_one::<String>("model").unwrap();
            let prefix = sub_matches.get_one::<String>("prefix").unwrap();
            let length: usize = sub_matches.get_one::<String>("length").unwrap().parse()?;
            let temperature: f64 = sub_matches.get_one::<String>("temperature").unwrap().parse()?;

            info!("Loading model from: {}", model_file);
            let model = CharacterPredictor::load(model_file)?;

            info!("Generating text...");
            let generated = model.generate(prefix, length, temperature)?;
            
            println!("\nGenerated text:");
            println!("{}", generated);
        }
        _ => {
            println!("Use --help for usage information");
        }
    }

    Ok(())
} 