use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element { 
    rsx! {
        footer {
            ul { 
                li {
                    a { 
                        href: "https://github.com/aldeimeter/",
                        target: "_blank",
                        img { 
                            src: "https://github.com/aldeimeter.png",
                            alt: "Artem Zaitsev's GitHub avatar",
                            class: "avatar",
                            title: "Artem Zaitsev"
                        }
                    }
                }
                li {
                    a { 
                        href: "https://github.com/admtrv/",
                        target: "_blank",
                        img { 
                            src: "https://github.com/admtrv.png",
                            alt: "Anton Dmitriev's GitHub avatar",
                            class: "avatar",
                            title: "Anton Dmitriev"
                        }
                    }
                }
                li {
                    a { 
                        href: "https://github.com/admtrv/typecrab",
                        target: "_blank",
                        img { 
                            src: "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png",
                            alt: "GitHub Logo",
                            class: "avatar",
                            title: "Typecrab Repository"
                        }
                    }
                }
            }
        }
    }
}
