/*
 * core/src/lib.rs
 */

mod config;
mod generator;
mod response;
mod listing;
mod languages;
mod test;
pub mod results;


const WORDS_DIR: &str = "resources/words";

#[cfg(not(target_arch = "wasm32"))]
const QUOTES_DIR: &str = "resources/quotes";

pub const SCHEMES_DIR: &str = "resources/schemes";

pub use response::{
    Level,
    Response
};
pub use config::{
    GameMode,
    Config,
    Language,
    validate_config
};
pub use generator::generate_content;
pub use listing::{
    list_languages,
    list_schemes
};
pub use test::{
    Test
};
pub use results::{
    Key,
    Event,
    Word,
    RawResults,
    process_results
};
pub use languages::{
    language_from_str,
    QuotesLanguages,
    WordsLanguages,
    Schemes,
};

#[cfg(target_arch = "wasm32")]
pub use languages::{
    BASE_PATH
};

