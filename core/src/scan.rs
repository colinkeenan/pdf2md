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
const MARKS_SCRIPT: &str = include_str!("scan_marks.py");
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
    /// how far each of those lines is set in from the block's left edge,
    /// in letters (empty: not known)
    indents: Vec<usize>,
}

#[derive(Debug, Clone, Default)]
struct PageRead {
    /// the PDF's page number (1 = first sheet)
    number: u32,
    width: f32,
    height: f32,
    blocks: Vec<Raw>,
    /// the page's columns, where the PDF's own text says how they lie
    laid: Option<Laid>,
}

/// The columns of a page set in columns, from the places of the words in
/// the PDF's own text: for each column its lines from the top down, and
/// of each line whether it is set in (it goes on from the line before).
#[derive(Debug, Clone, Default)]
struct Laid {
    /// from where to where down the page the columns reach
    top: f32,
    bottom: f32,
    columns: Vec<Vec<(bool, String)>>,
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
                page.blocks.push(Raw { kind, left, top, right, bottom, lines, html, text: Vec::new(), indents: Vec::new() });
            }
            Some("line") => {
                if let Some(b) = page.blocks.last_mut() {
                    // (marker writes some line ends out as "\n": two lines)
                    let text = line.split_once('\t').map_or("", |x| x.1);
                    b.text.extend(text.split("\\n").map(str::trim).filter(|t| !t.is_empty()).map(str::to_string));
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
    /// the displayed equations that are a block of their own: key, page
    /// of the PDF, where on it (in points)
    equations: Vec<(String, u32, Figure)>,
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
    // (a page read before the reader handed on the lines of its blocks,
    // with a block of many lines on it, a column of an index: read again)
    for n in first..=last {
        if std::fs::read_to_string(file(n)).is_ok_and(|t| lacks_lines(&t)) {
            let _ = std::fs::remove_file(file(n));
        }
    }
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
    // Footnotes whose marks marker did not read: looked for on the
    // pictures of their pages.
    find_marks(path, &dir, &py, &mut pages);
    // A line of which marker read too little for the room it takes
    // ("cm." in a box four times as wide: the page says "p.   cm."): the
    // PDF's own words, if they are marker's and more.
    for page in pages.iter_mut() {
        let number = page.number;
        for b in page.blocks.iter_mut().filter(|b| b.kind == "Text" && b.lines <= 1 && b.bottom - b.top <= 24.0) {
            let inl = all_inline(&b.html);
            let read = plain(&inl);
            let letters = read.chars().count();
            if letters == 0 || letters > 12 || !inl.iter().all(|i| matches!(i, Inline::Text(_))) || b.right - b.left < 12.0 * letters as f32 {
                continue;
            }
            // (looked for a little to the left and right of marker's box, too)
            let wider = Raw { left: b.left - 16.0, right: b.right + 6.0, top: b.top - 3.0, bottom: b.bottom + 5.0, ..b.clone() };
            let there = area_text(path, number, &wider);
            let more = there.split_whitespace().count() > read.split_whitespace().count() && there.chars().count() <= 4 * letters + 8;
            if more && there.to_lowercase().ends_with(&read.to_lowercase()) {
                b.html = there.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;");
            }
        }
    }
    // A page in columns (an index): how its columns lie, by the words
    // of the PDF's own text.
    for page in pages.iter_mut() {
        let many_lines = page.blocks.iter().any(|b| b.lines >= 20 && b.right - b.left > 0.6 * page.width);
        if columns(page).is_some() || many_lines {
            page.laid = lay_columns(page, &page_words(path, page.number));
        }
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
            // (and for a heading of several lines: the title of a part)
            if b.lines >= 2 || (all && b.bottom - b.top > 20.0) || (b.kind == "SectionHeader" && b.bottom - b.top > 30.0) {
                // (a heading's last line may reach a little under marker's box)
                let roomy = if b.kind == "SectionHeader" { Raw { left: b.left - 3.0, right: b.right + 3.0, top: b.top - 2.0, bottom: b.bottom + 6.0, ..b.clone() } } else { b.clone() };
                b.text = area_lines(path, number, &roomy);
            }
        }
    }
    let mut scan = build(&pages, &format!("{:016x}", fnv(dir.to_string_lossy().as_bytes())));
    check_equations(path, &mut scan);
    Ok(scan)
}

/// Is this a page file of the first kind (no lines of blocks in it) with
/// a block of text of many lines?
fn lacks_lines(text: &str) -> bool {
    let mut lines = text.lines().peekable();
    if lines.peek().is_some_and(|l| l.split('\t').count() >= 4) {
        return false;
    }
    while let Some(l) = lines.next() {
        let f: Vec<&str> = l.splitn(8, '\t').collect();
        let many = f.len() >= 7 && f[0] == "block" && !matches!(f[1], "Table" | "Form" | "TableOfContents") && f[6].parse::<usize>().is_ok_and(|n| n >= 6);
        if many && !lines.peek().is_some_and(|n| n.starts_with("line\t")) {
            return true;
        }
    }
    false
}

/// A word of the PDF's own text with its place on the page, in points.
#[derive(Debug, Clone)]
struct Word {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
    text: String,
}

/// The words the PDF itself has on a page (`pdftotext -bbox`): the text an
/// earlier reading left under the picture of a scanned page.
fn page_words(path: &Path, page: u32) -> Vec<Word> {
    let page = page.to_string();
    let Ok(out) = Command::new("pdftotext").args(["-bbox", "-f", &page, "-l", &page]).arg(path).arg("-").output() else { return Vec::new() };
    let text = String::from_utf8_lossy(&out.stdout);
    let mut words = Vec::new();
    for w in text.split("<word ").skip(1) {
        let Some((tag, rest)) = w.split_once('>') else { continue };
        let num = |name: &str| tag.split(&format!("{name}=\"")).nth(1).and_then(|v| v.split('"').next()).and_then(|v| v.parse::<f32>().ok());
        let (Some(left), Some(top), Some(right), Some(bottom)) = (num("xMin"), num("yMin"), num("xMax"), num("yMax")) else { continue };
        let text = unescape(rest.split("</word>").next().unwrap_or(""));
        if !text.trim().is_empty() {
            words.push(Word { left, top, right, bottom, text: text.trim().to_string() });
        }
    }
    words
}

/// How alike two lines of text are, 0 to 1: the share of the pairs of
/// letters they have in common (letters and digits only, small).
fn alike(a: &str, b: &str) -> f32 {
    let pairs = |t: &str| -> Vec<(char, char)> {
        let c: Vec<char> = t.chars().filter(|c| c.is_alphanumeric()).flat_map(char::to_lowercase).collect();
        c.windows(2).map(|w| (w[0], w[1])).collect()
    };
    let (pa, mut pb) = (pairs(a), pairs(b));
    if pa.is_empty() || pb.is_empty() {
        return 0.0;
    }
    let total = (pa.len() + pb.len()) as f32;
    let mut common = 0;
    for p in pa {
        if let Some(k) = pb.iter().position(|q| *q == p) {
            pb.swap_remove(k);
            common += 1;
        }
    }
    2.0 * common as f32 / total
}

/// The columns of a page whose text stands in two columns, by the words
/// of the PDF's own text (None: it has none there, or they do not stand
/// in two columns). `words`: those words. The lines take marker's reading
/// of them where one of its lines is plainly the same line: marker reads
/// better, but gives a column's lines without saying which are set in,
/// and at times deals the lines of two columns into one.
fn lay_columns(page: &PageRead, words: &[Word]) -> Option<Laid> {
    let side_by_side = columns(page);
    let column_block = |i: usize, b: &Raw| -> bool {
        let texty = matches!(b.kind.as_str(), "Text" | "TextInlineMath" | "ListItem" | "Code");
        // (a block of a column, or one that marker laid over both)
        let over_both = b.right - b.left > 0.6 * page.width && b.lines >= 20;
        texty && b.bottom > 0.06 * page.height && (over_both || side_by_side.as_ref().is_some_and(|(l, r)| l.contains(&i) || r.contains(&i)))
    };
    let blocks: Vec<&Raw> = page.blocks.iter().enumerate().filter(|(i, b)| column_block(*i, b)).map(|(_, b)| b).collect();
    if blocks.is_empty() {
        return None;
    }
    let (top, bottom) = (blocks.iter().map(|b| b.top).fold(f32::MAX, f32::min) - 3.0, blocks.iter().map(|b| b.bottom).fold(0.0, f32::max) + 5.0);
    let (left, right) = (blocks.iter().map(|b| b.left).fold(f32::MAX, f32::min) - 4.0, blocks.iter().map(|b| b.right).fold(0.0, f32::max) + 4.0);
    let middle = |w: &Word| ((w.left + w.right) / 2.0, (w.top + w.bottom) / 2.0);
    let words: Vec<&Word> = words.iter().filter(|w| (left..right).contains(&middle(w).0) && (top..bottom).contains(&middle(w).1)).collect();
    if words.len() < 20 {
        return None;
    }
    // the gutter: where, about the middle, the fewest words lie across
    let across = |x: f32| words.iter().filter(|w| w.left < x && w.right > x).count();
    let (from, to) = (left + 0.3 * (right - left), left + 0.7 * (right - left));
    let centre = (left + right) / 2.0;
    let gutter = (0..=((to - from) / 2.0) as usize).map(|k| from + 2.0 * k as f32).min_by(|a, b| across(*a).cmp(&across(*b)).then((a - centre).abs().total_cmp(&(b - centre).abs())))?;
    let (l, r): (Vec<&Word>, Vec<&Word>) = words.iter().partition(|w| middle(w).0 < gutter);
    if across(gutter) * 50 > words.len() || l.len() * 5 < words.len() || r.len() * 5 < words.len() {
        return None;
    }
    // marker's lines of these blocks, each to be used once
    let mut read: Vec<String> = blocks.iter().flat_map(|b| if b.text.len() >= 2 { b.text.clone() } else { vec![plain(&all_inline(&b.html))] }).collect();
    let mut column = |mut words: Vec<&Word>| -> Vec<(bool, String)> {
        // the lines: words at one height, from the left
        words.sort_by(|a, b| middle(a).1.total_cmp(&middle(b).1));
        let mut lines: Vec<Vec<&Word>> = Vec::new();
        for w in words {
            match lines.last_mut() {
                Some(line) if middle(w).1 - middle(line[0]).1 < 0.6 * (line[0].bottom - line[0].top).max(4.0) => line.push(w),
                _ => lines.push(vec![w]),
            }
        }
        for line in lines.iter_mut() {
            line.sort_by(|a, b| a.left.total_cmp(&b.left));
        }
        // where the column's lines start: the leftmost place where a
        // good many of them do (lines set in can be more than the others;
        // a stray word left of the column is no line start)
        let mut starts: Vec<f32> = lines.iter().map(|l| l[0].left).collect();
        starts.sort_by(f32::total_cmp);
        let many = (starts.len() / 10).max(2).min(starts.len());
        let edge = starts.iter().copied().find(|s| starts.iter().filter(|o| (*o - s).abs() <= 2.0).count() >= many).unwrap_or(starts[0]);
        lines
            .iter()
            .map(|line| {
                let own = line.iter().map(|w| w.text.as_str()).collect::<Vec<_>>().join(" ");
                let best = read.iter().enumerate().map(|(k, t)| (alike(t, &own), k)).max_by(|a, b| a.0.total_cmp(&b.0));
                let text = match best {
                    Some((score, k)) if score >= 0.72 => read.swap_remove(k),
                    _ => own,
                };
                (line[0].left > edge + 4.0, text)
            })
            .collect()
    };
    let columns = vec![column(l), column(r)];
    Some(Laid { top, bottom, columns })
}

