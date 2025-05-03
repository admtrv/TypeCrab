/*
 * core/src/config.rs
 */

use crate::{
    response::{
        Level,
        Response
    },
    languages::{WordsLanguages, QuotesLanguages}
};

use serde::{Serialize, Deserialize};

pub type ConfigResponse = Response<Config>;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GameMode {
    Words,
    Quote,
    Zen,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Language {
    Words(WordsLanguages),
    Quotes(QuotesLanguages)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub mode: GameMode,
    pub language: Language,
    pub file: Option<String>,
    pub word_count: usize,
    pub time_limit: Option<u32>,
    pub punctuation: bool,
    pub numbers: bool,
    pub backtrack: bool,
    pub death: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mode: GameMode::Words,
            language: Language::Words(WordsLanguages::En),
            file: None,
            word_count: 25,
            time_limit: None,
            punctuation: false,
            numbers: false,
            backtrack: true,
            death: false,
        }
    }
}

impl Config {
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    pub fn from_json_string(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

// API function that returns modified config if something is wrong with the initial config
pub fn validate_config(mut config: Config) -> ConfigResponse {
    let mut messages: Vec<String> = Vec::new();
    let mut level = Level::Info;

    // word_count / time_limit validation
    if config.word_count == 0 {
        config.word_count = 30;
        messages.push("invalid word count, set to 30".to_string());
        level.escalate(Level::Warning);
    }

    if let Some(0) = config.time_limit {
        config.time_limit = None;
        messages.push("invalid time limit, disabled".to_string());
        level.escalate(Level::Warning);
    }

    // custom file validation
    if let Some(_) = config.file {
        if matches!(config.mode, GameMode::Zen) {
            messages.push("provided custom file, but chosen zen mode".to_string());
            level.escalate(Level::Error);
        }
    }

    // mode-specific validation
    match config.mode {
        GameMode::Words => {
            if !matches!(config.language, Language::Words(_)) {
                messages.push("invalid language for words mode, fallback to 'en'".to_string());
                config.language = Language::Words(WordsLanguages::En);
                level.escalate(Level::Warning);
            }
        }

        GameMode::Quote => {
            if !matches!(config.language, Language::Quotes(_)) {
                messages.push("invalid language for quote mode, fallback to 'words' mode".to_string());
                config.mode = GameMode::Words;
                config.language = Language::Words(WordsLanguages::En);
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

fn build_response(config: Config, notes: Vec<String>, level: Level) -> ConfigResponse {
    if notes.is_empty() {
        return Response::plain(config);
    }

    let joined = notes.join(", ");
    Response {
        payload: config,
        message: Some((level, joined)),
    }
}
