//! Layout-aware PDF reading: every piece of text with its position and font.
//!
//! Plain text (see `extract.rs`) loses everything that tells a human reader
//! what they are looking at: that a "3" is a raised footnote marker, that a
//! line is a heading because it is bigger, that a symbol is math because it is
//! set in a math font. This module keeps that information.
//!
//! We run poppler's `pdftohtml -xml`, which prints something like
//!
//! ```xml
//! <fontspec id="2" size="15" family="YIEKEP+CMR10" color="#000000"/>
//! <text top="159" left="568" width="9" height="15" font="3">x</text>
//! <text top="157" left="576" width="3" height="10" font="8">′</text>
//! ```
//!
//! and turn it into a [`Doc`]: pages -> lines -> spans. Coordinates are in
//! pdftohtml's pixels (1.5 per PDF point); `top` grows downwards.
//!
//! Nothing here guesses about chapters or footnotes yet. That is the job of
//! later stages, which can now ask precise questions ("is this span raised
//! and smaller than its line?") instead of guessing from plain text.

use std::collections::HashMap;
use crate::progress::Step;
use std::path::Path;
use std::process::Command;

/// One font as declared by a `<fontspec>`.
#[derive(Debug, Clone)]
pub struct Font {
    /// Size in pdftohtml pixels (a 10 pt font is 15).
    pub size: f32,
    /// Font name without the random subset prefix: "YIEKEP+CMR10" -> "CMR10".
    pub family: String,
    /// True for fonts that are only used for mathematics (see [`is_math_font`]).
    pub math: bool,
}

/// Is a span raised (superscript), lowered (subscript) or on the baseline?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Script {
    Normal,
    Super,
    Sub,
}

/// One `<text>` element: a run of characters in a single font.
#[derive(Debug, Clone)]
pub struct Span {
    pub text: String,
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
    /// Index into [`Doc::fonts`].
    pub font: usize,
    pub bold: bool,
    pub italic: bool,
    /// Filled in when spans are grouped into lines.
    pub script: Script,
    /// A drawn bar sits right above it (Ā). Set by [`read_pdf`].
    pub overline: bool,
    /// LaTeX read from an equation image (see [`crate::ocr`]); the span
    /// stands for that whole piece of math.
    pub tex: Option<String>,
    /// The PDF page this text links to (an internal link, as in a linked
    /// table of contents or index).
    pub link: Option<u32>,
    /// A web address this text links to, and the words of the link itself
    /// (the span can hold more text than the link: "…org/. So then").
    pub url: Option<String>,
    pub url_text: Option<String>,
    /// Where on the page `link` goes, in pdftohtml's pixels from the top
    /// (read from the PDF's link annotation; see [`crate::annots`]).
    pub link_top: Option<f32>,
    /// An equation image that could not be read, inside a line of text: it
    /// stays a small picture in the sentence (the PDF page it is on).
    pub picture: Option<u32>,
    /// The equation image this span was read from (its cache key).
    pub image_key: Option<String>,
}

impl Span {
    pub fn right(&self) -> f32 {
        self.left + self.width
    }
    pub fn bottom(&self) -> f32 {
        self.top + self.height
    }
}

/// Spans that sit on the same baseline, sorted left to right.
#[derive(Debug, Clone)]
pub struct Line {
    pub spans: Vec<Span>,
    /// Position and font size of the line's largest span (its "main" text).
    pub top: f32,
    pub left: f32,
    pub right: f32,
    pub size: f32,
    /// 0 on normal pages; 1 = left column, 2 = right column on two-column
    /// pages (set by `pages.rs`).
    pub col: u8,
    /// Fraction/root bars inside this line (set by `blocks.rs`).
    pub bars: Vec<Rule>,
}

/// A thin horizontal bar drawn on the page (fraction bar, overline, root
/// bar). Not text, so it comes from `pdftocairo -svg`, not `pdftohtml`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rule {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

/// A tall bracket drawn on the page, read from its glyph outline in the
/// SVG (pdftohtml gives such glyphs no character, or drops them).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Delim {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    /// one of ( ) [ ] | ⟨ ⟩ { }
    pub kind: char,
}

#[derive(Debug, Clone)]
pub struct Page {
    /// Physical page number in the PDF (1 = first sheet), not the printed one.
    pub number: u32,
    pub width: f32,
    pub height: f32,
    pub lines: Vec<Line>,
    pub rules: Vec<Rule>,
    /// Panels of colour drawn behind text (a table's grey ground), with
    /// how light each is, 0 (black) to 1 (white).
    pub shades: Vec<(Rule, f32)>,
    /// Pictures (drawings, images) with their labels, top to bottom.
    pub figures: Vec<Figure>,
    /// Embedded images (pictures or, in ebook-made PDFs, equations).
    pub images: Vec<PageImage>,
    /// Printed pages that start on this page (ebook-made PDFs without page
    /// numbers): height where each starts, and its number. From the index.
    pub marks: Vec<(f32, String)>,
    /// Blocks read from images that go between the lines: program code
    /// (an image read as text) and horizontal rules. Height and block.
    pub codes: Vec<(f32, crate::Block)>,
}

/// A picture on a page: a drawing or an embedded image together with the
/// text labels inside it, in pdftohtml's pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Figure {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

#[derive(Debug, Clone)]
pub struct Doc {
    pub fonts: Vec<Font>,
    pub pages: Vec<Page>,
    /// The most common font size, weighted by number of characters.
    /// Anything clearly bigger is a heading candidate; smaller is a
    /// footnote/superscript candidate.
    pub body_size: f32,
}

/// Fonts that only ever contain math symbols. TeX books use the Computer
/// Modern / AMS fonts; others use names containing "Math" or "Symbol".
pub fn is_math_font(family: &str) -> bool {
    const PREFIXES: &[&str] = &["CMMI", "CMSY", "CMEX", "MSAM", "MSBM", "EUFM", "EUEX", "RSFS", "MTMI", "MTSY", "MTEX"];
    const CONTAINS: &[&str] = &["MATH", "SYMBOL", "STIX", "EULER"];
    let f = family.to_uppercase();
    PREFIXES.iter().any(|p| f.starts_with(p)) || CONTAINS.iter().any(|c| f.contains(c))
}

/// Bold fonts: TeX's bold math italic (CMMIB), bold symbols (CMBSY), bold
/// roman (CMBX), bold sans (CMSSBX), and fonts named "...Bold...".
pub fn is_bold_font(family: &str) -> bool {
    let f = family.to_uppercase();
    ["CMMIB", "CMBSY", "CMBX", "CMSSBX", "EUFB", "EURB"].iter().any(|p| f.starts_with(p))
        || f.contains("BOLD")
        || f.ends_with("-BD")
}

/// Clean up extracted text: expand ligatures ("ﬁ" -> "fi") and drop
/// invisible control / private-use characters that some TeX fonts produce.
/// A letter with an accent that TeX fonts draw as two glyphs ("¨o" in
/// "Schr¨odinger") as one character.
fn compose(accent: char, letter: char) -> Option<char> {
    let (from, to) = match accent {
        '¨' => ("aeiouyAEIOU", "äëïöüÿÄËÏÖÜ"),
        '´' => ("aeiouyAEIOUc", "áéíóúýÁÉÍÓÚć"),
        '`' => ("aeiouAEIOU", "àèìòùÀÈÌÒÙ"),
        'ˆ' => ("aeiouAEIOU", "âêîôûÂÊÎÔÛ"),
        '˜' => ("anoANO", "ãñõÃÑÕ"),
        'ˇ' => ("cszrCSZR", "čšžřČŠŽŘ"),
        _ => return None,
    };
    from.chars().position(|c| c == letter).and_then(|k| to.chars().nth(k))
}

/// An accent glyph followed by a letter as one letter (precomposed where
/// Unicode has one, else the letter with a combining accent).
pub fn compose_accents(t: &str) -> String {
    let mut out = String::with_capacity(t.len());
    let mut chars = t.chars().peekable();
    while let Some(c) = chars.next() {
        let next = chars.peek().copied().filter(|n| n.is_alphabetic());
        let combining = match c {
            '¨' => Some('\u{308}'),
            '´' => Some('\u{301}'),
            'ˆ' => Some('\u{302}'),
            '˜' => Some('\u{303}'),
            'ˇ' => Some('\u{30C}'),
            '˙' => Some('\u{307}'),
            _ => None,
        };
        match (next, combining) {
            (Some(n), Some(m)) => {
                match compose(c, n) {
                    Some(l) => out.push(l),
                    None => {
                        out.push(n);
                        out.push(m);
                    }
                }
                chars.next();
            }
            _ => out.push(c),
        }
    }
    out
}

pub fn clean_text(t: &str) -> String {
    let t = compose_accents(t);
    let mut out = String::with_capacity(t.len());
    let chars = t.chars();
    for c in chars {
        match c {
            'ﬀ' => out.push_str("ff"),
            'ﬁ' => out.push_str("fi"),
            'ﬂ' => out.push_str("fl"),
            'ﬃ' => out.push_str("ffi"),
            'ﬄ' => out.push_str("ffl"),
            '\u{E000}'..='\u{F8FF}' => {}
            c if c.is_control() => {}
            c => out.push(c),
        }
    }
    out
}

/// Pages given to one `pdftocairo`.
const SVG_PART: usize = 8;

/// `pdftocairo -svg` of `total` pages from page `first` on, as the parts it
/// was made in, each with its number of pages. The program writes nothing
/// before it is through with all its pages, which takes minutes for a
/// scanned book: run on a few pages at a time (on all processors) it tells
/// how far it is. None if it is missing or fails: the conversion still
/// works, just without drawn bars, brackets and pictures.
fn svg_parts(path: &Path, first: usize, total: usize) -> Option<Vec<(usize, String)>> {
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    let run = |from: Option<usize>, to: usize| -> Option<String> {
        let mut cmd = Command::new("pdftocairo");
        cmd.arg("-svg");
        if let Some(from) = from {
            cmd.args(["-f", &from.to_string(), "-l", &to.to_string()]);
        }
        let out = cmd.arg(path).arg("-").stdin(std::process::Stdio::null()).output().ok()?;
        out.status.success().then(|| String::from_utf8_lossy(&out.stdout).into_owned())
    };
    crate::progress::report(Step::Drawings, 0, total.max(1));
    if total == 0 {
        // (the number of pages is not known: all at once)
        let svg = run(None, 0)?;
        let pages = svg_pages(&svg).len();
        crate::progress::report(Step::Drawings, 1, 1);
        return Some(vec![(pages, svg)]);
    }
    let ranges: Vec<(usize, usize)> = (0..total).step_by(SVG_PART).map(|at| (first + at, first + (at + SVG_PART).min(total) - 1)).collect();
    let parts: Vec<std::sync::Mutex<Option<String>>> = ranges.iter().map(|_| std::sync::Mutex::new(None)).collect();
    let (next, done, failed) = (AtomicUsize::new(0), AtomicUsize::new(0), AtomicBool::new(false));
    let workers = std::thread::available_parallelism().map_or(1, |n| n.get()).min(ranges.len());
    std::thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| {
                while !failed.load(Ordering::Relaxed) {
                    let i = next.fetch_add(1, Ordering::Relaxed);
                    let Some(&(from, to)) = ranges.get(i) else { break };
                    match run(Some(from), to) {
                        Some(svg) => *parts[i].lock().unwrap_or_else(|e| e.into_inner()) = Some(svg),
                        None => failed.store(true, Ordering::Relaxed),
                    }
                    let pages = done.fetch_add(to + 1 - from, Ordering::Relaxed) + to + 1 - from;
                    // (the step is over when the parts are read, below)
                    crate::progress::report(Step::Drawings, pages.min(total - 1), total);
                }
            });
        }
    });
    if failed.load(Ordering::Relaxed) {
        return None;
    }
    let parts = ranges.iter().zip(parts).map(|(&(from, to), part)| Some((to + 1 - from, part.into_inner().ok()??))).collect();
    parts
}

/// Read a PDF. `pages` limits it to a range (first, last), both 1-based.
pub fn read_pdf(path: &Path, pages: Option<(u32, u32)>) -> Result<Doc, String> {
    let mut doc = read_text(path, pages)?;
    read_drawings(&mut doc, path, pages);
    Ok(doc)
}

/// How many pages are read: all of the PDF, or the range asked for.
fn pages_read(path: &Path, pages: Option<(u32, u32)>) -> usize {
    let in_all = crate::progress::page_count(path);
    match pages {
        // (a range may name more pages than there are)
        Some((first, last)) => (if in_all > 0 { last.min(in_all as u32) } else { last } + 1).saturating_sub(first) as usize,
        None => in_all,
    }
}

/// The text of a PDF with its positions and fonts (`pdftohtml`).
pub fn read_text(path: &Path, pages: Option<(u32, u32)>) -> Result<Doc, String> {
    let mut cmd = Command::new("pdftohtml");
    // -xml: XML with positions; -i: skip images; -q: quiet; -stdout: no files
    cmd.args(["-xml", "-i", "-q", "-stdout", "-nodrm"]);
    if let Some((first, last)) = pages {
        cmd.args(["-f", &first.to_string(), "-l", &last.to_string()]);
    }
    cmd.arg(path);
    // how many pages there are to read, to tell how far it is
    let total = pages_read(path, pages);
    let (ok, stdout, stderr) = crate::progress::run_counting(cmd, b"<page ", Step::Text, total).map_err(|e| {
        format!("could not run pdftohtml ({e}); install poppler (Arch) or poppler-utils")
    })?;
    if !ok {
        return Err(format!("pdftohtml failed: {}", String::from_utf8_lossy(&stderr).trim()));
    }
    let mut doc = parse_xml_with(&String::from_utf8_lossy(&stdout), &crate::glyphs::lost_fonts(path))?;
    add_link_positions(&mut doc, path);
    printed_page_marks(&mut doc);
    crate::progress::report(Step::Text, total, total);
    Ok(doc)
}

