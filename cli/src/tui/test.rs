/*
 * cli/src/tui/test.rs
 */

use ratatui::{
    buffer::Buffer,
    layout::{
        Constraint,
        Direction,
        Layout,
        Rect
    },
    style::{
        Color,
        Modifier,
        Style
    },
    text::{
        Line,
        Span
    },
    widgets::{
        Block,
        BorderType,
        Borders,
        Paragraph,
        Widget
    },
};
use unicode_width::UnicodeWidthStr;
use core::Level;

use crate::logic::Test;

pub struct TestView<'a> {
    pub test: &'a Test,
    pub status: Option<(Level, String)>,
}

impl<'a> Widget for TestView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),    // test
                Constraint::Length(3), // status bar
            ])
            .split(area);

        // build test lines
        let test_lines = build_test(self.test, layout[0].width as usize);

        // render test
        let prompt = Paragraph::new(test_lines)
            .block(styled_block("Test"));
        prompt.render(layout[0], buf);

        // render status
        let status_line = build_status(&self.status);
        let status = Paragraph::new(status_line)
            .block(styled_block("Status"));
        status.render(layout[1], buf);
    }
}

fn build_test(test: &Test, max_width: usize) -> Vec<Line> {
    let mut lines = Vec::new();
    let mut current_spans = Vec::new();
    let mut current_width = 0;

    for (i, word) in test.words.iter().enumerate() {

        // highlighting word (by symbols)
        let spans_for_word = word_to_spans(i, test);

        // resulting word width
        let mut word_width = 0;
        for sp in &spans_for_word {
            word_width += sp.content.as_ref().width();
        }

        // if not first in line, then space
        let space = if current_spans.is_empty() { 0 } else { 1 };

        // if word not fits in line, then new line
        if current_width + word_width + space > max_width {
            lines.push(Line::from(current_spans));
            current_spans = Vec::new();
            current_width = 0;
        }

        // add space
        if space == 1 {
            current_spans.push(Span::raw(" "));
            current_width += 1;
        }

        // add spans
        current_spans.extend(spans_for_word);
        current_width += word_width;

        // if word ends with '/n', then new line
        if word.text.ends_with('\n') || word.progress.ends_with('\n') {
            lines.push(Line::from(current_spans));
            current_spans = Vec::new();
            current_width = 0;
        }
    }

    // if there are left, add them
    if !current_spans.is_empty() {
        lines.push(Line::from(current_spans));
    }

    lines
}

// if i > current_word, then grey (inactive word)
// else highlighting word (by symbols)
fn word_to_spans(i: usize, test: &Test) -> Vec<Span<'static>> {
    if i > test.current_word {
        return vec![Span::styled(
            test.words[i].text.clone(),
            Style::default().fg(Color::DarkGray),
        )];
    }

    let typed = &test.words[i].progress;
    let text = &test.words[i].text;
    let is_current = i == test.current_word;

    highlight_word(typed, text, is_current)
}

// highlighting word by symbol:
//  before first mistake - all green
//  after first mistake - all red
//  current symbol - underline
//  remaining part - grey (inactive part)
fn highlight_word(typed: &str, text: &str, is_current: bool) -> Vec<Span<'static>> {
    // split both strings into characters
    let typed_chars: Vec<char> = typed.chars().collect();
    let text_chars: Vec<char> = text.chars().collect();

    let mut spans = Vec::new();
    let mut mismatch_happened = false;
    let mut i = 0;

    // compare each typed char with target char
    while i < typed_chars.len() && i < text_chars.len() {
        let t_char = typed_chars[i];
        let r_char = text_chars[i];

        if mismatch_happened {
            // after first mistake - everything red
            spans.push(Span::styled(t_char.to_string(), Style::default().fg(Color::Red)));
        } else if t_char == r_char {
            // correct character - green
            spans.push(Span::styled(t_char.to_string(), Style::default().fg(Color::Green)));
        } else {
            // first mistake - red and set flag
            mismatch_happened = true;
            spans.push(Span::styled(t_char.to_string(), Style::default().fg(Color::Red)));
        }
        i += 1;
    }

    // remaining typed chars (extra input) - red
    for &c in &typed_chars[i..] {
        spans.push(Span::styled(c.to_string(), Style::default().fg(Color::Red)));
    }

    // if user not completed word
    if i < text_chars.len() {
        if is_current {
            // current word - underline next expected char
            spans.push(Span::styled(
                text_chars[i].to_string(),
                Style::default().fg(Color::White).add_modifier(Modifier::UNDERLINED),
            ));
            // rest - gray
            for &ch in &text_chars[i + 1..] {
                spans.push(Span::styled(ch.to_string(), Style::default().fg(Color::DarkGray)));
            }
        } else {
            // nt current - all remaining chars gray
            for &ch in &text_chars[i..] {
                spans.push(Span::styled(ch.to_string(), Style::default().fg(Color::DarkGray)));
            }
        }
    }

    spans
}

fn build_status(status: &Option<(Level, String)>) -> Line {
    match status {
        Some((level, msg)) => {
            let label_color = match level {
                Level::Info => Color::Green,
                Level::Warning => Color::LightYellow,
                Level::Error => Color::Red,
            };
            Line::from(vec![
                Span::styled(format!("{}: ", level_label(level)), Style::default().fg(label_color)),
                Span::styled(msg, Style::default().fg(Color::White)),
            ])
        }
        None => Line::from(""),
    }
}

fn level_label(level: &Level) -> &'static str {
    match level {
        Level::Info => "info",
        Level::Warning => "warning",
        Level::Error => "error",
    }
}

fn styled_block(title: &str) -> Block<'_> {
    Block::default()
        .title(Span::styled(title, Style::default().fg(Color::White)))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::LightYellow))
}
