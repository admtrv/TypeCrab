use dioxus::prelude::*;
use dioxus_toast::{ToastInfo, ToastManager};
use typingcore::{Config, GameMode, validate_config, language_from_str, Language, WordsLanguages, QuotesLanguages, Level, Schemes, BASE_PATH};
use web_sys::{console, window, HtmlLinkElement};
use web_sys::wasm_bindgen::JsCast;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct StoredConfig {
    pub id: String,
    pub name: String,
    pub config: Config,
}

impl StoredConfig {
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    pub fn from_json_string(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}
impl Default for StoredConfig {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: "default config".to_string(),
            config: Config::default()
        }
    }
}

const SETTINGS_CSS: Asset = asset!("/assets/styling/settings.css");

#[component]
pub fn Settings() -> Element { 

    let mut toast: Signal<ToastManager> = use_context();

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

    let mut configs = use_signal(|| {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(json)) = storage.get_item("configs") {
                    if let Ok(parsed_configs) = serde_json::from_str::<Vec<StoredConfig>>(&json) {
                        return parsed_configs;
                    }
                }
            }
        }
        vec![current_config.read().clone()]
    });

    let mut current_scheme = use_signal(|| {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(scheme)) = storage.get_item("current_scheme") {
                    return scheme;
                }
            }
        }
        Schemes::Catppuccin.as_str().to_string() 
    });

    use_effect(move || {
        let scheme = current_scheme.read().clone();
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                if let Some(existing) = document.get_element_by_id("scheme-style") {
                    existing.remove();
                }
                if let Ok(link) = document.create_element("link") {
                    let link: HtmlLinkElement = link.dyn_into::<HtmlLinkElement>()
                        .expect("Failed to cast Element to HtmlLinkElement");
                    link.set_id("scheme-style");
                    link.set_rel("stylesheet");
                    link.set_href(&format!("/{}/assets/schemes/{}.css", BASE_PATH, scheme));
                    if let Some(head) = document.head() {
                        head.append_child(&link).unwrap();
                    }
                }
            }

            if let Ok(Some(storage)) = window.local_storage() {
                if let Err(e) = storage.set_item("current_scheme", &scheme) {
                    toast.write().popup(ToastInfo::error(format!("Failed to save scheme to localStorage: {:?}", e).as_str(),"Error"));
                }
            }
        }
    });

    let language_options = match current_config.read().config.mode {
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
    let current_language = match current_config.read().config.language {
        Language::Words(lang) => lang.as_str(),
        Language::Quotes(lang) => lang.as_str(),
    };
    rsx! {
        document::Link { rel: "stylesheet", href: SETTINGS_CSS}
        main {
            div {
                id: "settings-container",
                label {
                    "config",
                    select {
                        for config in configs.read().clone() {
                        option {
                            value: "{config.id}",
                                    selected: config.id == *current_config.read().id,
                                    "{config.name}"
                            }
                        }     
                    }
                }
                button {
                    id: "create",
                    onclick: move |_| {
                        let mut new_configs = (*configs.read()).clone();
                        let config = StoredConfig::default();
                        new_configs.push(config.clone());
                        configs.set(new_configs.clone());
                        current_config.set(config);
                    },
                    "create" 
                }
                button {
                    id: "delete",
                    onclick: move |_| {
                        let mut new_configs = (*configs.read()).clone();
                        // Prevent deleting if only one config remains
                        if new_configs.len() > 1 {
                            new_configs.retain(|config| config.id != current_config.read().id);
                            configs.set(new_configs.clone());
                            
                            // Set new current config to first available config
                            if let Some(new_current) = new_configs.first() {
                                current_config.set(new_current.clone());
                                
                                // Save new current config to localStorage
                                if let Ok(json) = new_current.to_json_string() {
                                    if let Some(window) = window() {
                                        if let Ok(Some(storage)) = window.local_storage() {
                                            if let Err(e) = storage.set_item("current_config", &json) {
                                                toast.write().popup(ToastInfo::error(format!("Failed to save to localStorage: {:?}",e).as_str(), "Error"));
                                                console::log_1(&format!("Failed to save to localStorage: {:?}", e).into());
                                            }
                                        }
                                    }
                                }
                            }
                            
                            // Save updated configs list to localStorage
                            if let Ok(json) = serde_json::to_string_pretty(&*configs.read()) {
                                if let Some(window) = window() {
                                    if let Ok(Some(storage)) = window.local_storage() {
                                        if let Err(e) = storage.set_item("configs", &json) {
                                            toast.write().popup(ToastInfo::error(format!("Failed to save to localStorage: {:?}",e).as_str(), "Error"));
                                            console::log_1(&format!("Failed to save to localStorage: {:?}", e).into());
                                        }
                                    }
                                }
                            }
                        }
                    },
                    disabled: configs.read().len() <= 1,
                    "delete"
                }
            }
            form {
                onsubmit: move |event| {
                    let mut new_config = (*current_config.read()).clone();
                    if let Some(name) = event.data.values().get("name") {
                        new_config.name = name.0[0].to_string();
                    }
                    if let Some(mode) = event.data.values().get("mode") {
                        new_config.config.mode = match mode.0[0].as_str() {
                            "words" => GameMode::Words,
                            "quote" => GameMode::Quote,
                            "zen" => GameMode::Zen,
                            _ => GameMode::Words,
                        };
                    }

                    // Parse language (if not Zen mode)
                    if new_config.config.mode != GameMode::Zen {
                        if let Some(lang) = event.data.values().get("language") {
                            new_config.config.language = language_from_str(&lang.0[0], new_config.config.mode);
                        }
                    }

                    // Parse word count
                    if let Some(word_count) = event.data.values().get("word-count") {
                        if let Ok(num) = word_count.0[0].parse::<usize>() {
                            new_config.config.word_count = num;
                        }
                    }

                    // Parse time limit
                    if let Some(time_limit) = event.data.values().get("time-limit") {
                        if let Ok(num) = time_limit.0[0].parse::<u32>() {
                            new_config.config.time_limit = Some(num);
                        }
                    }

                    // Parse checkboxes
                    if new_config.config.mode == GameMode::Words {
                        new_config.config.punctuation = event.data.values().get("punctuation").map(|v| v == "on").unwrap_or(false);
                        new_config.config.numbers = event.data.values().get("numbers").map(|v| v == "on").unwrap_or(false);
                    }

                    new_config.config.backtrack = event.data.values().get("backtrack").map(|v| v == "on").unwrap_or(false);
                    new_config.config.death = event.data.values().get("death").map(|v| v == "on").unwrap_or(false);

                    
                    let config_response = validate_config(new_config.config);

                    if let Some((Level::Error, msg)) = &config_response.message {
                        toast.write().popup(ToastInfo::error(msg, "Validation Error"));
                    }
                    new_config.config = config_response.payload;
                    // Update config
                    current_config.set(new_config.clone());
                    if let Ok(json) = new_config.to_json_string() {
                        if let Some(window) = window() {
                            if let Ok(Some(storage)) = window.local_storage() {
                                if let Err(e) = storage.set_item("current_config", &json) {
                                    toast.write().popup(ToastInfo::error(format!("Failed to save to localStorage: {:?}",e).as_str(), "Error"));
                                    console::log_1(&format!("Failed to save to localStorage: {:?}", e).into());
                                } else {
                                    toast.write().popup(ToastInfo::success("Config saved!", "Success"));
                                    console::log_1(&"Saved to localStorage".into());
                                }
                            }
                        }
                    } else {
                        console::log_1(&"Failed to serialize config".into());
                    }

                    let mut new_configs = (*configs.read()).clone(); // Clone the configs to modify
                    for config in new_configs.iter_mut() { // Use iter_mut for mutable references
                        if config.id == current_config.read().id {
                            *config = current_config.read().clone(); // Update the matching config
                        }
                    }
                    configs.set(new_configs); // Update the configs with the modified version

                    // Serialize and save to localStorage
                    if let Ok(json) = serde_json::to_string_pretty(&*configs.read()) {
                        if let Some(window) = window() {
                            if let Ok(Some(storage)) = window.local_storage() {
                                if let Err(e) = storage.set_item("configs", &json) {
                                    console::log_1(&format!("Failed to save to localStorage: {:?}", e).into());
                                } else {
                                    console::log_1(&"Saved to localStorage".into());
                                }
                            }
                        }
                    } else {
                        console::log_1(&"Failed to serialize config".into());
                        toast.write().popup(ToastInfo::success("Configs saved!", "Success"));
                    }
                },
                label { "config name",
                    input {
                        name: "name",
                        r#type: "text",
                        placeholder: "enter config name",
                        value: "{current_config.read().name}"
                    }
                }
                label { "game mode",
                        select {
                            name: "mode",
                            onchange: move |event| {
                                let new_mode = match event.value().as_str() {
                                    "words" => GameMode::Words,
                                    "quote" => GameMode::Quote,
                                    "zen" => GameMode::Zen,
                                    _ => GameMode::Words, // fallback in case of unexpected value
                                };


                                let mut new_config = (*current_config.read()).clone(); 
                                new_config.config.mode = new_mode;
                                current_config.set(new_config); 
                            },
                            option { value: "words", selected: current_config.read().config.mode == GameMode::Words, "words" }
                        option { value: "quote", selected: current_config.read().config.mode == GameMode::Quote, "quote" }
                        option { value: "zen", selected: current_config.read().config.mode == GameMode::Zen, "zen" }
                    }
                }
                if current_config.read().config.mode != GameMode::Zen {
                    label { "language",
                        select {
                        name: "language",
                                    for lang in language_options {
                                    option {
                                    value: "{lang}",
                                    selected: lang == current_language,
                                    "{lang}"
                                }
                            }
                        }
                    }
                    label { "word count", 
                        input {
                            name: "word-count",
                            r#type: "number",
                            min: "1",
                            max: "250",
                            value: "{current_config.read().config.word_count}",

                        }
                    }
                }
                label { "time limit (optional)",
                    input {
                        name: "time-limit",
                        r#type: "number",
                        min: "0",
                        value: "{current_config.read().config.time_limit.unwrap_or(0)}",
                    }
                }

                if current_config.read().config.mode == GameMode::Words {
                    label {"punctuation", 
                        input {
                        name: "punctuation",
                        r#type: "checkbox",
                        checked:"{current_config.read().config.punctuation}"
                        }
                    }
                    label {"numbers", 
                        input {
                            name: "numbers",
                            r#type: "checkbox",
                            checked:"{current_config.read().config.numbers}"
                        } 
                    }
                }

                label {"backtrack", 
                    input {
                        name: "backtrack",
                        r#type: "checkbox",
                        checked:"{current_config.read().config.backtrack}"
                    } 
                }

                label {"death" ,
                    input {
                        name: "death",
                        r#type: "checkbox",
                        checked:"{current_config.read().config.death}"
                    } 
                }
                input {
                    r#type: "submit",
                    value: "save",
                }
            }
            div {
                id: "scheme-select",
                label {
                "color scheme",
                    select {
                        name: "scheme",
                        onchange: move |event| {
                            let new_scheme = event.value();
                            current_scheme.set(new_scheme);
                        },
                        for scheme in Schemes::all() {
                            option {
                            value: "{scheme.as_str()}",
                            selected: scheme.as_str() == *current_scheme.read(),
                            "{scheme.as_str()}"
                            }
                        }
                    }
                }
            }
        }
    } 
}
