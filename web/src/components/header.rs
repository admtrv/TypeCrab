use dioxus::prelude::*;
use dioxus_router::prelude::*; 

const LOGO_PNG: Asset = asset!("/public/images/logos/horizontal-white-color.png");

#[component]
pub fn Header() -> Element { 
    rsx! {
        header { 
            Link { 
                to: "/", 
                img { 
                    id: "logo",
                    src: LOGO_PNG, 
                    alt: "Typecrab Logo"
                }
            },
            nav { 
                ul { 
                    li {
                        Link { 
                            to: "/",
                            "test"
                        }
                    }
                    li {
                        Link { 
                            to: "/settings",
                            "settings"
                        }
                    }
                }
            }
        }
    }
}
