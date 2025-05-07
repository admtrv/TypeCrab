use dioxus::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum LetterState {
    Correct,
    Incorrect,
}

#[derive(Props, PartialEq)]
pub struct LetterProps {
    letter: char,
    state: Option<LetterState>,
}

#[component]
pub fn Letter(props: LetterProps) -> Element { 
    let class = match props.state {
        Some(LetterState::Correct) => "letter correct",
        Some(LetterState::Incorrect) => "letter incorrect",
        None => "letter",
    };
    rsx!{
        span {
           class: "{class}",
            "{props.letter}"
        }
    }
}
