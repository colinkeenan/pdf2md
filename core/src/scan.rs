//! Scanned books: PDFs whose pages are pictures, with no text to take.
//!
//! Such a book is read by marker (a page-reading model, run on this
//! machine as a separate program; see `scan_read.py`), which gives each
//! page as blocks: paragraphs with their formulas as LaTeX, headings,
//! displayed equations, footnotes, and where the pictures are. Here those
//! blocks become the blocks of this crate, so that chapters, page markers,
//! notes, links and the files are made as for any other book.
//!
//! marker and its models have licences of their own, which bind whoever
//! runs them: see the folder `licenses/marker` and the README.

use crate::layout::{Doc, Figure};
use crate::progress::{self, Step};
use crate::{Blk, Block, Inline, TocEntry};
use std::path::{Path, PathBuf};
use std::process::Command;

const SCRIPT: &str = include_str!("scan_read.py");
/// marker's points to pdftohtml's pixels.
const PX: f32 = 1.5;

/// The type size given to ordinary text, in pixels (a guess: sizes only
/// matter against each other here).
const BODY: f32 = 15.0;

fn data_dir() -> Option<PathBuf> {
    Some(PathBuf::from(std::env::var_os("HOME")?).join(".local/share/pdf2md/scan"))
}

/// The Python that has marker installed: `PDF2MD_SCAN_PYTHON`, or the
/// environment set up as the README describes.
fn python() -> Option<PathBuf> {
    if let Some(p) = std::env::var_os("PDF2MD_SCAN_PYTHON") {
        return Some(PathBuf::from(p));
    }
    let p = data_dir()?.join(".venv/bin/python");
    p.exists().then_some(p)
}

/// Is this PDF a scan? `doc` is what `pdftohtml` gave: next to no text,
/// while nearly every page holds an image.
pub fn looks_scanned(doc: &Doc, path: &Path, pages: Option<(u32, u32)>) -> bool {
    let n = doc.pages.len();
    let letters: usize = doc.pages.iter().flat_map(|p| &p.lines).flat_map(|l| &l.spans).map(|s| s.text.chars().filter(|c| c.is_alphabetic()).count()).sum();
    if n == 0 || letters > 100 * n {
        return false;
    }
    let mut cmd = Command::new("pdfimages");
    cmd.arg("-list");
    if let Some((first, last)) = pages {
        cmd.args(["-f", &first.to_string(), "-l", &last.to_string()]);
    }
    let Ok(out) = cmd.arg(path).output() else { return false };
    // (the list: two lines of heading, then a line for each image that
    // starts with its page)
    let with_image: std::collections::HashSet<u32> =
        String::from_utf8_lossy(&out.stdout).lines().skip(2).filter_map(|l| l.split_whitespace().next()?.parse().ok()).collect();
    with_image.len() * 5 >= n * 4
}

fn fnv(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325u64, |h, b| (h ^ *b as u64).wrapping_mul(0x100000001b3))
}

/// Where the pages of this PDF are kept once read (by what the file holds,
/// so a renamed or moved book is not read again).
fn cache_dir(path: &Path) -> Option<PathBuf> {
    let base = std::env::var_os("XDG_CACHE_HOME").map(PathBuf::from).or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".cache")))?;
    let bytes = std::fs::read(path).ok()?;
    Some(base.join("pdf2md").join("scan").join(format!("{:016x}", fnv(&bytes))))
}

/// A block of a page as marker gave it.
#[derive(Debug, Clone, Default)]
struct Raw {
    kind: String,
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
    /// text lines in it (0: not known)
    lines: usize,
    html: String,
    /// a table's text, line by line
    text: Vec<String>,
}

#[derive(Debug, Clone, Default)]
struct PageRead {
    /// the PDF's page number (1 = first sheet)
    number: u32,
    width: f32,
    height: f32,
    blocks: Vec<Raw>,
}

fn parse_page(number: u32, text: &str) -> PageRead {
    let mut page = PageRead { number, ..Default::default() };
    for line in text.lines() {
        let mut f = line.splitn(8, '\t');
        match f.next() {
            Some("page") => {
                let mut size = f.map(|v| v.parse().unwrap_or(0.0));
                (page.width, page.height) = (size.next().unwrap_or(0.0), size.next().unwrap_or(0.0));
            }
            Some("block") => {
                let kind = f.next().unwrap_or("").to_string();
                let mut num = || f.next().and_then(|v| v.parse::<f32>().ok()).unwrap_or(0.0);
                let (left, top, right, bottom, lines) = (num(), num(), num(), num(), num() as usize);
                // (marker writes the raised marks of footnotes as text:
                // "&lt;sup&gt;2&lt;/sup&gt;", and at the start of a note
                // "<sup>&amp;</sup>lt;sup&gt;1&lt;/sup&gt;")
                let mut html = f.next().unwrap_or("").replace("<sup>&amp;</sup>lt;sup&gt;", "<sup>");
                for tag in ["sup", "/sup", "sub", "/sub"] {
                    html = html.replace(&format!("&lt;{tag}&gt;"), &format!("<{tag}>"));
                }
                page.blocks.push(Raw { kind, left, top, right, bottom, lines, html, text: Vec::new() });
            }
            Some("line") => {
                if let Some(b) = page.blocks.last_mut() {
                    b.text.push(line.split_once('\t').map_or("", |x| x.1).to_string());
                }
            }
            _ => {}
        }
    }
    // A running head that marker took for text ("xii Preface", at the very
    // top of the page, with the page's number): a running head.
    let top = page.blocks.iter().position(|b| b.kind != "PageHeader" && b.kind != "PageFooter");
    if let Some(b) = top.map(|k| &mut page.blocks[k]) {
        let text = crate::plain(&all_inline(&b.html));
        if b.kind == "Text" && b.bottom < 0.05 * page.height && text.split_whitespace().count() <= 12 && number_in(&text).is_some() {
            b.kind = "PageHeader".into();
        }
    }
    page
}

/// What a scanned book was read as: the blocks of [`crate::blocks::build`]
/// and what goes with them.
pub struct Scan {
    pub blks: Vec<Blk>,
    pub labels: Vec<Option<String>>,
    pub pdf_pages: Vec<u32>,
    pub chapter_size: Option<f32>,
}

/// Read the pages `first..=last` of a scanned PDF.
pub fn read(path: &Path, first: u32, last: u32) -> Result<Scan, String> {
    let total = (last + 1).saturating_sub(first) as usize;
    let Some(py) = python() else {
        return Err("this PDF is a scan (pictures of pages, no text), and the page reader for scanned books is not installed. \
                    The README says how to set it up (“Scanned books”)."
            .into());
    };
    let dir = cache_dir(path).ok_or("no folder to keep the pages read in")?;
    std::fs::create_dir_all(&dir).map_err(|e| format!("could not create {}: {e}", dir.display()))?;
    let script = dir.join("scan_read.py");
    std::fs::write(&script, SCRIPT).map_err(|e| e.to_string())?;
    progress::skip(Step::Drawings);
    let file = |n: u32| dir.join(format!("p{n:04}.txt"));
    let missing = (first..=last).filter(|n| !file(*n).exists()).count();
    if missing > 0 {
        eprintln!("Reading {missing} scanned pages with marker…");
        progress::report(Step::Scan, total - missing, total);
        if !run_reader(Command::new(&py).arg(&script).arg(path).arg(&dir).arg(first.to_string()).arg(last.to_string())) {
            return Err("the page reader for scanned books failed (its messages are on the terminal)".into());
        }
    }
    progress::report(Step::Scan, total, total);
    let mut pages = Vec::new();
    for n in first..=last {
        let text = std::fs::read_to_string(file(n)).map_err(|_| format!("page {n} was not read"))?;
        let mut page = parse_page(n, &text);
        // a heading marker saw but could not read ("Chapter VI.4" in
        // small capitals, "Index"): its words from the PDF itself
        for b in page.blocks.iter_mut().filter(|b| b.kind == "SectionHeader") {
            if !has_words(&all_inline(&b.html)) {
                let words = area_text(path, n, b);
                b.html = words.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;");
            }
        }
        pages.push(page);
    }
    // Where the lines of a block end (pages read before the reader said
    // so, and blocks it read as one line): asked of the PDF itself, for
    // blocks of several lines and for pages set centered, where every
    // line ends as the author ended it.
    let mut pages = imprints(&pages);
    for page in pages.iter_mut() {
        let all = centered_page(page);
        let number = page.number;
        for b in page.blocks.iter_mut().filter(|b| is_text(b) && b.text.is_empty()) {
            if b.lines >= 2 || (all && b.bottom - b.top > 20.0) {
                b.text = area_lines(path, number, b);
            }
        }
    }
    Ok(build(&pages, &format!("{:016x}", fnv(dir.to_string_lossy().as_bytes()))))
}

