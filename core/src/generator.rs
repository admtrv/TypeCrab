/*
 * core/src/generator.rs
 */

use std::{fs, io::{self, BufRead}, path::Path};
use rand::{seq::SliceRandom, rng};
use rand::prelude::IndexedRandom;

use crate::config::{GameMode, TestConfig};
use crate::response::{Response};

const WORDS_DIR: &str = "resources/words";
const QUOTES_DIR: &str = "resources/quotes";

pub type GeneratorResponse = Response<Vec<String>>;

// api function, generates test content according to config
pub fn generate_content(config: &TestConfig) -> GeneratorResponse {

    // if user specified file, read from it directly
    if let Some(user_file) = &config.file {
        let lines = match load_file(user_file) {
            Ok(lines) => lines,
            Err(e) => { return GeneratorResponse::with_error(Vec::new(), format!("invalid file '{}', {}", user_file, e), ); }
        };

        return match config.mode {
            GameMode::Words => GeneratorResponse::plain(finalize_lines(lines, config.word_count)),
            GameMode::Quote => GeneratorResponse::plain(split_lines(lines)),
            GameMode::Zen => GeneratorResponse::with_error(Vec::new(), "zen mode should not receive a file".to_string())
        };
    }

    // else integrated resources
    match config.mode {

        // words mode
        GameMode::Words => {
            match load_words(&config.language, config.word_count) {
                Ok(lines) => GeneratorResponse::plain(lines),
                Err(e) => GeneratorResponse::with_error(Vec::new(), e),
            }
        }

        // quote mode
        GameMode::Quote => {
            match load_quote(&config.language) {
                Ok(lines) => GeneratorResponse::plain(split_lines(lines)),
                Err(e) => GeneratorResponse::with_error(Vec::new(), e),
            }
        }

        // zen mode
        GameMode::Zen => GeneratorResponse::with_info(Vec::new(), "zen mode doesn't require words"),
    }
}

fn load_words(lang: &str, count: usize) -> Result<Vec<String>, String> {
    let path = Path::new(WORDS_DIR).join(format!("{}.txt", lang));
    let lines = load_file(&path)
        .map_err(|e| format!("cannot read words '{}', {}", path.display(), e))?;
    Ok(finalize_lines(lines, count))
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
fn split_lines(lines: Vec<String>) -> Vec<String> {
    let len = lines.len();
    let mut result = Vec::new();

    for (i, line) in lines.into_iter().enumerate() {
        let mut words: Vec<String> = line.split_whitespace().map(|w| w.to_string()).collect();

        if i != len - 1 {
            words.push("\n".to_string());
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

fn finalize_lines(mut lines: Vec<String>, count: usize) -> Vec<String> {
    let mut rng = rng();
    lines.shuffle(&mut rng);
    lines.truncate(count);
    lines
}
