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

fn read_correction(k: &str) -> Option<String> {
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
    if found.is_empty() || std::fs::create_dir_all(&cdir).is_err() {
        return 0;
    }
    found.iter().filter(|(k, (_, latex))| std::fs::write(cdir.join(format!("{k}.tex")), format!("{latex}\n")).is_ok()).count()
}

fn norm(t: &str) -> String {
    t.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The marked equations of a Markdown text: picture key and LaTeX (without
/// its number), for displayed ones (`<!-- pdf2md:eq KEY -->` above `$$`)
/// and ones inside sentences (the marker right before `$…$`).
pub fn marked_equations(text: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut rest = text;
    while let Some(i) = rest.find("<!-- pdf2md:eq ") {
        rest = &rest[i + 15..];
        let Some(k) = rest.get(..16).filter(|k| k.chars().all(|c| c.is_ascii_hexdigit())) else { continue };
        let k = k.to_string();
        let after = rest.get(20..).unwrap_or("");
        let latex = if let Some(inline) = after.strip_prefix('$') {
            let Some(end) = inline.find('$') else { break };
            let latex = strip_numbers(&inline[..end]);
            rest = &inline[end..];
            latex
        } else {
            let Some(start) = rest.find("$$\n") else { break };
            let body = &rest[start + 3..];
            let Some(end) = body.find("\n$$") else { break };
            let latex = strip_numbers(&body[..end]);
            rest = &body[end..];
            latex
        };
        out.push((k, latex));
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
        Some(Reading::Latex(align_rows(&unglue(unwrap_cell(&strip_numbers(t))))))
    }
}

/// An equation image never holds the equation's number (that is text
/// beside it); a \tag{…} or \label{…} in a reading is the model's
/// invention, and would clash with the real number.
fn strip_numbers(t: &str) -> String {
    let mut out = String::with_capacity(t.len());
    let mut rest = t;
    while let Some(i) = ["\\tag*{", "\\tag{", "\\label{"].iter().filter_map(|p| rest.find(p).map(|i| (i, p.len()))).min() {
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
fn align_rows(t: &str) -> String {
    let Some(body) = t.strip_prefix("\\begin{aligned}\n& ").and_then(|b| b.strip_suffix("\n\\end{aligned}")) else {
        return t.to_string();
    };
    let rows: Vec<&str> = body.split(" \\\\\n& ").collect();
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
                    && ([".kind", ".lines"].iter().any(|e| !dir.join(format!("{k}{e}")).exists())
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
    let ok = Command::new(&py).arg(&script).arg(&pending).arg(&dir).status().is_ok_and(|s| s.success());
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
}
