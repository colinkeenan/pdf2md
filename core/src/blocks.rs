//! Lines -> blocks: headings, paragraphs, displayed equations, table of
//! contents, index lines and page-bottom footnotes.
//!
//! The decisions use the layout, measured against the book's own typical
//! values (see [`Stats`]):
//!
//! * **heading**: bigger than body text, or a short line that is entirely
//!   italic/bold, starts at the left margin and has extra space above it;
//! * **displayed equation**: mostly math-font text and indented well in
//!   from the margin, or carrying an equation number "(16)" at the right
//!   margin, plus the rows of fractions and limits around such lines;
//! * **new paragraph**: first-line indent, extra vertical space, or the
//!   previous line ended a sentence well short of the right margin.

use crate::layout::{plain_text, Doc, Line, Rule, Script};
use crate::mathtex::{self, MSpan};
use crate::pages::{is_page_number, PageInfo};
use crate::{key, plain, Blk, Block, Inline, TocEntry};
use std::collections::{HashMap, HashSet};

/// Typical measurements of the book's body text.
#[derive(Debug, Clone)]
pub struct Stats {
    /// Left and right text margins.
    pub left: f32,
    pub right: f32,
    /// Body font size.
    pub body: f32,
    /// Distance between the tops of consecutive body lines.
    pub spacing: f32,
    /// Font size of chapter titles, if there is a clear one.
    pub chapter_size: Option<f32>,
    /// Pages that hold the table of contents.
    pub toc_pages: HashSet<usize>,
    /// Hyphenated words seen inside lines ("wave-function"), used to decide
    /// whether a hyphen at a line end is part of the word.
    pub hyphenated: HashSet<String>,
}

/// The most common value (rounded to whole pixels).
fn mode(values: impl Iterator<Item = f32>) -> Option<f32> {
    let mut counts: HashMap<i32, usize> = HashMap::new();
    for v in values {
        *counts.entry(v.round() as i32).or_default() += 1;
    }
    counts
        .into_iter()
        .max_by_key(|&(v, c)| (c, v))
        .map(|(v, _)| v as f32)
}

/// A line set in a larger font whose large text is real words (not a big
/// bracket or summation sign from a displayed equation).
fn big_text_line(doc: &Doc, l: &Line, body: f32) -> bool {
    l.size >= 1.12 * body
        && l.spans
            .iter()
            .filter(|s| doc.size_of(s) >= l.size - 0.5)
            .all(|s| !doc.is_math(s))
        && l.spans.iter().any(|s| {
            // at least one real word (diagram labels like "x₁ x₂" are not titles)
            doc.size_of(s) >= l.size - 0.5
                && s.text
                    .split_whitespace()
                    .any(|w| w.chars().filter(|c| c.is_alphabetic()).count() >= 3)
        })
}

fn is_contents_title(t: &str) -> bool {
    matches!(key(t).as_str(), "contents" | "tableofcontents")
}

pub fn stats(doc: &Doc, pages: &[PageInfo]) -> Stats {
    let body = doc.body_size;
    let full = |p: &PageInfo, l: &Line| l.size == body && l.right - l.left > 0.5 * p.width;
    let left = mode(pages.iter().flat_map(|p| p.lines.iter().filter(|l| full(p, l)).map(|l| l.left)))
        .unwrap_or(0.0);
    let right = mode(pages.iter().flat_map(|p| p.lines.iter().filter(|l| full(p, l)).map(|l| l.right)))
        .unwrap_or(0.0);
    let spacing = mode(pages.iter().flat_map(|p| {
        p.lines.windows(2).filter_map(move |w| {
            let d = w[1].top - w[0].top;
            (w[0].size == body && w[1].size == body && d >= body && d <= 3.0 * body).then_some(d)
        })
    }))
    .unwrap_or(body * 1.2);

    // Table of contents: the page with a "Contents" title and the pages right
    // after it that are still mostly "title .... page" lines.
    let mut toc_pages = HashSet::new();
    let mut in_toc = false;
    for (pi, p) in pages.iter().enumerate() {
        let starts = p.lines.iter().any(|l| is_contents_title(&plain_text(l)));
        let entries = p.lines.iter().filter(|l| toc_split(&plain_text(l)).is_some() || link_target(l).is_some()).count();
        let mostly = entries * 2 >= p.lines.len() && entries >= 3;
        in_toc = (starts && entries > 0) || (in_toc && mostly);
        if in_toc {
            toc_pages.insert(pi);
        }
    }

    // Chapter titles: the most common "big" font size outside the TOC.
    // But when some big size has titles that start with a bare chapter
    // number ("3 Theory of Angular Momentum", or "3" on a line of its own
    // above the title), the largest such size is the
    // chapter size, even if numbered subsections ("3.1.2 …") are more common.
    let mut counts: HashMap<i32, usize> = HashMap::new();
    let mut numbered: HashMap<i32, usize> = HashMap::new();
    for (pi, p) in pages.iter().enumerate() {
        if toc_pages.contains(&pi) {
            continue;
        }
        for l in &p.lines {
            let k = (l.size * 10.0).round() as i32;
            let text: String = l.spans.iter().map(|s| s.text.as_str()).collect::<Vec<_>>().join(" ");
            let mut w = text.split_whitespace();
            let number = w.next().is_some_and(|n| n.len() <= 2 && n.chars().all(|c| c.is_ascii_digit()));
            let next = w.next();
            let big = big_text_line(doc, l, body);
            if big {
                *counts.entry(k).or_default() += 1;
            }
            // "3 Theory of …", or the "3" alone on its own line
            let alone = next.is_none() && l.size >= 1.12 * body && l.spans.iter().all(|s| !doc.is_math(s));
            if number && (alone || (big && next.is_some_and(|t| t.chars().next().is_some_and(char::is_uppercase)))) {
                *numbered.entry(k).or_default() += 1;
            }
        }
    }
    let chapter_size = numbered
        .iter()
        .filter(|&(_, &c)| c >= 2)
        .map(|(&s, _)| s)
        .max()
        .or_else(|| counts.into_iter().filter(|&(_, c)| c >= 2).max_by_key(|&(s, c)| (c, s)).map(|(s, _)| s))
        .map(|s| s as f32 / 10.0);

    let mut hyphenated = HashSet::new();
    for p in &doc.pages {
        for l in &p.lines {
            for s in &l.spans {
                for w in s.text.split_whitespace() {
                    let w = w.trim_matches(|c: char| !c.is_alphanumeric());
                    if let Some((a, b)) = w.split_once('-') {
                        if !a.is_empty() && !b.is_empty() {
                            hyphenated.insert(w.to_lowercase());
                        }
                    }
                }
            }
        }
    }

    Stats { left, right, body, spacing, chapter_size, toc_pages, hyphenated }
}

