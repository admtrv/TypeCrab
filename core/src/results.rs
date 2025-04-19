/*
 * core/src/results.rs
 */

use std::time::Duration;
use std::collections::HashMap;

use crate::{response::Response};

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
#[derive(Debug, Clone)] 
pub struct KeyPresses {
    pub correct: usize,    
    pub incorrect: usize,
    pub extra: usize,
    pub missed: usize,

}

impl Default for KeyPresses {
    fn default() -> Self {
        KeyPresses {
            correct: 0,
            incorrect: 0,
            extra: 0,
            missed: 0,
        }
    }
}

#[derive(Debug, Clone)] 
pub struct FinalResults {
    pub wpm: f64,                       // Words per minute (correct chars only)
    pub raw_wpm: f64,                   // Words per minute (all chars)
    pub key_presses: KeyPresses,        // keypresses data
    pub accuracy: f64,                  // Percentage of correct keypresses  
    pub consistency: f64,               // Consistency score 
    pub worst_keys: Vec<(char, char, usize)>, // Keys with most errors 
    pub graph_data: Vec<(f64, f64, f64, usize, usize, usize)>, // (time, wpm, raw_wpm, incorrect, extra, missed)
    pub errors: Vec<(char, usize)>
}

impl Default for FinalResults {
    fn default() -> Self {
        FinalResults {
            wpm: 0.0,
            raw_wpm: 0.0,
            accuracy: 0.0,
            consistency: 0.0,
            key_presses: KeyPresses::default(),
            worst_keys: Vec::new(),
            graph_data: Vec::new(),
            errors: Vec::new(),
        }
    }
}

