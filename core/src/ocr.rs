//! Equations that are pictures. Ebook-made PDFs (calibre, Kindle) carry
//! their math as small raster images instead of text. Each image is handed
//! to a local math-recognition model (`math_ocr.py`, run with the Python of
//! its own environment) that answers with LaTeX, or says the image is a
//! picture and no math (a diagram, a logo), or that it cannot read it
//! (such images stay pictures). Answers
//! are cached by image content in
//! `~/.cache/pdf2md/ocr/`, so converting a book again is instant.
//!
//! Without the recognizer the conversion still works: such images then
//! stay pictures.

use crate::progress::Step;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

/// What the recognizer made of one image.
#[derive(Debug, Clone, PartialEq)]
pub enum Reading {
    Latex(String),
    /// a plot, a diagram or lettering (a logo), not math
    Drawing,
    /// math the recognizer could not read plausibly
    Unreadable,
    /// program code (read as plain text), not math
    Code(String),
}

/// The recognizer script, written next to the cache when it is run.
const SCRIPT: &str = include_str!("math_ocr.py");

/// Stable name for an image: FNV-1a 64 of its data.
pub fn key(png_base64: &str) -> String {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in png_base64.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("{h:016x}")
}

/// Hand corrections of equations, by image: `~/.local/share/pdf2md/corrections/`
/// (apart from the cache, so clearing the cache never loses them).
fn corrections_dir() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")))?;
    Some(base.join("pdf2md").join("corrections"))
}

/// The reader's correction of a table's cell (`row`, `column` from 0),
/// if there is one: the cell's Markdown as the reader left it (which may
/// be nothing: a cell emptied by hand).
pub(crate) fn read_cell_correction(table: &str, row: usize, column: usize) -> Option<String> {
    let t = std::fs::read_to_string(corrections_dir()?.join(format!("{table}.{row}.{column}.cell"))).ok()?;
    Some(t.trim().to_string())
}

/// The tables of a Markdown text that carry a key (`<!-- pdf2md:table KEY
/// -->` above `<table>`), cell by cell: "KEY.row.column" and the cell's
/// Markdown.
pub fn marked_tables(text: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut rest = text;
    while let Some(i) = rest.find("<!-- pdf2md:table ") {
        rest = &rest[i + 18..];
        let Some(k) = rest.get(..16).filter(|k| k.chars().all(|c| c.is_ascii_hexdigit())) else { continue };
        let Some(end) = rest.find("</table>") else { break };
        let table = &rest[..end];
        for (r, row) in table.split("<tr>").skip(1).enumerate() {
            for (c, cell) in row.split("<td").skip(1).enumerate() {
                let inside = cell.split_once('>').map_or("", |x| x.1);
                let inside = inside.split("</td>").next().unwrap_or("");
                out.push((format!("{k}.{r}.{c}"), inside.trim().to_string()));
            }
        }
        rest = &rest[end..];
    }
    out
}

/// How the picture under each table that carries a key stands in a
/// Markdown text: "KEY.picture" and "open" (shown), "closed" (folded away:
/// `<details>` without `open`) or "removed".
pub fn marked_table_pictures(text: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut rest = text;
    while let Some(i) = rest.find("<!-- pdf2md:table ") {
        rest = &rest[i + 18..];
        let Some(k) = rest.get(..16).filter(|k| k.chars().all(|c| c.is_ascii_hexdigit())) else { continue };
        let Some(end) = rest.find("</table>") else { break };
        rest = &rest[end + 8..];
        let after = rest.trim_start();
        let state = match after.strip_prefix("<details") {
            Some(tag) if tag.split('>').next().is_some_and(|t| t.contains("open")) => "open",
            Some(_) => "closed",
            None => "removed",
        };
        out.push((format!("{k}.picture"), state.to_string()));
    }
    out
}

/// How the reader left the picture under a table, if not shown: "closed"
/// or "removed".
pub(crate) fn read_picture_state(table: &str) -> Option<String> {
    let t = std::fs::read_to_string(corrections_dir()?.join(format!("{table}.picture"))).ok()?;
    let t = t.trim();
    matches!(t, "closed" | "removed").then(|| t.to_string())
}

pub(crate) fn read_correction(k: &str) -> Option<String> {
    let t = std::fs::read_to_string(corrections_dir()?.join(format!("{k}.tex"))).ok()?;
    let t = t.trim();
    (!t.is_empty()).then(|| t.to_string())
}