/// "Subject Matter . . . . 1" -> ("Subject Matter", "1").
/// The equation number in a reference: "(3.422)", "(3.422", "3.425b" ->
/// "3.422" … (chapter or appendix letter, dot, number, optional letter).
pub fn equation_ref(t: &str) -> Option<&str> {
    let t = t.trim().trim_start_matches(['[', '(']).trim_end_matches([')', ']', ',', ';', '.']);
    let (a, b) = t.split_once('.')?;
    let b = b.trim_end_matches(|c: char| c.is_ascii_lowercase());
    (!a.is_empty() && a.len() <= 2 && a.chars().all(|c| c.is_ascii_alphanumeric()) && !b.is_empty() && b.chars().all(|c| c.is_ascii_digit()))
        .then_some(t)
}

/// The PDF page a line links to, if its text is a link (a linked table of
/// contents entry): "@N", standing in for a printed page number.
fn link_target(l: &Line) -> Option<String> {
    let has_words = l.spans.iter().any(|s| s.link.is_some() && s.text.chars().filter(|c| c.is_alphabetic()).count() >= 2);
    let n = l.spans.iter().find_map(|s| s.link)?;
    has_words.then(|| format!("@{n}"))
}

fn toc_split(t: &str) -> Option<(String, String)> {
    let t = t.trim();
    let (rest, last) = t.rsplit_once(char::is_whitespace)?;
    if !is_page_number(last) {
        return None;
    }
    let title = rest.trim_end_matches(|c: char| c == '.' || c.is_whitespace());
    (title.chars().any(|c| c.is_alphabetic())).then(|| (title.to_string(), last.to_string()))
}

// ---------------------------------------------------------------------------
// Inline text

fn push_text(out: &mut Vec<Inline>, t: &str) {
    if let Some(Inline::Text(prev)) = out.last_mut() {
        prev.push_str(t);
    } else {
        out.push(Inline::Text(t.to_string()));
    }
}

