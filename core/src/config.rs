/*
 * core/src/config.rs
 */

use std::{fs, path::Path};
use crate::response::{Level, Response};

const WORDS_DIR: &str = "resources/words";
const QUOTES_DIR: &str = "resources/quotes";

pub type ConfigResponse = Response<TestConfig>;

#[derive(Debug, Clone, Copy)]
pub enum GameMode {
    Words,
    Quote,
    Zen,
}

#[derive(Debug)]
pub struct TestConfig {
    pub mode: GameMode,
    pub language: String,
    pub file: Option<String>,
    pub word_count: usize,
    pub time_limit: u32,
    pub punctuation: bool,
    pub numbers: bool,
    pub backtrack: bool,
    pub death: bool,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            mode: GameMode::Words,
            language: "en".to_string(),
            file: None,
            word_count: 25,
            time_limit: 60,
            punctuation: false,
            numbers: false,
            backtrack: true,
            death: false,
        }
    }
}

// api function, that returns modified config if something wrong with initial
pub fn validate_config(mut config: TestConfig) -> ConfigResponse {
    let mut messages: Vec<String> = Vec::new();
    let mut level = Level::Info;

    // 1. word_count / time_limit validation
    if config.word_count == 0 {
        config.word_count = 25;
        messages.push("invalid word count, set to 25".to_string());
        level.escalate(Level::Warning);
    }

    if config.time_limit == 0 {
        config.time_limit = 60;
        messages.push("invalid time limit, set to 60".to_string());
        level.escalate(Level::Warning);
    }

    // 2. custom file validation
    if let Some(ref path_str) = config.file {
        if matches!(config.mode, GameMode::Zen) {
            messages.push("provided custom file, but chosen zen mode".to_string());
            level.escalate(Level::Error);
        }

        let path = Path::new(path_str);
        if !path.exists() {
            messages.push(format!("file '{}' doesn't exist", path.display()));
            level.escalate(Level::Error);
        }
    }

    // 3. mode-specific validation
    match config.mode {
        GameMode::Words => {
            let path = Path::new(WORDS_DIR).join(format!("{}.txt", config.language));
            if !path.exists() {
                messages.push(format!("no words found for '{}', fallback to 'en'", config.language));
                config.language = "en".to_string();
                level.escalate(Level::Warning);
            }
        }

        GameMode::Quote => {
            let dir = Path::new(QUOTES_DIR).join(&config.language);
            let has_any = dir.is_dir()
                && fs::read_dir(&dir)
                .map(|mut iter| iter.next().is_some())
                .unwrap_or(false);

            if !has_any {
                messages.push(format!("no quotes found for '{}', fallback to 'words' mode", config.language));
                config.mode = GameMode::Words;
                level.escalate(Level::Warning);
            } else {
                if config.word_count != 25 {
                    config.word_count = 25;
                    messages.push("quote mode ignores word count".to_string());
                    level.escalate(Level::Warning);
                }
                if config.punctuation {
                    config.punctuation = false;
                    messages.push("quote mode ignores punctuation".to_string());
                    level.escalate(Level::Warning);
                }
                if config.numbers {
                    config.numbers = false;
                    messages.push("quote mode ignores numbers".to_string());
                    level.escalate(Level::Warning);
                }
            }
        }

        GameMode::Zen => {
            if config.word_count != 25 {
                config.word_count = 25;
                messages.push("zen mode ignores word count".to_string());
                level.escalate(Level::Warning);
            }
            if config.punctuation {
                config.punctuation = false;
                messages.push("zen mode ignores punctuation".to_string());
                level.escalate(Level::Warning);
            }
            if config.numbers {
                config.numbers = false;
                messages.push("zen mode ignores numbers".to_string());
                level.escalate(Level::Warning);
            }
        }
    }

    build_response(config, messages, level)
}

fn build_response(config: TestConfig, notes: Vec<String>, level: Level) -> ConfigResponse {
    if notes.is_empty() {
        return Response::plain(config);
    }

    let joined = notes.join(", ");
    Response {
        payload: config,
        message: Some((level, joined)),
    }
}
