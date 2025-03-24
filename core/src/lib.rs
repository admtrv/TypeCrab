/*
 * core/src/lib.rs
 */

mod config;
mod generator;

pub use config::{GameMode, TestConfig};
pub use generator::generate_content;