/// Collapse runs of spaces and trim the ends.
fn tidy(mut v: Vec<Inline>) -> Vec<Inline> {
    // "$q_0 \not$= 0": the slash of ≠ ended the formula and its "=" begins
    // the text after it
    for k in 1..v.len() {
        let (a, b) = v.split_at_mut(k);
        if let (Inline::Math(m), Inline::Text(t)) = (&mut a[k - 1], &mut b[0]) {
            if m.trim_end().ends_with("\\not") && t.starts_with('=') {
                *m = format!("{}\\neq", m.trim_end().trim_end_matches("\\not"));
                t.remove(0);
                if !t.starts_with(' ') {
                    t.insert(0, ' ');
                }
            }
        }
    }
    for i in v.iter_mut() {
        if let Inline::Text(t) = i {
            let mut s = String::with_capacity(t.len());
            let mut space = false;
            for c in t.chars() {
                if c.is_whitespace() {
                    space = true;
                } else {
                    if space {
                        s.push(' ');
                    }
                    space = false;
                    s.push(c);
                }
            }
            if space {
                s.push(' ');
            }
            *t = s;
        }
    }
    if let Some(Inline::Text(t)) = v.first_mut() {
        *t = t.trim_start().to_string();
    }
    if let Some(Inline::Text(t)) = v.last_mut() {
        *t = t.trim_end().to_string();
    }
    v.retain(|i| !matches!(i, Inline::Text(t) if t.is_empty()));
    v
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum K {
    Text,
    Math,
    Op,
    Marker,
}

/// One line of prose -> inlines. Runs of math-font spans (with the
/// operators and digits between them) become `$...$`; a small raised number
/// glued to a word is a footnote-marker candidate.
pub fn line_inlines(doc: &Doc, l: &Line, st: &Stats) -> Vec<Inline> {
    // A text span like ") (by definition)" starts with the closing bracket
    // of the formula before it, and "the (" ends with an opening one: split
    // those brackets off so they can join the formula.
    let mut pieces: Vec<crate::layout::Span> = Vec::with_capacity(l.spans.len());
    for s in &l.spans {
        let t = &s.text;
        let n = t.chars().count().max(1) as f32;
        let math = doc.is_math(s);
        let lead = t.starts_with([')', ']']) && t.chars().count() > 1 && !math;
        // ... and ". Likewise ˜" ends with the accent of the symbol after it
        let trail = t.ends_with(['(', '[', '˜', 'ˆ', '˙', '¨', 'ˇ']) && t.chars().count() > 1 && !math;
        if lead || trail {
            let cut = if lead { 1 } else { t.chars().count() - 1 };
            let a: String = t.chars().take(cut).collect();
            let b: String = t.chars().skip(cut).collect();
            let split_x = s.left + s.width * cut as f32 / n;
            let mut sa = s.clone();
            sa.text = a;
            sa.width = split_x - s.left;
            let mut sb = s.clone();
            sb.text = b;
            sb.left = split_x;
            sb.width = s.right() - split_x;
            pieces.push(sa);
            pieces.push(sb);
        } else {
            pieces.push(s.clone());
        }
    }
    // Glue touching spans of the same font and script ("9" + "0" = "90").
    let mut spans: Vec<crate::layout::Span> = Vec::with_capacity(pieces.len());
    for s in &pieces {
        if let Some(p) = spans.last_mut() {
            let digits = |t: &str| !t.is_empty() && t.chars().all(|c| c.is_ascii_digit());
            let joinable = s.script != Script::Normal || (digits(&p.text) && digits(&s.text));
            if joinable && p.font == s.font && p.script == s.script && (s.left - p.right()).abs() < 1.0 {
                p.text.push_str(&s.text);
                p.width = s.right() - p.left;
                continue;
            }
        }
        spans.push(s.clone());
    }
    let spans = &spans;
    let n = spans.len();
    let gap = |a: usize, b: usize| spans[b].left - spans[a].right();
    let mut kinds: Vec<K> = (0..n)
        .map(|i| {
            let s = &spans[i];
            let math = doc.is_math(s);
            let t = s.text.trim();
            if s.script == Script::Super && !math {
                let after_word = i > 0
                    && !doc.is_math(&spans[i - 1])
                    && spans[i - 1].script == Script::Normal
                    && gap(i - 1, i) < 0.5 * st.body;
                let stacked = spans
                    .get(i + 1)
                    .is_some_and(|nx| nx.script == Script::Sub && (nx.left - s.left).abs() < 3.0);
                let digits = !t.is_empty() && t.len() <= 3 && t.bytes().all(|b| b.is_ascii_digit());
                let symbol = !t.is_empty() && t.chars().all(|c| "*†‡§¶".contains(c));
                let ordinal = matches!(t, "st" | "nd" | "rd" | "th");
                if ((digits || symbol) && (after_word || i == 0) && !stacked) || (ordinal && i > 0) {
                    return K::Marker;
                }
            }
            // an accent drawn over a math symbol (˜ over a) is part of it
            let accent_over_math = matches!(t, "˜" | "ˆ" | "˙" | "¨" | "¯" | "ˇ")
                && spans.iter().any(|o| doc.is_math(o) && o.left < s.right() && o.right() > s.left);
            if math || s.script != Script::Normal || accent_over_math {
                K::Math
            } else if !t.is_empty() && t.chars().all(|c| c.is_ascii_digit() || "=+−-–()[]/<>|,.;:·×*!′".contains(c)) {
                K::Op
            } else {
                K::Text
            }
        })
        .collect();
    // Digits/operators touching math belong to the formula.
    loop {
        let mut changed = false;
        for i in 0..n {
            if kinds[i] != K::Op {
                continue;
            }
            // "=", "+" ... may be set off by a full word space.
            let t = spans[i].text.trim();
            let operator = t.starts_with(['=', '+', '−', '-', '×', '<', '>', '≈', '≡', '·'])
                || t.ends_with(['=', '+', '−', '-', '×', '<', '>', '≈', '≡', '·']);
            let reach = if operator { 0.5 } else { 0.25 } * st.body;
            let touch_left = i > 0 && kinds[i - 1] == K::Math && gap(i - 1, i) < reach;
            let touch_right = i + 1 < n && kinds[i + 1] == K::Math && gap(i, i + 1) < reach;
            if touch_left || touch_right {
                kinds[i] = K::Math;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    // Trailing punctuation stays outside the formula.
    for i in 0..n {
        let t = spans[i].text.trim();
        let ends_run = i + 1 == n || kinds[i + 1] != K::Math;
        if kinds[i] == K::Math
            && ends_run
            && !doc.is_math(&spans[i])
            && spans[i].script == Script::Normal
            && !t.is_empty()
            && t.chars().all(|c| ".,;:".contains(c))
        {
            kinds[i] = K::Text;
        }
    }

    let mut out: Vec<Inline> = Vec::new();
    let mut prev_right: Option<f32> = None;
    let mut i = 0;
    while i < n {
        let s = &spans[i];
        let space = prev_right.is_some_and(|r| s.left - r > 0.15 * st.body);
        match kinds[i] {
            K::Math => {
                let mut j = i;
                while j < n && kinds[j] == K::Math {
                    j += 1;
                }
                let ms: Vec<MSpan> = spans[i..j].iter().map(|s| MSpan::new(doc, s)).collect();
                // stray bracket glyphs alone in a line of prose say nothing
                let only_brackets = ms.iter().all(|m| m.family.starts_with("CMEX") && m.text.trim().is_empty());
                let x0 = spans[i..j].iter().map(|s| s.left).fold(f32::MAX, f32::min);
                let x1 = spans[i..j].iter().map(|s| s.right()).fold(f32::MIN, f32::max);
                let bars: Vec<Rule> =
                    l.bars.iter().filter(|b| b.left >= x0 - 3.0 && b.right <= x1 + 3.0).copied().collect();
                let t = if only_brackets { String::new() } else { mathtex::build_with(&ms, &bars) };
                if space {
                    push_text(&mut out, " ");
                }
                if !t.is_empty() {
                    // one equation picture: say which (a correction is kept)
                    if let [one] = &spans[i..j] {
                        if let Some(k) = &one.image_key {
                            out.push(Inline::Source(k.clone()));
                        }
                    }
                    out.push(Inline::Math(t));
                }
                prev_right = Some(spans[j - 1].right());
                i = j;
                continue;
            }
            K::Marker => {
                out.push(Inline::Sup(s.text.trim().to_string()));
                // a marker that links to its note's page (an ebook's notes
                // at the end of the chapter)
                if let Some(n) = s.link {
                    out.push(Inline::PageLink(n));
                }
            }
            // a web link (before bold/italic: a bold address is a link too)
            _ if s.url.is_some() && s.url_text.as_deref().is_some_and(|t| t.chars().any(char::is_alphanumeric)) => {
                let href = s.url.clone().unwrap_or_default();
                let link = s.url_text.clone().unwrap_or_default();
                let link = link.trim();
                // words of the span outside the link stay plain text
                let (before, after) = match s.text.find(link) {
                    Some(k) => (&s.text[..k], &s.text[k + link.len()..]),
                    None => ("", ""),
                };
                let continues = matches!(out.last(), Some(Inline::Link { href: h, .. }) if *h == href) && before.trim().is_empty();
                if continues {
                    // the same address over several pieces (a line break inside it)
                    if let Some(Inline::Link { text: t, .. }) = out.last_mut() {
                        t.push_str(&crate::emit::escape_text(link));
                    }
                } else {
                    if space {
                        push_text(&mut out, " ");
                    }
                    if !before.trim().is_empty() {
                        push_text(&mut out, before);
                    }
                    out.push(Inline::Link { text: crate::emit::escape_text(link), href });
                }
                if !after.is_empty() {
                    push_text(&mut out, after);
                }
            }
            _ if s.bold || crate::layout::is_bold_font(doc.family(s)) => {
                // bold words: one `**..**` piece for a whole bold phrase
                if let Some(Inline::Strong(e)) = out.last_mut() {
                    if space {
                        e.push(' ');
                    }
                    e.push_str(&s.text);
                } else {
                    if space {
                        push_text(&mut out, " ");
                    }
                    out.push(Inline::Strong(s.text.clone()));
                }
            }
            _ if s.italic => {
                // italic words: one `_..._` piece for a whole italic phrase
                if let Some(Inline::Emph(e)) = out.last_mut() {
                    if space {
                        e.push(' ');
                    }
                    e.push_str(&s.text);
                } else {
                    if space {
                        push_text(&mut out, " ");
                    }
                    out.push(Inline::Emph(s.text.clone()));
                }
            }
            _ if s.picture.is_some() => {
                if space {
                    push_text(&mut out, " ");
                }
                let area = crate::layout::Figure { left: s.left, right: s.right(), top: s.top, bottom: s.bottom() };
                out.push(Inline::Picture { page: s.picture.unwrap_or(0), area });
            }
            _ => {
                if space {
                    push_text(&mut out, " ");
                }
                push_text(&mut out, &s.text);
                // text that links into the book: a page number in a linked
                // index, a reference to an equation "(3.422)", a section,
                // a figure … (see links.rs)
                if let Some(n) = s.link {
                    out.push(Inline::PageLink(n));
                }
            }
        }
        prev_right = Some(s.right());
        i += 1;
    }
    tidy(out)
}

// ---------------------------------------------------------------------------
// Tables

/// Runs of 3+ consecutive lines laid out in the same columns, with words in
/// the last column (a list like "A* : complex conjugate"). Returns
/// (first line, end, column starts).
///
/// A column is an x position where every row of the run has a span
/// starting (within a few pixels) and no span runs across it. Lines of prose
/// never qualify: their text runs across every position. Matrices are
/// excluded because their last column has no words.
fn find_tables(doc: &Doc, lines: &[Line], st: &Stats) -> Vec<(usize, usize, Vec<f32>)> {
    let columns = |run: &[Line]| -> Vec<f32> {
        let mut cols: Vec<f32> = Vec::new();
        for s in &run[0].spans {
            let x = s.left;
            if cols.last().is_some_and(|&c| x - c < st.body) {
                continue;
            }
            let ok = run.iter().all(|l| {
                let starts = l.spans.iter().any(|t| (t.left - x).abs() <= 6.0);
                let crosses = l.spans.iter().any(|t| t.left < x - 6.0 && t.right() > x + 2.0);
                starts && !crosses
            });
            if ok {
                cols.push(x);
            }
        }
        cols
    };
    let words_in_last = |l: &Line, cols: &[f32]| {
        let last = *cols.last().unwrap();
        l.spans.iter().any(|s| {
            s.left >= last - 6.0
                && !doc.is_math(s)
                && s.text.split_whitespace().any(|w| w.chars().filter(|c| c.is_alphabetic()).count() >= 3)
        })
    };
    let mut out = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        // lines starting at the same x, one under the other
        let mut j = i + 1;
        while j < lines.len()
            && (lines[j].left - lines[i].left).abs() <= 3.0
            && lines[j].top - lines[j - 1].top <= 1.6 * st.spacing
        {
            j += 1;
        }
        // numbered notes ("²³ text") are entries, not a table
        let notes = lines[i..j].iter().any(|l| l.spans.first().is_some_and(|s| s.script == Script::Super));
        if j - i >= 3 && !notes {
            let cols = columns(&lines[i..j]);
            let rows = j - i;
            // Prose runs to the right margin; a table's rows mostly stop short.
            let short = lines[i..j].iter().filter(|l| l.right < st.right - 3.0 * st.body).count();
            // The first column holds short items ("A*", "1.", "Ry").
            let narrow_first = cols.len() >= 2 && cols[1] - cols[0] <= 0.3 * (st.right - st.left);
            if narrow_first
                && short * 3 >= rows * 2
                && lines[i..j].iter().filter(|l| words_in_last(l, &cols)).count() * 2 >= rows
            {
                out.push((i, j, cols));
                i = j;
                continue;
            }
        }
        i += 1;
    }
    out
}

/// Escape text for `\text{..}` in LaTeX.
fn latex_text(t: &str) -> String {
    let mut s = String::new();
    for c in t.chars() {
        match c {
            '\\' => s.push_str("\\backslash "),
            '{' | '}' | '%' | '#' | '&' | '$' | '_' => {
                s.push('\\');
                s.push(c);
            }
            c => s.push(c),
        }
    }
    s
}

/// A table as a LaTeX `array`: text cells become `\text{..}`, the rest math.
fn table_latex(doc: &Doc, lines: &[Line], cols: &[f32]) -> String {
    let spec = if cols.len() >= 3 {
        format!("l{}l", "c".repeat(cols.len() - 2))
    } else {
        "l".repeat(cols.len())
    };
    let rows: Vec<String> = lines
        .iter()
        .map(|l| {
            let cells: Vec<String> = (0..cols.len())
                .map(|c| {
                    let lo = cols[c] - 6.0;
                    let hi = cols.get(c + 1).map_or(f32::MAX, |x| x - 6.0);
                    let spans: Vec<&crate::layout::Span> =
                        l.spans.iter().filter(|s| s.left >= lo && s.left < hi).collect();
                    let plain_text = spans.iter().all(|s| !doc.is_math(s) && s.script == Script::Normal && !s.overline)
                        && spans.iter().any(|s| s.text.chars().any(|c| c.is_alphabetic()));
                    if plain_text {
                        let t: Vec<&str> = spans.iter().map(|s| s.text.trim()).collect();
                        format!("\\text{{{}}}", latex_text(&t.join(" ")))
                    } else {
                        let ms: Vec<MSpan> = spans.iter().map(|s| MSpan::new(doc, s)).collect();
                        mathtex::build(&ms)
                    }
                })
                .collect();
            cells.join(" & ")
        })
        .collect();
    format!("\\begin{{array}}{{{spec}}}\n{} \\\\\n\\end{{array}}", rows.join(" \\\\\n"))
}

// ---------------------------------------------------------------------------
// Line classification

/// (share of characters that are math, number of ordinary words, has an
/// equation number at the right margin)
fn math_info(doc: &Doc, l: &Line, st: &Stats) -> (f32, usize, bool) {
    // Math share is measured against letters of ordinary text, so a
    // centred row of plain numbers (a matrix row) counts as math.
    let mut letters = 0usize;
    let mut mathc = 0usize;
    let mut other = 0usize;
    let mut words = 0usize;
    for s in &l.spans {
        let c = s.text.chars().filter(|c| !c.is_whitespace()).count();
        if doc.is_math(s) || s.script != Script::Normal {
            mathc += c;
        } else {
            letters += s.text.chars().filter(|c| c.is_alphabetic()).count();
            other += s.text.chars().filter(|c| !c.is_alphabetic() && !c.is_whitespace()).count();
            words += s
                .text
                .split_whitespace()
                .filter(|w| {
                    let letters = w.chars().filter(|c| c.is_alphabetic()).count();
                    letters >= 3 && !mathtex::is_function_word(w.trim_matches(|c: char| !c.is_alphabetic()))
                })
                .count();
        }
    }
    // An equation number at the right margin, after a wide gap (or alone).
    let tag = l.spans.last().is_some_and(|s| {
        // (an equation image can reach close to its number: any gap will do)
        let prev = (l.spans.len() >= 2).then(|| &l.spans[l.spans.len() - 2]);
        let before = prev.is_none_or(|p| s.left - p.right() >= if p.tex.is_some() { 0.0 } else { 2.0 * st.body });
        mathtex::tag_of(&s.text).is_some() && s.right() >= st.right - 1.5 * st.body && before
    });
    let ratio = if mathc + letters == 0 {
        if other > 0 { 1.0 } else { 0.0 }
    } else {
        mathc as f32 / (mathc + letters) as f32
    };
    // Plain numbers alone are only math when centred (see display_flags).
    let ratio = if mathc == 0 && ratio > 0.0 && letters == 0 { 0.5 } else { ratio };
    (ratio, words, tag)
}

fn display_flags(doc: &Doc, lines: &[Line], st: &Stats) -> Vec<bool> {
    let info: Vec<(f32, usize, bool)> = lines.iter().map(|l| math_info(doc, l, st)).collect();
    let indented = |l: &Line| l.left - st.left >= 4.0 * st.body;
    let mut f: Vec<bool> = lines
        .iter()
        .zip(&info)
        .map(|(l, &(ratio, words, tag))| {
            tag || (words <= 2 && ((indented(l) && ratio >= 0.25) || ratio >= 0.7))
        })
        .collect();
    // Grow each equation by the rows around it (numerators, limits...).
    loop {
        let mut changed = false;
        for i in 0..lines.len() {
            if f[i] {
                continue;
            }
            let (ratio, words, _) = info[i];
            let l = &lines[i];
            // Prose lines starting at the margin ("Also¹") are not part of
            // the equation even when they sit right above it.
            let at_margin = l.left - st.left < 2.0 * st.body;
            let candidate = words <= 3
                && (ratio > 0.0 || l.size < 0.9 * st.body || indented(l))
                && (!at_margin || ratio >= 0.7);
            // ... or it lies beside a tall bracket of the equation (the rows
            // of cases after a big "{")
            let beside_bracket = |j: usize| {
                let cy = l.top + 0.5 * st.body;
                lines[j].spans.iter().any(|s| {
                    doc.family(s).starts_with("CMEX") && s.bottom() - s.top > 1.5 * st.body && s.top <= cy && s.bottom() >= cy
                })
            };
            let near = |j: usize| f[j] && ((l.top - lines[j].top).abs() <= 1.25 * st.body || beside_bracket(j));
            if candidate && ((i > 0 && near(i - 1)) || (i + 1 < lines.len() && near(i + 1))) {
                f[i] = true;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    f
}

/// A line set in the middle of the text column: its middle at the
/// column's middle, and clearly in from both margins (a paragraph's
/// indented first line or its short last line is not).
fn is_centered(l: &Line, st: &Stats) -> bool {
    let mid = (st.left + st.right) / 2.0;
    ((l.left + l.right) / 2.0 - mid).abs() <= 0.4 * st.body
        && l.left - st.left >= st.body
        && st.right - l.right >= st.body
}

/// How far a line is in from the nearer margin.
fn inset(l: &Line, st: &Stats) -> f32 {
    (l.left - st.left).min(st.right - l.right)
}

fn ends_sentence(t: &str) -> bool {
    matches!(t.trim_end().chars().last(), Some('.' | ':' | '?' | '!'))
}

/// Does this line start a new numbered entry (a note "¹ ..." or a
/// reference "[3] ...")?
fn starts_entry(l: &Line) -> bool {
    let Some(first) = l.spans.first() else { return false };
    let t = first.text.trim();
    if first.script == Script::Super && !t.is_empty() && t.bytes().all(|b| b.is_ascii_digit()) {
        return true;
    }
    let p = plain_text(l);
    p.strip_prefix('[')
        .and_then(|r| r.split_once(']'))
        .is_some_and(|(n, _)| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
}

/// Is this a heading? Returns its font size.
fn heading_size(
    doc: &Doc,
    l: &Line,
    prev_top: Option<f32>,
    next_top: Option<f32>,
    st: &Stats,
) -> Option<f32> {
    let text = plain_text(l);
    let n = text.chars().count();
    if n == 0 || n > 120 || !text.chars().any(|c| c.is_alphabetic()) {
        return None;
    }
    if big_text_line(doc, l, st.body) {
        return Some(l.size);
    }
    // A short, entirely italic or bold line at the margin with space above.
    let words: Vec<_> = l
        .spans
        .iter()
        .filter(|s| !doc.is_math(s) && s.text.chars().any(|c| c.is_alphabetic()))
        .collect();
    let styled = !words.is_empty() && words.iter().all(|s| s.italic || s.bold);
    let space_above = prev_top.is_none_or(|t| l.top - t >= 1.4 * st.spacing);
    let space_below = next_top.is_some_and(|t| t - l.top >= 1.4 * st.spacing);
    let at_margin = (l.left - st.left).abs() <= 0.6 * st.body;
    // An unstyled heading: a short title-like line with space above and below.
    if !styled
        && at_margin
        && space_above
        && space_below
        && text.split_whitespace().count() <= 12
        && text.starts_with(|c: char| c.is_uppercase())
        && !text.ends_with(['.', ',', ';', ':'])
        && l.right <= st.right - 3.0 * st.body
        && l.spans.iter().all(|s| s.script == Script::Normal)
    {
        return Some(l.size);
    }
    let ok = styled
        && (l.left - st.left).abs() <= 0.6 * st.body
        && l.right <= st.right - 3.0 * st.body
        && !text.ends_with([',', ';'])
        && space_above;
    ok.then_some(l.size)
}

/// Footnotes at the bottom of a page: the last lines, in a smaller font,
/// starting with a marker. Returns the index where they start.
fn page_notes_start(doc: &Doc, lines: &[Line], st: &Stats) -> usize {
    // a line's type size, not counting equation images (given body size)
    let text_size = |l: &Line| {
        l.spans.iter().filter(|s| s.tex.is_none()).map(|s| doc.size_of(s)).fold(0.0f32, f32::max).max(if l.spans.iter().all(|s| s.tex.is_some()) { l.size } else { 0.0 })
    };
    let mut k = lines.len();
    while k > 0 && text_size(&lines[k - 1]) <= 0.9 * st.body {
        k -= 1;
    }
    // a whole page of small type: notes continued from the page before
    if k == 0 && lines.len() >= 3 {
        return 0;
    }
    let ok = lines.get(k).is_some_and(|l| {
        let first = l.spans.first().map(|s| s.text.trim().to_string()).unwrap_or_default();
        let marker = !first.is_empty()
            && first.chars().all(|c| c.is_ascii_digit() || "*†‡§¶".contains(c));
        let (_, words, _) = math_info(doc, l, st);
        // (the note's number can stand raised on a line of its own, its
        // text on the line just below)
        let lone = l.spans.len() == 1 && lines.get(k + 1).is_some_and(|n| math_info(doc, n, st).1 >= 2);
        marker && (words >= 2 || lone)
    });
    if ok {
        k
    } else {
        lines.len()
    }
}

/// A fraction inside a line of text has its numerator and denominator just
/// above and below the line, so line grouping puts them on lines of their
/// own. Move them back into the line that holds the fraction bar, and give
/// each line the bars it contains.
fn attach_fraction_rows(doc: &Doc, lines: &[Line], bars: &[Rule]) -> Vec<Line> {
    let mut lines: Vec<Line> = lines.to_vec();
    for b in bars {
        // a root's bar starts at its root sign: it goes with the sign's line
        let root = lines.iter().position(|l| {
            l.spans.iter().any(|s| {
                s.text.trim() == "√" && (b.left - s.right()).abs() <= 3.0 && s.top <= b.bottom + 2.0 && s.bottom() >= b.top
            })
        });
        if let Some(k) = root {
            lines[k].bars.push(*b);
            continue;
        }
        let y = (b.top + b.bottom) / 2.0;
        let Some(home) = lines.iter().position(|l| {
            y >= l.top && y <= l.top + l.size && l.left <= b.left + 1.0 && l.right >= b.right - 1.0
        }) else {
            continue;
        };
        let size = lines[home].size;
        let mut moved = Vec::new();
        for (k, l) in lines.iter_mut().enumerate() {
            if k == home {
                continue;
            }
            l.spans.retain(|s| {
                let c = s.left + s.width / 2.0;
                let inside = c >= b.left - 2.0 && c <= b.right + 2.0;
                let near = (b.top - s.bottom() <= 0.8 * size && s.bottom() <= b.top + 2.0)
                    || (s.top - b.bottom <= 0.8 * size && s.top >= b.bottom - 2.0);
                if inside && near {
                    moved.push(s.clone());
                    false
                } else {
                    true
                }
            });
        }
        if !moved.is_empty() {
            let mut spans = std::mem::take(&mut lines[home].spans);
            spans.extend(moved);
            let (col, mut lb) = (lines[home].col, std::mem::take(&mut lines[home].bars));
            lines[home] = crate::layout::make_line(spans, &|s| doc.size_of(s));
            lines[home].col = col;
            lb.push(*b);
            lines[home].bars = lb;
        } else {
            lines[home].bars.push(*b);
        }
        lines.retain(|l| !l.spans.is_empty());
    }
    lines.sort_by(|a, b| a.top.total_cmp(&b.top));
    lines
}

// ---------------------------------------------------------------------------
// Building

struct Prev {
    left: f32,
    right: f32,
    top: f32,
    page: usize,
    ends_sentence: bool,
    /// the web address the line ends in (an address broken over two lines)
    last_url: Option<String>,
}

struct Builder<'a> {
    doc: &'a Doc,
    st: &'a Stats,
    out: Vec<Blk>,
    para: Vec<Inline>,
    para_page: usize,
    para_lines: usize,
    prev: Option<Prev>,
    anchor: Option<String>,
    page: usize,
    index_mode: bool,
    /// all lines of the paragraph being built are centered
    para_centered: bool,
    /// how far its most inset line is in from the margins
    para_inset: f32,
}

impl Builder<'_> {
    fn push(&mut self, block: Block) {
        self.out.push(Blk { block, page: self.page, centered: false });
    }

    fn flush(&mut self) {
        if !self.para.is_empty() {
            let p = std::mem::take(&mut self.para);
            // (one line at least clearly in from the margins: an indented
            // one-line paragraph that ends early is centered by chance)
            let centered = self.para_centered && self.para_inset >= 2.5 * self.st.body;
            // not centered after all: its kept line breaks become spaces
            let p = if centered {
                p
            } else {
                p.into_iter().map(|i| if i == Inline::Break { Inline::Text(" ".into()) } else { i }).collect()
            };
            self.out.push(Blk { block: Block::Para(tidy(p)), page: self.para_page, centered });
        }
        self.prev = None;
        self.para_lines = 0;
    }

    /// Put the pending page anchor in front of the next block.
    fn anchor_block(&mut self) {
        if let Some(a) = self.anchor.take() {
            self.push(Block::Anchor(a));
        }
    }

    fn starts_para(&self, l: &Line, next: Option<&Line>, pi: usize) -> bool {
        let st = self.st;
        let Some(p) = &self.prev else { return true };
        // the rest of a web address broken over two lines
        if p.last_url.is_some() && l.spans.first().and_then(|s| s.url.as_ref()) == p.last_url.as_ref() {
            return false;
        }
        if starts_entry(l) {
            return true;
        }
        if p.page == pi && l.top - p.top > 1.6 * st.spacing {
            return true;
        }
        if p.right < st.right - 3.0 * st.body && p.ends_sentence {
            return true;
        }
        let step = 0.6 * st.body;
        if l.left - st.left >= step {
            // Hanging indent (references): the first line sticks out and
            // the rest are indented.
            let hanging = self.para_lines == 1
                && p.left < l.left - step
                && next.is_none_or(|n| n.left >= l.left - step || starts_entry(n));
            // An indented block (a quotation): same indent as before.
            let same_indent = (p.left - l.left).abs() < step;
            return !(hanging || same_indent);
        }
        false
    }

    /// Add a line of prose to the open paragraph, repairing hyphenation.
    fn join(&mut self, inl: Vec<Inline>) {
        // a web address continued on this line: one link, nothing between
        if let (Some(Inline::Link { text: a, href: ha }), Some(Inline::Link { text: b, href: hb })) = (self.para.last_mut(), inl.first()) {
            if ha == hb && (ha.starts_with("http") || ha.starts_with("mailto:")) {
                a.push_str(b);
                self.para.extend(inl.into_iter().skip(1));
                return;
            }
        }
        if let (Some(Inline::Text(a)), Some(Inline::Text(b))) = (self.para.last_mut(), inl.first()) {
            let a_word: String = a
                .trim_end_matches('-')
                .chars()
                .rev()
                .take_while(|c| c.is_alphabetic())
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect();
            if a.ends_with('-') && !a_word.is_empty() && b.starts_with(|c: char| c.is_lowercase()) {
                let b_word: String = b.chars().take_while(|c| c.is_alphabetic()).collect();
                let compound = format!("{}-{}", a_word, b_word).to_lowercase();
                if !self.st.hyphenated.contains(&compound) {
                    a.pop();
                }
                a.push_str(b);
                self.para.extend(inl.into_iter().skip(1));
                return;
            }
        }
        push_text(&mut self.para, " ");
        for i in inl {
            match i {
                Inline::Text(t) => push_text(&mut self.para, &t),
                other => self.para.push(other),
            }
        }
    }

    fn index_line(&mut self, l: &Line, base: f32, inl: Vec<Inline>) {
        let st = self.st;
        let off = l.left - base;
        let starts_digit = plain(&inl).starts_with(|c: char| c.is_ascii_digit());
        if off >= 2.6 * st.body || starts_digit {
            if let Some(Blk { block: Block::Item { text, .. }, .. }) = self.out.last_mut() {
                push_text(text, " ");
                text.extend(inl);
                return;
            }
        }
        let level = if off >= 0.6 * st.body { 1 } else { 0 };
        self.anchor_block();
        self.push(Block::Item { level, text: inl });
    }

    fn toc_page(&mut self, lines: &[Line]) {
        self.flush();
        self.anchor_block();
        let mut entries: Vec<TocEntry> = Vec::new();
        let mut pending: Vec<Inline> = Vec::new();
        for l in lines {
            let text = plain_text(l);
            if is_contents_title(&text) {
                let inl = crate::no_emph(line_inlines(self.doc, l, self.st));
                self.push(Block::Heading { level: 2, size: l.size, id: String::new(), text: inl });
                continue;
            }
            let mut inl = crate::no_emph(line_inlines(self.doc, l, self.st));
            // a printed page number at the end, or else a link to the page
            let page = match toc_split(&text) {
                Some((_, page)) => {
                    // drop the page number and dot leaders from the last text piece
                    if let Some(Inline::Text(t)) = inl.last_mut() {
                        let trimmed = t.trim_end();
                        let cut = trimmed.rsplit_once(char::is_whitespace).map_or("", |(a, _)| a);
                        *t = cut.trim_end_matches(|c: char| c == '.' || c.is_whitespace()).to_string();
                    }
                    page
                }
                None => match link_target(l) {
                    Some(target) => target,
                    None => {
                        // a title that wraps onto the next line
                        pending.extend(inl);
                        push_text(&mut pending, " ");
                        continue;
                    }
                },
            };
            // a linked title wrapping onto the next line links to the same
            // page: it continues the entry above
            if page.starts_with('@') && pending.is_empty() {
                if let Some(prev) = entries.last_mut().filter(|e| e.page == page) {
                    let mut title = std::mem::take(&mut prev.title);
                    push_text(&mut title, " ");
                    title.extend(inl);
                    prev.title = tidy(title);
                    continue;
                }
            }
            let mut title = std::mem::take(&mut pending);
            title.extend(inl);
            let title = tidy(title);
            if !title.is_empty() {
                entries.push(TocEntry { title, page, left: l.left, size: l.size });
            }
        }
        // Several TOC pages make one table (only page anchors in between).
        let last = self.out.iter().rposition(|b| !matches!(b.block, Block::Anchor(_)));
        if let Some(Block::Toc(prev)) = last.map(|p| &mut self.out[p].block) {
            prev.extend(entries);
        } else if !entries.is_empty() {
            self.push(Block::Toc(entries));
        }
    }
}

pub fn build(doc: &Doc, pages: &[PageInfo], st: &Stats) -> Vec<Blk> {
    let mut b = Builder {
        doc,
        st,
        out: Vec::new(),
        para: Vec::new(),
        para_page: 0,
        para_lines: 0,
        prev: None,
        anchor: None,
        page: 0,
        index_mode: false,
        para_centered: false,
        para_inset: 0.0,
    };
    // printed page marks (ebook PDFs without page numbers; see layout.rs)
    let use_marks = pages.iter().all(|p| p.label.is_none());
    for (pi, page) in pages.iter().enumerate() {
        b.page = pi;
        if let Some(l) = &page.label {
            b.anchor = Some(format!("p{l}"));
        } else if !use_marks {
            b.anchor = None;
        }
        let marks: &[(f32, String)] = if use_marks { &doc.pages[page.index].marks } else { &[] };
        let mut mi = 0;
        if st.toc_pages.contains(&pi) {
            b.toc_page(&page.lines);
            continue;
        }
        let lines = &attach_fraction_rows(doc, &page.lines, &doc.pages[page.index].rules);
        let end = page_notes_start(doc, lines, st);
        let disp = display_flags(doc, &lines[..end], st);
        // left edge of each column, for index indentation
        let mut col_left: HashMap<u8, f32> = HashMap::new();
        for l in lines {
            let e = col_left.entry(l.col).or_insert(l.left);
            *e = e.min(l.left);
        }

        let tables = find_tables(doc, &lines[..end], st);

        let mut prev_top: Option<f32> = None;
        let mut last_heading: Option<f32> = None; // top of a heading just written
        // pictures go in where they stand on the page
        let figs = &doc.pages[page.index].figures;
        let number = doc.pages[page.index].number;
        let mut fi = 0;
        let mut i = 0;
        let codes = &doc.pages[page.index].codes;
        let mut ci = 0;
        while i < end {
            let l = &lines[i];
            // program code (an image read as text) above this line
            while ci < codes.len() && codes[ci].0 <= l.top {
                b.flush();
                b.anchor_block();
                b.push(Block::Code(codes[ci].1.clone()));
                ci += 1;
                prev_top = None;
            }
            // a printed page starts here
            while mi < marks.len() && marks[mi].0 <= l.top + 2.0 {
                if b.anchor.is_some() {
                    b.flush();
                    b.anchor_block();
                }
                b.anchor = Some(format!("p{}", marks[mi].1));
                mi += 1;
            }
            while fi < figs.len() && figs[fi].top <= l.top {
                b.flush();
                b.anchor_block();
                b.push(Block::Figure { page: number, n: fi + 1, area: figs[fi], label: page.label.clone() });
                fi += 1;
                prev_top = None;
            }
            if let Some((_, j, cols)) = tables.iter().find(|t| t.0 == i).filter(|_| !b.index_mode) {
                b.flush();
                b.anchor_block();
                b.push(Block::Equation { latex: table_latex(doc, &lines[i..*j], cols), tag: None, source: None });
                prev_top = Some(lines[j - 1].top);
                last_heading = None;
                i = *j;
                continue;
            }
            if disp[i] && !b.index_mode {
                let mut j = i;
                while j < end && disp[j] {
                    j += 1;
                }
                b.flush();
                b.anchor_block();
                let spans: Vec<MSpan> = lines[i..j]
                    .iter()
                    .flat_map(|l| l.spans.iter().map(|s| MSpan::new(doc, s)))
                    .collect();
                // drawn bars (fraction bars, root bars) inside the equation
                let (x0, x1, y0, y1) = spans.iter().fold(
                    (f32::MAX, f32::MIN, f32::MAX, f32::MIN),
                    |(a, b, c, d), s| (a.min(s.left), b.max(s.right), c.min(s.top), d.max(s.bottom)),
                );
                let bars: Vec<Rule> = doc.pages[page.index]
                    .rules
                    .iter()
                    .filter(|r| r.left >= x0 - 5.0 && r.right <= x1 + 5.0 && r.top >= y0 - 5.0 && r.bottom <= y1 + 5.0)
                    .copied()
                    .collect();
                // one equation image (and its number): remember which
                let parts: Vec<&crate::layout::Span> = lines[i..j].iter().flat_map(|l| &l.spans).filter(|s| mathtex::tag_of(&s.text).is_none() && (s.tex.is_some() || !s.text.trim().is_empty())).collect();
                let source = match parts.as_slice() {
                    [one] => one.image_key.clone(),
                    _ => None,
                };
                let eqs = mathtex::display(spans, &bars, st.right, st.body);
                let single = eqs.len() == 1;
                for (latex, tag) in eqs {
                    if !latex.is_empty() || tag.is_some() {
                        b.push(Block::Equation { latex, tag, source: if single { source.clone() } else { None } });
                    }
                }
                prev_top = Some(lines[j - 1].top);
                last_heading = None;
                i = j;
                continue;
            }
            let next_top = (i + 1 < end && !disp[i + 1]).then(|| lines[i + 1].top);
            if let Some(size) = heading_size(doc, l, prev_top, next_top, st) {
                b.flush();
                let text = crate::no_emph(line_inlines(doc, l, st));
                // A title broken over two lines is one heading.
                let mut merged = false;
                if let (Some(top), Some(Blk { block: Block::Heading { size: s0, text: t0, .. }, .. })) =
                    (last_heading, b.out.last_mut())
                {
                    if (*s0 - size).abs() < 0.5 && l.top - top <= 1.6 * size {
                        push_text(t0, " ");
                        t0.extend(text.clone());
                        merged = true;
                    }
                }
                if !merged {
                    b.anchor_block();
                    b.push(Block::Heading { level: 0, size, id: String::new(), text: text.clone() });
                    if let Some(last) = b.out.last_mut() {
                        last.centered = is_centered(l, st) && inset(l, st) >= 2.5 * st.body;
                    }
                }
                if st.chapter_size.is_some_and(|c| (c - size).abs() < 0.6) {
                    b.index_mode = key(&plain(&text)) == "index";
                }
                prev_top = Some(l.top);
                last_heading = Some(l.top);
                i += 1;
                continue;
            }
            last_heading = None;
            let inl = line_inlines(doc, l, st);
            if b.index_mode {
                let base = col_left.get(&l.col).copied().unwrap_or(st.left);
                b.index_line(l, base, inl);
                prev_top = Some(l.top);
                i += 1;
                continue;
            }
            let next = (i + 1 < end && !disp[i + 1]).then(|| &lines[i + 1]);
            if b.starts_para(l, next, pi) {
                b.flush();
                b.anchor_block();
                b.para_page = pi;
                b.para = inl;
                b.para_centered = is_centered(l, st);
                b.para_inset = inset(l, st);
            } else {
                if let Some(a) = b.anchor.take() {
                    b.para.push(Inline::Anchor(a));
                }
                // centered lines keep their line breaks
                let centered = b.para_centered && is_centered(l, st);
                if centered {
                    b.para.push(Inline::Break);
                    b.para.extend(inl);
                } else {
                    b.join(inl);
                }
                b.para_centered &= is_centered(l, st);
                b.para_inset = b.para_inset.max(inset(l, st));
            }
            b.para_lines += 1;
            b.prev = Some(Prev {
                left: l.left,
                right: l.right,
                top: l.top,
                page: pi,
                ends_sentence: ends_sentence(&plain_text(l)),
                last_url: l.spans.last().and_then(|s| s.url.clone()),
            });
            prev_top = Some(l.top);
            i += 1;
        }
        // code below the last line of text
        while ci < codes.len() {
            b.flush();
            b.anchor_block();
            b.push(Block::Code(codes[ci].1.clone()));
            ci += 1;
        }
        // pictures below the last line of text
        while fi < figs.len() {
            b.flush();
            b.anchor_block();
            b.push(Block::Figure { page: number, n: fi + 1, area: figs[fi], label: page.label.clone() });
            fi += 1;
        }
        // An empty page still gets its anchor.
        if let Some(a) = b.anchor.take() {
            if b.para.is_empty() {
                b.push(Block::Anchor(a));
            } else {
                b.para.push(Inline::Anchor(a));
            }
        }
        // printed pages that start below the last line of text: at the
        // start of what follows
        while mi < marks.len() {
            if b.anchor.is_some() {
                b.flush();
                b.anchor_block();
            }
            b.anchor = Some(format!("p{}", marks[mi].1));
            mi += 1;
        }
        // Footnotes at the bottom of the page.
        let is_marker = |t: &str| !t.is_empty() && t.chars().all(|c| c.is_ascii_digit() || "*†‡§¶".contains(c));
        let mut lone_label: Option<String> = None;
        for l in &lines[end..] {
            let inl = line_inlines(doc, l, st);
            let first = l.spans.first().map(|s| s.text.trim().to_string()).unwrap_or_default();
            // a note number alone on its line labels the text below it
            if l.spans.len() == 1 && is_marker(&first) {
                lone_label = Some(first);
                continue;
            }
            if let Some(label) = lone_label.take() {
                b.out.push(Blk { block: Block::PageNote { label, text: tidy(inl) }, page: pi, centered: false });
                continue;
            }
            let is_start = is_marker(&first);
            if is_start {
                // drop the marker itself from the text
                let mut text = inl;
                match text.first_mut() {
                    Some(Inline::Sup(_)) => {
                        text.remove(0);
                    }
                    Some(Inline::Text(t)) => {
                        *t = t.trim_start().trim_start_matches(&first[..]).to_string();
                    }
                    _ => {}
                }
                b.out.push(Blk { block: Block::PageNote { label: first, text: tidy(text) }, page: pi, centered: false });
            } else if let Some(Blk { block: Block::PageNote { text, .. }, .. }) = b.out.last_mut() {
                push_text(text, " ");
                text.extend(inl);
            }
        }
    }
    b.flush();
    b.out
}
