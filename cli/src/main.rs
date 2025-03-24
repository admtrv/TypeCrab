/*
 * cli/src/main.rs
 */

use clap::{Parser, ArgGroup};
use core::{TestConfig, GameMode, generate_content};

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
    ArgGroup::new("language_group")
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
    let opt = Opt::parse();

    // defining mode
    let mode = match (opt.words, opt.quote, opt.zen) {
        (true, false, false) => GameMode::Words,
        (false, true, false) => GameMode::Quote,
        (false, false, true) => GameMode::Zen,
        (false, false, false) => {
            GameMode::Words // default mode
        }
        _ => {
            eprintln!("warning: invalid mode combination, defaulting to 'words' mode");
            GameMode::Words
        }
    };

    let config = TestConfig {
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

    match generate_content(&config) {
        Ok(output) => {
            println!("--- CLI ---");
            println!("configuration: {:?}", config);
            if let Some(warning) = output.warning {
                println!("{}", warning);
            }
            println!("test lines:");
            for (i, line) in output.lines.iter().enumerate() {
                println!("  {:>3}. {}", i + 1, line);
            }
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}
