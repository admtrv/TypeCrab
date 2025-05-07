use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::*,
    text::{Line, Span},
    widgets::Widget,
};

use crate::tui::scheme::{COLOR_ORANGE, COLOR_WHITE, STYLE_BACKGROUND};

pub struct StartView;

impl Widget for StartView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // background
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                buf[(x, y)].set_style(*STYLE_BACKGROUND);
            }
        }

        // content
        let lines = build_start();

        let height = lines.len() as u16;
        let width = lines.iter().map(Line::width).max().unwrap_or(0) as u16;

        let y_offset = area.y + (area.height.saturating_sub(height)) / 2;
        let x_offset = area.x + (area.width.saturating_sub(width)) / 2;

        for (i, line) in lines.into_iter().enumerate() {
            let y = y_offset + i as u16;
            if y >= area.y + area.height {
                break;
            }

            let mut x = x_offset;
            for span in line.spans {
                let content = span.content.clone();
                let style = span.style.patch(*STYLE_BACKGROUND);
                let width = span.width();
                buf.set_string(x, y, content, style);
                x += width as u16;
            }

        }
    }
}

fn build_start() -> Vec<Line<'static>> {
    const LOGO_LINES: &[&str] = &[
        "   ████████",
        "▄▄▄████████",
        "████       ",
        "▀▀▀████    ",
        "   ████    ",
        "           ",
    ];

    const TEXT_LINES: &[&str] = &[
        "                                                 ▄▄    ",
        "▄██▄▄ ▄▄   ▄▄ ▄▄▄▄▄   ▄▄▄▄   ▄▄▄▄  ▄ ▄▄▄   ▄▄▄▄  ██▄▄▄ ",
        "▀██▀▀ ▀██ ██▀ ██▀▀██ ██▀▀██ ██▀ ▀▀ ██▀▀██ ▀▀  ██ ██▀▀██",
        " ██▄   ▀███▀  ██  ██ ██▀▀▀  ██▄ ▄▄ ██     ▄█▀▀██ ██  ██",
        "  ▀▀  ▄▄██    ██▀▀▀   ▀▀▀▀▀  ▀▀▀▀  ▀▀     ▀▀▀▀▀▀ ▀▀▀▀▀ ",
        "      ▀▀▀     ▀▀                                       ",
    ];

    let logo_height = LOGO_LINES.len();
    let text_height = TEXT_LINES.len();

    let side_top_pad = if logo_height > text_height { (logo_height - text_height) / 2 } else { 0 };
    let total_height = logo_height.max(text_height);

    let mut lines = Vec::with_capacity(total_height + 2);

    for i in 0..total_height {
        // logo line
        let logo = LOGO_LINES.get(i).unwrap_or(&"");

        // text line
        let text_idx = i.checked_sub(side_top_pad).unwrap_or(usize::MAX);
        let text = TEXT_LINES.get(text_idx).unwrap_or(&"");

        let logo_span = Span::styled(*logo, Style::default().fg(*COLOR_ORANGE));
        let gap_span = Span::raw("  ");
        let text_span = Span::styled(*text, Style::default().fg(*COLOR_WHITE));

        lines.push(Line::from(vec![logo_span, gap_span, text_span]));
    }
    
    lines
}