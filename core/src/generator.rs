/*
 * core/src/generator.rs
 */

use std::{fs, io::{self, BufRead}, path::{Path, PathBuf}, };
use rand::{seq::SliceRandom, rng};
use rand::prelude::IndexedRandom;
use crate::config::{GameMode, TestConfig};

const WORDS_DIR: &str = "assets/words";
const QUOTES_DIR: &str = "assets/quotes";

pub struct GenerationOutput {
    pub lines: Vec<String>,
    pub warning: Option<String>,
}

// main api function, generates test content according to config
pub fn generate_content(config: &TestConfig) -> Result<GenerationOutput, String> {

    // if user specified file, read from it directly
    if let Some(user_file) = &config.file {
        let lines = load_file(user_file)
            .map_err(|e| format!("error: invalid file '{}', {}", user_file, e))?;

        return Ok(GenerationOutput {
            lines: finalize_lines(lines, config.word_count),
            warning: None,
        });
    }

    // else depends on mode
    match config.mode {

        // words mode
        GameMode::Words => {
            let lines = load_words(&config.language, config.word_count)?;
            Ok(GenerationOutput {
                lines,
                warning: None,
            })
        }

        // quote mode
        GameMode::Quote => {
            if let Ok(lines) = load_quote(&config.language) {
                if !lines.is_empty() {
                    return Ok(GenerationOutput {
                        lines,
                        warning: None,
                    });
                }
            }

            // fallback to words if no quote files
            load_words(&config.language, config.word_count)
                .map(|lines| GenerationOutput {
                    lines,
                    warning: Some(format!(
                        "warning: no quotes found for '{}', falling back to 'words' mode", config.language)),
                })
                .map_err(|e| format!("error quote fallback failed, {}", e))
        }

        // zen mode
        GameMode::Zen => Ok(GenerationOutput {
            lines: Vec::new(),
            warning: Some("info: zen mode doesn't require words".to_string()),
        }),
    }
}

fn load_words(lang: &str, count: usize) -> Result<Vec<String>, String> {

    let path = Path::new(WORDS_DIR).join(format!("{}.txt", lang));
    if !path.exists() {
        return Err(format!("error: word list '{}' not found", path.display()));
    }

    let lines = load_file(&path)
        .map_err(|e| format!("error: cannot read word list '{}', {}", path.display(), e))?;

    Ok(finalize_lines(lines, count))
}

fn load_quote(lang: &str) -> Result<Vec<String>, String> {

    let dir = Path::new(QUOTES_DIR).join(lang);

    if !dir.is_dir() {
        return Err(format!("error: quotes directory for '{}' not found", lang));
    }

    let files: Vec<PathBuf> = fs::read_dir(&dir)
        .map_err(|e| format!("error: cannot read directory '{}', {}", dir.display(), e))?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_file())
        .collect();

    if files.is_empty() {
        return Err(format!("warning: no quote files in '{}'", dir.display()));
    }

    let mut rng = rng();
    let file = files
        .choose(&mut rng)
        .expect("non-empty file list ensured by check above");

    load_file(file).map_err(|e| format!("error: failed to read quote '{}', {}", file.display(), e))
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
