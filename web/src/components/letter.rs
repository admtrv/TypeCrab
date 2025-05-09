use dioxus::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum LetterState {
    Correct,
    Incorrect,
    Active,
}

#[derive(Props, PartialEq, Clone)]
pub struct LetterProps {
    letter: char,
    state: Option<LetterState>,
}

#[component]
pub fn Letter(props: LetterProps) -> Element { 
    let class = match props.state {
        Some(LetterState::Correct) => "letter correct",
        Some(LetterState::Incorrect) => "letter incorrect",
        Some(LetterState::Active) => "letter active",
        None => "letter", // Use "active" for the next expected character
    };
    rsx! {
        span {
            class: "{class}",
            "{props.letter}"
        }
    }
}
