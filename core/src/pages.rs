//! Page clean-up, before any reading of the text itself:
//!
//! * remove the running header ("THE DIRAC THEORY 6") at the top of a page
//!   and a lone page number at the bottom, remembering the *printed* page
//!   number (the index and the table of contents refer to those, not to the
//!   PDF's own page count);
//! * split two-column pages (like an index) into a left and a right column,
//!   so the left column is read before the right one.

use crate::layout::{make_line, plain_text, Doc, Line};

pub struct PageInfo {
    /// Index into `Doc::pages`.
    pub index: usize,
    /// Printed page number ("6", or "iv" in the front matter), if known.
    pub label: Option<String>,
    pub width: f32,
    pub height: f32,
    pub lines: Vec<Line>,
}

pub fn is_arabic(t: &str) -> bool {
    !t.is_empty() && t.len() <= 4 && t.bytes().all(|b| b.is_ascii_digit())
}

pub fn is_roman(t: &str) -> bool {
    !t.is_empty() && t.len() <= 7 && t.chars().all(|c| "ivxlc".contains(c))
}

pub fn is_page_number(t: &str) -> bool {
    is_arabic(t) || is_roman(t)
}

pub fn prepare(doc: &Doc) -> Vec<PageInfo> {
    let mut pages: Vec<PageInfo> = doc
        .pages
        .iter()
        .enumerate()
        .map(|(index, p)| {
            let mut lines = p.lines.clone();
            let mut label = None;

            // Running header: the first line, close to the top edge, with a
            // clear gap below it. Its first or last word may be the page number.
            if let Some(first) = lines.first() {
                let gap_ok = lines
                    .get(1)
                    .is_none_or(|n| n.top - first.top >= 2.0 * doc.body_size);
                if first.top < p.height * 0.09 && gap_ok {
                    let text = plain_text(first);
                    let words: Vec<&str> = text.split_whitespace().collect();
                    if words.len() <= 15 {
                        label = [words.first(), words.last()]
                            .into_iter()
                            .flatten()
                            .find(|w| is_page_number(w))
                            .map(|w| w.to_string());
                        lines.remove(0);
                    }
                }
            }
            // Footer: a lone page number near the bottom edge.
            if let Some(last) = lines.last() {
                let text = plain_text(last);
                if last.top > p.height * 0.88 && is_page_number(text.trim()) {
                    label.get_or_insert(text.trim().to_string());
                    lines.pop();
                }
            }
            let lines = split_columns(doc, lines, p.width);
            PageInfo { index, label, width: p.width, height: p.height, lines }
        })
        .collect();

    // A page without a number between numbered pages: count on from the
    // previous one.
    for i in 1..pages.len() {
        if pages[i].label.is_some() {
            continue;
        }
        if let Some(n) = pages[i - 1].label.as_ref().and_then(|l| l.parse::<u32>().ok()) {
            let next_ok = pages
                .get(i + 1)
                .and_then(|p| p.label.as_ref())
                .is_none_or(|l| l.parse::<u32>().ok() == Some(n + 2));
            if next_ok {
                pages[i].label = Some((n + 1).to_string());
            }
        }
    }
    pages
}

/// Find a vertical gap down the middle of the page that no line crosses but
/// many lines have text on both sides of. If there is one, split every line
/// at the gap and put the left column first.
fn split_columns(doc: &Doc, lines: Vec<Line>, width: f32) -> Vec<Line> {
    let mut best_both = 0;
    let mut best_xs: Vec<f32> = Vec::new();
    let mut x = width * 0.3;
    while x <= width * 0.7 {
        let mut crossing = false;
        let mut both = 0;
        for l in &lines {
            if l.spans.iter().any(|s| s.left < x && s.right() > x) {
                crossing = true;
                break;
            }
            let left = l.spans.iter().any(|s| s.right() <= x);
            let right = l.spans.iter().any(|s| s.left >= x);
            if left && right {
                both += 1;
            }
        }
        if !crossing && both >= 5 {
            if both > best_both {
                best_both = both;
                best_xs.clear();
            }
            if both == best_both {
                best_xs.push(x);
            }
        }
        x += 2.0;
    }
    if best_xs.is_empty() {
        return lines;
    }
    let gutter = best_xs[best_xs.len() / 2];
    let size_of = |s: &crate::layout::Span| doc.size_of(s);
    let mut left_col = Vec::new();
    let mut right_col = Vec::new();
    for l in lines {
        let (a, b): (Vec<_>, Vec<_>) = l.spans.into_iter().partition(|s| s.right() <= gutter);
        if !a.is_empty() {
            let mut line = make_line(a, &size_of);
            line.col = 1;
            left_col.push(line);
        }
        if !b.is_empty() {
            let mut line = make_line(b, &size_of);
            line.col = 2;
            right_col.push(line);
        }
    }
    left_col.extend(right_col);
    left_col
}
