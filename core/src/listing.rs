/*
 * core/src/listing.rs
 */

use std::fs;
use std::path::Path;

use crate::{
    response::Response,
    WORDS_DIR,
};

pub type ListingResponse = Response<Vec<String>>;

// api function, that lists available languages
pub fn list_languages() -> ListingResponse {
    let dir = Path::new(WORDS_DIR);

    let Ok(entries) = fs::read_dir(dir) else {
        return Response::with_error(Vec::new(), format!("cannot read directory '{}'", dir.display()));
    };

    let mut langs = entries
        .flatten()
        .filter_map(|e| e.path().file_stem()?.to_str().map(|s| s.to_string()))
        .collect::<Vec<_>>();

    if langs.is_empty() {
        Response::with_error(Vec::new(), "no languages found")
    } else {
        langs.sort();
        Response::plain(langs)
    }
}