pub fn process_results(raw_results: RawResults) -> Response<FinalResults> {
    if raw_results.events.is_empty() {
        println!("ERROR: No typing events recorded");
        return Response::with_error(FinalResults::default(), "No typing events recorded");
    }

    let mut correct_keypresses = 0;
    let mut incorrect_keypresses = 0;
    let mut extra_keypress = 0;
    let mut missed_keypresses = 0;
    let mut key_errors: HashMap<(char, char), usize> = HashMap::new();
    let mut graph_data = Vec::new();
    let mut total_correct_chars = 0;
    let mut total_typed_chars = 0;

    let first_time = raw_results.events.iter()
        .filter(|e| e.correct.is_some())
        .map(|e| e.time)
        .min()
        .unwrap_or(Duration::ZERO);
    let last_time = raw_results.events.iter()
        .filter(|e| e.correct.is_some())
        .map(|e| e.time)
        .max()
        .unwrap_or(Duration::ZERO);

    let total_duration = if last_time > first_time {
        (last_time - first_time).as_secs_f64()
    } else {
        0.0
    };
    println!("INFO: Total duration: {} seconds, first_time: {:?}, last_time: {:?}", total_duration, first_time, last_time);

    for (word_idx, word) in raw_results.words.iter().enumerate() {
        println!("INFO: Processing word {}: text='{}', progress='{}', events_count={}", word_idx, word.text, word.progress, word.events.len());
        let expected = word.text.chars().collect::<Vec<_>>();
        let mut char_index = 0;

        for (event_idx, event) in word.events.iter().enumerate() {
            println!("DEBUG: Word {} Event {}: key={:?}, correct={:?}, char_index={}, time={:?}", word_idx, event_idx, event.key, event.correct, char_index, event.time);
            if let Some(correct) = event.correct {
                if char_index >= expected.len() {
                    println!("WARN: char_index {} exceeds expected length {} for word '{}'", char_index, expected.len(), word.text);
                }
                char_index += 1;
                if correct {
                    correct_keypresses += 1;
                    total_correct_chars += 1;
                    total_typed_chars += 1;
                    println!("DEBUG: Correct keypress: char_index={}, expected={:?}", char_index - 1, expected.get(char_index - 1));
                } else {
                    incorrect_keypresses += 1;
                    total_typed_chars += 1;
                    if let Key::Char(typed) = event.key {
                        if char_index <= expected.len() {
                            let expected_char = expected[char_index - 1];
                            if expected_char == typed {
                                println!("ERROR: Unexpected error counted: word='{}', char_index={}, expected='{}', typed='{}'", word.text, char_index - 1, expected_char, typed);
                            }
                            *key_errors.entry((expected_char, typed)).or_insert(0) += 1;
                            println!("DEBUG: Incorrect keypress: char_index={}, expected='{}', typed='{}'", char_index - 1, expected_char, typed);
                        } else {
                            println!("WARN: Incorrect keypress beyond word length: typed='{}', word='{}'", typed, word.text);
                        }
                    } else {
                        println!("DEBUG: Non-char incorrect keypress: key={:?}", event.key);
                    }
                }
            } else if Key::Backspace == event.key {
                if char_index > 0 {
                    char_index -= 1;
                    println!("DEBUG: Backspace: char_index reduced to {}", char_index);
                } else {
                    println!("WARN: Backspace at char_index=0 for word '{}'", word.text);
                }
            } else {
                println!("DEBUG: System event ignored: key={:?}", event.key);
            }
        }

        let typed = word.progress.chars().collect::<Vec<_>>();
        println!("INFO: Word {} summary: expected_len={}, typed_len={}, char_index={}", word_idx, expected.len(), typed.len(), char_index);

        if typed.len() > expected.len() {
            extra_keypress += typed.len() - expected.len();
            println!("WARN: Extra keypresses: word='{}', extra_count={}", word.text, typed.len() - expected.len());
        }

        if expected.len() > typed.len() {
            missed_keypresses += expected.len() - typed.len();
            println!("WARN: Missed keypresses: word='{}', missed_count={}", word.text, expected.len() - typed.len());
        }
    }

    let total_minutes = total_duration / 60.0;
    let wpm = if total_minutes > 0.0 {
        (total_correct_chars as f64 / 5.0) / total_minutes
    } else {
        0.0
    };

    let raw_wpm = if total_minutes > 0.0 {
        (total_typed_chars as f64 / 5.0) / total_minutes
    } else {
        0.0
    };

    let total_keypresses = correct_keypresses + incorrect_keypresses + extra_keypress;

    let accuracy = if total_keypresses > 0 {
        (correct_keypresses as f64 / total_keypresses as f64) * 100.0
    } else {
        0.0
    };
    println!("INFO: WPM: {}, Raw WPM: {}, Accuracy: {}%, Total keypresses: {}", wpm, raw_wpm, accuracy, total_keypresses);

    // Graph data calculation
    let mut current_correct_chars = 0;
    let mut current_typed_chars = 0;
    let mut current_incorrect = 0;
    let mut current_extra = 0;
    let mut current_missed = 0;
    let mut last_time_secs = 0.0;

    for event in &raw_results.events {
        let event_time = (event.time.as_secs_f64() - first_time.as_secs_f64()).max(0.0);
        match event.correct {
            Some(true) => {
                current_correct_chars += 1;
                current_typed_chars += 1;
            }
            Some(false) => {
                current_incorrect += 1;
                current_typed_chars += 1;
            }
            None => {
                if let Key::Char(_) = event.key {
                    current_extra += 1;
                    current_typed_chars += 1;
                }
                continue;
            }
        }
        if event_time.floor() > last_time_secs && event_time > 0.0 {
            let current_wpm = if event_time > 0.0 {
                (current_correct_chars as f64 / 5.0) / (event_time / 60.0)
            } else {
                0.0
            };

            let current_raw_wpm = if event_time > 0.0 {
                (current_typed_chars as f64 / 5.0) / (event_time / 60.0)
            } else {
                0.0
            };

            graph_data.push((
                event_time,
                current_wpm,
                current_raw_wpm,
                current_incorrect,
                current_extra,
                current_missed
            ));
            last_time_secs = event_time.floor();
        }
    }

    // Calculate consistency
    let mut inter_key_times = Vec::new();
    let mut last_time = None;
    for event in &raw_results.events {
        if event.correct.is_none() {
            continue;
        }
        let current_time = event.time.as_secs_f64();
        if let Some(prev_time) = last_time {
            let interval = current_time - prev_time;
            inter_key_times.push(interval);
        }
        last_time = Some(current_time);
    }

    let consistency = if !inter_key_times.is_empty() {
        let mean = inter_key_times.iter().sum::<f64>() / inter_key_times.len() as f64;
        let variance = inter_key_times.iter()
            .map(|t| (t - mean).powi(2))
            .sum::<f64>() / inter_key_times.len() as f64;
        let std_dev = variance.sqrt();
        (1.0 - std_dev.min(1.0)) * 100.0
    } else {
        0.0
    };
    println!("INFO: Consistency: {}%, Inter-key times count: {}", consistency, inter_key_times.len());

    let mut worst_keys: Vec<(char, char, usize)> = key_errors.clone().into_iter()
        .map(|((expected, typed), count)| (expected, typed, count))
        .collect();
    worst_keys.sort_by(|a, b| b.2.cmp(&a.2));
    worst_keys.truncate(3);
    let mut error_counts: HashMap<char, usize> = HashMap::new();
    
    // Sum counts for each expected character
    for ((expected, _), count) in key_errors {
        *error_counts.entry(expected).or_insert(0) += count;
    }
    
    // Convert to vector and sort
    let mut errors: Vec<(char, usize)> = error_counts.into_iter().collect();
    errors.sort_by(|a, b| b.1.cmp(&a.1));
    println!("INFO: Worst keys: {:?}", worst_keys);

    Response::plain(FinalResults {
        wpm,
        raw_wpm,
        accuracy,
        consistency,
        key_presses: KeyPresses {
            correct: correct_keypresses,
            incorrect: incorrect_keypresses,
            extra: extra_keypress,
            missed: missed_keypresses
        },
        worst_keys,
        graph_data,
        errors
    })
}

