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
    /// Font size of the most common headings (the sections), if any.
    pub section_size: Option<f32>,
    /// Pages that hold the table of contents.
    pub toc_pages: HashSet<usize>,
    /// Hyphenated words seen inside lines ("wave-function"), used to decide
    /// whether a hyphen at a line end is part of the word.
    pub hyphenated: HashSet<String>,
    /// The words that stand whole within the lines (in small letters).
    pub words: HashSet<String>,
    /// An article, not a book: see [`stats`].
    pub article: bool,
    /// The first line of a paragraph is set in from the margin (so a line
    /// at the margin goes on with the paragraph before it).
    pub indents: bool,
    /// In a document unfolded from two columns: the margins of what runs
    /// across both columns (the title, the authors, the abstract).
    pub wide: Option<(f32, f32)>,
}

impl Stats {
    /// The margins a line is set between: the text's — or, for a line
    /// that runs out over the right margin of a column, the margins of
    /// what is set across both columns.
    fn frame(&self, l: &Line) -> (f32, f32) {
        match self.wide {
            Some(w) if l.right > self.right + self.body => w,
            _ => (self.left, self.right),
        }
    }

    /// A word cut by a hyphen at a line's end, "a-" and "b": is the hyphen
    /// the word's own ("four-dimensional")? It is if the book writes the
    /// word so within a line; or if it never writes the two as one word
    /// and both halves are words of their own in it.
    pub(crate) fn keeps_hyphen(&self, a: &str, b: &str) -> bool {
        let (a, b) = (a.to_lowercase(), b.to_lowercase());
        if self.hyphenated.contains(&format!("{a}-{b}")) {
            return true;
        }
        !self.words.contains(&format!("{a}{b}")) && self.words.contains(&a) && self.words.contains(&b)
    }
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
            // at least one real word (diagram labels like "x₁ x₂" are not titles),
            // in the line's size or, beside a larger number ("I.1"), smaller
            // but still large
            doc.size_of(s) >= (l.size - 0.5).min(1.3 * body)
                && !doc.is_math(s)
                && s.text
                    .split_whitespace()
                    .any(|w| w.chars().filter(|c| c.is_alphabetic()).count() >= 3)
        })
}

pub(crate) fn is_contents_title(t: &str) -> bool {
    matches!(key(t).as_str(), "contents" | "tableofcontents")
}

/// "Chapter 3", "Appendix B", "CHAPTER IV": what a chapter is called by,
/// on a line of its own above its name.
pub(crate) fn is_chapter_label(t: &str) -> bool {
    let mut w = t.split_whitespace();
    let (Some(kind), Some(n), None) = (w.next(), w.next(), w.next()) else { return false };
    let n = n.trim_end_matches(['.', ':']);
    matches!(kind.to_lowercase().as_str(), "chapter" | "appendix")
        && !n.is_empty()
        && ((n.len() <= 3 && n.chars().all(|c| c.is_ascii_digit())) || (n.len() == 1 && n.chars().all(|c| c.is_ascii_uppercase())) || (n.len() <= 5 && n.chars().all(|c| "IVXL".contains(c))))
}

pub fn stats(doc: &Doc, pages: &[PageInfo]) -> Stats {
    let body = doc.body_size;
    // (in two columns a full line is a column wide)
    let share = if doc.gutter.is_some() { 0.3 } else { 0.5 };
    let full = |p: &PageInfo, l: &Line| l.size == body && l.right - l.left > share * p.width;
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
    let mut titled: HashMap<i32, usize> = HashMap::new();
    // ("Chapter 3" alone on its line, in large type, its name under it)
    let mut labelled: HashMap<i32, usize> = HashMap::new();
    for (pi, p) in pages.iter().enumerate() {
        if toc_pages.contains(&pi) {
            continue;
        }
        for l in &p.lines {
            let k = (l.size * 10.0).round() as i32;
            let text: String = l.spans.iter().map(|s| s.text.as_str()).collect::<Vec<_>>().join(" ");
            let mut w = text.split_whitespace();
            // ("3", or the "IV.2" of a book that counts its chapters within parts)
            let number = w.next().is_some_and(|n| {
                (n.len() <= 2 && n.chars().all(|c| c.is_ascii_digit()))
                    || n.split_once('.').is_some_and(|(part, ch)| {
                        !part.is_empty() && part.len() <= 4 && part.chars().all(|c| "IVX".contains(c)) && !ch.is_empty() && ch.len() <= 2 && ch.chars().all(|c| c.is_ascii_digit())
                    })
            });
            let next = w.next();
            let big = big_text_line(doc, l, body);
            if big {
                *counts.entry(k).or_default() += 1;
            }
            if big && is_chapter_label(&text) {
                *labelled.entry(k).or_default() += 1;
            }
            // "3 Theory of …", or the "3" alone on its own line
            let alone = next.is_none() && l.size >= 1.12 * body && l.spans.iter().all(|s| !doc.is_math(s));
            // (a number with its title on the line says more than a number
            // alone, which the "0" on a figure's axis is, too)
            if number && big && next.is_some_and(|t| t.chars().next().is_some_and(char::is_uppercase)) {
                *titled.entry(k).or_default() += 1;
            } else if number && alone {
                *numbered.entry(k).or_default() += 1;
            }
        }
    }
    let section_size = counts.iter().filter(|&(_, &c)| c >= 2).max_by_key(|&(&s, &c)| (c, s)).map(|(&s, _)| s as f32 / 10.0);
    let largest = |sizes: &HashMap<i32, usize>| sizes.iter().filter(|&(_, &c)| c >= 2).map(|(&s, _)| s).max();
    let chapter_size = largest(&titled)
        .or_else(|| largest(&labelled))
        .or_else(|| largest(&numbered))
        .or_else(|| counts.into_iter().filter(|&(_, c)| c >= 2).max_by_key(|&(s, c)| (c, s)).map(|(s, _)| s))
        .map(|s| s as f32 / 10.0);

    let mut hyphenated = HashSet::new();
    let mut words = HashSet::new();
    for p in &doc.pages {
        // (the line before ended in a hyphen: its last word and the first
        // of this line are halves, not words)
        let mut cut = false;
        for l in &p.lines {
            let last = l.spans.len().saturating_sub(1);
            for (k, s) in l.spans.iter().enumerate() {
                let count = s.text.split_whitespace().count();
                for (n, w) in s.text.split_whitespace().enumerate() {
                    let half = (cut && k == 0 && n == 0) || (k == last && n + 1 == count && w.ends_with('-'));
                    let w = w.trim_matches(|c: char| !c.is_alphanumeric());
                    if !half && w.chars().count() >= 3 && w.chars().all(char::is_alphabetic) {
                        words.insert(w.to_lowercase());
                    }
                    if let Some((a, b)) = w.split_once('-') {
                        if !a.is_empty() && !b.is_empty() {
                            hyphenated.insert(w.to_lowercase());
                        }
                    }
                }
            }
            cut = l.spans.last().is_some_and(|s| s.text.trim_end().ends_with('-'));
        }
    }

    // An article: no chapters, and sections with numbers of their own
    // ("IV. THE UNISTOCHASTIC FORMULATION"). Its sections are what the
    // chapters are in a book: each a file of its own.
    let sections = pages.iter().flat_map(|p| p.lines.iter()).filter(|l| section_title(doc, l).is_some()).count();
    let article = chapter_size.is_none() && sections >= 2;
    let chapter_size = if article { Some(ARTICLE_SECTION) } else { chapter_size };
    // Are the paragraphs' first lines set in? Counted where a paragraph
    // clearly ends: a line that stops short with a full stop, and the
    // next line right under it.
    let (mut set_in, mut flat) = (0, 0);
    for (pi, p) in pages.iter().enumerate() {
        if toc_pages.contains(&pi) {
            continue;
        }
        for w in p.lines.windows(2) {
            let d = w[1].top - w[0].top;
            let ends = w[0].right < right - 3.0 * body && w[0].right - w[0].left > 0.2 * (right - left) && (w[0].left - left).abs() < 0.6 * body && ends_sentence(&plain_text(&w[0]));
            if w[0].size == body && w[1].size == body && d >= 0.8 * spacing && d <= 1.6 * spacing && ends && w[1].right >= right - body {
                match w[1].left - left {
                    x if x.abs() < 0.6 * body => flat += 1,
                    x if x > 0.0 && x <= 4.0 * body => set_in += 1,
                    _ => {}
                }
            }
        }
    }
    let indents = set_in >= 10 && set_in >= 4 * flat;
    if std::env::var_os("PDF2MD_STATSDEBUG").is_some() {
        eprintln!("STATS left {left} right {right} body {body} spacing {spacing} chapter {chapter_size:?} section {section_size:?} article {article} gutter {:?} paragraphs set in {set_in}, not {flat}", doc.gutter);
    }
    Stats { left, right, body, spacing, chapter_size, section_size, toc_pages, hyphenated, words, article, indents, wide: (doc.column_offset > 0.0).then_some((left, right + doc.column_offset)) }
}

/// The size a section title of an article is given: larger than any
/// type, so that it heads its file as a chapter's title does.
pub const ARTICLE_SECTION: f32 = 999.0;

fn bold_words(doc: &Doc, l: &Line) -> bool {
    let words: Vec<_> = l.spans.iter().filter(|s| !doc.is_math(s) && s.text.chars().any(|c| c.is_alphabetic())).collect();
    !words.is_empty() && words.iter().all(|s| s.bold || crate::layout::is_bold_font(doc.family(s)))
}

/// The title of a numbered section of an article, in bold: "IV. THE
/// UNISTOCHASTIC FORMULATION OF QUANTUM THEORY", "2. Method". Its number.
pub(crate) fn section_title(doc: &Doc, l: &Line) -> Option<u32> {
    let text = plain_text(l);
    let (number, title) = text.trim().split_once(". ")?;
    let n = if !number.is_empty() && number.len() <= 2 && number.chars().all(|c| c.is_ascii_digit()) {
        number.parse().ok()?
    } else if !number.is_empty() && number.len() <= 5 && number.chars().all(|c| "IVXL".contains(c)) {
        roman(number)?
    } else {
        return None;
    };
    let titled = title.trim().starts_with(char::is_uppercase) && title.chars().count() <= 100 && !title.trim_end().ends_with(['.', ',', ';', ':']);
    (titled && bold_words(doc, l)).then_some(n)
}

pub(crate) fn roman(t: &str) -> Option<u32> {
    let value = |c: char| match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        _ => 50,
    };
    let d: Vec<i64> = t.chars().map(value).collect();
    u32::try_from((0..d.len()).map(|k| if d.get(k + 1).is_some_and(|n| *n > d[k]) { -d[k] } else { d[k] }).sum::<i64>()).ok().filter(|n| *n > 0)
}

