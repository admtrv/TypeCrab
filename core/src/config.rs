/*
 * core/src/config.rs
 */

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