/// Before a book is converted again: find the equations the reader has
/// corrected by hand in the old Markdown (each displayed equation read
/// from an image carries `<!-- pdf2md:eq KEY -->` above it) and keep each
/// correction for that image. Returns how many were kept. Where the
/// chapter files and book.md disagree, the most recently changed file wins.
pub fn collect_corrections(dir: &std::path::Path) -> usize {
    let (Some(cdir), Ok(entries)) = (corrections_dir(), std::fs::read_dir(dir)) else { return 0 };
    // what pdf2md wrote there for each equation (see `record_written`)
    let written_then = read_written(dir);
    // image -> (time of the file, LaTeX as it stands there)
    let mut found: HashMap<String, (std::time::SystemTime, String)> = HashMap::new();
    for e in entries.flatten() {
        let p = e.path();
        if p.extension().is_none_or(|x| x != "md") {
            continue;
        }
        let (Ok(text), Ok(meta)) = (std::fs::read_to_string(&p), e.metadata()) else { continue };
        let time = meta.modified().unwrap_or(std::time::UNIX_EPOCH);
        for (k, latex) in marked_equations(&text) {
            let changed = match &written_then {
                // compared with exactly what was written
                Some(w) => w.get(&k).is_some_and(|w| *w != norm(&latex)),
                // a folder from before the record: compared with the current
                // reading, and only while the picture is still read as math
                // (a reading that became code or a drawing is no correction)
                None => {
                    let now = read_correction(&k).or_else(|| cache_dir().and_then(|d| read_cached(&d, &k)).and_then(|r| match r {
                        Reading::Latex(t) => Some(t),
                        _ => None,
                    }));
                    now.is_some_and(|w| norm(&w) != norm(&latex))
                }
            };
            if !changed {
                continue;
            }
            match found.get(&k) {
                Some((t, _)) if *t >= time => {}
                _ => {
                    found.insert(k, (time, latex));
                }
            }
        }
    }
    // The cells of tables the reader corrected: each cell that is not
    // what the conversion wrote there (only with the record of that: a
    // table has no reading to compare with).
    let mut cells: HashMap<String, (std::time::SystemTime, String)> = HashMap::new();
    let mut pictures: HashMap<String, (std::time::SystemTime, String)> = HashMap::new();
    if let (Some(written), Ok(entries)) = (&written_then, std::fs::read_dir(dir)) {
        for e in entries.flatten() {
            let p = e.path();
            if p.extension().is_none_or(|x| x != "md") {
                continue;
            }
            let (Ok(text), Ok(meta)) = (std::fs::read_to_string(&p), e.metadata()) else { continue };
            let time = meta.modified().unwrap_or(std::time::UNIX_EPOCH);
            for (id, cell) in marked_tables(&text) {
                if written.get(&id).is_none_or(|w| *w == norm(&cell)) {
                    continue;
                }
                match cells.get(&id) {
                    Some((t, _)) if *t >= time => {}
                    _ => {
                        cells.insert(id, (time, cell));
                    }
                }
            }
            // (and the picture under a table: folded away, taken out, or shown again)
            for (id, state) in marked_table_pictures(&text) {
                if written.get(&id).is_none_or(|w| *w == state) {
                    continue;
                }
                match pictures.get(&id) {
                    Some((t, _)) if *t >= time => {}
                    _ => {
                        pictures.insert(id, (time, state));
                    }
                }
            }
        }
    }
    if (found.is_empty() && cells.is_empty() && pictures.is_empty()) || std::fs::create_dir_all(&cdir).is_err() {
        return 0;
    }
    let mut kept = 0;
    for (id, (_, state)) in &pictures {
        // (shown again: nothing to keep any more)
        let done = if state == "open" { std::fs::remove_file(cdir.join(id)).is_ok() } else { std::fs::write(cdir.join(id), format!("{state}\n")).is_ok() };
        kept += usize::from(done);
    }
    found.iter().filter(|(k, (_, latex))| std::fs::write(cdir.join(format!("{k}.tex")), format!("{latex}\n")).is_ok()).count()
        + cells.iter().filter(|(id, (_, cell))| std::fs::write(cdir.join(format!("{id}.cell")), format!("{cell}\n")).is_ok()).count()
        + kept
}

fn norm(t: &str) -> String {
    t.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The marked equations of a Markdown text: picture key and LaTeX (without
/// its number), for displayed ones (`<!-- pdf2md:eq KEY -->` above the
/// ```` ```math ```` block) and ones inside sentences (the marker right
/// behind ``$`…`$``: in front, a marker that starts a paragraph would make
/// GitHub take the whole line for HTML and show it as it is).
///
/// A book written before formulas were put in GitHub's forms has them
/// between `$` and `$$`, the marker in front in sentences too: read as
/// well, the corrections made in it count.
pub fn marked_equations(text: &str) -> Vec<(String, String)> {
    use crate::emit::{DISPLAY_CLOSE, DISPLAY_OPEN, MATH_CLOSE, MATH_OPEN};
    const MARK: &str = "<!-- pdf2md:eq ";
    let mut out = Vec::new();
    let mut from = 0;
    while let Some(at) = text[from..].find(MARK).map(|i| from + i) {
        from = at + MARK.len();
        let Some(k) = text.get(from..from + 16).filter(|k| k.chars().all(|c| c.is_ascii_hexdigit())) else { continue };
        let after = text.get(from + 20..).unwrap_or("");
        let before = &text[..at];
        let latex = if let Some(inline) = before.strip_suffix(MATH_CLOSE) {
            let Some(start) = inline.rfind(MATH_OPEN) else { continue };
            &inline[start + MATH_OPEN.len()..]
        } else if let Some(inline) = after.strip_prefix('$').filter(|i| !i.starts_with('$')) {
            let Some(end) = inline.find('$') else { break };
            from = text.len() - inline.len() + end;
            &inline[..end]
        } else {
            let forms = [(DISPLAY_OPEN, DISPLAY_CLOSE), ("$$\n", "\n$$")];
            let Some((start, open, close)) = forms.iter().filter_map(|(o, c)| after.find(o).map(|i| (i, o, c))).min() else { break };
            let body = &after[start + open.len()..];
            let Some(end) = body.find(close) else { break };
            from = text.len() - body.len() + end;
            &body[..end]
        };
        out.push((k.to_string(), strip_numbers(latex)));
    }
    out
}

/// The record of what a conversion wrote for each marked equation, kept in
/// the book folder (`.pdf2md-written`: key, tab, LaTeX on one line), so the
/// next conversion can tell the reader's edits from improved readings.
const WRITTEN: &str = ".pdf2md-written";

pub fn record_written(dir: &std::path::Path, texts: &[&str]) -> std::io::Result<()> {
    let mut seen = std::collections::HashSet::new();
    let mut s = String::new();
    for t in texts {
        for (k, latex) in marked_equations(t) {
            if seen.insert(k.clone()) {
                s.push_str(&format!("{k}\t{}\n", norm(&latex)));
            }
        }
    }
    // (and each cell of each table that has a key: "KEY.row.column")
    for t in texts {
        for (id, cell) in marked_tables(t) {
            if seen.insert(id.clone()) {
                s.push_str(&format!("{id}\t{}\n", norm(&cell)));
            }
        }
    }
    // (and how the picture under each such table was written)
    for t in texts {
        for (id, state) in marked_table_pictures(t) {
            if state != "removed" && seen.insert(id.clone()) {
                s.push_str(&format!("{id}\t{state}\n"));
            }
        }
    }
    if s.is_empty() {
        // no equation pictures (a TeX book): nothing to record
        let _ = std::fs::remove_file(dir.join(WRITTEN));
        return Ok(());
    }
    std::fs::write(dir.join(WRITTEN), s)
}

fn read_written(dir: &std::path::Path) -> Option<HashMap<String, String>> {
    let t = std::fs::read_to_string(dir.join(WRITTEN)).ok()?;
    Some(t.lines().filter_map(|l| l.split_once('\t')).map(|(k, v)| (k.to_string(), v.to_string())).collect())
}

fn cache_dir() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".cache")))?;
    Some(base.join("pdf2md").join("ocr"))
}