/// The words in a block's area of a page (see [`area_lines`]).
fn area_text(path: &Path, page: u32, b: &Raw) -> String {
    area_lines(path, page, b).join(" ")
}

/// The text in a block's area of a page, line by line: the text the PDF
/// carries there under the picture, else what Tesseract reads in it.
/// Empty if neither knows.
fn area_lines(path: &Path, page: u32, b: &Raw) -> Vec<String> {
    let (x, y, w, h) = (b.left.floor().max(0.0), b.top.floor().max(0.0), (b.right - b.left).ceil().max(1.0), (b.bottom - b.top).ceil().max(1.0));
    let page = page.to_string();
    let arg = |v: f32| (v as i64).to_string();
    let lines = |t: &str| -> Vec<String> { t.lines().map(|l| l.split_whitespace().collect::<Vec<_>>().join(" ")).filter(|l| !l.is_empty()).collect() };
    let hidden = Command::new("pdftotext")
        .args(["-layout", "-f", &page, "-l", &page, "-x", &arg(x), "-y", &arg(y), "-W", &arg(w), "-H", &arg(h)])
        .arg(path)
        .arg("-")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
        .unwrap_or_default();
    if hidden.chars().any(char::is_alphanumeric) {
        return lines(&hidden);
    }
    // (cut out at 300 dots per inch, which pdftoppm counts its pixels in)
    let k = 300.0 / 72.0;
    let cut = Command::new("pdftoppm")
        .args(["-f", &page, "-l", &page, "-r", "300", "-gray", "-png", "-x", &arg(x * k), "-y", &arg(y * k), "-W", &arg(w * k), "-H", &arg(h * k)])
        .arg(path)
        .output();
    let Ok(cut) = cut else { return Vec::new() };
    let read = Command::new("tesseract").args(["-", "-", "--psm", "6"]).env("OMP_THREAD_LIMIT", "1").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::null()).spawn();
    let Ok(mut read) = read else { return Vec::new() };
    if let Some(mut pipe) = read.stdin.take() {
        use std::io::Write;
        let _ = pipe.write_all(&cut.stdout);
    }
    read.wait_with_output().map(|o| lines(&String::from_utf8_lossy(&o.stdout))).unwrap_or_default()
}

/// Run the reader, passing on what it says and reporting how far it is.
fn run_reader(cmd: &mut Command) -> bool {
    use std::io::BufRead;
    let Ok(mut child) = cmd.stderr(std::process::Stdio::piped()).stdout(std::process::Stdio::null()).spawn() else { return false };
    if let Some(pipe) = child.stderr.take() {
        for line in std::io::BufReader::new(pipe).lines().map_while(Result::ok) {
            let count = line.trim().strip_prefix("scan ").and_then(|c| c.split_once('/')).and_then(|(d, t)| Some((d.parse().ok()?, t.parse().ok()?)));
            match count {
                Some((done, total)) => {
                    eprintln!("{line}");
                    // (the step is over when the pages are taken in)
                    let total: usize = total;
                    progress::report(Step::Scan, usize::min(done, total.saturating_sub(1)), total);
                }
                // (the libraries under marker are talkative: only its own lines)
                None if line.starts_with("  ") && !line.starts_with("   ") => eprintln!("{line}"),
                None => {}
            }
        }
    }
    child.wait().is_ok_and(|s| s.success())
}

// ---- marker's html ------------------------------------------------------

fn unescape(t: &str) -> String {
    let mut t = t.to_string();
    for (a, b) in [("&lt;", "<"), ("&gt;", ">"), ("&quot;", "\""), ("&#39;", "'"), ("&#x27;", "'"), ("&nbsp;", " "), ("&amp;", "&")] {
        if t.contains(a) {
            t = t.replace(a, b);
        }
    }
    t
}

/// A piece of html: a tag (its name in lowercase, closing or not, and its
/// inside) or text.
enum Piece<'a> {
    Tag(String, bool, &'a str),
    Text(&'a str),
}

fn pieces(html: &str) -> Vec<Piece<'_>> {
    let mut out = Vec::new();
    let mut rest = html;
    while !rest.is_empty() {
        let tag = rest.strip_prefix('<').and_then(|r| r.split_once('>')).filter(|(t, _)| t.starts_with(|c: char| c.is_ascii_alphabetic() || c == '/'));
        match tag {
            Some((tag, after)) => {
                let name = tag.trim_start_matches('/').split([' ', '/']).next().unwrap_or("").to_lowercase();
                out.push(Piece::Tag(name, tag.starts_with('/'), tag));
                rest = after;
            }
            None => {
                let first = rest.chars().next().map_or(1, char::len_utf8);
                let end = rest[first..].find('<').map_or(rest.len(), |i| i + first);
                out.push(Piece::Text(&rest[..end]));
                rest = &rest[end..];
            }
        }
    }
    out
}

/// What a block's html holds, in order: running text, and formulas set
/// on lines of their own.
enum Part {
    Inline(Vec<Inline>),
    Display(String),
}

fn push_text(out: &mut Vec<Inline>, i: Inline) {
    match (out.last_mut(), &i) {
        (Some(Inline::Text(a)), Inline::Text(b)) => a.push_str(b),
        _ => out.push(i),
    }
}

fn parts(html: &str) -> Vec<Part> {
    let mut out: Vec<Part> = Vec::new();
    let mut run: Vec<Inline> = Vec::new();
    let (mut italic, mut bold, mut sup, mut sub) = (0i32, 0i32, 0i32, 0i32);
    let ps = pieces(html);
    let mut k = 0;
    while k < ps.len() {
        match &ps[k] {
            Piece::Tag(name, false, tag) if name == "math" => {
                // (its LaTeX is all up to the closing tag)
                let mut tex = String::new();
                k += 1;
                while k < ps.len() && !matches!(&ps[k], Piece::Tag(n, true, _) if n == "math") {
                    match &ps[k] {
                        Piece::Text(t) => tex.push_str(t),
                        Piece::Tag(_, _, t) => tex.push_str(&format!("<{t}>")),
                    }
                    k += 1;
                }
                let tex = unescape(&tex).trim().to_string();
                if tag.contains("display=\"block\"") {
                    if !run.is_empty() {
                        out.push(Part::Inline(std::mem::take(&mut run)));
                    }
                    out.push(Part::Display(tex));
                } else if !tex.is_empty() {
                    run.push(Inline::Math(tex));
                }
            }
            Piece::Tag(name, closing, _) => {
                let d = if *closing { -1 } else { 1 };
                match name.as_str() {
                    "i" | "em" => italic = (italic + d).max(0),
                    "b" | "strong" => bold = (bold + d).max(0),
                    "sup" => sup = (sup + d).max(0),
                    "sub" => sub = (sub + d).max(0),
                    "br" => run.push(Inline::Break),
                    _ => {}
                }
            }
            Piece::Text(t) => {
                let t = unescape(t);
                let inner = t.trim();
                if inner.is_empty() {
                    if !t.is_empty() {
                        push_text(&mut run, Inline::Text(" ".into()));
                    }
                } else if sup > 0 {
                    run.push(Inline::Sup(inner.to_string()));
                } else if sub > 0 {
                    run.push(Inline::Math(format!("_{{{inner}}}")));
                } else if bold > 0 || italic > 0 {
                    // (the spaces around the words stay outside the marks)
                    if t.starts_with(char::is_whitespace) {
                        push_text(&mut run, Inline::Text(" ".into()));
                    }
                    run.push(if bold > 0 { Inline::Strong(inner.to_string()) } else { Inline::Emph(inner.to_string()) });
                    if t.ends_with(char::is_whitespace) {
                        push_text(&mut run, Inline::Text(" ".into()));
                    }
                } else {
                    push_text(&mut run, Inline::Text(t));
                }
            }
        }
        k += 1;
    }
    if !run.is_empty() {
        out.push(Part::Inline(run));
    }
    out
}

