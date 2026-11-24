//! What the reader adds to a converted book, kept when the book is
//! converted again.
//!
//! * **Notes**: `<details class="mine"> … </details>` blocks and alert
//!   boxes (`> [!NOTE]`, `> [!WARNING]` …); pdf2md never writes either.
//!   Each is put back right after the block it followed (found by that
//!   block's first words; under an equation read from a picture, by the
//!   picture), else after the nearest anchor above it, else at the end of
//!   its chapter.
//! * **Prose fixes** (the conversion got the book's words wrong):
//!   `<span class="fix" data-was="as converted">as in the book</span>`;
//!   redone in the same paragraph.
//!
//! * **Fixes of a displayed equation built from the PDF's text**: the
//!   corrected block (```` ```math ````, or between `$$` in a book written
//!   in the standard form) under a mark that holds the equation
//!   as converted, `<!-- pdf2md:fix as converted -->`. Every block of the
//!   chapter that is converted to that LaTeX again is replaced. A block
//!   that was no equation of the book (a piece torn off a formula) is
//!   taken out: `<!-- pdf2md:gone as converted -->` stands for it. An alert
//!   that begins "**Edited by Claude.**" right under the block is part of
//!   the fix and goes wherever the fix goes.
//!
//! (Corrections of equations read from pictures are kept per picture by
//! [`crate::ocr`].)
//!
//! mdBun's mark of an equation changed to improve on the original, not to
//! match it (`<!-- mdBun:improved -->` on the line under the block), is
//! part of a corrected block here, and a note under such an equation
//! stands under the mark. (Under an equation read from a picture the mark
//! is kept per picture, like the correction.)

use crate::ocr::IMPROVED;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    /// chapter file it was in (without .md), if known
    pub slug: Option<String>,
    /// first words of the block before it
    pub after: String,
    /// nearest anchor id above it
    pub anchor: Option<String>,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fix {
    pub slug: Option<String>,
    /// first words of its paragraph as converted (with the fix undone)
    pub paragraph: String,
    /// the converted text it replaces, and the whole span
    pub was: String,
    pub span: String,
}

/// A marker the reader put inside a sentence (`<sup>[✎](#mine-3)</sup>`,
/// or any `<sup class="mine">…</sup>`), pointing to a note.
#[derive(Debug, Clone, PartialEq)]
pub struct Mark {
    pub slug: Option<String>,
    /// first words of its paragraph
    pub paragraph: String,
    /// the text right before it in the paragraph
    pub before: String,
    pub mark: String,
}

/// A displayed equation corrected by hand (one built from the PDF's
/// text: it has no picture to be kept by).
#[derive(Debug, Clone, PartialEq)]
pub struct EqFix {
    pub slug: Option<String>,
    /// the equation's LaTeX as converted (what stands between the fences)
    pub was: String,
    /// the mark, the corrected block, and the alert that belongs to it
    pub block: String,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UserData {
    pub notes: Vec<Note>,
    pub fixes: Vec<Fix>,
    pub marks: Vec<Mark>,
    pub eqs: Vec<EqFix>,
}

impl UserData {
    pub fn is_empty(&self) -> bool {
        self.notes.is_empty() && self.fixes.is_empty() && self.marks.is_empty() && self.eqs.is_empty()
    }
}

const EQ_FIX: &str = "<!-- pdf2md:fix ";
/// How a displayed equation's block opens and closes, in the two forms of
/// math (see [`crate::emit::MathForm`]).
const BLOCKS: [(&str, &str); 2] = [("```math\n", "\n```"), ("$$\n", "\n$$")];
const EQ_GONE: &str = "<!-- pdf2md:gone ";
const BY_CLAUDE: &str = "\n\n> [!NOTE]\n> **Edited by Claude.**";

/// The mark of a corrected equation for its LaTeX as converted (a comment
/// cannot hold two hyphens in a row).
pub fn eq_fix_mark(was: &str) -> String {
    format!("{EQ_FIX}{} -->", was.replace("--", "-&#45;"))
}

/// The mark that stands for a block taken out.
pub fn eq_gone_mark(was: &str) -> String {
    format!("{EQ_GONE}{} -->", was.replace("--", "-&#45;"))
}

/// The corrected equations of a text: (start, end, as converted).
fn eq_fix_ranges(text: &str) -> Vec<(usize, usize, String)> {
    let mut out = Vec::new();
    let mut from = 0;
    loop {
        // (the next mark of either kind; "gone": a block that was no
        // equation of the book, taken out — its mark stands alone)
        let next = [EQ_FIX, EQ_GONE].into_iter().filter_map(|m| text[from..].find(m).map(|i| (from + i, m))).min_by_key(|x| x.0);
        let Some((i, kind)) = next else { break };
        from = i + kind.len();
        if i > 0 && !text[..i].ends_with('\n') {
            continue;
        }
        let Some(close) = text[from..].find(" -->").map(|c| from + c) else { continue };
        let was = unescape(&text[from..close]);
        let mut end = close + 4;
        if kind == EQ_FIX {
            // (the corrected block, in either form of math)
            let Some((open, close)) = BLOCKS.into_iter().find(|(open, _)| text[end..].strip_prefix("\n\n").is_some_and(|t| t.starts_with(open))) else { continue };
            let body = end + 2 + open.len();
            let Some(e) = text[body..].find(close).map(|e| body + e + close.len()) else { continue };
            end = e;
            // (mdBun's mark of an improvement, on the line under the
            // block: part of the fix)
            if text[end..].strip_prefix('\n').is_some_and(|l| l.starts_with(IMPROVED)) {
                end += 1 + IMPROVED.len();
            }
        }
        if text[end..].starts_with(BY_CLAUDE) {
            end += 2;
            for line in text[end..].split_inclusive('\n') {
                if !line.starts_with('>') {
                    break;
                }
                end += line.len();
            }
            end = text[..end].trim_end().len();
        }
        out.push((i, end, was));
        from = end;
    }
    out
}

/// Byte ranges of the reader's markers in a text.
fn mark_ranges(text: &str) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    for open in ["<sup>[✎](", "<sup class=\"mine\">"] {
        let mut from = 0;
        while let Some(i) = text[from..].find(open).map(|i| from + i) {
            let Some(end) = text[i..].find("</sup>").map(|e| i + e + 6) else { break };
            out.push((i, end));
            from = end;
        }
    }
    out.sort();
    out
}

