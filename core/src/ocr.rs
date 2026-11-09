//! Equations that are pictures. Ebook-made PDFs (calibre, Kindle) carry
//! their math as small raster images instead of text. Each image is handed
//! to a local math-recognition model (`math_ocr.py`, run with the Python of
//! its own environment) that answers with LaTeX, or says the image is a
//! drawing, or that it cannot read it (such images stay pictures). Answers
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
    /// a plot or diagram, not math
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
        Some(Reading::Latex(unglue(&strip_numbers(t))))
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

/// Read every image (given as base64 PNG), from the cache or by running
/// the recognizer once for all new ones. Images it could not read are
/// missing from the answer.
pub fn recognize(images: &[&str]) -> HashMap<String, Reading> {
    let mut out = HashMap::new();
    let Some(dir) = cache_dir() else { return out };
    let mut todo: Vec<(String, &str)> = Vec::new();
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
                // math read before the second reading existed: check it now
                let unverified = matches!(r, Reading::Latex(_)) && !dir.join(format!("{k}.verify")).exists();
                out.insert(k.clone(), r);
                if unverified {
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
    if todo.len() > new {
        eprintln!("Reading {} equations a second time, to list the doubtful ones…", todo.len() - new);
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
    fn decodes_base64() {
        assert_eq!(base64_decode("aGVsbG8=").unwrap(), b"hello");
        assert_eq!(base64_decode("aGVsbG8hIQ==").unwrap(), b"hello!!");
        assert_eq!(base64_decode("YWJj").unwrap(), b"abc");
    }
}
