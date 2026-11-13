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

use crate::layout::{plain_text, Doc, Figure, Line, Rule, Script};
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

pub(crate) fn is_contents_title(t: &str) -> bool {
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
pub(crate) enum K {
    Text,
    Math,
    Op,
    Marker,
}

/// A book without TeX's math fonts (an ebook) sets its math as text.
pub(crate) fn is_text_math(doc: &Doc) -> bool {
    !doc.fonts.iter().any(|f| {
        let u = f.family.to_uppercase();
        ["CMMI", "CMSY", "CMEX", "CMR", "MSAM", "MSBM"].iter().any(|p| u.starts_with(p))
    })
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
    // In a book without TeX's math fonts (an ebook), math is set as text:
    // where it begins and ends is told from the words and symbols
    if is_text_math(doc) {
        let (spans, kinds) = crate::textmath::classify(doc, &spans, st);
        return emit_line(doc, l, st, &spans, &kinds);
    }
    let spans = &spans;
    let n = spans.len();
    let gap = |a: usize, b: usize| spans[b].left - spans[a].right();
    // A linked number ("2.28" of "(2.28)", a page number in the index) is a
    // reference, never part of the formula beside it; nor are its brackets.
    let reference = |i: usize| {
        let t = spans[i].text.trim();
        spans[i].link.is_some() && t.starts_with(|c: char| c.is_ascii_digit()) && t.chars().all(|c| c.is_ascii_digit() || c == '.')
    };
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
            } else if !reference(i) && !t.is_empty() && t.chars().all(|c| c.is_ascii_digit() || "=+−-–()[]/<>|,.;:·×*!′".contains(c)) {
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
            if (t.ends_with('(') && i + 1 < n && reference(i + 1)) || (t.starts_with(')') && i > 0 && reference(i - 1)) {
                continue;
            }
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

    emit_line(doc, l, st, spans, &kinds)
}

/// The spans of a line, each known to be text, math or a note marker, as
/// inlines: a run of math spans is one formula.
fn emit_line(doc: &Doc, l: &Line, st: &Stats, spans: &[crate::layout::Span], kinds: &[K]) -> Vec<Inline> {
    let n = spans.len();
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
                    // inside a sentence: no second script on one symbol, and
                    // `split` (display only) as `aligned`
                    let t = mathtex::fix_double_scripts(&t).replace("{split}", "{aligned}");
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
    // "$m_1 + m_2 =$ $m$": formulas apart only by a space, the first ending
    // (or the second starting) with an operator, are one formula
    let ops = ['=', '+', '−', '-', '<', '>', '≈', '≡', '×', '·'];
    let mut merged: Vec<Inline> = Vec::with_capacity(out.len());
    for x in out {
        if let Inline::Math(b) = &x {
            let n = merged.len();
            if n >= 2 {
                if let (Inline::Math(a), Inline::Text(sp)) = (&merged[n - 2], &merged[n - 1]) {
                    if sp.trim().is_empty() && (a.trim_end().ends_with(ops) || b.trim_start().starts_with(ops)) {
                        let joined = format!("{} {}", a.trim_end(), b.trim_start());
                        merged.truncate(n - 2);
                        merged.push(Inline::Math(joined));
                        continue;
                    }
                }
            }
        }
        merged.push(x);
    }
    let out = merged;
    tidy(out)
}

// ---------------------------------------------------------------------------
// Tables

/// A table that the book holds as a picture comes from the math
/// recognizer as one formula, an array. Under a table's caption such a
/// formula is the table: its rows and cells, with the picture it was read
/// from right under it (a cell can be misread; the picture shows what
/// stands there). `boxes`: where each equation picture is on its page.
///
/// Such a book sets the caption centered on a panel on top of the table,
/// as its first row; it comes as centered paragraphs, cut where its lines
/// end and where the page turns. They are the table's title then: one
/// cell across its columns.
///
/// A picture of a table can also stand without a caption (in a problem).
/// What the recognizer makes of it is no array then, or need not be: a
/// large picture that is a block of its own is marked to be looked at
/// (see [`crate::TABLE_MAYBE`]).
fn picture_tables(blks: Vec<Blk>, boxes: &HashMap<String, (u32, Figure)>) -> Vec<Blk> {
    let is_caption = |inl: &[Inline]| crate::links::caption(inl).is_some_and(|(kind, _)| kind == "table");
    let array = |tex: &str| tex.trim_start().starts_with("\\begin{array}");
    let caption_start = caption_above;
    let mut out: Vec<Blk> = Vec::new();
    for blk in blks {
        // the formula of this block, if it is all of it or stands at its end
        let found: Option<(Vec<Inline>, String, String)> = match &blk.block {
            Block::Equation { latex, source: Some(k), .. } if array(latex) => Some((Vec::new(), k.clone(), latex.clone())),
            Block::Para(inl) => match inl.as_slice() {
                [before @ .., Inline::Source(k), Inline::Math(tex)] if array(tex) => Some((before.to_vec(), k.clone(), tex.clone())),
                _ => None,
            },
            _ => None,
        };
        // (under a table's caption: in the block itself, or one of the
        // last few, or at the start of the centered paragraphs above)
        let captioned = |before: &[Inline]| {
            is_caption(before)
                || out.iter().rev().filter(|b| !matches!(b.block, Block::Anchor(_))).take(3).any(|b| matches!(&b.block, Block::Para(inl) if is_caption(inl)))
                || (blk.centered && caption_start(&out).is_some())
        };
        let table = found.as_ref().filter(|(before, _, _)| captioned(before)).and_then(|(_, _, tex)| array_table(tex));
        let (Some((before, key, _)), Some(rows)) = (found, table) else {
            // A large picture that is all of its block may be a table
            // without a caption: it is looked at later (rules across it
            // tell), so where it stands goes with it.
            let alone = match &blk.block {
                Block::Equation { source: Some(k), .. } => Some(k),
                Block::Para(inl) => match inl.as_slice() {
                    [Inline::Source(k), Inline::Math(_)] => Some(k),
                    _ => None,
                },
                _ => None,
            };
            let large = alone.and_then(|k| boxes.get(k)).filter(|(_, a)| a.bottom - a.top >= 100.0 && a.right - a.left >= 100.0).copied();
            let page = blk.page;
            out.push(blk);
            if let Some((number, area)) = large {
                out.push(Blk { block: Block::Picture { page: number, area, alt: crate::TABLE_MAYBE.into() }, page, centered: false });
            }
            continue;
        };
        let (page, centered) = (blk.page, blk.centered);
        // The title: the centered paragraphs from the caption's start to
        // the table (the page's mark among them stays in front of the table).
        let mut title: Vec<Inline> = Vec::new();
        let start = caption_start(&out).filter(|_| centered && !is_caption(&before));
        if let Some(start) = start {
            title = take_caption(&mut out, start);
        }
        if centered && (start.is_some() || is_caption(&before)) {
            title.extend(before.clone());
            title = tidy(title.into_iter().map(|i| if i == Inline::Break { Inline::Text(" ".into()) } else { i }).collect());
        } else if has_text(&before) {
            out.push(Blk { block: Block::Para(tidy(before)), page, centered });
        }
        // (keyed by the picture it was read from)
        let table = Block::Table { key: crate::table_key(boxes.get(&key).map_or(0, |b| b.0), &rows), rows, widths: Vec::new(), title, note: Vec::new() };
        out.push(Blk { block: table, page, centered: false });
        if let Some((number, area)) = boxes.get(&key) {
            out.push(Blk { block: Block::Picture { page: *number, area: *area, alt: crate::TABLE_READ.into() }, page, centered: false });
        }
    }
    out
}

/// Where the caption starts that the last blocks are: centered paragraphs
/// (and marks of pages) back to one that starts as a table's caption
/// does, a dozen blocks at most. A book that sets a table's caption
/// centered on a panel on top of the table gives it so, cut where its
/// lines end and where the page turns.
fn caption_above(out: &[Blk]) -> Option<usize> {
    for (k, b) in out.iter().enumerate().rev().take(12) {
        match &b.block {
            Block::Para(inl) if b.centered => {
                if crate::links::caption(inl).is_some_and(|(kind, _)| kind == "table") {
                    return Some(k);
                }
            }
            Block::Anchor(_) => {}
            // (a line of the caption with a formula in it, taken for a
            // displayed formula: it has no number)
            Block::Equation { tag: None, source: None, .. } => {}
            _ => return None,
        }
    }
    None
}

/// The caption that the blocks from `start` on are, taken out of them as
/// one run of text; the marks of pages among them stay.
/// (Its last blocks must be blocks of a caption: see `caption_above`.)
fn take_caption(out: &mut Vec<Blk>, start: usize) -> Vec<Inline> {
    let mut caption: Vec<Inline> = Vec::new();
    let mut marks = Vec::new();
    for b in out.split_off(start) {
        match b.block {
            Block::Para(inl) => {
                if !caption.is_empty() {
                    caption.push(Inline::Text(" ".into()));
                }
                caption.extend(inl.into_iter().map(|i| if i == Inline::Break { Inline::Text(" ".into()) } else { i }));
            }
            // (a line taken for a displayed formula: words, if it is only
            // words, else a formula in the line)
            Block::Equation { latex, .. } => {
                let words = latex.trim().strip_prefix("\\text{").and_then(|t| t.strip_suffix('}')).filter(|t| !t.contains(['{', '}', '\\']));
                caption.push(Inline::Text(" ".into()));
                caption.push(match words {
                    Some(w) => Inline::Text(w.trim().to_string()),
                    None => Inline::Math(latex),
                });
            }
            _ => marks.push(b),
        }
    }
    out.extend(marks);
    caption
}

fn has_text(inl: &[Inline]) -> bool {
    inl.iter().any(|i| match i {
        Inline::Text(t) | Inline::Emph(t) | Inline::Strong(t) => !t.trim().is_empty(),
        Inline::Math(_) | Inline::Link { .. } => true,
        _ => false,
    })
}

/// The rows and cells of a LaTeX array (`\begin{array}{ccc} a & b \\ c & d
/// \end{array}`), each cell a formula, or words if it holds only words.
/// None if it is no table: fewer than two rows or two columns.
pub(crate) fn array_table(tex: &str) -> Option<Vec<Vec<crate::Cell>>> {
    let inner = tex.trim().strip_prefix("\\begin{array}")?;
    let inner = inner[..inner.rfind("\\end{array}")?].trim_start();
    // (the columns' alignment: "{c c c}")
    let inner = match inner.strip_prefix('{') {
        Some(rest) => &rest[rest.find('}')? + 1..],
        None => inner,
    };
    // cut at "\\" and "&" that stand in no braces and no inner array
    let mut rows: Vec<Vec<String>> = vec![vec![String::new()]];
    let (mut depth, mut nested) = (0i32, 0i32);
    let mut rest = inner;
    while let Some(c) = rest.chars().next() {
        let cell = rows.last_mut().unwrap().last_mut().unwrap();
        if rest.starts_with("\\begin{") {
            nested += 1;
        } else if rest.starts_with("\\end{") {
            nested -= 1;
        }
        if depth == 0 && nested == 0 && rest.starts_with("\\\\") {
            rows.push(vec![String::new()]);
            rest = &rest[2..];
            continue;
        }
        if depth == 0 && nested == 0 && c == '&' {
            rows.last_mut().unwrap().push(String::new());
            rest = &rest[1..];
            continue;
        }
        if c == '\\' && rest.len() > 1 {
            // (an escaped character, or a command's backslash: not a brace)
            let next = rest[1..].chars().next().unwrap();
            cell.push(c);
            cell.push(next);
            rest = &rest[1 + next.len_utf8()..];
            continue;
        }
        match c {
            '{' => depth += 1,
            '}' => depth -= 1,
            _ => {}
        }
        cell.push(c);
        rest = &rest[c.len_utf8()..];
    }
    let mut rows: Vec<Vec<crate::Cell>> = rows.iter().map(|r| r.iter().map(|c| table_cell(c)).collect::<Vec<_>>()).filter(|r: &Vec<crate::Cell>| r.iter().any(|c| !c.text.is_empty())).collect();
    let columns = rows.iter().map(Vec::len).max().unwrap_or(0);
    if rows.len() < 2 || columns < 2 {
        return None;
    }
    for r in rows.iter_mut() {
        r.resize_with(columns, || crate::Cell { span: 1, ..Default::default() });
    }
    Some(rows)
}

/// A cell of a table read as LaTeX: a formula, or words if it holds only
/// words.
pub(crate) fn table_cell(cell: &str) -> crate::Cell {
    {
        let mut t = cell.replace("\\hline", "");
        let mut t_ref = t.trim();
        // ({{…}}: braces around all of it say nothing)
        while t_ref.starts_with('{') && t_ref.ends_with('}') && wraps(t_ref) {
            t_ref = t_ref[1..t_ref.len() - 1].trim();
        }
        t = t_ref.to_string();
        let words = ["\\mathrm{", "\\text{", "\\textrm{", "\\operatorname{"].iter().find_map(|w| {
            let inside = t.strip_prefix(w)?.strip_suffix('}')?;
            inside.chars().all(|c| c.is_alphabetic() || " ,.-–'’~".contains(c)).then(|| inside.replace('~', " "))
        });
        // (words the recognizer wrote as if they were letters of a formula)
        let plain_words = t.chars().all(|c| c.is_alphabetic() || c == ' ') && t.split_whitespace().any(|w| w.chars().count() >= 3);
        let text = match words {
            Some(w) => vec![Inline::Text(w.trim().to_string())],
            None if t.is_empty() => Vec::new(),
            None if plain_words => vec![Inline::Text(t)],
            None => vec![Inline::Math(t)],
        };
        crate::Cell { text, span: 1, ..Default::default() }
    }
}

/// Does the "{" that starts `t` close at its end?
fn wraps(t: &str) -> bool {
    let mut depth = 0;
    let mut prev = ' ';
    for (i, c) in t.char_indices() {
        if prev != '\\' {
            match c {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        return i == t.len() - 1;
                    }
                }
                _ => {}
            }
        }
        prev = c;
    }
    false
}

