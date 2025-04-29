/*
 * cli/src/tui/result.rs
 */

use once_cell::sync::Lazy;
use ratatui::{
    buffer::Buffer,
    layout::{
        Constraint,
        Direction,
        Layout,
        Rect
    },
    style::Style,
    symbols::Marker,
    text::{
        Line,
        Span
    },
    widgets::{
        Axis,
        Chart,
        Dataset,
        GraphType,
        Paragraph,
        Widget
    },
};
use std::collections::HashSet;
use core::results::FinalResults;

use crate::tui::scheme::{
    styled_block,
    COLOR_LIGHT,
    COLOR_RED,
    COLOR_WHITE,
    COLOR_ORANGE
};

// info block styles
static STYLE_INFO_LABEL: Lazy<Style> = Lazy::new(|| Style::default().fg(*COLOR_WHITE));
static STYLE_INFO_VALUE: Lazy<Style> = Lazy::new(|| Style::default().fg(*COLOR_ORANGE));

// errors block styles
static STYLE_KEY_OK: Lazy<Style> = Lazy::new(|| Style::default().fg(*COLOR_ORANGE));
static STYLE_KEY_ERR: Lazy<Style> = Lazy::new(|| Style::default().fg(*COLOR_RED));

// graph block styles
static STYLE_GRAPH_WPM: Lazy<Style> = Lazy::new(|| Style::default().fg(*COLOR_ORANGE));
static STYLE_GRAPH_RAW: Lazy<Style> = Lazy::new(|| Style::default().fg(*COLOR_LIGHT));
static STYLE_GRAPH_ERR: Lazy<Style> = Lazy::new(|| Style::default().fg(*COLOR_RED));


pub struct ResultView<'a> {
    pub results: &'a FinalResults,
}

impl<'a> Widget for ResultView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // 70% for graph, 30% for down part
        let parts_v = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(area);

        draw_graph(self.results, parts_v[0], buf);

        // 40% for info, 60% for errors
        let parts_h = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(parts_v[1]);

        draw_info(self.results, parts_h[0], buf);
        draw_keyboard(self.results, parts_h[1], buf);
    }
}

fn draw_graph(results: &FinalResults, area: Rect, buf: &mut Buffer) {
    if results.graph_data.is_empty() {
        Chart::default().block(styled_block("Graph")).render(area, buf);
        return;
    }

    let mut wpm_pts = Vec::<(f64, f64)>::new();
    let mut raw_pts = Vec::<(f64, f64)>::new();
    let mut err_pts = Vec::<(f64, f64)>::new();
    let mut prev_incorrect = 0;

    for (t, w, r, incorrect, ..) in &results.graph_data {
        wpm_pts.push((*t, *w));
        raw_pts.push((*t, *r));
        if *incorrect > prev_incorrect {
            err_pts.push((*t, *w));
        }
        prev_incorrect = *incorrect;
    }

    let datasets = vec![
        Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(*STYLE_GRAPH_RAW)
            .data(&raw_pts),
        Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(*STYLE_GRAPH_WPM)
            .data(&wpm_pts),
        Dataset::default()
            .marker(Marker::Dot)
            .graph_type(GraphType::Scatter)
            .style(*STYLE_GRAPH_ERR)
            .data(&err_pts),
    ];

    let x_max = results
        .graph_data
        .last()
        .map(|(t, ..)| t.ceil().max(1.0))
        .unwrap_or(1.0);

    let y_max = results.graph_data
        .iter()
        .map(|&(_, w, r, _, _, _)| w.max(r))
        .fold(0.0, f64::max)
        .ceil()
        .max(1.0);
    
    let y_top = ((y_max / 10.0).ceil() * 10.0) as u64;

    let x_labels: Vec<Line> = (0..=x_max as u64).map(|v| Line::from(v.to_string())).collect();
    let y_labels: Vec<Line> = (0..=y_top).step_by(10).map(|v| Line::from(v.to_string())).collect();

    Chart::new(datasets)
        .block(styled_block("Graph"))
        .x_axis(
            Axis::default()
                .bounds([0.0, x_max])
                .labels(x_labels)
                .title(Line::from("s")),    // ox
        )
        .y_axis(
            Axis::default()
                .bounds([0.0, y_top as f64])
                .labels(y_labels)
                .title(Line::from("wpm")),  // oy
        )
        .render(area, buf);
}

fn draw_info(results: &FinalResults, area: Rect, buf: &mut Buffer) {
    let k = &results.key_presses;

    // stat: value
    let rows = [
        ("WPM: ", results.wpm.round().to_string()),
        ("RAW: ", results.raw_wpm.round().to_string()),
        ("Accuracy: ", format!("{}%", results.accuracy.round() as u32)),
        ("Consistency: ", format!("{}%", results.consistency.round() as u32)),
        ("Characters: ", format!("{}/{}/{}/{}", k.correct, k.incorrect, k.extra, k.missed),),
    ];

    let lines: Vec<Line> = rows
        .iter()
        .map(|(label, value)| {
            Line::from(vec![
                Span::styled(*label, *STYLE_INFO_LABEL),
                Span::styled(value.clone(), *STYLE_INFO_VALUE),
            ])
        })
        .collect();

    Paragraph::new(lines)
        .block(styled_block("Info"))
        .render(area, buf);
}

fn draw_keyboard(results: &FinalResults, area: Rect, buf: &mut Buffer) {
    const KEYS: [&[&str]; 4] = [
        &["`", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "="],
        &["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P", "[", "]", "\\"],
        &["A", "S", "D", "F", "G", "H", "J", "K", "L", ";", "'"],
        &["Z", "X", "C", "V", "B", "N", "M", ",", ".", "/"],
    ];
    const SHIFTS: [usize; 4] = [0, 2, 4, 6];

    let error_keys: HashSet<char> = results
        .errors
        .iter()
        .map(|(c, _)| c.to_ascii_uppercase())
        .collect();

    // horizontal centering
    let row_lens: Vec<usize> = KEYS
        .iter()
        .enumerate()
        .map(|(i, r)| r.len() * 3 + SHIFTS[i])
        .collect();

    let max_row_len = *row_lens.iter().max().unwrap_or(&0);
    let base_left = (area.width as usize).saturating_sub(max_row_len) / 2;

    // vertical centering
    let inner_height = area.height.saturating_sub(2) as usize;

    let total_pad = inner_height.saturating_sub(KEYS.len());
    let top_pad    = total_pad / 2;
    let bottom_pad = total_pad - top_pad;

    let mut lines = Vec::<Line>::with_capacity(inner_height);

    for _ in 0..top_pad {
        lines.push(Line::default());
    }


    // keyboard rendering
    for (row_idx, row) in KEYS.iter().enumerate() {
        let key_spans: Vec<Span> = row
            .iter()
            .map(|key| {
                let style = if error_keys.contains(&key.chars().next().unwrap()) {
                    *STYLE_KEY_ERR
                } else {
                    *STYLE_KEY_OK
                };
                Span::styled(format!(" {} ", key), style)
            })
            .collect();

        let mut line_spans = Vec::<Span>::new();
        line_spans.push(Span::raw(" ".repeat(base_left + SHIFTS[row_idx])));
        line_spans.extend(key_spans);

        lines.push(Line::from(line_spans));
    }

    for _ in 0..bottom_pad {
        lines.push(Line::default());
    }

    Paragraph::new(lines)
        .block(styled_block("Errors"))
        .render(area, buf);
}

