/*
 * cli/src/logic.rs
 */

use crossterm::event::{
    KeyCode,
    KeyEvent,
    KeyEventKind
};
use std::time::Instant;

use core::Config;

#[derive(Debug, Clone)]
pub struct Event {
    pub time: Instant,          // when it happened
    pub key: KeyEvent,          // what key
    pub correct: Option<bool>,  // true - correct, false - mistake, none - system move
}

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

#[derive(Debug)]
pub struct Test {
    pub words: Vec<Word>,
    pub current_word: usize,
    pub complete: bool,
    pub backtrack: bool,
    pub death: bool,
}

impl Test {
    pub fn new(words: Vec<String>, config: &Config) -> Self {
        let words = words.into_iter().map(Word::from).collect();
        Self {
            words,
            current_word: 0,
            complete: false,
            backtrack: config.backtrack,
            death: config.death,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        if self.words.is_empty() {
            self.complete = true;
            return;
        }

        match key.code {

            // finalize current word
            KeyCode::Char(' ') | KeyCode::Enter => {
                let current = &mut self.words[self.current_word];
                if !current.progress.is_empty() || current.text.is_empty() {
                    let correct = current.text == current.progress;
                    current.events.push(Event {
                        time: Instant::now(),
                        key,
                        correct: Some(correct),
                    });

                    // end test if wrong and sudden death enabled
                    if self.death && !correct {
                        self.complete = true;
                        return;
                    } else {
                        self.next_word();
                    }
                }
            }

            // process backspace/backtracking
            KeyCode::Backspace => {
                let current = &mut self.words[self.current_word];
                if current.progress.is_empty() {
                    if self.backtrack && self.current_word > 0 {
                        self.prev_word();
                    }
                } else {
                    current.progress.pop();
                    current.events.push(Event {
                        time: Instant::now(),
                        key,
                        correct: None,
                    });
                }
            }

            // process character input
            KeyCode::Char(c) => {
                let current = &mut self.words[self.current_word];
                current.progress.push(c);

                let partial_correct = current.text.starts_with(&current.progress);
                current.events.push(Event {
                    time: Instant::now(),
                    key,
                    correct: Some(partial_correct),
                });

                if self.death && !partial_correct {
                    self.complete = true;
                    return;
                }

                if current.progress == current.text && self.current_word == self.words.len() - 1 {
                    self.complete = true;
                }
            }

            _ => {}
        }
    }

    fn prev_word(&mut self) {
        if self.current_word > 0 {
            self.current_word -= 1;
        }
    }

    fn next_word(&mut self) {
        if self.current_word == self.words.len() - 1 {
            self.complete = true;
        } else {
            self.current_word += 1;
        }
    }
}

#[derive(Debug, Clone)]
pub struct RawData {
    pub words: Vec<Word>,
    pub events: Vec<Event>,
}

impl From<&Test> for RawData {
    fn from(test: &Test) -> Self {
        let words: Vec<Word> = test.words
            .iter()
            .map(|w| Word {
                text: w.text.clone(),
                progress: w.progress.clone(),
                events: w.events.iter().map(|e| Event {
                    time: e.time,
                    key: e.key,
                    correct: e.correct,
                }).collect(),
            })
            .collect();

        let events = words.iter().flat_map(|w| w.events.clone()).collect();

        RawData { words, events }
    }
}