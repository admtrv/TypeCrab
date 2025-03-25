/*
 * cli/src/main.rs
 */

use clap::{Parser, ArgGroup};
use core::{GameMode, TestConfig, validate_config, Level, generate_content};

#[derive(Debug, Parser)]
#[command(
    name = "TypeCrab",
    about = "A minimalistic, customizable typing test.",
    version
)]
#[command(group(
    ArgGroup::new("mode_group")
        .args(&["words", "quote", "zen"])
        .multiple(false)
))]
#[command(group(
    ArgGroup::new("source")
        .args(&["language", "file"])
        .multiple(false)
))]
struct Opt {
    /// List installed languages
    #[arg(long)]
    list: bool,

    /// Enable words mode [default]
    #[arg(short, long)]
    words: bool,

    /// Enable quote mode
    #[arg(short, long)]
    quote: bool,

    /// Enable zen mode
    #[arg(short, long)]
    zen: bool,

    /// Include punctuation in test text
    #[arg(short, long)]
    punctuation: bool,

    /// Include numbers in test text
    #[arg(short, long)]
    numbers: bool,

    /// Enable backtracking of completed words [default]
    #[arg(long, default_value_t = true)]
    backtrack: bool,

    /// Enable sudden death on first mistake
    #[arg(long)]
    death: bool,

    /// Specify test language
    #[arg(short, long, value_name = "lang", default_value = "en")]
    language: String,

    /// Specify custom test file
    #[arg(long, value_name = "path")]
    file: Option<String>,

    /// Specify word count
    #[arg(short, long, value_name = "n", default_value_t = 25)]
    count: usize,

    /// Specify time limit
    #[arg(short, long, value_name = "sec", default_value_t = 60)]
    time: u32,
}

fn main() {

    // 1. initial configuration
    let opt = Opt::parse();


    let mode = if opt.quote {   // defining mode
        GameMode::Quote
    } else if opt.zen {
        GameMode::Zen
    } else {
        GameMode::Words // explicitly --words or default
    };

    let initial_config = TestConfig {
        mode,
        language: opt.language,
        file: opt.file,
        word_count: opt.count,
        time_limit: opt.time,
        punctuation: opt.punctuation,
        numbers: opt.numbers,
        backtrack: opt.backtrack,
        death: opt.death,
    };

    // 2. configuration validation
    let config_response = validate_config(initial_config);
    let config = config_response.payload;

    println!("--- CLI ---");

    if let Some((level, message)) = &config_response.message {
        match level {
            Level::Info => println!("info: {}", message),
            Level::Warning => println!("warning: {}", message),
            Level::Error => {
                eprintln!("error: {}", message);
                std::process::exit(1);
            }
        }
    }

    println!("configuration: {:?}", config);

    // 3. test generation
    let generation_response = generate_content(&config);

    if let Some((level, message)) = &generation_response.message {
        match level {
            Level::Info => println!("info: {}", message),
            Level::Warning => println!("warning: {}", message),
            Level::Error => {
                eprintln!("error: {}", message);
                std::process::exit(1);
            }
        }
    }

    println!("test lines:");
    for (i, line) in generation_response.payload.iter().enumerate() {
        println!("  {:>3}. {}", i + 1, line);
    }
}
