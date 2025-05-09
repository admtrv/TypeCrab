use dioxus::prelude::*;
use web_sys::{console, window, Storage};
use typingcore::{
    Level,
    generate_content, 
    process_results,
    RawResults,
    Config,
    results::{
        Key
    },
    Test
};
use web_sys::js_sys::Date;
use crate::pages::settings::{StoredConfig};
use crate::components::{Letter, LetterState};

fn convert_key(event: Event<KeyboardData>) -> Key {
    match event.code(){
        Code::Enter => Key::Enter,
        Code::Backspace => Key::Backspace,
        Code::Escape => Key::Escape,
        Code::Space => Key::Space,
        code => {
            let code_str = code.to_string();
            if code_str.starts_with("Key") && code_str.len() == 4 {
                Key::Char(code_str.chars().nth(3).unwrap().to_ascii_lowercase())
            } else {
                Key::Other(code_str)
            }
        }
    }
}
#[component]
pub fn TypingTest() -> Element { 
    let mut current_config = use_signal(|| {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(json)) = storage.get_item("current_config") {
                    if let Ok(parsed_config) = StoredConfig::from_json_string(&json) {
                        return parsed_config;
                    }
                }
            }
        }
        StoredConfig::default()
    });
    
    let words = use_resource(move || async move {
        let generation_response = generate_content(&current_config.read().config).await;

        if let Some((Level::Error, msg)) = &generation_response.message {
            console::log_1(&msg.as_str().into());
        }

        generation_response.payload
    });
    let mut test = use_signal(|| None::<Test>);
    let mut test_start = use_signal(|| None::<f64>);
    let mut complete = use_signal(|| false);

    use_effect(move || {
        if *complete.read() && test.read().is_some() {
            let raw_results = RawResults::from(test.read().as_ref().unwrap());
            // api final results generation from raw test results
            let final_results = process_results(raw_results).payload;
        }
    });
    let on_keydown = move |event: Event<KeyboardData>| {
        if *complete.read() {
            return;
            // TODO: handle restart
        }

        // Initialize test on first key press if not already started
        if test.read().is_none() {
            if let Some(words) = words.read().as_ref() {
                if !words.is_empty() {
                    test.set(Some(Test::new(words.clone(), &current_config.read().config)));
                    test_start.set(Some(Date::now()));
                }
            }
        }

        // Handle key press if test is active
        if let Some(ref mut test_state) = *test.write() {
            let key = convert_key(event.clone());
            test_state.handle_key(key);
            if event.data.code() == Code::Escape {
                complete.set(true);
            }
            if test_state.complete {
                complete.set(true);

            }
        }
    };

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

                for &char in &text_chars[i+1..] {
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

    rsx! {
        main {
            tabindex: 0, 
            onkeydown: on_keydown, 
            style: "outline: none;", 
            match *words.read() {
                None => rsx! { "Loading..." },
                Some(ref payload) => {
                    if let Some(ref test_state) = *test.read() {
                        rsx! {
                            div {
                                for (i, word) in payload.iter().enumerate() {
                                    div {
                                        class: "word",
                                        {
                                            let is_current = i == test_state.current_word;
                                            let typed = &test_state.words[i].progress;
                                            let text = &test_state.words[i].text;
                                            let chars = highlight_word(typed, text,is_current);
                                            rsx! {
                                                for(char, state) in chars {
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
                            div {
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