fn without_marks(t: &str) -> String {
    let mut out = String::new();
    let mut last = 0;
    for (a, b) in mark_ranges(t) {
        out.push_str(&t[last..a]);
        last = b;
    }
    out.push_str(&t[last..]);
    out
}

/// The first words of a block of Markdown, without markup: what finds the
/// block again in a new conversion.
pub fn fingerprint(block: &str) -> String {
    let mut t = String::new();
    let mut rest = block;
    // drop tags, comments, link targets
    while let Some(i) = rest.find(['<', ']']) {
        t.push_str(&rest[..i]);
        let c = rest.as_bytes()[i];
        rest = &rest[i + 1..];
        if c == b'<' {
            rest = rest.find('>').map_or("", |j| &rest[j + 1..]);
        } else if rest.starts_with('(') {
            rest = rest.find(')').map_or("", |j| &rest[j + 1..]);
        }
    }
    t.push_str(rest);
    let words: Vec<String> = t
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .take(16)
        .map(str::to_lowercase)
        .collect();
    words.join(" ")
}

fn unescape(t: &str) -> String {
    // (a character by its number, "&#36;" for "$": so a formula in the
    // converted text can be kept in an attribute without a reader of
    // Markdown taking it for one)
    let mut out = String::with_capacity(t.len());
    let mut rest = t;
    while let Some(i) = rest.find("&#") {
        out.push_str(&rest[..i]);
        let after = &rest[i + 2..];
        let digits: String = after.chars().take_while(char::is_ascii_digit).collect();
        match (digits.parse::<u32>().ok().and_then(char::from_u32), after[digits.len()..].starts_with(';')) {
            (Some(c), true) if !digits.is_empty() => {
                out.push(c);
                rest = &after[digits.len() + 1..];
            }
            _ => {
                out.push_str("&#");
                rest = after;
            }
        }
    }
    out.push_str(rest);
    out.replace("&quot;", "\"").replace("&lt;", "<").replace("&gt;", ">").replace("&amp;", "&")
}

/// Byte ranges of the reader's notes in a file: <details class="mine">
/// blocks (nested <details> counted) and alert boxes.
fn note_ranges(text: &str) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let mut from = 0;
    while let Some(i) = text[from..].find("<details class=\"mine\"").map(|i| from + i) {
        let mut depth = 0;
        let mut k = i;
        let mut end = text.len();
        while k < text.len() {
            let open = text[k..].find("<details").map(|x| k + x);
            let close = text[k..].find("</details>").map(|x| k + x);
            match (open, close) {
                (Some(o), Some(c)) if o < c => {
                    depth += 1;
                    k = o + 8;
                }
                (_, Some(c)) => {
                    depth -= 1;
                    k = c + 10;
                    if depth == 0 {
                        end = k;
                        break;
                    }
                }
                _ => break,
            }
        }
        out.push((i, end));
        from = end;
    }
    // alert boxes: a run of "> " lines starting with "> [!"
    let mut pos = 0;
    let mut lines = text.split_inclusive('\n').peekable();
    while let Some(line) = lines.next() {
        let start = pos;
        pos += line.len();
        if line.starts_with("> [!") && !out.iter().any(|&(a, b)| start >= a && start < b) {
            while let Some(next) = lines.peek() {
                if next.starts_with('>') {
                    pos += next.len();
                    lines.next();
                } else {
                    break;
                }
            }
            out.push((start, pos));
        }
    }
    out.sort();
    out
}

/// The reader's notes and prose fixes in one Markdown file. `slug_at`
/// names the chapter at a byte position (book.md holds them all).
/// What `Note::after` begins with for a note under a displayed equation
/// read from a picture (no block's first words read like this), before
/// the picture's key.
const EQUATION: &str = "pdf2md:eq ";

/// The picture of the displayed equation that ends this text: its
/// ```` ```math ```` block under its mark (`<!-- pdf2md:eq KEY -->`), or
/// the mark alone (a note that came to stand between the two).
fn equation_above(before: &str) -> Option<&str> {
    let before = before.trim_end();
    // (mdBun's mark of an improvement stands under the block)
    let before = before.strip_suffix(IMPROVED).map_or(before, str::trim_end);
    fn mark(t: &str) -> Option<&str> {
        t.trim_end().rsplit('\n').next()?.strip_prefix("<!-- pdf2md:eq ")?.strip_suffix(" -->")
    }
    if let Some(k) = mark(before) {
        return Some(k);
    }
    let start = before.rfind("\n\n")?;
    (before[start + 2..].starts_with("```math") && before.ends_with("```")).then(|| mark(&before[..start]))?
}

/// What `Note::after` begins with for a note under program code read
/// from a picture, before the picture's key.
const CODE: &str = "pdf2md:code ";

/// The picture of the program code that ends this text: its fenced block
/// under its mark (`<!-- pdf2md:code KEY -->`).
fn code_above(before: &str) -> Option<&str> {
    use crate::ocr::CODE_MARK;
    let before = before.trim_end();
    let at = before.rfind(CODE_MARK)?;
    let (k, rest) = before[at + CODE_MARK.len()..].split_once(" -->\n")?;
    let inner = rest.trim_start_matches('\n').strip_prefix("```\n")?.strip_suffix("```")?;
    // (the block that ends here is that mark's own: no fence between)
    (inner.is_empty() || (inner.ends_with('\n') && !inner.contains("\n```") && !inner.starts_with("```"))).then_some(k)
}