/// Running text without spaces at its ends, and with single spaces in it.
fn tidy(mut inl: Vec<Inline>) -> Vec<Inline> {
    for i in inl.iter_mut() {
        if let Inline::Text(t) = i {
            let mut s = String::with_capacity(t.len());
            for c in t.chars() {
                if !(c.is_whitespace() && s.ends_with(' ')) {
                    s.push(if c.is_whitespace() { ' ' } else { c });
                }
            }
            *t = s;
        }
    }
    if let Some(Inline::Text(t)) = inl.first_mut() {
        *t = t.trim_start().to_string();
    }
    if let Some(Inline::Text(t)) = inl.last_mut() {
        *t = t.trim_end().to_string();
    }
    inl.retain(|i| !matches!(i, Inline::Text(t) if t.is_empty()));
    inl
}

fn all_inline(html: &str) -> Vec<Inline> {
    let mut out = Vec::new();
    for p in parts(html) {
        match p {
            Part::Inline(i) => out.extend(i),
            Part::Display(tex) => out.push(Inline::Math(tex)),
        }
    }
    tidy(out)
}

/// "(12)", "(I.2.15)", "(3a)" at the end of what stands beside a formula.
fn tag_of(text: &str) -> Option<String> {
    let t = text.trim().strip_suffix(')')?;
    let (_, n) = t.rsplit_once('(')?;
    (!n.is_empty() && n.len() <= 10 && n.chars().all(|c| c.is_alphanumeric() || ".'′*-".contains(c)) && n.chars().any(|c| c.is_ascii_digit())).then(|| n.to_string())
}

/// Several formulas of one block: the rows of one, lined up.
fn rows(maths: &[String]) -> String {
    match maths {
        [one] => one.clone(),
        _ => crate::ocr::align_rows(&format!("\\begin{{aligned}}\n{}\n\\end{{aligned}}", maths.iter().map(|m| format!("& {m}")).collect::<Vec<_>>().join(" \\\\\n"))),
    }
}

/// A displayed equation with its number, from its LaTeX and what stood
/// beside it.
fn equation(maths: &[String], beside: &str, key: String) -> Block {
    let mut latex = rows(maths);
    let mut tag = tag_of(beside);
    // (a number the model wrote into the formula: once is enough)
    if let Some((before, after)) = latex.split_once("\\tag{") {
        if let (true, Some((inside, rest))) = (maths.len() == 1, after.split_once('}')) {
            tag.get_or_insert(inside.trim().to_string());
            latex = format!("{}{}", before.trim_end(), rest).trim().to_string();
        }
    }
    // (a name for the equation, which only LaTeX itself can use)
    while let Some((before, after)) = latex.split_once("\\label{") {
        latex = format!("{}{}", before, after.split_once('}').map_or("", |x| x.1).trim_start());
    }
    // the reader's own correction of it comes first
    if let Some(fixed) = crate::ocr::read_correction(&key) {
        latex = fixed;
    }
    Block::Equation { latex, tag, source: Some(key) }
}

fn plain(inl: &[Inline]) -> String {
    crate::plain(inl)
}

fn has_words(inl: &[Inline]) -> bool {
    inl.iter().any(|i| match i {
        Inline::Text(t) | Inline::Emph(t) | Inline::Strong(t) | Inline::Sup(t) => t.chars().any(char::is_alphanumeric),
        Inline::Math(_) => true,
        _ => false,
    })
}

// ---- page numbers -------------------------------------------------------

fn roman(t: &str) -> Option<i32> {
    let v = |c| match c {
        'i' => 1,
        'v' => 5,
        'x' => 10,
        'l' => 50,
        'c' => 100,
        _ => 0,
    };
    let d: Vec<i32> = t.chars().map(v).collect();
    if d.is_empty() || d.contains(&0) {
        return None;
    }
    Some((0..d.len()).map(|i| if d.get(i + 1).is_some_and(|n| *n > d[i]) { -d[i] } else { d[i] }).sum())
}

fn to_roman(mut n: i32) -> String {
    let mut s = String::new();
    for (v, r) in [(100, "c"), (90, "xc"), (50, "l"), (40, "xl"), (10, "x"), (9, "ix"), (5, "v"), (4, "iv"), (1, "i")] {
        while n >= v {
            s.push_str(r);
            n -= v;
        }
    }
    s
}

/// The printed page number in a running head or foot: the head or foot is
/// the number, or starts or ends with it. (true: in roman numerals.)
fn number_in(text: &str) -> Option<(bool, i32)> {
    // (roman numerals are sometimes read in Cyrillic letters that look the same)
    let text: String = text.chars().map(|c| match c { 'х' => 'x', 'і' => 'i', 'Х' => 'X', 'І' => 'I', 'с' => 'c', c => c }).collect();
    let words: Vec<&str> = text.split_whitespace().collect();
    let read = |w: &&str| -> Option<(bool, i32)> {
        if crate::pages::is_arabic(w) {
            return Some((false, w.parse().ok()?));
        }
        // (alone, also in capitals; beside the words of a head only in
        // small letters and of two letters or more: "xii Preface", but
        // not "MIX", "Part I")
        let alone = words.len() == 1 && crate::pages::is_roman(&w.to_lowercase());
        (alone || (w.len() >= 2 && crate::pages::is_roman(w))).then(|| roman(&w.to_lowercase())).flatten().map(|n| (true, n))
    };
    let found = [words.first(), words.last()].into_iter().flatten().find_map(read);
    found
}

/// The printed number of each page. A page that shows none (the first of
/// a chapter), or one read wrong, goes by its neighbours: printed numbers
/// run with the pages.
fn labels(pages: &[PageRead]) -> Vec<Option<String>> {
    let seen: Vec<Option<(bool, i32)>> = pages
        .iter()
        .map(|p| p.blocks.iter().filter(|b| b.kind == "PageHeader" || b.kind == "PageFooter").find_map(|b| number_in(&plain(&all_inline(&b.html)))))
        .collect();
    // (kind of numerals, number less the page's place) of each page
    let offset = |i: usize| seen[i].map(|(r, n)| (r, n - i as i32));
    let labels = (0..pages.len())
        .map(|i| {
            let mut votes: std::collections::HashMap<(bool, i32), (usize, usize)> = Default::default();
            for j in i.saturating_sub(6)..(i + 7).min(pages.len()) {
                if let Some(o) = offset(j) {
                    let v = votes.entry(o).or_insert((0, usize::MAX));
                    v.0 += 1;
                    v.1 = v.1.min(i.abs_diff(j));
                }
            }
            // what most pages around say; the nearest, if they are even
            let best = votes.into_iter().filter(|(_, (n, _))| *n >= 2).max_by_key(|(_, (n, near))| (*n, std::cmp::Reverse(*near))).map(|(o, _)| o);
            let (is_roman, n) = match (best, seen[i]) {
                (Some((r, o)), _) => (r, o + i as i32),
                (None, Some(own)) => own,
                (None, None) => return None,
            };
            (n > 0).then(|| if is_roman { to_roman(n) } else { n.to_string() })
        })
        .collect();
    // pages without numbers after the last numbered one (an index) count on
    let mut labels: Vec<Option<String>> = labels;
    for i in 1..labels.len() {
        if let (None, Some(Ok(n))) = (&labels[i], labels[i - 1].as_ref().map(|l| l.parse::<u32>())) {
            if seen[i].is_none() {
                labels[i] = Some((n + 1).to_string());
            }
        }
    }
    labels
}

// ---- headings -----------------------------------------------------------

/// "Chapter 3", "Part II", "Appendix A": a title that says what it heads.
fn names_chapter(t: &str) -> bool {
    let mut w = t.split_whitespace();
    matches!(w.next().map(str::to_lowercase).as_deref(), Some("chapter" | "part" | "appendix")) && w.next().is_some_and(|n| n.len() <= 6)
}