/// What is drawn on the pages (`pdftocairo -svg`): bars, tall brackets,
/// pictures, and the images, which are read if they are equations. If
/// pdftocairo is missing the conversion still works, just without them.
pub fn read_drawings(doc: &mut Doc, path: &Path, pages: Option<(u32, u32)>) {
    let total = pages_read(path, pages);
    let first = pages.map_or(1, |(first, _)| first as usize);
    let parts = svg_parts(path, first, total);
    let lost = crate::glyphs::lost_fonts(path);
    if !lost.is_empty() {
        if let Some(parts) = &parts {
            restore_glyphs(doc, path, parts, &lost);
        }
        drop_empty(doc, &lost);
    }
    if let Some(parts) = parts {
        // Lost big brackets are a TeX font problem; in books without
        // TeX's math fonts, outlines of large text glyphs are no brackets.
        let tex_math = doc.fonts.iter().any(|f| {
            let u = f.family.to_uppercase();
            ["CMMI", "CMSY", "CMEX", "CMR", "MSAM", "MSBM"].iter().any(|p| u.starts_with(p))
        });
        // what each part holds, page by page, in one row for the book
        fn all<T>(parts: &[(usize, String)], parse: impl Fn(&str) -> Vec<Vec<T>>) -> Vec<Vec<T>> {
            let mut out = Vec::new();
            for (pages, svg) in parts {
                let mut of_part = parse(svg);
                of_part.resize_with(*pages, Vec::new);
                out.append(&mut of_part);
            }
            out
        }
        for (page, r) in doc.pages.iter_mut().zip(all(&parts, parse_svg_rules)) {
            page.rules = r;
        }
        for (page, s) in doc.pages.iter_mut().zip(all(&parts, parse_svg_shades)) {
            page.shades = s;
        }
        mark_overlines(doc);
        if tex_math {
            insert_delims(doc, all(&parts, parse_svg_delims));
        }
        for (page, f) in doc.pages.iter_mut().zip(all(&parts, parse_svg_figures)) {
            page.figures = f;
            // The light panel a table (or the note under it) stands on is
            // no picture: its text is text, not the labels of a drawing.
            let panels: Vec<Rule> = page.shades.iter().filter(|(_, light)| *light >= 0.7).map(|(p, _)| *p).collect();
            page.figures.retain(|f| {
                let area = (f.right - f.left) * (f.bottom - f.top);
                let covered: f32 = panels
                    .iter()
                    .map(|p| (f.right.min(p.right) - f.left.max(p.left)).max(0.0) * (f.bottom.min(p.bottom) - f.top.max(p.top)).max(0.0))
                    .sum();
                area <= 0.0 || covered < 0.5 * area
            });
        }
        for (page, imgs) in doc.pages.iter_mut().zip(all(&parts, parse_svg_images)) {
            page.images = imgs;
        }
        // labels of drawn pictures (text pieces); an embedded image has
        // its labels in its pixels, so text around it is never one
        take_figure_labels(doc);
        place_images(doc);
        join_split_equations(doc);
        title_words(doc);
    }
}

/// A glyph drawn on a page (`<use xlink:href="#glyph-4-5" x=".." y=".."/>`),
/// in points: where it starts on its base line, and the left, right and
/// top edge of what is drawn.
#[derive(Clone, Copy)]
struct Placed {
    x: f32,
    y: f32,
    ink: (f32, f32, f32),
}

/// The glyphs of each page in the order they are drawn.
fn parse_svg_uses(svg: &str) -> Vec<Vec<Placed>> {
    let text = |tag: &str, name: &str| -> Option<String> { tag.split(&format!(" {name}=\"")).nth(1)?.split('"').next().map(str::to_string) };
    // how far each glyph's outline reaches
    let mut inks: HashMap<&str, (f32, f32, f32)> = HashMap::new();
    let defs = svg.split("</defs>").next().unwrap_or("");
    for g in defs.split("<g id=\"").skip(1) {
        let Some((id, body)) = g.split_once('"') else { continue };
        let body = body.split("</g>").next().unwrap_or("");
        let Some(d) = body.split(" d=\"").nth(1).and_then(|r| r.split('"').next()) else { continue };
        let o = outline(d);
        if !o.pts.is_empty() {
            inks.insert(id, (o.bbox.0, o.bbox.1, o.bbox.2));
        }
    }
    svg_pages(svg)
        .into_iter()
        .map(|page| {
            page.split("<use ")
                .skip(1)
                .filter_map(|tag| {
                    let tag = format!(" {}", tag.split("/>").next()?);
                    let id = text(&tag, "xlink:href")?;
                    let id = id.strip_prefix("#glyph")?;
                    let (x, y): (f32, f32) = (text(&tag, "x")?.parse().ok()?, text(&tag, "y")?.parse().ok()?);
                    let ink = inks.get(format!("glyph{id}").as_str()).map_or((x, x, y), |(l, r, t)| (x + l, x + r, y + t));
                    Some(Placed { x, y, ink })
                })
                .collect()
        })
        .collect()
}

/// Glyphs drawn over the letter they belong to: their place is where they
/// are drawn, not where the type moves on.
const OVER: [&str; 4] = ["\u{20d7}", "\u{338}", "˜", "ˆ"];

/// Write the glyphs pdftohtml left out into the empty pieces of text it
/// gave for them. What each glyph is, the PDF's fonts say by its name
/// ([`crate::glyphs`]); where it is, the SVG says: both list the glyphs of
/// a page in the order they are drawn. A page where the two do not count
/// the same number of glyphs is left alone.
fn restore_glyphs(doc: &mut Doc, path: &Path, parts: &[(usize, String)], lost: &HashMap<String, bool>) {
    use crate::glyphs::Glyph;
    let (Some(first), Some(last)) = (doc.pages.first().map(|p| p.number), doc.pages.last().map(|p| p.number)) else { return };
    let Some(glyphs) = crate::glyphs::read(path, first, last) else { return };
    let debug = std::env::var_os("PDF2MD_GLYPHDEBUG").is_some();
    let mut uses: Vec<Vec<Placed>> = Vec::new();
    for (pages, svg) in parts {
        let mut of_part = parse_svg_uses(svg);
        of_part.resize_with(*pages, Vec::new);
        uses.append(&mut of_part);
    }
    let families: Vec<String> = doc.fonts.iter().map(|f| f.family.strip_prefix("CMEX/").unwrap_or(&f.family).to_string()).collect();
    let extension: Vec<bool> = doc.fonts.iter().map(|f| f.family.starts_with("CMEX/")).collect();
    let kept: Vec<bool> = doc.fonts.iter().map(|f| lossy(&f.family, lost).is_some()).collect();
    let same = |base: &str, family: &str| base == family || base.strip_prefix(family).is_some_and(|rest| rest.starts_with('-'));
    let (mut filled, mut missed, mut pages_off) = (0, 0, 0);
    for (page, placed) in doc.pages.iter_mut().zip(uses) {
        let Some(shown) = glyphs.pages.get(&page.number) else { continue };
        // (a space is a glyph that draws nothing: it may not be in the SVG)
        let drawn: Vec<(usize, u8)> = if shown.len() == placed.len() {
            shown.clone()
        } else {
            shown.iter().copied().filter(|&(_, code)| code != b' ').collect()
        };
        if drawn.len() != placed.len() {
            pages_off += 1;
            if debug {
                eprintln!("GLYPHS page {}: {} shown, {} drawn", page.number, shown.len(), placed.len());
            }
            continue;
        }
        // the glyphs of each piece of text, in pdftohtml's pixels
        let mut of_span: HashMap<(usize, usize), Vec<(Placed, Glyph)>> = HashMap::new();
        for (&(f, code), at) in drawn.iter().zip(&placed) {
            let info = &glyphs.fonts[f];
            if lossy(&info.base, lost).is_none() {
                continue;
            }
            let glyph = info.glyph(code);
            if debug && glyph == Glyph::Unknown {
                eprintln!("GLYPHS unknown {} {:?} {code}", info.base, info.name(code));
            }
            let at = Placed { x: 1.5 * at.x, y: 1.5 * at.y, ink: (1.5 * at.ink.0, 1.5 * at.ink.1, 1.5 * at.ink.2) };
            // the piece of this font the glyph starts in (its base line is
            // inside the piece's box, or at its top: big symbols hang)
            let mut best: Option<((usize, usize), f32)> = None;
            for (li, line) in page.lines.iter().enumerate() {
                for (si, s) in line.spans.iter().enumerate() {
                    if !kept[s.font] || !same(&info.base, &families[s.font]) || at.y < s.top - 3.0 || at.y > s.bottom() + 3.0 {
                        continue;
                    }
                    let off = (s.left - at.x).max(at.x - (s.right() - 1.0).max(s.left)).max(0.0);
                    if off <= 1.5 && best.is_none_or(|(_, o)| off < o) {
                        best = Some(((li, si), off));
                    }
                }
            }
            match best {
                Some((span, _)) => of_span.entry(span).or_default().push((at, glyph)),
                None if matches!(glyph, Glyph::Lost(_)) => {
                    missed += 1;
                    if debug {
                        eprintln!("GLYPHS page {}: no place for {glyph:?} of {} at {:.0},{:.0}", page.number, info.base, at.x, at.y);
                    }
                }
                None => {}
            }
        }
        // (the pieces of a line from its end, so that one cut into its
        // glyphs leaves the places of those before it)
        let mut spans: Vec<_> = of_span.into_iter().collect();
        spans.sort_by_key(|s| std::cmp::Reverse(s.0));
        // big symbols written, to be hung where they are drawn: line, the
        // piece's place as pdftohtml gave it, the top of what is drawn
        let mut hung: Vec<(usize, f32, f32, f32)> = Vec::new();
        for ((li, si), mut found) in spans {
            let lost_here = found.iter().filter(|(_, g)| matches!(g, Glyph::Lost(_))).count();
            if lost_here == 0 {
                continue;
            }
            found.sort_by(|a, b| a.0.x.total_cmp(&b.0.x));
            // (a glyph nothing is known of is left out)
            let found: Vec<(Placed, bool, String)> = found
                .into_iter()
                .filter_map(|(at, g)| match g {
                    Glyph::Known(t) => Some((at, false, clean_text(&t))),
                    Glyph::Lost(t) => Some((at, true, t.to_string())),
                    Glyph::Unknown => None,
                })
                .collect();
            let span = page.lines[li].spans[si].clone();
            let over = |t: &str| OVER.contains(&t);
            // one glyph, or the glyphs of a big-symbol font (which pdftohtml
            // writes without spaces): the piece stays one
            if extension[span.font] || found.len() == 1 {
                let s = &mut page.lines[li].spans[si];
                s.text = found.iter().filter(|f| f.1).map(|f| f.2.as_str()).collect();
                if let [(at, _, t)] = found.as_slice() {
                    if over(t) {
                        (s.left, s.width) = (at.ink.0, (at.ink.1 - at.ink.0).max(1.0));
                    }
                }
                if extension[span.font] && !s.text.is_empty() && !found.iter().any(|f| over(&f.2)) {
                    let top = found.iter().filter(|f| f.1).map(|f| f.0.ink.2).fold(f32::MAX, f32::min);
                    hung.push((li, s.left, s.top, top));
                }
                filled += lost_here;
                continue;
            }
            // Text with gaps. What pdftohtml gave must be what is known of
            // it (it knows some glyphs by their names, too).
            let given: String = span.text.split_whitespace().collect();
            let join = |with: &dyn Fn(&(Placed, bool, String)) -> bool| -> String { found.iter().filter(|f| with(f)).flat_map(|f| f.2.split_whitespace()).collect() };
            if join(&|_| true) == given {
                continue;
            }
            if !given.is_empty() && join(&|f| !f.1 || given.contains(f.2.as_str())) != given {
                // not what pdftohtml gave: its text stays, and the lost
                // glyphs are pieces of their own beside it
                if debug {
                    eprintln!("GLYPHS page {}: {:?} does not go into {:?}", page.number, found.iter().map(|f| &f.2).collect::<Vec<_>>(), span.text);
                }
                let pieces: Vec<Span> = found
                    .iter()
                    .filter(|f| f.1)
                    .map(|(at, _, t)| {
                        let mut s = span.clone();
                        s.text = t.clone();
                        (s.left, s.width) = if over(t) { (at.ink.0, (at.ink.1 - at.ink.0).max(1.0)) } else { (at.x, (at.ink.1 - at.x).max(1.0)) };
                        s
                    })
                    .collect();
                page.lines[li].spans.splice(si + 1..si + 1, pieces);
                filled += lost_here;
                continue;
            }
            // each glyph is a piece of its own, where it is drawn
            let pieces: Vec<Span> = found
                .iter()
                .enumerate()
                .map(|(k, (at, _, t))| {
                    let mut s = span.clone();
                    s.text = t.clone();
                    let next = found.get(k + 1).map_or(span.right(), |n| n.0.x);
                    (s.left, s.width) = if over(t) { (at.ink.0, (at.ink.1 - at.ink.0).max(1.0)) } else { (at.x, (next - at.x).max(1.0)) };
                    s
                })
                .collect();
            page.lines[li].spans.splice(si..=si, pieces);
            filled += lost_here;
        }
        // A big symbol hangs from its base line, and pdftohtml's box for it
        // is around that line, not around the symbol; how far above, each
        // font has its own way. The box starts where the symbol is drawn
        // now, and the symbol goes to the line at or under that (as
        // `group_lines` does for TeX's), away from the limit set over it.
        let fonts = doc.fonts.clone();
        let size_of = |s: &Span| fonts.get(s.font).map_or(s.height, |f| f.size);
        let mut remake = vec![false; page.lines.len()];
        for (li, left, top, ink_top) in hung {
            let Some(si) = page.lines[li].spans.iter().position(|s| extension[s.font] && s.left == left && s.top == top) else { continue };
            let mut s = page.lines[li].spans.remove(si);
            s.top = ink_top;
            let to = (0..page.lines.len())
                .filter(|&k| k == li || !page.lines[k].spans.is_empty())
                .filter(|&k| page.lines[k].top >= s.top - 0.3 * s.height && page.lines[k].top <= s.top + 3.0 * s.height)
                .min_by(|&a, &b| page.lines[a].top.total_cmp(&page.lines[b].top))
                .unwrap_or(li);
            page.lines[to].spans.push(s);
            (remake[li], remake[to]) = (true, true);
        }
        for (k, line) in page.lines.iter_mut().enumerate() {
            if remake[k] && !line.spans.is_empty() {
                *line = Line { col: line.col, bars: std::mem::take(&mut line.bars), ..make_line(std::mem::take(&mut line.spans), &size_of) };
            }
        }
        page.lines.retain(|l| !l.spans.is_empty());
        page.lines.sort_by(|a, b| a.top.total_cmp(&b.top));
    }
    if debug {
        eprintln!("GLYPHS {filled} written, {missed} without a place, {pages_off} pages not counted alike");
    }
}

