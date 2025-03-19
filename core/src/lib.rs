// core/src/lib.rs

#[derive(Debug, Clone, Copy)]
pub enum GameMode {
    Words,
    Quote,
    Zen,
}

impl GameMode {
    pub fn from_flags(words: bool, quote: bool, zen: bool) -> Self {
        match (words, quote, zen) {
            (true, false, false) => Self::Words,
            (false, true, false) => Self::Quote,
            (false, false, true) => Self::Zen,
            _ => {
                println!("Invalid mode selection. Defaulting to Words mode.");
                Self::Words
            }
        }
    }
}

#[derive(Debug)]
pub struct TestConfig {
    pub mode: GameMode,
    pub language: String,
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
            word_count: 25,
            time_limit: 60,
            punctuation: false,
            numbers: false,
            backtrack: true,
            death: false,
        }
    }
}

pub struct TypingTest {
    config: TestConfig,
}

impl TypingTest {
    pub fn new(config: TestConfig) -> Self {
        Self { config }
    }

    pub fn start(&self) {
        println!("Start configuration:");
        println!("  Mode: {:?}", self.config.mode);
        println!("  Language: {}", self.config.language);
        println!("  Word Count: {}", self.config.word_count);
        println!("  Time Limit: {}", self.config.time_limit);
        println!("  Include Punctuation: {}", self.config.punctuation);
        println!("  Include Numbers: {}", self.config.numbers);
        println!("  Backtracking: {}", self.config.backtrack);
        println!("  Sudden Death: {}", self.config.death);
    }
}
