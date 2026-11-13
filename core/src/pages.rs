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

/// A page number's value, and whether it is in Roman numerals.
fn number_of(label: &str) -> Option<(u32, bool)> {
    if let Ok(n) = label.parse::<u32>() {
        return Some((n, false));
    }
    if !is_roman(label) {
        return None;
    }
    let value = |c: char| match c {
        'i' => 1,
        'v' => 5,
        'x' => 10,
        'l' => 50,
        _ => 100,
    };
    let digits: Vec<u32> = label.chars().map(value).collect();
    let n = digits.iter().enumerate().map(|(k, d)| if digits.get(k + 1).is_some_and(|next| next > d) { -(*d as i64) } else { *d as i64 }).sum::<i64>();
    // (only a numeral written as it should be: "iiii" or "vx" is no number)
    u32::try_from(n).ok().filter(|n| to_roman(*n) == label).map(|n| (n, true))
}

fn to_roman(mut n: u32) -> String {
    let mut out = String::new();
    for (value, numeral) in [(100, "c"), (90, "xc"), (50, "l"), (40, "xl"), (10, "x"), (9, "ix"), (5, "v"), (4, "iv"), (1, "i")] {
        while n >= value {
            out.push_str(numeral);
            n -= value;
        }
    }
    out
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
                // (not a lone "i" or "v" at the foot of a page, the end of
                // a formula, in the part of the book numbered in figures:
                // the page before has such a number in its running head)
                let figures_before = index > 0
                    && doc.pages[index - 1].lines.first().is_some_and(|h| {
                        let head = plain_text(h);
                        h.top < p.height * 0.09 && [head.split_whitespace().next(), head.split_whitespace().last()].into_iter().flatten().any(is_arabic)
                    });
                if last.top > p.height * 0.88 && is_page_number(text.trim()) && !(is_roman(text.trim()) && figures_before) {
                    label.get_or_insert(text.trim().to_string());
                    lines.pop();
                }
            }
            let lines = split_columns(doc, lines, p.width);
            PageInfo { index, label, width: p.width, height: p.height, lines }
        })
        .collect();

    // The pages before the first page numbered in figures, back to page 1
    // (a part's title page, an empty page and the page the first chapter
    // opens on come after pages in Roman numerals): count back.
    for i in (0..pages.len().saturating_sub(1)).rev() {
        if pages[i].label.is_some() {
            continue;
        }
        if let Some((n, false)) = pages[i + 1].label.as_deref().and_then(number_of).filter(|(n, _)| *n > 1) {
            pages[i].label = Some((n - 1).to_string());
        }
    }
    // A page without a number after a numbered page (the page a chapter
    // opens on has no running head): count on from the page before, in
    // figures or in Roman numerals as that one is numbered.
    for i in 1..pages.len() {
        if pages[i].label.is_some() {
            continue;
        }
        let Some((n, roman)) = pages[i - 1].label.as_deref().and_then(number_of) else { continue };
        let next_ok = pages.get(i + 1).and_then(|p| p.label.as_deref()).is_none_or(|l| number_of(l) == Some((n + 2, roman)) || (roman && number_of(l) == Some((1, false))));
        // (after the last page in Roman numerals comes page 1)
        if next_ok {
            pages[i].label = Some(if roman { to_roman(n + 1) } else { (n + 1).to_string() });
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
    // Titles at the left and their page numbers at the right margin (a
    // table of contents without dots between them) are lines, not columns.
    let (mut both, mut numbers) = (0, 0);
    for l in &lines {
        let right: Vec<&str> = l.spans.iter().filter(|s| s.left >= gutter).map(|s| s.text.trim()).collect();
        if !right.is_empty() && l.spans.iter().any(|s| s.right() <= gutter) {
            both += 1;
            numbers += usize::from(right.len() == 1 && is_page_number(right[0]));
        }
    }
    if numbers * 5 >= both * 4 {
        return lines;
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roman_numbers() {
        assert_eq!(number_of("xxiv"), Some((24, true)));
        assert_eq!(number_of("17"), Some((17, false)));
        assert_eq!(number_of("vx"), None);
        assert_eq!(to_roman(29), "xxix");
    }
}