/// The type sizes of the book's headings: of each heading (by its place
/// in `heights`) the size of its kind. A heading's height says its size
/// only roughly, so sizes close together are one kind. `heights`: the
/// height of the heading's block and its number of lines.
fn sizes(heights: &[(f32, usize)]) -> Vec<f32> {
    // the kinds: from the headings of one line
    let mut single: Vec<f32> = heights.iter().filter(|h| h.1 <= 1).map(|h| h.0).collect();
    single.sort_by(f32::total_cmp);
    let mut kinds: Vec<Vec<f32>> = Vec::new();
    for h in single {
        match kinds.last_mut() {
            // (within an eighth of the kind's first: the same type)
            Some(k) if h <= k[0] * 1.125 => k.push(h),
            _ => kinds.push(vec![h]),
        }
    }
    let kinds: Vec<f32> = kinds.iter().map(|k| k[k.len() / 2]).collect();
    let nearest = |h: f32| kinds.iter().copied().min_by(|a, b| (a / h).ln().abs().total_cmp(&(b / h).ln().abs()));
    heights
        .iter()
        .map(|&(h, lines)| {
            let size = if lines <= 1 {
                nearest(h)
            } else {
                // several lines: between the height shared out evenly and
                // the height of lines set far apart
                let (low, high) = (h / (1.0 + 1.6 * (lines - 1) as f32), h / lines as f32);
                kinds.iter().copied().filter(|k| *k >= low * 0.95 && *k <= high * 1.05).max_by(f32::total_cmp).or_else(|| nearest((low * high).sqrt()))
            };
            (size.unwrap_or(h) * PX * 10.0).round() / 10.0
        })
        .collect()
}

/// The type sizes given to chapter titles and to the headings inside a
/// chapter, where they are told apart by their place (see [`openers`]).
const CHAPTER: f32 = 30.0;
const SECTION: f32 = 19.5;

/// The heading a page opens with.
#[derive(Clone, Copy, PartialEq, Debug)]
struct Opener {
    /// which block of the page it is
    at: usize,
    /// the block of its title, if it only says "Chapter 3"
    title: Option<usize>,
    /// it is the title of a chapter (not one of the title pages, the
    /// preface, the contents, which come before the first chapter)
    chapter: bool,
}

/// The chapter openings of a book with running heads: a page that starts
/// with a heading and has no running head starts a chapter (the height of
/// a heading's box says too little about its type size to tell a chapter
/// title from a section's: it depends on the letters in it). In a book
/// whose chapters say so ("Chapter 3", "Part II"), what opens a page
/// before the first of them is front matter. None for a book without
/// running heads: there the headings go by their sizes.
fn openers(pages: &[PageRead]) -> Option<Vec<Option<Opener>>> {
    let has_head = |p: &PageRead| p.blocks.iter().any(|b| b.kind == "PageHeader");
    if pages.len() < 4 || pages.iter().filter(|p| has_head(p)).count() * 2 < pages.len() {
        return None;
    }
    let text = |b: &Raw| plain(&all_inline(&b.html));
    let mut out: Vec<Option<Opener>> = pages
        .iter()
        .map(|p| {
            let at = p.blocks.iter().position(|b| b.kind != "PageHeader" && b.kind != "PageFooter")?;
            if has_head(p) || p.blocks[at].kind != "SectionHeader" {
                return None;
            }
            // "Chapter I.5" alone, its title in the heading under it
            let t = text(&p.blocks[at]);
            // (or a heading nobody could read)
            let alone = (names_chapter(&t) && t.split_whitespace().count() <= 2) || !t.chars().any(char::is_alphanumeric);
            let title = (at + 1 < p.blocks.len() && alone && p.blocks[at + 1].kind == "SectionHeader").then_some(at + 1);
            Some(Opener { at, title, chapter: names_chapter(&t) })
        })
        .collect();
    let named = out.iter().position(|o| o.is_some_and(|o| o.chapter));
    for (i, o) in out.iter_mut().enumerate() {
        if let Some(o) = o {
            o.chapter = named.is_none_or(|first| i >= first);
        }
    }
    Some(out)
}

// ---- tables of contents -------------------------------------------------

/// The entries of a table of contents given line by line ("I.1  Who
/// Needs It? . . . 3"); None if the lines are no such table.
fn contents(lines: &[String]) -> Option<Vec<TocEntry>> {
    let mut entries = Vec::new();
    let mut with_page = 0;
    for line in lines {
        let indent = line.chars().take_while(|c| *c == ' ').count() as f32;
        let words: Vec<&str> = line.split_whitespace().filter(|w| !w.chars().all(|c| c == '.' || c == '·' || c == '…')).collect();
        let Some(last) = words.last() else { continue };
        let page = (words.len() > 1 && crate::pages::is_page_number(last)).then(|| last.to_string());
        let title = words[..words.len() - page.is_some() as usize].join(" ");
        if !title.chars().any(char::is_alphabetic) {
            continue;
        }
        with_page += page.is_some() as usize;
        // (a line in capitals without a page: the name of a part, which
        // goes on under "PART I")
        let part = page.is_none() && !title.chars().any(char::is_lowercase);
        if let (true, Some(before)) = (part, entries.last_mut().filter(|e: &&mut TocEntry| e.page.is_empty() && e.size > BODY && names_chapter(&crate::plain(&e.title)))) {
            before.title = vec![Inline::Text(format!("{} {}", crate::plain(&before.title), title))];
            continue;
        }
        // (a title too long for its line goes on in the next, where its page is)
        if let Some(before) = entries.last_mut().filter(|e: &&mut TocEntry| e.page.is_empty() && e.size == BODY && !part) {
            before.title = vec![Inline::Text(format!("{} {}", crate::plain(&before.title), title.trim_end_matches(['.', ' '])))];
            before.page = page.unwrap_or_default();
            continue;
        }
        entries.push(TocEntry {
            title: vec![Inline::Text(title.trim_end_matches(['.', ' ']).to_string())],
            page: page.unwrap_or_default(),
            left: if part { 0.0 } else { (indent / 4.0).round() * 12.0 },
            size: if part { BODY + 3.0 } else { BODY },
        });
    }
    (entries.len() >= 3 && with_page * 2 >= entries.len()).then_some(entries)
}

// ---- the blocks ---------------------------------------------------------

/// A footnote: its mark and its text.
fn footnote(html: &str) -> Option<(String, Vec<Inline>)> {
    let mut inl = all_inline(html);
    let label = match inl.first() {
        Some(Inline::Sup(l)) => {
            let l = l.clone();
            inl.remove(0);
            l
        }
        Some(Inline::Text(t)) => {
            // "1 When I took …", "* See …"
            let mark: String = t.chars().take_while(|c| c.is_ascii_digit() || "*†‡§".contains(*c)).collect();
            if mark.is_empty() || mark.len() > 3 {
                return None;
            }
            let rest = t[mark.len()..].trim_start().to_string();
            inl[0] = Inline::Text(rest);
            mark
        }
        _ => return None,
    };
    Some((label, tidy(inl)))
}

/// Program code, by what only code is full of: braces, semicolons,
/// assignments, calls, arrows, the words of a language.
fn looks_like_code(t: &str) -> bool {
    let words = t.split_whitespace().count().max(1);
    let signs = t.matches(['{', '}', ';']).count() + t.matches(" = ").count() + t.matches("()").count() + t.matches("->").count() + t.matches("==").count();
    let keywords = t.split(|c: char| !c.is_alphanumeric() && c != '#').filter(|w| ["def", "return", "import", "#include", "void", "int", "function", "printf", "print", "elif", "endif", "var", "const", "fn", "let"].contains(w)).count();
    (signs + 2 * keywords) * 5 >= words && signs + keywords >= 3
}

/// Blocks of running text and headings.
fn is_text(b: &Raw) -> bool {
    matches!(b.kind.as_str(), "Text" | "TextInlineMath" | "SectionHeader" | "Caption" | "ListItem")
}

