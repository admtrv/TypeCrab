/*
 * core/src/generator.rs
 */

use std::{fs, io::{self, BufRead}, path::{Path, PathBuf}, };
use rand::{seq::SliceRandom, rng};
use rand::prelude::IndexedRandom;
use crate::config::{GameMode, TestConfig};

const WORDS_DIR: &str = "assets/words";
const QUOTES_DIR: &str = "assets/quotes";

#[derive(Debug)]
pub enum Level {
    Info,
    Warning,
    Error,
}

pub struct Response {
    pub lines: Vec<String>,
    pub message: Option<(Level, String)>,
}

impl Response {
    pub fn with_info<S: Into<String>>(lines: Vec<String>, msg: S) -> Self {
        Self {
            lines,
            message: Some((Level::Info, msg.into())),
        }
    }

    pub fn with_warning<S: Into<String>>(lines: Vec<String>, msg: S) -> Self {
        Self {
            lines,
            message: Some((Level::Warning, msg.into())),
        }
    }

    pub fn with_error<S: Into<String>>(msg: S) -> Self {
        Self {
            lines: Vec::new(),
            message: Some((Level::Error, msg.into())),
        }
    }

    pub fn plain(lines: Vec<String>) -> Self {
        Self {
            lines,
            message: None,
        }
    }
}


// main api function, generates test content according to config
pub fn generate_content(config: &TestConfig) -> Response {

    // if user specified file, read from it directly
    if let Some(user_file) = &config.file {
        match load_file(user_file) {
            Ok(lines) => {
                return Response::plain(finalize_lines(lines, config.word_count));
            }
            Err(e) => {
                return Response::with_error(format!("invalid file '{}', {}", user_file, e));
            }
        }
    }

    // else depends on mode
    match config.mode {

        // words mode
        GameMode::Words => {
            match load_words(&config.language, config.word_count) {
                Ok(lines) => Response::plain(lines),
                Err(e) => Response::with_error(e),
            }
        }

        // quote mode
        GameMode::Quote => {
            if let Ok(lines) = load_quote(&config.language) {
                if !lines.is_empty() {
                    return Response::plain(lines);
                }
            }

            // fallback to words if no quote files
            load_words(&config.language, config.word_count)
                .map(|lines| Response::with_warning(lines, format!("no quotes found for '{}', falling back to 'words' mode", config.language)))
                .unwrap_or_else(|e| Response::with_error(format!("quote fallback failed, {}", e)))
        }

        // zen mode
        GameMode::Zen => Response::with_info(Vec::new(), "zen mode doesn't require words"),
    }
}

fn load_words(lang: &str, count: usize) -> Result<Vec<String>, String> {

    let path = Path::new(WORDS_DIR).join(format!("{}.txt", lang));
    if !path.exists() {
        return Err(format!("word list '{}' not found", path.display()));
    }

    let lines = load_file(&path)
        .map_err(|e| format!("cannot read word list '{}', {}", path.display(), e))?;

    Ok(finalize_lines(lines, count))
}

fn load_quote(lang: &str) -> Result<Vec<String>, String> {

    let dir = Path::new(QUOTES_DIR).join(lang);

    if !dir.is_dir() {
        return Err(format!("quotes directory for '{}' not found", lang));
    }

    let files: Vec<PathBuf> = fs::read_dir(&dir)
        .map_err(|e| format!("cannot read directory '{}', {}", dir.display(), e))?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_file())
        .collect();

    if files.is_empty() {
        return Err(format!("no quote files in '{}'", dir.display()));
    }

    let mut rng = rng();
    let file = files
        .choose(&mut rng)
        .expect("non-empty file list ensured by check above");

    load_file(file).map_err(|e| format!("failed to read quote '{}', {}", file.display(), e))
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
