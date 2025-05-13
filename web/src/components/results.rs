use dioxus::prelude::*;
use std::collections::HashSet;
use typingcore::results::FinalResults;
use web_sys::{console};

fn x_axis_labels(x_max: f64) -> Vec<Element> {
    let step = if x_max >= 15.0 { 5 } else { 1 };
    (0..=(x_max as u64)).step_by(step).map(|i| {
        let x = 10.0 + (i as f64 / x_max * 180.0); // Scale to fit 180 units
        rsx!(
            text {
                x: "{x}",
                y: "75", // Adjusted from 95
                class: "axis-label",
                "{i}"
            }
        )
    }).collect()
}

fn y_axis_labels(y_top: u64) -> Vec<Element> {
    (0..=y_top).step_by(10).map(|i| {
        let y = 70.0 - (i as f64 / y_top as f64 * 60.0); // Scale to fit 60 units (70 - 10 top)
        rsx!(
            text {
                x: "8",
                y: "{y}",
                class: "axis-label",
                text_anchor: "end",
                "{i}"
            }
        )
    }).collect()
}

fn raw_wpm_polyline(raw_pts: &Vec<(f64, f64)>, x_max: f64, y_top: u64) -> Option<Element> {
    if raw_pts.is_empty() {
        return None;
    }

    let point_str = raw_pts.iter().map(|(t, w)| {
        let x = 10.0 + (t / x_max * 180.0); // Scale to 180 units
        let y = 70.0 - (w / y_top as f64 * 60.0); // Scale to 60 units
        format!("{},{}", x, y)
    }).collect::<Vec<_>>().join(" ");

    Some(rsx!(
        polyline {
            points: "{point_str}",
            class: "raw-wpm-line",
        }
    ))
}

fn wpm_polyline(wpm_pts: &Vec<(f64, f64)>, x_max: f64, y_top: u64) -> Option<Element> {
    if wpm_pts.is_empty() {
        return None;
    }

    let point_str = wpm_pts.iter().map(|(t, w)| {
        let x = 10.0 + (t / x_max * 180.0); // Scale to 180 units
        let y = 70.0 - (w / y_top as f64 * 60.0); // Scale to 60 units
        format!("{},{}", x, y)
    }).collect::<Vec<_>>().join(" ");

    Some(rsx!(
        polyline {
            points: "{point_str}",
            class: "wpm-line",
        }
    ))
}

fn error_point_circles(err_pts: &Vec<(f64, f64)>, x_max: f64, y_top: u64) -> Vec<Element> {
    err_pts.iter().map(|(t, w)| {
        let x = 10.0 + (t / x_max * 180.0); // Scale to 180 units
        let y = 70.0 - (w / y_top as f64 * 60.0); // Scale to 60 units
        rsx!(
            circle {
                cx: "{x}",
                cy: "{y}",
                r: "0.8", // Slightly smaller circle
                class: "error-point"
            }
        )
    }).collect()
}

#[derive(Props, Clone, PartialEq)]
pub struct ResultsProps {
    pub results: FinalResults,
}

#[component]
pub fn Results(props: ResultsProps) -> Element {
    let results = &props.results;

    let mut wpm_pts = vec![];
    let mut raw_pts = vec![];
    let mut err_pts = vec![];
    let mut prev_incorrect = 0;

    for (t, w, r, incorrect, _, _) in &results.graph_data {
        wpm_pts.push((*t, *w));
        raw_pts.push((*t, *r));
        if *incorrect > prev_incorrect {
            err_pts.push((*t, *w));
        }
        prev_incorrect = *incorrect;
    }

    let x_max = results.graph_data.last().map(|(t, ..)| t.ceil().max(1.0)).unwrap_or(1.0);
    let y_max = results
        .graph_data
        .iter()
        .map(|&(_, w, r, _, _, _)| w.max(r))
        .fold(0.0, f64::max)
        .ceil()
        .max(1.0);

    let y_top = ((y_max / 10.0).ceil() * 10.0) as u64;

    let error_keys: HashSet<char> = results.errors.iter().map(|(c, _)| c.to_ascii_uppercase()).collect();

    const KEYS: [[&str; 13]; 4] = [
        ["`", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "="],
        ["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P", "[", "]", "\\"],
        ["A", "S", "D", "F", "G", "H", "J", "K", "L", ";", "'", "", ""],
        ["Z", "X", "C", "V", "B", "N", "M", ",", ".", "/", "", "", ""],
    ];
    const SHIFTS: [usize; 4] = [0, 5, 6, 7];

    rsx! {
        div {
            class: "results-container",
            // Graph section
            div {
                class: "graph-section",
                div {
                    class: "graph-panel",
                    svg {
                        class: "graph-svg",
                        view_box: "0 0 200 80", // Changed from 200 100 to make the graph shorter
                        preserve_aspect_ratio: "xMidYMid meet",
                        line { x1: "10", y1: "70", x2: "190", y2: "70", stroke: "white", stroke_width: "0.5" } // Adjusted Y-axis
                        line { x1: "10", y1: "70", x2: "10", y2: "10", stroke: "white", stroke_width: "0.5" } // Adjusted Y-axis
                        {x_axis_labels(x_max).into_iter()}
                        {y_axis_labels(y_top).into_iter()}
                        {raw_wpm_polyline(&raw_pts, x_max, y_top).into_iter()}
                        {wpm_polyline(&wpm_pts, x_max, y_top).into_iter()}
                        {error_point_circles(&err_pts, x_max, y_top).into_iter()}
                        text { x: "195", y: "75", class: "axis-title", "sec" } // Adjusted Y position
                        text { x: "5", y: "5", class: "axis-title", "wpm" }
                    }
                }
            }
            // Bottom section
            div {
                class: "bottom-section",
                // Info section
                div {
                    class: "info-section",
                    div {
                        class: "info-panel",
                        div {
                            class: "info-content",
                            div {
                                class: "info-row",
                                span { class: "info-label", "wpm: " }
                                span { class: "info-value", "{results.wpm.round()}" }
                            }
                            div {
                                class: "info-row",
                                span { class: "info-label", "raw: " }
                                span { class: "info-value", "{results.raw_wpm.round()}" }
                            }
                            div {
                                class: "info-row",
                                span { class: "info-label", "accuracy: " }
                                span { class: "info-value", "{results.accuracy.round()}%" }
                            }
                            div {
                                class: "info-row",
                                span { class: "info-label", "consistency: " }
                                span { class: "info-value", "{results.consistency.round()}%" }
                            }
                            div {
                                class: "info-row tooltip",
                                span { class: "tooltiptext", "correct/incorrect/extra/missed" } 
                                span { class: "info-label", "characters: " } 
                                span { class: "info-value",
                                    "{results.key_presses.correct}/{results.key_presses.incorrect}/{results.key_presses.extra}/{results.key_presses.missed}"
                                }
                            }
                        }
                    }
                }
                // Keyboard section
                div {
                    class: "keyboard-section",
                    div {
                        class: "keyboard-panel",
                        div {
                            class: "keyboard-content",
                            for (row_idx , row) in KEYS.iter().enumerate() {
                                div {
                                    class: "keyboard-row",
                                    style: "margin-left: {SHIFTS[row_idx]}rem;",
                                    for key in row.iter().filter(|&&k| !k.is_empty()) {
                                        div {
                                            class: "key",
                                            class: if error_keys.contains(&key.chars().next().unwrap_or(' ')) { "key-error" } else { "key-normal" },
                                            "{key}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