fn collect_file(text: &str, slug_at: &dyn Fn(usize) -> Option<String>) -> UserData {
    // What stands in a bulleted item is set in two spaces under its
    // bullet: read as if it stood at the margin (a note there, a
    // corrected equation), so it is found, and kept without the indent —
    // it is set in again with its item when the book is written.
    let mut margin = String::with_capacity(text.len());
    // (where each such line begins in the text read, and how much was
    // taken out before it: to name the chapter a place is in)
    let mut taken: Vec<(usize, usize)> = Vec::new();
    for l in text.split_inclusive('\n') {
        match l.strip_prefix("  ").filter(|r| !r.starts_with(' ')) {
            Some(r) => {
                taken.push((margin.len(), 2 * (taken.len() + 1)));
                margin.push_str(r);
            }
            None => margin.push_str(l),
        }
    }
    let slug_of = slug_at;
    let slug_at = &|p: usize| slug_of(p + taken.iter().rev().find(|(at, _)| *at <= p).map_or(0, |(_, n)| *n));
    let text = margin.as_str();
    let mut data = UserData::default();
    let eqs = eq_fix_ranges(text);
    for (a, b, was) in &eqs {
        data.eqs.push(EqFix { slug: slug_at(*a), was: was.clone(), block: text[*a..*b].to_string() });
    }
    // (the alert of a corrected equation is part of its fix, no note)
    let notes: Vec<(usize, usize)> = note_ranges(text).into_iter().filter(|&(a, _)| !eqs.iter().any(|(x, y, _)| a >= *x && a < *y)).collect();
    let inside = |p: usize| notes.iter().any(|&(a, b)| p >= a && p < b);
    for &(a, b) in &notes {
        // the block before it (skipping other notes)
        let mut end = a;
        let mut after = String::new();
        while end > 0 {
            let before = text[..end].trim_end();
            let start = before.rfind("\n\n").map_or(0, |i| i + 2);
            if inside(start) {
                end = start;
                continue;
            }
            // (under a displayed equation read from a picture: kept by the
            // picture, for the equation's LaTeX may be corrected since)
            if let Some(k) = equation_above(before) {
                after = format!("{EQUATION}{k}");
                break;
            }
            // (under program code read from a picture: kept by the picture too)
            if let Some(k) = code_above(before) {
                after = format!("{CODE}{k}");
                break;
            }
            // (as converted: fixes undone, so it is found with or without them)
            after = fingerprint(&undo_fixes(&before[start..]));
            if !after.is_empty() {
                break;
            }
            end = start;
        }
        let anchor = text[..a]
            .match_indices("id=\"")
            .filter(|(i, _)| !inside(*i))
            .last()
            .and_then(|(i, _)| text[i + 4..].split('"').next().map(str::to_string));
        data.notes.push(Note { slug: slug_at(a), after, anchor, text: text[a..b].trim_end().to_string() });
    }
    // prose fixes
    let mut from = 0;
    while let Some(i) = text[from..].find("<span class=\"fix\" data-was=\"").map(|i| from + i) {
        let Some(close) = text[i..].find("</span>").map(|c| i + c + 7) else { break };
        from = close;
        if inside(i) {
            continue;
        }
        let span = &text[i..close];
        let was = unescape(span[28..].split('"').next().unwrap_or(""));
        // its paragraph, as converted (all fixes undone)
        let p0 = text[..i].rfind("\n\n").map_or(0, |k| k + 2);
        let p1 = text[i..].find("\n\n").map_or(text.len(), |k| i + k);
        let para = undo_fixes(&text[p0..p1]);
        data.fixes.push(Fix { slug: slug_at(i), paragraph: fingerprint(&para), was, span: span.to_string() });
    }
    // markers inside sentences
    for (a, b) in mark_ranges(text) {
        if inside(a) {
            continue;
        }
        let p0 = text[..a].rfind("\n\n").map_or(0, |k| k + 2);
        let p1 = text[b..].find("\n\n").map_or(text.len(), |k| b + k);
        // the text right before it (other markers left out), up to 40 bytes
        let before = without_marks(&text[p0..a]);
        let mut cut = before.len().saturating_sub(40);
        while !before.is_char_boundary(cut) {
            cut += 1;
        }
        data.marks.push(Mark {
            slug: slug_at(a),
            paragraph: fingerprint(&undo_fixes(&text[p0..p1])),
            before: before[cut..].to_string(),
            mark: text[a..b].to_string(),
        });
    }
    data
}

fn undo_fixes(t: &str) -> String {
    let mut out = String::new();
    let mut rest = t;
    while let Some(i) = rest.find("<span class=\"fix\" data-was=\"") {
        out.push_str(&rest[..i]);
        let span = &rest[i..];
        let was = unescape(span[28..].split('"').next().unwrap_or(""));
        out.push_str(&was);
        rest = span.find("</span>").map_or("", |c| &span[c + 7..]);
    }
    out.push_str(rest);
    out
}

/// Where the chapters begin in the text of book.md, each with its file's
/// name: they start with `<a id="slug"></a>`; their title follows, under
/// the bar and marker of the page they open on.
// (not an anchor inside a chapter of a course, "28-recitation-3--eq-3.7":
// the last equation of a chapter has the next chapter's heading within
// a few lines of it, and its correction did not reach book.md)
fn chapter_starts(text: &str) -> Vec<(usize, String)> {
    text.match_indices("<a id=\"")
        .filter_map(|(i, _)| {
            let id = text[i + 7..].split('"').next()?;
            let line_end = text[i..].find('\n').map_or(text.len(), |k| i + k);
            (text[i..line_end].trim() == format!("<a id=\"{id}\"></a>") && text[line_end..].lines().filter(|l| !l.trim().is_empty()).take(8).any(|l| l.starts_with("# ")) && id.chars().next().is_some_and(|c| c.is_ascii_digit()) && !id.contains("--"))
                .then(|| (i, id.to_string()))
        })
        .collect()
}

/// Notes and fixes in a converted book's folder (chapter files and
/// book.md; the same note found in both counts once).
pub fn collect(dir: &Path) -> UserData {
    let mut all = UserData::default();
    let Ok(entries) = std::fs::read_dir(dir) else { return all };
    let mut files: Vec<_> = entries.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "md")).collect();
    // chapter files first, book.md last: its notes count only if new
    files.sort_by_key(|p| p.file_name().is_some_and(|n| n == "book.md"));
    for p in files {
        let Ok(text) = std::fs::read_to_string(&p) else { continue };
        let name = p.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
        let data = if name == "book" {
            let starts = chapter_starts(&text);
            collect_file(&text, &|pos| starts.iter().rev().find(|(i, _)| *i <= pos).map(|(_, s)| s.clone()))
        } else if name == "check-equations" {
            continue;
        } else {
            collect_file(&text, &|_| Some(name.clone()))
        };
        if name == "book" {
            // (a note of book.md that a chapter's file has too is that note,
            // one for one: two notes of the same words are two notes)
            let mut taken = vec![false; all.notes.len()];
            for n in data.notes {
                let free = |i: &usize| !taken[*i] && all.notes[*i].text == n.text;
                let hit = (0..all.notes.len()).filter(free).find(|&i| all.notes[i].after == n.after).or_else(|| (0..all.notes.len()).find(free));
                match hit {
                    Some(i) => taken[i] = true,
                    None => {
                        all.notes.push(n);
                        taken.push(true);
                    }
                }
            }
        } else {
            all.notes.extend(data.notes);
        }
        for f in data.fixes {
            if !all.fixes.iter().any(|g| g.span == f.span && g.paragraph == f.paragraph) {
                all.fixes.push(f);
            }
        }
        for m in data.marks {
            if !all.marks.iter().any(|n| n.mark == m.mark && n.paragraph == m.paragraph) {
                all.marks.push(m);
            }
        }
        for e in data.eqs {
            if !all.eqs.iter().any(|g| g.slug == e.slug && g.was == e.was) {
                all.eqs.push(e);
            }
        }
    }
    all
}