/// The displayed equations read a second time, by the math recognizer
/// from a picture of each (cut from its page at 150 dots per inch), so
/// that the ones the two readers read differently are listed to check.
fn check_equations(path: &Path, scan: &mut Scan) {
    second_readings(path, scan);
    // A formula marker did not finish (a matrix cut off in its third
    // row, a table of numbers without end), or wrote words into: the
    // second reading of it, if that one is whole.
    for b in scan.blks.iter_mut() {
        if let Block::Equation { latex, source: Some(k), .. } = &mut b.block {
            if !whole(latex) && crate::ocr::read_correction(k).is_none() {
                if let Some(second) = crate::ocr::second_reading(k).filter(|s| whole(s)) {
                    *latex = tame(&second);
                }
            }
        }
    }
}

/// Is this LaTeX a whole formula: what it opens it closes, and there is
/// no "$" in it (a sentence the model ran on into)?
fn whole(latex: &str) -> bool {
    let plain = latex.replace("\\{", "").replace("\\}", "").replace("\\$", "");
    let count = |t: &str| plain.matches(t).count();
    count("{") == count("}") && count("\\begin{") == count("\\end{") && count("\\left") == count("\\right") && !plain.contains('$') && !plain.trim().is_empty()
}

/// In a formula set in rows, a bracket that grows with what it holds
/// (\left( … \right)) must close in the cell it opens in, or the formula
/// does not render: where one does not, the brackets of that cell are
/// plain ones.
fn tame(latex: &str) -> String {
    if !latex.contains("\\\\") && !latex.contains('&') {
        return latex.to_string();
    }
    let mut out = String::with_capacity(latex.len());
    let mut rest = latex;
    // (cell after cell: up to the next "&" or "\\")
    loop {
        let end = [rest.find('&'), rest.find("\\\\")].into_iter().flatten().min();
        let cell = &rest[..end.unwrap_or(rest.len())];
        if cell.matches("\\left").count() != cell.matches("\\right").count() {
            // ("\left." and "\right." are nothing at all)
            out.push_str(&cell.replace("\\left.", "").replace("\\right.", "").replace("\\left", "").replace("\\right", ""));
        } else {
            out.push_str(cell);
        }
        let Some(end) = end else { break };
        let mark = if rest[end..].starts_with('&') { 1 } else { 2 };
        out.push_str(&rest[end..end + mark]);
        rest = &rest[end + mark..];
    }
    out
}

/// The displayed equations read a second time (see [`check_equations`]).
fn second_readings(path: &Path, scan: &Scan) {
    // (key -> the LaTeX the book gets)
    let latex: std::collections::HashMap<&str, &str> =
        scan.blks.iter().filter_map(|b| if let Block::Equation { latex, source: Some(k), .. } = &b.block { Some((k.as_str(), latex.as_str())) } else { None }).collect();
    let cut = |page: u32, area: &Figure| -> Option<Vec<u8>> {
        let k = 150.0 / 72.0;
        let arg = |v: f32| ((v * k).round() as i64).max(0).to_string();
        let page = page.to_string();
        // (a little room around it: marker's boxes are tight)
        let out = Command::new("pdftoppm")
            .args(["-f", &page, "-l", &page, "-r", "150", "-png", "-x", &arg(area.left - 4.0), "-y", &arg(area.top - 3.0), "-W", &arg(area.right - area.left + 8.0), "-H", &arg(area.bottom - area.top + 6.0)])
            .arg(path)
            .output()
            .ok()?;
        (out.status.success() && !out.stdout.is_empty()).then_some(out.stdout)
    };
    let todo: Vec<&(String, u32, Figure)> = scan.equations.iter().filter(|e| latex.contains_key(e.0.as_str())).collect();
    // cut out on all processors
    let next = std::sync::atomic::AtomicUsize::new(0);
    let done: std::sync::Mutex<Vec<(String, String, Vec<u8>)>> = Default::default();
    std::thread::scope(|scope| {
        for _ in 0..std::thread::available_parallelism().map_or(1, |n| n.get()).min(todo.len().max(1)) {
            scope.spawn(|| loop {
                let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let Some((key, page, area)) = todo.get(i) else { break };
                if let Some(png) = cut(*page, area) {
                    done.lock().unwrap_or_else(|e| e.into_inner()).push((key.clone(), latex[key.as_str()].to_string(), png));
                }
            });
        }
    });
    let mut equations = done.into_inner().unwrap_or_default();
    equations.sort_by(|a, b| a.0.cmp(&b.0));
    crate::ocr::second_readings(&equations);
}

/// The footnotes of each page whose mark is not in the page's text: their
/// labels ("1", "3").
fn unmarked(page: &PageRead) -> Vec<String> {
    let mut raised: Vec<String> = Vec::new();
    for b in page.blocks.iter().filter(|b| b.kind != "Footnote" && b.kind != "PageHeader" && b.kind != "PageFooter") {
        raised.extend(all_inline(&b.html).into_iter().filter_map(|i| if let Inline::Sup(t) = i { Some(t) } else { None }));
    }
    let mut out = Vec::new();
    for b in page.blocks.iter().filter(|b| b.kind == "Footnote") {
        if let Some((label, _)) = footnote(&b.html) {
            if label.chars().all(|c| c.is_ascii_digit()) && !raised.contains(&label) && !out.contains(&label) {
                out.push(label);
            }
        }
    }
    out
}

