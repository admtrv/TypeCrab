// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;
use components::*;
use pages::*;
/// Define a components module that contains all shared components for our app.
use typingcore::{
    Config,
    GameMode,
    Level,
    RawResults,
    process_results,
    language_from_str,
    SCHEMES_DIR,
    generate_content,
    list_languages,
    list_schemes,
    validate_config,
};
mod components;
mod pages;

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
// The asset macro also minifies some assets like CSS and JS to make bundled smaller
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    dioxus::launch(App);
}

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
///
#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[route("/")]
    TypingTest {},
    #[route("/settings")]
    Settings {}
}
#[component]
fn App() -> Element {
    let config = use_signal(|| Config::default());
    
    let words = use_resource(move || async move {
        let generation_response = generate_content(&config.read()).await;

        if let Some((Level::Error, msg)) = &generation_response.message {
            std::process::exit(1);
        }

        generation_response.payload
    });


    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Header {}
        Router::<Route> {}
        {words.read().as_ref().map(|vec| vec.join(" ")).unwrap_or_default()}
        Footer {}
    }
}