/// What a table hands from its page to the next (see [`panel_tables`]).
#[derive(Default)]
struct Carried {
    /// The columns of a table that reaches the foot of its page.
    columns: Option<Vec<f32>>,
    /// The lines of a head that starts at the foot of its page.
    head: Vec<Line>,
    /// Set on the page after: its table took those lines into its head.
    head_taken: bool,
}

/// The tables of a page that are set on a panel of grey (an ebook-made
/// PDF): (first line, end, the table). Such a table has a short rule
/// under each cell of its head, so the rules say where its columns are.
/// Its rows have no marks and stand close together, but every cell is
/// set in the middle of its row's height: a column in which the cells are
/// short shows the rows (its cells stand apart), and the lines of the
/// other columns go to the row whose middle their own middle fits.
///
/// A table that reaches the foot of its page goes on at the top of the
/// next, on a panel without rules (they stand under the head only):
/// `carried` holds the columns of such a table from one page to the next.
///
/// A table can also start at the foot of a page with no more than the
/// first lines of its head, the rest of the head and its rules following
/// on the next page. Those lines are held back (the entry for them has no
/// table) and go into the head of the table there, if `next_takes` says
/// that the next page has such a table.
///
/// The answer for each table: its first line, where it ends, the table
/// (or none, see above), where it stands, and whether it is the rest of
/// the table of the page before.
fn panel_tables(
    doc: &Doc,
    page: &crate::layout::Page,
    lines: &[Line],
    st: &Stats,
    carried: &mut Carried,
    next_takes: &dyn Fn(&[Line]) -> bool,
) -> Vec<(usize, usize, Option<Block>, Figure, bool)> {
    let mut out = Vec::new();
    let from_before = std::mem::take(carried);
    for (panel, light) in &page.shades {
        if *light < 0.7 {
            continue;
        }
        let tall = panel.bottom - panel.top >= 2.0 * st.body;
        let mut goes_on = false;
        // the lines on the panel: they must be a run of the page's lines
        let inside = |l: &Line| l.top >= panel.top - 3.0 && l.top < panel.bottom && l.left >= panel.left - 3.0 && l.right <= panel.right + 3.0;
        let area = Figure { left: panel.left, right: panel.right, top: panel.top, bottom: panel.bottom };
        // the rules on the panel, from the top down
        let on_panel = |r: &&Rule| r.top >= panel.top - 2.0 && r.top <= panel.bottom + 2.0 && r.left >= panel.left - 2.0 && r.right <= panel.right + 2.0;
        let rules: Vec<&Rule> = page.rules.iter().filter(on_panel).collect();
        // where the columns begin and end: where the rules do
        let mut edges: Vec<f32> = rules.iter().flat_map(|r| [r.left, r.right]).collect();
        edges.sort_by(f32::total_cmp);
        edges.dedup_by(|a, b| (*a - *b).abs() < 3.0);
        if edges.len() < 3 {
            // (no rules: the rest of the table of the page before, if this
            // panel starts the page and stands where that table stood)
            match &from_before.columns {
                Some(before) if tall && panel.top < 0.15 * page.height && (panel.left - before[0]).abs() < 6.0 => {
                    edges = before.clone();
                    goes_on = true;
                }
                _ => {
                    // (or, at the foot of the page, the first lines of the
                    // head of a table that the next page has)
                    if let Some(first) = lines.iter().position(inside).filter(|_| panel.bottom > 0.85 * page.height) {
                        let end = first + lines[first..].iter().take_while(|l| inside(l)).count();
                        if !lines[end..].iter().any(inside) && next_takes(&lines[first..end]) {
                            carried.head = lines[first..end].to_vec();
                            out.push((first, end, None, area, false));
                        }
                    }
                    continue;
                }
            }
        }
        if !tall {
            continue;
        }
        let columns = edges.len() - 1;
        let column_of = |x: f32| (0..columns).find(|c| x < edges[c + 1]).unwrap_or(columns - 1);
        let Some(first) = lines.iter().position(inside) else { continue };
        let end = first + lines[first..].iter().take_while(|l| inside(l)).count();
        if std::env::var_os("PDF2MD_TABLEDEBUG").is_some() {
            eprintln!(
                "PANEL page {} columns {} lines {}..{} more inside after: {} | {:.0},{:.0}-{:.0},{:.0}",
                page.number,
                columns,
                first,
                end,
                lines[end..].iter().filter(|l| inside(l)).count(),
                panel.left,
                panel.top,
                panel.right,
                panel.bottom
            );
            for l in lines.iter().filter(|l| l.top >= panel.top - 3.0 && l.top < panel.bottom) {
                eprintln!("   line top {:.0} left {:.0} right {:.0} inside {}", l.top, l.left, l.right, inside(l));
            }
        }
        if lines[end..].iter().any(inside) || end - first < 2 {
            continue;
        }
        // each line's part in each column
        let size_of = |s: &crate::layout::Span| doc.size_of(s);
        let mut parts: Vec<Vec<Line>> = vec![Vec::new(); columns];
        // (the lines of the head that stand on the page before, if this
        // table opens the page with rules of its own: as if they stood
        // right above the panel)
        let mut head: Vec<Line> = Vec::new();
        if rules.len() >= 2 && panel.top < 0.15 * page.height && from_before.head.first().is_some_and(|l| (l.left - panel.left).abs() < 0.5 * (panel.right - panel.left)) {
            let lowest = from_before.head.iter().map(|l| l.top + l.size).fold(f32::MIN, f32::max);
            let up = lowest - (panel.top - 2.0 * st.body);
            for l in &from_before.head {
                let mut l = l.clone();
                l.top -= up;
                for s in &mut l.spans {
                    s.top -= up;
                }
                head.push(l);
            }
        }
        for l in head.iter().chain(&lines[first..end]) {
            // (a piece of text that runs across a column's edge, the words
            // of two cells with spaces between them: cut at the edge)
            let mut pieces: Vec<crate::layout::Span> = Vec::new();
            for s in &l.spans {
                let mut rest = s.clone();
                for e in &edges[1..edges.len() - 1] {
                    let chars: Vec<char> = rest.text.chars().collect();
                    if rest.tex.is_some() || rest.left >= *e - 4.0 || rest.right() <= *e + 4.0 || chars.len() < 3 {
                        continue;
                    }
                    // the space nearest the edge, by the width of a letter
                    let per = rest.width / chars.len() as f32;
                    let at = (*e - rest.left) / per;
                    // (or where a word begins right behind another: the two
                    // cells' words can stand with no space between them)
                    let between = |k: &usize| chars[*k] == ' ' || (*k > 0 && chars[*k].is_uppercase() && chars[*k - 1].is_lowercase());
                    let Some(cut) = (0..chars.len()).filter(between).min_by(|a, b| (*a as f32 - at).abs().total_cmp(&(*b as f32 - at).abs())) else { continue };
                    if (cut as f32 - at).abs() > 6.0 {
                        continue;
                    }
                    let skip = usize::from(chars[cut] == ' ');
                    let (head, tail): (String, String) = (chars[..cut].iter().collect(), chars[cut + skip..].iter().collect());
                    if head.trim().is_empty() || tail.trim().is_empty() {
                        continue;
                    }
                    let mut before = rest.clone();
                    before.text = head;
                    before.width = per * cut as f32;
                    rest.text = tail;
                    rest.left += per * (cut + skip) as f32;
                    rest.width = (s.right() - rest.left).max(1.0);
                    pieces.push(before);
                }
                pieces.push(rest);
            }
            for (c, of_column) in parts.iter_mut().enumerate() {
                let spans: Vec<crate::layout::Span> = pieces.iter().filter(|s| column_of((s.left + s.right()) / 2.0) == c).cloned().collect();
                if spans.iter().any(|s| !s.text.trim().is_empty() || s.tex.is_some()) {
                    of_column.push(crate::layout::make_line(spans, &size_of));
                }
            }
        }
        for of_column in parts.iter_mut() {
            of_column.sort_by(|a, b| a.top.total_cmp(&b.top));
            // A small piece set a little lower right behind a line's last
            // piece is its subscript ("N" and, half a line down, "1"),
            // not a line of its own.
            let mut k = 1;
            while k < of_column.len() {
                let (above, below) = (&of_column[k - 1], &of_column[k]);
                let small = below.spans.len() == 1 && below.size <= 0.85 * above.size && below.spans[0].text.trim().chars().count() <= 3;
                let behind = (below.left - above.right).abs() <= 0.4 * above.size && below.top > above.top && below.top < above.top + 0.7 * above.size;
                if small && behind {
                    let mut sub = of_column.remove(k).spans.remove(0);
                    sub.script = crate::layout::Script::Sub;
                    let mut spans = of_column[k - 1].spans.clone();
                    spans.push(sub);
                    of_column[k - 1] = crate::layout::make_line(spans, &size_of);
                } else {
                    k += 1;
                }
            }
        }
        // bands: the head above the first rule, the body below (a rule
        // across the body parts it again)
        let mut cuts: Vec<f32> = rules.iter().map(|r| r.top).filter(|y| *y < panel.bottom - 4.0).collect();
        cuts.sort_by(f32::total_cmp);
        cuts.dedup_by(|a, b| (*a - *b).abs() < 3.0);
        let mut bounds = vec![f32::MIN];
        bounds.extend(cuts);
        bounds.push(f32::MAX);
        let middle = |l: &Line| l.top + l.size / 2.0;
        let mut rows: Vec<Vec<crate::Cell>> = Vec::new();
        for (k, band) in bounds.windows(2).enumerate() {
            let of_band: Vec<Vec<&Line>> = parts.iter().map(|c| c.iter().filter(|l| middle(l) >= band[0] && middle(l) < band[1]).collect()).collect();
            if of_band.iter().all(Vec::is_empty) {
                continue;
            }
            // the head is one row; the body's rows are found
            let dealt = if k == 0 && bounds.len() > 2 { deal_rows(&of_band, &[0.0], st) } else { table_rows(&of_band, st) };
            for of_row in &dealt {
                let mut row = Vec::new();
                for mine in of_row {
                    let mut text: Vec<Inline> = Vec::new();
                    for l in mine.iter().copied() {
                        if !text.is_empty() {
                            push_text(&mut text, " ");
                        }
                        text.extend(line_inlines(doc, l, st));
                    }
                    row.push(crate::Cell { text: tidy(text), span: 1, ..Default::default() });
                }
                rows.push(row);
            }
        }
        if rows.len() >= 2 {
            // (with where the panel is: the picture of the table as it is set)
            out.push((first, end, Some(Block::Table { key: crate::table_key(page.number, &rows), rows, widths: Vec::new(), title: Vec::new(), note: Vec::new() }), area, goes_on));
            carried.head_taken |= !head.is_empty();
            if panel.bottom > 0.85 * page.height {
                carried.columns = Some(edges.clone());
            }
        }
    }
    out
}