/// The chapter files are named differently than when the notes were made
/// (the number in front changed, or prefaces became part of the front
/// matter): each note, fix and marker moves to the file that now has its
/// chapter's name, or else to the file that holds its paragraph. `files`:
/// name (without .md) and text of each chapter file as it is written now.
pub fn follow_renames(data: &mut UserData, files: &[(&str, &str)]) {
    let name = |slug: &str| slug.split_once('-').map_or(slug, |(_, n)| n).to_string();
    let prints: Vec<std::collections::HashSet<String>> =
        files.iter().map(|(_, text)| text.split("\n\n").map(fingerprint).filter(|f| !f.is_empty()).collect()).collect();
    let moved = |slug: &mut Option<String>, paragraph: &str| {
        let Some(old) = slug.as_deref() else { return };
        if files.iter().any(|(s, _)| *s == old) {
            return;
        }
        let by_name = files.iter().find(|(s, _)| name(s) == name(old)).map(|(s, _)| s.to_string());
        let by_text = || prints.iter().position(|p| !paragraph.is_empty() && p.contains(paragraph)).map(|i| files[i].0.to_string());
        if let Some(new) = by_name.or_else(by_text) {
            *slug = Some(new);
        }
    };
    for n in data.notes.iter_mut() {
        moved(&mut n.slug, &n.after);
    }
    for f in data.fixes.iter_mut() {
        moved(&mut f.slug, &f.paragraph);
    }
    for m in data.marks.iter_mut() {
        moved(&mut m.slug, &m.paragraph);
    }
    for e in data.eqs.iter_mut() {
        moved(&mut e.slug, "");
    }
}

/// Put notes and fixes into a newly written Markdown file (`slug`: the
/// chapter file it is; None for book.md). Returns the notes that found no
/// place (for the end of their chapter).
///
/// In book.md each chapter's notes and fixes go into that chapter's part
/// of the text: a sentence or an equation that two chapters share (a
/// summary repeats it) is corrected where the reader corrected it, not in
/// the first chapter that has it, nor in both. (Read again from book.md
/// it would be taken for a correction of the other chapter too.)
pub fn apply(text: &mut String, data: &UserData, slug: Option<&str>) -> Vec<Note> {
    if slug.is_some() {
        return apply_in(text, data, slug, false);
    }
    let starts = chapter_starts(text);
    if starts.is_empty() {
        return apply_in(text, data, None, true);
    }
    let mut lost = Vec::new();
    let mut out = String::with_capacity(text.len() + text.len() / 8);
    out.push_str(&text[..starts[0].0]);
    for (k, (from, name)) in starts.iter().enumerate() {
        let to = starts.get(k + 1).map_or(text.len(), |s| s.0);
        let mut part = text[*from..to].to_string();
        lost.extend(apply_in(&mut part, data, Some(name), true));
        out.push_str(&part);
    }
    // (what belongs to no chapter of the book as it is now: wherever it fits)
    let known = |s: &Option<String>| s.as_ref().is_some_and(|s| starts.iter().any(|(_, name)| name == s));
    let rest = UserData {
        notes: data.notes.iter().filter(|n| !known(&n.slug)).cloned().collect(),
        fixes: data.fixes.iter().filter(|f| !known(&f.slug)).cloned().collect(),
        marks: data.marks.iter().filter(|m| !known(&m.slug)).cloned().collect(),
        eqs: data.eqs.iter().filter(|e| !known(&e.slug)).cloned().collect(),
    };
    if !rest.is_empty() {
        lost.extend(apply_in(&mut out, &rest, None, true));
    }
    *text = out;
    lost
}

