/*
 * core/src/lib.rs
 */

mod config;
mod generator;
mod response;
mod listing;
pub mod results;

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
pub use results::{
    Key,
    Event,
    Word,
    RawResults
};

