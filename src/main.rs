// main.rs

use clap::{Parser, ArgGroup};

#[derive(Debug, Parser)]
#[command(about, version)]
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
    // program info

    /// List installed languages
    #[arg(long)]
    list_languages: bool,

    // game modes

    /// Enable words mode [default]
    #[arg(short, long, default_value_t = true)]
    words: bool,

    /// Enable quote mode
    #[arg(short,long)]
    quote: bool,

    /// Enable zen mode
    #[arg(short,long)]
    zen: bool,

    // configuration

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
    #[arg(short, long, value_name = "lang", default_value = "eng")]
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

    // arguments print
    println!("Parsed arguments: {:#?}", opt);
}