/// The rows of a table from the lines of each of its columns (see
/// [`panel_tables`]): for each row, the lines of each column. The column
/// that has the most groups of lines standing apart shows the rows: each
/// of its groups is a cell. The lines of the other columns go with them,
/// in the one of two ways that fits the table better: its cells are set
/// in the middle of their row's height, or at its top. If no column
/// shows the rows (every column is one group), each line is a row.
fn table_rows<'a>(columns: &[Vec<&'a Line>], st: &Stats) -> Vec<Vec<Vec<&'a Line>>> {
    let middle = |l: &Line| l.top + l.size / 2.0;
    // (the top of the first and of the last line of each group)
    let groups = |column: &[&Line]| -> Vec<(f32, f32)> {
        let mut out: Vec<(f32, f32)> = Vec::new();
        for l in column {
            match out.last_mut() {
                // (the next line of the same cell: no more than a line and a third lower)
                Some((_, last)) if l.top - *last <= 1.35 * st.spacing.max(l.size) => *last = l.top,
                _ => out.push((l.top, l.top)),
            }
        }
        out
    };
    let size = columns.iter().flatten().map(|l| l.size).fold(st.body, f32::max);
    let best = columns.iter().filter(|c| !c.is_empty()).map(|c| (groups(c), c.len())).max_by(|a, b| a.0.len().cmp(&b.0.len()).then(b.1.cmp(&a.1)));
    let cells = match best {
        Some((cells, _)) if cells.len() > 1 => cells,
        // no column tells: a line a row, by the column with the most lines
        _ => {
            let centres: Vec<f32> = columns.iter().max_by_key(|c| c.len()).map(|c| c.iter().map(|l| middle(l)).collect()).unwrap_or_default();
            return deal_rows(columns, &centres, st);
        }
    };
    // cells in the middle of their rows
    let centres: Vec<f32> = cells.iter().map(|(a, b)| (a + b) / 2.0 + size / 2.0).collect();
    let centred = deal_rows(columns, &centres, st);
    let misfit_centred: f32 = centred
        .iter()
        .zip(&centres)
        .flat_map(|(row, c)| row.iter().filter_map(move |cell| Some(((cell.first()?.top + cell.last()?.top + size) / 2.0 - c).abs())))
        .sum();
    // cells at the top of their rows
    let tops: Vec<f32> = cells.iter().map(|c| c.0).collect();
    let mut topped: Vec<Vec<Vec<&Line>>> = vec![vec![Vec::new(); columns.len()]; tops.len()];
    for (c, column) in columns.iter().enumerate() {
        for l in column {
            let r = tops.iter().rposition(|t| l.top >= *t - 0.4 * size).unwrap_or(0);
            topped[r][c].push(*l);
        }
    }
    let misfit_topped: f32 = topped.iter().zip(&tops).flat_map(|(row, t)| row.iter().filter_map(move |cell| Some((cell.first()?.top - t).abs()))).sum();
    if misfit_topped < misfit_centred {
        topped
    } else {
        centred
    }
}

