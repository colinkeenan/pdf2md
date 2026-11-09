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
        Some(Reading::Latex(t.to_string()))
    }
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
        match read_cached(&dir, &k) {
            Some(r) => {
                out.insert(k, r);
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
    eprintln!("Reading {} equation images with the math recognizer…", todo.len());
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
    fn decodes_base64() {
        assert_eq!(base64_decode("aGVsbG8=").unwrap(), b"hello");
        assert_eq!(base64_decode("aGVsbG8hIQ==").unwrap(), b"hello!!");
        assert_eq!(base64_decode("YWJj").unwrap(), b"abc");
    }
}
