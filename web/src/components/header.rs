use dioxus::prelude::*;

const LOGO_PNG: Asset = asset!("/public/images/logos/horizontal-black-color.png");

#[component]
pub fn Header() -> Element { 
    rsx! {
        header { 
                a{ href: "/",
                    img { id: "logo",
                        src: LOGO_PNG, 
                        alt: "Typecrab Logo"
                    }
                },
            nav { 
                ul { 
                    li {
                        a { 
                            href: "/",
                            "test"
                        }
                    }
                    li {
                        a { 
                            href: "/settings",
                            "settings"
                        }
                    }
                }
            }
        }
    }
}
