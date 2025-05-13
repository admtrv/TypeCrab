use dioxus::prelude::*;
use dioxus_toast::{ToastFrame, ToastInfo, ToastManager};
use web_sys::HtmlLinkElement;
use typingcore::{Schemes, BASE_PATH};
use web_sys::wasm_bindgen::JsCast;

use components::*;
use pages::*;
mod components;
mod pages;

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const APPLE_ICON: Asset = asset!("/public/images/icons/apple-touch-icon.png");
const ICON_32: Asset = asset!("/public/images/icons/favicon-32x32.png");
const ICON_16: Asset = asset!("/public/images/icons/favicon-16x16.png");
const MANIFEST : Asset = asset!("/assets/site.webmanifest");

fn main() {
    dioxus::launch(App);
}

#[derive(Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(AppLayout)] // Reference AppLayout here
        #[route("/")]
        TypingTest {},
        #[route("/settings")]
        Settings {},
}

#[component]
fn AppLayout() -> Element {
    rsx! {
        Header {} 
        Outlet::<Route> {} 
        Footer {}
    }
}
#[component]
fn App() -> Element {
    let mut toast = use_context_provider(|| Signal::new(ToastManager::default()));
    
    // Signal to store the current scheme, initialized from localStorage or default
    let mut current_scheme = use_signal(|| {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(scheme)) = storage.get_item("current_scheme") {
                    return scheme;
                }
            }
        }
        Schemes::Catppuccin.as_str().to_string()
    });

    // Effect to handle scheme loading and updating on mount and when current_scheme changes
    use_effect(move || {
        let scheme = current_scheme.read().clone();
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                // Remove existing scheme stylesheet if it exists
                if let Some(existing) = document.get_element_by_id("scheme-style") {
                    existing.remove();
                }
                // Create and append new scheme stylesheet
                if let Ok(link) = document.create_element("link") {
                    let link: HtmlLinkElement = link
                        .dyn_into::<HtmlLinkElement>()
                        .expect("Failed to cast Element to HtmlLinkElement");
                    link.set_id("scheme-style");
                    link.set_rel("stylesheet");
                    link.set_href(&format!("/{}/assets/schemes/{}.css", BASE_PATH, scheme));
                    if let Some(head) = document.head() {
                        let _ = head.append_child(&link);
                    }
                }
            }

            // Save scheme to localStorage
            if let Ok(Some(storage)) = window.local_storage() {
                if let Err(e) = storage.set_item("current_scheme", &scheme) {
                    toast
                        .write()
                        .popup(ToastInfo::error(
                            format!("Failed to save scheme to localStorage: {:?}", e).as_str(),
                            "Error",
                        ));
                }
            }
        }
    });

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "apple-touch-icon", sizes: "180x180", href: APPLE_ICON }
        document::Link { rel: "icon", type: "image/png", sizes: "32x32", href: ICON_32 }
        document::Link { rel: "icon", type: "image/png", sizes: "16x16", href:  ICON_16 }
        document::Link { rel: "manifest", href: MANIFEST }
        ToastFrame { manager: toast }
        Router::<Route> {}
    }
}
