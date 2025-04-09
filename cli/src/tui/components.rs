/*
 * cli/src/tui/components.rs
 */

use once_cell::sync::{
    Lazy,
    OnceCell
};
use ratatui::prelude::{
    Color,
    Modifier,
    Span,
    Style
};
use ratatui::widgets::{
    Block,
    BorderType,
    Borders
};
use regex::Regex;
use std::collections::HashMap;
use std::fs;

static SCHEME_VARS: OnceCell<HashMap<String, String>> = OnceCell::new();

// function to load scheme into hash map
pub fn load_scheme_file(path: &str) -> Result<(), String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("cannot read scheme file '{}'", path))?;
    let map = parse_css_variables(&content)?;

    SCHEME_VARS
        .set(map)
        .map_err(|_| "scheme was already loaded".to_string())?;

    Ok(())
}

// parse css lines like '--var-name: #rrggbb;'
fn parse_css_variables(content: &str) -> Result<HashMap<String, String>, String> {
    let re = Regex::new(r"--([^:]+):\s*([^;]+);")
        .map_err(|e| "invalid regex".to_string())?;

    let mut map = HashMap::new();
    for caps in re.captures_iter(content) {
        let var_name = caps[1].trim().to_string();
        let var_value = caps[2].trim().to_string();
        map.insert(var_name, var_value);
    }

    Ok(map)
}

// from #rrggbb to color object
fn parse_hex_color(hex_str: &str) -> Color {
    let hex = hex_str.trim();
    if let Some(h) = hex.strip_prefix('#') {
        if h.len() == 6 {
            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&h[0..2], 16),
                u8::from_str_radix(&h[2..4], 16),
                u8::from_str_radix(&h[4..6], 16),
            ) {
                return Color::Rgb(r, g, b);
            }
        }
    }
    // if error, then return default value
    Color::White
}

// helper to get color from hash map
fn scheme_color(var_name: &str, fallback: Color) -> Color {
    if let Some(map) = SCHEME_VARS.get() {
        if let Some(hex) = map.get(var_name) {
            return parse_hex_color(hex);
        }
    }
    fallback
}

// common colors
pub static COLOR_RED: Lazy<Color> = Lazy::new(|| scheme_color("red-color", Color::Red));
pub static COLOR_GREEN: Lazy<Color> = Lazy::new(|| scheme_color("green-color", Color::Green));
pub static COLOR_YELLOW: Lazy<Color> = Lazy::new(|| scheme_color("yellow-color", Color::LightYellow));
pub static COLOR_WHITE: Lazy<Color> = Lazy::new(|| scheme_color("white-color", Color::White));
pub static COLOR_DARK:  Lazy<Color> = Lazy::new(|| scheme_color("dark-color", Color::Black));
pub static COLOR_LIGHT: Lazy<Color> = Lazy::new(|| scheme_color("light-color", Color::DarkGray));

// common styles

// message level styles
pub static STYLE_INFO: Lazy<Style> = Lazy::new(|| Style::default().fg(*COLOR_GREEN));
pub static STYLE_WARNING: Lazy<Style> = Lazy::new(|| Style::default().fg(*COLOR_YELLOW));
pub static STYLE_ERROR: Lazy<Style> = Lazy::new(|| Style::default().fg(*COLOR_RED));

// input feedback styles
pub static STYLE_CORRECT: Lazy<Style> = Lazy::new(|| Style::default().fg(*COLOR_GREEN));
pub static STYLE_INCORRECT: Lazy<Style> = Lazy::new(|| Style::default().fg(*COLOR_RED));

// current character style
pub static STYLE_ACTIVE: Lazy<Style> = Lazy::new(|| Style::default().fg(*COLOR_WHITE));
pub static STYLE_UNDERLINE: Lazy<Style> = Lazy::new(|| Style::default().fg(*COLOR_WHITE).add_modifier(Modifier::UNDERLINED));
pub static STYLE_INACTIVE: Lazy<Style> = Lazy::new(|| Style::default().fg(*COLOR_LIGHT));

// block styles
pub static STYLE_BACKGROUND: Lazy<Style> = Lazy::new(|| Style::default().bg(*COLOR_DARK));
pub static STYLE_BORDER: Lazy<Style> = Lazy::new(|| Style::default().fg(*COLOR_YELLOW));
pub static STYLE_TITLE: Lazy<Style> = Lazy::new(|| Style::default().fg(*COLOR_WHITE));


pub(crate) fn styled_block(title: &str) -> Block<'_> {
    Block::default()
        // background
        .style(*STYLE_BACKGROUND)

        // title
        .title(Span::styled(title, *STYLE_TITLE))

        // border
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(*STYLE_BORDER)
}