/// The pieces of text kept for [`restore_glyphs`] that are still empty
/// stand for nothing.
fn drop_empty(doc: &mut Doc, lost: &HashMap<String, bool>) {
    let fonts = doc.fonts.clone();
    let size_of = |s: &Span| fonts.get(s.font).map_or(s.height, |f| f.size);
    let empty = |s: &Span| s.text.trim().is_empty() && fonts.get(s.font).is_some_and(|f| !f.family.starts_with("CMEX") && lossy(&f.family, lost).is_some());
    for page in doc.pages.iter_mut() {
        for line in page.lines.iter_mut() {
            if line.spans.iter().any(empty) {
                let mut spans = std::mem::take(&mut line.spans);
                spans.retain(|s| !empty(s));
                if !spans.is_empty() {
                    *line = Line { col: line.col, bars: std::mem::take(&mut line.bars), ..make_line(spans, &size_of) };
                }
            }
        }
        page.lines.retain(|l| !l.spans.is_empty());
    }
}

/// Pictures on each page of `pdftocairo -svg` output, in pdftohtml's
/// pixels: every drawn path that is not a glyph or a thin bar (fraction
/// bars and rules are used elsewhere), and every embedded image. Pieces
/// close together are one picture, and pictures side by side in a row
/// (four little diagrams) are one too.
pub fn parse_svg_figures(svg: &str) -> Vec<Vec<Figure>> {
    let attr = |tag: &str, name: &str| -> Option<f32> {
        tag.split(&format!(" {name}=\"")).nth(1)?.split('"').next()?.trim_end_matches("px").parse().ok()
    };
    // matrix(a, b, c, d, e, f)
    let transform = |tag: &str| -> [f32; 6] {
        tag.split("transform=\"matrix(")
            .nth(1)
            .and_then(|r| r.split(')').next())
            .map(|m| {
                let v: Vec<f32> = m.split(',').filter_map(|t| t.trim().parse().ok()).collect();
                if v.len() == 6 { [v[0], v[1], v[2], v[3], v[4], v[5]] } else { [1.0, 0.0, 0.0, 1.0, 0.0, 0.0] }
            })
            .unwrap_or([1.0, 0.0, 0.0, 1.0, 0.0, 0.0])
    };
    let apply = |m: &[f32; 6], x: f32, y: f32| (m[0] * x + m[2] * y + m[4], m[1] * x + m[3] * y + m[5]);
    svg_pages(svg)
        .into_iter()
        .map(|page| {
            let body = page.split("</defs>").last().unwrap_or(page);
            // (left, right, top, bottom, is an image)
            let mut boxes: Vec<(f32, f32, f32, f32, bool)> = Vec::new();
            for tag in body.split("<path ").skip(1) {
                let tag = format!(" {}", tag.split("/>").next().unwrap_or(""));
                if tag.contains(" id=\"") {
                    continue;
                }
                let Some(d) = tag.split(" d=\"").nth(1).and_then(|r| r.split('"').next()) else { continue };
                let nums: Vec<f32> = d
                    .split(|c: char| c.is_ascii_alphabetic() || c.is_whitespace() || c == ',')
                    .filter_map(|t| t.parse().ok())
                    .collect();
                if nums.len() < 4 {
                    continue;
                }
                let m = transform(&tag);
                let pts: Vec<(f32, f32)> = nums.chunks(2).filter(|c| c.len() == 2).map(|c| apply(&m, c[0], c[1])).collect();
                let (l, r) = pts.iter().fold((f32::MAX, f32::MIN), |(a, b), p| (a.min(p.0), b.max(p.0)));
                let (t, b) = pts.iter().fold((f32::MAX, f32::MIN), |(a, b), p| (a.min(p.1), b.max(p.1)));
                // a thin filled bar without curves: a fraction bar or rule
                let bar = !tag.contains("stroke=") && !d.contains(['C', 'Q']) && ((b - t <= 1.5) || (r - l <= 1.5));
                // a background box reaching far off the page (ebooks have
                // them) is no drawing
                let off_page = b - t > 1200.0 || t < -50.0;
                if !bar && !off_page {
                    boxes.push((l, r, t, b, false));
                }
            }
            for tag in body.split("<image ").skip(1) {
                let tag = format!(" {}", tag.split('>').next().unwrap_or(""));
                let (Some(w), Some(h)) = (attr(&tag, "width"), attr(&tag, "height")) else { continue };
                let (x, y) = (attr(&tag, "x").unwrap_or(0.0), attr(&tag, "y").unwrap_or(0.0));
                let m = transform(&tag);
                let pts = [apply(&m, x, y), apply(&m, x + w, y + h)];
                let (l, r) = (pts[0].0.min(pts[1].0), pts[0].0.max(pts[1].0));
                let (t, b) = (pts[0].1.min(pts[1].1), pts[0].1.max(pts[1].1));
                boxes.push((l, r, t, b, true));
            }
            // pieces close together form one picture
            let mut groups: Vec<(f32, f32, f32, f32, usize, bool)> =
                boxes.iter().map(|&(l, r, t, b, img)| (l, r, t, b, 1, img)).collect();
            let near = |a: &(f32, f32, f32, f32, usize, bool), b: &(f32, f32, f32, f32, usize, bool)| {
                let gap = 12.0;
                let close = a.0 <= b.1 + gap && b.0 <= a.1 + gap && a.2 <= b.3 + gap && b.2 <= a.3 + gap;
                // side by side in a row
                let v_overlap = a.3.min(b.3) - a.2.max(b.2);
                let row = v_overlap > 0.5 * (a.3 - a.2).min(b.3 - b.2) && (b.0 - a.1).max(a.0 - b.1) < 100.0;
                close || (row && a.4 >= 2 && b.4 >= 2)
            };
            loop {
                let mut merged = false;
                'outer: for i in 0..groups.len() {
                    for j in i + 1..groups.len() {
                        if near(&groups[i], &groups[j]) {
                            let (a, b) = (groups[i], groups[j]);
                            groups[i] = (a.0.min(b.0), a.1.max(b.1), a.2.min(b.2), a.3.max(b.3), a.4 + b.4, a.5 || b.5);
                            groups.remove(j);
                            merged = true;
                            break 'outer;
                        }
                    }
                }
                if !merged {
                    break;
                }
            }
            let mut figs: Vec<Figure> = groups
                .into_iter()
                // a real picture: an image, or several strokes of some size
                // (or a wide strip of many pieces: a legend drawn as paths)
                .filter(|g| g.5 || (g.4 >= 3 && g.1 - g.0 >= 15.0 && g.3 - g.2 >= 15.0) || (g.4 >= 8 && g.1 - g.0 >= 40.0))
                .map(|g| Figure { left: g.0 * 1.5, right: g.1 * 1.5, top: g.2 * 1.5, bottom: g.3 * 1.5 })
                .collect();
            figs.sort_by(|a, b| a.top.total_cmp(&b.top));
            figs
        })
        .collect()
}

/// The labels of a picture (x₁, G₁ …) are text: take the text inside it, or
/// just around it on lines that lie within it, into the picture, and out
/// of the text flow.
fn take_figure_labels(doc: &mut Doc) {
    let m = doc.body_size;
    for page in doc.pages.iter_mut() {
        // A "picture" that would take whole lines of prose as its labels is
        // no picture (background boxes of an ebook page): drop it.
        page.figures.retain(|f| {
            let (l, r, t, b) = (f.left - m, f.right + m, f.top - 1.2 * m, f.bottom + 1.2 * m);
            let prose = page
                .lines
                .iter()
                .filter(|line| line.top < b && line.top + line.size > t && line.left >= l && line.right <= r)
                .filter(|line| line.spans.iter().map(|s| s.text.split_whitespace().filter(|w| w.chars().filter(|c| c.is_alphabetic()).count() >= 3).count()).sum::<usize>() >= 6)
                .count();
            // Nor is a frame or rule drawn around a title (the boxed number
            // and the line under a chapter's title): the labels of a drawing
            // are never words set clearly larger than the book's text.
            // (or a number of a part alone in its frame, "II", beside the title)
            let sizes = &doc.fonts;
            let large = |s: &Span| sizes.get(s.font).map_or(s.height, |f| f.size) >= 1.3 * m;
            let titled = page.lines.iter().any(|line| {
                line.size >= 1.3 * m
                    && line.top >= t
                    && line.top + line.size <= b
                    && line.left >= l
                    && line.right <= r
                    && line.spans.iter().any(|s| s.text.split_whitespace().any(|w| w.chars().filter(|c| c.is_alphabetic()).count() >= 3))
            }) || page.lines.iter().flat_map(|line| line.spans.iter()).any(|s| {
                large(s) && s.left >= f.left - 1.0 && s.right() <= f.right + 1.0 && s.top >= f.top - 0.5 * s.height && s.bottom() <= f.bottom + 0.5 * s.height && s.text.trim().chars().all(|c| "IVXLN0123456789".contains(c)) && !s.text.trim().is_empty()
            });
            prose <= 3 && !titled
        });
        for f in page.figures.iter_mut() {
            let (l, r, t, b) = (f.left - m, f.right + m, f.top - 1.2 * m, f.bottom + 1.2 * m);
            for line in page.lines.iter_mut() {
                // the picture's caption ("Fig. 3.6 …", "Table 5.1 …") is text
                if crate::links::starts_like_caption(&plain_text(line)) {
                    continue;
                }
                // a whole line beside the picture is prose, not a label
                let line_inside = line.left >= l && line.right <= r;
                let spans = std::mem::take(&mut line.spans);
                // ... but a tiny label right beside it, far from the rest of
                // its line (an axis label "x" next to a formula), is one
                // (labels inside the picture are not "the rest of its line")
                let (fl, fr, ft, fb) = (f.left, f.right, f.top, f.bottom);
                let inside = |s: &Span| s.left >= fl - 1.0 && s.right() <= fr + 1.0 && s.top >= ft - 1.0 && s.bottom() <= fb + 1.0;
                let lonely = |s: &Span| {
                    s.text.trim().chars().count() <= 3
                        && spans
                            .iter()
                            .filter(|o| !std::ptr::eq(*o, s) && !inside(o))
                            .all(|o| (s.left - o.right()).max(o.left - s.right()) >= 2.0 * m)
                };
                let mut kept = Vec::new();
                for s in spans.iter() {
                    let inside = inside(s);
                    let near = s.left < r && s.right() > l && s.top < b && s.bottom() > t;
                    let around = near && (line_inside || lonely(s));
                    let s = s.clone();
                    if inside || around {
                        f.left = f.left.min(s.left);
                        f.right = f.right.max(s.right());
                        f.top = f.top.min(s.top);
                        f.bottom = f.bottom.max(s.bottom());
                    } else {
                        kept.push(s);
                    }
                }
                line.spans = kept;
            }
            page.lines.retain(|l| !l.spans.is_empty());
        }
    }
}

/// An image embedded in a page (a raster picture, or in ebook-made PDFs
/// an equation), in pdftohtml's pixels, with its PNG data as base64.
#[derive(Debug, Clone)]
pub struct PageImage {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub png_base64: String,
}

/// The embedded images of each page of `pdftocairo -svg` output. Cairo
/// writes an image either in place or once in <defs> (`<image id=…>`) and
/// places it with `<use xlink:href="#id" transform=…>`.
pub fn parse_svg_images(svg: &str) -> Vec<Vec<PageImage>> {
    let attr = |tag: &str, name: &str| -> Option<String> { Some(tag.split(&format!(" {name}=\"")).nth(1)?.split('"').next()?.to_string()) };
    let num = |tag: &str, name: &str| attr(tag, name).and_then(|v| v.trim_end_matches("px").parse::<f32>().ok());
    let transform = |tag: &str| -> [f32; 6] {
        attr(tag, "transform")
            .and_then(|t| {
                let v: Vec<f32> = t.strip_prefix("matrix(")?.trim_end_matches(')').split(',').filter_map(|x| x.trim().parse().ok()).collect();
                (v.len() == 6).then(|| [v[0], v[1], v[2], v[3], v[4], v[5]])
            })
            .unwrap_or([1.0, 0.0, 0.0, 1.0, 0.0, 0.0])
    };
    let place = |m: [f32; 6], x: f32, y: f32, w: f32, h: f32, data: String| {
        let p = |x: f32, y: f32| (m[0] * x + m[2] * y + m[4], m[1] * x + m[3] * y + m[5]);
        let (a, b) = (p(x, y), p(x + w, y + h));
        PageImage {
            left: a.0.min(b.0) * 1.5,
            right: a.0.max(b.0) * 1.5,
            top: a.1.min(b.1) * 1.5,
            bottom: a.1.max(b.1) * 1.5,
            png_base64: data,
        }
    };
    let image = |tag: &str| -> Option<(f32, f32, f32, f32, String)> {
        let data = attr(tag, "xlink:href")?.strip_prefix("data:image/png;base64,")?.to_string();
        Some((num(tag, "x").unwrap_or(0.0), num(tag, "y").unwrap_or(0.0), num(tag, "width")?, num(tag, "height")?, data))
    };
    // <defs> of a single page, or shared by all pages at the top
    let mut by_id = std::collections::HashMap::new();
    for defs in svg.split("<defs>").skip(1) {
        let defs = defs.split("</defs>").next().unwrap_or("");
        for tag in defs.split("<image ").skip(1) {
            let tag = format!(" {}", tag.split('>').next().unwrap_or(""));
            if let (Some(id), Some(img)) = (attr(&tag, "id"), image(&tag)) {
                by_id.insert(id, img);
            }
        }
    }
    svg_pages(svg)
        .into_iter()
        .map(|page| {
            let body = page.split("</defs>").last().unwrap_or(page);
            let mut out = Vec::new();
            for tag in body.split('<').skip(1) {
                let tag = tag.split('>').next().unwrap_or("");
                if let Some(rest) = tag.strip_prefix("image ") {
                    let rest = format!(" {rest}");
                    if let Some((x, y, w, h, d)) = image(&rest) {
                        out.push(place(transform(&rest), x, y, w, h, d));
                    }
                } else if let Some(rest) = tag.strip_prefix("use ") {
                    let rest = format!(" {rest}");
                    let Some(id) = attr(&rest, "xlink:href").and_then(|h| h.strip_prefix('#').map(str::to_string)) else { continue };
                    if let Some((x, y, w, h, d)) = by_id.get(&id) {
                        out.push(place(transform(&rest), *x, *y, *w, *h, d.clone()));
                    }
                }
            }
            out
        })
        .collect()
}