/// The Python that has the model installed: `PDF2MD_OCR_PYTHON`, or the
/// environment set up as the README describes.
fn python() -> Option<PathBuf> {
    if let Some(p) = std::env::var_os("PDF2MD_OCR_PYTHON") {
        return Some(PathBuf::from(p));
    }
    let home = PathBuf::from(std::env::var_os("HOME")?);
    let p = home.join(".local/share/pdf2md/ocr/.venv/bin/python");
    p.exists().then_some(p)
}

fn read_cached(dir: &std::path::Path, k: &str) -> Option<Reading> {
    let t = std::fs::read_to_string(dir.join(format!("{k}.tex"))).ok()?;
    if let Some(code) = t.strip_prefix("%CODE\n") {
        return Some(Reading::Code(code.trim_end().to_string()));
    }
    let t = t.trim();
    if t == "%DRAWING" {
        Some(Reading::Drawing)
    } else if t == "%UNREADABLE" {
        Some(Reading::Unreadable)
    } else if t.is_empty() {
        None
    } else {
        let latex = align_rows(&unglue(unwrap_cell(&strip_numbers(t))));
        // words set small under parts of the equation, found in the image
        let labels: Vec<(f32, String)> = std::fs::read_to_string(dir.join(format!("{k}.labels")))
            .unwrap_or_default()
            .lines()
            .filter_map(|l| l.split_once('\t'))
            .filter_map(|(at, word)| Some((at.trim().parse().ok()?, word.trim().to_string())))
            .collect();
        Some(Reading::Latex(with_labels(&latex, &labels)))
    }
}

/// An equation with words set small under parts of it ("bra" and "ket"
/// under the two halves of a product), each word centred where it stands
/// in the image: `labels` gives the middle of each word as a fraction of
/// the equation's width. The words go in a second row, each moved to its
/// place from the left edge; how wide the equation comes out is estimated
/// (`measure`), so a word may stand a little off.
fn with_labels(latex: &str, labels: &[(f32, String)]) -> String {
    // (an equation of several rows has no one width to measure along)
    if labels.is_empty() || latex.contains("\\\\") || latex.contains("\\begin") {
        return latex.to_string();
    }
    let width = crate::measure::math_em(latex);
    let row: String = labels
        .iter()
        .map(|(at, word)| {
            let word: String = word.chars().filter(|c| !matches!(c, '\\' | '{' | '}' | '$' | '%' | '#' | '&' | '_' | '^' | '~')).collect();
            format!("\\mathrlap{{\\kern{:.2}em\\mathclap{{\\scriptsize\\text{{{word}}}}}}}", (at * width).max(0.0))
        })
        .collect();
    format!("\\begin{{array}}{{l}}\n{latex} \\\\[-0.2em]\n{row}\n\\end{{array}}")
}

/// An equation image never holds the equation's number (that is text
/// beside it); a \tag{…} or \label{…} in a reading is the model's
/// invention, and would clash with the real number.
fn strip_numbers(t: &str) -> String {
    let mut out = String::with_capacity(t.len());
    let mut rest = t;
    // (the number as pdf2md writes it, and as it wrote it before)
    while let Some(i) = [crate::emit::NUMBER_OPEN, "\\tag*{", "\\tag{", "\\label{"].iter().filter_map(|p| rest.find(p).map(|i| (i, p.len()))).min() {
        let (i, len) = i;
        out.push_str(&rest[..i]);
        let after = &rest[i + len..];
        rest = after.find('}').map_or("", |j| &after[j + 1..]);
    }
    out.push_str(rest);
    out.trim().to_string()
}

/// A table of one cell ("\begin{array}{l}{{\frac{1}{2}}}\end{array}", as
/// the model sometimes writes a small fraction) is just what is in it.
fn unwrap_cell(t: &str) -> &str {
    let inner = t.strip_prefix("\\begin{array}{").and_then(|r| r.split_once('}')).and_then(|(cols, r)| {
        (cols.len() == 1).then_some(r)?.strip_suffix("\\end{array}")
    });
    match inner {
        Some(cell) if !cell.contains("\\\\") && !cell.contains('&') && !cell.contains("\\begin{") => cell.trim(),
        _ => t,
    }
}

/// Commands that stand between the two sides of an equation, like "=".
const RELATIONS: [&str; 16] = [
    "approx", "simeq", "equiv", "sim", "cong", "propto", "neq", "ne", "leq", "geq", "le", "ge", "to", "rightarrow",
    "Rightarrow", "longrightarrow",
];
/// Commands a continued right-hand side starts with, like "+".
const OPERATORS: [&str; 5] = ["times", "cdot", "pm", "mp", "cdots"];

/// Where the first "=" (or other relation) outside all brackets is.
fn top_relation(row: &str) -> Option<usize> {
    let b = row.as_bytes();
    let (mut depth, mut i) = (0i32, 0);
    while i < b.len() {
        match b[i] {
            b'\\' => {
                let n = b[i + 1..].iter().take_while(|c| c.is_ascii_alphabetic()).count();
                let name = &row[i + 1..i + 1 + n];
                match name {
                    "left" | "begin" => depth += 1,
                    "right" | "end" => depth = (depth - 1).max(0),
                    _ if depth == 0 && RELATIONS.contains(&name) => return Some(i),
                    _ => {}
                }
                if matches!(name, "left" | "right") {
                    // the bracket after it is counted by the command
                    i += 1 + n;
                    while i < b.len() && b[i] == b' ' {
                        i += 1;
                    }
                    i += if b.get(i) == Some(&b'\\') { 2 } else { 1 };
                    continue;
                }
                // "\\{", "\\,": two characters
                i += 1 + n.max(1);
                continue;
            }
            b'{' | b'(' | b'[' => depth += 1,
            b'}' | b')' | b']' => depth = (depth - 1).max(0),
            b'=' if depth == 0 => return Some(i),
            _ => {}
        }
        i += 1;
    }
    None
}

