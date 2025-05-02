use dioxus::prelude::*;
use typingcore::{Config, GameMode, validate_config, language_from_str, Language, WordsLanguages, QuotesLanguages};

#[component]
pub fn Settings() -> Element { 
    let mut config = use_signal(|| Config::default());

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
            }
        }
    } 
}