/// Embedded images: a picture with a "Fig." caption right below it (or
/// at the top of the next page, when the picture ends its page) is a
/// figure; any other image is first given to the math recognizer, and
/// becomes a piece of math in its line (inline, or a displayed equation
/// when it stands alone), or a figure if the recognizer says it is a
/// drawing or cannot read it.
fn place_images(doc: &mut Doc) {
    // "Fig. 2.6", or a bold "Figure 4.1" — not prose that starts with
    // "Figure 2.6 shows …"
    let is_caption = |l: &Line| {
        let t: String = l.spans.iter().map(|s| s.text.as_str()).collect();
        let t = t.trim_start();
        t.strip_prefix("Fig.").is_some_and(|r| r.trim_start().starts_with(|c: char| c.is_ascii_alphanumeric()))
            || (t.starts_with("Figure ") && crate::links::starts_like_caption(t) && l.spans.iter().find(|s| !s.text.trim().is_empty()).is_some_and(|s| s.bold))
    };
    // does each page's following page start with a caption?
    let top_line = |p: &Page| p.lines.iter().min_by(|a, b| a.top.total_cmp(&b.top)).is_some_and(is_caption);
    let next_starts: Vec<bool> = (0..doc.pages.len()).map(|i| doc.pages.get(i + 1).is_some_and(top_line)).collect();
    let captioned = |page: &Page, caption_follows: bool, im: &PageImage| {
        im.bottom - im.top >= 60.0
            && (page.lines.iter().any(|l| l.top >= im.bottom - 2.0 && l.top < im.bottom + 60.0 && is_caption(l))
                // the picture ends its page, the caption starts the next
                || (caption_follows && !page.lines.iter().any(|l| l.top >= im.bottom - 2.0) && !page.images.iter().any(|o| o.top >= im.bottom - 2.0)))
    };
    let wanted: Vec<&str> = doc
        .pages
        .iter()
        .zip(&next_starts)
        .flat_map(|(p, &nx)| p.images.iter().filter(move |im| !captioned(p, nx, im) && !crate::ocr::is_rule(&im.png_base64)).map(|im| im.png_base64.as_str()))
        .collect();
    if doc.pages.iter().all(|p| p.images.is_empty()) {
        return;
    }
    let readings = crate::ocr::recognize(&wanted);
    let font = doc.fonts.len();
    doc.fonts.push(Font { size: doc.body_size, family: "ImageMath".into(), math: true });
    let picture_font = doc.fonts.len();
    doc.fonts.push(Font { size: doc.body_size, family: "ImagePicture".into(), math: false });
    let fonts = doc.fonts.clone();
    let size_of = |s: &Span| fonts.get(s.font).map_or(s.height, |f| f.size);
    for (page, &nx) in doc.pages.iter_mut().zip(&next_starts) {
        // which images are figures by their caption (asked while the page
        // still holds its images)
        let by_caption: Vec<bool> = page.images.iter().map(|im| captioned(page, nx, im)).collect();
        let images = std::mem::take(&mut page.images);
        let mut changed = vec![false; page.lines.len()];
        for (im, &has_caption) in images.iter().zip(&by_caption) {
            let captioned = |_: &Page, _: &PageImage| has_caption;
            if crate::ocr::is_rule(&im.png_base64) {
                page.codes.push((im.top, crate::Block::Rule));
                continue;
            }
            let fig = Figure { left: im.left, right: im.right, top: im.top, bottom: im.bottom };
            let reading = if captioned(page, im) { None } else { readings.get(&crate::ocr::key(&im.png_base64)) };
            if let Some(crate::ocr::Reading::Code(code)) = reading {
                page.codes.push((im.top, crate::Block::Code(code.clone())));
                continue;
            }
            let tex = match reading {
                Some(crate::ocr::Reading::Latex(tex)) => Some(tex.clone()),
                _ => None,
            };
            // not read: a picture; inside a line of text it stays there
            let in_text = |l: &Line| {
                l.top < im.bottom && l.top + l.size > im.top && l.spans.iter().any(|s| s.tex.is_none() && s.text.chars().any(char::is_alphabetic))
            };
            let drawing = matches!(reading, Some(crate::ocr::Reading::Drawing)) || captioned(page, im);
            if tex.is_none() && (drawing || !page.lines.iter().any(in_text)) {
                if std::env::var("PDF2MD_IMGDEBUG").is_ok() {
                    let kind = if captioned(page, im) { "captioned" } else { match reading { Some(crate::ocr::Reading::Drawing) => "drawing", Some(crate::ocr::Reading::Unreadable) => "unreadable", _ => "no reading" } };
                    eprintln!("IMGFIG page {} {:.0}x{:.0} {kind}", page.number, (im.right - im.left) / 1.5 * 96.0 / 72.0, (im.bottom - im.top) / 1.5 * 96.0 / 72.0);
                }
                page.figures.push(fig);
                continue;
            }
            let number = page.number;
            let span = Span {
                text: String::new(),
                left: im.left,
                top: im.top,
                width: im.right - im.left,
                height: im.bottom - im.top,
                font: if tex.is_some() { font } else { picture_font },
                bold: false,
                italic: false,
                script: Script::Normal,
                overline: false,
                picture: tex.is_none().then_some(number),
                tex,
                link: None,
                url: None,
                url_text: None,
                link_top: None,
                image_key: Some(crate::ocr::key(&im.png_base64)),
            };
            // the line it shares the most height with, else a line of its own
            let overlap = |l: &Line| (im.bottom.min(l.top + l.size) - im.top.max(l.top)).max(0.0);
            match (0..page.lines.len()).filter(|&k| overlap(&page.lines[k]) > 0.0).max_by(|&a, &b| overlap(&page.lines[a]).total_cmp(&overlap(&page.lines[b]))) {
                Some(k) => {
                    // inside a piece of text: the text goes around it
                    let spans = &mut page.lines[k].spans;
                    let inside = |s: &Span| s.tex.is_none() && s.picture.is_none() && s.left + 2.0 < im.left && im.right + 2.0 < s.right();
                    if let Some(j) = spans.iter().position(inside) {
                        if let Some((before, after)) = split_around(&spans[j], im.left, im.right) {
                            spans[j] = before;
                            spans.push(after);
                        }
                    }
                    spans.push(span);
                    changed[k] = true;
                }
                None => {
                    page.lines.push(make_line(vec![span], &size_of));
                    changed.push(false);
                }
            }
        }
        for (k, l) in page.lines.iter_mut().enumerate() {
            if changed[k] {
                let col = l.col;
                *l = make_line(std::mem::take(&mut l.spans), &size_of);
                l.col = col;
            }
        }
        page.lines.sort_by(|a, b| a.top.total_cmp(&b.top));
        page.figures.sort_by(|a, b| a.top.total_cmp(&b.top));
    }
}

/// About how wide a character of the text font is, in type sizes: to tell
/// where inside a piece of text a place on the page is.
pub(crate) fn char_width(c: char) -> f32 {
    match c {
        'i' | 'j' | 'l' | 't' | 'f' | 'r' | 'I' | '.' | ',' | ';' | ':' | '\'' | '’' | '′' | '!' | '|' | '(' | ')' | '-' => 0.3,
        ' ' | '\u{a0}' => 0.27,
        'm' | 'w' | 'M' | 'W' => 0.8,
        c if c.is_uppercase() => 0.68,
        _ => 0.48,
    }
}

/// A piece of text with an image somewhere inside it (an ebook's symbol
/// drawn as a picture, in a sentence the PDF holds as one piece): the text
/// before the image and the text after it. The PDF does not say where in
/// the text the image is, only where on the page: the place is the gap
/// between words nearest to it, by the usual widths of the letters -- and
/// of the gaps near there, one the text itself marks as the place: wider
/// than the other gaps of the piece, or before a hyphen or a comma
/// ("an  -polarized beam").
fn split_around(s: &Span, left: f32, right: f32) -> Option<(Span, Span)> {
    let width = char_width;
    let chars: Vec<(usize, char)> = s.text.char_indices().collect();
    let total: f32 = chars.iter().map(|&(_, c)| width(c)).sum();
    if total <= 0.0 || s.width <= 0.0 {
        return None;
    }
    let scale = s.width / total;
    // gaps between words: (first byte, byte after, spaces, where on the page)
    let mut gaps: Vec<(usize, usize, usize, f32)> = Vec::new();
    let (mut x, mut i) = (s.left, 0);
    while i < chars.len() {
        if chars[i].1.is_whitespace() {
            let (start, x0) = (i, x);
            while i < chars.len() && chars[i].1.is_whitespace() {
                x += width(chars[i].1) * scale;
                i += 1;
            }
            if start > 0 && i < chars.len() {
                gaps.push((chars[start].0, chars[i].0, i - start, (x0 + x) / 2.0));
            }
        } else {
            x += width(chars[i].1) * scale;
            i += 1;
        }
    }
    let usual = gaps.iter().map(|g| g.2).min()?;
    let before_mark = |g: &(usize, usize, usize, f32)| s.text[g.1..].starts_with(['-', '‐', ',', '.', ';', ':', ')', '′', '’']);
    let centre = (left + right) / 2.0;
    let off = |g: &(usize, usize, usize, f32)| (g.3 - centre).abs();
    let near = 5.0 * s.width / chars.len() as f32;
    let nearest = |marked: bool| {
        gaps.iter().filter(|g| !marked || ((g.2 > usual || before_mark(g)) && off(g) <= near)).min_by(|a, b| off(a).total_cmp(&off(b)))
    };
    let g = nearest(true).or_else(|| nearest(false))?;
    if std::env::var("PDF2MD_IMGDEBUG").is_ok() {
        let marked = (g.2 > usual || before_mark(g)) && off(g) <= near;
        let mask = |t: &str| t.chars().map(|c| if c.is_alphabetic() { 'w' } else { c }).collect::<String>();
        eprintln!("IMGSPLIT marked={marked} off={:.0} near={near:.0} gaps={} |{}⟦⟧{}|", off(g), gaps.len(), mask(&s.text[..g.0]), mask(&s.text[g.0..]));
    }
    let (mut before, mut after) = (s.clone(), s.clone());
    before.text = format!("{} ", &s.text[..g.0]);
    before.width = (left - s.left - 1.0).max(1.0);
    after.text = if before_mark(g) { s.text[g.1..].to_string() } else { format!(" {}", &s.text[g.1..]) };
    after.left = right + 1.0;
    after.width = (s.right() - right - 1.0).max(1.0);
    Some((before, after))
}

/// Where each internal link goes on its target page: the PDF's link
/// annotation under the span (pdftohtml only tells the page).
fn add_link_positions(doc: &mut Doc, path: &Path) {
    if !doc.pages.iter().flat_map(|p| p.lines.iter().flat_map(|l| &l.spans)).any(|s| s.link.is_some()) {
        return;
    }
    let annots = crate::annots::read_links(path);
    let (mut linked, mut placed) = (0, 0);
    for page in doc.pages.iter_mut() {
        let Some(list) = annots.get(&page.number) else { continue };
        for s in page.lines.iter_mut().flat_map(|l| l.spans.iter_mut()) {
            let Some(n) = s.link else { continue };
            linked += 1;
            let (cx, cy) = ((s.left + s.right()) / 2.0, s.top + s.height / 2.0);
            if let Some(a) = list.iter().find(|a| a.dest_page == n && cx >= a.left - 2.0 && cx <= a.right + 2.0 && cy >= a.top - 4.0 && cy <= a.bottom + 4.0) {
                s.link_top = a.dest_top;
                placed += a.dest_top.is_some() as usize;
            }
        }
    }
    if std::env::var("PDF2MD_LINKDEBUG").is_ok() {
        eprintln!("LINKS annotations on {} pages, {} in all; linked spans {linked}, with a target position {placed}", annots.len(), annots.values().map(Vec::len).sum::<usize>());
    }
}

