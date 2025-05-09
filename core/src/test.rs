/*
 * core/test.rs
 */

use instant::Instant; 

use crate::{
    Config,
    results::{
        Key,
        Event,
        Word,
        RawResults
    },
    GameMode
};


#[derive(Debug)]
pub struct Test {
    pub words: Vec<Word>,
    pub current_word: usize,
    pub complete: bool,
    pub backtrack: bool,
    pub death: bool,
    pub mode: GameMode,
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
            mode: config.mode,
            start_time: Instant::now(),
        }
    }

    pub fn handle_key(&mut self, key: Key) {
        let elapsed = self.start_time.elapsed();

        if self.words.is_empty() {
            self.complete = true;
            return;
        }

        let current = &mut self.words[self.current_word];

        // zen mode
        if self.mode == GameMode::Zen {
            match key {
                Key::CtrlC | Key::Escape => {
                    current.events.push(Event { time: elapsed, key, correct: None });
                    self.complete = true;
                }

                Key::Enter | Key::Space => {
                    if current.progress.is_empty() {
                        return;
                    }

                    current.events.push(Event { time: elapsed, key, correct: None });
                    self.next_word();
                }


                Key::Backspace => {
                    if current.progress.is_empty() {
                        if self.backtrack && self.current_word > 0 {
                            self.prev_word();

                            self.words[self.current_word].events.push(Event {
                                time: elapsed,
                                key,
                                correct: None,
                            });
                        }
                    } else {
                        current.progress.pop();
                        current.text.pop();
                        current.events.push(Event { time: elapsed, key, correct: None });
                    }
                }

                Key::Char(c) => {
                    current.progress.push(c);
                    current.text.push(c);
                    current.events.push(Event { time: elapsed, key, correct: Some(true) });
                }

                _ => {}
            }

            return;
        }

        // other modes
        match key {
            // end current test
            Key::CtrlC | Key::Escape => {
                current.events.push(Event {
                    time: elapsed,
                    key,
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
                        key,
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
                            key,
                            correct: None,
                        });
                    }
                } else {
                    current.progress.pop();
                    current.events.push(Event {
                        time: elapsed,
                        key,
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
                    key: key.clone(),
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
        if self.mode == GameMode::Zen {
            // zen mode has no boundaries = append new empty word at the end
            if self.current_word == self.words.len() - 1 {
                self.words.push(Word::from(String::new()));
            }
            self.current_word += 1;
        } else {
            // mark test complete if last word reached
            if self.current_word == self.words.len() - 1 {
                self.complete = true;
            } else {
                self.current_word += 1;
            }
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