/// The publisher's name at the foot of the title page is no running
/// foot: a foot of several words without a page number, on a page with no
/// other head or foot, that the pages around do not repeat, is text.
fn imprints(pages: &[PageRead]) -> Vec<PageRead> {
    let text = |b: &Raw| crate::key(&plain(&all_inline(&b.html)));
    let heads = |p: &PageRead| -> Vec<String> { p.blocks.iter().filter(|b| b.kind == "PageHeader" || b.kind == "PageFooter").map(text).collect() };
    let mut out = pages.to_vec();
    for (i, page) in out.iter_mut().enumerate() {
        let around: Vec<String> = (i.saturating_sub(4)..(i + 5).min(pages.len())).filter(|j| *j != i).flat_map(|j| heads(&pages[j])).collect();
        let alone = page.blocks.iter().filter(|b| b.kind == "PageHeader" || b.kind == "PageFooter").count() == 1;
        for b in page.blocks.iter_mut().filter(|b| b.kind == "PageFooter") {
            let words = plain(&all_inline(&b.html));
            if alone && words.split_whitespace().count() >= 2 && number_in(&words).is_none() && !around.contains(&text(b)) {
                b.kind = "Text".into();
            }
        }
    }
    out
}

/// Is everything on this page set centered (a title page, a dedication)?
/// Every block of text lies about the middle of the page and is clearly
/// narrower than the page.
fn centered_page(page: &PageRead) -> bool {
    let middle = page.width / 2.0;
    let centered = |b: &Raw| ((b.left + b.right) / 2.0 - middle).abs() <= 0.045 * page.width && b.right - b.left < 0.8 * page.width;
    let mut text = page.blocks.iter().filter(|b| is_text(b) && has_words(&all_inline(&b.html))).peekable();
    // (a table, a table of contents or an equation has a shape of its own)
    let shaped = page.blocks.iter().any(|b| matches!(b.kind.as_str(), "Equation" | "Table" | "Form" | "TableOfContents") && (b.kind == "Equation" || !b.text.is_empty()));
    page.width > 0.0 && text.peek().is_some() && text.all(centered) && !shaped
}

/// Running text with its lines ended where `lines` end them, for the
/// line ends that `keep` says are meant (by their place among the lines).
/// The text as it was if it is not the words of `lines`, or holds more
/// than words (a formula).
fn with_breaks(inl: Vec<Inline>, lines: &[String], keep: &dyn Fn(usize) -> bool) -> Vec<Inline> {
    let counts: Vec<usize> = lines.iter().map(|l| l.split_whitespace().count()).collect();
    let words = |i: &Inline| match i {
        Inline::Text(t) | Inline::Emph(t) | Inline::Strong(t) => Some(t.split_whitespace().count()),
        _ => None,
    };
    let total: Option<usize> = inl.iter().map(words).sum();
    if lines.len() < 2 || total != Some(counts.iter().sum()) {
        return inl;
    }
    // the place of each line's last word
    let mut ends: Vec<usize> = Vec::new();
    for c in &counts[..counts.len() - 1] {
        ends.push(ends.last().copied().unwrap_or(0) + c);
    }
    let mut out: Vec<Inline> = Vec::new();
    let (mut n, mut fresh) = (0, true);
    for i in &inl {
        let (Inline::Text(t) | Inline::Emph(t) | Inline::Strong(t)) = i else { continue };
        for w in t.split_whitespace() {
            match (out.last_mut(), i) {
                (Some(Inline::Text(last)), Inline::Text(_)) | (Some(Inline::Emph(last)), Inline::Emph(_)) | (Some(Inline::Strong(last)), Inline::Strong(_)) if !fresh => {
                    last.push(' ');
                    last.push_str(w);
                }
                _ => {
                    if !fresh {
                        push_text(&mut out, Inline::Text(" ".into()));
                    }
                    let piece = match i {
                        Inline::Emph(_) => Inline::Emph(w.into()),
                        Inline::Strong(_) => Inline::Strong(w.into()),
                        _ => Inline::Text(w.into()),
                    };
                    push_text(&mut out, piece);
                }
            }
            n += 1;
            fresh = false;
            if let Some(k) = ends.iter().position(|e| *e == n) {
                if keep(k) {
                    // (a space, so that the words stay apart where the
                    // break is not written: in an anchor's name)
                    push_text(&mut out, Inline::Text(" ".into()));
                    out.push(Inline::Break);
                    fresh = true;
                }
            }
        }
    }
    out
}

/// Does line `k` of a block's `lines` end where the author ended it (a
/// hard return), not where the measure did? It does if the first word of
/// the next line would have fitted after it: the line is clearly shorter
/// than the block's longest line, or even that line stops short of the
/// right margin (`room`: the space from the block's right edge to the
/// margin, in letters).
fn hard_return(lines: &[String], k: usize, room: f32) -> bool {
    let len = |l: &String| l.chars().count() as f32;
    let longest = lines.iter().map(len).fold(1.0, f32::max);
    let Some(next) = lines.get(k + 1).and_then(|l| l.split_whitespace().next()) else { return false };
    let needed = next.chars().count() as f32 + 1.0;
    len(&lines[k]) + needed <= longest + room.max(0.0)
}

/// How much of block `a` lies inside block `b`, 0 to 1.
fn inside(a: &Raw, b: &Raw) -> f32 {
    let w = (a.right.min(b.right) - a.left.max(b.left)).max(0.0);
    let h = (a.bottom.min(b.bottom) - a.top.max(b.top)).max(0.0);
    let area = (a.right - a.left) * (a.bottom - a.top);
    if area > 0.0 {
        w * h / area
    } else {
        0.0
    }
}

/// Is this page the book's cover? The first sheet of the PDF, a picture
/// over a third of it or more, and only a few words (the title, the
/// author) besides.
fn is_cover(page: &PageRead) -> bool {
    let sheet = page.width * page.height;
    let picture = page.blocks.iter().filter(|b| b.kind == "Picture" || b.kind == "Figure").map(|b| (b.right - b.left) * (b.bottom - b.top)).fold(0.0, f32::max);
    let words: usize = page.blocks.iter().filter(|b| b.kind != "Picture" && b.kind != "Figure").map(|b| plain(&all_inline(&b.html)).split_whitespace().count()).sum();
    page.number == 1 && sheet > 0.0 && picture >= sheet / 3.0 && words <= 30
}

fn ends_sentence(t: &str) -> bool {
    t.trim_end().ends_with(['.', '!', '?', ':', ';', '”', '"', ')'])
}