/// The small raised number that points to a footnote is often not read
/// by marker. Where a page has a footnote without its mark in the text,
/// the mark is looked for on the picture of the page, and of the page
/// before and the page after (`scan_marks.py`: a small piece of ink set
/// high, right behind a word, that reads as the note's number), and put
/// into the text behind the word it follows there.
fn find_marks(path: &Path, dir: &Path, py: &Path, pages: &mut [PageRead]) {
    let missing: Vec<Vec<String>> = pages.iter().map(unmarked).collect();
    if missing.iter().all(Vec::is_empty) {
        return;
    }
    // where a page's footnotes begin: no mark is looked for below
    let limit = |p: &PageRead| p.blocks.iter().filter(|b| b.kind == "Footnote").map(|b| b.top).fold(0.0f32, |a, t| if a == 0.0 { t } else { a.min(t) });
    // what to look for on which page: its own unmarked notes, and those of
    // the pages beside it
    let mut wanted: Vec<Vec<String>> = vec![Vec::new(); pages.len()];
    for (i, labels) in missing.iter().enumerate() {
        let (from, to) = (i.saturating_sub(1), (i + 2).min(pages.len()));
        for of_page in &mut wanted[from..to] {
            for l in labels {
                if !of_page.contains(l) {
                    of_page.push(l.clone());
                }
            }
        }
    }
    let file = |n: u32| dir.join(format!("m{n:04}.txt"));
    let head = |i: usize| {
        let mut labels = wanted[i].clone();
        labels.sort();
        format!("want\t{}\t{:.0}", labels.join(","), limit(&pages[i]))
    };
    // (looked at before, for the same marks: kept)
    let todo: Vec<usize> = (0..pages.len()).filter(|i| !wanted[*i].is_empty() && std::fs::read_to_string(file(pages[*i].number)).ok().and_then(|t| t.lines().next().map(str::to_string)) != Some(head(*i))).collect();
    if !todo.is_empty() {
        let script = dir.join("scan_marks.py");
        if std::fs::write(&script, MARKS_SCRIPT).is_err() {
            return;
        }
        eprintln!("Looking for the marks of footnotes on {} pages…", todo.len());
        progress::report(Step::Marks, 0, todo.len());
        let requests: String = todo.iter().map(|i| format!("{}\n", head(*i).replacen("want", &pages[*i].number.to_string(), 1))).collect();
        let child = Command::new(py).arg(&script).arg(path).arg(dir).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::piped()).spawn();
        let Ok(mut child) = child else { return };
        if let Some(mut pipe) = child.stdin.take() {
            use std::io::Write;
            let _ = pipe.write_all(requests.as_bytes());
        }
        if let Some(pipe) = child.stderr.take() {
            use std::io::BufRead;
            for line in std::io::BufReader::new(pipe).lines().map_while(Result::ok) {
                if let Some((done, total)) = line.trim().strip_prefix("marks ").and_then(|c| c.split_once('/')).and_then(|(d, t)| Some((d.parse::<usize>().ok()?, t.parse::<usize>().ok()?))) {
                    progress::report(Step::Marks, done.min(total.saturating_sub(1)), total);
                }
            }
        }
        let _ = child.wait();
        progress::report(Step::Marks, todo.len(), todo.len());
    }
    // the marks found on each page: label, and the words up to the one it follows
    let found: Vec<Vec<(String, Vec<String>)>> = (0..pages.len())
        .map(|i| {
            let text = std::fs::read_to_string(file(pages[i].number)).unwrap_or_default();
            text.lines()
                .filter_map(|l| {
                    let mut f = l.split('\t');
                    (f.next() == Some("mark")).then(|| Some((f.next()?.to_string(), f.next()?.split_whitespace().map(str::to_string).collect())))?
                })
                .collect()
        })
        .collect();
    let mut placed = 0;
    for (i, labels) in missing.iter().enumerate() {
        for label in labels {
            // on the note's own page first, then the page before, the page after
            // (not a page that has a note of that number itself: its mark is that note's)
            let own = |j: usize| pages[j].blocks.iter().any(|b| b.kind == "Footnote" && footnote(&b.html).is_some_and(|(l, _)| &l == label));
            let mut tries = vec![i];
            if i > 0 && !own(i - 1) {
                tries.push(i - 1);
            }
            if i + 1 < pages.len() && !own(i + 1) {
                tries.push(i + 1);
            }
            'pages: for j in tries {
                for (l, words) in &found[j] {
                    if l == label && place_mark(&mut pages[j], words, label) {
                        placed += 1;
                        break 'pages;
                    }
                }
            }
        }
    }
    let unmarked: usize = missing.iter().map(Vec::len).sum();
    eprintln!("  {placed} of {unmarked} footnote marks that the page reader missed were found on the pages' pictures");
}

/// The words of a block's html that stand outside tags and formulas, each
/// with the place right behind it and the punctuation that follows it.
fn text_words(html: &str) -> Vec<(String, usize)> {
    let mut out = Vec::new();
    let b = html.as_bytes();
    let mut i = 0;
    let mut word = String::new();
    while i < html.len() {
        if b[i] == b'<' {
            if !word.is_empty() {
                out.push((std::mem::take(&mut word), i));
            }
            // (a formula: skipped whole)
            let end = if html[i..].starts_with("<math") { html[i..].find("</math>").map(|e| i + e + 7) } else { html[i..].find('>').map(|e| i + e + 1) };
            i = end.unwrap_or(html.len());
            continue;
        }
        let c = html[i..].chars().next().unwrap();
        if c.is_alphanumeric() {
            word.extend(c.to_lowercase());
        } else if !word.is_empty() {
            out.push((std::mem::take(&mut word), i));
        }
        i += c.len_utf8();
    }
    if !word.is_empty() {
        out.push((word, html.len()));
    }
    // behind the punctuation that closes the word (a mark stands behind the full stop)
    for (_, at) in out.iter_mut() {
        while html[*at..].starts_with(['.', ',', ';', ':', '!', '?', ')', '"', '”', '’', '\'']) {
            *at += html[*at..].chars().next().map_or(1, char::len_utf8);
        }
    }
    out
}