/// The lines of a table's columns dealt to its rows (they have their
/// middles at `centres`): for each row, the lines of each column. A line
/// goes to the row it lies in; where one row ends and the next begins is
/// between their middles, nearer the lower one's middle (rows are as high
/// as their highest cell, which is found by dealing twice).
fn deal_rows<'a>(columns: &[Vec<&'a Line>], centres: &[f32], st: &Stats) -> Vec<Vec<Vec<&'a Line>>> {
    let rows = centres.len();
    let middle = |l: &Line| l.top + l.size / 2.0;
    let mut heights = vec![1.0f32; rows];
    let mut dealt: Vec<Vec<Vec<&Line>>> = Vec::new();
    for _ in 0..3 {
        // the lower edge of each row but the last
        let edges: Vec<f32> = (0..rows.saturating_sub(1)).map(|r| centres[r] + (centres[r + 1] - centres[r]) * heights[r] / (heights[r] + heights[r + 1])).collect();
        dealt = vec![vec![Vec::new(); columns.len()]; rows];
        for (c, column) in columns.iter().enumerate() {
            for l in column {
                let r = edges.iter().position(|e| middle(l) < *e).unwrap_or(rows.saturating_sub(1));
                if r < rows {
                    dealt[r][c].push(*l);
                }
            }
        }
        // how high each row is: its cell of the most lines
        for (r, of_row) in dealt.iter().enumerate() {
            let most = of_row.iter().map(|cell| match (cell.first(), cell.last()) {
                (Some(a), Some(b)) => b.top - a.top + st.spacing.max(a.size),
                _ => 0.0,
            });
            heights[r] = most.fold(st.spacing.max(1.0), f32::max);
        }
    }
    dealt
}

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