/// "Subject Matter . . . . 1" -> ("Subject Matter", "1").
/// The equation number in a reference: "(3.422)", "(3.422", "3.425b" ->
/// "3.422" … (chapter or appendix letter, dot, number, optional letter).
pub fn equation_ref(t: &str) -> Option<&str> {
    let t = t.trim().trim_start_matches(['[', '(']).trim_end_matches([')', ']', ',', ';', '.']);
    let (a, b) = t.split_once('.')?;
    let b = b.trim_end_matches(|c: char| c.is_ascii_lowercase());
    // ("3.422", or numbered by section: "9.4.18")
    let digits = |x: &str| !x.is_empty() && x.chars().all(|c| c.is_ascii_digit());
    let number = digits(b) || b.split_once('.').is_some_and(|(x, y)| digits(x) && digits(y));
    (!a.is_empty() && a.len() <= 2 && a.chars().all(|c| c.is_ascii_alphanumeric()) && number).then_some(t)
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

/// Names of functions that a book sets in the text's font, in a sentence:
/// before a formula they are part of it. (Not "max" or "sup", which are
/// words too.)
const FUNCTION_NAMES: [&str; 20] = ["lim", "log", "ln", "sin", "cos", "tan", "cot", "sinh", "cosh", "tanh", "coth", "exp", "det", "tr", "Tr", "sgn", "Re", "Im", "arcsin", "arctan"];

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
        let trail = t.ends_with(['(', '[', '˜', 'ˆ', '˙', '¨', 'ˇ', '¯']) && t.chars().count() > 1 && !math;
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
    // A number at the end of a piece of prose with an exponent or index
    // right after it ("of dimension 5" and a raised ∗, "is 2" and a raised
    // n−1) is the base of that script: it is cut off the words before it,
    // so that it joins the formula.
    let mut k = 0;
    while k + 1 < pieces.len() {
        let (s, next) = (&pieces[k], &pieces[k + 1]);
        let chars: Vec<char> = s.text.trim_end().chars().collect();
        let digits = chars.iter().rev().take_while(|c| c.is_ascii_digit()).count();
        let cut = chars.len() - digits;
        let based = next.script != Script::Normal && s.script == Script::Normal && !doc.is_math(s) && (next.left - s.right()).abs() < 0.25 * st.body;
        if based && digits > 0 && cut > 0 && chars[cut - 1].is_whitespace() {
            let n = s.text.chars().count().max(1) as f32;
            let split_x = s.left + s.width * cut as f32 / n;
            let (mut words, mut number) = (s.clone(), s.clone());
            (words.text, words.width) = (chars[..cut].iter().collect(), split_x - s.left);
            (number.text, number.left, number.width) = (chars[cut..].iter().collect(), split_x, s.right() - split_x);
            pieces.splice(k..=k, [words, number]);
            k += 1;
        }
        k += 1;
    }
    // The name of a function before its formula ("is log" and (Λ/m), "lim"
    // and a lowered K→∞) is the formula's first word: it is cut off the
    // words before it, and joins the formula.
    let mut k = 0;
    while k + 1 < pieces.len() {
        let (s, next) = (&pieces[k], &pieces[k + 1]);
        let trimmed = s.text.trim_end();
        let word = trimmed.rsplit(char::is_whitespace).next().unwrap_or("");
        let cut = trimmed.chars().count() - word.chars().count();
        // (a figure set against the name, "−2tr(…)": the name begins behind it)
        let figures = word.chars().take_while(char::is_ascii_digit).count();
        let (word, cut) = if figures > 0 && FUNCTION_NAMES.contains(&&word[figures..]) { (&word[figures..], cut + figures) } else { (word, cut) };
        let before_math = (doc.is_math(next) || next.script != Script::Normal) && next.left - s.right() < 0.5 * st.body;
        // (so is a capital Greek letter of the text font that has indices,
        // with the bracket that opens before it: "Then (Φ" and a raised *)
        let greek = word.trim_start_matches(['(', '[']);
        let symbol = !greek.is_empty() && greek.chars().all(|c| "ΓΔΘΛΞΠΣΥΦΨΩ".contains(c)) && next.script != Script::Normal;
        if cut > 0 && (FUNCTION_NAMES.contains(&word) || symbol) && before_math && !doc.is_math(s) && s.script == Script::Normal && !s.bold && !s.italic {
            let n = s.text.chars().count().max(1) as f32;
            let split_x = s.left + s.width * cut as f32 / n;
            let (mut words, mut name) = (s.clone(), s.clone());
            (words.text, words.width) = (s.text.chars().take(cut).collect(), split_x - s.left);
            (name.text, name.left, name.width) = (word.to_string(), split_x, s.right() - split_x);
            pieces.splice(k..=k, [words, name]);
            k += 1;
        }
        k += 1;
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
            // (a note's sign may be of a math font: the star of an author's name)
            let sign = !t.is_empty() && t.chars().all(|c| "∗†‡".contains(c));
            if s.script == Script::Super && (!math || sign) {
                // (after a bracket that closes a formula, "(pc)" and a
                // raised 2, it is the formula's power: the bracket alone in
                // the text font, with math before it)
                let closes_math = i >= 2 && matches!(spans[i - 1].text.trim(), ")" | "]" | "|") && (doc.is_math(&spans[i - 2]) || spans[i - 2].script != Script::Normal);
                // (several marks in a row, "1, 2," and a star: the places an
                // author works at, and a note)
                let marks = |t: &str| t.starts_with(|c: char| c.is_ascii_digit()) && t.len() <= 8 && t.contains(',') && t.chars().all(|c| c.is_ascii_digit() || c == ',' || c == ' ');
                let after_marks = i > 0 && spans[i - 1].script == Script::Super && !doc.is_math(&spans[i - 1]) && marks(spans[i - 1].text.trim()) && gap(i - 1, i) < 0.5 * st.body;
                let after_word = (i > 0
                    && !doc.is_math(&spans[i - 1])
                    && spans[i - 1].script == Script::Normal
                    && !closes_math
                    && gap(i - 1, i) < 0.5 * st.body)
                    || after_marks;
                let stacked = spans
                    .get(i + 1)
                    .is_some_and(|nx| (nx.left - s.left).abs() < 3.0 && (nx.script == Script::Sub || (nx.top >= s.bottom() - 1.0 && nx.height < 0.85 * l.size && nx.top < s.bottom() + 0.5 * l.size)));
                let digits = !t.is_empty() && t.len() <= 3 && t.bytes().all(|b| b.is_ascii_digit());
                let symbol = !t.is_empty() && t.chars().all(|c| "*∗†‡§¶".contains(c));
                let list = marks(t) && i > 0 && spans[i - 1].text.trim_end().ends_with(char::is_alphabetic);
                let ordinal = matches!(t, "st" | "nd" | "rd" | "th");
                // (a raised number on a number is its power where the
                // exponent goes on in math, 2 and a raised 10/2 − 1, or the
                // number follows a sign of the formula, "= 2" and a raised 4)
                let on_number = i > 0 && !spans[i - 1].text.trim().is_empty() && spans[i - 1].text.trim().bytes().all(|b| b.is_ascii_digit());
                let goes_on = spans.get(i + 1).is_some_and(|nx| nx.script == Script::Super && doc.is_math(nx) && gap(i, i + 1) < 0.15 * st.body);
                let in_formula = i >= 2 && doc.is_math(&spans[i - 2]) && gap(i - 2, i - 1) < 0.5 * st.body;
                let power = digits && on_number && (goes_on || in_formula);
                // (a star of a math font is a note's sign only behind such
                // marks or a word: on a letter it is the letter's own)
                let on_word = i > 0 && spans[i - 1].text.trim_end().chars().rev().take_while(|c| c.is_lowercase()).count() >= 3;
                let sign_ok = !math || after_marks || on_word || i == 0;
                if ((digits || symbol || list) && sign_ok && (after_word || i == 0) && !stacked && !power) || (ordinal && i > 0) {
                    return K::Marker;
                }
            }
            // an accent drawn over a math symbol (˜ over a) is part of it
            let accent_over_math = matches!(t, "˜" | "ˆ" | "˙" | "¨" | "¯" | "ˇ")
                && spans.iter().any(|o| doc.is_math(o) && o.left < s.right() && o.right() > s.left);
            // (one letter alone in a bold font is a symbol of a formula: a
            // vector **r**, with whatever index it bears)
            // (not in a line that is bold altogether, a title)
            let is_bold = |s: &crate::layout::Span| s.bold || crate::layout::is_bold_font(doc.family(s));
            let bold_letter = is_bold(s) && t.len() == 1 && t.starts_with(|c: char| c.is_ascii_alphabetic()) && s.link.is_none() && spans.iter().any(|o| !doc.is_math(o) && !is_bold(o) && o.text.chars().filter(|c| c.is_alphabetic()).count() >= 3);
            if math || s.script != Script::Normal || accent_over_math || bold_letter {
                K::Math
            } else if !reference(i) && !t.is_empty() && t.chars().all(|c| c.is_ascii_digit() || "=+−-–()[]/<>|,.;:·×*!′".contains(c)) {
                K::Op
            } else {
                K::Text
            }
        })
        .collect();
    // TeX sets the capital Greek letters of a formula from its text font:
    // one alone is a symbol.
    for i in 0..n {
        let text = spans[i].text.trim();
        // (or with the bracket that opens before it, "(Φ" and its indices)
        let greek = text.trim_start_matches(['(', '[']);
        let scripted = spans.get(i + 1).is_some_and(|nx| nx.script != Script::Normal);
        if kinds[i] == K::Text && !greek.is_empty() && greek.chars().all(|c| "ΓΔΘΛΞΠΣΥΦΨΩ".contains(c)) && (greek == text || scripted) {
            kinds[i] = K::Math;
        }
    }
    // The name of a function right before math is the formula's.
    for i in 0..n.saturating_sub(1) {
        let s = &spans[i];
        if kinds[i] == K::Text && kinds[i + 1] == K::Math && FUNCTION_NAMES.contains(&s.text.trim()) && !s.bold && !s.italic && gap(i, i + 1) < 0.5 * st.body {
            kinds[i] = K::Math;
        }
    }
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
        // (in smaller type, a note or an exercise, the space is smaller too)
        let space = prev_right.is_some_and(|r| s.left - r > 0.15 * st.body.min(l.size));
        // (between a word and a formula a space may be squeezed to two
        // points of the page's grid, where the pieces' places are whole
        // numbers: still a space, but for "th" and "s" set against a letter)
        let space = space || {
            let gap = prev_right.map_or(0.0, |r| s.left - r);
            let word_before = i > 0 && kinds[i - 1] != K::Math && kinds[i] == K::Math && spans[i - 1].text.ends_with(char::is_alphabetic);
            let first: String = s.text.chars().take_while(|c| c.is_alphabetic()).collect();
            let word_after = i > 0 && kinds[i - 1] == K::Math && kinds[i] == K::Text && !first.is_empty() && !["th", "st", "nd", "rd", "s"].contains(&first.as_str());
            (word_before || word_after) && gap >= 0.12 * st.body.min(l.size) && gap >= 1.5
        };
        // (a comma or full stop after an italic letter stands a little off it)
        let closes = kinds[i] != K::Math && s.text.starts_with([',', '.', ';', ':', ')', ']', '?', '!']) && !s.text.starts_with(". .") && !s.text.starts_with("..");
        let space = space && !(closes && prev_right.is_some_and(|r| s.left - r <= 0.3 * st.body.min(l.size)));
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
                // A run of small raised pieces right after a word is the
                // word's power: (length) and a raised ½(D − 2). (Its
                // fraction bar lies well above the line's math axis.)
                let raised = spans[i].script == Script::Super
                    && i > 0
                    && kinds[i - 1] == K::Text
                    && spans[i].left - spans[i - 1].right() < 0.25 * st.body
                    && spans[i..j].iter().all(|s| doc.size_of(s) < 0.85 * doc.size_of(&spans[i - 1]))
                    && (bars.iter().any(|b| spans[i - 1].top + 0.8 * spans[i - 1].height - b.top > 0.5 * doc.size_of(&spans[i - 1])) || bars.is_empty());
                // (told by its stacked fraction, or a prime alone: other
                // raised runs are mostly limits that lost their line)
                let t = if raised && (t.starts_with("\\tfrac") || t == "\\prime") { format!("{{}}^{{{t}}}") } else { t };
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
                    // (an address whose "_" is a drawn line, not a letter:
                    // written as the link has it)
                    let address = href.strip_prefix("mailto:").filter(|a| a.replace('_', " ") == link.trim() || a.replace('_', "") == link.trim());
                    out.push(Inline::Link { text: crate::emit::escape_text(address.unwrap_or(link)), href: href.clone() });
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
            // (not a reference the PDF links, a number in italic text: it
            // stays plain, and is made a link further on)
            _ if (s.italic || crate::layout::is_italic_font(doc.family(s))) && s.link.is_none() => {
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
                out.push(Blk { block: Block::Picture { page: number, area, alt: crate::TABLE_MAYBE.into() }, page, centered: false, goes_on: false });
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
            out.push(Blk { block: Block::Para(tidy(before)), page, centered, goes_on: false });
        }
        // (keyed by the picture it was read from)
        let table = Block::Table { key: crate::table_key(boxes.get(&key).map_or(0, |b| b.0), &rows), rows, widths: Vec::new(), title, note: Vec::new() };
        out.push(Blk { block: table, page, centered: false, goes_on: false });
        if let Some((number, area)) = boxes.get(&key) {
            out.push(Blk { block: Block::Picture { page: *number, area: *area, alt: crate::TABLE_READ.into() }, page, centered: false, goes_on: false });
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

/// A title whose number, or whose first letter, the book sets in a drawn
/// frame ("I.1" in a box before "Who Needs It?", the Q of "Quantum Field
/// Theory"): the title with that part marked as framed. A title that
/// stands in the frame whole is framed whole.
fn framed_title(doc: &Doc, page: &crate::layout::Page, l: &Line, st: &Stats) -> Option<Vec<Inline>> {
    let first = l.spans.first()?;
    let frame = page.frames.iter().find(|f| first.left >= f.left - 3.0 && first.left < f.right && first.top < f.bottom && first.bottom() > f.top)?;
    let line_of = |spans: Vec<crate::layout::Span>| crate::layout::make_line(spans, &|s| doc.size_of(s));
    let (inside, rest): (Vec<_>, Vec<_>) = l.spans.iter().cloned().partition(|s| s.left + s.width / 2.0 <= frame.right + 1.0);
    let mut out = Vec::new();
    // (the line the book draws under the title, from the frame's foot on)
    if page.rules.iter().any(|r| (r.top - frame.bottom).abs() <= 3.0 && r.left <= frame.left + 3.0 && r.right >= frame.right + 60.0) {
        out.push(Inline::Ruled);
    }
    if inside.is_empty() {
        // (only the first letter is in the frame: the piece of text goes on)
        let mut chars = first.text.chars();
        let letter = chars.next()?;
        let mut rest = l.spans.clone();
        rest[0].text = chars.collect();
        out.push(Inline::Framed(letter.to_string()));
        out.extend(crate::no_emph(line_inlines(doc, &line_of(rest), st)));
        return Some(out);
    }
    out.push(Inline::Framed(plain(&line_inlines(doc, &line_of(inside), st))));
    if !rest.is_empty() {
        out.push(Inline::Text(" ".into()));
        out.extend(crate::no_emph(line_inlines(doc, &line_of(rest), st)));
    }
    Some(out)
}

/// Text in a frame of drawn lines, or in a grid of them: a table, whose
/// cells are the fields of the grid (a frame alone: one cell).
///
/// A line of the page that crosses several fields is cut at the upright
/// lines. A cell of symbols only is one formula, whatever lines it was set
/// in (a sum sign with its index under it); a cell with words keeps its
/// lines where the frame has one column, as an indented line is part of
/// how boxed text is set.
///
/// The number of the equation at the margin beside it is given apart, by
/// the table's first line: it is written under the table.
type Found = (usize, usize, Option<Block>, Figure, bool);
fn grid_tables(doc: &Doc, page: &crate::layout::Page, lines: &[Line], st: &Stats) -> (Vec<Found>, HashMap<usize, String>) {
    let mut out = Vec::new();
    let mut tags = HashMap::new();
    for g in &page.grids {
        let (left, right, top, bottom) = (g.xs[0], g.xs[g.xs.len() - 1], g.ys[0], g.ys[g.ys.len() - 1]);
        // (what stands right of the frame on a line of it: the equation's number)
        // (… not the comma or full stop that closes the equation, just outside the frame)
        let beside = |l: &Line| -> Vec<String> { l.spans.iter().filter(|s| s.left >= right - 1.0).map(|s| s.text.trim().to_string()).filter(|t| !t.is_empty() && !t.chars().all(|c| ".,;:".contains(c))).collect() };
        let inside = |l: &Line| {
            let out = beside(l);
            // (the number may be a line of its own, at the middle of the frame's height)
            let number_only = l.left >= right - 1.0 && out.len() == 1 && mathtex::tag_of(&out[0]).is_some();
            l.top >= top - 3.0 && l.top < bottom && ((l.left >= left - 3.0 && l.left < right && (l.right <= right + 0.6 * l.size || (out.len() == 1 && mathtex::tag_of(&out[0]).is_some()))) || number_only)
        };
        if std::env::var_os("PDF2MD_TABLEDEBUG").is_some() {
            eprintln!("GRID {left:.0}..{right:.0} x {top:.0}..{bottom:.0} ({} x {})", g.xs.len() - 1, g.ys.len() - 1);
            for l in lines.iter().filter(|l| l.top >= top - 40.0 && l.top <= bottom + 10.0) {
                eprintln!("  line {:.0}..{:.0} top {:.0} size {:.0} inside {} {:?}", l.left, l.right, l.top, l.size, inside(l), l.spans.iter().map(|s| s.text.chars().take(6).collect::<String>()).collect::<Vec<_>>());
            }
        }
        let Some(first) = lines.iter().position(inside) else { continue };
        let end = first + lines[first..].iter().take_while(|l| inside(l)).count();
        // (they must be a run of the page's lines)
        if lines[end..].iter().any(inside) {
            continue;
        }
        let (rows_n, cols_n) = (g.ys.len() - 1, g.xs.len() - 1);
        // A frame alone around a displayed equation (its number beside the
        // frame, or nothing in it but symbols) is that equation in a box,
        // not a table of one cell: built as any displayed equation is, of
        // however many lines, with the bars drawn inside the frame.
        if rows_n == 1 && cols_n == 1 {
            let within: Vec<&crate::layout::Span> = lines[first..end].iter().flat_map(|l| l.spans.iter()).filter(|s| s.left < right - 1.0).collect();
            let number = lines[first..end].iter().flat_map(|l| l.spans.iter()).filter(|s| s.left >= right - 1.0).find_map(|s| mathtex::tag_of(s.text.trim()));
            // (a few words among many symbols, "if" and "otherwise" and the
            // names of units, are of the equation; a framed sentence stays text)
            let words = within.iter().filter(|s| !doc.is_math(s)).flat_map(|s| s.text.split_whitespace()).filter(|w| w.chars().filter(|c| c.is_alphabetic()).count() >= 3 && !crate::mathtex::is_function_word(w)).count();
            let symbols = within.iter().filter(|s| doc.is_math(s)).count();
            if symbols > 0 && (words == 0 || symbols >= 3 * words) {
                let spans: Vec<MSpan> = within.iter().map(|s| MSpan::new(doc, s)).collect();
                let bars: Vec<Rule> = page.rules.iter().filter(|r| r.left >= left + 2.0 && r.right <= right - 2.0 && r.top >= top + 2.0 && r.bottom <= bottom - 2.0).copied().collect();
                let eqs = mathtex::display(spans, &bars, st.right, st.body);
                if let [(latex, _)] = &eqs[..] {
                    if !latex.is_empty() {
                        let area = Figure { left, right, top, bottom };
                        out.push((first, end, Some(Block::Equation { latex: format!("\\boxed{{{latex}}}"), tag: number, source: None }), area, false));
                        continue;
                    }
                }
            }
        }
        let mut cells: Vec<Vec<Vec<Line>>> = vec![vec![Vec::new(); cols_n]; rows_n];
        for l in &lines[first..end] {
            let mid = l.top + l.size / 2.0;
            let r = (0..rows_n).rfind(|&r| mid >= g.ys[r] - 3.0).unwrap_or(0);
            let mut of_col: Vec<Vec<crate::layout::Span>> = vec![Vec::new(); cols_n];
            // (a piece of text that runs across an upright line, the
            // symbols of three fields with wide gaps between them, is cut
            // into its words)
            let crosses = |s: &crate::layout::Span| g.xs[1..cols_n].iter().any(|x| s.left < x - 2.0 && s.right() > x + 2.0);
            let pieces: Vec<crate::layout::Span> = l.spans.iter().flat_map(|s| if crosses(s) { crate::layout::split_span_words(s) } else { vec![s.clone()] }).collect();
            for s in &pieces {
                let c = s.left + s.width / 2.0;
                if s.left >= right - 1.0 {
                    if let Some(tag) = mathtex::tag_of(s.text.trim()) {
                        tags.insert(first, tag);
                    }
                    continue;
                }
                of_col[(0..cols_n).rfind(|&k| c >= g.xs[k]).unwrap_or(0)].push(s.clone());
            }
            for (c, spans) in of_col.into_iter().enumerate() {
                if spans.is_empty() {
                    continue;
                }
                let mut piece = crate::layout::make_line(spans, &|s| doc.size_of(s));
                piece.bars = l.bars.iter().filter(|b| b.left >= g.xs[c] - 2.0 && b.right <= g.xs[c + 1] + 2.0).copied().collect();
                cells[r][c].push(piece);
            }
        }
        let rows: Vec<Vec<crate::Cell>> = cells
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|pieces| {
                        let words = pieces.iter().flat_map(|l| l.spans.iter()).any(|s| !doc.is_math(s) && s.text.split_whitespace().any(|w| w.chars().filter(|c| c.is_alphabetic()).count() >= 3));
                        let mut text: Vec<Inline> = Vec::new();
                        if !words && !pieces.is_empty() {
                            let spans: Vec<MSpan> = pieces.iter().flat_map(|l| l.spans.iter()).map(|s| MSpan::new(doc, s)).collect();
                            let bars: Vec<Rule> = pieces.iter().flat_map(|l| l.bars.iter().copied()).collect();
                            text.push(Inline::Math(mathtex::fix_double_scripts(&mathtex::build_with(&spans, &bars))));
                        } else {
                            for (k, l) in pieces.iter().enumerate() {
                                if k > 0 && cols_n == 1 {
                                    text.push(Inline::Break);
                                } else if k > 0 {
                                    push_text(&mut text, " ");
                                }
                                text.extend(line_inlines(doc, l, st));
                            }
                        }
                        crate::Cell { text: tidy(text), span: 1, ..Default::default() }
                    })
                    .collect()
            })
            .collect();
        let area = Figure { left, right, top, bottom };
        out.push((first, end, Some(Block::Table { key: crate::table_key(page.number, &rows), rows, widths: Vec::new(), title: Vec::new(), note: Vec::new() }), area, false));
    }
    (out, tags)
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
            // (no column begins right behind a root sign: the sign and what
            // it is the root of are one cell)
            let radical = |t: &crate::layout::Span| t.text.trim() == "√" || (matches!(t.text.trim(), "p" | "q" | "r" | "s") && doc.family(t).starts_with("CMEX")) || (t.text.trim() == "p" && doc.family(t).starts_with("CMSY"));
            let ok = run.iter().all(|l| {
                let starts = l.spans.iter().any(|t| (t.left - x).abs() <= 6.0);
                let crosses = l.spans.iter().any(|t| t.left < x - 6.0 && t.right() > x + 2.0);
                // (the sign's box is narrower than the sign: nothing else
                // stands between it and the column)
                let under_root = l.spans.iter().any(|t| radical(t) && t.left < x - 6.0 && x - t.right() <= st.body && !l.spans.iter().any(|o| !radical(o) && o.left > t.left && o.left < x - 6.0));
                starts && !crosses && !under_root
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
fn table_latex(doc: &Doc, rules: &[Rule], lines: &[Line], cols: &[f32]) -> String {
    let spec = if cols.len() >= 3 {
        format!("l{}l", "c".repeat(cols.len() - 2))
    } else {
        "l".repeat(cols.len())
    };
    if std::env::var_os("PDF2MD_TABLEDEBUG").is_some() {
        for l in lines {
            eprintln!("TROW top {:.0} {:?}", l.top, l.spans.iter().map(|s| format!("{}@{:.0},{:.0}", s.text, s.left, s.top)).collect::<Vec<_>>());
        }
    }
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
                        // (the bars drawn in the cell: of its small fractions and its roots)
                        let (x0, x1, y0, y1) = ms.iter().fold((f32::MAX, f32::MIN, f32::MAX, f32::MIN), |(a, b, c, d), s| (a.min(s.left), b.max(s.right), c.min(s.top), d.max(s.bottom)));
                        let bars: Vec<Rule> = rules.iter().filter(|r| r.left >= x0 - 3.0 && r.right <= x1 + 3.0 && r.top >= y0 - 3.0 && r.bottom <= y1 + 3.0).copied().collect();
                        mathtex::build_with(&ms, &bars)
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
    // (the raised marks behind a name, "1, 2," and a star, are no math)
    let mut named = false;
    for s in &l.spans {
        let c = s.text.chars().filter(|c| !c.is_whitespace()).count();
        let mark = s.script == Script::Super && !s.text.trim().is_empty() && s.text.chars().all(|c| c.is_ascii_digit() || c.is_whitespace() || ",*∗†‡§¶".contains(c));
        if mark && named {
            continue;
        }
        named = !doc.is_math(s) && s.script == Script::Normal && s.text.trim_end().ends_with(|c: char| c.is_alphabetic() || c == '.' || c == ',') && s.text.chars().filter(|c| c.is_alphabetic()).count() >= 3;
        if doc.is_math(s) || s.script != Script::Normal {
            mathc += c;
        } else {
            // (the name of a function, "cos", is part of its formula)
            let named: usize = s.text.split_whitespace().filter(|w| mathtex::is_function_word(w.trim_matches(|c: char| !c.is_alphabetic()))).map(|w| w.chars().filter(|c| c.is_alphabetic()).count()).sum();
            letters += s.text.chars().filter(|c| c.is_alphabetic()).count() - named;
            mathc += named;
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
            // (a line set in with a sum or integral sign of the large kind
            // is set apart, with the few words of a label beside it)
            let big_sign = l.spans.iter().any(|s| doc.family(s).starts_with("CMEX") && matches!(s.text.trim(), "P" | "X" | "Z" | "R" | "Q" | "Y"));
            tag || (words <= 2 && ((indented(l) && ratio >= 0.25) || ratio >= 0.7)) || (big_sign && words <= 4 && indented(l) && ratio >= 0.25)
        })
        .collect();
    // A line that opens with a word at the margin, in the size of the
    // text, goes on with the sentence after a formula set apart ("with
    // F_μν ≡ ∂_μ A_ν − ∂_ν A_μ."), however much of it is math.
    for i in 0..lines.len() {
        let l = &lines[i];
        let word_first = l.spans.first().is_some_and(|s| {
            let w = s.text.split_whitespace().next().unwrap_or("");
            !doc.is_math(s) && s.script == Script::Normal && s.tex.is_none() && w.chars().count() >= 2 && w.chars().all(char::is_lowercase)
        });
        if f[i] && !info[i].2 && word_first && (l.left - st.left).abs() < 1.5 && l.size >= 0.9 * st.body {
            f[i] = false;
        }
    }
    // A line of a bulleted item that opens with a word where the item's
    // words begin ("• Additive Identity:" and under it "For a ∈ ℝ, a + 0 =
    // a.") is a line of the item's text, however much of it is math: a
    // formula set apart stands in the middle, not at the item's left.
    let mut item: Option<f32> = None;
    for i in 0..lines.len() {
        let l = &lines[i];
        let Some(first) = l.spans.first() else { continue };
        if first.text.trim_start().starts_with('•') {
            // (where its words begin: after the bullet, in its piece or in the next)
            let own = first.text.trim_start().trim_start_matches('•').trim();
            item = Some(if own.is_empty() { l.spans.get(1).map_or(l.left + 0.9 * l.size, |s| s.left) } else { l.left + 0.9 * l.size });
            continue;
        }
        let Some(left) = item else { continue };
        if l.left < left - 0.5 * l.size {
            // (a line further left ends the item; not a row of a formula of it)
            if !f[i] {
                item = None;
            }
            continue;
        }
        let word_first = !doc.is_math(first) && first.script == Script::Normal && first.tex.is_none() && first.text.split_whitespace().next().is_some_and(|w| w.chars().count() >= 2 && w.chars().all(char::is_alphabetic));
        if f[i] && !info[i].2 && word_first && (l.left - left).abs() <= 0.6 * l.size && l.size >= 0.9 * st.body {
            f[i] = false;
        }
    }
    // The short last line of a paragraph may be all math ("in (M − I).}"):
    // it starts where the line of prose above it starts, at the paragraph's
    // line spacing, and that line runs to the right margin. A formula set
    // apart has more room above it.
    for i in 1..lines.len() {
        let (l, above) = (&lines[i], &lines[i - 1]);
        let prose_above = !f[i - 1] && info[i - 1].1 >= 1 && above.right >= st.right - st.body;
        // (the line above may be the paragraph's first, set in from the margin)
        // (… or an entry's first, with its number out at the margin and
        // its words beginning, after a gap, where this line begins)
        let under = (l.left - above.left).abs() < 1.5 || ((l.left - st.left).abs() < 1.5 && above.left > l.left && above.left - l.left <= 2.0 * st.body) || (l.left > above.left && l.left - above.left <= 4.0 * st.body && above.spans.windows(2).any(|w| (w[1].left - l.left).abs() < 1.5 && w[1].left - w[0].right() >= 0.6 * l.size));
        // (the top of a line with raised letters is a little higher than its neighbours')
        if f[i] && !info[i].2 && prose_above && under && l.top - above.top <= (1.6 * l.size).max(if l.size >= 0.9 * st.body { 1.25 * st.spacing } else { 0.0 }) && l.size >= 0.75 * above.size {
            f[i] = false;
        }
    }
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
            // (small type alone does not make a row of it: a word between
            // two formulas of a passage set small, "and", "becomes")
            let candidate = words <= 3
                && (ratio > 0.0 || (l.size < 0.9 * st.body && words == 0) || indented(l))
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
            // (the bracket may stand on the middle line of five rows, the
            // line with the "=", two lines away from the first and last row)
            let bracket_near = (i.saturating_sub(3)..(i + 4).min(lines.len())).any(|j| j != i && f[j] && beside_bracket(j));
            // (a row of plain numbers right of the bracket is its row, even
            // when the matrix opens the line near the margin)
            let after_bracket = |j: usize| {
                let cy = l.top + 0.5 * st.body;
                lines[j].spans.iter().any(|s| doc.family(s).starts_with("CMEX") && s.bottom() - s.top > 1.5 * st.body && s.top <= cy && s.bottom() >= cy && s.left <= l.left && l.left - s.left <= 2.0 * st.body)
            };
            let row = words == 0 && ratio > 0.0 && (i.saturating_sub(3)..(i + 4).min(lines.len())).any(|j| j != i && f[j] && after_bracket(j));
            if row {
                f[i] = true;
                changed = true;
                continue;
            }
            if candidate && ((i > 0 && near(i - 1)) || (i + 1 < lines.len() && near(i + 1)) || bracket_near) {
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

/// A statement in words set apart as a formula is, with a tall brace at
/// its right and an equation's number: lines of text beside a line that
/// is nothing but the brace and the number. From line `i`: where it ends,
/// and the statement as a formula of text lines closed by the brace.
fn braced_statement(doc: &Doc, lines: &[Line], disp: &[bool], i: usize, end: usize, st: &Stats) -> Option<(usize, String, Option<String>)> {
    let brace = |s: &crate::layout::Span| doc.family(s).starts_with("CMEX") && s.bottom() - s.top >= 1.5 * st.body && !s.text.trim().is_empty() && s.text.trim().chars().all(|c| c == '}' || c == 'o' || c == ')');
    let k = (i..end.min(i + 4)).find(|k| {
        let spans = &lines[*k].spans;
        disp[*k] && spans.iter().filter(|s| brace(s)).count() == 1 && spans.iter().all(|s| brace(s) || mathtex::tag_of(&s.text).is_some() || s.text.trim().is_empty())
    })?;
    let closing = lines[k].spans.iter().find(|s| brace(s))?;
    if closing.text.trim() != "}" {
        return None;
    }
    let row = |l: &Line| {
        let mid = l.top + l.size / 2.0;
        mid >= closing.top - 2.0 && mid <= closing.bottom() + 2.0 && l.right <= closing.left + 2.0 && math_info(doc, l, st).1 >= 1
    };
    let mut j = i;
    while j < end && (j == k || row(&lines[j])) {
        j += 1;
    }
    if j <= k || j - i < 2 {
        return None;
    }
    let rows: Vec<String> = (i..j)
        .filter(|n| *n != k)
        .map(|n| {
            tidy(line_inlines(doc, &lines[n], st))
                .into_iter()
                .filter_map(|x| match x {
                    Inline::Text(t) if t.trim().is_empty() => None,
                    Inline::Text(t) => Some(format!("\\text{{{}}}", latex_text(&t))),
                    Inline::Emph(t) => Some(format!("\\textit{{{}}}", latex_text(&t))),
                    Inline::Strong(t) => Some(format!("\\textbf{{{}}}", latex_text(&t))),
                    Inline::Math(t) => Some(t),
                    other => Some(format!("\\text{{{}}}", latex_text(&plain(&[other])))).filter(|t| t.len() > 7),
                })
                .collect::<Vec<_>>()
                .join("")
        })
        .collect();
    let tag = lines[k].spans.iter().find_map(|s| mathtex::tag_of(&s.text));
    Some((j, format!("\\left. \\begin{{array}}{{l}} {} \\end{{array}} \\right\\}}", rows.join(" \\\\ ")), tag))
}

/// A line set in the middle of the text column: its middle at the
/// column's middle, and clearly in from both margins (a paragraph's
/// indented first line or its short last line is not).
fn is_centered(l: &Line, st: &Stats) -> bool {
    let (left, right) = st.frame(l);
    let mid = (left + right) / 2.0;
    ((l.left + l.right) / 2.0 - mid).abs() <= 0.4 * st.body
        && l.left - left >= st.body
        && right - l.right >= st.body
}

/// How far a line is in from the nearer margin.
fn inset(l: &Line, st: &Stats) -> f32 {
    let (left, right) = st.frame(l);
    (l.left - left).min(right - l.right)
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
        // (not the numerator of a stacked fraction the line begins with,
        // "½μ²ϕ² term": a small figure stands right under it)
        let stacked = l.spans.iter().skip(1).any(|s| {
            s.script == Script::Sub && s.text.trim().bytes().all(|b| b.is_ascii_digit()) && !s.text.trim().is_empty() && s.left < first.right() + 1.0 && s.right() > first.left - 1.0 && s.top > first.top
        });
        return !stacked;
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
    // An article's section: its numbered title, or a title without a
    // number set as those are, in bold capitals ("ACKNOWLEDGMENTS", and
    // the second line of a long title).
    if st.article {
        let capitals = bold_words(doc, l) && text.chars().filter(|c| c.is_alphabetic()).count() >= 4 && !text.chars().any(|c| c.is_lowercase()) && text.split_whitespace().count() <= 10;
        if section_title(doc, l).is_some() || capitals {
            return Some(ARTICLE_SECTION);
        }
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
fn page_notes_start(doc: &Doc, lines: &[Line], st: &Stats, note_open: bool, rules: &[Rule]) -> usize {
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
            && first.chars().all(|c| c.is_ascii_digit() || "*∗†‡§¶".contains(c));
        let (_, words, _) = math_info(doc, l, st);
        // (a note that goes on from the page before: its lines stand, as
        // notes do, under the short rule drawn over the notes of a page)
        let above = k.checked_sub(1).map_or(f32::MIN, |a| lines[a].top);
        let ruled = note_open && rules.iter().any(|r| r.bottom - r.top <= 3.0 && r.right - r.left >= 2.0 * st.body && r.right - r.left <= 0.5 * (st.right - st.left) && r.left <= l.left + 0.5 * st.body && r.top < l.top && l.top - r.top <= 3.0 * st.body && r.top > above);
        if ruled && words >= 2 {
            return true;
        }
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
    // the bars of roots (they start at a root sign)
    let root_bars: Vec<bool> = bars
        .iter()
        .map(|b| lines.iter().any(|l| l.spans.iter().any(|s| s.text.trim() == "√" && (b.left - s.right()).abs() <= 3.0 && s.top <= b.bottom + 2.0 && s.bottom() >= b.top)))
        .collect();
    for b in bars {
        // a root's bar starts at its root sign: it goes with the sign's line
        let at_sign = |s: &crate::layout::Span| (b.left - s.right()).abs() <= 3.0 && s.top <= b.bottom + 2.0 && s.bottom() >= b.top;
        let root = lines.iter().position(|l| l.spans.iter().any(|s| s.text.trim() == "√" && at_sign(s)));
        if let Some(k) = root {
            lines[k].bars.push(*b);
            continue;
        }
        // (a larger root sign of the big-symbol font: its bar lies above
        // the line's letters, and belongs to the sign's line all the same)
        let big_root = lines.iter().position(|l| l.spans.iter().any(|s| doc.family(s).starts_with("CMEX") && matches!(s.text.trim(), "p" | "q" | "r" | "s") && at_sign(s)));
        let y = (b.top + b.bottom) / 2.0;
        // (the bar is of the line whose letters it stands level with: a
        // tall bracket may have given the line a top far above them)
        // (or the fraction opens that line, and the bar lies before its
        // first letter)
        let level = |l: &Line| l.left <= b.right + 2.0 * l.size && l.right >= b.left - 2.0 * l.size && l.spans.iter().any(|s| !doc.family(s).starts_with("CMEX") && s.script == Script::Normal && doc.size_of(s) >= 0.8 * l.size.min(doc.body_size) && s.height <= 1.5 * doc.size_of(s) && y >= s.top + 0.2 * s.height && y <= s.bottom() - 0.2 * s.height && (s.right() < b.left || s.left > b.right) && (b.left - s.right()).max(s.left - b.right) < 5.0 * l.size);
        let Some(home) = big_root.or_else(|| lines.iter().position(|l| y >= l.top && y <= l.top + l.size && l.left <= b.left + 1.0 && l.right >= b.right - 1.0)).or_else(|| lines.iter().position(level)) else {
            continue;
        };
        let size = lines[home].size;
        let mut moved = Vec::new();
        // (the pieces right at a little bar, over and under it: what stands
        // behind one of them, further from the bar, is not of its fraction —
        // the index of a letter in the line above, over the 1 of a ½)
        let narrow = b.right - b.left <= 1.2 * size;
        let halves: Vec<(f32, f32, f32)> = lines
            .iter()
            .flat_map(|l| l.spans.iter())
            .filter(|s| {
                let c = s.left + s.width / 2.0;
                narrow && c >= b.left - 2.0 && c <= b.right + 2.0 && (b.top - s.bottom()).max(s.top - b.bottom) <= 3.0
            })
            .map(|s| (s.top, s.bottom(), s.left))
            .collect();
        for (k, l) in lines.iter_mut().enumerate() {
            if k == home {
                continue;
            }
            // (a line of the text, with its words, is no numerator or
            // denominator set apart: the letters of a formula in it that
            // lie under a bar of the line above stay in it. A numerator
            // put with the words of the line above stands lower than they.)
            let words = l.spans.iter().any(|s| !doc.is_math(s) && s.text.split_whitespace().any(|w| w.chars().filter(|c| c.is_alphabetic()).count() >= 4));
            let line_top = l.top;
            let words_on_line = l.spans.iter().any(|s| !doc.is_math(s) && s.script == Script::Normal && s.text.split_whitespace().any(|w| w.chars().filter(|c| c.is_alphabetic()).count() >= 4));
            // (where the letters standing on the line end: an index
            // follows its letter)
            let letter_ends: Vec<f32> = l.spans.iter().filter(|s| s.script == Script::Normal && (s.top - line_top).abs() <= 0.3 * size).map(|s| s.right()).collect();
            l.spans.retain(|s| {
                // (standing on that line of text like its words: of that line)
                // unless it is right at the bar. (And what lies under a
                // root's bar is under the root.)
                let at_bar = (b.top - s.bottom()).max(s.top - b.bottom) <= 0.35 * size;
                if words && big_root.is_none() && (s.top - line_top).abs() <= 0.3 * size && !at_bar {
                    return true;
                }
                // (words much wider than the bar are no numerator of its
                // fraction: the end of the line of text over a little ½)
                if !doc.is_math(s) && s.width > 2.0 * (b.right - b.left) && s.text.split_whitespace().any(|w| w.chars().filter(|c| c.is_alphabetic()).count() >= 4) {
                    return true;
                }
                let c = s.left + s.width / 2.0;
                let inside = c >= b.left - 2.0 && c <= b.right + 2.0;
                // (under a root's bar is what the root is of; over it, the
                // line above)
                let near = (big_root.is_none() && b.top - s.bottom() <= 0.8 * size && s.bottom() <= b.top + 2.0)
                    || (s.top - b.bottom <= 0.8 * size && s.top >= b.bottom - 2.0);
                // (what stands between two bars, closer to the other, is of
                // that fraction: the 2 of a ½ in the line above, right over
                // this one's 1)
                let gap_to = |r: &Rule| (r.top - s.bottom()).max(s.top - r.bottom).max(0.0);
                let mid = s.top + s.height / 2.0;
                let others = bars.iter().zip(&root_bars).any(|(o, root)| !root && (o.top < mid) != (b.top < mid) && c >= o.left - 2.0 && c <= o.right + 2.0 && gap_to(o) < gap_to(b));
                // (nor is a full stop a numerator or a denominator: the
                // raised dot of a product in the next line's exponent)
                // (nor the index of a letter in the line of text above, the
                // 0 of ∂₀ over the ½ of the next line)
                // (it begins, or ends, well inside the height of its own
                // line; a numerator put with the line above begins under it)
                let script_of_text = words_on_line
                    && s.script == Script::Sub
                    && s.bottom() <= b.top + 2.0
                    && s.top < line_top + 0.75 * size
                    && letter_ends.iter().any(|r| (s.left - r).abs() <= 0.2 * size);
                // (away from the bar, on the side where a piece stands right at
                // it, and beginning before that piece: its scripts begin
                // after it, or with it)
                let away = (b.top - s.bottom()).max(s.top - b.bottom) > 0.4 * size;
                let side: Vec<&(f32, f32, f32)> = halves.iter().filter(|h| (s.bottom() <= b.top + 2.0) == (h.1 <= b.top + 2.0)).collect();
                let behind = away && !side.is_empty() && !side.iter().any(|h| s.left >= h.2 - 1.0);
                // (… or beyond that piece altogether, and in larger type: the
                // minus sign that opens the next row of a table, under the 2
                // of a ½)
                let beyond = side.iter().any(|h| (s.top >= h.1 - 1.5 || s.bottom() <= h.0 + 1.5) && s.height > 1.2 * (h.1 - h.0));
                if inside && near && !others && !script_of_text && !behind && !beyond && s.text.trim() != "." {
                    if std::env::var_os("PDF2MD_FRACDEBUG").is_some() {
                        eprintln!("FRAC bar {:.0}..{:.0}@{:.0} home top {:.0} takes {:?}@{},{} from line top {:.0} script {:?}", b.left, b.right, b.top, 0.0, s.text, s.left, s.top, line_top, s.script);
                    }
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
    /// its type size
    size: f32,
    /// the line names who said what stands above it ("—N. P. Ong,
    /// Princeton University"): it ends its paragraph
    signs: bool,
}

/// A line that names the author of the words above it: a dash and a name,
/// short, as under a quotation.
fn signs_quote(text: &str) -> bool {
    let Some(name) = text.trim_start().strip_prefix('—') else { return false };
    name.trim_start().starts_with(char::is_uppercase) && text.split_whitespace().count() <= 12
}

/// A paragraph that stops in the middle of its sentence.
struct Held {
    /// its last line
    last: Prev,
    /// where its block is in the builder's `out`
    at: usize,
    /// how many lines it has
    lines: usize,
    /// the bulleted item it stands in (where the item's text begins)
    item: Option<f32>,
}

/// A word cut by a hyphen where a paragraph stops in front of pictures
/// ("coordi-"), the paragraph going on behind them ("nate systems"): its
/// first half goes over to the part that goes on, where the word is whole
/// (with its hyphen if that is the word's own, see `Stats::keeps_hyphen`).
fn carry_cut_word(before: &mut [Inline], after: &mut [Inline], st: &Stats) {
    let (Some(Inline::Text(a)), Some(Inline::Text(b))) = (before.last_mut(), after.first_mut()) else { return };
    let word: String = a.trim_end_matches('-').chars().rev().take_while(|c| c.is_alphabetic()).collect::<Vec<_>>().into_iter().rev().collect();
    let rest: String = b.chars().take_while(|c| c.is_alphabetic()).collect();
    if !a.ends_with('-') || a.ends_with("--") || word.is_empty() || !b.starts_with(|c: char| c.is_lowercase()) {
        return;
    }
    let end = if st.keeps_hyphen(&word, &rest) { a.len() } else { a.len() - 1 };
    let cut = a.len() - word.len() - 1;
    b.insert_str(0, &a[cut..end]);
    a.truncate(cut);
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
    /// How far every line of the open paragraph is in from the left
    /// margin and from the right one, at least (a quotation: both).
    para_in: (f32, f32),
    /// the list of references of an article has begun
    refs: bool,
    /// where in `para` the line breaks stand that the book made on purpose
    /// (hard returns): kept whether or not the paragraph is centered
    hard: Vec<usize>,
    /// a bulleted item is open: where its text begins (the lines set in
    /// under the bullet stand there)
    item: Option<f32>,
    /// The last line of a paragraph that stops in the middle of its
    /// sentence, full to the right margin. If only pictures, captions and
    /// page marks follow it, the next line of text may go on with the
    /// sentence (see `goes_on_behind`).
    held: Option<Held>,
    /// the open paragraph goes on with such a sentence
    para_goes_on: bool,
}

impl Builder<'_> {
    fn push(&mut self, block: Block) {
        self.out.push(Blk { block, page: self.page, centered: false, goes_on: false });
    }

    fn flush(&mut self) {
        if !self.para.is_empty() {
            let p = std::mem::take(&mut self.para);
            let hard = std::mem::take(&mut self.hard);
            // (one line at least clearly in from the margins: an indented
            // one-line paragraph that ends early is centered by chance)
            let centered = self.para_centered && self.para_inset >= 2.5 * self.st.body;
            // A paragraph of several lines, each set in from both margins
            // and not centered: a quotation set apart, written as a block
            // quotation ("> "). Not in a bulleted item, whose lines are all
            // set in. (In an article: in books the test also takes the
            // set-in parts of problems.)
            let (left, right) = self.para_in;
            let quoted = self.st.article && !centered && self.item.is_none() && self.para_lines >= 2 && left >= 0.8 * self.st.body && right >= 0.8 * self.st.body && left <= 6.0 * self.st.body && !self.index_mode;
            // (its lines end short of the text's margin: none of them ends
            // early on purpose)
            let hard = if quoted { Vec::new() } else { hard };
            // not centered after all: its kept line breaks become spaces
            // (but not the hard returns)
            let p: Vec<Inline> = if centered {
                p
            } else {
                // (… and a word cut at the end of such a line is put together)
                let mut out: Vec<Inline> = Vec::with_capacity(p.len());
                let mut k = 0;
                while k < p.len() {
                    if p[k] == Inline::Break && !hard.contains(&k) {
                        let cut = match (out.last(), p.get(k + 1)) {
                            (Some(Inline::Text(a)), Some(Inline::Text(b))) => {
                                let word: String = a.trim_end_matches('-').chars().rev().take_while(|c| c.is_alphabetic()).collect::<Vec<_>>().into_iter().rev().collect();
                                let next: String = b.chars().take_while(|c| c.is_alphabetic()).collect();
                                a.ends_with('-') && !word.is_empty() && b.starts_with(|c: char| c.is_lowercase()) && !self.st.keeps_hyphen(&word, &next)
                            }
                            _ => false,
                        };
                        match (cut, out.last_mut()) {
                            (true, Some(Inline::Text(a))) => {
                                a.pop();
                            }
                            _ => out.push(Inline::Text(" ".into())),
                        }
                    } else {
                        out.push(p[k].clone());
                    }
                    k += 1;
                }
                out
            };
            // (a book with math fonts sets a formula's figures and brackets
            // in the text's font: the pieces of one formula, put together)
            // (the two sides of a sign once more: a bracket's mending may have
            // made a formula of the text behind an open "=")
            let p = if is_text_math(self.doc) { tidy(p) } else { tidy(crate::brackets::sides(tidy(crate::brackets::mend(crate::brackets::sides(tidy(p)))))) };
            // A paragraph that is one equation picture and nothing else
            // (but the comma or full stop after it) is a formula set apart
            // without a number: written as one.
            let sources: Vec<&String> = p.iter().filter_map(|i| if let Inline::Source(k) = i { Some(k) } else { None }).collect();
            let formulas: Vec<&String> = p.iter().filter_map(|i| if let Inline::Math(t) = i { Some(t) } else { None }).collect();
            let rest_is_points = p.iter().all(|i| match i {
                Inline::Source(_) | Inline::Math(_) => true,
                Inline::Text(t) => t.chars().all(|c| c.is_whitespace() || ".,;:".contains(c)),
                _ => false,
            });
            if let ([key], [tex], true) = (sources.as_slice(), formulas.as_slice(), rest_is_points) {
                let points: String = p.iter().filter_map(|i| if let Inline::Text(t) = i { Some(t.trim()) } else { None }).collect();
                let latex = if points.is_empty() || tex.trim_end().ends_with(['.', ',', ';']) { tex.to_string() } else { format!("{} {points}", tex.trim_end()) };
                let block = Block::Equation { latex, tag: None, source: Some(key.to_string()) };
                self.out.push(Blk { block, page: self.para_page, centered: false, goes_on: false });
            } else {
                // The list of references of an article that gives it no
                // title: it begins with entry "[1]", after the text.
                if self.st.article && !self.refs && plain(&p).trim_start().starts_with("[1] ") && 2 * self.para_page >= self.doc.pages.len() {
                    self.refs = true;
                    let titled = self.out.iter().rev().find_map(|o| if let Block::Heading { text, .. } = &o.block { Some(key(&plain(text)) == "references") } else { None }).unwrap_or(false);
                    if !titled {
                        self.out.push(Blk { block: Block::Heading { level: 0, size: ARTICLE_SECTION, id: String::new(), text: vec![Inline::Text("References".into())] }, page: self.para_page, centered: false, goes_on: false });
                    }
                }
                if quoted {
                    self.out.push(Blk { block: Block::Quote, page: self.para_page, centered: false, goes_on: false });
                }
                // The sentence may go on behind the pictures that follow:
                // its last line is kept to try the next line of text on.
                // (A caption between them leaves it as it is, and a line
                // set centered: a letter or a name under a picture.)
                let plain_para = !quoted && !centered && !self.index_mode;
                if crate::links::caption(&p).is_none() && !centered {
                    self.held = match self.prev.take() {
                        Some(last) if plain_para && !last.ends_sentence && last.right >= self.st.right - self.st.body => Some(Held { last, at: self.out.len(), lines: self.para_lines, item: self.item }),
                        _ => None,
                    };
                }
                let goes_on = plain_para && std::mem::take(&mut self.para_goes_on);
                self.out.push(Blk { block: Block::Para(p), page: self.para_page, centered, goes_on });
                if quoted {
                    self.out.push(Blk { block: Block::QuoteEnd, page: self.para_page, centered: false, goes_on: false });
                }
            }
        }
        self.prev = None;
        self.para_lines = 0;
        self.para_goes_on = false;
    }

    /// Does the paragraph that line `l` opens go on with a sentence that
    /// pictures broke off? A book sets its pictures at the top of a page
    /// or where there is room, in the middle of a sentence too: the
    /// sentence stops at the end of a full line and goes on under the
    /// pictures and their captions. It does if the line would be the
    /// next line of that paragraph, were it the first line of a new page.
    /// (A word cut there is put together, in the part that goes on.)
    fn goes_on_behind(&mut self, l: &Line, inl: &mut [Inline], next: Option<&Line>, pi: usize) -> bool {
        let Some(h) = &self.held else { return false };
        if h.last.page > pi || self.item.is_some() || self.index_mode || crate::links::starts_like_caption(&plain_text(l)) {
            return false;
        }
        let Some(Held { last, at, lines, item }) = self.held.take() else { return false };
        let page = last.page;
        // A bulleted item that the pictures ended (they stand at the
        // margin): it goes on if the line is set in as its text is, and
        // its end mark is taken out again.
        let end_mark = item.is_some() && matches!(self.out.get(at + 1), Some(Blk { block: Block::BulletEnd, .. }));
        if item.is_some_and(|left| !end_mark || l.left < left - 0.5 * l.size) {
            return false;
        }
        let mark = end_mark.then(|| self.out.remove(at + 1));
        // (nothing but pictures and such since, and one of them at least)
        let floats = crate::before_floats(&self.out, self.out.len()) == Some(at) && !self.out[at + 1..].iter().all(|o| matches!(o.block, Block::Anchor(_)));
        let starts = !floats || {
            // (as over a page: the room the pictures take is no paragraph's end)
            let open = (self.prev.take(), self.para_lines, self.item);
            (self.prev, self.para_lines, self.item) = (Some(Prev { page: usize::MAX, ..last }), lines, item);
            let starts = self.starts_para(l, next, pi);
            let last = self.prev.take().map(|p| Prev { page, ..p });
            (self.prev, self.para_lines, self.item) = open;
            // (a line that begins something of its own may be a letter
            // or a name under a picture: the sentence is still open)
            self.held = last.map(|last| Held { last, at, lines, item });
            starts
        };
        // A line that opens with a capital letter may begin a sentence, and
        // a paragraph: it goes on only in a book that sets a paragraph's
        // first line in (this one is at the margin).
        let capital = plain_text(l).trim_start().starts_with(char::is_uppercase) && (item.is_some() || !self.st.indents);
        if starts || capital {
            if let Some(mark) = mark {
                self.out.insert(at + 1, mark);
            }
            return false;
        }
        self.held = None;
        self.item = item;
        if let Some(Blk { block: Block::Para(before), .. }) = self.out.get_mut(at) {
            carry_cut_word(before, inl, self.st);
        }
        true
    }

    /// The open bulleted item ends here (its last paragraph is written).
    fn close_item(&mut self) {
        if self.item.take().is_some() {
            self.push(Block::BulletEnd);
        }
    }

    /// Put the pending page anchor in front of the next block.
    /// (Inside a bulleted item that goes on over the page, it stands in
    /// the item.)
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
        if starts_entry(l) || p.signs {
            return true;
        }
        // (the next part of a problem, "(b) Show that …", after a sentence)
        let part = {
            let t = plain_text(l);
            t.trim_start().strip_prefix('(').and_then(|r| r.split_once(')')).is_some_and(|(n, rest)| {
                rest.starts_with(' ') && ((n.chars().count() == 1 && n.chars().all(|c| c.is_ascii_lowercase())) || (!n.is_empty() && n.chars().count() <= 4 && n.chars().all(|c| "ivx".contains(c))))
            })
        };
        if part && p.ends_sentence {
            return true;
        }
        // (a paragraph goes on over a page in the type it is set in: the
        // dedication is not the end of the copyright page's last line)
        if p.page != pi && (l.size - p.size).abs() > 0.5 {
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
            // (or the first line is full and stops in the middle of its
            // sentence: the lines under "(a) Show that …" begin under "Show")
            let goes_on = p.right >= st.right - st.body && !p.ends_sentence;
            let hanging = self.para_lines == 1
                && p.left < l.left - step
                && (goes_on || next.is_none_or(|n| n.left >= l.left - step || starts_entry(n)));
            // An indented block (a quotation): same indent as before.
            let same_indent = (p.left - l.left).abs() < step;
            // (… whose paragraphs have their first line set in, like the
            // book's: the line above is one, full to the right margin; not a
            // name set to the right over its place)
            let first_line = self.para_lines == 1 && p.left > l.left && p.left - l.left <= 2.0 * st.body && p.right >= st.right - st.body && !p.ends_sentence && p.right - p.left >= 0.6 * (st.right - st.left);
            // (… or a quotation's: set in from both margins, its lines end
            // where this one ends, short of the right margin)
            let quoted_first = self.para_lines == 1 && p.left > l.left && p.left - l.left <= 2.0 * st.body && (p.right - l.right).abs() <= step && l.right < st.right - st.body && !p.ends_sentence;
            return !(hanging || same_indent || first_line || quoted_first);
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
        // (a word cut at the end of a line that holds a linked reference:
        // the link's mark stands behind the line's text, between the two
        // halves of the word. The hyphen goes; the mark stays where it is.)
        if let ([.., Inline::Text(a), Inline::PageLink(_)], Some(Inline::Text(b))) = (self.para.as_mut_slice(), inl.first()) {
            let a_word: String = a.trim_end_matches('-').chars().rev().take_while(|c| c.is_alphabetic()).collect::<Vec<_>>().into_iter().rev().collect();
            if a.ends_with('-') && !a_word.is_empty() && b.starts_with(|c: char| c.is_lowercase()) {
                let b_word: String = b.chars().take_while(|c| c.is_alphabetic()).collect();
                if !self.st.keeps_hyphen(&a_word, &b_word) {
                    a.pop();
                }
                self.para.extend(inl);
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
                if !self.st.keeps_hyphen(&a_word, &b_word) {
                    a.pop();
                }
                a.push_str(b);
                self.para.extend(inl.into_iter().skip(1));
                return;
            }
        }
        // an italic or bold phrase that goes on over the line's end: one
        // piece, and a word cut at the end of the line put together as in
        // plain text
        let styled = matches!((self.para.last(), inl.first()), (Some(Inline::Emph(_)), Some(Inline::Emph(_))) | (Some(Inline::Strong(_)), Some(Inline::Strong(_))));
        if styled {
            if let (Some(Inline::Emph(a) | Inline::Strong(a)), Some(Inline::Emph(b) | Inline::Strong(b))) = (self.para.last_mut(), inl.first()) {
                let a_word: String = a.trim_end_matches('-').chars().rev().take_while(|c| c.is_alphabetic()).collect::<Vec<_>>().into_iter().rev().collect();
                if a.ends_with('-') && !a_word.is_empty() && b.starts_with(|c: char| c.is_lowercase()) {
                    let b_word: String = b.chars().take_while(|c| c.is_alphabetic()).collect();
                    if !self.st.keeps_hyphen(&a_word, &b_word) {
                        a.pop();
                    }
                } else {
                    a.push(' ');
                }
                a.push_str(b);
                self.para.extend(inl.into_iter().skip(1));
                return;
            }
        }
        // a double name cut after its dash ("Stern–" / "Gerlach", "Clebsch-"
        // / "Gordan"): the dash stays and no space comes in
        if let (Some(Inline::Text(a)), Some(Inline::Text(b))) = (self.para.last_mut(), inl.first()) {
            let mut end = a.chars().rev();
            let dash = matches!(end.next(), Some('-' | '–')) && end.next().is_some_and(char::is_alphabetic);
            if dash && b.starts_with(char::is_uppercase) {
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

    fn toc_page(&mut self, lines: &[Line], page: &crate::layout::Page) {
        self.flush();
        self.anchor_block();
        let mut entries: Vec<TocEntry> = Vec::new();
        let mut pending: Vec<Inline> = Vec::new();
        let mut pending_left: Option<f32> = None;
        for l in lines {
            let text = plain_text(l);
            if is_contents_title(&text) {
                // (in its frame, if the book sets it in one)
                let inl = framed_title(self.doc, page, l, self.st).unwrap_or_else(|| crate::no_emph(line_inlines(self.doc, l, self.st)));
                self.push(Block::Heading { level: 2, size: l.size, id: String::new(), text: inl });
                continue;
            }
            let mut inl = crate::no_emph(line_inlines(self.doc, l, self.st));
            // a printed page number at the end, or else a link to the page
            let page = match toc_split(&text) {
                Some((_, page)) => {
                    // drop the page number and dot leaders from the last text piece
                    // (the last one to be seen: a mark of where the entry links to can follow it)
                    if let Some(Inline::Text(t)) = inl.iter_mut().rev().find(|i| matches!(i, Inline::Text(_))) {
                        let trimmed = t.trim_end();
                        let cut = trimmed.rsplit_once(char::is_whitespace).map_or("", |(a, _)| a);
                        *t = cut.trim_end_matches(|c: char| c == '.' || c.is_whitespace()).to_string();
                    }
                    page
                }
                None => match link_target(l) {
                    Some(target) => target,
                    // the name of a part, in larger type and without a
                    // page: an entry of its own (its number set once more
                    // in a frame before it, "II Part II: …", is not said twice)
                    None if l.size >= 1.15 * self.st.body && pending.is_empty() => {
                        let mut words = text.split_whitespace();
                        let framed = matches!((words.next(), words.next(), words.next()), (Some(n), Some("Part"), Some(again)) if n == again.trim_end_matches(':'));
                        let title = if framed { vec![Inline::Text(text.split_once(char::is_whitespace).map_or("", |(_, rest)| rest).trim().to_string())] } else { tidy(inl) };
                        // (the entry shows the number in its frame, as the
                        // book does; it is found by its name)
                        let label = match framed_title(self.doc, page, l, self.st).filter(|_| framed) {
                            Some(shown) => tidy(shown.into_iter().filter(|i| *i != Inline::Ruled).collect()),
                            None => Vec::new(),
                        };
                        entries.push(TocEntry { title, page: String::new(), left: l.left, size: l.size, columns: false, number: String::new(), label, part: false });
                        continue;
                    }
                    None => {
                        // a title that wraps onto the next line (the entry
                        // stands where its first line starts)
                        if pending.is_empty() {
                            pending_left = Some(l.left);
                        }
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
            let left = pending_left.take().unwrap_or(l.left);
            if !title.is_empty() {
                entries.push(TocEntry { title, page, left, size: l.size, columns: false, number: String::new(), label: Vec::new(), part: false });
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
        item: None,
        held: None,
        para_goes_on: false,
        para_inset: 0.0,
        para_in: (0.0, 0.0),
        refs: false,
    };
    // printed page marks (ebook PDFs without page numbers; see layout.rs)
    let use_marks = pages.iter().all(|p| p.label.is_none());
    // (the columns of a table that goes on over the page)
    let mut carried_table = Carried::default();
    // where the contents start (found by their entries, or by their title)
    let contents_page = st
        .toc_pages
        .iter()
        .min()
        .copied()
        .or_else(|| pages.iter().position(|p| p.lines.first().is_some_and(|l| is_contents_title(&plain_text(l)))));
    // the book's title as its title pages have it: a page that only says it
    // again later on (a second half-title) opens nothing
    let book_titles: HashSet<String> = pages[..contents_page.unwrap_or(0)]
        .iter()
        .flat_map(|p| p.lines.iter())
        .filter(|l| big_text_line(doc, l, st.body))
        .map(|l| key(&plain_text(l)))
        .collect();
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
            b.toc_page(&page.lines, &doc.pages[page.index]);
            continue;
        }
        let lines = &attach_fraction_rows(doc, &page.lines, &doc.pages[page.index].rules);
        let note_open = matches!(b.out.last(), Some(Blk { block: Block::PageNote { .. }, .. }));
        let mut end = page_notes_start(doc, lines, st, note_open, &doc.pages[page.index].rules);
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
        let mut panels = panel_tables(doc, &doc.pages[page.index], lines, st, &mut carried_table, &next_takes);
        let (grids, grid_tags) = grid_tables(doc, &doc.pages[page.index], lines, st);
        panels.extend(grids);
        panels.sort_by_key(|p| p.0);
        let on_table = |i: usize, j: usize| panels.iter().any(|p| i < p.1 && p.0 < j);
        if let Some(last) = panels.iter().map(|t| t.1).max().filter(|last| end < *last) {
            let on_panel = |l: &Line| doc.pages[page.index].shades.iter().any(|(p, _)| l.top >= p.top - 3.0 && l.top < p.bottom && l.left >= p.left - 3.0 && l.right <= p.right + 3.0);
            let below = last + lines[last..].iter().take_while(|l| on_panel(l)).count();
            end = below + page_notes_start(doc, &lines[below..], st, false, &[]);
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
                // (a picture and its caption stand at the margin)
                b.close_item();
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
                        // (an equation in a box: the equation, with its number)
                        (_, equation @ Block::Equation { .. }) => b.push(equation),
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
                            b.out.insert(at, Blk { block: picture, page, centered: false, goes_on: false });
                        }
                        (_, table) => {
                            b.push(table);
                            // (the printed table under it, to check it against)
                            b.push(picture);
                            // (a framed equation's number, with its anchor)
                            if let Some(tag) = grid_tags.get(&i) {
                                b.push(Block::Equation { latex: String::new(), tag: Some(tag.clone()), source: None });
                            }
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
                b.push(Block::Equation { latex: table_latex(doc, &doc.pages[page.index].rules, &lines[i..*j], cols), tag: None, source: None });
                prev_top = Some(lines[j - 1].top);
                last_heading = None;
                i = *j;
                continue;
            }
            if let Some((j, latex, tag)) = braced_statement(doc, lines, &disp, i, end, st).filter(|_| !b.index_mode) {
                b.flush();
                b.anchor_block();
                b.push(Block::Equation { latex, tag, source: None });
                prev_top = Some(lines[j - 1].top);
                last_heading = None;
                i = j;
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
                if std::env::var_os("PDF2MD_DISPDEBUG").is_some() {
                    for e in &eqs {
                        eprintln!("EQ {}", e.0);
                    }
                }
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
            // A row of pictures side by side with a line of names under
            // them, one name under each (portraits): each picture and its
            // name are one cell of a row, as on the page. Left as they
            // come, the pictures stand one under the other and the names
            // run together in one line.
            {
                let first = b.out.iter().rposition(|o| !(o.page == pi && matches!(o.block, Block::Figure { .. }))).map_or(0, |k| k + 1);
                // (from left to right, in whatever order the page drew them)
                let mut areas: Vec<Figure> = b.out[first..].iter().filter_map(|o| if let Block::Figure { area, .. } = &o.block { Some(*area) } else { None }).collect();
                areas.sort_by(|x, y| x.left.total_cmp(&y.left));
                let beside = areas.len() >= 2 && areas.windows(2).all(|w| w[1].left >= w[0].right - 2.0 && w[1].top < w[0].bottom && w[1].bottom > w[0].top);
                let foot = areas.iter().map(|a| a.bottom).fold(f32::MIN, f32::max);
                let under = |s: &crate::layout::Span| {
                    let c = s.left + s.width / 2.0;
                    areas.iter().position(|a| c >= a.left - 0.2 * (a.right - a.left) && c <= a.right + 0.2 * (a.right - a.left))
                };
                let named = beside
                    && b.para.is_empty()
                    && l.top >= foot - 4.0
                    && l.top - foot <= 2.5 * st.body
                    && l.spans.iter().all(|s| under(s).is_some())
                    && (0..areas.len()).all(|k| l.spans.iter().any(|s| under(s) == Some(k)));
                if named {
                    let mut figures = b.out.split_off(first);
                    figures.sort_by(|x, y| match (&x.block, &y.block) {
                        (Block::Figure { area: p, .. }, Block::Figure { area: q, .. }) => p.left.total_cmp(&q.left),
                        _ => std::cmp::Ordering::Equal,
                    });
                    b.push(Block::Columns(areas.len()));
                    for (k, figure) in figures.into_iter().enumerate() {
                        if k > 0 {
                            b.push(Block::ColumnBreak);
                        }
                        // (the picture is cut out with a little room around
                        // it: not down into its name)
                        let mut figure = figure;
                        if let Block::Figure { area, .. } = &mut figure.block {
                            area.bottom = area.bottom.min(l.top - 12.0).max(area.top + 1.0);
                        }
                        b.out.push(figure);
                        let spans: Vec<crate::layout::Span> = l.spans.iter().filter(|s| under(s) == Some(k)).cloned().collect();
                        let name = crate::layout::make_line(spans, &|s| doc.size_of(s));
                        b.push(Block::Para(line_inlines(doc, &name, st)));
                        if let Some(last) = b.out.last_mut() {
                            last.centered = true;
                        }
                    }
                    b.push(Block::ColumnsEnd);
                    prev_top = Some(l.top);
                    last_heading = None;
                    b.prev = None;
                    i += 1;
                    continue;
                }
            }
            // On the book's first page, a line in large type that stands
            // well apart under the title and its subtitle is the author's
            // name or the date: a line of the title page, no heading of the
            // book (it would be a section of its contents).
            let byline = page.index == 0
                && prev_top.is_some_and(|p| l.top - p > 2.0 * l.size)
                && page.lines.iter().any(|o| o.size > l.size + 0.5)
                && is_centered(l, st)
                // (not the title of an article's first section, on its first page)
                && heading_size(doc, l, prev_top, next_top, before_display, st).is_some_and(|s| s != ARTICLE_SECTION);
            if byline {
                b.flush();
                b.close_item();
                b.anchor_block();
                b.push(Block::Para(vec![Inline::Strong(plain(&line_inlines(doc, l, st)))]));
                if let Some(last) = b.out.last_mut() {
                    last.centered = true;
                }
                prev_top = Some(l.top);
                last_heading = None;
                i += 1;
                continue;
            }
            if let Some(size) = heading_size(doc, l, prev_top, next_top, before_display, st) {
                // A page that opens well down from its top with a title
                // clearly larger than the sections' opens a chapter too,
                // whatever its size ("Preface", "Part I", "Appendix A" in a
                // book whose numbered chapters have a type size of their own).
                // Not before the contents: those are the book's title pages.
                let titles_over = contents_page.is_none_or(|first| pi >= first) && !(page.lines.len() == 1 && book_titles.contains(&key(&plain_text(l))));
                let size = match (st.chapter_size, st.section_size) {
                    (Some(c), Some(sec)) if i == 0 && titles_over && c > 1.25 * sec && size >= 1.25 * sec && l.top >= 0.15 * page.height => c,
                    _ => size,
                };
                b.flush();
                b.close_item();
                let text = framed_title(doc, &doc.pages[page.index], l, st).unwrap_or_else(|| crate::no_emph(line_inlines(doc, l, st)));
                // A title broken over two lines is one heading.
                let mut merged = false;
                if let (Some(top), Some(Blk { block: Block::Heading { size: s0, text: t0, .. }, .. })) =
                    (last_heading, b.out.last_mut())
                {
                    if (*s0 - size).abs() < 0.5 && l.top - top <= 1.6 * size {
                        push_text(t0, " ");
                        t0.extend(text.clone());
                        merged = true;
                    } else if (*s0 - size).abs() < 0.5 && l.top - top <= 3.0 * size && is_chapter_label(&plain(t0)) {
                        // "Chapter 3" and, under it, the chapter's name:
                        // one title in two lines, as it is set
                        t0.push(Inline::Break);
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
            // (the next column of a page that was unfolded goes on as a new
            // page does: no space above its first line is a paragraph's end)
            if let Some(prev) = b.prev.as_mut() {
                if prev.page == pi && doc.pages[page.index].turns.iter().any(|t| prev.top < *t && l.top >= *t) {
                    prev.page = usize::MAX;
                }
            }
            let mut inl = line_inlines(doc, l, st);
            // A bulleted item: the line that opens with "•" begins it; the
            // lines under it are set in to where its words begin, and so
            // are the equations and paragraphs that belong to it. A line
            // of text further left ends it.
            let bullet = !b.index_mode && l.spans.first().is_some_and(|s| s.text.trim_start().starts_with('•')) && matches!(inl.first(), Some(Inline::Text(t)) if t.trim_start().starts_with('•'));
            if bullet {
                b.flush();
                b.close_item();
                b.anchor_block();
                b.push(Block::Bullet);
                b.item = Some(l.left + 0.9 * l.size);
                if let Some(Inline::Text(t)) = inl.first_mut() {
                    *t = t.trim_start().trim_start_matches('•').trim_start().to_string();
                    if t.is_empty() {
                        inl.remove(0);
                    }
                }
            } else if b.item.is_some_and(|left| l.left < left - 0.5 * l.size) {
                b.flush();
                b.close_item();
            }
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
            // The name under a quotation is the last line of the
            // quotation's paragraph, on a line of its own; what follows
            // it is another paragraph, also where the page ends with the
            // name.
            let signs = signs_quote(&plain_text(l))
                && l.right < st.right - 3.0 * st.body
                && !b.para.is_empty()
                && b.prev.as_ref().is_some_and(|p| !p.signs && (p.page != pi || l.top - p.top <= 1.6 * st.spacing));
            let starts = !signs && b.starts_para(l, next, pi);
            let room = if starts { None } else { b.room_above(l, measure, pi) };
            let hard = signs || room.is_some_and(|r| r >= HARD_RETURN_ROOM);
            // more space above than between the lines of a paragraph in
            // this type size: after a hard return, or between two lines of
            // plain words (a formula makes its line taller, not spaced)
            let pitch = st.spacing * l.size / st.body;
            let spaced = !starts
                && !signs
                && b.prev.as_ref().is_some_and(|p| {
                    p.page == pi && l.top - p.top > 1.4 * pitch && (hard || (p.plain_size == Some(l.size) && words_only(doc, l)))
                });
            // In a bulleted item, a line that begins where the item's words
            // begin, right under a line that ended early, at the spacing of
            // the lines of a paragraph: a new line of the item's text, not
            // a new paragraph (no space stands between them in the book).
            let item_line = !bullet
                && starts
                && !b.para.is_empty()
                && b.item.is_some_and(|left| (l.left - left).abs() <= 0.6 * l.size)
                // (a paragraph of its own inside an item has a little more
                // room above it than a line has)
                && b.prev.as_ref().is_some_and(|p| p.page == pi && l.top - p.top <= 1.15 * st.spacing * l.size.max(p.size) / st.body && l.top > p.top);
            // (… under a line that runs to the right margin it is that
            // line's sentence going on, "(i.e." and the rest: no new line)
            let goes_on = item_line && b.prev.as_ref().is_some_and(|p| p.right >= st.right - 2.0 * st.body);
            if (starts || spaced) && !item_line {
                b.flush();
                b.anchor_block();
                b.para_goes_on = !bullet && b.goes_on_behind(l, &mut inl, next, pi);
                b.para_page = pi;
                b.para = inl;
                b.para_centered = is_centered(l, st);
                b.para_inset = inset(l, st);
                b.para_in = (l.left - st.left, st.right - l.right);
            } else {
                if let Some(a) = b.anchor.take() {
                    b.para.push(Inline::Anchor(a));
                }
                // centered lines keep their line breaks
                let centered = b.para_centered && is_centered(l, st);
                let hard = hard || (item_line && !goes_on);
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
                b.para_in = (b.para_in.0.min(l.left - st.left), b.para_in.1.min(st.right - l.right));
            }
            b.para_lines += 1;
            // (the bullet's box begins higher than the words of its line:
            // the line's top is that of its words)
            let top = if bullet { l.spans.iter().filter(|s| !s.text.trim_start().starts_with('•') && !s.text.trim().is_empty()).map(|s| s.top).fold(f32::MAX, f32::min) } else { l.top };
            b.prev = Some(Prev {
                left: l.left,
                right: l.right,
                top: if top == f32::MAX { l.top } else { top },
                page: pi,
                ends_sentence: ends_sentence(&plain_text(l)),
                plain_size: words_only(doc, l).then_some(l.size),
                last_url: l.spans.last().and_then(|s| s.url.clone()),
                size: l.size,
                signs: signs_quote(&plain_text(l)) && l.right < st.right - 3.0 * st.body,
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
        let is_marker = |t: &str| !t.is_empty() && t.chars().all(|c| c.is_ascii_digit() || "*∗†‡§¶".contains(c));
        let mut lone_label: Option<String> = None;
        // (the note of the page before that goes on at the foot of this page)
        let open_note = if note_open { b.out.iter().rposition(|o| matches!(o.block, Block::PageNote { .. })) } else { None };
        let mut began = false;
        for l in &lines[end..] {
            let inl = line_inlines(doc, l, st);
            let first = l.spans.first().map(|s| s.text.trim().to_string()).unwrap_or_default();
            // a note number alone on its line labels the text below it
            if l.spans.len() == 1 && is_marker(&first) {
                lone_label = Some(first);
                continue;
            }
            if let Some(label) = lone_label.take() {
                b.out.push(Blk { block: Block::PageNote { label, text: tidy(inl) }, page: pi, centered: false, goes_on: false });
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
                b.out.push(Blk { block: Block::PageNote { label: first, text: tidy(text) }, page: pi, centered: false, goes_on: false });
                began = true;
            } else if let Some(at) = if began { Some(b.out.len() - 1) } else { open_note } {
                // (as the lines of a paragraph are joined: a word cut at
                // the line's end is put together)
                if let Block::PageNote { text, .. } = &mut b.out[at].block {
                    let so_far = std::mem::take(text);
                    // (the open paragraph of the page's text is put aside meanwhile)
                    let open = std::mem::replace(&mut b.para, so_far);
                    b.join(inl);
                    let joined = std::mem::replace(&mut b.para, open);
                    if let Block::PageNote { text, .. } = &mut b.out[at].block {
                        *text = joined;
                    }
                }
            }
        }
    }
    b.flush();
    b.close_item();
    // (the text of a note is mended as a paragraph's is: the pieces of
    // one formula, set in two fonts, put together)
    if !is_text_math(doc) {
        for o in b.out.iter_mut() {
            if let Block::PageNote { text, .. } = &mut o.block {
                *text = tidy(crate::brackets::sides(tidy(crate::brackets::mend(crate::brackets::sides(tidy(std::mem::take(text)))))));
            }
        }
    }
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
    fn a_word_cut_in_front_of_pictures_is_whole_behind_them() {
        let words = |w: &[&str]| w.iter().map(|w| w.to_string()).collect::<HashSet<String>>();
        let st = Stats { left: 0.0, right: 500.0, body: 25.0, spacing: 26.0, chapter_size: None, section_size: None, toc_pages: Default::default(), hyphenated: words(&["two-dimensional"]), words: words(&["coordinate"]), article: false, indents: true, wide: None };
        let text = |t: &str| vec![Inline::Text(t.into())];
        let (mut before, mut after) = (text("This is the coordi-"), text("nate option."));
        carry_cut_word(&mut before, &mut after, &st);
        assert_eq!((before, after), (text("This is the "), text("coordinate option.")));
        // the word's own hyphen stays
        let (mut before, mut after) = (text("in a two-"), text("dimensional grid"));
        carry_cut_word(&mut before, &mut after, &st);
        assert_eq!((before, after), (text("in a "), text("two-dimensional grid")));
        // no word is cut: a dash, a capital letter behind
        for (a, b) in [("as he said --", "then"), ("the contri-", "Together with")] {
            let (mut before, mut after) = (text(a), text(b));
            carry_cut_word(&mut before, &mut after, &st);
            assert_eq!((before, after), (text(a), text(b)));
        }
    }

    #[test]
    fn a_word_cut_at_a_line_end_keeps_the_hyphen_that_is_its_own() {
        let words = |list: &[&str]| list.iter().map(|w| w.to_string()).collect();
        let st = Stats { left: 0.0, right: 500.0, body: 25.0, spacing: 26.0, chapter_size: None, section_size: None, toc_pages: Default::default(), hyphenated: words(&["two-dimensional"]), words: words(&["four", "dimensional", "space", "time", "spacetime", "the", "oretically"]), article: false, indents: false, wide: None };
        // written so within a line
        assert!(st.keeps_hyphen("two", "dimensional"));
        // both halves are words, and the book never writes them as one
        assert!(st.keeps_hyphen("four", "dimensional"));
        // the book writes them as one
        assert!(!st.keeps_hyphen("space", "time"));
        // a half that is no word
        assert!(!st.keeps_hyphen("in", "tegrate"));
    }

    #[test]
    fn the_rows_of_a_table_are_found_by_how_its_cells_stand() {
        let st = Stats { left: 0.0, right: 500.0, body: 25.0, spacing: 26.0, chapter_size: None, section_size: None, toc_pages: Default::default(), hyphenated: Default::default(), words: Default::default(), article: false, indents: false, wide: None };
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