/// Put the mark of footnote `label` into the text of the page behind the
/// word it follows: the last of `words` (what Tesseract read there; the
/// word before it tells which one, where the word comes more than once).
/// Only if that place is the only one; says whether it did.
fn place_mark(page: &mut PageRead, words: &[String], label: &str) -> bool {
    let norm = |w: &str| w.chars().filter(|c| c.is_alphanumeric()).flat_map(char::to_lowercase).collect::<String>();
    let Some(last) = words.last().map(|w| norm(w)).filter(|w| !w.is_empty()) else { return false };
    let before = words.len().checked_sub(2).map(|k| norm(&words[k])).filter(|w| !w.is_empty());
    let texty = |b: &Raw| is_text(b) || b.kind == "Equation";
    // (block, place) of each fit, with the word before it or without
    let fits = |with_before: bool| -> Vec<(usize, usize)> {
        let mut out = Vec::new();
        for (k, b) in page.blocks.iter().enumerate().filter(|(_, b)| texty(b)) {
            let ws = text_words(&b.html);
            for (n, (w, at)) in ws.iter().enumerate() {
                let prev_ok = !with_before || before.as_ref().is_none_or(|p| n > 0 && &ws[n - 1].0 == p);
                if *w == last && prev_ok {
                    out.push((k, *at));
                }
            }
        }
        out
    };
    let mut places = fits(true);
    if places.is_empty() {
        places = fits(false);
    }
    let [(k, at)] = places[..] else { return false };
    page.blocks[k].html.insert_str(at, &format!("<sup>{label}</sup>"));
    true
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
    for (a, b) in [("&lt;", "<"), ("&gt;", ">"), ("&quot;", "\""), ("&#39;", "'"), ("&#x27;", "'"), ("&apos;", "'"), ("&nbsp;", " "), ("&amp;", "&")] {
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
fn tidy(inl: Vec<Inline>) -> Vec<Inline> {
    // A formula that goes on in the next line of the book comes as two
    // ("$a =$" and "$b$"): one formula. (Written one right behind the
    // other they would even read as "$$", the mark of a displayed one.)
    let mut joined: Vec<Inline> = Vec::new();
    for i in inl {
        let n = joined.len();
        let goes_on = |m: &str| m.trim_end().ends_with(['=', '+', '-', ',', '(']) || m.trim_end().ends_with("\\cdot") || m.trim_end().ends_with("\\times");
        match (&i, n) {
            (Inline::Math(b), n) if n >= 1 && matches!(&joined[n - 1], Inline::Math(_)) => {
                let Inline::Math(a) = &mut joined[n - 1] else { unreachable!() };
                a.push(' ');
                a.push_str(b);
            }
            (Inline::Math(b), n) if n >= 2 && matches!((&joined[n - 2], &joined[n - 1]), (Inline::Math(a), Inline::Text(t)) if t.trim().is_empty() && goes_on(a)) => {
                joined.pop();
                let Some(Inline::Math(a)) = joined.last_mut() else { unreachable!() };
                a.push(' ');
                a.push_str(b);
            }
            _ => joined.push(i),
        }
    }
    let mut inl = joined;
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
    let mut latex = tame(&rows(maths));
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

// ---- tables -------------------------------------------------------------

/// The rows and cells of a table as marker read it (`<table>` … `<td>`).
/// None if they do not hold together as a table: fewer than two rows or
/// two columns, rows of unlike length, or too many cells with nothing in
/// them (marker sees tables in empty pages).
fn read_table(html: &str) -> Option<Vec<Vec<crate::Cell>>> {
    let mut rows: Vec<Vec<crate::Cell>> = Vec::new();
    for row in html.split("<tr").skip(1) {
        let row = row.split("</tr>").next().unwrap_or("");
        let mut cells = Vec::new();
        for (k, piece) in row.split("<td").flat_map(|p| p.split("<th")).enumerate() {
            // (what stands before the first cell is the row's own tag)
            if k == 0 {
                continue;
            }
            let Some((tag, rest)) = piece.split_once('>') else { continue };
            let inside = rest.split("</td>").next().unwrap_or("").split("</th>").next().unwrap_or("");
            let span = tag.split("colspan=\"").nth(1).and_then(|v| v.split('"').next()).and_then(|v| v.parse::<usize>().ok()).unwrap_or(1).max(1);
            cells.push(crate::Cell { text: cell_inlines(inside), span, ..Default::default() });
        }
        if !cells.is_empty() {
            rows.push(cells);
        }
    }
    let width = |r: &Vec<crate::Cell>| r.iter().map(|c| c.span).sum::<usize>();
    let columns = rows.iter().map(width).max().unwrap_or(0);
    let cells: usize = rows.iter().map(Vec::len).sum();
    let filled = rows.iter().flatten().filter(|c| has_words(&c.text)).count();
    let even = rows.iter().all(|r| width(r) + 1 >= columns);
    (rows.len() >= 2 && columns >= 2 && even && filled * 5 >= cells * 3).then_some(rows)
}

/// What a cell of marker's table holds. marker writes a cell's formulas
/// as LaTeX among its words, unmarked ("Maxwell f^2 \\sim a \\partial^2 a"):
/// a run of pieces with "\\", "^", "_" or "=" in them is a formula, and
/// the single letters and numbers beside them belong to it.
fn cell_inlines(html: &str) -> Vec<Inline> {
    let marked = all_inline(html);
    let mut out: Vec<Inline> = Vec::new();
    for i in marked {
        let Inline::Text(t) = &i else {
            out.push(i);
            continue;
        };
        let words: Vec<&str> = t.split_whitespace().collect();
        let sure = |w: &str| w.contains('\\') || w.contains('^') || w.contains('_') || w.contains('=') || w == "~";
        let small = |w: &str| {
            let core = w.trim_matches(|c: char| "()[],.".contains(c));
            core.chars().count() <= 1 || core.chars().all(|c| c.is_ascii_digit()) || ["+", "-", "−", "/"].contains(&w)
        };
        // which words are in a formula: the sure ones, and small ones that
        // touch them
        let mut math: Vec<bool> = words.iter().map(|w| sure(w)).collect();
        for _ in 0..words.len() {
            for k in 0..words.len() {
                let beside = (k > 0 && math[k - 1]) || math.get(k + 1).copied().unwrap_or(false);
                if !math[k] && small(words[k]) && beside {
                    math[k] = true;
                }
            }
        }
        let mut k = 0;
        while k < words.len() {
            let end = (k..words.len()).find(|j| math[*j] != math[k]).unwrap_or(words.len());
            let piece = words[k..end].join(" ");
            // (the space the words had before them, or between two pieces)
            if !out.is_empty() && (k > 0 || t.starts_with(char::is_whitespace)) {
                push_text(&mut out, Inline::Text(" ".into()));
            }
            if math[k] {
                out.push(Inline::Math(piece));
            } else {
                push_text(&mut out, Inline::Text(piece));
            }
            k = end;
        }
        if t.ends_with(char::is_whitespace) && !words.is_empty() {
            push_text(&mut out, Inline::Text(" ".into()));
        }
    }
    tidy(out)
}

// ---- symbols in sentences ----------------------------------------------

/// marker writes most formulas in a sentence as formulas, but leaves the
/// plainest as text: "J(x)", a lone "m", "h = 1". They are found by their
/// shape, with caution (a letter can be an initial, a label, a unit):
///
/// * a letter or two with what they are a function of in brackets right
///   behind: "J(x)", "W(J)", "D(x-y)", "SU(2)";
/// * a single letter other than "a", "A", "I" — not an initial ("S.
///   Coleman"), not a label ("(b)", "b)", "Appendix B", "Part V"), not a
///   unit behind a number;
/// * a short relation written without spaces ("m=0");
/// * and what such a symbol is set equal to, added to, and so on, with
///   spaces: "h = 1", "x + y".
fn symbols(inl: Vec<Inline>) -> Vec<Inline> {
    const BEFORE_LABEL: [&str; 16] =
        ["part", "chapter", "appendix", "appendixes", "section", "volume", "vol", "table", "figure", "fig", "exercise", "problem", "type", "class", "case", "step"];
    const OPERATORS: [&str; 10] = ["=", "+", "-", "−", "<", ">", "≈", "→", "×", "≡"];
    let short_letters = |t: &str| t.split(|c: char| !c.is_alphabetic()).all(|run| run.chars().count() <= 2);
    // (prefix, the word itself, suffix) of a word with its punctuation
    let cut = |w: &str| -> (String, String, String) {
        let open: String = w.chars().take_while(|c| "(\"“‘[".contains(*c)).collect();
        let rest = &w[open.len()..];
        let mut core = rest.trim_end_matches(|c: char| ".,;:!?\"”’]".contains(c)).to_string();
        // (a closing bracket that has no opening one in the word is punctuation)
        while core.ends_with(')') && core.matches(')').count() > core.matches('(').count() {
            core.pop();
            core = core.trim_end_matches(|c: char| ".,;:!?\"”’]".contains(c)).to_string();
        }
        let close = rest[core.len()..].to_string();
        (open, core, close)
    };
    let function = |core: &str| -> bool {
        let Some(at) = core.find('(') else { return false };
        let (head, args) = (&core[..at], &core[at..]);
        let head_ok = (1..=3).contains(&head.chars().count()) && head.chars().all(|c| c.is_ascii_alphabetic()) && (head.chars().count() <= 2 || head.chars().all(|c| c.is_ascii_uppercase()));
        let inside = args.trim_start_matches('(').trim_end_matches(')');
        head_ok
            && args.ends_with(')')
            && args.matches('(').count() == args.matches(')').count()
            && !inside.is_empty()
            && args.chars().count() <= 16
            && args.chars().all(|c| c.is_alphanumeric() || "()+-−*/=^_'′,.".contains(c))
            && short_letters(args)
    };
    let relation = |core: &str| -> bool {
        let n = core.chars().count();
        (3..=12).contains(&n)
            && core.contains('=')
            && !core.starts_with('=')
            && !core.ends_with('=')
            && core.chars().all(|c| c.is_alphanumeric() || "()+-−*/=^_'′".contains(c))
            && short_letters(core)
            && core.chars().any(char::is_alphabetic)
    };
    let tex = |t: &str| t.replace('′', "'").replace('−', "-");
    let mut out: Vec<Inline> = Vec::new();
    for i in inl {
        let Inline::Text(text) = &i else {
            out.push(i);
            continue;
        };
        let words: Vec<&str> = text.split(' ').collect();
        let parts: Vec<(String, String, String)> = words.iter().map(|w| cut(w)).collect();
        let number = |k: usize| parts.get(k).is_some_and(|p| !p.1.is_empty() && p.1.chars().all(|c| c.is_ascii_digit() || c == '.'));
        // which words are symbols by themselves
        let symbol: Vec<bool> = (0..words.len())
            .map(|k| {
                let (open, core, close) = &parts[k];
                if function(core) || relation(core) {
                    return true;
                }
                let mut c = core.chars();
                let (Some(letter), None) = (c.next(), c.next()) else { return false };
                if !letter.is_ascii_alphabetic() || "aAI".contains(letter) {
                    return false;
                }
                let before = k.checked_sub(1).map(|j| parts[j].1.to_lowercase()).unwrap_or_default();
                let next_capital = parts.get(k + 1).is_some_and(|p| p.1.starts_with(char::is_uppercase));
                let initial = letter.is_uppercase() && close.starts_with('.') && (next_capital || k == 0);
                let label = (open.contains('(') || close.starts_with(')')) || BEFORE_LABEL.contains(&before.as_str());
                let unit = k > 0 && number(k - 1) && parts[k - 1].2.is_empty();
                // ("Figure IV.7.1a and b")
                let sublabel = k >= 2 && before == "and" && parts[k - 2].1.chars().rev().nth(1).is_some_and(|c| c.is_ascii_digit());
                !(initial || label || unit || sublabel)
            })
            .collect();
        // "a" is a symbol where it stands with one: "a and b", "a, b"
        let mut symbol = symbol;
        for k in 0..words.len() {
            let list = |j: usize| words.get(j).is_some_and(|w| matches!(*w, "and" | "or" | "to"));
            let single = |j: usize| symbol.get(j).copied().unwrap_or(false) && parts[j].1.chars().count() == 1;
            if parts[k].1 == "a" && parts[k].0.is_empty() {
                let ahead = (list(k + 1) && single(k + 2) && parts[k].2.is_empty()) || (parts[k].2 == "," && single(k + 1));
                let behind = k >= 2 && list(k - 1) && single(k - 2);
                symbol[k] = ahead || behind;
            }
        }
        let mut piece = String::new();
        let flush = |piece: &mut String, out: &mut Vec<Inline>| {
            if !piece.is_empty() {
                push_text(out, Inline::Text(std::mem::take(piece)));
            }
        };
        let mut k = 0;
        while k < words.len() {
            if !symbol[k] {
                piece.push_str(words[k]);
                if k + 1 < words.len() {
                    piece.push(' ');
                }
                k += 1;
                continue;
            }
            // the symbol, and what it is set equal to, added to …
            let (open, core, mut close) = parts[k].clone();
            let mut formula = tex(&core);
            let mut end = k;
            while close.is_empty() && end + 2 < words.len() && OPERATORS.contains(&words[end + 1]) && (symbol[end + 2] || number(end + 2)) && parts[end + 2].0.is_empty() {
                formula.push_str(&format!(" {} {}", tex(words[end + 1]), tex(&parts[end + 2].1)));
                close = parts[end + 2].2.clone();
                end += 2;
            }
            piece.push_str(&open);
            flush(&mut piece, &mut out);
            out.push(Inline::Math(formula));
            piece.push_str(&close);
            if end + 1 < words.len() {
                piece.push(' ');
            }
            k = end + 1;
        }
        flush(&mut piece, &mut out);
    }
    out
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
        // ("x Contents": one letter beside a word or two)
        (alone || (crate::pages::is_roman(w) && (w.len() >= 2 || words.len() <= 3))).then(|| roman(&w.to_lowercase())).flatten().map(|n| (true, n))
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

/// A contents entry as these functions make it.
fn entry(title: String, page: String, left: f32, part: bool) -> TocEntry {
    TocEntry { title: vec![Inline::Text(title)], page, left, size: if part { BODY + 3.0 } else { BODY }, columns: false, number: String::new(), label: Vec::new(), part }
}

/// Is this what stands in the first column of a table of contents: the
/// number of a chapter ("I.5", "12", "A", "VIII.2")?
fn chapter_mark(t: &str) -> bool {
    let n = t.chars().count();
    (1..=8).contains(&n)
        && !t.contains(' ')
        && t.chars().all(|c| c.is_ascii_alphanumeric() || c == '.')
        && (t.chars().any(|c| c.is_ascii_digit()) || (n <= 4 && t.chars().all(|c| c.is_ascii_uppercase() || c == '.')))
}

/// The entries of a table of contents given line by line, laid out as on
/// the page ("I.1     Who Needs It? . . .     3"); None if the lines are no
/// such table. A number that stands apart in front of a title is the
/// entry's number (the first column of contents set in columns).
fn contents(lines: &[String]) -> Option<Vec<TocEntry>> {
    let mut entries: Vec<TocEntry> = Vec::new();
    let mut with_page = 0;
    for line in lines {
        let indent = line.chars().take_while(|c| *c == ' ').count() as f32;
        // what stands apart on the line (two spaces or more between)
        let leaders = |w: &&str| !w.chars().all(|c| c == '.' || c == '·' || c == '…' || c == ' ');
        let mut fields: Vec<String> =
            line.split("  ").map(str::trim).filter(|f| !f.is_empty()).map(|f| f.split_whitespace().filter(leaders).collect::<Vec<_>>().join(" ")).filter(|f| !f.is_empty()).collect();
        // the page: the last of them, or the last word
        let mut page = None;
        if fields.len() > 1 && fields.last().is_some_and(|f| crate::pages::is_page_number(f)) {
            page = fields.pop();
        } else if let Some((rest, last)) = fields.last().and_then(|f| f.rsplit_once(' ')).map(|(a, b)| (a.to_string(), b.to_string())) {
            if crate::pages::is_page_number(&last) {
                *fields.last_mut().unwrap() = rest;
                page = Some(last);
            }
        }
        let number = if fields.len() >= 2 && indent < 4.0 && chapter_mark(&fields[0]) { fields.remove(0) } else { String::new() };
        let title = fields.join(" ").trim_end_matches(['.', ' ']).to_string();
        if !title.chars().any(char::is_alphabetic) {
            continue;
        }
        with_page += page.is_some() as usize;
        // (a line in capitals without a page: the name of a part, which
        // goes on under "PART I")
        let part = page.is_none() && number.is_empty() && !title.chars().any(char::is_lowercase);
        if let (true, Some(before)) = (part, entries.last_mut().filter(|e| e.part && names_chapter(&crate::plain(&e.title)))) {
            let first = crate::plain(&before.title);
            before.label = vec![Inline::Text(format!("{first} ")), Inline::Break, Inline::Text(title.clone())];
            before.title = vec![Inline::Text(format!("{first} {title}"))];
            continue;
        }
        // (a title too long for its line goes on in the next, where its page is)
        if let Some(before) = entries.last_mut().filter(|e| e.page.is_empty() && !e.part && !part && number.is_empty()) {
            before.title = vec![Inline::Text(format!("{} {}", crate::plain(&before.title), title))];
            before.page = page.unwrap_or_default();
            continue;
        }
        let mut e = entry(title, page.unwrap_or_default(), if part { 0.0 } else { (indent / 4.0).round() * 12.0 }, part);
        e.number = number;
        entries.push(e);
    }
    (entries.len() >= 3 && with_page * 2 >= entries.len()).then_some(entries)
}

/// "Chapter I.5: Coulomb and Newton" as the contents set it in columns:
/// the number, and the title without it.
fn numbered(title: &str) -> Option<(String, String)> {
    let (head, rest) = title.split_once(": ")?;
    let mut words = head.split_whitespace();
    let (kind, number) = (words.next()?.to_lowercase(), words.next()?);
    (words.next().is_none() && matches!(kind.as_str(), "chapter" | "appendix") && chapter_mark(number)).then(|| (number.to_string(), rest.trim().to_string()))
}

/// The contents know the book's titles less well than the book does: they
/// are read from the PDF's own text, which has "It9" for "It?" and loses a
/// letter here and there. `opened`: the titles that open a page, in the
/// book's order, with the page's printed number. An entry whose page
/// opens with a title is that title (in contents set in columns: its
/// number in the first column, the rest in the second); a part's name is
/// the title that opens the part, before the chapter that follows it.
fn name_entries(blks: &mut [Blk], opened: &[(Option<String>, Vec<Inline>)]) {
    let columns = blks.iter().any(|b| matches!(&b.block, Block::Toc(e) if e.iter().filter(|e| !e.number.is_empty()).count() >= 3));
    let at_page = |page: &str| opened.iter().position(|(label, _)| !page.is_empty() && label.as_deref() == Some(page));
    let mut all: Vec<&mut TocEntry> = blks.iter_mut().filter_map(|b| if let Block::Toc(e) = &mut b.block { Some(e.iter_mut()) } else { None }).flatten().collect();
    for k in 0..all.len() {
        all[k].columns = columns;
        if all[k].part {
            // the part that the next entry's chapter comes right after
            let next = all[k + 1..].iter().find(|e| !e.part).and_then(|e| at_page(&e.page));
            if let Some((_, title)) = next.and_then(|j| j.checked_sub(1)).map(|j| &opened[j]).filter(|(_, t)| names_chapter(&plain(t))) {
                // (the words as the part's own page has them, in the
                // lines the contents set them in: "PART IV", and under it
                // the part's name)
                let words = plain(title);
                let first = match all[k].label.first() {
                    Some(Inline::Text(t)) if all[k].label.contains(&Inline::Break) => t.split_whitespace().count(),
                    _ => 2,
                };
                let mut w = words.split_whitespace();
                let head: Vec<&str> = w.by_ref().take(first).collect();
                let rest: Vec<&str> = w.collect();
                all[k].label = if rest.is_empty() { vec![Inline::Text(words.clone())] } else { vec![Inline::Text(format!("{} ", head.join(" "))), Inline::Break, Inline::Text(rest.join(" "))] };
                all[k].title = vec![Inline::Text(words)];
            }
        } else if let Some((_, title)) = at_page(&all[k].page).map(|j| &opened[j]) {
            let words = plain(title);
            match numbered(&words).filter(|_| columns) {
                Some((number, rest)) => {
                    // (the title without "Chapter I.5: ", its formulas kept)
                    let mut label = title.clone();
                    let cut = words.len() - rest.len();
                    match label.first_mut() {
                        Some(Inline::Text(t)) if t.len() >= cut && t.is_char_boundary(cut) => *t = t[cut..].to_string(),
                        _ => label = vec![Inline::Text(rest)],
                    }
                    (all[k].number, all[k].label) = (number, label);
                }
                // (a number the contents have and the title has not stays)
                None if !all[k].number.is_empty() && !words.starts_with(&all[k].number) => all[k].label = title.clone(),
                None => all[k].number.clear(),
            }
            all[k].title = title.clone();
        } else if !all[k].number.is_empty() && !columns {
            // (contents in a list: the number is part of the title)
            let whole = format!("{} {}", all[k].number, plain(&all[k].title));
            all[k].title = vec![Inline::Text(whole)];
            all[k].number.clear();
        }
    }
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

/// A page set in two columns (an index): the blocks of its left and of
/// its right column, each from the top down. The blocks of text stand
/// either left or right of the middle, a good part of the page's height
/// on both sides; what spans the page (a title, a line under it) is in
/// neither.
fn columns(page: &PageRead) -> Option<(Vec<usize>, Vec<usize>)> {
    let (mut left, mut right): (Vec<usize>, Vec<usize>) = Default::default();
    for (i, b) in page.blocks.iter().enumerate().filter(|(_, b)| matches!(b.kind.as_str(), "Text" | "TextInlineMath" | "ListItem") && has_words(&all_inline(&b.html))) {
        if b.right <= 0.54 * page.width {
            left.push(i);
        } else if b.left >= 0.46 * page.width {
            right.push(i);
        } else if b.bottom > 0.3 * page.height {
            // (text across the page, not at its top: no columns)
            return None;
        }
    }
    let height = |c: &[usize]| c.iter().map(|i| page.blocks[*i].bottom - page.blocks[*i].top).sum::<f32>();
    if height(&left) < 0.25 * page.height || height(&right) < 0.25 * page.height {
        return None;
    }
    // (a short left block at the very top beside nothing is a running head)
    let top = |i: &usize| page.blocks[*i].top;
    left.sort_by(|a, b| top(a).total_cmp(&top(b)));
    right.sort_by(|a, b| top(a).total_cmp(&top(b)));
    Some((left, right))
}

/// The entries of a column of an index, from a block of it: a line set
/// in from the others goes on from the line before; a block whose lines
/// are not known is one entry.
fn entries(b: &Raw) -> Vec<Vec<Inline>> {
    if b.text.len() < 2 {
        return vec![all_inline(&b.html)];
    }
    let least = b.indents.iter().copied().min().unwrap_or(0);
    let lines: Vec<(bool, String)> = b.text.iter().enumerate().map(|(k, line)| (b.indents.get(k).is_some_and(|i| *i > least + 1), line.clone())).collect();
    join_lines(&lines)
}

/// The entries of lines of which some go on from the line before.
fn join_lines(lines: &[(bool, String)]) -> Vec<Vec<Inline>> {
    // words written with a hyphen somewhere in these lines ("meson-meson")
    let hyphened: std::collections::HashSet<String> = lines
        .iter()
        .flat_map(|(_, l)| l.split_whitespace())
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase())
        .filter(|w| w.contains('-'))
        .collect();
    let mut out: Vec<String> = Vec::new();
    for (goes_on, line) in lines {
        let line = line.trim();
        match out.last_mut() {
            Some(entry) if *goes_on => {
                let last = entry.split_whitespace().last().unwrap_or("").trim_start_matches(|c: char| !c.is_alphanumeric()).to_lowercase();
                let next = line.split_whitespace().next().unwrap_or("").trim_end_matches(|c: char| !c.is_alphanumeric()).to_lowercase();
                // A line that ends in a hyphen: pages "248-" and "52" are
                // one range, and a word written with a hyphen elsewhere
                // keeps it; else the word was cut at the line's end.
                let range = last.trim_end_matches('-').ends_with(|c: char| c.is_ascii_digit()) && next.starts_with(|c: char| c.is_ascii_digit());
                if entry.ends_with('-') && (range || hyphened.contains(&format!("{last}{next}"))) {
                } else if entry.ends_with('-') && line.starts_with(char::is_lowercase) {
                    entry.pop();
                } else {
                    entry.push(' ');
                }
                entry.push_str(line);
            }
            _ => out.push(line.to_string()),
        }
    }
    out.iter().map(|t| line_inlines(t)).collect()
}

/// A line as marker hands it on with a block's lines: plain text, with
/// its formulas as LaTeX but unmarked ("\\sigma model, 318"). What is
/// written with a backslash is a formula.
fn line_inlines(line: &str) -> Vec<Inline> {
    let mut out: Vec<Inline> = Vec::new();
    for word in unescape(line).split_whitespace() {
        // (the comma after a formula is not part of it)
        let body = word.trim_end_matches([',', ';', '.', ':']);
        let math = body.contains('\\') && body.chars().any(char::is_alphabetic);
        if !out.is_empty() {
            push_text(&mut out, Inline::Text(" ".into()));
        }
        if math {
            out.push(Inline::Math(body.to_string()));
            push_text(&mut out, Inline::Text(word[body.len()..].to_string()));
        } else {
            push_text(&mut out, Inline::Text(word.to_string()));
        }
    }
    out.retain(|i| !matches!(i, Inline::Text(t) if t.is_empty()));
    out
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

/// How wide the text is set: the usual width of a full paragraph.
fn measure(pages: &[PageRead]) -> Option<f32> {
    let mut widths: Vec<f32> = pages.iter().flat_map(|p| p.blocks.iter().filter(|b| b.kind == "Text" && b.right - b.left > 0.7 * p.width).map(|b| b.right - b.left)).collect();
    widths.sort_by(f32::total_cmp);
    widths.get(widths.len() / 2).copied()
}

/// Where the text starts on a page, and where its lines end at the latest.
fn margins(page: &PageRead, measure: Option<f32>) -> (f32, f32) {
    let left = page.blocks.iter().filter(|b| is_text(b)).map(|b| b.left).fold(f32::MAX, f32::min);
    (left, left + measure.unwrap_or(0.9 * page.width))
}

/// marker gives each line of an address (or of a verse, or of the lines
/// about a book on its copyright page) as a paragraph of its own. Lines
/// of text that follow each other without space between them, each ended
/// before the right margin, are one paragraph with its line ends kept;
/// pieces side by side are one line. Not on a page of ordinary paragraphs,
/// and not on a page in columns (an index): there every entry is its own.
fn paragraphs_of_lines(mut pages: Vec<PageRead>, measure: Option<f32>) -> Vec<PageRead> {
    for page in pages.iter_mut() {
        let (_, right) = margins(page, measure);
        let line = |b: &Raw| b.kind == "Text" && b.text.len() < 2 && b.bottom - b.top <= 24.0 && has_words(&all_inline(&b.html));
        // Only on a page made of such lines (a copyright page, a poem). On
        // a page of ordinary paragraphs, a short line after another is
        // the end of one paragraph and the start of the next.
        // (nor on a page of equations with a few words between them)
        let paragraphs = page.blocks.iter().any(|b| b.kind == "Equation" || (b.kind == "Text" && b.bottom - b.top > 40.0 && b.right - b.left > 0.7 * page.width));
        if paragraphs || page.blocks.iter().filter(|b| is_text(b) && b.left > 0.45 * page.width).count() >= 3 {
            continue;
        }
        let mut out: Vec<Raw> = Vec::new();
        // (the last line of the paragraph being gathered)
        let mut last: Option<Raw> = None;
        for b in std::mem::take(&mut page.blocks) {
            let joins = match &last {
                Some(l) if line(&b) => {
                    let beside = (b.top - l.top).abs() <= 4.0 && b.left >= l.right - 2.0;
                    let under = b.top > l.top + 4.0 && b.top - l.bottom <= 6.0 && right - l.right >= 0.06 * page.width;
                    Some(beside).filter(|beside| *beside || under)
                }
                _ => None,
            };
            match (joins, out.last_mut()) {
                (Some(beside), Some(para)) => {
                    para.html = format!("{}{}{}", para.html, if beside { " " } else { " <br>" }, b.html);
                    (para.left, para.right, para.bottom) = (para.left.min(b.left), para.right.max(b.right), para.bottom.max(b.bottom));
                    para.lines = 0;
                    // (a piece beside the line: the line still ends where it did)
                    if !beside {
                        last = Some(b);
                    }
                }
                _ => {
                    last = line(&b).then(|| b.clone());
                    out.push(b);
                }
            }
        }
        page.blocks = out;
    }
    pages
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

/// Put entries of the table of contents on page `pi`: with the ones of
/// that page that are there already, else as a block after the page's
/// anchor.
fn toc(blks: &mut Vec<Blk>, pi: usize, entries: Vec<TocEntry>, anchor: &mut Option<String>) {
    if let Some(Blk { block: Block::Toc(there), page, .. }) = blks.last_mut() {
        if *page == pi {
            there.extend(entries);
            return;
        }
    }
    if let Some(a) = anchor.take() {
        blks.push(Blk { block: Block::Anchor(a), page: pi, centered: false });
    }
    blks.push(Blk { block: Block::Toc(entries), page: pi, centered: false });
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
    let measure = measure(pages);
    let pages = &paragraphs_of_lines(imprints(pages), measure);
    let labels = labels(pages);
    // the sizes of the headings, in the order they come
    let heads: Vec<(f32, usize)> = pages.iter().flat_map(|p| &p.blocks).filter(|b| b.kind == "SectionHeader").map(|b| (b.bottom - b.top, b.lines)).collect();
    let head_sizes = sizes(&heads);
    let mut head = 0;
    let openers = openers(pages);
    // (the title under a "Chapter 3" was joined to it: not a heading again)
    let mut joined = None;
    let mut blks: Vec<Blk> = Vec::new();
    let mut checked: Vec<(String, u32, Figure)> = Vec::new();
    // (size, text) of the headings, to find the size of the chapter titles
    let mut titles: Vec<(f32, String)> = Vec::new();
    for (pi, page) in pages.iter().enumerate() {
        // a page set centered keeps its look: centered, every line as it ends
        let all_centered = centered_page(page);
        let push = |blks: &mut Vec<Blk>, block: Block| {
            let centered = all_centered && !matches!(block, Block::Anchor(_));
            blks.push(Blk { block, page: pi, centered })
        };
        // where the lines of this page end at the latest
        let (_, right) = margins(page, measure);
        // (the title page of a part, also when the part's first words
        // stand under the title: centered, in its lines)
        let part_page = openers.as_ref().is_some_and(|o| o[pi].is_some_and(|o| plain(&all_inline(&page.blocks[o.at].html)).to_lowercase().starts_with("part ")));
        let breaks = |b: &Raw, inl: Vec<Inline>| -> Vec<Inline> {
            if b.kind == "SectionHeader" && part_page && b.text.len() >= 2 {
                return with_breaks(inl, &b.text, &|_| true);
            }
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
        // A page in columns keeps them: what spans the page above them,
        // then the left column, then the right, side by side.
        // (laid by the PDF's own words: the entries are known already)
        let in_laid = |b: &Raw| page.laid.as_ref().is_some_and(|l| matches!(b.kind.as_str(), "Text" | "TextInlineMath" | "ListItem" | "Code") && b.top >= l.top - 1.0 && b.bottom <= l.bottom + 1.0);
        let cols = if page.laid.is_some() { None } else { columns(page) };
        let order: Vec<(Option<usize>, Option<Block>)> = match (&page.laid, &cols) {
            (Some(laid), _) => {
                // (entries with page numbers: an index, a list; else paragraphs)
                let lines = laid.columns.iter().flatten().count().max(1);
                let numbered = laid.columns.iter().flatten().filter(|(_, t)| t.chars().any(|c| c.is_ascii_digit())).count() * 5 >= lines * 2;
                let entry = |text: Vec<Inline>| (None, Some(if numbered { Block::Item { level: 0, text } } else { Block::Para(text) }));
                let mark = |b: Block| (None, Some(b));
                let outside = |above: bool| (0..page.blocks.len()).filter(move |i| !in_laid(&page.blocks[*i]) && (page.blocks[*i].top < laid.top) == above).map(|i| (Some(i), None));
                let mut order: Vec<(Option<usize>, Option<Block>)> = outside(true).collect();
                order.push(mark(Block::Columns(laid.columns.len())));
                for (k, column) in laid.columns.iter().enumerate() {
                    if k > 0 {
                        order.push(mark(Block::ColumnBreak));
                    }
                    order.extend(join_lines(column).into_iter().filter(|t| has_words(t)).map(entry));
                }
                order.push(mark(Block::ColumnsEnd));
                order.extend(outside(false));
                order
            }
            (None, Some((left, right))) => {
                let first = left.iter().chain(right).map(|i| page.blocks[*i].top).fold(f32::MAX, f32::min);
                let in_columns = |i: usize| left.contains(&i) || right.contains(&i);
                let above = (0..page.blocks.len()).filter(|i| !in_columns(*i) && page.blocks[*i].top < first);
                let below = (0..page.blocks.len()).filter(|i| !in_columns(*i) && page.blocks[*i].top >= first);
                let at = |i: usize| (Some(i), None);
                let mark = |b: Block| (None, Some(b));
                above
                    .map(at)
                    .chain([mark(Block::Columns(2))])
                    .chain(left.iter().copied().map(at))
                    .chain([mark(Block::ColumnBreak)])
                    .chain(right.iter().copied().map(at))
                    .chain([mark(Block::ColumnsEnd)])
                    .chain(below.map(at))
                    .collect()
            }
            (None, None) => (0..page.blocks.len()).map(|i| (Some(i), None)).collect(),
        };
        // (a column of entries with page numbers: an index, a list)
        let listed = cols.as_ref().is_some_and(|(l, r)| {
            let lines: Vec<&String> = l.iter().chain(r).flat_map(|i| &page.blocks[*i].text).collect();
            !lines.is_empty() && lines.iter().filter(|t| t.chars().any(|c| c.is_ascii_digit())).count() * 5 >= lines.len() * 2
        });
        let pictures: Vec<&Raw> = page.blocks.iter().filter(|b| b.kind == "Picture" || b.kind == "Figure").collect();
        let contents_page = page.blocks.iter().any(|b| matches!(b.kind.as_str(), "Table" | "Form" | "TableOfContents") && contents(&b.text).is_some());
        for (at, mark) in order {
            let Some(bi) = at else {
                if let Some(a) = anchor.take() {
                    push(&mut blks, Block::Anchor(a));
                }
                blks.extend(mark.map(|block| Blk { block, page: pi, centered: false }));
                first_text = false;
                continue;
            };
            let b = &page.blocks[bi];
            let area = Figure { left: b.left * PX, right: b.right * PX, top: b.top * PX, bottom: b.bottom * PX };
            let mut made: Vec<Block> = Vec::new();
            // an entry of an index in columns, or a column of them
            if listed && cols.as_ref().is_some_and(|(l, r)| l.contains(&bi) || r.contains(&bi)) {
                if let Some(a) = anchor.take() {
                    push(&mut blks, Block::Anchor(a));
                }
                for text in entries(b) {
                    if has_words(&text) {
                        blks.push(Blk { block: Block::Item { level: 0, text }, page: pi, centered: false });
                    }
                }
                first_text = false;
                continue;
            }
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
                    // a heading on a page of the contents, other than
                    // their title: the name of a part, which marker took
                    // out of the table
                    let words = plain(&all_inline(&b.html));
                    if contents_page && !crate::blocks::is_contents_title(&words) && has_words(&all_inline(&b.html)) {
                        toc(&mut blks, pi, vec![entry(words, String::new(), 0.0, true)], &mut anchor);
                        continue;
                    }
                    let mut text = breaks(b, crate::no_emph(all_inline(&b.html)));
                    if let Some(openers) = &openers {
                        // by where the headings stand, not by their heights
                        size = SECTION;
                        if let Some(o) = openers[pi].filter(|o| o.at == bi) {
                            size = if o.chapter { CHAPTER } else { SECTION + 3.0 };
                            if let Some(title) = o.title {
                                // (on a page set centered the title is
                                // the next of its lines)
                                let under = &page.blocks[title];
                                if has_words(&text) {
                                    if all_centered || part_page {
                                        text.extend([Inline::Text(" ".into()), Inline::Break]);
                                    } else {
                                        text.push(Inline::Text(": ".into()));
                                    }
                                }
                                text.extend(breaks(under, crate::no_emph(all_inline(&under.html))));
                                text = crate::no_emph(text);
                                joined = Some((pi, title));
                            }
                        }
                    }
                    if has_words(&text) {
                        titles.push((size, plain(&text)));
                        // (what opens a page of the front matter is a
                        // part of it: a heading of the first level there)
                        let level = usize::from(openers.as_ref().is_some_and(|o| o[pi].is_some_and(|o| o.at == bi && !o.chapter)));
                        made.push(Block::Heading { level, size, id: String::new(), text });
                    }
                }
                "Footnote" => match footnote(&b.html) {
                    Some((label, text)) => made.push(Block::PageNote { label, text: symbols(text) }),
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
                        Some(entries) => toc(&mut blks, pi, entries, &mut anchor),
                        // a table marker read into rows and cells that hold
                        // together: a table, with the picture of it right
                        // under it (a cell can be misread)
                        None => match read_table(&b.html) {
                            Some(rows) => {
                                made.push(Block::Table { key: crate::table_key(page.number, &rows), rows, widths: Vec::new() });
                                made.push(Block::Picture { page: page.number, area, alt: crate::TABLE_READ.into() });
                            }
                            None => {
                                figures += 1;
                                made.push(Block::Figure { page: page.number, n: figures, area, label: labels[pi].clone() });
                            }
                        },
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
                        let k = key();
                        // (where it is on the page, in points, for its picture)
                        checked.push((k.clone(), page.number, Figure { left: b.left, right: b.right, top: b.top, bottom: b.bottom }));
                        made.push(equation(&maths, &beside, k));
                    } else {
                        for p in ps {
                            match p {
                                Part::Inline(i) => {
                                    let i = symbols(breaks(b, tidy(i)));
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
                let title = part_page && openers.as_ref().is_some_and(|o| o[pi].is_some_and(|o| o.at == bi)) && matches!(block, Block::Heading { .. });
                push(&mut blks, block);
                if let (true, Some(last)) = (title, blks.last_mut()) {
                    last.centered = true;
                }
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
    // the titles that open a page, in order, for the contents
    let opening = |size: f32| if openers.is_some() { size == CHAPTER || size == SECTION + 3.0 } else { Some(size) == chapter_size };
    let opened: Vec<(Option<String>, Vec<Inline>)> =
        blks.iter().filter_map(|b| if let Block::Heading { size, text, .. } = &b.block { opening(*size).then(|| (labels[b.page].clone(), text.clone())) } else { None }).collect();
    name_entries(&mut blks, &opened);
    Scan { equations: checked, blks, labels, pdf_pages: pages.iter().map(|p| p.number).collect(), chapter_size }
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

    #[test]
    fn an_index_in_two_columns_keeps_them() {
        let column = |left: f32, top: f32, html: &str, lines: &[&str]| {
            let lines: String = lines.iter().map(|l| format!("line\t{l}\n")).collect();
            format!("block\tText\t{left}\t{top}\t{}\t780.0\t{}\t{html}\n{lines}", left + 210.0, lines.matches('\n').count())
        };
        let mut page = parse_page(
            9,
            &format!(
                "page\t487.0\t842.0\nblock\tSectionHeader\t200.0\t40.0\t280.0\t60.0\t1\t<h1>Index</h1>\n{}{}",
                column(20.0, 100.0, "<p>x</p>", &["bosons: bad behavior of, 166;", "pairing into, 270", "Bose statistics, 117"]),
                column(250.0, 100.0, "<p>y</p>", &["closed forms, 219", "coherence length, 271"]),
            ),
        );
        // (the second line of the left column is set in: it goes on)
        page.blocks[1].indents = vec![0, 6, 0];
        let scan = build(&[page], "book");
        let shown: Vec<String> = scan
            .blks
            .iter()
            .map(|b| match &b.block {
                Block::Heading { text, .. } => format!("# {}", plain(text)),
                Block::Item { text, .. } => format!("- {}", plain(text)),
                Block::Columns(n) => format!("columns {n}"),
                Block::ColumnBreak => "next column".into(),
                Block::ColumnsEnd => "end".into(),
                other => format!("{other:?}"),
            })
            .collect();
        assert_eq!(
            shown,
            ["# Index", "columns 2", "- bosons: bad behavior of, 166; pairing into, 270", "- Bose statistics, 117", "next column", "- closed forms, 219", "- coherence length, 271", "end"]
        );
        // By the PDF's own words: marker laid one block over both columns
        // and dealt their lines into one another; the words say which
        // line is in which column and which is set in, and marker's
        // lines, which read better, are put in their places.
        let mut both = parse_page(
            10,
            "page\t487.0\t842.0\nblock\tCode\t20.0\t100.0\t470.0\t700.0\t40\t<pre>x</pre>\nline\tclosed forms, 219\nline\tbosons: bad behavior of, 166;\nline\tpairing into, 270\nline\tBose statistics, 117\nline\tcoherence length, 271",
        );
        let word = |left: f32, top: f32, text: &str| Word { left, top, right: left + 5.0 * text.len() as f32, bottom: top + 10.0, text: text.to_string() };
        let mut words = Vec::new();
        for row in 0..12 {
            // (lines enough for two columns; the first rows are the ones read)
            let y = 100.0 + 14.0 * row as f32;
            let (l, r): (&[&str], &[&str]) = match row {
                0 => (&["bosons:", "bad", "behavi0r", "of,", "166;"], &["closed", "f0rms,", "219"]),
                1 => (&["pairing", "into,", "270"], &["coherence", "length,", "271"]),
                2 => (&["Bose", "statistics,", "117"], &["other", "entry,", "5"]),
                _ => (&["more", "words,", "12"], &["and", "more,", "34"]),
            };
            let indent = if row == 1 { 12.0 } else { 0.0 };
            let mut x = 20.0 + indent;
            for w in l {
                words.push(word(x, y, w));
                x += 5.0 * w.len() as f32 + 4.0;
            }
            let mut x = 260.0;
            for w in r {
                words.push(word(x, y, w));
                x += 5.0 * w.len() as f32 + 4.0;
            }
        }
        both.laid = lay_columns(&both, &words);
        let laid = both.laid.as_ref().unwrap();
        assert_eq!(laid.columns[0][..3], [(false, "bosons: bad behavior of, 166;".to_string()), (true, "pairing into, 270".to_string()), (false, "Bose statistics, 117".to_string())]);
        assert_eq!(laid.columns[1][..2], [(false, "closed forms, 219".to_string()), (false, "coherence length, 271".to_string())]);
        let scan = build(&[both], "book");
        let items: Vec<String> = scan.blks.iter().filter_map(|b| if let Block::Item { text, .. } = &b.block { Some(plain(text)) } else { None }).collect();
        assert_eq!(items[..2], ["bosons: bad behavior of, 166; pairing into, 270", "Bose statistics, 117"]);
        // lines put together: a range of pages, a word with its hyphen, a word cut in two
        let lines = |l: &[&str]| -> Vec<(bool, String)> { l.iter().enumerate().map(|(k, t)| (k > 0, t.to_string())).collect() };
        let one = |l: &[&str]| plain(&join_lines(&lines(l))[0]);
        assert_eq!(one(&["anomaly, 243; consequences of, 248-", "52; meson-meson scattering, see meson-", "meson amplitude; renormal-", "ization, 150"]), "anomaly, 243; consequences of, 248-52; meson-meson scattering, see meson-meson amplitude; renormalization, 150");
        assert_eq!(line_inlines("\\sigma model, 318; &apos;t Hooft"), [Inline::Math("\\sigma".into()), Inline::Text(" model, 318; 't Hooft".into())]);
        assert!(lacks_lines("page\t487.0\t842.0\nblock\tText\t1\t1\t9\t9\t51\t<p>x</p>\n"));
        assert!(!lacks_lines("page\t487.0\t842.0\t2\nblock\tText\t1\t1\t9\t9\t51\t<p>x</p>\n"));
        assert!(!lacks_lines("page\t487.0\t842.0\nblock\tText\t1\t1\t9\t9\t2\t<p>x</p>\n"));
    }

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
        // a formula in two pieces is one; two formulas with a word between are two
        let two = "<p>and <math display=\"inline\">\\delta M =</math><math display=\"inline\">e^{x}</math> or <math display=\"inline\">a +</math> <math display=\"inline\">b</math>, <math display=\"inline\">c</math> and <math display=\"inline\">d</math></p>";
        let maths: Vec<String> = all_inline(two).into_iter().filter_map(|i| if let Inline::Math(m) = i { Some(m) } else { None }).collect();
        assert_eq!(maths, ["\\delta M = e^{x}", "a + b", "c", "d"]);
        assert_eq!(tag_of("(see below)"), None);
        let note = parse_page(1, "block\tFootnote\t0\t0\t9\t9\t1\t<p><sup>&amp;</sup>lt;sup&gt;1&lt;/sup&gt;When I was young</p>");
        // a mark found on the page's picture goes behind its word
        let mut marked = parse_page(1, "page\t487\t842\nblock\tText\t0\t0\t9\t9\t1\t<p>If it is <i>OK</i> we go on; else <math>x^2</math> is not OK. Then more.</p>\nblock\tFootnote\t0\t800\t9\t820\t1\t<p><sup>1</sup>A note.</p>");
        assert_eq!(unmarked(&marked), ["1"]);
        assert!(!place_mark(&mut marked, &["this".into(), "OK".into()], "1"), "two places: not placed");
        assert!(place_mark(&mut marked, &["is".into(), "not".into(), "OK".into()], "1"));
        assert!(marked.blocks[0].html.contains("is not OK.<sup>1</sup> Then more."));
        assert!(unmarked(&marked).is_empty());
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
    fn plain_symbols_in_a_sentence_are_formulas() {
        let shown = |t: &str| -> String {
            symbols(vec![Inline::Text(t.into())])
                .iter()
                .map(|i| match i {
                    Inline::Text(t) => t.clone(),
                    Inline::Math(m) => format!("${m}$"),
                    other => format!("{other:?}"),
                })
                .collect()
        };
        assert_eq!(shown("for J(x) real, W(J) and D(x-y), as in SU(2)."), "for $J(x)$ real, $W(J)$ and $D(x-y)$, as in $SU(2)$.");
        assert_eq!(shown("the mass m of a particle, with h = 1 and x + y, or m=0."), "the mass $m$ of a particle, with $h = 1$ and $x + y$, or $m=0$.");
        // not symbols: the article, initials, labels, units, words in brackets
        assert_eq!(shown("I gave A. Zee and S. Coleman a copy."), "I gave A. Zee and S. Coleman a copy.");
        assert_eq!(shown("see Part V, Appendix B and case (b), or b) below; 5 m high"), "see Part V, Appendix B and case (b), or b) below; 5 m high");
        assert_eq!(shown("a function (see above) is(not) one"), "a function (see above) is(not) one");
        assert_eq!(shown("in the limit T → ∞ we find Z"), "in the limit $T$ → ∞ we find $Z$");
        assert_eq!(shown("that a and b annihilate; constants a, b, as a rule; Figure 4.1a and b show a and the"), "that $a$ and $b$ annihilate; constants $a$, $b$, as a rule; Figure 4.1a and b show a and the");
    }

    #[test]
    fn formulas_that_would_not_render_are_mended() {
        // a growing bracket that closes in another row: plain brackets there
        let rows = "\\begin{aligned} a &= \\left( b \\\\ &\\quad + c \\right) + \\left( d \\right) \\end{aligned}";
        // (all of them in the cells where one does not close)
        assert_eq!(tame(rows), "\\begin{aligned} a &= ( b \\\\ &\\quad + c ) + ( d ) \\end{aligned}");
        assert_eq!(tame("\\left( x \\right)"), "\\left( x \\right)");
        assert!(whole("J = \\begin{pmatrix} 0 & 1 \\\\ 1 & 0 \\end{pmatrix}"));
        assert!(!whole("J = \\begin{pmatrix} 0 & 1 \\\\ 1 &"));
        assert!(!whole("a = b, where $c$ is"));
        assert!(!whole("\\begin{array}{ccccc"));
    }

    #[test]
    fn a_table_marker_read_is_a_table() {
        let html = "<table><tbody><tr><th>Dynamics of a</th><th>K</th><th>Effective Lagrangian \\mathcal{L}(A) \\sim A</th></tr><tr><td>Meissner a2</td><td>1</td><td>A(\\varepsilon \\partial) A \\sim A \\partial^2 A</td></tr><tr><td>Maxwell f^2 \\sim a \\partial^2 a</td><td>\\partial^2</td><td>A A</td></tr></tbody></table>";
        let rows = read_table(html).unwrap();
        let shown: Vec<Vec<String>> = rows
            .iter()
            .map(|r| {
                r.iter()
                    .map(|c| {
                        c.text
                            .iter()
                            .map(|i| match i {
                                Inline::Text(t) => t.clone(),
                                Inline::Math(m) => format!("${m}$"),
                                other => format!("{other:?}"),
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect();
        assert_eq!(
            shown,
            [
                vec!["Dynamics of a", "K", "Effective Lagrangian $\\mathcal{L}(A) \\sim A$"],
                vec!["Meissner a2", "1", "$A(\\varepsilon \\partial) A \\sim A \\partial^2 A$"],
                vec!["Maxwell $f^2 \\sim a \\partial^2 a$", "$\\partial^2$", "A A"]
            ]
        );
        // (formulas marker marked itself keep the space before them)
        let marked = cell_inlines("Effective Lagrangian <math>\\mathcal{L}(A)</math>");
        assert_eq!(marked, [Inline::Text("Effective Lagrangian ".into()), Inline::Math("\\mathcal{L}(A)".into())]);
        // an empty page read as a table of empty cells is none
        assert!(read_table("<table><tbody><tr><th></th><th></th></tr><tr><td></td><td>x</td></tr><tr><td></td><td></td></tr></tbody></table>").is_none());
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
        // lines that marker gave one by one: a paragraph, a number beside
        // its last line in that line; what follows a gap, or a line that
        // went to the margin, is a new paragraph
        let wide = block("Text", 16.0, 60.0, 458.0, 120.0, "<p>A paragraph as wide as the page is, of some lines, to show how wide the text is set on it.</p>", &[]);
        let lines = [
            block("Text", 16.0, 140.0, 381.0, 161.0, "<p>Published by the Press, 41 William Street, Princeton,</p>", &[]),
            block("Text", 16.0, 155.0, 114.0, 170.0, "<p>New Jersey 08540</p>", &[]),
            block("Text", 30.0, 171.0, 114.0, 184.0, "<p>530.14 dc21</p>", &[]),
            block("Text", 220.0, 171.0, 288.0, 184.0, "<p>2002031743</p>", &[]),
            block("Text", 16.0, 205.0, 300.0, 218.0, "<p>British Library data is available</p>", &[]),
            block("Text", 16.0, 230.0, 455.0, 243.0, "<p>A line that runs all the way to the right margin of the page it is on</p>", &[]),
            block("Text", 16.0, 244.0, 200.0, 257.0, "<p>and one after it.</p>", &[]),
        ];
        let page = parse_page(5, &format!("page\t487.0\t842.0\n{}", lines.concat()));
        // (on a page of ordinary paragraphs the same lines stay apart)
        let other = parse_page(6, &format!("page\t487.0\t842.0\n{wide}{}", lines[..2].concat()));
        let scan = build(&[page, other], "book");
        let paras: Vec<String> = scan.blks.iter().filter_map(|b| if let Block::Para(t) = &b.block { Some(emit_plain(t)) } else { None }).collect();
        assert_eq!(
            paras,
            [
                "Published by the Press, 41 William Street, Princeton, /New Jersey 08540 /530.14 dc21 2002031743",
                "British Library data is available",
                "A line that runs all the way to the right margin of the page it is on",
                "and one after it.",
                "A paragraph as wide as the page is, of some lines, to show how wide the text is set on it.",
                "Published by the Press, 41 William Street, Princeton,",
                "New Jersey 08540",
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
        let got: Vec<(&str, String, &str, bool)> = entries.iter().map(|e| (e.number.as_str(), plain(&e.title), e.page.as_str(), e.part)).collect();
        assert_eq!(
            got,
            [
                ("", "Preface".to_string(), "xi", false),
                ("", "PART I MOTIVATION AND FOUNDATION".to_string(), "", true),
                ("I.1", "Who Needs It?".to_string(), "3", false),
                ("I.2", "Path Integrals".to_string(), "7", false)
            ]
        );
        // (the part's name keeps its two lines)
        assert!(entries[1].label.contains(&Inline::Break));
        assert_eq!(numbered("Chapter I.5: Coulomb and Newton: Repulsion"), Some(("I.5".to_string(), "Coulomb and Newton: Repulsion".to_string())));
        assert_eq!(numbered("Appendix B: A Brief Review"), Some(("B".to_string(), "A Brief Review".to_string())));
        assert_eq!(numbered("Closing Words"), None);
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
