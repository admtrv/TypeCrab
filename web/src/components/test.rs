use std::rc::Rc;
use dioxus::prelude::*;
use typingcore::{
    results::{Key},
    Test,
    Config,
    GameMode,
};
use super::letter::{LetterState, Letter};
use web_sys::js_sys::Date;

fn convert_key(event: Event<KeyboardData>) -> Key {

    match event.key().to_string().as_str() {
        "Enter" => Key::Enter,
        "Backspace" => Key::Backspace,
        "Escape" => Key::Escape,
        " " => Key::Space,
        key => {
            if key.len() == 1 {
                // Single character keys (e.g., "a", "A", "1", "@") are treated as chars
                Key::Char(key.chars().next().unwrap())
            } else {
                // Other keys (e.g., "Shift", "Control", "F1") are treated as Other
                Key::Other(key.to_string())
            }
        }
    }
}

fn highlight_word(typed: &str, text: &str, is_current: bool) -> Vec<(char, Option<LetterState>)> {


    let typed_chars: Vec<char> = typed.chars().collect();
    let text_chars: Vec<char> = text.chars().collect();

    let mut result = Vec::new();
    let mut missmatch_happened = false;
    let mut i = 0;

    while i < typed_chars.len() && i < text_chars.len() {
        let typed_char = typed_chars[i];
        let text_char = text_chars[i];
        if missmatch_happened {
            result.push((text_char, Some(LetterState::Incorrect)));
        } else if typed_char == text_char {
            result.push((typed_char, Some(LetterState::Correct)));
        } else {
            missmatch_happened = true;
            result.push((text_char, Some(LetterState::Incorrect)));
        }
        i += 1;
    }

    for &char in &typed_chars[i..] {
        result.push((char, Some(LetterState::Incorrect)));
    }

    if i < text_chars.len() {
        if is_current {
            result.push((text_chars[i], Some(LetterState::Active)));
            for &char in &text_chars[i + 1..] {
                result.push((char, None));
            }
        } else {
            for &char in &text_chars[i..] {
                result.push((char, None));
            }
        }
    }

    result
}

#[derive(Props, Clone, PartialEq)]
pub struct TestProps {
    pub test: Signal<Option<Test>>,
    pub config: Config,
    pub complete: Signal<bool>,
    pub test_start: Signal<Option<f64>>,
    pub words: Signal<Option<Rc<Vec<String>>>>,
}

#[component]
pub fn TestComponent(mut props: TestProps) -> Element {
    let game_mode = props.config.mode;
    let on_keydown = move |event: Event<KeyboardData>| {
        if *props.complete.read() {
            if game_mode == GameMode::Zen {
                props.test.set(None);
                props.test_start.set(None);
                props.complete.set(false);
            }
            return;
        }

        // Initialize test on first key press if not already started
        if props.test.read().is_none() {
            if let Some(ref words) = props.words.as_ref() {
                if !words.is_empty() {
                    // Clone the words to avoid moving
                    props.test.set(Some(Test::new(words.clone().to_vec(), &props.config)));
                    props.test_start.set(Some(Date::now()));
                }
            }
        }

        // Handle key press if test is active
        if let Some(ref mut test_state) = *props.test.write() {
            let key = convert_key(event.clone());
            test_state.handle_key(key);
            if event.data.code() == Code::Escape {
                props.complete.set(true);
            }
            if test_state.complete {
                props.complete.set(true);
            }
        }
    };

    rsx! {
        div {
            tabindex: 0,
            onkeydown: on_keydown,
            onmounted: move |elem| async move { elem.set_focus(true).await; },
            style: "outline: none; :focus {{ outline: 2px solid blue; }}",
            class: "test-container",
            match *props.words.read() {
                None => rsx! { div { class:"loading", "Loading..." } },
                Some(ref payload) => {
                    if let Some(ref test_state) = *props.test.read() {
                        rsx! {
                            div { class: "words-container",
                                for (i, word) in test_state.words.iter().enumerate() {
                                    div {
                                        class: "word",
                                        {
                                            let is_current = i == test_state.current_word;
                                            let typed = &test_state.words[i].progress;
                                            let text = if game_mode == GameMode::Zen {
                                                &test_state.words[i].text
                                            } else {
                                                &payload[i]
                                            };
                                            let chars = highlight_word(typed, text, is_current);
                                            rsx! {
                                                for (char, state) in chars {
                                                    Letter {
                                                        letter: char,
                                                        state: state
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                } else {
                        rsx! {
                            div { class: "words-container",
                                for word in payload.iter() {
                                    div {
                                        class: "word",
                                        for char in word.chars() {
                                            Letter {
                                                letter: char,
                                                state: None
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
