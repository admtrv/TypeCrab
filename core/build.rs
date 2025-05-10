use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

// Define constants for directory paths
const QUOTES_DIR: &str = "../resources/quotes";
const WORDS_DIR: &str = "../resources/words";

// Function to convert kebab-case to CamelCase for enum variants
fn kebab_to_camel(s: &str) -> String {
    s.split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

// Function to recursively copy a directory and its contents
fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

// Function to generate languages.rs file with enums
fn generate_languages_file() -> io::Result<()> {
    let mut words_variants = Vec::new();
    let words_path = Path::new(WORDS_DIR);

    // Collect variants for WordsLanguages from .txt files
    for entry in fs::read_dir(words_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("txt") {
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                let variant_name = kebab_to_camel(file_name.strip_suffix(".txt").unwrap_or(file_name));
                words_variants.push((variant_name, file_name.to_string()));
            }
        }
    }

    let mut quotes_variants = Vec::new();
    let quotes_path = Path::new(QUOTES_DIR);

    // Collect variants and quote files for QuotesLanguages from folders
    for entry in fs::read_dir(quotes_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(folder_name) = path.file_name().and_then(|s| s.to_str()) {
                let variant_name = kebab_to_camel(folder_name);
                // Collect .txt files in the language directory
                let mut quote_files = Vec::new();
                for quote_entry in fs::read_dir(&path)? {
                    let quote_entry = quote_entry?;
                    let quote_path = quote_entry.path();
                    if quote_path.is_file()
                        && quote_path.extension().and_then(|s| s.to_str()) == Some("txt")
                    {
                        if let Some(quote_file_name) = quote_path.file_name().and_then(|s| s.to_str())
                        {
                            quote_files.push(quote_file_name.to_string());
                        }
                    }
                }
                quote_files.sort(); // Ensure consistent order
                quotes_variants.push((variant_name, folder_name.to_string(), quote_files));
            }
        }
    }

    // Generate languages.rs content
    let mut content = String::new();
    content.push_str("use crate::config::{GameMode, Language};\n\n");
    content.push_str("use serde::{Serialize, Deserialize};\n\n");

    // WordsLanguages enum
    content.push_str("/// Auto-generated enum for words languages\n");
    content.push_str("#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]\npub enum WordsLanguages {\n");
    for (variant_name, _) in &words_variants {
        content.push_str(&format!("    {},\n", variant_name));
    }
    content.push_str("}\n\n");

    content.push_str("impl WordsLanguages {\n");
    content.push_str("    pub fn file_path(&self) -> &'static str { match self {\n");
    for (variant_name, file_name) in &words_variants {
        content.push_str(&format!(
            "        WordsLanguages::{} => \"/assets/words/{}\",\n",
            variant_name, file_name
        ));
    }
    content.push_str("    } }\n\n");

    content.push_str("    pub fn all() -> &'static [WordsLanguages] { &[\n");
    for (variant_name, _) in &words_variants {
        content.push_str(&format!("        WordsLanguages::{},\n", variant_name));
    }
    content.push_str("    ] }\n\n");

    content.push_str("    pub fn as_str(&self) -> &'static str { match self {\n");
    for (variant_name, file_name) in &words_variants {
        let file_name_no_ext = file_name.strip_suffix(".txt").unwrap_or(file_name);
        content.push_str(&format!(
            "        WordsLanguages::{} => \"{}\",\n",
            variant_name, file_name_no_ext
        ));
    }
    content.push_str("    } }\n");
    content.push_str("}\n\n");

    // QuotesLanguages enum
    content.push_str("/// Auto-generated enum for quotes languages\n");
    content.push_str("#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]\npub enum QuotesLanguages {\n");
    for (variant_name, _, _) in &quotes_variants {
        content.push_str(&format!("    {},\n", variant_name));
    }
    content.push_str("}\n\n");

    content.push_str("impl QuotesLanguages {\n");
    content.push_str("    pub fn dir_path(&self) -> &'static str { match self {\n");
    for (variant_name, folder_name, _) in &quotes_variants {
        content.push_str(&format!(
            "        QuotesLanguages::{} => \"/assets/quotes/{}/\",\n",
            variant_name, folder_name
        ));
    }
    content.push_str("    } }\n\n");

    content.push_str("    pub fn quote_files(&self) -> &'static [&'static str] { match self {\n");
    for (variant_name, _, quote_files) in &quotes_variants {
        content.push_str(&format!("        QuotesLanguages::{} => &[\n", variant_name));
        for file_name in quote_files {
            content.push_str(&format!("            \"{}\",\n", file_name));
        }
        content.push_str("        ],\n");
    }
    content.push_str("    } }\n\n");

    content.push_str("    pub fn all() -> &'static [QuotesLanguages] { &[\n");
    for (variant_name, _, _) in &quotes_variants {
        content.push_str(&format!("        QuotesLanguages::{},\n", variant_name));
    }
    content.push_str("    ] }\n\n");

    content.push_str("    pub fn as_str(&self) -> &'static str { match self {\n");
    for (variant_name, folder_name, _) in &quotes_variants {
        content.push_str(&format!(
            "        QuotesLanguages::{} => \"{}\",\n",
            variant_name, folder_name
        ));
    }
    content.push_str("    } }\n");
    content.push_str("}\n\n");

    // language_from_str function
    content.push_str("/// Converts a language string and game mode to a Language enum variant.\n");
    content.push_str("/// Defaults to Language::Words(WordsLanguages::En) if no match is found.\n");
    content.push_str("pub fn language_from_str(lang: &str, mode: GameMode) -> Language {\n");
    content.push_str("    match mode {\n");
    content.push_str("        GameMode::Quote => {\n");
    content.push_str("            QuotesLanguages::all()\n");
    content.push_str("                .iter()\n");
    content.push_str("                .find(|&l| l.as_str() == lang)\n");
    content.push_str("                .map(|&l| Language::Quotes(l))\n");
    content.push_str("                .unwrap_or(Language::Words(WordsLanguages::En))\n");
    content.push_str("        }\n");
    content.push_str("        GameMode::Words | GameMode::Zen => {\n");
    content.push_str("            WordsLanguages::all()\n");
    content.push_str("                .iter()\n");
    content.push_str("                .find(|&l| l.as_str() == lang)\n");
    content.push_str("                .map(|&l| Language::Words(l))\n");
    content.push_str("                .unwrap_or(Language::Words(WordsLanguages::En))\n");
    content.push_str("        }\n");
    content.push_str("    }\n");
    content.push_str("}\n");

    // Write to languages.rs
    let mut file = File::create("./src/languages.rs")?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

fn main() -> io::Result<()> {
    let target = std::env::var("TARGET").expect("TARGET not set");

    if target.contains("wasm32") {
        println!("cargo:rustc-cfg=getrandom_backend=\"wasm_js\"");
    }
    let source = Path::new("../resources");
    let destination = Path::new("../web/public/");

    copy_dir_recursive(source, destination)?;
    println!("Directory copied successfully!");

    generate_languages_file()?;
    println!("languages.rs generated successfully!");

    Ok(())
}
