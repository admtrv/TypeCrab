/*
 * cli/src/test.rs
 */

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        Block,
        BorderType,
        Borders,
        Paragraph,
        Widget,
    },
};

use unicode_width::UnicodeWidthStr;

use core::Level;

pub struct TestView<'a> {
    pub words: &'a [String],
    pub status: Option<(Level, String)>,
}

impl<'a> Widget for TestView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),       // test
                Constraint::Length(3),    // status bar
            ])
            .split(area);

        // build test lines
        let test_lines = build_test(self.words, layout[0].width as usize);

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

fn build_test(words: &[String], max_width: usize) -> Vec<Line> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0;

    for word in words {
        if word == "\n" {
            lines.push(Line::from(Span::styled(
                current_line.clone(),
                Style::default().fg(Color::DarkGray),
            )));
            current_line.clear();
            current_width = 0;
            continue;
        }

        let word_width = UnicodeWidthStr::width(word.as_str());
        let space = if current_line.is_empty() { 0 } else { 1 };

        if current_width + word_width + space > max_width {
            lines.push(Line::from(Span::styled(
                current_line.clone(),
                Style::default().fg(Color::DarkGray),
            )));
            current_line.clear();
            current_width = 0;
        }

        if !current_line.is_empty() {
            current_line.push(' ');
        }

        current_line.push_str(word);
        current_width += word_width + space;
    }

    if !current_line.is_empty() {
        lines.push(Line::from(Span::styled(
            current_line,
            Style::default().fg(Color::DarkGray),
        )));
    }

    lines
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
