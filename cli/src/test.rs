/*
 * cli/src/test.rs
 */

use crossterm::event::{
    KeyCode,
    KeyEvent,
    KeyEventKind,
    KeyModifiers
};
use std::time::Instant;

use core::{
    Config,
    results::{
        Key,
        Event,
        Word,
        RawResults
    }
};

#[derive(Debug)]
pub struct Test {
    pub words: Vec<Word>,
    pub current_word: usize,
    pub complete: bool,
    pub backtrack: bool,
    pub death: bool,
    start_time: Instant,
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
            start_time: Instant::now(),
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        let event_key = convert_key(&key);
        let elapsed = self.start_time.elapsed();

        if self.words.is_empty() {
            self.complete = true;
            return;
        }

        let current = &mut self.words[self.current_word];

        match event_key {
            // end current test
            Key::CtrlC | Key::Escape => {
                current.events.push(Event {
                    time: elapsed,
                    key: event_key,
                    correct: None,
                });
                self.complete = true;
            }

            // finalize current word
            Key::Enter | Key::Space => {
                if !current.progress.is_empty() || current.text.is_empty() {
                    let correct = current.text == current.progress;
                    current.events.push(Event {
                        time: elapsed,
                        key: event_key,
                        correct: None,
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
            Key::Backspace => {
                if current.progress.is_empty() {
                    if self.backtrack && self.current_word > 0 {
                        self.prev_word();
                        // save backspace key press in prev word
                        self.words[self.current_word].events.push(Event {
                            time: elapsed,
                            key: event_key,
                            correct: None
                        });
                    }
                } else {
                    current.progress.pop();
                    current.events.push(Event {
                        time: elapsed,
                        key: event_key,
                        correct: None,
                    });
                }
            }

            // process character input
            Key::Char(c) => {
                current.progress.push(c);

                let partial_correct = current.text.starts_with(&current.progress);
                current.events.push(Event {
                    time: elapsed,
                    key: event_key.clone(),
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

impl From<&Test> for RawResults {
    fn from(test: &Test) -> Self {
        let words = test.words.clone();
        let events = words.iter().flat_map(|w| w.events.clone()).collect();

        RawResults { words, events }
    }
}

fn convert_key(key: &KeyEvent) -> Key {
    if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
        return Key::CtrlC;
    }

    match key.code {
        KeyCode::Enter => Key::Enter,
        KeyCode::Backspace => Key::Backspace,
        KeyCode::Esc => Key::Escape,
        KeyCode::Char(' ') => Key::Space,
        KeyCode::Char(c) => Key::Char(c),
        other => Key::Other(format!("{:?}", other)),
    }
}