/// (`book`: the text is book.md's, or a part of it.)
fn apply_in(text: &mut String, data: &UserData, slug: Option<&str>, book: bool) -> Vec<Note> {
    let ours = |s: &Option<String>| slug.is_none() || s.as_deref() == slug;
    // fixes: in the paragraph with the same first words
    for f in data.fixes.iter().filter(|f| ours(&f.slug)) {
        let mut pos = 0;
        for block in text.clone().split("\n\n") {
            // (the paragraph as converted: a fix put back before this one,
            // near the paragraph's start, changes its first words)
            if fingerprint(&undo_fixes(block)) == f.paragraph {
                if let Some(k) = block.find(&f.was) {
                    let at = pos + k;
                    text.replace_range(at..at + f.was.len(), &f.span);
                }
                break;
            }
            pos += block.len() + 2;
        }
    }
    let mut lost = Vec::new();
    // corrected equations: every block converted to that LaTeX again
    // (one whose equation is converted differently now is kept with the
    // notes that lost their place)
    for e in data.eqs.iter().filter(|e| ours(&e.slug)) {
        // (the equation as converted, in the form of math the corrected
        // block is written in: a block of the other form is none of its
        // kind. A block taken out has no form: it is looked for in both.)
        let under = e.block.find(" -->").map_or("", |c| &e.block[c + 4..]);
        let form = BLOCKS.into_iter().find(|(open, _)| under.strip_prefix("\n\n").is_some_and(|t| t.starts_with(open)));
        let mut found = text.contains(&e.block);
        // (a text whose bulleted items are written already, a course's,
        // has the block of an item two spaces in: there it is looked for
        // so, and the corrected block is set in like it)
        let set_in = |t: &str| t.split('\n').map(|l| if l.is_empty() { String::new() } else { format!("  {l}") }).collect::<Vec<_>>().join("\n");
        for (open, close) in BLOCKS.into_iter().filter(|f| form.is_none_or(|g| g == *f)) {
            let at_margin = format!("{open}{}{close}", e.was);
            for (needle, block) in [(at_margin.clone(), e.block.clone()), (set_in(&at_margin), set_in(&e.block))] {
                let mut from = 0;
                while let Some(i) = text[from..].find(&needle).map(|i| from + i) {
                    // (not the corrected block of a fix put back before this
                    // one: such a block stands under a mark, which may be of
                    // several lines)
                    let fixed = text[..i].strip_suffix(" -->\n\n").and_then(|t| t.rfind("<!-- ").map(|k| &t[k..])).is_some_and(|m| m.starts_with(EQ_FIX));
                    let fresh = (i == 0 || text[..i].ends_with("\n\n")) && !fixed && text[i + needle.len()..].chars().next().is_none_or(|c| c == '\n');
                    if fresh {
                        text.replace_range(i..i + needle.len(), &block);
                        from = i + block.len();
                        found = true;
                    } else {
                        from = i + needle.len();
                    }
                }
            }
        }
        if !found && !book {
            lost.push(Note { slug: e.slug.clone(), after: String::new(), anchor: None, text: e.block.clone() });
        }
    }
    // markers: in their paragraph after the same text, else at its end
    for m in data.marks.iter().filter(|m| ours(&m.slug)) {
        let mut pos = 0;
        let mut placed = false;
        for block in text.clone().split("\n\n") {
            if fingerprint(&undo_fixes(block)) == m.paragraph {
                let at = if m.before.is_empty() {
                    None
                } else {
                    // find the context, skipping markers already put back
                    let plain = without_marks(block);
                    plain.find(&m.before).map(|k| {
                        let end_plain = k + m.before.len();
                        // map the position back into the block with markers
                        let mut seen = 0;
                        let mut last = 0;
                        let mut real = end_plain;
                        for (a, b) in mark_ranges(block) {
                            if seen + (a - last) >= end_plain {
                                break;
                            }
                            seen += a - last;
                            real += b - a;
                            last = b;
                        }
                        real
                    })
                };
                let at = pos + at.unwrap_or(block.len());
                text.insert_str(at, &m.mark);
                placed = true;
                break;
            }
            pos += block.len() + 2;
        }
        if !placed {
            lost.push(Note { slug: m.slug.clone(), after: m.paragraph.clone(), anchor: None, text: format!("A marker {} that followed “{}”", m.mark, m.before.trim()) });
        }
    }
    // notes: after their block, else after the block holding their anchor
    let mut inserts: Vec<(usize, &Note)> = Vec::new();
    for n in data.notes.iter().filter(|n| ours(&n.slug)) {
        let mut pos = 0;
        let mut at = None;
        // (under an equation read from a picture: under that picture's
        // equation, whatever its LaTeX is now)
        if let Some(k) = n.after.strip_prefix(EQUATION) {
            let open = format!("<!-- pdf2md:eq {k} -->\n\n```math\n");
            at = text.find(&open).and_then(|i| {
                let from = i + open.len();
                let end = text[from..].find("\n```").map(|j| from + j + 4)?;
                // (under mdBun's mark of an improvement, which has to
                // stay on the line right under the block)
                Some(end + text[end..].strip_prefix('\n').filter(|l| l.starts_with(IMPROVED)).map_or(0, |_| 1 + IMPROVED.len()))
            });
        }
        if let Some(k) = n.after.strip_prefix(CODE) {
            let open = format!("{}{k} -->\n\n```\n", crate::ocr::CODE_MARK);
            at = text.find(&open).and_then(|i| {
                let from = i + open.len() - 1;
                text[from..].find("\n```").map(|j| from + j + 4)
            });
        }
        for block in text.split("\n\n") {
            if at.is_some() || n.after.starts_with(EQUATION) || n.after.starts_with(CODE) {
                break;
            }
            if !n.after.is_empty() && fingerprint(&undo_fixes(block)) == n.after {
                // after the block's text (before a final newline of the file)
                at = Some(pos + block.trim_end_matches('\n').len());
                break;
            }
            pos += block.len() + 2;
        }
        if at.is_none() {
            if let Some(a) = &n.anchor {
                if let Some(i) = text.find(&format!("id=\"{a}\"")) {
                    at = Some(text[i..].find("\n\n").map_or(text.len(), |k| i + k));
                }
            }
        }
        match at {
            Some(p) => inserts.push((p, n)),
            None => lost.push(n.clone()),
        }
    }
    // from the end, so earlier places stay valid; notes at one place keep
    // their order
    inserts.sort_by_key(|(p, _)| std::cmp::Reverse(*p));
    let mut i = 0;
    while i < inserts.len() {
        let p = inserts[i].0;
        let mut same: Vec<&Note> = inserts[i..].iter().take_while(|(q, _)| *q == p).map(|(_, n)| *n).collect();
        i += same.len();
        same.sort_by_key(|n| data.notes.iter().position(|m| m == *n));
        let block: String = same.iter().map(|n| format!("\n\n{}", n.text)).collect();
        text.insert_str(p, &block);
    }
    lost
}

/// Keep a copy of the notes and fixes outside the book folder:
/// ~/.local/share/pdf2md/notes/<book folder name>.md (overwritten each time).
pub fn backup(data: &UserData, book_dir: &Path) {
    if data.is_empty() {
        return;
    }
    let Some(base) = std::env::var_os("XDG_DATA_HOME")
        .map(std::path::PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| std::path::PathBuf::from(h).join(".local/share")))
    else {
        return;
    };
    let dir = base.join("pdf2md").join("notes");
    if std::fs::create_dir_all(&dir).is_err() {
        return;
    }
    let name = book_dir.file_name().map(|s| s.to_string_lossy().into_owned()).unwrap_or_else(|| "book".into());
    let mut s = format!("# Notes and fixes in {name}\n\n");
    for n in &data.notes {
        s.push_str(&format!("<!-- in {} after: {} -->\n\n{}\n\n", n.slug.as_deref().unwrap_or("?"), n.after, n.text));
    }
    for f in &data.fixes {
        s.push_str(&format!("<!-- fix in {} -->\n{}\n\n", f.slug.as_deref().unwrap_or("?"), f.span));
    }
    for e in &data.eqs {
        s.push_str(&format!("<!-- equation fix in {} -->\n{}\n\n", e.slug.as_deref().unwrap_or("?"), e.block));
    }
    let _ = std::fs::write(dir.join(format!("{name}.md")), s);
}

#[cfg(test)]
mod tests {
    use super::*;

