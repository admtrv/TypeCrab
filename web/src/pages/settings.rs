use dioxus::prelude::*;
use typingcore::{Config, GameMode, validate_config, language_from_str, Language, WordsLanguages, QuotesLanguages};
use web_sys::{console, window, Storage};

#[component]
pub fn Settings() -> Element { 
    let mut config = use_signal(|| {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(json)) = storage.get_item("config") {
                    if let Ok(parsed_config) = Config::from_json_string(&json) {
                        return parsed_config;
                    }
                }
            }
        }
        Config::default()
    });

    let language_options = match config.read().mode {
        GameMode::Words => WordsLanguages::all()
            .iter()
            .map(|lang| lang.as_str().to_string())
            .collect::<Vec<_>>(),
        GameMode::Quote => QuotesLanguages::all()
            .iter()
            .map(|lang| lang.as_str().to_string())
            .collect::<Vec<_>>(),
        GameMode::Zen => vec!["en".to_string()], // No language options for Zen
    };
    let current_language = match config.read().language {
        Language::Words(lang) => lang.as_str(),
        Language::Quotes(lang) => lang.as_str(),
    };
    rsx! {
        main {
            form {
                onsubmit: move |event| {
                    let mut new_config = (*config.read()).clone();
                    if let Some(mode) = event.data.values().get("mode") {
                        new_config.mode = match mode.0[0].as_str() {
                            "words" => GameMode::Words,
                            "quote" => GameMode::Quote,
                            "zen" => GameMode::Zen,
                            _ => GameMode::Words,
                        };
                    }

                    // Parse language (if not Zen mode)
                    if new_config.mode != GameMode::Zen {
                        if let Some(lang) = event.data.values().get("language") {
                            new_config.language = language_from_str(&lang.0[0], new_config.mode);
                        }
                    }

                    // Parse word count
                    if let Some(word_count) = event.data.values().get("word-count") {
                        if let Ok(num) = word_count.0[0].parse::<usize>() {
                            new_config.word_count = num;
                        }
                    }

                    // Parse time limit
                    if let Some(time_limit) = event.data.values().get("time-limit") {
                        if let Ok(num) = time_limit.0[0].parse::<u32>() {
                            new_config.time_limit = Some(num);
                        }
                    }

                    // Parse checkboxes
                    if new_config.mode == GameMode::Words {
                        new_config.punctuation = event.data.values().get("punctuation").map(|v| v == "on").unwrap_or(false);
                        new_config.numbers = event.data.values().get("numbers").map(|v| v == "on").unwrap_or(false);
                    }

                    new_config.backtrack = event.data.values().get("backtrack").map(|v| v == "on").unwrap_or(false);
                    new_config.death = event.data.values().get("death").map(|v| v == "on").unwrap_or(false);

                    // Update config
                    config.set(new_config.clone());
                    if let Ok(json) = new_config.to_json_string() {
                        if let Some(window) = window() {
                            if let Ok(Some(storage)) = window.local_storage() {
                                if let Err(e) = storage.set_item("config", &json) {
                                    console::log_1(&format!("Failed to save to localStorage: {:?}", e).into());
                                } else {
                                    console::log_1(&"Saved to localStorage".into());
                                }
                            }
                        }
                    } else {
                        console::log_1(&"Failed to serialize config".into());
                    }
                },
                label { "Game mode: " }
                select {
                    name: "mode",
                    onchange: move |event| {
                        let new_mode = match event.value().as_str() {
                            "words" => GameMode::Words,
                            "quote" => GameMode::Quote,
                            "zen" => GameMode::Zen,
                            _ => GameMode::Words, // fallback in case of unexpected value
                        };


                        let mut new_config = (*config.read()).clone(); // ✅ dereference before clone
                        new_config.mode = new_mode;
                        config.set(new_config); // ✅ update signal
                    },
                    option { value: "words", selected: config.read().mode == GameMode::Words, "Words" }
                    option { value: "quote", selected: config.read().mode == GameMode::Quote, "Quote" }
                    option { value: "zen", selected: config.read().mode == GameMode::Zen, "Zen" }
                }
                if config.read().mode != GameMode::Zen {
                    label { "Language: " }
                    select {
                        name: "language",
                        onchange: move |event| {
                            let new_language = language_from_str(&event.value(), config.read().mode);
                            let mut new_config = (*config.read()).clone();
                            new_config.language = new_language;
                            config.set(new_config);
                        },
                        for lang in language_options {
                            option {
                                value: "{lang}",
                                selected: lang == current_language,
                                "{lang}"
                            }
                        }
                    }
                }
                label { "Word count" }
                input {
                    name: "word-count",
                    r#type: "number",
                    min: "1",
                    value: "{config.read().word_count}",
                    onchange: move |event| {
                        match event.value().parse::<usize>() {
                            Ok(num) => {
                            let mut new_config = (*config.read()).clone();
                            new_config.word_count= num;
                            config.set(new_config);
                            }
                            Err(_) => todo!()
                        };
                    }
                }
                label { "Time limit (Optional)" }
                input {
                    name: "time-limit",
                    r#type: "number",
                    min: "0",
                    value: "{config.read().time_limit.unwrap_or(0)}",
                    onchange: move |event| {
                        match event.value().parse::<u32>() {
                            Ok(num) => {
                            let mut new_config = (*config.read()).clone();
                            new_config.time_limit = Some(num);
                            config.set(new_config);
                            }
                            Err(_) => todo!()
                        };
                    }
                }
            
                if config.read().mode == GameMode::Words {
                    label {"Punctuation"}
                    input {
                        name: "punctuation",
                        r#type: "checkbox",
                        checked:"{config.read().punctuation}"
                    },
                    label {"Numbers"}
                    input {
                        name: "numbers",
                        r#type: "checkbox",
                        checked:"{config.read().numbers}"
                    } 
                }

                label {"Backtrack"}
                input {
                    name: "backtrack",
                    r#type: "checkbox",
                    checked:"{config.read().backtrack}"
                } 
                label {"Death"}
                input {
                    name: "death",
                    r#type: "checkbox",
                    checked:"{config.read().death}"
                } 
                input {
                    r#type: "submit",
                    value: "Save config",
                }
            }
        }
    } 
}