/// Does the row go on with the equation of the row above: with a relation
/// ("= …", 1) or inside its right-hand side ("+ …", "× …", 2)?
fn continues(row: &str) -> u8 {
    if top_relation(row) == Some(0) {
        return 1;
    }
    let name: String = row.strip_prefix('\\').unwrap_or("").chars().take_while(char::is_ascii_alphabetic).collect();
    if row.starts_with(['+', '-']) || OPERATORS.contains(&name.as_str()) {
        2
    } else {
        0
    }
}

/// An equation read line by line (see `math_ocr.py`): its rows lined up
/// at the "=", as in the book. A row that goes on with "=" has it under
/// the "=" of the first row, a row that goes on with "+" or "×" starts to
/// the right of it, and a second equation has its own "=" there. When the
/// first row has no "=" (a long left-hand side), the rows that go on are
/// set in a little instead.
pub(crate) fn align_rows(t: &str) -> String {
    if let Some(body) = t.strip_prefix("\\begin{aligned}\n& ").and_then(|b| b.strip_suffix("\n\\end{aligned}")) {
        let rows: Vec<&str> = body.split(" \\\\\n& ").collect();
        return aligned(&rows);
    }
    // the model's own table of rows: see `array_rows`
    match array_rows(t) {
        Some(rows) => aligned(&rows.iter().map(String::as_str).collect::<Vec<_>>()),
        None => t.to_string(),
    }
}

fn aligned(rows: &[&str]) -> String {
    let lined_up = top_relation(rows[0]).is_some_and(|i| i > 0);
    let rows: Vec<String> = rows
        .iter()
        .map(|row| match (continues(row), top_relation(row)) {
            (1, _) if lined_up => format!("&{row}"),
            (2, _) if lined_up => format!("&\\quad {row}"),
            (0, Some(i)) if lined_up => format!("{}&{}", &row[..i], &row[i..]),
            (0, _) => format!("& {row}"),
            _ => format!("&\\quad {row}"),
        })
        .collect();
    format!("\\begin{{aligned}}\n{}\n\\end{{aligned}}", rows.join(" \\\\\n"))
}

/// The pieces of `t` between the separators (`\\` or `&`) that stand
/// outside all braces and inner environments.
fn split_top<'a>(t: &'a str, sep: &str) -> Vec<&'a str> {
    let b = t.as_bytes();
    let (mut depth, mut i, mut start) = (0i32, 0, 0);
    let mut out = Vec::new();
    while i < b.len() {
        if depth == 0 && t[i..].starts_with(sep) {
            out.push(&t[start..i]);
            i += sep.len();
            start = i;
            continue;
        }
        match b[i] {
            b'\\' => {
                if t[i..].starts_with("\\begin{") {
                    depth += 1;
                } else if t[i..].starts_with("\\end{") {
                    depth -= 1;
                }
                // "\\{", "\\&", "\\\\" are two characters
                i += 2;
                continue;
            }
            b'{' => depth += 1,
            b'}' => depth -= 1,
            _ => {}
        }
        i += 1;
    }
    out.push(&t[start.min(t.len())..]);
    out
}