    const OLD: &str = "# Chapter\n\nFirst paragraph about spin.\n\n<a id=\"eq-1.1\"></a>\n\n$$\nx=1 \\tag{1.1}\n$$\n\n<details class=\"mine\">\n<summary>✎ My note</summary>\n\nWhy *x* is 1.\n\n</details>\n\nSecond paragraph with a <span class=\"fix\" data-was=\"rnistake\">mistake</span> in it.\n\n> [!WARNING]\n> I think this is wrong.\n\nThird paragraph.\n";

    #[test]
    fn a_note_under_an_equation_stays_under_it_when_the_equation_is_corrected() {
        let eq = |latex: &str, note: &str| format!("Before it.\n\n<a id=\"img-0123456789abcdef\"></a>\n<!-- pdf2md:eq 0123456789abcdef -->\n\n```math\n{latex} \\qquad \\text{{(1.1)}}\n```{note}\n\nAfter it.\n");
        let note = "\n\n> [!NOTE]\n> The index was read as I.";
        let d = collect_file(&eq("u_{EI} = A r", note), &|_| Some("c".into()));
        assert_eq!(d.notes[0].after, "pdf2md:eq 0123456789abcdef");
        // (the next conversion writes the corrected equation)
        let mut new = eq("u_{El} = A r", "");
        assert!(apply(&mut new, &d, Some("c")).is_empty());
        assert_eq!(new, eq("u_{El} = A r", note));
        assert_eq!(collect_file(&new, &|_| Some("c".into())), d);
        // (a note that came to stand between the mark and the equation)
        let between = eq("u_{El} = A r", "").replace(" -->\n\n```math", " -->\n\n> [!NOTE]\n> The index was read as I.\n\n```math");
        assert_eq!(collect_file(&between, &|_| Some("c".into())), d);
    }

    #[test]
    fn a_note_under_program_code_stays_under_it_when_the_code_is_corrected() {
        let code = |text: &str, note: &str| format!("Before it.\n\n<!-- pdf2md:code 0123456789abcdef -->\n\n```\n{text}\n```{note}\n\nAfter it.\n");
        let note = "\n\n> [!NOTE]\n> The letter l was read as 1.";
        // (a listing with an empty line in it: the note is under all of it)
        let d = collect_file(&code("f[1_] = 1\n\n  g[x]", note), &|_| Some("c".into()));
        assert_eq!(d.notes[0].after, "pdf2md:code 0123456789abcdef");
        let mut new = code("f[l_] = l\n\n  g[x]", "");
        assert!(apply(&mut new, &d, Some("c")).is_empty());
        assert_eq!(new, code("f[l_] = l\n\n  g[x]", note));
        assert_eq!(collect_file(&new, &|_| Some("c".into())), d);
        // (a block of code that has no mark, or another block's: by its words)
        let plain = "Before it.\n\n<!-- pdf2md:code 0123456789abcdef -->\n\n```\na\n```\n\nSo:\n\n```\nb\n```\n\n> [!NOTE]\n> Mine.\n";
        assert!(!collect_file(plain, &|_| Some("c".into())).notes[0].after.starts_with(CODE));
    }

    #[test]
    fn the_mark_of_an_improvement_stays_right_under_its_equation() {
        // read from a picture: the conversion writes the mark again (kept
        // by picture), and a note under the equation goes under the mark
        let eq = |note: &str| format!("Before it.\n\n<a id=\"img-0123456789abcdef\"></a>\n<!-- pdf2md:eq 0123456789abcdef -->\n\n```math\nu = A r\n```\n{IMPROVED}{note}\n\nAfter it.\n");
        let note = "\n\n> [!WARNING]\n> The book has B here.";
        let d = collect_file(&eq(note), &|_| Some("c".into()));
        assert_eq!(d.notes[0].after, "pdf2md:eq 0123456789abcdef");
        let mut new = eq("");
        assert!(apply(&mut new, &d, Some("c")).is_empty());
        assert_eq!(new, eq(note));
        // built from the PDF's text: the mark is part of the corrected
        // block, between the block and the alert that belongs to it
        let was = "x = {}_{2} y";
        let fixed = format!("{}\n\n```math\nx = y^{{2}}\n```\n{IMPROVED}\n\n> [!NOTE]\n> **Edited by Claude.** The square.", eq_fix_mark(was));
        let edited = format!("# Ch\n\nSo\n\n{fixed}\n\nEnd.\n");
        let d = collect_file(&edited, &|_| Some("c".into()));
        assert_eq!((d.eqs.len(), d.notes.len()), (1, 0), "{d:?}");
        assert_eq!(d.eqs[0].block, fixed);
        let mut new = format!("# Ch\n\nSo\n\n```math\n{was}\n```\n\nEnd.\n");
        assert!(apply(&mut new, &d, Some("c")).is_empty());
        assert_eq!(new, edited);
        // (in a list item, two spaces in)
        let item = edited.replace("So\n\n", "- So\n\n").replace(&fixed, &fixed.lines().map(|l| if l.is_empty() { String::new() } else { format!("  {l}") }).collect::<Vec<_>>().join("\n"));
        assert_eq!(collect_file(&item, &|_| Some("c".into())).eqs[0].block, fixed);
        // (and put back into a text whose items are written already)
        let mut new = format!("# Ch\n\n- So\n\n  ```math\n  {was}\n  ```\n\nEnd.\n");
        assert!(apply(&mut new, &d, Some("c")).is_empty());
        assert_eq!(new, item);
    }

    #[test]
    fn a_corrected_equation_between_dollars_is_kept_like_a_fenced_one() {
        // (a book written in the standard form; the mark of several lines)
        let was = "a & = & b -- c \\\\\nd & = & e \\tag{1.2}";
        let fixed = format!("{}\n\n$$\na & = & b - c \\\\\nd & = & e \\tag{{1.2}}\n$$\n{IMPROVED}", eq_fix_mark(was));
        let edited = format!("# Ch\n\nSo\n\n{fixed}\n\n> [!NOTE]\n> Mine.\n\nand\n\n$$\nx = 1\n$$\n\nEnd.\n");
        let d = collect_file(&edited, &|_| Some("c".into()));
        assert_eq!((d.eqs.len(), d.notes.len()), (1, 1), "{d:?}");
        assert_eq!((d.eqs[0].was.as_str(), d.eqs[0].block.as_str()), (was, fixed.as_str()));
        let fresh = |form: [&str; 2]| format!("# Ch\n\nSo\n\n{}{was}{}\n\nand\n\n$$\nx = 1\n$$\n\nEnd.\n", form[0], form[1]);
        let mut new = fresh(["$$\n", "\n$$"]);
        assert!(apply(&mut new, &d, Some("c")).is_empty());
        assert_eq!(new, edited);
        // (put back once: the corrected block is not taken for a block to correct)
        let mut again = new.clone();
        apply(&mut again, &UserData { eqs: d.eqs.clone(), ..Default::default() }, Some("c"));
        assert_eq!(again, new);
        // (the same LaTeX in the other form is another block: the fix
        // finds no place and is kept with the notes that lost theirs)
        let mut other = fresh(["```math\n", "\n```"]);
        let lost = apply(&mut other, &d, Some("c"));
        assert!(other.contains(&format!("```math\n{was}\n```")) && lost.iter().any(|n| n.text == fixed), "{other}");
        // (a block taken out is looked for in both forms)
        let gone = UserData { eqs: vec![EqFix { slug: Some("c".into()), was: "x = 1".into(), block: eq_gone_mark("x = 1") }], ..Default::default() };
        let mut new = fresh(["$$\n", "\n$$"]);
        assert!(apply(&mut new, &gone, Some("c")).is_empty());
        assert!(new.contains(&format!("and\n\n{}\n\nEnd.", eq_gone_mark("x = 1"))), "{new}");
    }

