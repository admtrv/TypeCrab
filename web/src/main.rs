// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;
use dioxus_toast::{ToastFrame, ToastInfo, ToastManager};

use components::*;
use pages::*;
/// Define a components module that contains all shared components for our app.
mod components;
mod pages;

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
// The asset macro also minifies some assets like CSS and JS to make bundled smaller
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const DEFAULT_SCHEME_CSS: Asset = asset!("/public/schemes/catppuccin.css");

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
    let toast = use_context_provider(|| Signal::new(ToastManager::default()));



    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: DEFAULT_SCHEME_CSS, id: "scheme-style" }
        ToastFrame { manager: toast }
        Header {}
        Router::<Route> {}
        Footer {}
    }
}