/// What a cell holds, without the braces the model wraps it in ("{{x}}")
/// and without an empty group at its start ("{}=").
fn cell(t: &str) -> &str {
    let mut t = t.trim();
    loop {
        let wrapped = t.starts_with('{') && t.ends_with('}') && t.len() >= 2 && {
            // the first brace closes at the end, not before
            let (mut d, mut closes_at_end) = (0, true);
            let b = t.as_bytes();
            let mut i = 0;
            while i < b.len() {
                match b[i] {
                    b'\\' => i += 1,
                    b'{' => d += 1,
                    b'}' => {
                        d -= 1;
                        if d == 0 && i + 1 < b.len() {
                            closes_at_end = false;
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
            closes_at_end
        };
        if wrapped {
            t = t[1..t.len() - 1].trim();
        } else if let Some(r) = t.strip_prefix("{}") {
            t = r.trim();
        } else if t == "\\" || t == "\\ " {
            return "";
        } else {
            return t;
        }
    }
}

/// A reading that is one table of rows — the model writes a multi-line
/// equation as `\begin{array}{r l} … \end{array}`, often with the rows cut
/// into columns unevenly (the first row all in the left column, the second
/// in the right one), so that the "=" of the rows do not meet. The rows,
/// each as one piece, to be lined up at the "=" like rows read one by one.
/// None for a real table: columns that are not cut at the "=", or rows
/// that are no equations at all.
fn array_rows(t: &str) -> Option<Vec<String>> {
    let rest = t.trim().strip_prefix("\\begin{array}{")?;
    let (cols, rest) = rest.split_once('}')?;
    let cols: String = cols.chars().filter(|c| !c.is_whitespace()).collect();
    if !matches!(cols.as_str(), "c" | "l" | "r" | "rl" | "rcl") {
        return None;
    }
    // the table is the whole reading (but for a full stop after it)
    let end = rest.rfind("\\end{array}")?;
    let (body, after) = (&rest[..end], rest[end + "\\end{array}".len()..].trim());
    if !after.chars().all(|c| ".,;".contains(c)) || body.matches("\\begin{array}").count() != body.matches("\\end{array}").count() {
        return None;
    }
    let mut rows: Vec<String> = Vec::new();
    let mut at_relation = true;
    for row in split_top(body, "\\\\") {
        let cells: Vec<&str> = split_top(row, "&").into_iter().map(cell).collect();
        // columns cut somewhere else than before a "=": a table
        // (in "a & = & b" the "=" has a column of its own)
        let lone_relation = |c: &str| continues(c) == 1 && c.len() <= 8;
        if (1..cells.len()).any(|k| !cells[k].is_empty() && continues(cells[k]) == 0 && !lone_relation(cells[k - 1])) {
            at_relation = false;
        }
        let row = cells.into_iter().filter(|c| !c.is_empty()).collect::<Vec<_>>().join(" ");
        if !row.is_empty() {
            rows.push(row);
        }
    }
    if let Some(last) = rows.last_mut() {
        last.push_str(after);
    }
    let equations = rows.iter().any(|r| top_relation(r).is_some());
    (rows.len() >= 2 && at_relation && equations).then_some(rows)
}

/// Every command KaTeX knows (taken from its source), one per line.
const LATEX_COMMANDS: &str = include_str!("latex_commands.txt");

/// A command glued to the letters after it ("\\cdotb" for "\\cdot b"): the
/// model sometimes leaves out the space. When a command name is no command
/// but starts with one, followed by letters, a space goes between them.
fn unglue(t: &str) -> String {
    use std::sync::OnceLock;
    static KNOWN: OnceLock<std::collections::HashSet<&'static str>> = OnceLock::new();
    let known = KNOWN.get_or_init(|| LATEX_COMMANDS.lines().map(str::trim).filter(|l| !l.is_empty()).collect());
    let mut out = String::with_capacity(t.len() + 8);
    let mut rest = t;
    while let Some(i) = rest.find('\\') {
        out.push_str(&rest[..i]);
        let after = &rest[i + 1..];
        let name_len = after.find(|c: char| !c.is_ascii_alphabetic()).unwrap_or(after.len());
        let name = &after[..name_len];
        if name_len == 0 {
            // "\\\\", "\\{", "\\,": not a name
            let c = after.chars().next().map_or(0, char::len_utf8);
            out.push_str(&rest[i..i + 1 + c]);
            rest = &after[c..];
            continue;
        }
        let fixed = if known.contains(name) {
            None
        } else {
            // the longest command it starts with
            (1..name.len()).rev().map(|k| &name[..k]).find(|p| known.contains(p))
        };
        match fixed {
            Some(p) => out.push_str(&format!("\\{p} {}", &name[p.len()..])),
            None => out.push_str(&format!("\\{name}")),
        }
        rest = &after[name_len..];
    }
    out.push_str(rest);
    out
}

/// An image narrower than this (pixels) is a single symbol; the
/// recognizer reads it with white space added around it (`math_ocr.py`).
const SMALL: u32 = 30;

/// Read every image (given as base64 PNG), from the cache or by running
/// the recognizer once for all new ones. Images it could not read are
/// missing from the answer.
pub fn recognize(images: &[&str]) -> HashMap<String, Reading> {
    let mut out = HashMap::new();
    let Some(dir) = cache_dir() else { return out };
    let mut todo: Vec<(String, &str)> = Vec::new();
    let (mut second, mut untested) = (0, 0);
    for &img in images {
        let k = key(img);
        if out.contains_key(&k) || todo.iter().any(|(t, _)| *t == k) {
            continue;
        }
        // the reader's own correction comes first
        if let Some(c) = read_correction(&k) {
            out.insert(k, Reading::Latex(c));
            continue;
        }
        match read_cached(&dir, &k) {
            Some(r) => {
                // math read before the second reading existed, before the
                // tests that tell pictures from math, before lines read as
                // one were taken apart, or (a very small image) before such
                // images were read with a margin: check it now
                let math = matches!(r, Reading::Latex(_));
                let unverified = math && !dir.join(format!("{k}.verify")).exists();
                let small = png_size(img).is_some_and(|(w, _)| w < SMALL);
                let unsorted = math
                    && ([".kind", ".lines", ".labels"].iter().any(|e| !dir.join(format!("{k}{e}")).exists())
                        || (small && !dir.join(format!("{k}.small")).exists()));
                out.insert(k.clone(), r);
                if unverified || unsorted {
                    second += unverified as usize;
                    untested += (unsorted && !unverified) as usize;
                    todo.push((k, img));
                }
            }
            None => todo.push((k, img)),
        }
    }
    if todo.is_empty() {
        return out;
    }
    let Some(py) = python() else { return out };
    let pending = dir.join("pending");
    if std::fs::create_dir_all(&pending).is_err() {
        return out;
    }
    for (k, img) in &todo {
        if let Some(bytes) = base64_decode(img) {
            let _ = std::fs::write(pending.join(format!("{k}.png")), bytes);
        }
    }
    let script = dir.join("math_ocr.py");
    let _ = std::fs::write(&script, SCRIPT);
    let new = todo.iter().filter(|(k, _)| !out.contains_key(k)).count();
    if new > 0 {
        eprintln!("Reading {new} equation images with the math recognizer…");
    }
    if second > 0 {
        eprintln!("Reading {second} equations a second time, to list the doubtful ones…");
    }
    if untested > 0 {
        eprintln!("Checking {untested} images read as math before: diagrams or logos? lines run together? single symbols misread?");
    }
    if new == 0 {
        // only checks of earlier readings: nothing is read for the first time
        for step in [Step::Words, Step::Formulas, Step::Again] {
            crate::progress::skip(step);
        }
    }
    // (the recognizer says how far it is from here on; it starts with this)
    crate::progress::report(Step::Sorting, 0, todo.len());
    let ok = run_recognizer(Command::new(&py).arg(&script).arg(&pending).arg(&dir));
    if !ok {
        eprintln!("The math recognizer failed; those equations stay pictures.");
    }
    for (k, _) in &todo {
        let _ = std::fs::remove_file(pending.join(format!("{k}.png")));
        if let Some(r) = read_cached(&dir, k) {
            out.insert(k.clone(), r);
        }
    }
    out
}

/// What the math recognizer read an equation as when it read it a second
/// time (see [`second_readings`]), if it did.
pub fn second_reading(k: &str) -> Option<String> {
    let t = std::fs::read_to_string(cache_dir()?.join(format!("{k}.second"))).ok()?;
    let t = t.trim();
    (!t.is_empty() && !t.starts_with('%')).then(|| t.to_string())
}

/// A second reading of equations that something else read (the page
/// reader of scanned books), to tell which are worth checking against the
/// book: each `(key, LaTeX as read, picture of the equation as PNG)` is
/// read by the math recognizer from its picture, and where that reading
/// is another formula, [`needs_check`] says so for the key from then on.
/// Without the recognizer nothing is said about any of them.
pub fn second_readings(equations: &[(String, String, Vec<u8>)]) {
    let (Some(dir), Some(py)) = (cache_dir(), python()) else { return };
    let pending = dir.join("pending");
    if std::fs::create_dir_all(&pending).is_err() {
        return;
    }
    let mut todo = Vec::new();
    for (k, latex, png) in equations {
        // (read a second time before, and the same formula as then)
        let same = std::fs::read_to_string(dir.join(format!("{k}.tex"))).is_ok_and(|t| t.trim() == latex.trim());
        if same && dir.join(format!("{k}.verify")).exists() {
            continue;
        }
        let _ = std::fs::write(dir.join(format!("{k}.tex")), format!("{latex}\n"));
        // (not to be read again in lines, or without words under it:
        // only compared)
        for mark in ["kind", "lines", "labels", "small"] {
            let _ = std::fs::write(dir.join(format!("{k}.{mark}")), "\n");
        }
        for old in ["verify", "second"] {
            let _ = std::fs::remove_file(dir.join(format!("{k}.{old}")));
        }
        if std::fs::write(pending.join(format!("{k}.png")), png).is_ok() {
            todo.push(k);
        }
    }
    if todo.is_empty() {
        return;
    }
    let script = dir.join("math_ocr.py");
    let _ = std::fs::write(&script, SCRIPT);
    eprintln!("Reading {} equations a second time, to list the doubtful ones…", todo.len());
    for step in [Step::Words, Step::Formulas, Step::Again, Step::Lines, Step::Labels] {
        crate::progress::skip(step);
    }
    crate::progress::report(Step::Sorting, 0, todo.len());
    if !run_recognizer(Command::new(&py).arg(&script).arg(&pending).arg(&dir)) {
        eprintln!("The math recognizer failed; the equations are not compared.");
    }
    for k in todo {
        let _ = std::fs::remove_file(pending.join(format!("{k}.png")));
    }
}

const TABLE_SCRIPT: &str = include_str!("table_ocr.py");

/// Tables that the book holds as pictures, read cell by cell. Read whole,
/// as one formula, such a table comes out with cells in the wrong rows
/// and columns (see `table_ocr.py`); here each table that stands over its
/// picture is cut from the PDF once more, sharper, and its rows and
/// columns are found on the picture itself. A table that does not part
/// into rows and columns there, or cannot be read (no recognizer), stays
/// as it was read whole. A reading is kept, under the table's key.
pub fn read_picture_tables(book: &mut crate::Book, pdf: &std::path::Path) {
    use crate::Block;
    let (Some(dir), Some(py)) = (cache_dir(), python()) else { return };
    // (chapter, block, key, page, area) of each table over its picture
    let mut tables = Vec::new();
    for (c, ch) in book.chapters.iter().enumerate() {
        for (b, pair) in ch.blocks.windows(2).enumerate() {
            match (&pair[0].block, &pair[1].block) {
                (Block::Table { key, .. }, Block::Picture { page, area, alt }) if alt == crate::TABLE_READ && !key.is_empty() => {
                    tables.push((c, b, key.clone(), *page, *area));
                }
                _ => {}
            }
        }
    }
    let todo: Vec<_> = tables.iter().filter(|t| !dir.join(format!("{}.cells", t.2)).exists()).collect();
    if !todo.is_empty() {
        let pending = dir.join("tables");
        if std::fs::create_dir_all(&pending).is_err() {
            return;
        }
        for (_, _, key, page, area) in &todo {
            // pdftohtml pixels are 1.5 × points; 300 dpi is 300/72 px per point
            let px = |v: f32| (v / 1.5 * 300.0 / 72.0).round().max(0.0) as i64;
            let pad = 4.0;
            let _ = Command::new("pdftoppm")
                .args(["-png", "-singlefile", "-r", "300"])
                .args(["-f", &page.to_string(), "-l", &page.to_string()])
                .args(["-x", &px(area.left - pad).to_string(), "-y", &px(area.top - pad).to_string()])
                .args(["-W", &px(area.right - area.left + 2.0 * pad).to_string(), "-H", &px(area.bottom - area.top + 2.0 * pad).to_string()])
                .arg(pdf)
                .arg(pending.join(key))
                .status();
        }
        // (the reader of cells uses the recognizer's own reading of one picture)
        let _ = std::fs::write(dir.join("math_ocr.py"), SCRIPT);
        let script = dir.join("table_ocr.py");
        let _ = std::fs::write(&script, TABLE_SCRIPT);
        eprintln!("Reading {} tables cell by cell…", todo.len());
        if !run_recognizer(Command::new(&py).arg(&script).arg(&pending).arg(&dir)) {
            eprintln!("The tables could not be read cell by cell; they stay as read whole.");
        }
        for (_, _, key, _, _) in &todo {
            let _ = std::fs::remove_file(pending.join(format!("{key}.png")));
            // (not looked at again if it is no table of rows and columns)
            let cells = dir.join(format!("{key}.cells"));
            if !cells.exists() {
                let _ = std::fs::write(cells, "");
            }
        }
    }
    for (c, b, key, _, _) in tables {
        let Ok(text) = std::fs::read_to_string(dir.join(format!("{key}.cells"))) else { continue };
        let mut read: Vec<Vec<crate::Cell>> = text.lines().map(|l| l.split('\t').map(crate::blocks::table_cell).collect()).collect();
        read.retain(|r: &Vec<crate::Cell>| r.iter().any(|cell| !cell.text.is_empty()));
        let columns = read.iter().map(Vec::len).max().unwrap_or(0);
        if read.len() < 2 || columns < 2 {
            continue;
        }
        for r in read.iter_mut() {
            r.resize_with(columns, || crate::Cell { span: 1, ..Default::default() });
        }
        if let Block::Table { rows, .. } = &mut book.chapters[c].blocks[b].block {
            *rows = read;
        }
    }
}

/// Run the recognizer, passing on what it says and reporting how far it
/// is: it prints lines like "  reading 16/420", with one word for each of
/// its steps.
fn run_recognizer(cmd: &mut Command) -> bool {
    use std::io::BufRead;
    let Ok(mut child) = cmd.stderr(std::process::Stdio::piped()).spawn() else { return false };
    if let Some(pipe) = child.stderr.take() {
        for line in std::io::BufReader::new(pipe).lines().map_while(Result::ok) {
            let count = [
                ("sorting ", Step::Sorting),
                ("text ", Step::Words),
                ("loading ", Step::Model),
                ("reading ", Step::Formulas),
                ("again ", Step::Again),
                ("lines ", Step::Lines),
                ("labels ", Step::Labels),
                ("checking ", Step::Check),
            ]
            .iter()
            .find_map(|(word, step)| {
                let (done, total) = line.trim().strip_prefix(word)?.split_once('/')?;
                Some((*step, done.parse().ok()?, total.parse().ok()?))
            });
            eprintln!("{line}");
            if let Some((step, done, total)) = count {
                crate::progress::report(step, done, total);
            }
        }
    }
    child.wait().is_ok_and(|s| s.success())
}

/// Was the equation of this image read as a different formula a second
/// time (and not corrected by hand since)? Then it is worth checking
/// against the book: how many symbols the two readings differ in.
pub fn needs_check(k: &str) -> Option<usize> {
    if read_correction(k).is_some() {
        return None;
    }
    let v = std::fs::read_to_string(cache_dir()?.join(format!("{k}.verify"))).ok()?;
    let n = v.trim().strip_prefix("check")?.trim();
    Some(n.parse().unwrap_or(1))
}

/// Width and height of a PNG, from its header.
pub fn png_size(png_base64: &str) -> Option<(u32, u32)> {
    let head = base64_decode(png_base64.get(..32)?)?;
    let be = |b: &[u8]| u32::from_be_bytes([b[0], b[1], b[2], b[3]]);
    (head.get(12..16)? == b"IHDR").then(|| (be(&head[16..20]), be(&head[20..24])))
}

/// A horizontal line drawn as an image (a few pixels high, many wide): not
/// math. Given to the recognizer it is a gray smear, from which texify
/// invents text.
pub fn is_rule(png_base64: &str) -> bool {
    png_size(png_base64).is_some_and(|(w, h)| h <= 3 && w >= 40)
}

pub fn base64_decode(s: &str) -> Option<Vec<u8>> {
    let val = |c: u8| -> Option<u32> {
        Some(match c {
            b'A'..=b'Z' => c - b'A',
            b'a'..=b'z' => c - b'a' + 26,
            b'0'..=b'9' => c - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            _ => return None,
        } as u32)
    };
    let bytes: Vec<u8> = s.bytes().filter(|c| !c.is_ascii_whitespace() && *c != b'=').collect();
    let mut out = Vec::with_capacity(bytes.len() * 3 / 4);
    for chunk in bytes.chunks(4) {
        let mut n = 0u32;
        for (i, &c) in chunk.iter().enumerate() {
            n |= val(c)? << (18 - 6 * i);
        }
        let k = chunk.len();
        if k < 2 {
            return None;
        }
        out.push((n >> 16) as u8);
        if k > 2 {
            out.push((n >> 8) as u8);
        }
        if k > 3 {
            out.push(n as u8);
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marked_equations_are_read_in_both_forms() {
        // as written now, and as written before (a book converted earlier)
        let k = "0123456789abcdef";
        let now = format!("<!-- pdf2md:eq {k} -->\n\n```math\na \\\\ \nb \\qquad \\text{{(1)}}\n```\n\nSee $`x^{{*}}`$<!-- pdf2md:eq {k} --> here, and $`y`$ too.");
        let before = format!("<!-- pdf2md:eq {k} -->\n$$\na \\\\\nb \\tag{{1}}\n$$\n\nSee <!-- pdf2md:eq {k} -->$x^{{*}}$ here.");
        for text in [now, before] {
            let found: Vec<String> = marked_equations(&text).into_iter().map(|(_, l)| norm(&l)).collect();
            assert_eq!(found, ["a \\\\ b", "x^{*}"], "{text}");
        }
    }

    #[test]
    fn how_the_picture_under_a_table_stands_is_found() {
        let table = |k: &str, after: &str| format!("<!-- pdf2md:table {k} -->\n<table>\n<tr>\n<td>\n\na\n\n</td>\n</tr>\n</table>\n\n{after}\n\n");
        let md = table("0123456789abcdef", "<details open>\n<summary>The table as printed</summary>\n\n![x](images/a.png)\n\n</details>")
            + &table("1123456789abcdef", "<details>\n<summary>The table as printed</summary>\n\n![x](images/b.png)\n\n</details>")
            + &table("2123456789abcdef", "Text goes on.");
        let states: Vec<(String, String)> = marked_table_pictures(&md);
        let want = [("0123456789abcdef.picture", "open"), ("1123456789abcdef.picture", "closed"), ("2123456789abcdef.picture", "removed")];
        assert_eq!(states, want.iter().map(|(k, s)| (k.to_string(), s.to_string())).collect::<Vec<_>>());
    }

    #[test]
    fn the_cells_of_a_marked_table_are_found() {
        let md = "Text.\n\n<!-- pdf2md:table 0123456789abcdef -->\n<table>\n<tr>\n<td></td>\n<td align=\"center\">\n\nWKB\n\n</td>\n</tr>\n<tr>\n<td>\n\n$n$\n\n</td>\n<td colspan=\"2\">\n\n2.320 and\nmore\n\n</td>\n</tr>\n</table>\n\n<table>\n<tr><td>\n\nno key\n\n</td></tr>\n</table>\n";
        let cells = marked_tables(md);
        assert_eq!(
            cells,
            [
                ("0123456789abcdef.0.0".to_string(), "".to_string()),
                ("0123456789abcdef.0.1".to_string(), "WKB".to_string()),
                ("0123456789abcdef.1.0".to_string(), "$n$".to_string()),
                ("0123456789abcdef.1.1".to_string(), "2.320 and\nmore".to_string()),
            ]
        );
    }

    #[test]
    fn lines_rows_up_at_the_equals_sign() {
        let block = |rows: &[&str]| format!("\\begin{{aligned}}\n{}\n\\end{{aligned}}", rows.join(" \\\\\n"));
        let read = |rows: &[&str]| format!("\\begin{{aligned}}\n& {}\n\\end{{aligned}}", rows.join(" \\\\\n& "));
        // a continued equation
        assert_eq!(
            align_rows(&read(&["a=b+c", "=d", "+e", "\\approx f"])),
            block(&["a&=b+c", "&=d", "&\\quad +e", "&\\approx f"])
        );
        // two equations; an "=" inside brackets is not the place
        assert_eq!(
            align_rows(&read(&["f(x=0)=1,", "\\left(a=b\\right)g\\equiv 2."])),
            block(&["f(x=0)&=1,", "\\left(a=b\\right)g&\\equiv 2."])
        );
        // no "=" in the first row: left-aligned, what goes on set in
        assert_eq!(align_rows(&read(&["\\int f\\,dx", "=1."])), block(&["& \\int f\\,dx", "&\\quad =1."]));
        assert_eq!(unwrap_cell("\\begin{array}{l}{{\\frac{1}{2}}}\\end{array}"), "{{\\frac{1}{2}}}");
        assert_eq!(unwrap_cell("\\begin{array}{l}a\\\\ b\\end{array}"), "\\begin{array}{l}a\\\\ b\\end{array}");
        // the model's own table, its rows cut into columns unevenly
        assert_eq!(
            align_rows("\\begin{array}{r l}{{\\hbar}=1.05\\times10^{-27}\\,\\mathrm{erg}}\\\\ {\\ }&{{}=6.58\\times10^{-16}\\,\\mathrm{eV}.}\\end{array}"),
            block(&["{\\hbar}&=1.05\\times10^{-27}\\,\\mathrm{erg}", "&=6.58\\times10^{-16}\\,\\mathrm{eV}."])
        );
        // a real table stays
        let table = "\\begin{array}{r l}{V=\\infty}&{\\mathrm{for~}x<0,}\\\\ {V=0}&{\\mathrm{for~}x>0}\\end{array}";
        assert_eq!(align_rows(table), table);
        // not read line by line: untouched
        assert_eq!(align_rows("a=b"), "a=b");
    }

    #[test]
    fn unglues_commands() {
        assert_eq!(unglue("a\\cdotb"), "a\\cdot b");
        assert_eq!(unglue("a\\cdot\\!b"), "a\\cdot\\!b");
        assert_eq!(unglue("x\\leftarrow y"), "x\\leftarrow y");
        assert_eq!(unglue("\\left(\\alpha\\right)"), "\\left(\\alpha\\right)");
        assert_eq!(unglue("\\alphab+\\varepsilon"), "\\alpha b+\\varepsilon");
        assert_eq!(unglue("a \\\\ b \\, c"), "a \\\\ b \\, c");
        assert_eq!(unglue("\\frac{1}{2}"), "\\frac{1}{2}");
    }

    #[test]
    fn strips_invented_numbers() {
        assert_eq!(strip_numbers("a=b \\tag{1}"), "a=b");
        assert_eq!(strip_numbers("a=b \\qquad \\text{(1.2a)}"), "a=b");
        assert_eq!(strip_numbers("\\label{x} a \\\\ b \\tag*{2}"), "a \\\\ b");
        assert_eq!(strip_numbers("x^{2}"), "x^{2}");
    }

    #[test]
    fn tells_rules_from_math() {
        // PNG headers: 364×1 (a rule), 8×1 (a minus sign), 364×40 (an equation)
        assert_eq!(png_size("iVBORw0KGgoAAAANSUhEUgAAAWwAAAABCAAAAAAAAAAA"), Some((364, 1)));
        assert!(is_rule("iVBORw0KGgoAAAANSUhEUgAAAWwAAAABCAAAAAAAAAAA"));
        assert!(!is_rule("iVBORw0KGgoAAAANSUhEUgAAAAgAAAABCAAAAAAAAAAA"));
        assert!(!is_rule("iVBORw0KGgoAAAANSUhEUgAAAWwAAAAoCAAAAAAAAAAA"));
    }

    #[test]
    fn decodes_base64() {
        assert_eq!(base64_decode("aGVsbG8=").unwrap(), b"hello");
        assert_eq!(base64_decode("aGVsbG8hIQ==").unwrap(), b"hello!!");
        assert_eq!(base64_decode("YWJj").unwrap(), b"abc");
    }

    #[test]
    fn words_under_an_equation_stand_where_they_stood() {
        // "x y" is about 1.1 em wide: a word under its middle is moved half of that
        let t = with_labels("x y", &[(0.5, "bra".into())]);
        assert!(t.starts_with("\\begin{array}{l}\nx y \\\\[-0.2em]\n\\mathrlap{\\kern0.5"), "{t}");
        assert!(t.contains("\\mathclap{\\scriptsize\\text{bra}}"));
        // none, or an equation of several rows: as it is
        assert_eq!(with_labels("x y", &[]), "x y");
        assert_eq!(with_labels("a \\\\ b", &[(0.5, "bra".into())]), "a \\\\ b");
    }
}