/// A line of words only: no formula, nothing raised or lowered.
fn words_only(doc: &Doc, l: &Line) -> bool {
    l.spans.iter().all(|s| s.tex.is_none() && s.picture.is_none() && s.script == Script::Normal && !doc.is_math(s))
}

/// Width of what a line starts with and cannot be broken: its first word,
/// or a formula, with whatever is glued to it.
fn first_chunk_width(l: &Line) -> f32 {
    let mut w = 0.0;
    for (i, s) in l.spans.iter().enumerate() {
        if i > 0 && (s.left - l.spans[i - 1].right() > 0.15 * l.size || s.text.starts_with(char::is_whitespace)) {
            break;
        }
        let t = s.text.trim_start();
        if s.tex.is_some() || t.is_empty() {
            w += s.width;
            continue;
        }
        match t.find(char::is_whitespace) {
            Some(k) => {
                w += s.width * t[..k].chars().count() as f32 / s.text.chars().count().max(1) as f32;
                break;
            }
            None => w += s.width,
        }
    }
    w
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
    before_display: bool,
    st: &Stats,
) -> Option<f32> {
    let text = plain_text(l);
    let n = text.chars().count();
    if n == 0 || n > 120 || !text.chars().any(|c| c.is_alphabetic()) {
        return None;
    }
    // "Fig. 3.12 Energy levels": a caption, however short and bold
    if crate::links::starts_like_caption(&text) {
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
    // An unstyled heading: a short title-like line with space above and
    // below (and text below: a line right above a formula leads up to it).
    if !styled
        && !before_display
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
    // an italic sentence broken up by displayed equations (the statement of
    // a lemma: "which is analogous to", "in real Euclidian space.") is prose;
    // so is a styled line of three words or more that a displayed equation
    // follows at once ("But this can be rewritten as", "Theorem 6 Suppose"):
    // it leads up to the equation. (A title of a word or two can stand
    // right above one: "Case 2", "Appendix".)
    let leads_up = before_display && text.split_whitespace().count() >= 3;
    let fragment = text.starts_with(|c: char| c.is_lowercase()) || leads_up;
    let ok = styled
        && !fragment
        && (l.left - st.left).abs() <= 0.6 * st.body
        && l.right <= st.right - 3.0 * st.body
        && !text.ends_with([',', ';'])
        && space_above;
    ok.then_some(l.size)
}

/// Footnotes at the bottom of a page: the last lines, in a smaller font,
/// starting with a marker. Returns the index where they start.
/// `note_open`: the page before ended in a note (which may go on here).
fn page_notes_start(doc: &Doc, lines: &[Line], st: &Stats, note_open: bool) -> usize {
    // a line's type size, not counting equation images (given body size)
    let text_size = |l: &Line| {
        l.spans.iter().filter(|s| s.tex.is_none()).map(|s| doc.size_of(s)).fold(0.0f32, f32::max).max(if l.spans.iter().all(|s| s.tex.is_some()) { l.size } else { 0.0 })
    };
    let mut k = lines.len();
    while k > 0 && text_size(&lines[k - 1]) <= 0.9 * st.body {
        k -= 1;
    }
    // a whole page of small type: notes continued from the page before,
    // if a note is open (not a copyright page, and not a full-page picture
    // with its caption)
    if k == 0 && lines.len() >= 3 && note_open && !crate::links::starts_like_caption(&plain_text(&lines[0])) {
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
    // (from the top down; on a page in columns, column after column)
    lines.sort_by(|a, b| a.col.cmp(&b.col).then(a.top.total_cmp(&b.top)));
    lines
}

// ---------------------------------------------------------------------------
// Building

/// Room left behind a line (in type sizes, after the next line's first
/// word) from which the break is a hard return.
const HARD_RETURN_ROOM: f32 = 0.5;

struct Prev {
    left: f32,
    right: f32,
    top: f32,
    page: usize,
    ends_sentence: bool,
    /// its type size, if it is words only (no formula, nothing raised or
    /// lowered, which make a line taller)
    plain_size: Option<f32>,
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
    /// where in `para` the line breaks stand that the book made on purpose
    /// (hard returns): kept whether or not the paragraph is centered
    hard: Vec<usize>,
}

impl Builder<'_> {
    fn push(&mut self, block: Block) {
        self.out.push(Blk { block, page: self.page, centered: false });
    }

    fn flush(&mut self) {
        if !self.para.is_empty() {
            let p = std::mem::take(&mut self.para);
            let hard = std::mem::take(&mut self.hard);
            // (one line at least clearly in from the margins: an indented
            // one-line paragraph that ends early is centered by chance)
            let centered = self.para_centered && self.para_inset >= 2.5 * self.st.body;
            // not centered after all: its kept line breaks become spaces
            // (but not the hard returns)
            let p = if centered {
                p
            } else {
                p.into_iter().enumerate().map(|(k, i)| if i == Inline::Break && !hard.contains(&k) { Inline::Text(" ".into()) } else { i }).collect()
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

    /// A hard return before this line: the line above ends early although
    /// this line's first word would have fit behind it (text that wraps
    /// only breaks where the next word does not fit). `measure` is the
    /// right edge the text runs to. Returns the room left over, in type
    /// sizes, if any.
    fn room_above(&self, l: &Line, measure: f32, pi: usize) -> Option<f32> {
        let p = self.prev.as_ref()?;
        if p.page != pi || l.top <= p.top {
            return None;
        }
        let room = measure - (p.right + 0.3 * l.size + first_chunk_width(l));
        (room > 0.0).then_some(room / l.size)
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
        // a formula continued on this line ("$m_1 + m_2 =$" / "$m$"): one
        let ops = ['=', '+', '−', '-', '<', '>', '≈', '≡', '×', '·'];
        // (math set as text: whatever stands side by side with no word or
        // comma between is one formula, as it is within a line)
        let text_math = is_text_math(self.doc);
        if let (Some(Inline::Math(a)), Some(Inline::Math(b))) = (self.para.last_mut(), inl.first()) {
            if text_math || a.trim_end().ends_with(ops) || b.trim_start().starts_with(ops) {
                *a = format!("{} {}", a.trim_end(), b.trim_start());
                self.para.extend(inl.into_iter().skip(1));
                return;
            }
        }
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
            // (not a numbered title: "1.1.1 …" right under "1.1 …" is the
            // next entry, though its section begins at the same place)
            if page.starts_with('@') && pending.is_empty() && crate::structure::section_depth(&text).is_none() {
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
                entries.push(TocEntry { title, page, left: l.left, size: l.size, columns: false, number: String::new(), label: Vec::new(), part: false });
            }
        }
        // (each page of the contents keeps its entries: the page marks
        // stand between them as in the book)
        if !entries.is_empty() {
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
        hard: Vec::new(),
        para_inset: 0.0,
    };
    // printed page marks (ebook PDFs without page numbers; see layout.rs)
    let use_marks = pages.iter().all(|p| p.label.is_none());
    // (the columns of a table that goes on over the page)
    let mut carried_table = Carried::default();
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
        let note_open = matches!(b.out.last(), Some(Blk { block: Block::PageNote { .. }, .. }));
        let mut end = page_notes_start(doc, lines, st, note_open);
        // tables set on a panel of grey, with a rule under each cell of
        // their head: real tables. (A raised mark in such a table, or the
        // note under it on a panel of its own, is not where the page's
        // footnotes start: those are looked for below the panels.)
        // (a head that starts at the foot of this page: does the table of
        // the next page take its lines?)
        let next_takes = |head: &[Line]| {
            pages.get(pi + 1).filter(|_| !st.toc_pages.contains(&(pi + 1))).is_some_and(|next| {
                let lines = attach_fraction_rows(doc, &next.lines, &doc.pages[next.index].rules);
                let mut carried = Carried { head: head.to_vec(), ..Default::default() };
                panel_tables(doc, &doc.pages[next.index], &lines, st, &mut carried, &|_| false);
                carried.head_taken
            })
        };
        let panels = panel_tables(doc, &doc.pages[page.index], lines, st, &mut carried_table, &next_takes);
        let on_table = |i: usize, j: usize| panels.iter().any(|p| i < p.1 && p.0 < j);
        if let Some(last) = panels.iter().map(|t| t.1).max().filter(|last| end < *last) {
            let on_panel = |l: &Line| doc.pages[page.index].shades.iter().any(|(p, _)| l.top >= p.top - 3.0 && l.top < p.bottom && l.left >= p.left - 3.0 && l.right <= p.right + 3.0);
            let below = last + lines[last..].iter().take_while(|l| on_panel(l)).count();
            end = below + page_notes_start(doc, &lines[below..], st, false);
        }
        let mut disp = display_flags(doc, &lines[..end], st);
        // (a line of a table is no displayed formula, and none runs on into one)
        for (first, last, _, _, _) in &panels {
            for flag in disp.iter_mut().take(*last).skip(*first) {
                *flag = false;
            }
        }
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
        // (0: the page is not in columns; else the column being written)
        let mut col = 0;
        while i < end {
            let l = &lines[i];
            // a page in columns keeps them, side by side
            if l.col != col {
                b.flush();
                if col == 0 {
                    b.anchor_block();
                    b.push(Block::Columns(2));
                }
                if l.col == 0 {
                    b.push(Block::ColumnsEnd);
                } else if col != 0 || l.col > 1 {
                    b.push(Block::ColumnBreak);
                }
                col = l.col;
            }
            // program code or a rule (read from an image) above this line
            while ci < codes.len() && codes[ci].0 <= l.top {
                b.flush();
                b.anchor_block();
                b.push(codes[ci].1.clone());
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
            if let Some((_, j, table, area, goes_on)) = panels.iter().find(|t| t.0 == i) {
                b.flush();
                let mut j = *j;
                // (no table: lines of a head that the table of the next page takes)
                if let Some(table) = table {
                    // The rest of the table of the page before: where that
                    // table stands, if only its picture, the page's mark
                    // and the page's notes have come since.
                    let before = b.out.iter().rposition(|o| !matches!(o.block, Block::Picture { .. } | Block::Anchor(_) | Block::PageNote { .. })).filter(|k| *goes_on && matches!(b.out[*k].block, Block::Table { .. }));
                    b.anchor_block();
                    let mut table = table.clone();
                    // The caption on its panel on top of the table: the
                    // table's title, a cell across its columns.
                    if let (Some(start), Block::Table { title, .. }, None) = (caption_above(&b.out), &mut table, before) {
                        *title = tidy(take_caption(&mut b.out, start));
                    }
                    // The table's own note, on a panel right under it: the
                    // foot of the table, a cell across its columns (its
                    // lines are short only because the panel is narrow),
                    // and part of the picture.
                    let mut area = *area;
                    let under = |p: &Rule| p.top >= area.bottom - 4.0 && p.top <= area.bottom + st.body && p.left < area.right && p.right > area.left;
                    if let Some((note, _)) = doc.pages[page.index].shades.iter().find(|(p, light)| *light >= 0.7 && under(p)) {
                        let on_note = |l: &Line| l.top >= note.top - 3.0 && l.top < note.bottom && l.left >= note.left - 3.0 && l.right <= note.right + 3.0;
                        let last = j + lines[j..end].iter().take_while(|l| on_note(l)).count();
                        if last > j {
                            b.para_page = pi;
                            b.para_centered = false;
                            for (k, l) in lines[j..last].iter().enumerate() {
                                let inl = line_inlines(doc, l, st);
                                if k == 0 {
                                    b.para = inl;
                                } else {
                                    b.join(inl);
                                }
                            }
                            if let Block::Table { note, .. } = &mut table {
                                *note = tidy(std::mem::take(&mut b.para));
                            }
                            b.flush();
                            area = Figure { left: area.left.min(note.left), right: area.right.max(note.right), top: area.top, bottom: note.bottom };
                            j = last;
                        }
                    }
                    let picture = Block::Picture { page: number, area, alt: crate::TABLE_SET.into() };
                    match (before, table) {
                        // One table with the part of the page before: its
                        // rows go under that part's rows (the cells of that
                        // part keep their places, and the table its key),
                        // and its picture goes behind that part's picture.
                        (Some(k), Block::Table { rows: more, note: foot, .. }) => {
                            if let Block::Table { rows, note, .. } = &mut b.out[k].block {
                                rows.extend(more);
                                if !foot.is_empty() {
                                    *note = foot;
                                }
                            }
                            let at = k + 1 + b.out[k + 1..].iter().take_while(|o| matches!(o.block, Block::Picture { .. })).count();
                            let page = b.out[k].page;
                            b.out.insert(at, Blk { block: picture, page, centered: false });
                        }
                        (_, table) => {
                            b.push(table);
                            // (the printed table under it, to check it against)
                            b.push(picture);
                        }
                    }
                }
                prev_top = Some(lines[j - 1].top);
                last_heading = None;
                i = j;
                continue;
            }
            if let Some((_, j, cols)) = tables.iter().find(|t| t.0 == i && !on_table(t.0, t.1)).filter(|_| !b.index_mode) {
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
            // (a displayed equation, or a line that is nothing but a formula read from an image)
            let only_formula = |l: &Line| l.spans.iter().any(|s| s.tex.is_some()) && l.spans.iter().all(|s| s.tex.is_some() || s.text.trim().is_empty());
            let before_display = i + 1 < end && (disp[i + 1] || only_formula(&lines[i + 1]));
            if let Some(size) = heading_size(doc, l, prev_top, next_top, before_display, st) {
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
            // a hard return above this line: a line of its own, and a
            // paragraph of its own if there is space above it too
            let at_margin = |l: &Line| (l.left - st.left).abs() < 0.6 * st.body;
            let measure = if at_margin(l) {
                st.right
            } else {
                lines[..end].iter().filter(|o| (o.left - l.left).abs() < 0.6 * st.body && o.size == l.size).map(|o| o.right).fold(l.right, f32::max)
            };
            let starts = b.starts_para(l, next, pi);
            let room = if starts { None } else { b.room_above(l, measure, pi) };
            let hard = room.is_some_and(|r| r >= HARD_RETURN_ROOM);
            // more space above than between the lines of a paragraph in
            // this type size: after a hard return, or between two lines of
            // plain words (a formula makes its line taller, not spaced)
            let pitch = st.spacing * l.size / st.body;
            let spaced = !starts
                && b.prev.as_ref().is_some_and(|p| {
                    p.page == pi && l.top - p.top > 1.4 * pitch && (hard || (p.plain_size == Some(l.size) && words_only(doc, l)))
                });
            if starts || spaced {
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
                if centered || hard {
                    if hard {
                        b.hard.push(b.para.len());
                    }
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
                plain_size: words_only(doc, l).then_some(l.size),
                last_url: l.spans.last().and_then(|s| s.url.clone()),
            });
            prev_top = Some(l.top);
            i += 1;
        }
        if col != 0 {
            b.flush();
            b.push(Block::ColumnsEnd);
        }
        // code or a rule below the last line of text
        while ci < codes.len() {
            b.flush();
            b.anchor_block();
            b.push(codes[ci].1.clone());
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
    // where the equation pictures are on their pages
    let mut boxes: HashMap<String, (u32, Figure)> = HashMap::new();
    for page in &doc.pages {
        for s in page.lines.iter().flat_map(|l| &l.spans) {
            if let Some(k) = &s.image_key {
                boxes.insert(k.clone(), (page.number, Figure { left: s.left, right: s.right(), top: s.top, bottom: s.bottom() }));
            }
        }
    }
    picture_tables(b.out, &boxes)
}

#[cfg(test)]
mod table_tests {
    use super::*;
    use crate::layout::{Line, Script, Span};

    fn line(text: &str, top: f32) -> Line {
        let span = Span {
            text: text.into(),
            left: 0.0,
            top,
            width: 50.0,
            height: 25.0,
            font: 0,
            bold: false,
            italic: false,
            script: Script::Normal,
            overline: false,
            tex: None,
            link: None,
            url: None,
            url_text: None,
            link_top: None,
            picture: None,
            image_key: None,
        };
        crate::layout::make_line(vec![span], &|_| 25.0)
    }

    fn shown(rows: &[Vec<Vec<&Line>>]) -> Vec<Vec<String>> {
        rows.iter().map(|r| r.iter().map(|c| c.iter().map(|l| l.spans[0].text.clone()).collect::<Vec<_>>().join(" ")).collect()).collect()
    }

    #[test]
    fn an_array_read_from_a_picture_is_a_table() {
        let rows = array_table("\\begin{array}{c c c}{{}}&{{\\mathrm{Bosons}}}&{{\\mathrm{Fermions}}}\\\\ \\hline {{}}&{{a_{i}a_{j}=0}}&{{\\{a_{i},a_{j}\\}=0}}\\\\ m & \\begin{pmatrix} 1 & 2 \\\\ 3 & 4 \\end{pmatrix} & \\end{array}").unwrap();
        let shown: Vec<Vec<String>> = rows
            .iter()
            .map(|r| {
                r.iter()
                    .map(|c| match c.text.first() {
                        Some(Inline::Text(t)) => t.clone(),
                        Some(Inline::Math(m)) => format!("${m}$"),
                        _ => String::new(),
                    })
                    .collect()
            })
            .collect();
        assert_eq!(
            shown,
            [
                vec!["", "Bosons", "Fermions"],
                vec!["", "$a_{i}a_{j}=0$", "$\\{a_{i},a_{j}\\}=0$"],
                vec!["$m$", "$\\begin{pmatrix} 1 & 2 \\\\ 3 & 4 \\end{pmatrix}$", ""]
            ]
        );
        assert!(array_table("\\begin{array}{c} a \\\\ b \\end{array}").is_none());
        assert!(array_table("x = 1").is_none());
    }

    #[test]
    fn the_rows_of_a_table_are_found_by_how_its_cells_stand() {
        let st = Stats { left: 0.0, right: 500.0, body: 25.0, spacing: 26.0, chapter_size: None, toc_pages: Default::default(), hyphenated: Default::default() };
        // cells in the middle of their rows: a name beside two and three lines
        let names = [line("ket", 838.0), line("observable", 906.0), line("base", 998.0)];
        let texts = [line("a1", 826.0), line("a2", 852.0), line("b1", 880.0), line("b2", 906.0), line("b3", 932.0), line("c1", 960.0), line("c2", 986.0), line("c3", 1012.0), line("c4", 1037.0)];
        let columns = vec![names.iter().collect::<Vec<_>>(), texts.iter().collect()];
        assert_eq!(shown(&table_rows(&columns, &st)), [["ket", "a1 a2"], ["observable", "b1 b2 b3"], ["base", "c1 c2 c3 c4"]]);
        // cells at the top of their rows: the short cell stands beside the first line
        let names = [line("weak", 400.0), line("strong", 478.0)];
        let texts = [line("a1", 400.0), line("a2", 426.0), line("a3", 452.0), line("b1", 478.0), line("b2", 504.0)];
        let columns = vec![names.iter().collect::<Vec<_>>(), texts.iter().collect()];
        assert_eq!(shown(&table_rows(&columns, &st)), [["weak", "a1 a2 a3"], ["strong", "b1 b2"]]);
        // rows of one line each: a line a row
        let n = [line("1", 100.0), line("2", 128.0), line("3", 156.0)];
        let v = [line("2.320", 100.0), line("4.082", 128.0), line("5.517", 156.0)];
        let columns = vec![n.iter().collect::<Vec<_>>(), v.iter().collect()];
        assert_eq!(shown(&table_rows(&columns, &st)), [["1", "2.320"], ["2", "4.082"], ["3", "5.517"]]);
    }
}
