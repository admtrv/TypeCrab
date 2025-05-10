//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component  to be used in our app.

mod header ;
pub use header::Header;

mod footer;
pub use footer::Footer;

mod letter;
pub use letter::{Letter, LetterProps, LetterState};

mod results;
pub use results::{Results};


