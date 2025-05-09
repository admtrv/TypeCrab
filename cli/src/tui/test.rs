/*
 * cli/src/tui/test.rs
 */

use ratatui::{
    buffer::Buffer,
    layout::{
        Constraint,
        Direction,
        Layout,
        Rect,
    },
    prelude::{
        Modifier,
        Style,
    },
    text::{
        Line,
        Span,
    },
    widgets::{
        Paragraph,
        Widget,
    },
};
use once_cell::sync::Lazy;
use unicode_width::UnicodeWidthStr;
use core::Level;

use core::Test;
use crate::tui::scheme::{
    styled_block,
    COLOR_GREEN,
    COLOR_YELLOW,
    COLOR_RED,
    COLOR_WHITE,
    COLOR_LIGHT
};

// message level styles
static STYLE_INFO: Lazy<Style> = Lazy::new(|| Style::default().fg(*COLOR_GREEN));
static STYLE_WARNING: Lazy<Style> = Lazy::new(|| Style::default().fg(*COLOR_YELLOW));
static STYLE_ERROR: Lazy<Style> = Lazy::new(|| Style::default().fg(*COLOR_RED));

// input feedback styles
static STYLE_CORRECT: Lazy<Style> = Lazy::new(|| Style::default().fg(*COLOR_GREEN));
static STYLE_INCORRECT: Lazy<Style> = Lazy::new(|| Style::default().fg(*COLOR_RED));

// current character style
static STYLE_ACTIVE: Lazy<Style> = Lazy::new(|| Style::default().fg(*COLOR_WHITE));
static STYLE_UNDERLINE: Lazy<Style> = Lazy::new(|| Style::default().fg(*COLOR_WHITE).add_modifier(Modifier::UNDERLINED));
static STYLE_INACTIVE: Lazy<Style> = Lazy::new(|| Style::default().fg(*COLOR_LIGHT));

pub struct TestView<'a> {
    pub test: &'a Test,
    pub status: Option<String>,
    pub warning: Option<(Level, String)>,
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
            .block(styled_block(" test "));
        prompt.render(layout[0], buf);

        // render status
        let status_line = build_status(&self.warning, &self.status);
        let status = Paragraph::new(status_line)
            .block(styled_block(" status "));
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
        let word_width = spans_for_word
            .iter()
            .map(|sp| UnicodeWidthStr::width(sp.content.as_ref()))
            .sum::<usize>();

        // if word not fits in line, then new line
        if current_width + word_width + if current_spans.is_empty() { 0 } else { 1 } > max_width - 2 {
            lines.push(Line::from(current_spans));
            current_spans = Vec::new();
            current_width = 0;
        }

        // add space
        if !current_spans.is_empty() {
            // avoid duplicate underline at end
            let underline_space = {
                let prev = i - 1;
                prev == test.current_word && test.words[prev].progress.len() >= test.words[prev].text.len() && test.current_word < test.words.len() - 1
            };

            if underline_space {
                current_spans.push(Span::styled(" ", *STYLE_UNDERLINE));
            } else {
                current_spans.push(Span::raw(" "));
            }
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
            *STYLE_INACTIVE,
        )];
    }

    let typed = &test.words[i].progress;
    let text = &test.words[i].text;
    let is_current = i == test.current_word;
    let is_last = i == test.words.len() - 1;

    highlight_word(typed, text, is_current, is_last)
}

// highlighting word by symbols:
//  before first mistake - all green
//  after first mistake - all red
//  current symbol - underline
//  remaining part - grey (inactive part)
fn highlight_word(typed: &str, text:  &str, is_current: bool, is_last: bool, ) -> Vec<Span<'static>> {
    let typed_chars: Vec<char> = typed.chars().collect();
    let text_chars: Vec<char> = text.chars().collect();

    let mut spans = Vec::new();
    let mut mismatch = false;
    let mut i = 0;

    // matched and mismatched part
    while i < typed_chars.len() && i < text_chars.len() {
        let t = typed_chars[i];
        let r = text_chars[i];

        if mismatch {
            spans.push(Span::styled(r.to_string(), *STYLE_INCORRECT));
        } else if t == r {
            spans.push(Span::styled(t.to_string(), *STYLE_CORRECT));
        } else {
            mismatch = true;
            spans.push(Span::styled(r.to_string(), *STYLE_INCORRECT));
        }
        i += 1;
    }

    // extra characters
    for &c in &typed_chars[i..] {
        spans.push(Span::styled(c.to_string(), *STYLE_INCORRECT));
    }

    if is_current {
        if i < text_chars.len() {
            // underline next expected character
            spans.push(Span::styled(text_chars[i].to_string(), *STYLE_UNDERLINE));

            // show remaining characters as inactive
            for &ch in &text_chars[i + 1..] {
                spans.push(Span::styled(ch.to_string(), *STYLE_INACTIVE));
            }
        } else if is_last {
            // underline space if at the end of the last word
            spans.push(Span::styled(" ", *STYLE_UNDERLINE));
        }
    } else if i < text_chars.len() {
        // inactive characters for incomplete word
        for &ch in &text_chars[i..] {
            spans.push(Span::styled(ch.to_string(), *STYLE_INACTIVE));
        }
    }

    spans
}

fn build_status(warning: &Option<(Level, String)>, status: &Option<String>) -> Line<'static> {

    // priority - warning message
    if let Some((level, warning)) = warning {
        let (label, style) = match level {
            Level::Info => ("info", *STYLE_INFO),
            Level::Warning => ("warning", *STYLE_WARNING),
            Level::Error => ("error", *STYLE_ERROR),
        };

        return Line::from(vec![
            Span::styled(format!("{label}: "), style.clone()),
            Span::styled(warning.clone(), *STYLE_ACTIVE),
        ]);
    }

    // else status - words or time
    if let Some(status) = status {
        return Line::from(Span::styled(status.clone(), *STYLE_ACTIVE));
    }

    // empty
    Line::from("")
}