/// The blocks of a scanned book from its pages. `book`: what tells this
/// book from others (for the keys of its equations).
fn build(pages: &[PageRead], book: &str) -> Scan {
    let pages = &imprints(pages);
    let labels = labels(pages);
    // how wide the text is set: the usual width of a full paragraph
    let mut widths: Vec<f32> = pages.iter().flat_map(|p| p.blocks.iter().filter(|b| b.kind == "Text" && b.right - b.left > 0.7 * p.width).map(|b| b.right - b.left)).collect();
    widths.sort_by(f32::total_cmp);
    let measure = widths.get(widths.len() / 2).copied();
    // the sizes of the headings, in the order they come
    let heads: Vec<(f32, usize)> = pages.iter().flat_map(|p| &p.blocks).filter(|b| b.kind == "SectionHeader").map(|b| (b.bottom - b.top, b.lines)).collect();
    let head_sizes = sizes(&heads);
    let mut head = 0;
    let openers = openers(pages);
    // (the title under a "Chapter 3" was joined to it: not a heading again)
    let mut joined = None;
    let mut blks: Vec<Blk> = Vec::new();
    // (size, text) of the headings, to find the size of the chapter titles
    let mut titles: Vec<(f32, String)> = Vec::new();
    for (pi, page) in pages.iter().enumerate() {
        // a page set centered keeps its look: centered, every line as it ends
        let all_centered = centered_page(page);
        let push = |blks: &mut Vec<Blk>, block: Block| {
            let centered = all_centered && !matches!(block, Block::Anchor(_));
            blks.push(Blk { block, page: pi, centered })
        };
        // where the text starts on this page, and where its lines end
        let left = page.blocks.iter().filter(|b| is_text(b)).map(|b| b.left).fold(f32::MAX, f32::min);
        let right = left + measure.unwrap_or(0.9 * page.width);
        let breaks = |b: &Raw, inl: Vec<Inline>| -> Vec<Inline> {
            // (a heading's lines are its own only on a page set centered:
            // a chapter title in two lines is one title)
            if b.text.len() < 2 || !is_text(b) || (b.kind == "SectionHeader" && !all_centered) {
                return inl;
            }
            // A block as wide as the text is set is a paragraph: its
            // lines end where the measure ends them. (Its lines may come
            // from the PDF's own text, which knows no other kind.)
            if !all_centered && right - b.right < 0.06 * page.width {
                return inl;
            }
            // (the room to the margin in letters, by the width of this block's letters)
            let longest = b.text.iter().map(|l| l.chars().count()).max().unwrap_or(1).max(1) as f32;
            let room = (right - b.right) / ((b.right - b.left).max(1.0) / longest);
            with_breaks(inl, &b.text, &|k| all_centered || hard_return(&b.text, k, room))
        };
        let mut anchor = labels[pi].as_ref().map(|l| format!("p{l}"));
        let (mut figures, mut equations) = (0, 0);
        let mut first_text = true;
        // The cover: a picture with the title and the author lettered on
        // it. All of the page is the picture; its words are not text.
        if is_cover(page) {
            push(&mut blks, Block::Figure {
                page: page.number,
                n: 1,
                area: Figure { left: 0.0, right: page.width * PX, top: 0.0, bottom: page.height * PX },
                label: labels[pi].clone(),
            });
            continue;
        }
        let pictures: Vec<&Raw> = page.blocks.iter().filter(|b| b.kind == "Picture" || b.kind == "Figure").collect();
        for (bi, b) in page.blocks.iter().enumerate() {
            let area = Figure { left: b.left * PX, right: b.right * PX, top: b.top * PX, bottom: b.bottom * PX };
            let mut made: Vec<Block> = Vec::new();
            // words lettered on a picture are in the picture
            if !matches!(b.kind.as_str(), "Picture" | "Figure") && pictures.iter().any(|p| inside(b, p) >= 0.6) {
                continue;
            }
            match b.kind.as_str() {
                "PageHeader" | "PageFooter" => continue,
                "SectionHeader" => {
                    let mut size = head_sizes.get(head).copied().unwrap_or(BODY);
                    head += 1;
                    if joined == Some((pi, bi)) {
                        continue;
                    }
                    let mut text = breaks(b, crate::no_emph(all_inline(&b.html)));
                    if let Some(openers) = &openers {
                        // by where the headings stand, not by their heights
                        size = SECTION;
                        if let Some(o) = openers[pi].filter(|o| o.at == bi) {
                            size = if o.chapter { CHAPTER } else { SECTION + 3.0 };
                            if let Some(title) = o.title {
                                if has_words(&text) {
                                    text.push(Inline::Text(": ".into()));
                                }
                                text.extend(crate::no_emph(all_inline(&page.blocks[title].html)));
                                text = crate::no_emph(text);
                                joined = Some((pi, title));
                            }
                        }
                    }
                    if has_words(&text) {
                        titles.push((size, plain(&text)));
                        made.push(Block::Heading { level: 0, size, id: String::new(), text });
                    }
                }
                "Footnote" => match footnote(&b.html) {
                    Some((label, text)) => made.push(Block::PageNote { label, text }),
                    None => made.push(Block::Para(all_inline(&b.html))),
                },
                "Picture" | "Figure" => {
                    figures += 1;
                    made.push(Block::Figure { page: page.number, n: figures, area, label: labels[pi].clone() });
                }
                "Table" | "Form" | "TableOfContents" => {
                    // (nothing written in it: marker saw a table in an
                    // empty page, or in a speck on one)
                    if b.text.iter().flat_map(|t| t.chars()).filter(|c| c.is_alphanumeric()).count() < 4 {
                        continue;
                    }
                    match contents(&b.text) {
                        Some(entries) => made.push(Block::Toc(entries)),
                        None => {
                            figures += 1;
                            made.push(Block::Figure { page: page.number, n: figures, area, label: labels[pi].clone() });
                        }
                    }
                }
                "Code" => {
                    let text: String = pieces(&b.html).iter().filter_map(|p| if let Piece::Text(t) = p { Some(unescape(t)) } else { None }).collect();
                    // (marker calls lists of names and numbers code, too)
                    if looks_like_code(&text) {
                        made.push(Block::Code(text.trim().to_string()));
                    } else {
                        let words = all_inline(&b.html);
                        if has_words(&words) {
                            made.push(Block::Para(words));
                        }
                    }
                }
                _ => {
                    // text, with the formulas set on lines of their own
                    // taken out of it; an equation's number stands beside it
                    let is_equation = b.kind == "Equation";
                    let ps = parts(&b.html);
                    let beside: String = ps.iter().filter_map(|p| if let Part::Inline(i) = p { Some(plain(i)) } else { None }).collect::<Vec<_>>().join(" ");
                    let maths: Vec<String> = ps.iter().filter_map(|p| if let Part::Display(t) = p { Some(t.clone()) } else { None }).collect();
                    let mut key = || {
                        equations += 1;
                        format!("{:016x}", fnv(format!("{book} {} {equations}", page.number).as_bytes()))
                    };
                    if is_equation && !maths.is_empty() && (beside.trim().is_empty() || tag_of(&beside).is_some_and(|t| beside.trim().len() <= t.len() + 2)) {
                        made.push(equation(&maths, &beside, key()));
                    } else {
                        for p in ps {
                            match p {
                                Part::Inline(i) => {
                                    let i = breaks(b, tidy(i));
                                    if has_words(&i) {
                                        made.push(Block::Para(i));
                                    }
                                }
                                Part::Display(tex) => made.push(equation(&[tex], "", key())),
                            }
                        }
                    }
                }
            }
            for block in made {
                // a paragraph that goes on from the page before: one
                // paragraph, with the page's mark where the page turns
                // (the footnotes of the page before stand after its text)
                let last = blks.iter().rposition(|o| !matches!(o.block, Block::PageNote { .. }));
                let goes_on = first_text
                    && matches!(&block, Block::Para(i) if plain(i).starts_with(char::is_lowercase))
                    && page.blocks[..bi].iter().all(|o| o.kind == "PageHeader" || o.kind == "PageFooter")
                    && last.is_some_and(|k| matches!(&blks[k], Blk { block: Block::Para(i), page: p, .. } if *p + 1 == pi && !ends_sentence(&plain(i))));
                first_text = false;
                if goes_on {
                    let (Some(Blk { block: Block::Para(before), .. }), Block::Para(after)) = (last.map(|k| &mut blks[k]), block) else { unreachable!() };
                    // (a word cut in two at the end of the page is one again)
                    let hyphen = matches!(before.last(), Some(Inline::Text(t)) if t.ends_with('-') && t.len() > 1);
                    match before.last_mut() {
                        Some(Inline::Text(t)) if hyphen => {
                            t.pop();
                        }
                        _ => before.push(Inline::Text(" ".into())),
                    }
                    before.extend(anchor.take().map(Inline::Anchor));
                    before.extend(after);
                    continue;
                }
                if let Some(a) = anchor.take() {
                    push(&mut blks, Block::Anchor(a));
                }
                push(&mut blks, block);
            }
        }
        // an empty page still has its mark
        if let Some(a) = anchor.take() {
            push(&mut blks, Block::Anchor(a));
        }
    }
    // The chapter titles: the size of the headings that say "Chapter 3"
    // (the largest such size), else the most common size that is larger
    // than most headings.
    let count = |size: f32, named: bool| titles.iter().filter(|(s, t)| *s == size && (!named || names_chapter(t))).count();
    let mut all: Vec<f32> = titles.iter().map(|t| t.0).collect();
    all.sort_by(f32::total_cmp);
    all.dedup();
    let chapter_size = if openers.is_some() {
        all.contains(&CHAPTER).then_some(CHAPTER)
    } else {
        all.iter().rev().copied().find(|s| count(*s, true) >= 2).or_else(|| {
            let smallest = all.first().copied()?;
            all.iter().copied().filter(|s| *s > smallest && count(*s, false) >= 3).max_by_key(|s| count(*s, false))
        })
    };
    if openers.is_none() {
        join_titles(&mut blks, chapter_size);
    }
    // An entry of the table of contents is read less well than the title
    // it names (and "I.5 Coulomb …" is "Chapter I.5: Coulomb …" there):
    // where a chapter opens on the entry's page, the entry is that title.
    let mut opens: std::collections::HashMap<String, Vec<Inline>> = Default::default();
    for b in &blks {
        if let (Block::Heading { size, text, .. }, Some(label)) = (&b.block, &labels[b.page]) {
            if Some(*size) == chapter_size {
                opens.entry(label.clone()).or_insert_with(|| text.clone());
            }
        }
    }
    for b in blks.iter_mut() {
        if let Block::Toc(entries) = &mut b.block {
            for e in entries.iter_mut() {
                if let Some(title) = opens.get(&e.page) {
                    e.title = title.clone();
                }
            }
        }
    }
    Scan { blks, labels, pdf_pages: pages.iter().map(|p| p.number).collect(), chapter_size }
}

