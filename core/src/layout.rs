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
    const PREFIXES: &[&str] = &["CMMI", "CMSY", "CMEX", "MSAM", "MSBM", "EUFM", "EUEX", "RSFS"];
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

/// Read a PDF. `pages` limits it to a range (first, last), both 1-based.
pub fn read_pdf(path: &Path, pages: Option<(u32, u32)>) -> Result<Doc, String> {
    let mut cmd = Command::new("pdftohtml");
    // -xml: XML with positions; -i: skip images; -q: quiet; -stdout: no files
    cmd.args(["-xml", "-i", "-q", "-stdout", "-nodrm"]);
    if let Some((first, last)) = pages {
        cmd.args(["-f", &first.to_string(), "-l", &last.to_string()]);
    }
    cmd.arg(path);
    let out = cmd.output().map_err(|e| {
        format!("could not run pdftohtml ({e}); install poppler (Arch) or poppler-utils")
    })?;
    if !out.status.success() {
        return Err(format!(
            "pdftohtml failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    let mut doc = parse_xml(&String::from_utf8_lossy(&out.stdout))?;

    // Drawn bars. If pdftocairo is missing the conversion still works,
    // just without overlines.
    let mut cmd = Command::new("pdftocairo");
    cmd.arg("-svg");
    if let Some((first, last)) = pages {
        cmd.args(["-f", &first.to_string(), "-l", &last.to_string()]);
    }
    cmd.arg(path).arg("-");
    if let Ok(out) = cmd.output() {
        if out.status.success() {
            let svg = String::from_utf8_lossy(&out.stdout);
            let rules = parse_svg_rules(&svg);
            for (page, r) in doc.pages.iter_mut().zip(rules) {
                page.rules = r;
            }
            mark_overlines(&mut doc);
            let delims = parse_svg_delims(&svg);
            insert_delims(&mut doc, delims);
        }
    }
    Ok(doc)
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
    let is_cmex = |s: &Span| fonts.get(s.font).is_some_and(|f| f.family.starts_with("CMEX"));
    let size_of = |s: &Span| fonts.get(s.font).map_or(s.height, |f| f.size);
    for (page, ds) in doc.pages.iter_mut().zip(delims) {
        let mut changed = vec![false; page.lines.len()];
        for d in ds {
            let overlaps = |s: &Span| s.left < d.right - 0.5 && s.right() > d.left + 0.5 && s.top < d.bottom && s.bottom() > d.top;
            let as_text = page.lines.iter().flat_map(|l| &l.spans).any(|s| {
                !is_cmex(s) && overlaps(s) && s.text.trim().chars().any(|c| "()[]|{}⟨⟩".contains(c))
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
                let raw = n.attribute("family").unwrap_or("");
                // "YIEKEP+CMR10" -> "CMR10"
                let family = raw.split_once('+').map_or(raw, |(_, f)| f).to_string();
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
                if width < 0.5 {
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
                if text.trim().is_empty() && !cmex {
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
        })
        .collect();

    Ok(Doc { fonts, pages, body_size })
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
                    (0..groups.len()).find(|&m| m != k && groups[m][0].top > groups[k][0].top && groups[m][0].top <= low && groups[m][0].bottom() >= low)
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