    #[test]
    fn a_corrected_equation_of_a_text_pdf_is_corrected_again_wherever_it_comes_back() {
        let was = "B_{2} = {}_{2} \\frac{a}{b}_{2} -- c";
        let fixed = format!("{}\n\n```math\nB_{{2}} = \\frac{{a}}{{b}} - c\n```\n\n> [!NOTE]\n> **Edited by Claude.** Two stray indices taken out.", eq_fix_mark(was));
        let edited = format!("# Ch\n\nSo\n\n{fixed}\n\n> [!NOTE]\n> My own note.\n\nand again\n\n{fixed}\n\nEnd.\n");
        let d = collect_file(&edited, &|_| Some("c".into()));
        assert_eq!(d.eqs.len(), 2);
        assert_eq!(d.eqs[0].was, was);
        assert_eq!(d.notes.len(), 1, "{:?}", d.notes);
        let mut d = d;
        d.eqs.truncate(1);
        let mut new = format!("# Ch\n\nSo\n\n```math\n{was}\n```\n\nand again\n\n```math\n{was}\n```\n\nEnd.\n");
        assert!(apply(&mut new, &d, Some("c")).is_empty());
        assert_eq!(new, edited);
        // (converted differently now: kept, at the chapter's end)
        let mut other = "# Ch\n\nSo\n\n```math\nB_{2} = \\frac{a}{b}\n```\n\nEnd.\n".to_string();
        let lost = apply(&mut other, &d, Some("c"));
        assert_eq!(lost.len(), 2);
        assert!(lost.iter().any(|n| n.text == fixed));
        // (a block taken out: its mark alone)
        // (… also right before another equation)
        let gone = format!("# Ch\n\nSo\n\n{}\n\n```math\nx = 1\n```\n\nEnd.\n", eq_gone_mark("{}_{y} \\quad 0"));
        let d = collect_file(&gone, &|_| Some("c".into()));
        assert_eq!((d.eqs.len(), d.eqs[0].was.as_str()), (1, "{}_{y} \\quad 0"));
        assert_eq!(d.eqs[0].block, eq_gone_mark("{}_{y} \\quad 0"));
        let mut new = "# Ch\n\nSo\n\n```math\n{}_{y} \\quad 0\n```\n\n```math\nx = 1\n```\n\nEnd.\n".to_string();
        assert!(apply(&mut new, &d, Some("c")).is_empty());
        assert_eq!(new, gone);
    }

    #[test]
    fn what_stands_in_a_bulleted_item_is_read_at_the_margin() {
        let item = "# Ch\n\n- First point, as we see in\n\n  <!-- pdf2md:fix x = {}_{2} y -->\n\n  ```math\n  x = y^{2}\n  ```\n\n  > [!NOTE]\n  > **Edited by Claude.** The square.\n\n  and so on.\n\n  > [!NOTE]\n  > Mine.\n\nAfter.\n";
        let d = collect_file(item, &|_| Some("c".into()));
        assert_eq!(d.eqs.len(), 1);
        assert_eq!(d.eqs[0].was, "x = {}_{2} y");
        assert_eq!(d.eqs[0].block, "<!-- pdf2md:fix x = {}_{2} y -->\n\n```math\nx = y^{2}\n```\n\n> [!NOTE]\n> **Edited by Claude.** The square.");
        assert_eq!(d.notes.len(), 1);
        assert_eq!((d.notes[0].text.as_str(), d.notes[0].after.as_str()), ("> [!NOTE]\n> Mine.", "and so on"));
        // (put back into the text as it is before its items are written)
        let mut new = "# Ch\n\n\u{1}First point, as we see in\n\n```math\nx = {}_{2} y\n```\n\nand so on.\n\n\u{2}After.\n".to_string();
        assert!(apply(&mut new, &d, Some("c")).is_empty());
        assert!(new.contains("```math\nx = y^{2}\n```\n\n> [!NOTE]\n> **Edited by Claude.** The square.\n\nand so on.\n\n> [!NOTE]\n> Mine.\n\n\u{2}After."), "{new}");
    }