/// "Chapter I.5" and, under it, "Coulomb and Newton": one title.
fn join_titles(blks: &mut Vec<Blk>, chapter_size: Option<f32>) {
    let Some(c) = chapter_size else { return };
    let mut k = 0;
    while k + 1 < blks.len() {
        let same_page = blks[k].page == blks[k + 1].page;
        let (a, b) = blks.split_at_mut(k + 1);
        if let (Block::Heading { size: s1, text: t1, .. }, Block::Heading { size: s2, text: t2, .. }) = (&mut a[k].block, &b[0].block) {
            if same_page && *s1 == c && *s2 >= c && names_chapter(&plain(t1)) && !names_chapter(&plain(t2)) {
                t1.push(Inline::Text(": ".into()));
                t1.extend(t2.iter().cloned());
                *t1 = crate::no_emph(std::mem::take(t1));
                blks.remove(k + 1);
                continue;
            }
        }
        k += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn page(number: u32, blocks: &[(&str, f32, f32, usize, &str)]) -> PageRead {
        let text: String =
            blocks.iter().map(|(kind, top, bottom, lines, html)| format!("block\t{kind}\t30.0\t{top}\t470.0\t{bottom}\t{lines}\t{html}\n")).collect();
        parse_page(number, &format!("page\t487.0\t842.0\n{text}"))
    }

    #[test]
    fn html_becomes_running_text() {
        let inl = all_inline("<p block-type=\"Text\">So <i>energy</i> is <math display=\"inline\">\\hbar\\omega &lt; 1</math>.<sup>2</sup> A &amp; B</p>");
        assert_eq!(
            inl,
            vec![
                Inline::Text("So ".into()),
                Inline::Emph("energy".into()),
                Inline::Text(" is ".into()),
                Inline::Math("\\hbar\\omega < 1".into()),
                Inline::Text(".".into()),
                Inline::Sup("2".into()),
                Inline::Text(" A & B".into()),
            ]
        );
        assert_eq!(tag_of(" (I.2.15)"), Some("I.2.15".into()));
        assert_eq!(tag_of("(see below)"), None);
        let note = parse_page(1, "block\tFootnote\t0\t0\t9\t9\t1\t<p><sup>&amp;</sup>lt;sup&gt;1&lt;/sup&gt;When I was young</p>");
        let (label, text) = footnote(&note.blocks[0].html).unwrap();
        assert_eq!((label.as_str(), plain(&text).as_str()), ("1", "When I was young"));
        let marked = parse_page(1, "block\tText\t0\t0\t9\t9\t1\t<p>the equation.&lt;sup&gt;2&lt;/sup&gt; Being</p>");
        assert_eq!(all_inline(&marked.blocks[0].html)[1], Inline::Sup("2".into()));
    }

    #[test]
    fn pages_become_blocks() {
        let pages = vec![
            page(
                10,
                &[
                    ("PageHeader", 15.0, 28.0, 1, "<p>16</p>"),
                    ("PageHeader", 15.0, 28.0, 1, "<p>I. Motivation</p>"),
                    ("SectionHeader", 100.0, 120.0, 1, "<h1>Chapter I.4</h1>"),
                    ("SectionHeader", 140.0, 191.0, 2, "<h1>From Field to Particle</h1>"),
                    ("SectionHeader", 250.0, 264.0, 1, "<h4>The vacuum</h4>"),
                    ("Text", 280.0, 300.0, 1, "<p>We write</p>"),
                    ("Equation", 310.0, 340.0, 1, "<p block-type=\"Equation\"><math display=\"block\">a = b</math><math display=\"block\">= c</math> (12)</p>"),
                    ("Text", 350.0, 400.0, 1, "<p>and the story goes on to the neigh-</p>"),
                    ("Footnote", 780.0, 800.0, 1, "<p><sup>1</sup>A note.</p>"),
                ],
            ),
            page(
                11,
                &[
                    ("Text", 50.0, 80.0, 1, "<p>bors of the lattice.</p>"),
                    ("Picture", 100.0, 300.0, 0, ""),
                    ("Form", 0.0, 842.0, 0, "<table><tbody><tr><th></th></tr></tbody></table>"),
                    ("SectionHeader", 400.0, 414.0, 1, "<h2>The classical limit</h2>"),
                ],
            ),
            page(12, &[("PageHeader", 15.0, 28.0, 1, "<p>18</p>"), ("SectionHeader", 100.0, 120.0, 1, "<h1>Chapter I.5</h1>")]),
        ];
        let scan = build(&pages, "book");
        // the page between two numbered ones has its number too
        assert_eq!(scan.labels, [Some("16".to_string()), Some("17".to_string()), Some("18".to_string())]);
        let kinds: Vec<String> = scan
            .blks
            .iter()
            .map(|b| match &b.block {
                Block::Anchor(a) => format!("anchor {a}"),
                Block::Heading { text, .. } => format!("# {}", plain(text)),
                Block::Para(i) => format!("para {}", plain(i)),
                Block::Equation { latex, tag, .. } => format!("eq {tag:?} {}", latex.contains("aligned")),
                Block::PageNote { label, .. } => format!("note {label}"),
                Block::Figure { page, .. } => format!("figure {page}"),
                other => format!("{other:?}"),
            })
            .collect();
        assert_eq!(
            kinds,
            [
                "anchor p16",
                "# Chapter I.4: From Field to Particle",
                "# The vacuum",
                "para We write",
                "eq Some(\"12\") true",
                "para and the story goes on to the neighbors of the lattice.",
                "note 1",
                "figure 11",
                "# The classical limit",
                "anchor p18",
                "# Chapter I.5",
            ]
        );
        // the mark of page 17 is inside the paragraph that crosses to it
        let crossing = scan.blks.iter().find_map(|b| if let Block::Para(i) = &b.block { i.iter().any(|x| matches!(x, Inline::Anchor(a) if a == "p17")).then_some(()) } else { None });
        assert!(crossing.is_some());
        // chapter titles are of one size, larger than the sections'
        let size = |t: &str| scan.blks.iter().find_map(|b| if let Block::Heading { size, text, .. } = &b.block { (plain(text).starts_with(t)).then_some(*size) } else { None }).unwrap();
        assert_eq!(Some(size("Chapter I.4")), scan.chapter_size);
        assert_eq!(size("Chapter I.4"), size("Chapter I.5"));
        assert!(size("The vacuum") < size("Chapter I.4") && size("The vacuum") == size("The classical limit"));
    }

    #[test]
    fn a_page_that_opens_with_a_heading_and_has_no_running_head_opens_a_chapter() {
        let head = ("PageHeader", 15.0, 28.0, 1, "<p>Some running head</p>");
        let text = ("Text", 50.0, 80.0, 1, "<p>Words of the book.</p>");
        let pages = vec![
            page(1, &[("SectionHeader", 100.0, 128.0, 1, "<h2>Preface</h2>"), text]),
            page(2, &[head, text]),
            page(3, &[("SectionHeader", 100.0, 120.0, 1, "<h1>Chapter I.1</h1>"), ("SectionHeader", 140.0, 158.0, 1, "<h3>Who Needs It?</h3>"), ("SectionHeader", 250.0, 265.0, 1, "<h4>A tall heading by</h4>"), text]),
            page(4, &[head, ("SectionHeader", 50.0, 60.0, 1, "<h4>The vacuum</h4>"), text]),
            page(5, &[head, text]),
            page(6, &[("SectionHeader", 100.0, 120.0, 1, "<h1>Further Reading</h1>"), text]),
        ];
        let scan = build(&pages, "book");
        let heads: Vec<(String, f32)> =
            scan.blks.iter().filter_map(|b| if let Block::Heading { size, text, .. } = &b.block { Some((plain(text), *size)) } else { None }).collect();
        assert_eq!(
            heads,
            [
                ("Preface".to_string(), SECTION + 3.0),
                ("Chapter I.1: Who Needs It?".to_string(), CHAPTER),
                ("A tall heading by".to_string(), SECTION),
                ("The vacuum".to_string(), SECTION),
                ("Further Reading".to_string(), CHAPTER),
            ]
        );
        assert_eq!(scan.chapter_size, Some(CHAPTER));
    }

    #[test]
    fn the_cover_is_one_picture() {
        let cover = parse_page(
            1,
            "page\t265.0\t400.0\nblock\tPicture\t0.0\t0.0\t264.0\t251.6\t0\t\nblock\tText\t14.0\t220.3\t259.0\t265.2\t1\t<p>uantum Field Theory</p>\nblock\tText\t159.4\t280.1\t230.6\t302.9\t1\t<p>A. Zee</p>",
        );
        // (the same blocks on a later page: a picture and the text that
        // is not lettered on it)
        let mut later = cover.clone();
        later.number = 7;
        let scan = build(&[cover, later], "book");
        let kinds: Vec<String> = scan
            .blks
            .iter()
            .map(|b| match &b.block {
                Block::Figure { area, .. } => format!("figure {:.0}x{:.0}", area.right - area.left, area.bottom - area.top),
                Block::Para(i) => format!("para {}", plain(i)),
                other => format!("{other:?}"),
            })
            .collect();
        assert_eq!(kinds, ["figure 398x600", "figure 396x377", "para A. Zee"]);
        // words that lie on a picture are part of it
        let lettered = parse_page(3, "page\t400.0\t600.0\nblock\tPicture\t50.0\t50.0\t350.0\t300.0\t0\t\nblock\tText\t100.0\t100.0\t200.0\t120.0\t1\t<p>x axis</p>\nblock\tText\t50.0\t320.0\t350.0\t340.0\t1\t<p>Below it.</p>");
        let scan = build(&[lettered], "book");
        assert_eq!(scan.blks.iter().filter(|b| matches!(b.block, Block::Para(_))).count(), 1);
    }

    #[test]
    fn a_centered_page_stays_centered_with_its_lines() {
        let block = |kind: &str, left: f32, top: f32, right: f32, bottom: f32, html: &str, lines: &[&str]| {
            let lines: String = lines.iter().map(|l| format!("line\t{l}\n")).collect();
            format!("block\t{kind}\t{left}\t{top}\t{right}\t{bottom}\t{}\t{html}\n{lines}", lines.matches('\n').count())
        };
        let title = parse_page(
            3,
            &format!(
                "page\t487.0\t842.0\n{}{}{}",
                block("SectionHeader", 59.0, 92.0, 432.0, 155.0, "<h2>QUANTUM FIELD THEORY IN A NUTSHELL</h2>", &["QUANTUM FIELD THEORY", "IN A NUTSHELL"]),
                block("Text", 109.0, 309.0, 380.0, 350.0, "<p>Kavli Institute for <i>Theoretical Physics</i> University of California</p>", &["Kavli Institute for Theoretical Physics", "University of California"]),
                block("PageFooter", 145.0, 768.0, 342.0, 802.0, "<p>PRINCETON UNIVERSITY PRESS</p>", &[]),
            ),
        );
        // a page of ordinary text: an address keeps its lines, a sentence
        // that the measure broke does not
        let text = parse_page(
            4,
            &format!(
                "page\t487.0\t842.0\n{}{}{}",
                block("Text", 16.0, 100.0, 458.0, 160.0, "<p>A paragraph as wide as the page is, of some lines, to show how wide the text is set on it.</p>", &[]),
                block("Text", 16.0, 200.0, 200.0, 230.0, "<p>41 William Street, Princeton, New Jersey 08540</p>", &["41 William Street, Princeton,", "New Jersey 08540"]),
                block("Text", 16.0, 300.0, 440.0, 330.0, "<p>This book was composed in Times Roman by Princeton Editorial Associates, Inc., Scottsdale</p>", &["This book was composed in Times Roman by Princeton Editorial", "Associates, Inc., Scottsdale"]),
            ),
        );
        let scan = build(&[title, text], "book");
        let shown: Vec<(bool, String)> = scan
            .blks
            .iter()
            .filter_map(|b| match &b.block {
                Block::Heading { text, .. } | Block::Para(text) => Some((b.centered, emit_plain(text))),
                _ => None,
            })
            .collect();
        assert_eq!(
            shown,
            [
                (true, "QUANTUM FIELD THEORY /IN A NUTSHELL".to_string()),
                (true, "Kavli Institute for _Theoretical Physics_ /University of California".to_string()),
                // (the publisher at the foot of the title page is text)
                (true, "PRINCETON UNIVERSITY PRESS".to_string()),
                (false, "A paragraph as wide as the page is, of some lines, to show how wide the text is set on it.".to_string()),
                (false, "41 William Street, Princeton, /New Jersey 08540".to_string()),
                (false, "This book was composed in Times Roman by Princeton Editorial Associates, Inc., Scottsdale".to_string()),
            ]
        );
    }

    /// Text with "/" where a line ends and italics marked.
    fn emit_plain(inl: &[Inline]) -> String {
        inl.iter()
            .map(|i| match i {
                Inline::Text(t) => t.clone(),
                Inline::Emph(t) => format!("_{t}_"),
                Inline::Break => "/".into(),
                other => format!("{other:?}"),
            })
            .collect()
    }

    #[test]
    fn a_table_of_contents_is_read_line_by_line() {
        let lines: Vec<String> = ["Preface                 xi", "             PART I", "       MOTIVATION AND FOUNDATION", "I.1     Who Needs It? . . . . .   3", "I.2     Path Integrals         7"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let entries = contents(&lines).unwrap();
        let got: Vec<(String, &str)> = entries.iter().map(|e| (plain(&e.title), e.page.as_str())).collect();
        assert_eq!(
            got,
            [
                ("Preface".to_string(), "xi"),
                ("PART I MOTIVATION AND FOUNDATION".to_string(), ""),
                ("I.1 Who Needs It?".to_string(), "3"),
                ("I.2 Path Integrals".to_string(), "7")
            ]
        );
        assert!(contents(&["Energy in MeV".to_string(), "up 2.2".to_string(), "down 4.7".to_string()]).is_none());
    }

    #[test]
    fn page_numbers_are_read_from_the_heads() {
        assert_eq!(number_in("I.3. From Mattress to Field 17"), Some((false, 17)));
        assert_eq!(number_in("vii"), Some((true, 7)));
        assert_eq!(number_in("Motivation and Foundation"), None);
        assert_eq!(number_in("хi"), Some((true, 11)));
        // a running head read as text is a running head
        let p = parse_page(1, "page\t487.0\t842.0\nblock\tText\t18.0\t17.0\t120.0\t30.0\t1\t<p>xii Preface</p>\nblock\tText\t18.0\t50.0\t460.0\t90.0\t1\t<p>12 apples</p>");
        assert_eq!((p.blocks[0].kind.as_str(), p.blocks[1].kind.as_str()), ("PageHeader", "Text"));
        assert!(looks_like_code("for (i = 0; i < n; i++) { sum = sum + a[i]; } return sum;"));
        assert!(!looks_like_code("Abrahams, Elihu, 347 anyon(s), 293; interacting, 294; statistics Adler, Stephen, 251 between, 2"));
        assert_eq!(to_roman(14), "xiv");
    }
}
