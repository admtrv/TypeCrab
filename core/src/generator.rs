/*
 * core/src/generator.rs
 */

use std::{
    fs,
    io::{
        self,
        BufRead
    },
    path::Path
};
use rand::{
    prelude::IndexedRandom,
    rng,
    Rng,
};

use crate::{
    config::{
        GameMode,
        Config,
        Language
    },
    response::Response,
    WORDS_DIR,
    QUOTES_DIR,
};


const PUNCTS: &[&str] = &[".", ",", "!", "?", ":", ";"];
const NUMBER_RANGE: std::ops::RangeInclusive<u32> = 1..=9999;

const PUNCT_PROBABILITY: f64 = 0.2;
const NUMBER_PROBABILITY: f64 = 0.2;

pub type GeneratorResponse = Response<Vec<String>>;

// api function, that generates test content according to config
pub fn generate_content(config: &Config) -> GeneratorResponse {

    // if user specified file, read from it directly
    if let Some(user_file) = &config.file {
        let lines = match load_file(user_file) {
            Ok(lines) => lines,
            Err(e) => { return GeneratorResponse::with_error(Vec::new(), format!("invalid file '{}', {}", user_file, e), ); }
        };

        return match config.mode {
            GameMode::Words => GeneratorResponse::plain(finalize_lines(lines, config)),
            GameMode::Quote => GeneratorResponse::plain(split_lines(lines)),
            GameMode::Zen => GeneratorResponse::with_error(Vec::new(), "zen mode should not receive a file".to_string())
        };
    }

    // else integrated resources
    match config.mode {

        // words mode
        GameMode::Words => {
            if let Language::Words(lang) = config.language {
                match load_words(lang.as_str()) {
                    Ok(lines) => GeneratorResponse::plain(finalize_lines(lines, config)),
                    Err(e) => GeneratorResponse::with_error(Vec::new(), e),
                }
            } else {
                GeneratorResponse::with_error(Vec::new(), "invalid language for words mode".to_string())
            }
        }

        // quote mode
        GameMode::Quote => {
            if let Language::Quotes(lang) = config.language {
                match load_quote(lang.as_str()) {
                    Ok(lines) => GeneratorResponse::plain(split_lines(lines)),
                    Err(e) => GeneratorResponse::with_error(Vec::new(), e),
                }
            } else {
                GeneratorResponse::with_error(Vec::new(), "invalid language for quote mode".to_string())
            }
        }

        // zen mode
        GameMode::Zen => GeneratorResponse::with_info(Vec::new(), "zen mode doesn't require words"),
    }
}

fn load_words(lang: &str) -> Result<Vec<String>, String> {
    let path = Path::new(WORDS_DIR).join(format!("{}.txt", lang));
    load_file(&path).map_err(|e| format!("cannot read words '{}', {}", path.display(), e))
}

fn load_quote(lang: &str) -> Result<Vec<String>, String> {
    let dir = Path::new(QUOTES_DIR).join(lang);
    let files = fs::read_dir(&dir)
        .map_err(|e| format!("cannot read directory '{}': {}", dir.display(), e))?
        .flatten()
        .map(|e| e.path())
        .collect::<Vec<_>>();

    let mut rng = rng();
    let file = files
        .choose(&mut rng)
        .expect("non-empty file list ensured by check above");

    let lines = load_file(file)
        .map_err(|e| format!("failed to read quote '{}', {}", file.display(), e))?;
    Ok(lines)
}

fn finalize_lines(lines: Vec<String>, config: &Config) -> Vec<String> {
    let mut rng = rng();

    let base_words: Vec<String> = lines
        .into_iter()
        .flat_map(|l| l.split_whitespace().map(|s| s.to_string()).collect::<Vec<_>>())
        .collect();

    let mut result = Vec::new();

    for _ in 0..config.word_count {
        let word = base_words.choose(&mut rng).cloned().unwrap_or_default();

        let mut final_word = word.clone();

        // punctuation
        if config.punctuation && rng.random_bool(PUNCT_PROBABILITY) {
            final_word.push_str(PUNCTS.choose(&mut rng).unwrap());
        }

        result.push(final_word);

        // numbers
        if config.numbers && rng.random_bool(NUMBER_PROBABILITY) {
            let number = rng.random_range(NUMBER_RANGE);
            result.push(number.to_string());
        }
    }

    result
}

fn split_lines(lines: Vec<String>) -> Vec<String> {
    let len = lines.len();
    let mut result = Vec::new();

    for (i, line) in lines.into_iter().enumerate() {
        let mut words: Vec<String> = line
            .split_whitespace()
            .map(|w| w.to_string())
            .collect();

        if !words.is_empty() && i != len - 1 {
            let last = words.len() - 1;
            words[last].push('\n');
        }

        result.extend(words);
    }

    result
}


fn load_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let file = fs::File::open(&path)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}
