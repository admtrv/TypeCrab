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
};

#[cfg(target_arch = "wasm32")]
use crate::{
    languages::{
        BASE_PATH
    }
};
#[cfg(not(target_arch = "wasm32"))]
use crate::{
    WORDS_DIR,
    QUOTES_DIR,
};

const PUNCTS: &[&str] = &[".", ",", "!", "?", ":", ";"];
const NUMBER_RANGE: std::ops::RangeInclusive<u32> = 1..=9999;

const PUNCT_PROBABILITY: f64 = 0.2;
const NUMBER_PROBABILITY: f64 = 0.2;

pub type GeneratorResponse = Response<Vec<String>>;

// api function, that generates test content according to config
macro_rules! generate_content {
    ($($maybe_async:tt)?) => {
        pub $($maybe_async)? fn generate_content(config: &Config) -> GeneratorResponse {
            // if user specified file, read from it directly
            if let Some(user_file) = &config.file {
                let lines = match load_file(user_file) {
                    Ok(lines) => lines,
                    Err(e) => {
                        return GeneratorResponse::with_error(
                            Vec::new(),
                            format!("invalid file '{}', {}", user_file, e),
                        );
                    }
                };

                return match config.mode {
                    GameMode::Words => GeneratorResponse::plain(finalize_lines(lines, config)),
                    GameMode::Quote => GeneratorResponse::plain(split_lines(lines)),
                    GameMode::Zen => GeneratorResponse::plain(vec![String::new()]),
                };
            }

            match config.mode {
                GameMode::Words => {
                    if let Language::Words(lang) = config.language {
                        #[cfg(target_arch = "wasm32")]
                        let lines = match load_words(lang.as_str()).await {
                            Ok(lines) => lines,
                            Err(e) => return GeneratorResponse::with_error(Vec::new(), e),
                        };

                        #[cfg(not(target_arch = "wasm32"))]
                        let lines = match load_words(lang.as_str()) {
                            Ok(lines) => lines,
                            Err(e) => return GeneratorResponse::with_error(Vec::new(), e),
                        };

                        GeneratorResponse::plain(finalize_lines(lines, config))
                    } else {
                        GeneratorResponse::with_error(
                            Vec::new(),
                            "invalid language for words mode".to_string(),
                        )
                    }
                }

                GameMode::Quote => {
                    if let Language::Quotes(lang) = config.language {

                        #[cfg(target_arch = "wasm32")]
                        let lines = match load_quote(lang.as_str()).await {
                            Ok(lines) => lines,
                            Err(e) => return GeneratorResponse::with_error(Vec::new(), e),
                        };
                        #[cfg(not(target_arch = "wasm32"))]
                        let lines = match load_quote(lang.as_str()) {
                            Ok(lines) => lines,
                            Err(e) => return GeneratorResponse::with_error(Vec::new(), e),
                        };
                        GeneratorResponse::plain(split_lines(lines))
                    } else {
                        GeneratorResponse::with_error(
                            Vec::new(),
                            "invalid language for quote mode".to_string(),
                        )
                    }
                }

                GameMode::Zen => {
                    GeneratorResponse::plain(vec![String::new()])
                }
            }
        }
    };
}

// Usage:
#[cfg(target_arch = "wasm32")]
generate_content!(async);

#[cfg(not(target_arch = "wasm32"))]
generate_content!();


#[cfg(target_arch = "wasm32")]
fn base_url() -> String {
    let origin = web_sys::window()
        .unwrap()
        .location()
        .origin()
        .unwrap();
    
    if BASE_PATH == "" {
        return origin;
    } else {
        return format!("{}/{}", origin, BASE_PATH)
    }
}

#[cfg(target_arch = "wasm32")]
async fn load_words(lang: &str) -> Result<Vec<String>, String> {
    let url = format!("{}/assets/words/{}.txt", base_url(), lang);
    let text = reqwest::get(&url)
        .await
        .map_err(|e| format!("failed to fetch words: {}", e))?
        .text()
        .await
        .map_err(|e| format!("failed to read response: {}", e))?;
    Ok(text.lines().map(|s| s.to_string()).collect())
}

#[cfg(not(target_arch = "wasm32"))]
fn load_words(lang: &str) -> Result<Vec<String>, String> {
    let path = Path::new(WORDS_DIR).join(format!("{}.txt", lang));
    load_file(&path).map_err(|e| format!("cannot read words '{}', {}", path.display(), e))
}

#[cfg(target_arch = "wasm32")]
async fn load_quote(lang: &str) -> Result<Vec<String>, String> {
    use crate::languages::{language_from_str};
    use crate::config::GameMode;

    // Convert language string to QuotesLanguages enum
    let language = match language_from_str(lang, GameMode::Quote) {
        Language::Quotes(lang) => lang,
        _ => return Err("invalid language for quote mode".to_string()),
    };

    // Get list of quote files for the language
    let quote_files = language.quote_files();
    if quote_files.is_empty() {
        return Err("no quote files available for this language".to_string());
    }

    // Select a random quote file
    let mut rng = rng();
    let selected_file = quote_files
        .choose(&mut rng)
        .ok_or_else(|| "failed to select quote file".to_string())?;

    // Construct URL and fetch the file
    let url = format!("{}/assets/quotes/{}/{}", base_url(), lang, selected_file);
    let text = reqwest::get(&url)
        .await
        .map_err(|e| format!("failed to fetch quote: {}", e))?
        .text()
        .await
        .map_err(|e| format!("failed to read response: {}", e))?;

    Ok(text.lines().map(|s| s.to_string()).collect())
}

#[cfg(not(target_arch = "wasm32"))]
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
