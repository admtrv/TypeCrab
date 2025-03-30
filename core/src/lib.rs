/*
 * core/src/lib.rs
 */

mod config;
mod generator;
mod response;
mod listing;

const WORDS_DIR: &str = "resources/words";
const QUOTES_DIR: &str = "resources/quotes";

pub use response::{
    Level,
    Response
};
pub use config::{
    GameMode,
    Config,
    validate_config
};
pub use generator::generate_content;
pub use listing::list_languages;

