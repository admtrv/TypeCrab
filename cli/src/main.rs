/*
 * cli/src/main.rs
 */

mod tui;
mod logic;

use std::{
    io,
    time::Duration,
};
use std::time::Instant;
use clap::{
    ArgGroup,
    Parser
};
use crossterm::{
    event,
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen
    }
};
use crossterm::event::{
    Event,
    KeyCode,
    KeyEventKind,
    KeyModifiers
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal
};
use core::{
    generate_content,
    validate_config,
    list_languages,
    GameMode,
    Config,
    Level
};

use tui::TestView;
use tui::ResultView;

use logic::{
    Test,
    RawData
};

const STYLE_ERROR: &str = "\x1b[1;31merror:\x1b[0m";        // 1;31 = bold red, 0m = reset
const STYLE_WARNING: &str = "\x1b[1;33mwarning:\x1b[0m";    // bold yellow
const STYLE_INFO: &str = "\x1b[1;32minfo:\x1b[0m";          // bold green


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

    /// Disable backtracking of completed words
    #[arg(long = "strict", action = clap::ArgAction::SetTrue)]
    strict: bool,

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

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let opt = Opt::parse();

    // if language listing
    if opt.list {
        // api languages listing
        let listing_response = list_languages();

        if let Some((Level::Error, msg)) = &listing_response.message {
            eprintln!("{STYLE_ERROR} {msg}");
            std::process::exit(1);
        } else {
            for lang in &listing_response.payload {
                println!("{lang}");
            }
            return Ok(());
        }
    }


    // initial config
    let mode = if opt.quote {
        GameMode::Quote
    } else if opt.zen {
        GameMode::Zen
    } else {
        GameMode::Words
    };

    let initial_config = Config {
        mode,
        language: opt.language,
        file: opt.file,
        word_count: opt.count,
        time_limit: opt.time,
        punctuation: opt.punctuation,
        numbers: opt.numbers,
        backtrack: !opt.strict,
        death: opt.death,
    };

    // api config validation
    let config_response = validate_config(initial_config);

    if let Some((Level::Error, msg)) = &config_response.message {
        eprintln!("{STYLE_ERROR} {msg}");
        std::process::exit(1);
    }

    let config = config_response.payload;

    // api words generation
    let generation_response = generate_content(&config);

    if let Some((Level::Error, msg)) = &generation_response.message {
        eprintln!("{STYLE_ERROR} {msg}");
        std::process::exit(1);
    }

    let words = &generation_response.payload;

    // new test
    let mut test = Test::new(words.clone(), &config);

    // entering tui
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let status_message = config_response.message.clone().or(generation_response.message.clone());
    let mut show_message = status_message.clone();
    let message_start = Instant::now();

    // main test cycle
    loop {
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {   // processing entered key
                test.handle_key(key);

                // hot keys
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => break,
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            break
                        }
                        _ => {}
                    }
                }
            }
        }

        if test.complete {
            break;
        }

        // status message
        if let Some(_) = show_message {
            if message_start.elapsed().as_secs() >= 3 {
                show_message = None;
            }
        }

        // rendering current state
        terminal.draw(|f| {
            let size = f.area();
            let view = TestView {
                test: &test,
                status: show_message.clone(),
            };
            f.render_widget(view, size);
        })?;
    }

    // returning from tui
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    // getting raw test data
    let raw_results = RawData::from(&test);

    // api result generation from raw data

    Ok(())
}