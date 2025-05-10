use std::rc::Rc;
use dioxus::prelude::*;
use web_sys::{console, window, Storage};
use typingcore::{
    Level,
    generate_content, 
    process_results,
    RawResults,
    results::{
        FinalResults,
    },
    Test
};
use crate::pages::settings::{StoredConfig};
use crate::components::{
    Results,
    TestComponent
};

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

    let mut test = use_signal(|| None::<Test>);
    let mut test_start = use_signal(|| None::<f64>);
    let mut complete = use_signal(|| false);
    let mut words = use_signal(|| None::<Rc<Vec<String>>>);
    let mut final_results = use_signal(|| None::<FinalResults>);

    let restart_test = move |_| {
        // Reset all relevant signals to their initial states
        test.set(None);
        test_start.set(None);
        complete.set(false);
        final_results.set(None);
        
        // Regenerate content based on current config
        let config = current_config.read().config.clone();
        spawn(async move {
            let generation_response = generate_content(&config).await;
            if let Some((Level::Error, msg)) = &generation_response.message {
                console::log_1(&msg.as_str().into());
            }
            words.set(Some(Rc::new(generation_response.payload)));
        });
    };

    use_effect(move || {
        let config = current_config.read().config.clone();
        spawn(async move {
            let generation_response = generate_content(&config).await;
            if let Some((Level::Error, msg)) = &generation_response.message {
                console::log_1(&msg.as_str().into());
            }
            words.set(Some(Rc::new(generation_response.payload)));
        });
    });

    use_effect(move || {
        if *complete.read() && test.read().is_some() {
            let raw_results = RawResults::from(test.read().as_ref().unwrap());
            final_results.set(Some(process_results(raw_results).payload));
        }
    });

    rsx! { 
        main { class: "typing-test-main",
            {
                if final_results.read().is_some() {
                    rsx! {
                        Results { results: final_results.read().as_ref().unwrap().clone() }
                    }
                } else {
                    rsx! {
                        TestComponent {
                            words: words,
                            test: test,
                            test_start: test_start,
                            complete: complete,
                            config: current_config.read().config.clone()
                        }
                    }
                }

            }
            button {
                class: "restart-button",
                onclick: restart_test,
                "restart"
            }
        }
    }
}
