/*
 * cli/src/tui/components.rs
 */

use once_cell::sync::Lazy;
use ratatui::prelude::{Color, Modifier, Span, Style};
use ratatui::widgets::{Block, BorderType, Borders};

// common colors
const COLOR_GREEN: Color = Color::Green;
const COLOR_RED: Color = Color::Red;
const COLOR_YELLOW: Color = Color::LightYellow;

const COLOR_WHITE: Color = Color::White;
const COLOR_GRAY: Color = Color::DarkGray;

// common styles

// message level styles
pub static STYLE_INFO: Lazy<Style> = Lazy::new(|| Style::default().fg(COLOR_GREEN));
pub static STYLE_WARNING: Lazy<Style> = Lazy::new(|| Style::default().fg(COLOR_YELLOW));
pub static STYLE_ERROR: Lazy<Style> = Lazy::new(|| Style::default().fg(COLOR_RED));

// input feedback styles
pub static STYLE_CORRECT: Lazy<Style> = Lazy::new(|| Style::default().fg(COLOR_GREEN));
pub static STYLE_INCORRECT: Lazy<Style> = Lazy::new(|| Style::default().fg(COLOR_RED));

// current character style
pub static STYLE_ACTIVE: Lazy<Style> = Lazy::new(|| Style::default().fg(COLOR_WHITE));
pub static STYLE_UNDERLINE: Lazy<Style> = Lazy::new(|| { Style::default().fg(COLOR_WHITE).add_modifier(Modifier::UNDERLINED)});
pub static STYLE_INACTIVE: Lazy<Style> = Lazy::new(|| Style::default().fg(COLOR_GRAY));

// block styles
pub static STYLE_BORDER: Lazy<Style> = Lazy::new(|| Style::default().fg(COLOR_YELLOW));
pub static STYLE_TITLE: Lazy<Style> = Lazy::new(|| Style::default().fg(COLOR_WHITE));


pub(crate) fn styled_block(title: &str) -> Block<'_> {
    Block::default()
        .title(Span::styled(title, *STYLE_TITLE))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(*STYLE_BORDER)
}
