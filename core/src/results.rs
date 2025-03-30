/*
 * core/src/results.rs
 */

use std::time::Duration;

// key representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Enter,
    Space,
    Backspace,
    Escape,
    CtrlC,
    Other(String),
}

// all events representation
#[derive(Debug, Clone)]
pub struct Event {
    pub time: Duration,         // when it happened
    pub key: Key,               // what key
    pub correct: Option<bool>,  // true - correct, false - mistake, none - system move
}

// one word representation
#[derive(Debug, Clone)]
pub struct Word {
    pub text: String,           // what needed to enter
    pub progress: String,       // what already entered
    pub events: Vec<Event>,     // all events
}

impl From<String> for Word {
    fn from(string: String) -> Self {
        Word {
            text: string,
            progress: String::new(),
            events: Vec::new(),
        }
    }
}

// raw test results representation
#[derive(Debug, Clone)]
pub struct RawResults {
    pub words: Vec<Word>,
    pub events: Vec<Event>,
}