/// Ebook-made PDFs have no printed page numbers, but their index does: each
/// number links to the spot where that printed page began. Collect those
/// spots (all citations of a page must agree, and the pages must come in
/// order; the few that do not are dropped) as page marks.
fn printed_page_marks(doc: &mut Doc) {
    use std::collections::BTreeMap;
    let mut cited: BTreeMap<u32, Vec<(u32, f32)>> = BTreeMap::new();
    let page_number = |s: &Span| -> Option<(u32, u32, f32)> {
        let (Some(n), Some(top)) = (s.link, s.link_top) else { return None };
        let t = s.text.trim().trim_end_matches([',', ';']);
        if s.script != Script::Normal || t.is_empty() || t.len() > 4 || !t.chars().all(|c| c.is_ascii_digit()) {
            return None;
        }
        Some((t.parse().ok()?, n, top))
    };
    for page in &doc.pages {
        // only index pages: many linked page numbers (a linked year or
        // volume number elsewhere is no printed page)
        let found: Vec<(u32, u32, f32)> = page.lines.iter().flat_map(|l| &l.spans).filter_map(page_number).collect();
        if found.len() < 15 {
            continue;
        }
        for (p, n, top) in found {
            cited.entry(p).or_default().push((n, top));
        }
    }
    // a page whose citations all go to one spot
    let pairs: Vec<(u32, u32, f32)> = cited
        .into_iter()
        .filter_map(|(p, v)| {
            let (n, top) = v[0];
            v.iter().all(|&(m, t)| m == n && (t - top).abs() < 3.0).then_some((p, n, top))
        })
        .collect();
    // the longest run in which later pages come later in the PDF
    let later = |a: &(u32, u32, f32), b: &(u32, u32, f32)| (b.1, b.2) > (a.1, a.2);
    let mut best: Vec<usize> = vec![1; pairs.len()];
    let mut prev: Vec<Option<usize>> = vec![None; pairs.len()];
    for i in 0..pairs.len() {
        for j in 0..i {
            if later(&pairs[j], &pairs[i]) && best[j] + 1 > best[i] {
                best[i] = best[j] + 1;
                prev[i] = Some(j);
            }
        }
    }
    let Some(mut k) = (0..pairs.len()).max_by_key(|&i| best[i]) else { return };
    if best[k] < 20 {
        return;
    }
    let mut chain = vec![k];
    while let Some(j) = prev[k] {
        chain.push(j);
        k = j;
    }
    chain.reverse();
    for &i in &chain {
        let (p, n, top) = pairs[i];
        if let Some(page) = doc.pages.iter_mut().find(|pg| pg.number == n) {
            page.marks.push((top, p.to_string()));
        }
    }
    // Pages the index does not cite: spaced between the known pages on
    // either side by the amount of text, marked as estimates.
    let mut estimates: Vec<(u32, f32, String)> = Vec::new();
    for w in chain.windows(2) {
        let ((p1, n1, t1), (p2, n2, t2)) = (pairs[w[0]], pairs[w[1]]);
        // (a printed page is about one PDF page: a far larger gap is no
        // run of uncited pages but a wrong pair)
        if p2 <= p1 + 1 || p2 - p1 > 2 * (n2 - n1) + 3 {
            continue;
        }
        // the lines between the two known spots, with their amount of text
        let lines: Vec<(u32, f32, usize)> = doc
            .pages
            .iter()
            .filter(|pg| pg.number >= n1 && pg.number <= n2)
            .flat_map(|pg| pg.lines.iter().map(move |l| (pg.number, l.top, l.spans.iter().map(|s| s.text.chars().count().max(if s.tex.is_some() { 10 } else { 0 })).sum::<usize>())))
            .filter(|&(n, t, _)| (n, t) > (n1, t1 + 1.0) && (n, t) < (n2, t2 - 1.0))
            .collect();
        let total: usize = lines.iter().map(|l| l.2).sum();
        if total == 0 {
            continue;
        }
        let missing = p2 - p1;
        let mut cum = 0usize;
        let mut k = 1;
        for &(n, t, c) in &lines {
            while k < missing && cum >= total * k as usize / missing as usize {
                estimates.push((n, t, format!("{}-approx", p1 + k)));
                k += 1;
            }
            cum += c;
        }
        // (rounding can leave the last ones: on the last line)
        if let Some(&(n, t, _)) = lines.last() {
            while k < missing {
                estimates.push((n, t, format!("{}-approx", p1 + k)));
                k += 1;
            }
        }
    }
    for (n, t, label) in estimates {
        if let Some(page) = doc.pages.iter_mut().find(|pg| pg.number == n) {
            page.marks.push((t, label));
        }
    }
    for page in doc.pages.iter_mut() {
        page.marks.sort_by(|a, b| a.0.total_cmp(&b.0));
    }
}

/// An equation number like "(1.2)", "(3.425b)" or "(A.3)".
fn is_eq_number(t: &str) -> bool {
    let t = t.trim();
    let Some(inner) = t.strip_prefix('(').and_then(|r| r.strip_suffix(')')) else { return false };
    let inner = inner.trim_end_matches(|c: char| c.is_ascii_lowercase());
    let mut parts = inner.split('.');
    let first = parts.next().unwrap_or("");
    let rest: Vec<&str> = parts.collect();
    !first.is_empty()
        && first.chars().all(|c| c.is_ascii_alphanumeric())
        && !rest.is_empty()
        && rest.iter().all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
}

/// Ebook-made PDFs sometimes put an equation's number at the bottom of one
/// page and the equation itself at the top of the next. Move the number
/// onto the equation's line, at the right margin where it belongs.
fn join_split_equations(doc: &mut Doc) {
    let fonts = doc.fonts.clone();
    let size_of = |s: &Span| fonts.get(s.font).map_or(s.height, |f| f.size);
    let is_math = |s: &Span| s.tex.is_some() || fonts.get(s.font).is_some_and(|f| f.math);
    for k in 0..doc.pages.len().saturating_sub(1) {
        let lone_number = doc.pages[k].lines.last().is_some_and(|l| is_eq_number(&l.spans.iter().map(|s| s.text.as_str()).collect::<String>()));
        let next_is_equation = doc.pages[k + 1].lines.first().is_some_and(|l| {
            l.spans.iter().any(|s| s.tex.is_some())
                && l.spans.iter().all(|s| is_math(s) || s.text.trim().chars().all(|c| ".,;".contains(c)))
        });
        if !(lone_number && next_is_equation) {
            continue;
        }
        let Some(number) = doc.pages[k].lines.pop() else { continue };
        let first = &mut doc.pages[k + 1].lines[0];
        let (top, height) = (first.top, first.size);
        let mut spans = std::mem::take(&mut first.spans);
        for mut s in number.spans {
            // keep its x (the right margin); move it onto the equation's line
            s.top = top + (height - s.height) / 2.0;
            spans.push(s);
        }
        let col = first.col;
        *first = make_line(spans, &size_of);
        first.col = col;
    }
}

/// The page parts of `pdftocairo -svg` output (several pages are wrapped
/// in <page> elements; a single page is not).
fn svg_pages(svg: &str) -> Vec<&str> {
    if svg.contains("<page>") {
        svg.split("<page>").skip(1).collect()
    } else {
        vec![svg.split("</defs>").last().unwrap_or(svg)]
    }
}

/// A glyph outline: its points and how much of it is straight or curved.
#[derive(Clone)]
struct Outline {
    pts: Vec<(f32, f32)>,
    straight: f32,
    curve: f32,
    curves: usize,
    /// number of glyphs joined into this outline
    pieces: usize,
    /// left, right, top, bottom
    bbox: (f32, f32, f32, f32),
}

fn outline(d: &str) -> Outline {
    let (mut straight, mut curve) = (0.0f32, 0.0f32);
    let mut pts = Vec::new();
    let mut cmd = ' ';
    let mut cur = (0.0f32, 0.0f32);
    let mut buf: Vec<f32> = Vec::new();
    for t in d.split_whitespace() {
        if let Ok(v) = t.parse::<f32>() {
            buf.push(v);
            if buf.len().is_multiple_of(2) {
                pts.push((buf[buf.len() - 2], buf[buf.len() - 1]));
            }
            let need = match cmd { 'C' => 6, 'Q' => 4, 'M' | 'L' => 2, _ => usize::MAX };
            if buf.len() == need {
                let end = (buf[need - 2], buf[need - 1]);
                let len = ((end.0 - cur.0).powi(2) + (end.1 - cur.1).powi(2)).sqrt();
                match cmd {
                    'L' => straight += len,
                    'C' | 'Q' => curve += len,
                    _ => {}
                }
                cur = end;
                buf.clear();
            }
        } else {
            cmd = t.chars().next().unwrap_or(' ');
            buf.clear();
        }
    }
    let bbox = pts.iter().fold((f32::MAX, f32::MIN, f32::MAX, f32::MIN), |(a, b, c, e), p| {
        (a.min(p.0), b.max(p.0), c.min(p.1), e.max(p.1))
    });
    Outline { pts, straight, curve, curves: d.matches('C').count(), pieces: 1, bbox }
}

/// Classify an outline (one glyph, or the stacked pieces of a tall
/// extensible bracket) as a bracket, from its shape: straight lines with
/// the long vertical strokes on one side are [ or ]; mostly curved outlines
/// whose extreme left (right) reaches the middle height are ( or ); a thin
/// straight bar is |; straight lines meeting in a point at mid-height are
/// ⟨ or ⟩. Returns (kind, min x, max x, min y, max y) in points.
fn classify_outline(o: &Outline) -> Option<(char, f32, f32, f32, f32)> {
    let pts = &o.pts;
    if pts.len() < 4 {
        return None;
    }
    let (x0, x1) = pts.iter().fold((f32::MAX, f32::MIN), |(a, b), p| (a.min(p.0), b.max(p.0)));
    let (y0, y1) = pts.iter().fold((f32::MAX, f32::MIN), |(a, b), p| (a.min(p.1), b.max(p.1)));
    let (w, h) = (x1 - x0, y1 - y0);
    // taller than ordinary text (a 10 pt parenthesis is 10 pt tall), narrow
    if h < 10.8 || w > 0.7 * h {
        return None;
    }
    let mid = (y0 + y1) / 2.0;
    // does the outline reach the middle height near the left (right) edge?
    let reaches_mid = |x: f32| {
        let ys: Vec<f32> = pts.iter().filter(|p| (p.0 - x).abs() <= 0.2 * w).map(|p| p.1).collect();
        let lo = ys.iter().fold(f32::MAX, |a, &b| a.min(b));
        let hi = ys.iter().fold(f32::MIN, |a, &b| a.max(b));
        lo <= mid + 0.2 * h && hi >= mid - 0.2 * h && hi - lo < 0.9 * h
            || ys.iter().any(|&y| (y - mid).abs() <= 0.2 * h)
    };
    let cusp = |x: f32| {
        let ys: Vec<f32> = pts.iter().filter(|p| (p.0 - x).abs() <= 0.25 * w).map(|p| p.1).collect();
        let lo = ys.iter().fold(f32::MAX, |a, &b| a.min(b));
        let hi = ys.iter().fold(f32::MIN, |a, &b| a.max(b));
        !ys.is_empty() && hi - lo < 0.15 * h && (lo + hi) / 2.0 > mid - 0.1 * h && (lo + hi) / 2.0 < mid + 0.1 * h
    };
    // total length of vertical strokes left and right of the middle
    let mid_x = (x0 + x1) / 2.0;
    let (mut vert_left, mut vert_right) = (0.0f32, 0.0f32);
    for k in 0..pts.len() {
        let (a, b) = (pts[k], pts[(k + 1) % pts.len()]);
        if (a.0 - b.0).abs() < 0.05 {
            if a.0 < mid_x {
                vert_left += (a.1 - b.1).abs();
            } else {
                vert_right += (a.1 - b.1).abs();
            }
        }
    }
    // a big root sign (√): its right edge is only reached at the top, where
    // the long stroke ends; an angle bracket reaches it at top and bottom
    let right_ys: Vec<f32> = pts.iter().filter(|p| (p.0 - x1).abs() <= 0.2 * w).map(|p| p.1).collect();
    if !right_ys.is_empty() && right_ys.iter().all(|&y| y <= y0 + 0.25 * h) {
        return None;
    }
    // a brace built from pieces: a point at mid-height on one side, and
    // straight vertical stems (an angle bracket has only slanted strokes)
    let stems = vert_left + vert_right >= 0.5 * h;
    let kind = if o.pieces > 1 && stems && cusp(x1) && !cusp(x0) {
        '}'
    } else if o.pieces > 1 && stems && cusp(x0) && !cusp(x1) {
        '{'
    } else if o.curve <= o.straight {
        if w < 1.2 {
            '|'
        } else if vert_left >= 1.5 * h && vert_right < 0.5 * h {
            '['
        } else if vert_right >= 1.5 * h && vert_left < 0.5 * h {
            ']'
        } else if reaches_mid(x0) && !reaches_mid(x1) {
            '⟨'
        } else if reaches_mid(x1) && !reaches_mid(x0) {
            '⟩'
        } else {
            return None;
        }
    } else if reaches_mid(x0) && !reaches_mid(x1) {
        // a brace has a sharp point at mid-height: near its outer edge the
        // outline is only at the point, where a parenthesis bulges over a
        // long stretch
        if (o.pieces == 1 && o.curves > 30) || cusp(x0) { '{' } else { '(' }
    } else if reaches_mid(x1) && !reaches_mid(x0) {
        if (o.pieces == 1 && o.curves > 30) || cusp(x1) { '}' } else { ')' }
    } else {
        return None; // e.g. an integral sign
    };
    Some((kind, x0, x1, y0, y1))
}

/// Tall brackets on each page of `pdftocairo -svg` output, in pdftohtml's
/// pixels. Glyph outlines are defined once (`<g id="glyph-4-5"><path d=..>`)
/// and placed with `<use xlink:href="#glyph-4-5" x=".." y=".."/>`.
pub fn parse_svg_delims(svg: &str) -> Vec<Vec<Delim>> {
    let mut outlines: std::collections::HashMap<String, Outline> = std::collections::HashMap::new();
    let defs = svg.split("</defs>").next().unwrap_or("");
    for g in defs.split("<g id=\"").skip(1) {
        let Some((id, body)) = g.split_once('"') else { continue };
        let body = body.split("</g>").next().unwrap_or("");
        let Some(d) = body.split(" d=\"").nth(1).and_then(|r| r.split('"').next()) else { continue };
        outlines.insert(id.to_string(), outline(d));
    }
    let attr = |tag: &str, name: &str| -> Option<String> {
        tag.split(&format!(" {name}=\"")).nth(1)?.split('"').next().map(str::to_string)
    };
    svg_pages(svg)
        .into_iter()
        .map(|page| {
            // every placed glyph, moved to its place on the page (points)
            let mut placed: Vec<Outline> = page
                .split("<use ")
                .skip(1)
                .filter_map(|tag| {
                    let tag = format!(" {}", tag.split("/>").next()?);
                    let id = attr(&tag, "xlink:href")?.trim_start_matches('#').to_string();
                    let o = outlines.get(&id)?;
                    let x: f32 = attr(&tag, "x")?.parse().ok()?;
                    let y: f32 = attr(&tag, "y")?.parse().ok()?;
                    let mut o = o.clone();
                    for p in o.pts.iter_mut() {
                        p.0 += x;
                        p.1 += y;
                    }
                    o.bbox = (o.bbox.0 + x, o.bbox.1 + x, o.bbox.2 + y, o.bbox.3 + y);
                    (o.pts.len() >= 4).then_some(o)
                })
                .collect();
            // Tall brackets are often built from pieces (top hook, straight
            // extension, bottom hook) stacked at one x: join touching narrow
            // pieces into one outline before classifying.
            let bbox = |o: &Outline| o.bbox;
            placed.sort_by(|a, b| a.bbox.2.total_cmp(&b.bbox.2));
            // (sorted by top, so many glyphs of the text beside a tall
            // bracket come between its pieces: search all narrow stacks)
            let mut stacks: Vec<Outline> = Vec::new();
            for o in placed {
                let (l, r, t, _) = bbox(&o);
                let narrow = r - l <= 8.0;
                let joined = narrow
                    && stacks.iter_mut().rev().filter(|s| s.bbox.1 - s.bbox.0 <= 8.0 && s.bbox.3 >= t - 3.0).any(|s| {
                        let (sl, sr, _, sb) = bbox(s);
                        let overlap = r.min(sr) - l.max(sl);
                        // (the pieces of a brace share only its thin stem)
                        let fits = sr - sl <= 8.0 && overlap >= (0.5 * (r - l).min(sr - sl)).min(0.9);
                        if fits && t <= sb + 1.0 && t >= sb - 3.0 {
                            s.pts.extend(o.pts.iter().copied());
                            s.straight += o.straight;
                            s.curve += o.curve;
                            s.curves += o.curves;
                            s.pieces += o.pieces;
                            s.bbox = (sl.min(l), sr.max(r), s.bbox.2.min(t), sb.max(o.bbox.3));
                            true
                        } else {
                            false
                        }
                    });
                if !joined {
                    stacks.push(o);
                }
            }
            if std::env::var_os("PDF2MD_SVGDEBUG").is_some() {
                for s in &stacks {
                    if s.bbox.3 - s.bbox.2 > 10.0 && s.bbox.1 - s.bbox.0 < 12.0 {
                        eprintln!("stack {:?} pieces {} -> {:?}", s.bbox, s.pieces, classify_outline(s).map(|c| c.0));
                    }
                }
            }
            stacks
                .iter()
                .filter_map(classify_outline)
                .map(|(kind, x0, x1, y0, y1)| Delim {
                    left: x0 * 1.5,
                    right: x1 * 1.5,
                    top: y0 * 1.5,
                    bottom: y1 * 1.5,
                    kind,
                })
                .collect()
        })
        .collect()
}