    #[test]
    fn two_notes_of_the_same_words_are_two_notes() {
        let dir = std::env::temp_dir().join(format!("pdf2md-notes-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let chapter = "# One\n\nFirst paragraph.\n\n> [!NOTE]\n> Bold in the book.\n\nSecond paragraph.\n\n> [!NOTE]\n> Bold in the book.\n";
        std::fs::write(dir.join("01-one.md"), chapter).unwrap();
        std::fs::write(dir.join("book.md"), format!("<a id=\"01-one\"></a>\n\n{chapter}")).unwrap();
        let d = collect(&dir);
        std::fs::remove_dir_all(&dir).unwrap();
        assert_eq!(d.notes.len(), 2);
        assert_ne!(d.notes[0].after, d.notes[1].after);
    }

    #[test]
    fn two_fixes_in_one_paragraph_are_both_redone() {
        let edited = "# Chapter\n\nThe <span class=\"fix\" data-was=\"frist\">first</span> word and the <span class=\"fix\" data-was=\"lsat\">last</span> one.\n";
        let d = collect_file(edited, &|_| Some("01-chapter".into()));
        assert_eq!(d.fixes.len(), 2);
        let mut new = "# Chapter\n\nThe frist word and the lsat one.\n".to_string();
        apply(&mut new, &d, Some("01-chapter"));
        assert_eq!(new, edited);
    }

    #[test]
    fn a_formula_kept_in_a_fix_by_its_characters_numbers() {
        let edited = "# Chapter\n\nSo <span class=\"fix\" data-was=\"&#36;&#96;e^{-Z}&#96;&#36;eff\">$`e^{-Z_{\\mathrm{eff}}}`$</span> it is.\n";
        let d = collect_file(edited, &|_| Some("01-chapter".into()));
        assert_eq!(d.fixes[0].was, "$`e^{-Z}`$eff");
        let mut new = "# Chapter\n\nSo $`e^{-Z}`$eff it is.\n".to_string();
        apply(&mut new, &d, Some("01-chapter"));
        assert_eq!(new, edited);
    }

    #[test]
    fn notes_follow_renamed_files() {
        let mut d = collect_file(OLD, &|_| Some("06-fundamental-concepts".into()));
        let mut e = collect_file(OLD, &|_| Some("02-preface".into()));
        let new = "# Chapter\n\nFirst paragraph about spin.\n\nSecond paragraph with a rnistake in it.\n";
        let files = [("00-front-matter", new), ("01-fundamental-concepts", "# Other\n\nNothing alike.\n")];
        // the chapter's new number
        follow_renames(&mut d, &files);
        assert!(d.notes.iter().all(|n| n.slug.as_deref() == Some("01-fundamental-concepts")));
        assert_eq!(d.fixes[0].slug.as_deref(), Some("01-fundamental-concepts"));
        // a preface that is part of the front matter now: by its paragraph
        follow_renames(&mut e, &files);
        assert_eq!(e.notes[1].slug.as_deref(), Some("00-front-matter"));
        assert_eq!(e.fixes[0].slug.as_deref(), Some("00-front-matter"));
    }

    #[test]
    fn finds_notes_and_fixes() {
        let d = collect_file(OLD, &|_| Some("01-ch".into()));
        assert_eq!(d.notes.len(), 2);
        assert_eq!(d.notes[0].anchor.as_deref(), Some("eq-1.1"));
        assert!(d.notes[0].text.starts_with("<details") && d.notes[0].text.ends_with("</details>"));
        assert_eq!(d.notes[1].after, "second paragraph with a rnistake in it");
        assert_eq!(d.fixes.len(), 1);
        assert_eq!(d.fixes[0].was, "rnistake");
    }

    #[test]
    fn keeps_markers_in_sentences() {
        let old = "# Ch\n\nThe spin is one half,<sup>[✎](#mine-1)</sup> as Stern found.\n\n<details class=\"mine\" id=\"mine-1\">\n<summary>✎ note</summary>\n\nWhy.\n\n</details>\n";
        let d = collect_file(old, &|_| Some("c".into()));
        assert_eq!(d.marks.len(), 1);
        let mut new = "# Ch\n\nThe spin is one half, as Stern found.\n".to_string();
        let lost = apply(&mut new, &d, Some("c"));
        assert!(lost.is_empty(), "{lost:?}");
        assert_eq!(new, old.trim_end().to_string() + "\n");
        // context gone: at the end of its paragraph
        let mut changed = "# Ch\n\nThe spin is one half, as Stern found.\n".replace("one half,", "1/2,");
        let d2 = UserData { marks: vec![Mark { paragraph: fingerprint("The spin is 1/2, as Stern found."), ..d.marks[0].clone() }], ..Default::default() };
        apply(&mut changed, &d2, Some("c"));
        assert!(changed.contains("as Stern found.\n<sup>") || changed.contains("as Stern found.<sup>"), "{changed}");
    }

    #[test]
    fn puts_them_back() {
        let d = collect_file(OLD, &|_| Some("01-ch".into()));
        // a fresh conversion: no notes, the mistake back, a new paragraph before
        let mut new = "# Chapter\n\nA new first paragraph.\n\nFirst paragraph about spin.\n\n<a id=\"eq-1.1\"></a>\n\n$$\nx=1 \\tag{1.1}\n$$\n\nSecond paragraph with a rnistake in it.\n\nThird paragraph.\n".to_string();
        let lost = apply(&mut new, &d, Some("01-ch"));
        assert!(lost.is_empty());
        let want = OLD.replace("# Chapter\n\n", "# Chapter\n\nA new first paragraph.\n\n");
        assert_eq!(new, want);
        // applying again to the result changes nothing
        let d2 = collect_file(&new, &|_| Some("01-ch".into()));
        assert_eq!(d2, d);
    }

    #[test]
    fn a_correction_stays_in_its_chapter_of_the_whole_book() {
        // two chapters hold the same equation and the same sentence; the
        // reader corrected them in the second
        let chapter = |n: &str| format!("<a id=\"{n}\"></a>\n\n# Title {n}\n\nThe same words here.\n\n```math\na b\n```\n\n");
        let book = format!("# Book\n\n{}{}", chapter("01-one"), chapter("02-two"));
        let block = format!("{} a b -->\n\n```math\na + b\n```", EQ_FIX);
        let d = UserData {
            eqs: vec![EqFix { slug: Some("02-two".into()), was: "a b".into(), block: block.clone() }],
            fixes: vec![Fix { slug: Some("02-two".into()), paragraph: fingerprint("The same words here."), was: "same".into(), span: "<span class=\"fix\" data-was=\"same\">very</span>".into() }],
            ..Default::default()
        };
        let mut new = book.clone();
        assert!(apply(&mut new, &d, None).is_empty());
        let two = new.find("<a id=\"02-two\"").unwrap();
        assert!(new[..two].contains("```math\na b\n```") && new[..two].contains("The same words here."), "{new}");
        assert!(new[two..].contains(&block) && !new[two..].contains("```math\na b\n```") && new[two..].contains("The <span class=\"fix\" data-was=\"same\">very</span> words here."), "{new}");
        // read again from the book, each is the second chapter's alone
        let read = collect_file(&new, &|pos| chapter_starts(&new).iter().rev().find(|(i, _)| *i <= pos).map(|(_, s)| s.clone()));
        assert_eq!(read.eqs.iter().map(|e| e.slug.as_deref()).collect::<Vec<_>>(), [Some("02-two")]);
        assert_eq!(read.fixes.iter().map(|f| f.slug.as_deref()).collect::<Vec<_>>(), [Some("02-two")]);
    }
}