/// One span per word, spread over the span's width by character count.
fn split_span_words(s: &Span) -> Vec<Span> {
    let chars: Vec<char> = s.text.chars().collect();
    let n = chars.len().max(1) as f32;
    let mut out = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        }
        let start = i;
        while i < chars.len() && !chars[i].is_whitespace() {
            i += 1;
        }
        let mut t = s.clone();
        t.text = chars[start..i].iter().collect();
        t.left = s.left + s.width * start as f32 / n;
        t.width = s.width * (i - start) as f32 / n;
        out.push(t);
    }
    out
}

/// Put the brackets found in the SVG into the text as spans of the
/// big-symbol font, replacing the empty pieces pdftohtml gives for them (or
/// filling in the ones it dropped). Brackets that are ordinary text-font
/// characters are already there and left alone.
fn insert_delims(doc: &mut Doc, delims: Vec<Vec<Delim>>) {
    let font = match doc.fonts.iter().position(|f| f.family.starts_with("CMEX")) {
        Some(i) => i,
        None => {
            doc.fonts.push(Font { size: doc.body_size, family: "CMEX10".into(), math: true });
            doc.fonts.len() - 1
        }
    };
    let fonts = doc.fonts.clone();
    let body = doc.body_size;
    let is_cmex = |s: &Span| fonts.get(s.font).is_some_and(|f| f.family.starts_with("CMEX"));
    let size_of = |s: &Span| fonts.get(s.font).map_or(s.height, |f| f.size);
    for (page, ds) in doc.pages.iter_mut().zip(delims) {
        let mut changed = vec![false; page.lines.len()];
        for d in ds {
            let overlaps = |s: &Span| s.left < d.right - 0.5 && s.right() > d.left + 0.5 && s.top < d.bottom && s.bottom() > d.top;
            let as_text = page.lines.iter().flat_map(|l| &l.spans).any(|s| {
                // (the bracket is in the text, or it is a stroke of a large
                // letter or figure of a title: the "1" of a chapter's number)
                !is_cmex(s) && overlaps(s) && (s.text.trim().chars().any(|c| "()[]|{}⟨⟩".contains(c)) || (size_of(s) >= 1.3 * body && s.text.chars().any(char::is_alphanumeric)))
            });
            if as_text || page.lines.is_empty() {
                continue;
            }
            for (k, l) in page.lines.iter_mut().enumerate() {
                // A lost bracket can sit in the space inside a span ("c α"
                // with the "(" between them): split such spans into words.
                if l.spans.iter().any(|s| s.left < d.left && s.right() > d.right && s.text.trim().contains(' ') && s.top < d.bottom && s.bottom() > d.top) {
                    l.spans = l
                        .spans
                        .drain(..)
                        .flat_map(|s| {
                            if s.left < d.left && s.right() > d.right && s.text.trim().contains(' ') {
                                split_span_words(&s)
                            } else {
                                vec![s]
                            }
                        })
                        .collect();
                    changed[k] = true;
                }
                // "DX" (a big ⟨ and a Σ in one span): one span per character
                if l.spans.iter().any(|s| is_cmex(s) && overlaps(s) && s.text.trim().chars().count() > 1) {
                    l.spans = l
                        .spans
                        .drain(..)
                        .flat_map(|s| {
                            if is_cmex(&s) && overlaps(&s) && s.text.trim().chars().count() > 1 {
                                let spaced: String = s.text.trim().chars().map(|c| format!("{c} ")).collect();
                                let mut t = s.clone();
                                t.text = spaced.trim_end().to_string();
                                split_span_words(&t)
                            } else {
                                vec![s]
                            }
                        })
                        .collect();
                    changed[k] = true;
                }
                let before = l.spans.len();
                l.spans.retain(|s| {
                    !(is_cmex(s) && overlaps(s) && (s.text.trim().is_empty() || "()[]|{}⟨⟩DE".contains(s.text.trim())))
                });
                changed[k] |= l.spans.len() != before;
            }
            let span = Span {
                text: d.kind.to_string(),
                left: d.left,
                top: d.top,
                width: d.right - d.left,
                height: d.bottom - d.top,
                font,
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
            // the line whose centre is nearest the bracket's centre
            let c = (d.top + d.bottom) / 2.0;
            let k = (0..page.lines.len())
                .filter(|&k| !page.lines[k].spans.is_empty())
                .min_by(|&a, &b| {
                    let ca = page.lines[a].top + page.lines[a].size / 2.0;
                    let cb = page.lines[b].top + page.lines[b].size / 2.0;
                    (ca - c).abs().total_cmp(&(cb - c).abs())
                })
                .unwrap_or(0);
            page.lines[k].spans.push(span);
            changed[k] = true;
        }
        for (k, l) in page.lines.iter_mut().enumerate() {
            if changed[k] && !l.spans.is_empty() {
                let col = l.col;
                *l = make_line(std::mem::take(&mut l.spans), &size_of);
                l.col = col;
            }
        }
        page.lines.retain(|l| !l.spans.is_empty());
    }
}

/// A rule drawn as a stroke, not as a filled rectangle (some typesetters
/// draw their fraction bars so): a thin black line from one point straight
/// across to another,
/// `<path fill="none" stroke-width="0.448" … stroke="rgb(0%, 0%, 0%)" …
/// d="M 0.0007 -0.0005 L 10.53 -0.0005" transform="matrix(1, 0, 0, -1, 158, 320)"/>`.
fn stroked_rule(tag: &str) -> Option<Rule> {
    let attr = |name: &str| tag.split(&format!("{name}=\"")).nth(1)?.split('"').next();
    if attr("fill") != Some("none") || attr("stroke") != Some("rgb(0%, 0%, 0%)") || tag.contains("stroke-dasharray") {
        return None;
    }
    let width: f32 = attr("stroke-width")?.parse().ok()?;
    let d = attr(" d")?;
    let mut words = d.split_whitespace();
    let (Some("M"), Some(x0), Some(y0), Some("L"), Some(x1), Some(y1), None) = (words.next(), words.next(), words.next(), words.next(), words.next(), words.next(), words.next()) else { return None };
    let (x0, y0, x1, y1): (f32, f32, f32, f32) = (x0.parse().ok()?, y0.parse().ok()?, x1.parse().ok()?, y1.parse().ok()?);
    // matrix(a, b, c, d, e, f)
    let m: Vec<f32> = match attr("transform") {
        Some(t) => t.strip_prefix("matrix(")?.trim_end_matches(')').split(',').filter_map(|v| v.trim().parse().ok()).collect(),
        None => vec![1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
    };
    if m.len() != 6 || m[1].abs() > 1e-3 || m[2].abs() > 1e-3 {
        return None;
    }
    let (x0, x1) = ((m[0] * x0 + m[4]).min(m[0] * x1 + m[4]), (m[0] * x0 + m[4]).max(m[0] * x1 + m[4]));
    let (y0, y1) = (m[3] * y0 + m[5], m[3] * y1 + m[5]);
    let width = width * m[3].abs();
    ((y1 - y0).abs() < 0.05 && width <= 1.5 && x1 - x0 >= 2.0).then(|| Rule { left: x0 * 1.5, right: x1 * 1.5, top: (y0 - width / 2.0) * 1.5, bottom: (y0 + width / 2.0) * 1.5 })
}

/// Thin horizontal black rectangles on each page of `pdftocairo -svg`
/// output, in pdftohtml's pixels (1.5 per point).
///
/// A rule is written as
/// `<path fill-rule="nonzero" fill="rgb(0%, 0%, 0%)" ... d="M x0 y0 L x1 y0 L x1 y1 L x0 y1 Z ..."/>`.
/// Glyph outlines live in `<defs>` with an `id`, and figures use strokes
/// or curves, so those are skipped.
pub fn parse_svg_rules(svg: &str) -> Vec<Vec<Rule>> {
    let mut pages = Vec::new();
    for chunk in svg_pages(svg) {
        let mut rules = Vec::new();
        for tag in chunk.split("<path ").skip(1) {
            let tag = tag.split("/>").next().unwrap_or("");
            if let Some(rule) = stroked_rule(tag) {
                rules.push(rule);
                continue;
            }
            if !tag.contains("fill=\"rgb(0%, 0%, 0%)\"") || tag.contains("id=\"") || tag.contains("transform=") {
                continue;
            }
            let Some(d) = tag.split("d=\"").nth(1).and_then(|r| r.split('"').next()) else { continue };
            if d.contains(['C', 'Q', 'A']) {
                continue; // curves: a glyph or a drawing, not a bar
            }
            // one path may hold several rectangles, each closed by Z
            for piece in d.split('Z') {
                let nums: Vec<f32> = piece
                    .split(|c: char| c.is_ascii_alphabetic() || c.is_whitespace() || c == ',')
                    .filter_map(|t| t.parse().ok())
                    .collect();
                if nums.len() < 8 {
                    continue;
                }
                let xs = nums.iter().step_by(2);
                let ys = nums.iter().skip(1).step_by(2);
                let (x0, x1) = xs.fold((f32::MAX, f32::MIN), |(a, b), &x| (a.min(x), b.max(x)));
                let (y0, y1) = ys.fold((f32::MAX, f32::MIN), |(a, b), &y| (a.min(y), b.max(y)));
                if y1 - y0 <= 1.5 && x1 - x0 >= 2.0 {
                    rules.push(Rule { left: x0 * 1.5, right: x1 * 1.5, top: y0 * 1.5, bottom: y1 * 1.5 });
                }
            }
        }
        pages.push(rules);
    }
    pages
}

/// The panels of colour on each page of `pdftocairo -svg` output, in
/// pdftohtml's pixels: rectangles filled with something between black and
/// white, big enough to stand behind a line of text or more (the grey
/// ground of a table and of its caption in an ebook-made PDF).
pub fn parse_svg_shades(svg: &str) -> Vec<Vec<(Rule, f32)>> {
    let mut pages = Vec::new();
    for chunk in svg_pages(svg) {
        let mut shades = Vec::new();
        for tag in chunk.split("<path ").skip(1) {
            let tag = tag.split("/>").next().unwrap_or("");
            if tag.contains("id=\"") || tag.contains("transform=") {
                continue;
            }
            // fill="rgb(86.2%, 86.2%, 87%)": how light it is
            let Some(rgb) = tag.split("fill=\"rgb(").nth(1).and_then(|r| r.split(')').next()) else { continue };
            let parts: Vec<f32> = rgb.split(',').filter_map(|v| v.trim().trim_end_matches('%').parse().ok()).collect();
            if parts.len() != 3 {
                continue;
            }
            let light = parts.iter().sum::<f32>() / 300.0;
            if !(0.3..0.98).contains(&light) {
                continue;
            }
            let Some(d) = tag.split("d=\"").nth(1).and_then(|r| r.split('"').next()) else { continue };
            if d.contains(['C', 'Q', 'A']) {
                continue;
            }
            for piece in d.split('Z') {
                let nums: Vec<f32> = piece.split(|c: char| c.is_ascii_alphabetic() || c.is_whitespace() || c == ',').filter_map(|t| t.parse().ok()).collect();
                if nums.len() != 8 {
                    continue;
                }
                let (x0, x1) = nums.iter().step_by(2).fold((f32::MAX, f32::MIN), |(a, b), &x| (a.min(x), b.max(x)));
                let (y0, y1) = nums.iter().skip(1).step_by(2).fold((f32::MAX, f32::MIN), |(a, b), &y| (a.min(y), b.max(y)));
                if x1 - x0 >= 20.0 && y1 - y0 >= 10.0 {
                    shades.push((Rule { left: x0 * 1.5, right: x1 * 1.5, top: y0 * 1.5, bottom: y1 * 1.5 }, light));
                }
            }
        }
        pages.push(shades);
    }
    pages
}

/// A bar just above a single span, with nothing above the bar (so it is not
/// a fraction bar) and no root sign right before it, is an overline: Ā.
fn mark_overlines(doc: &mut Doc) {
    let fonts = doc.fonts.clone();
    for page in doc.pages.iter_mut() {
        let all: Vec<(f32, f32, f32, f32, String, String)> = page
            .lines
            .iter()
            .flat_map(|l| l.spans.iter())
            .map(|s| {
                let fam = fonts.get(s.font).map_or(String::new(), |f| f.family.clone());
                (s.left, s.right(), s.top, s.bottom(), s.text.clone(), fam)
            })
            .collect();
        let mut overline_bars: Vec<usize> = Vec::new();
        // (line, span, bar left, bar right) of each overline found
        let mut marks: Vec<(usize, usize, f32, f32)> = Vec::new();
        for (ri, r) in page.rules.iter().enumerate() {
            for (li, line) in page.lines.iter_mut().enumerate() {
                for (si, s) in line.spans.iter_mut().enumerate() {
                    // (the bar may sit a little inside the top of the box)
                    let under = r.bottom >= s.top - 4.0 && r.bottom <= s.top + 0.25 * s.height;
                    let fits = r.left >= s.left - 2.0 && r.right <= s.right() + 3.0;
                    // over one letter of a longer span ("ūu") is enough
                    let n = s.text.chars().count().max(1) as f32;
                    if !(under && fits && r.right - r.left >= (0.5 * s.width).min(0.7 * s.width / n)) {
                        continue;
                    }
                    let overlaps = |a: &(f32, f32, f32, f32, String, String)| a.0 < r.right && a.1 > r.left;
                    // a fraction bar has a numerator right above it, lying
                    // within the bar's width (text sticking out past the bar
                    // is just the line above)
                    let above: Vec<_> = all
                        .iter()
                        .filter(|a| overlaps(a) && a.3 <= r.top + 3.0 && a.3 >= r.top - 0.5 * s.height && a.2 < r.top)
                        .collect();
                    // ... together with its scripts (the ⁺ of "A⁺" in the
                    // line above sticks out past the bar of Ā)
                    let scripts: Vec<_> = all
                        .iter()
                        .filter(|t| {
                            above.iter().any(|a| {
                                (t.0 - a.1).abs() <= 1.5 && t.3 - t.2 < 0.85 * (a.3 - a.2) && t.2 < a.3 && t.3 > a.2
                            })
                        })
                        .collect();
                    let numerator = !above.is_empty()
                        && above.iter().chain(&scripts).all(|a| a.0 >= r.left - 2.0 && a.1 <= r.right + 2.0);
                    let root = all.iter().any(|a| {
                        (a.5.starts_with("CMEX") || a.5.starts_with("CMSY") || a.4.contains('√'))
                            && (r.left - a.1).abs() < 3.0
                            && a.2 < s.bottom()
                            && a.3 > r.top - 2.0
                    });
                    if !numerator && !root {
                        marks.push((li, si, r.left, r.right));
                        overline_bars.push(ri);
                    }
                }
            }
        }
        // Mark the letters under each bar; a bar over part of a span splits
        // it (glyphs spread evenly over its width). Last spans first, so the
        // indices stay valid.
        marks.sort_by_key(|m| std::cmp::Reverse((m.0, m.1)));
        for (li, si, bl, br) in marks {
            let s = page.lines[li].spans[si].clone();
            let chars: Vec<char> = s.text.chars().collect();
            let n = chars.len().max(1);
            let w = s.width / n as f32;
            let first = (((bl - s.left) / w).round().max(0.0) as usize).min(n - 1);
            let mut last = (((br - s.left) / w).round().max(first as f32 + 1.0) as usize).min(n);
            // punctuation is never under a bar (",ψ̄")
            let mut first = first;
            let punct = |c: char| c.is_whitespace() || ",;.:()[]".contains(c);
            while first + 1 < last && punct(chars[first]) {
                first += 1;
            }
            while last > first + 1 && punct(chars[last - 1]) {
                last -= 1;
            }
            if first == 0 && last == n {
                page.lines[li].spans[si].overline = true;
                continue;
            }
            let piece = |a: usize, b: usize, over: bool| {
                let mut p = s.clone();
                p.text = chars[a..b].iter().collect();
                p.left = s.left + w * a as f32;
                p.width = w * (b - a) as f32;
                p.overline = over;
                p
            };
            let mut parts = Vec::new();
            if first > 0 {
                parts.push(piece(0, first, s.overline));
            }
            parts.push(piece(first, last, true));
            if last < n {
                parts.push(piece(last, n, s.overline));
            }
            page.lines[li].spans.splice(si..si + 1, parts);
        }
        // What is left are fraction bars and root bars (used by mathtex).
        let mut ri = 0;
        page.rules.retain(|_| {
            ri += 1;
            !overline_bars.contains(&(ri - 1))
        });
    }
}

/// Turn pdftohtml's XML into a [`Doc`]. Separate from [`read_pdf`] so tests
/// can feed it a hand-written XML string.
pub fn parse_xml(xml: &str) -> Result<Doc, String> {
    parse_xml_with(xml, &HashMap::new())
}

/// The family a font goes by here. A big-symbol font that pdftohtml cannot
/// read ("MTEX") goes as TeX's, "CMEX/MTEX": its glyphs are written as
/// pdftohtml writes TeX's (see [`restore_glyphs`]) and read like them.
fn family_name(raw: &str, lost: &HashMap<String, bool>) -> String {
    // "YIEKEP+CMR10" -> "CMR10"
    let family = raw.split_once('+').map_or(raw, |(_, f)| f);
    match lossy(family, lost) {
        Some(true) => format!("CMEX/{family}"),
        _ => family.to_string(),
    }
}

/// Is the font one pdftohtml loses glyphs of (and is it a big-symbol font)?
/// (pdftohtml drops the style from a name: "Scala" for "Scala-Bold")
fn lossy(family: &str, lost: &HashMap<String, bool>) -> Option<bool> {
    let family = family.strip_prefix("CMEX/").unwrap_or(family);
    lost.iter().find(|(base, _)| *base == family || base.strip_prefix(family).is_some_and(|rest| rest.starts_with('-'))).map(|(_, ext)| *ext)
}

/// As [`parse_xml`], told which fonts pdftohtml loses glyphs of
/// ([`crate::glyphs::lost_fonts`]): their empty pieces of text are kept, to
/// be filled in from the PDF.
pub fn parse_xml_with(xml: &str, lost: &HashMap<String, bool>) -> Result<Doc, String> {
    // The XML starts with a <!DOCTYPE ...> line, which roxmltree rejects
    // unless we allow it.
    let opts = roxmltree::ParsingOptions {
        allow_dtd: true,
        ..Default::default()
    };
    let tree = roxmltree::Document::parse_with_options(xml, opts).map_err(|e| e.to_string())?;

    // Fonts are declared once, on the first page that uses them, and then
    // referred to by id from every page. Ids are small numbers, so a Vec
    // indexed by id works; missing ids are filled with a placeholder.
    let mut fonts: Vec<Font> = Vec::new();
    let mut pages = Vec::new();

    let num = |n: roxmltree::Node, name: &str| -> f32 {
        n.attribute(name).and_then(|v| v.parse().ok()).unwrap_or(0.0)
    };

    for page_node in tree.descendants().filter(|n| n.has_tag_name("page")) {
        let mut spans = Vec::new();
        for n in page_node.children().filter(|n| n.is_element()) {
            if n.has_tag_name("fontspec") {
                let id = num(n, "id") as usize;
                let family = family_name(n.attribute("family").unwrap_or(""), lost);
                if fonts.len() <= id {
                    fonts.resize(
                        id + 1,
                        Font { size: 0.0, family: String::new(), math: false },
                    );
                }
                fonts[id] = Font {
                    size: num(n, "size"),
                    math: is_math_font(&family),
                    family,
                };
            } else if n.has_tag_name("text") {
                // The text may be wrapped in <b>, <i> or <a> tags; collect
                // all text inside, and note whether <b>/<i> appeared.
                let text: String = n
                    .descendants()
                    .filter(|d| d.is_text())
                    .filter_map(|d| d.text())
                    .collect();
                let text = clean_text(&text);
                // Zero-width text is rotated (e.g. the arXiv stamp in the
                // margin); it is not part of the book's text. Except the
                // slash of ≠ (CMSY "6" overprinted on "="): pdftohtml gives
                // it no width, so it gets the width of the sign it strikes.
                let not_slash = text.trim() == "6"
                    && fonts.get(num(n, "font") as usize).is_some_and(|f| f.family.contains("CMSY"));
                let width = if not_slash { 0.8 * num(n, "height") } else { num(n, "width") };
                // (nor is an empty piece kept to be filled in: an accent
                // moves the type on by nothing)
                let to_fill = text.trim().is_empty() && fonts.get(num(n, "font") as usize).is_some_and(|f| lossy(&f.family, lost).is_some());
                if width < 0.5 && !to_fill {
                    continue;
                }
                // Empty text is skipped, except for glyphs of TeX's big-symbol
                // font: those are pieces of tall brackets (around matrices),
                // whose position we need even though they have no character.
                let id = num(n, "font") as usize;
                let cmex = fonts.get(id).is_some_and(|f| f.family.starts_with("CMEX"));
                // The AMS symbol font's box (□, the d'Alembertian) also comes
                // out empty; it is by far its most common glyph in physics.
                let msam = fonts.get(id).is_some_and(|f| f.family.starts_with("MSAM"));
                let text = if text.trim().is_empty() && msam { "□".to_string() } else { text };
                let kept = fonts.get(id).is_some_and(|f| lossy(&f.family, lost).is_some());
                // (what pdftohtml gives for a big-symbol font it cannot read
                // is the glyph's number as a character: nothing)
                let text = if fonts.get(id).is_some_and(|f| f.family.starts_with("CMEX/")) { String::new() } else { text };
                if text.trim().is_empty() && !cmex && !kept {
                    continue;
                }
                spans.push(Span {
                    text,
                    left: num(n, "left"),
                    top: num(n, "top"),
                    width,
                    height: num(n, "height"),
                    font: num(n, "font") as usize,
                    bold: n.descendants().any(|d| d.has_tag_name("b")),
                    italic: n.descendants().any(|d| d.has_tag_name("i")),
                    script: Script::Normal,
                    overline: false,
                    tex: None,
                    link: n
                        .descendants()
                        .find(|d| d.has_tag_name("a"))
                        .and_then(|a| a.attribute("href"))
                        .filter(|h| !h.contains("://") && !h.starts_with("mailto:"))
                        .and_then(|h| h.rsplit_once('#'))
                        .and_then(|(_, n)| n.parse().ok()),
                    url: n
                        .descendants()
                        .find(|d| d.has_tag_name("a"))
                        .and_then(|a| a.attribute("href"))
                        .filter(|h| h.starts_with("http://") || h.starts_with("https://") || h.starts_with("mailto:"))
                        .map(str::to_string),
                    url_text: n
                        .descendants()
                        .find(|d| d.has_tag_name("a"))
                        .map(|a| clean_text(&a.descendants().filter(|d| d.is_text()).filter_map(|d| d.text()).collect::<String>())),
                    link_top: None,
                    picture: None,
                    image_key: None,
                });
            }
        }
        pages.push((
            num(page_node, "number") as u32,
            num(page_node, "width"),
            num(page_node, "height"),
            spans,
        ));
    }

    let size_of = |s: &Span| fonts.get(s.font).map_or(s.height, |f| f.size);

    // Body size: count characters per font size, take the most common.
    let mut counts: Vec<(f32, usize)> = Vec::new();
    for (_, _, _, spans) in &pages {
        for s in spans {
            let size = size_of(s);
            let n = s.text.chars().count();
            match counts.iter_mut().find(|(sz, _)| *sz == size) {
                Some((_, c)) => *c += n,
                None => counts.push((size, n)),
            }
        }
    }
    let body_size = counts.iter().max_by_key(|(_, c)| *c).map_or(0.0, |(s, _)| *s);

    let pages = pages
        .into_iter()
        .map(|(number, width, height, spans)| Page {
            number,
            width,
            height,
            lines: group_lines(spans, &size_of, &|s: &Span| {
                fonts.get(s.font).is_some_and(|f| f.family.starts_with("CMEX"))
            }),
            rules: Vec::new(),
            shades: Vec::new(),
            figures: Vec::new(),
            images: Vec::new(),
            marks: Vec::new(),
            codes: Vec::new(),
        })
        .collect();

    let mut doc = Doc { fonts, pages, body_size };
    blank_pages(&mut doc);
    large_capitals(&mut doc);
    title_words(&mut doc);
    Ok(doc)
}

/// A page that says only that it is blank ("This page intentionally left
/// blank") is a blank page.
fn blank_pages(doc: &mut Doc) {
    for page in doc.pages.iter_mut() {
        let words: Vec<String> = page.lines.iter().flat_map(|l| l.spans.iter()).flat_map(|s| s.text.split_whitespace()).map(str::to_lowercase).collect();
        let has = |w: &str| words.iter().any(|x| x.trim_matches(|c: char| !c.is_alphabetic()) == w);
        if words.len() <= 8 && has("page") && has("intentionally") && has("blank") {
            page.lines.clear();
        }
    }
}

/// A title's first letter set larger than the rest ("Q" and "uantum Field
/// Theory") is the first letter of its word: one piece of text with it.
fn large_capitals(doc: &mut Doc) {
    let fonts = doc.fonts.clone();
    let size_of = |s: &Span| fonts.get(s.font).map_or(s.height, |f| f.size);
    for line in doc.pages.iter_mut().flat_map(|p| p.lines.iter_mut()) {
        let [first, second, ..] = line.spans.as_slice() else { continue };
        let mut letters = first.text.trim().chars();
        let capital = matches!((letters.next(), letters.next()), (Some(c), None) if c.is_uppercase());
        let goes_on = second.text.starts_with(char::is_lowercase) && second.left - first.right() <= size_of(first);
        // (in a title: both larger than the text, and no math, where a
        // capital has its index after it)
        let title = size_of(second) >= 1.15 * doc.body_size && [first, second].iter().all(|s| !fonts.get(s.font).is_some_and(|f| f.math));
        if !capital || !goes_on || !title || size_of(first) < 1.15 * size_of(second) {
            continue;
        }
        let mut spans = std::mem::take(&mut line.spans);
        let capital = spans.remove(0);
        spans[0].text = format!("{}{}", capital.text.trim(), spans[0].text);
        spans[0].width = spans[0].right() - capital.left;
        spans[0].left = capital.left;
        *line = Line { col: line.col, bars: std::mem::take(&mut line.bars), ..make_line(spans, &size_of) };
    }
}

/// Words larger than the book's text are no index or exponent: a title
/// beside its much larger chapter number ("I.1 Who Needs It?") is on the
/// line like the number.
fn title_words(doc: &mut Doc) {
    let fonts = &doc.fonts;
    let body = doc.body_size;
    for line in doc.pages.iter_mut().flat_map(|p| p.lines.iter_mut()) {
        if line.size < 1.3 * body {
            continue;
        }
        let mut right = f32::MIN;
        for s in line.spans.iter_mut() {
            let font = fonts.get(s.font);
            let size = font.map_or(s.height, |f| f.size);
            let words = s.text.split_whitespace().any(|w| w.chars().filter(|c| c.is_alphabetic()).count() >= 3);
            if s.script != Script::Normal && size >= body && words && !font.is_some_and(|f| f.math) {
                s.script = Script::Normal;
                // (the title's second line starts under its first)
                if s.left < right && !s.text.starts_with(' ') {
                    s.text.insert(0, ' ');
                }
            }
            right = right.max(s.right());
        }
    }
}

/// Group a page's spans into lines and mark superscripts/subscripts.
///
/// Two passes:
/// 1. Big spans first (largest font size first). A big span joins a line when
///    its vertical middle falls inside that line's *main* span (its first,
///    largest span); otherwise it starts a new line.
/// 2. Small spans (clearly smaller than the biggest text on the page) then
///    join the line they overlap vertically the most. A raised "′" at
///    252..262 next to a "ψ" at 255..270 therefore joins ψ's line even though
///    it starts higher up. A small span overlapping no line (the "3" above a
///    summation sign, say) gets a line of its own.
///
/// Glyphs of TeX's big-symbol font (tall brackets, big Σ, root signs) are
/// reported with a small box at the *top* of the glyph, while the glyph
/// reaches down over the formula. They are set aside and then attached to
/// the nearest line at or below their top.
fn group_lines(
    spans: Vec<Span>,
    size_of: &dyn Fn(&Span) -> f32,
    big_symbol: &dyn Fn(&Span) -> bool,
) -> Vec<Line> {
    let (symbols, mut spans): (Vec<Span>, Vec<Span>) = spans.into_iter().partition(|s| big_symbol(s));
    // Largest font first; within one size, top to bottom, left to right.
    spans.sort_by(|a, b| {
        size_of(b)
            .total_cmp(&size_of(a))
            .then(a.top.total_cmp(&b.top))
            .then(a.left.total_cmp(&b.left))
    });

    // Each group is one line; its first span is the main span.
    let mut groups: Vec<Vec<Span>> = Vec::new();
    for s in spans {
        let small = groups
            .first()
            .is_some_and(|g| size_of(&s) < size_of(&g[0]) * 0.85);
        let found = if small {
            // vertical overlap between the span and each line's main span
            groups
                .iter_mut()
                .map(|g| {
                    let overlap = s.bottom().min(g[0].bottom()) - s.top.max(g[0].top);
                    (overlap, g)
                })
                .filter(|(overlap, _)| *overlap > 0.0)
                .max_by(|a, b| a.0.total_cmp(&b.0))
                .map(|(_, g)| g)
        } else {
            let mid = s.top + s.height / 2.0;
            groups
                .iter_mut()
                .find(|g| mid >= g[0].top && mid <= g[0].bottom())
        };
        match found {
            Some(g) => g.push(s),
            None => groups.push(vec![s]),
        }
    }
    // A root sign's box sits high, over the line above its radicand: one
    // hanging well below its line moves to the line its bottom falls in.
    // (a √ on a line of its own, between two lines, moves too)
    for k in 0..groups.len() {
        let alone = groups[k].iter().all(|s| s.text.trim() == "√");
        let mut j = if alone { 0 } else { 1 };
        while j < groups[k].len() {
            let s = &groups[k][j];
            let low = s.bottom() - 2.0;
            let hangs = alone || s.bottom() > groups[k][0].bottom() + 0.2 * s.height;
            let target = (s.text.trim() == "√" && hangs)
                .then(|| {
                    // (a line of root signs that have all moved away is empty)
                    (0..groups.len()).find(|&m| m != k && !groups[m].is_empty() && groups[m][0].top > groups[k][0].top && groups[m][0].top <= low && groups[m][0].bottom() >= low)
                })
                .flatten();
            match target {
                Some(m) => {
                    let s = groups[k].remove(j);
                    groups[m].push(s);
                }
                None => j += 1,
            }
        }
    }
    groups.retain(|g| !g.is_empty());
    for s in symbols {
        let below = (0..groups.len())
            .filter(|&k| groups[k][0].top >= s.top - 0.3 * s.height && groups[k][0].top <= s.top + 3.0 * s.height)
            .min_by(|&a, &b| groups[a][0].top.total_cmp(&groups[b][0].top));
        match below {
            Some(k) => groups[k].push(s),
            None => groups.push(vec![s]),
        }
    }
    let mut lines: Vec<Line> = groups.into_iter().map(|g| make_line(g, size_of)).collect();
    lines.sort_by(|a, b| a.top.total_cmp(&b.top));
    lines
}

/// Build a [`Line`] from spans that belong together: find the main (largest)
/// span, mark smaller raised/lowered spans as superscripts/subscripts, and
/// sort left to right.
pub fn make_line(mut g: Vec<Span>, size_of: &dyn Fn(&Span) -> f32) -> Line {
    let m = (0..g.len())
        .max_by(|&a, &b| size_of(&g[a]).total_cmp(&size_of(&g[b])).then(b.cmp(&a)))
        .unwrap_or(0);
    let (main_top, main_bottom, main_size, main_h) =
        (g[m].top, g[m].bottom(), size_of(&g[m]), g[m].height);
    for s in &mut g {
        s.script = Script::Normal;
        // Only clearly smaller text counts as a script; same-size text that
        // is slightly offset is just uneven typesetting.
        if size_of(s) < main_size * 0.85 {
            let shift = main_h * 0.2;
            if s.bottom() < main_bottom - shift {
                s.script = Script::Super;
            } else if s.top > main_top + shift {
                s.script = Script::Sub;
            }
        }
    }
    g.sort_by(|a, b| a.left.total_cmp(&b.left));
    Line {
        top: main_top,
        left: g.first().map_or(0.0, |s| s.left),
        right: g.iter().map(|s| s.right()).fold(0.0, f32::max),
        size: main_size,
        col: 0,
        bars: Vec::new(),
        spans: g,
    }
}

impl Doc {
    /// Font size of a span (from its font declaration).
    pub fn size_of(&self, s: &Span) -> f32 {
        self.fonts.get(s.font).map_or(s.height, |f| f.size)
    }
    /// Is the span set in a math-only font?
    pub fn is_math(&self, s: &Span) -> bool {
        self.fonts.get(s.font).is_some_and(|f| f.math)
    }
    pub fn family(&self, s: &Span) -> &str {
        self.fonts.get(s.font).map_or("", |f| f.family.as_str())
    }
}

/// A line's text without markup, words separated by single spaces.
pub fn plain_text(line: &Line) -> String {
    let mut out = String::new();
    let mut prev: Option<f32> = None;
    for s in &line.spans {
        if let Some(r) = prev {
            if s.left - r > line.size * 0.2 {
                out.push(' ');
            }
        }
        out.push_str(&s.text);
        prev = Some(s.right());
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Human-readable dump for looking at what the PDF really contains:
/// one line per text line with position, size and style, superscripts as
/// `^{..}`, subscripts as `_{..}` and math-font text inside `«..»`.
pub fn dump(doc: &Doc) -> String {
    let mut out = format!("body font size: {}\n\nfonts:\n", doc.body_size);
    for (i, f) in doc.fonts.iter().enumerate() {
        if !f.family.is_empty() {
            let math = if f.math { "  (math)" } else { "" };
            out += &format!("  {i:>3}  {:>4}  {}{math}\n", f.size, f.family);
        }
    }
    for p in &doc.pages {
        out += &format!("\n=== page {} ({} x {})\n", p.number, p.width, p.height);
        for r in &p.rules {
            out += &format!("  bar  top {:.1}  x {:.1}..{:.1}\n", r.top, r.left, r.right);
        }
        for f in &p.figures {
            out += &format!("  figure  y {:.0}..{:.0}  x {:.0}..{:.0}\n", f.top, f.bottom, f.left, f.right);
        }
        out += "   top  left right size style | text\n";
        for l in &p.lines {
            let main = l.spans.iter().max_by(|a, b| a.height.total_cmp(&b.height));
            let style = match main {
                Some(s) if s.bold && s.italic => "bi",
                Some(s) if s.bold => "b",
                Some(s) if s.italic => "i",
                _ => "",
            };
            out += &format!(
                "  {:>4} {:>5} {:>5} {:>4} {:<5} | {}\n",
                l.top, l.left, l.right, l.size, style, line_text(doc, l)
            );
        }
    }
    out
}

/// A line's text with script and math markup (used by [`dump`]).
pub fn line_text(doc: &Doc, line: &Line) -> String {
    let mut out = String::new();
    let mut prev_right: Option<f32> = None;
    for s in &line.spans {
        // Insert a space when there is a visible gap before this span.
        if let Some(r) = prev_right {
            if s.left - r > line.size * 0.2 && s.script == Script::Normal {
                out.push(' ');
            }
        }
        let math = doc.fonts.get(s.font).is_some_and(|f| f.math);
        let body = if math { format!("«{}»", s.text) } else { s.text.clone() };
        match s.script {
            Script::Normal => out += &body,
            Script::Super => out += &format!("^{{{body}}}"),
            Script::Sub => out += &format!("_{{{body}}}"),
        }
        prev_right = Some(s.right());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const XML: &str = r##"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE pdf2xml SYSTEM "pdf2xml.dtd">
<pdf2xml producer="poppler" version="26.10.0">
<page number="12" position="absolute" top="0" left="0" height="1188" width="918">
	<fontspec id="2" size="15" family="YIEKEP+CMR10" color="#000000"/>
	<fontspec id="3" size="15" family="AMQVYY+CMMI10" color="#000000"/>
	<fontspec id="7" size="10" family="NBZHGA+CMMI7" color="#000000"/>
	<fontspec id="8" size="10" family="NRXSJG+CMSY7" color="#000000"/>
<text top="159" left="111" width="452" height="15" font="2">Consider a general transformation: If</text>
<text top="159" left="568" width="9" height="15" font="3">x</text>
<text top="157" left="576" width="3" height="10" font="8">′</text>
<text top="166" left="576" width="7" height="10" font="7">µ</text>
<text top="159" left="590" width="160" height="15" font="2">are the <i>new</i> coordinates:</text>
<text top="208" left="779" width="27" height="15" font="2">(16)</text>
</page>
</pdf2xml>"##;

    #[test]
    fn text_goes_around_an_image_inside_it() {
        let span = |text: &str, left: f32, width: f32| Span {
            text: text.into(),
            left,
            top: 0.0,
            width,
            height: 22.0,
            font: 0,
            bold: false,
            italic: false,
            script: Script::Normal,
            overline: false,
            picture: None,
            tex: None,
            link: None,
            url: None,
            url_text: None,
            link_top: None,
            image_key: None,
        };
        // the image stands before the hyphen, not at the nearer word gap
        let s = span("first  filter  is  an  -polarized  beam,  which", 100.0, 470.0);
        let (a, b) = split_around(&s, 318.0, 330.0).unwrap();
        assert_eq!((a.text.as_str(), b.text.as_str()), ("first  filter  is  an ", "-polarized  beam,  which"));
        assert!(a.right() < 318.0 && b.left > 330.0 && (b.right() - s.right()).abs() < 0.1);
        // two images in one piece: each at its wider gap
        let s = span("Just as   and   in", 100.0, 180.0);
        let (a, b) = split_around(&s, 165.0, 177.0).unwrap();
        assert_eq!((a.text.as_str(), b.text.as_str()), ("Just as ", " and   in"));
        let (c, d) = split_around(&b, 226.0, 238.0).unwrap();
        assert_eq!((c.text.as_str(), d.text.as_str()), (" and ", " in"));
        // no gap between words: left whole
        assert!(split_around(&span("polarized", 100.0, 90.0), 140.0, 150.0).is_none());
    }

    #[test]
    fn reads_rules_from_svg() {
        // one bar (the overline of Ā on Dyson p. ii) and one glyph outline
        let svg = r#"<svg><defs><g id="glyph-0-1"><path d="M 1 1 C 2 2 3 3 4 4 Z"/></g></defs>
<pageSet><page>
<path fill-rule="nonzero" fill="rgb(0%, 0%, 0%)" fill-opacity="1" d="M 146.027344 554.171875 L 153.589844 554.171875 L 153.589844 554.652344 L 146.027344 554.652344 Z M 146.027344 554.171875 "/>
</page></pageSet></svg>"#;
        let pages = parse_svg_rules(svg);
        assert_eq!(pages.len(), 1);
        assert_eq!(pages[0].len(), 1);
        let r = pages[0][0];
        assert!((r.left - 219.04).abs() < 0.1 && (r.right - 230.38).abs() < 0.1);
        assert!((r.top - 831.26).abs() < 0.1);
    }

    #[test]
    fn parses_fonts_lines_and_scripts() {
        let doc = parse_xml(XML).unwrap();
        assert_eq!(doc.fonts[2].family, "CMR10");
        assert!(doc.fonts[3].math && !doc.fonts[2].math);
        assert_eq!(doc.body_size, 15.0);

        let page = &doc.pages[0];
        assert_eq!(page.number, 12);
        assert_eq!(page.lines.len(), 2);

        let l = &page.lines[0];
        let scripts: Vec<Script> = l.spans.iter().map(|s| s.script).collect();
        assert_eq!(
            scripts,
            [Script::Normal, Script::Normal, Script::Super, Script::Sub, Script::Normal]
        );
        assert!(l.spans[4].italic);
        assert_eq!(
            line_text(&doc, l),
            "Consider a general transformation: If «x»^{«′»}_{«µ»} are the new coordinates:"
        );
        assert_eq!(line_text(&doc, &page.lines[1]), "(16)");
    }
}
