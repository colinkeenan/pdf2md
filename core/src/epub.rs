//! An eBook (EPUB) as a book.
//!
//! An EPUB is a zip file of XHTML pages with a list of their order (the
//! "spine") and a table of contents. Nothing has to be guessed from places
//! on a page: headings, paragraphs, lists, tables, links and footnotes are
//! named as such, and math is either MathML (rewritten as LaTeX here), a
//! picture that carries its LaTeX as its description, or a plain picture
//! (kept as one).
//!
//! The book is written like one made from a PDF: a file for each chapter
//! of the table of contents, `book.md` with all of them, the pictures in
//! `images/`. The reader's notes and fixes are kept between conversions.

use crate::emit;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------- the package

/// The path a link names, seen from the file it stands in: "../x/a.xhtml"
/// from "text/ch1.xhtml" is "x/a.xhtml". Percent signs are read.
fn resolve(from: &str, href: &str) -> String {
    let mut parts: Vec<String> = from.split('/').map(str::to_string).collect();
    parts.pop();
    for p in href.split('/') {
        match p {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            p => parts.push(percent(p)),
        }
    }
    parts.join("/")
}

fn percent(t: &str) -> String {
    let b = t.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' {
            if let Some(v) = t.get(i + 1..i + 3).and_then(|h| u8::from_str_radix(h, 16).ok()) {
                out.push(v);
                i += 3;
                continue;
            }
        }
        out.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// XHTML with the named characters of HTML written by their numbers, so
/// an XML reader takes it (EPUB 2 pages use `&nbsp;` and the like), and
/// without its document type.
fn xml_ready(t: &str) -> String {
    const NAMES: [(&str, u32); 30] = [
        ("nbsp", 160), ("ldquo", 8220), ("rdquo", 8221), ("lsquo", 8216), ("rsquo", 8217), ("ndash", 8211), ("mdash", 8212), ("hellip", 8230), ("copy", 169), ("reg", 174),
        ("trade", 8482), ("times", 215), ("deg", 176), ("middot", 183), ("bull", 8226), ("minus", 8722), ("prime", 8242), ("Prime", 8243), ("shy", 173), ("thinsp", 8201),
        ("ensp", 8194), ("emsp", 8195), ("sect", 167), ("para", 182), ("eacute", 233), ("egrave", 232), ("ouml", 246), ("uuml", 252), ("auml", 228), ("plusmn", 177),
    ];
    let mut out = String::with_capacity(t.len());
    let mut rest = t;
    // (the document type may declare things an XML reader will not fetch)
    if let Some(i) = rest.find("<!DOCTYPE") {
        if let Some(e) = rest[i..].find('>') {
            out.push_str(&rest[..i]);
            rest = &rest[i + e + 1..];
        }
    }
    while let Some(i) = rest.find('&') {
        out.push_str(&rest[..i]);
        rest = &rest[i..];
        let name = rest[1..].split(';').next().unwrap_or("");
        match NAMES.iter().find(|(n, _)| *n == name) {
            Some((n, c)) if rest[1 + n.len()..].starts_with(';') => {
                out.push_str(&format!("&#{c};"));
                rest = &rest[n.len() + 2..];
            }
            _ => {
                out.push('&');
                rest = &rest[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

/// An entry of the table of contents: how deep (1 = a chapter), its
/// words, the page it points into and the place there.
#[derive(Debug, Clone, PartialEq)]
struct Entry {
    depth: usize,
    title: String,
    file: String,
    frag: Option<String>,
}

fn words(n: roxmltree::Node) -> String {
    let mut t = String::new();
    for d in n.descendants().filter(|d| d.is_text()) {
        t.push_str(d.text().unwrap_or(""));
    }
    t.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn target(from: &str, href: &str) -> (String, Option<String>) {
    let (file, frag) = href.split_once('#').map_or((href, None), |(f, g)| (f, Some(percent(g))));
    (if file.is_empty() { from.to_string() } else { resolve(from, file) }, frag.filter(|f| !f.is_empty()))
}

/// The contents of an EPUB 3 navigation page: the list in its `nav` of
/// type "toc" (or its first `nav`).
fn nav_entries(doc: &roxmltree::Document, path: &str) -> Vec<Entry> {
    let navs: Vec<roxmltree::Node> = doc.descendants().filter(|n| n.has_tag_name("nav")).collect();
    let toc = navs.iter().find(|n| n.attributes().any(|a| a.name() == "type" && a.value().split_whitespace().any(|v| v == "toc"))).or(navs.first());
    let mut out = Vec::new();
    fn list(ol: roxmltree::Node, depth: usize, path: &str, out: &mut Vec<Entry>) {
        for li in ol.children().filter(|c| c.has_tag_name("li")) {
            if let Some(a) = li.children().find(|c| c.has_tag_name("a") || c.has_tag_name("span")) {
                let (file, frag) = a.attribute("href").map_or((String::new(), None), |h| target(path, h));
                let title = words(a);
                if !title.is_empty() {
                    out.push(Entry { depth, title, file, frag });
                }
            }
            for inner in li.children().filter(|c| c.has_tag_name("ol") || c.has_tag_name("ul")) {
                list(inner, depth + 1, path, out);
            }
        }
    }
    if let Some(ol) = toc.and_then(|n| n.descendants().find(|c| c.has_tag_name("ol") || c.has_tag_name("ul"))) {
        list(ol, 1, path, &mut out);
    }
    out
}

/// The contents of an EPUB 2 book: the `navMap` of its NCX file.
fn ncx_entries(doc: &roxmltree::Document, path: &str) -> Vec<Entry> {
    let mut out = Vec::new();
    fn points(parent: roxmltree::Node, depth: usize, path: &str, out: &mut Vec<Entry>) {
        for p in parent.children().filter(|c| c.has_tag_name("navPoint")) {
            let title = p.children().find(|c| c.has_tag_name("navLabel")).map(words).unwrap_or_default();
            let src = p.children().find(|c| c.has_tag_name("content")).and_then(|c| c.attribute("src"));
            if let (false, Some(src)) = (title.is_empty(), src) {
                let (file, frag) = target(path, src);
                out.push(Entry { depth, title, file, frag });
            }
            points(p, depth + 1, path, out);
        }
    }
    if let Some(map) = doc.descendants().find(|n| n.has_tag_name("navMap")) {
        points(map, 1, path, &mut out);
    }
    out
}

struct Package {
    title: String,
    /// the pages in reading order (paths inside the EPUB)
    spine: Vec<String>,
    toc: Vec<Entry>,
    /// the navigation page itself, if it is one of the spine's pages
    nav: Option<String>,
}

fn read_package(root: &Path) -> Result<Package, String> {
    let read = |p: &str| std::fs::read_to_string(root.join(p)).map_err(|e| format!("{p}: {e}"));
    let container = read("META-INF/container.xml")?;
    let doc = roxmltree::Document::parse(&container).map_err(|e| format!("container.xml: {e}"))?;
    let opf_path = doc.descendants().find(|n| n.has_tag_name("rootfile")).and_then(|n| n.attribute("full-path")).ok_or("the EPUB names no package file")?.to_string();
    let opf = read(&opf_path)?;
    let doc = roxmltree::Document::parse(&opf).map_err(|e| format!("{opf_path}: {e}"))?;
    let title = doc.descendants().find(|n| n.has_tag_name("title")).map(words).unwrap_or_default();
    // id -> (path, kind, properties)
    let mut items: HashMap<&str, (String, &str, &str)> = HashMap::new();
    for i in doc.descendants().filter(|n| n.has_tag_name("item")) {
        if let (Some(id), Some(href)) = (i.attribute("id"), i.attribute("href")) {
            items.insert(id, (resolve(&opf_path, href), i.attribute("media-type").unwrap_or(""), i.attribute("properties").unwrap_or("")));
        }
    }
    let spine_node = doc.descendants().find(|n| n.has_tag_name("spine"));
    let spine: Vec<String> = spine_node
        .iter()
        .flat_map(|s| s.children())
        .filter(|c| c.has_tag_name("itemref"))
        .filter_map(|r| r.attribute("idref").and_then(|id| items.get(id)).map(|i| i.0.clone()))
        .collect();
    if spine.is_empty() {
        return Err("the EPUB lists no pages".into());
    }
    let nav = items.values().find(|i| i.2.split_whitespace().any(|p| p == "nav")).map(|i| i.0.clone());
    let ncx = spine_node.and_then(|s| s.attribute("toc")).and_then(|id| items.get(id)).or_else(|| items.values().find(|i| i.1 == "application/x-dtbncx+xml")).map(|i| i.0.clone());
    let mut toc = Vec::new();
    if let Some(p) = &nav {
        if let Ok(t) = read(p) {
            if let Ok(d) = roxmltree::Document::parse(&xml_ready(&t)) {
                toc = nav_entries(&d, p);
            }
        }
    }
    if toc.is_empty() {
        if let Some(p) = &ncx {
            if let Ok(t) = read(p) {
                if let Ok(d) = roxmltree::Document::parse(&xml_ready(&t)) {
                    toc = ncx_entries(&d, p);
                }
            }
        }
    }
    Ok(Package { title, nav: nav.filter(|n| spine.contains(n)), spine, toc })
}

// ---------------------------------------------------------------- MathML

/// A character of a formula as LaTeX.
fn math_char(c: char) -> String {
    match c {
        '\u{2061}' | '\u{2062}' | '\u{2063}' | '\u{2064}' | '\u{200b}' => String::new(),
        '−' => "-".into(),
        '√' => "\\surd ".into(),
        '∈' => "\\in ".into(),
        '∉' => "\\notin ".into(),
        '⊂' => "\\subset ".into(),
        '⊆' => "\\subseteq ".into(),
        '∪' => "\\cup ".into(),
        '∩' => "\\cap ".into(),
        '∀' => "\\forall ".into(),
        '∃' => "\\exists ".into(),
        '∅' => "\\emptyset ".into(),
        '¬' => "\\neg ".into(),
        '∧' => "\\wedge ".into(),
        '∨' => "\\vee ".into(),
        '⇔' => "\\Leftrightarrow ".into(),
        '⇐' => "\\Leftarrow ".into(),
        '↦' => "\\mapsto ".into(),
        '↑' => "\\uparrow ".into(),
        '↓' => "\\downarrow ".into(),
        '⋯' => "\\cdots ".into(),
        '⋮' => "\\vdots ".into(),
        '⋱' => "\\ddots ".into(),
        '∘' => "\\circ ".into(),
        '°' => "^{\\circ}".into(),
        '≲' => "\\lesssim ".into(),
        '≳' => "\\gtrsim ".into(),
        '≐' => "\\doteq ".into(),
        '∬' => "\\iint ".into(),
        '∭' => "\\iiint ".into(),
        '∠' => "\\angle ".into(),
        '⋆' => "\\star ".into(),
        '∖' => "\\setminus ".into(),
        '‖' => "\\|".into(),
        '⌊' => "\\lfloor ".into(),
        '⌋' => "\\rfloor ".into(),
        '⌈' => "\\lceil ".into(),
        '⌉' => "\\rceil ".into(),
        'ℝ' => "\\mathbb{R}".into(),
        'ℂ' => "\\mathbb{C}".into(),
        'ℤ' => "\\mathbb{Z}".into(),
        'ℕ' => "\\mathbb{N}".into(),
        'ℚ' => "\\mathbb{Q}".into(),
        'ℒ' => "\\mathcal{L}".into(),
        'ℋ' => "\\mathcal{H}".into(),
        'ℜ' => "\\Re ".into(),
        'ℑ' => "\\Im ".into(),
        '\u{2032}' => "'".into(),
        '\u{2033}' => "''".into(),
        '\u{a0}' | '\u{2009}' | '\u{2005}' | '\u{2004}' => "\\,".into(),
        '\u{2003}' => "\\quad ".into(),
        '\\' => "\\backslash ".into(),
        '^' => "\\hat{}".into(),
        '~' => "\\sim ".into(),
        '<' => "<".into(),
        '>' => ">".into(),
        c => match crate::mathtex::sym(c) {
            // (ħ is "~" only in the fonts of TeX PDFs)
            Some(s) => s.to_string(),
            None => c.to_string(),
        },
    }
}

fn math_text(t: &str) -> String {
    t.trim().chars().map(math_char).collect::<String>()
}

/// One piece of LaTeX as an argument: in braces unless it is one character.
fn arg(t: &str) -> String {
    let t = t.trim();
    format!("{{{t}}}")
}

fn elements<'a>(n: roxmltree::Node<'a, 'a>) -> Vec<roxmltree::Node<'a, 'a>> {
    n.children().filter(|c| c.is_element()).collect()
}

fn accent(mark: &str, over: bool) -> Option<&'static str> {
    Some(match (mark.trim(), over) {
        ("¯" | "‾" | "\u{304}" | "\u{305}" | "―" | "_", true) => "\\overline",
        ("^" | "ˆ" | "\u{302}", true) => "\\hat",
        ("~" | "˜" | "∼" | "\u{303}", true) => "\\tilde",
        ("→" | "\u{20d7}" | "⇀", true) => "\\vec",
        ("." | "˙" | "\u{307}", true) => "\\dot",
        (".." | "¨" | "\u{308}", true) => "\\ddot",
        ("ˇ" | "\u{30c}", true) => "\\check",
        ("⏞" | "\u{fe37}", true) => "\\overbrace",
        ("_" | "\u{332}" | "¯" | "‾", false) => "\\underline",
        ("⏟" | "\u{fe38}", false) => "\\underbrace",
        _ => return None,
    })
}

const FUNCTIONS: [&str; 30] = [
    "sin", "cos", "tan", "cot", "sec", "csc", "sinh", "cosh", "tanh", "coth", "arcsin", "arccos", "arctan", "exp", "log", "ln", "lim", "max", "min", "sup", "inf", "det", "dim", "ker", "arg", "deg", "gcd", "Pr",
    "lg", "hom",
];

/// A MathML element as LaTeX.
pub fn mathml(n: roxmltree::Node) -> String {
    let kids = elements(n);
    let all = |sep: &str| kids.iter().map(|k| mathml(*k)).filter(|t| !t.trim().is_empty()).collect::<Vec<_>>().join(sep);
    let kid = |i: usize| kids.get(i).map(|k| mathml(*k)).unwrap_or_default();
    let text = || n.descendants().filter(|d| d.is_text()).map(|d| d.text().unwrap_or("")).collect::<String>();
    // (under and over a sum, an integral, a limit: limits)
    let big = |t: &str| ["\\sum", "\\prod", "\\int", "\\oint", "\\lim", "\\max", "\\min", "\\sup", "\\inf", "\\bigcup", "\\bigcap", "\\iint", "\\iiint"].iter().any(|b| t.trim() == *b);
    match n.tag_name().name() {
        "semantics" => {
            // the LaTeX the formula was made from, where the book kept it
            let tex = kids.iter().find(|k| k.has_tag_name("annotation") && k.attribute("encoding").is_some_and(|e| e.contains("tex") || e.contains("TeX") || e.contains("latex")));
            match tex {
                Some(a) => a.text().unwrap_or("").trim().to_string(),
                None => kid(0),
            }
        }
        "annotation" | "annotation-xml" | "none" | "mprescripts" | "maligngroup" | "malignmark" => String::new(),
        "mi" => {
            let raw = text();
            let raw = raw.trim();
            let t = math_text(raw);
            let style = n.attribute("mathvariant").unwrap_or("");
            let letters = raw.chars().count() > 1 && raw.chars().all(|c| c.is_ascii_alphabetic());
            if letters && FUNCTIONS.contains(&raw) {
                format!("\\{raw}")
            } else if style.contains("bold") {
                format!("\\boldsymbol{{{t}}}")
            } else if style == "double-struck" {
                format!("\\mathbb{{{t}}}")
            } else if style == "script" || style == "calligraphic" {
                format!("\\mathcal{{{t}}}")
            } else if style == "fraktur" {
                format!("\\mathfrak{{{t}}}")
            } else if letters || style == "normal" && raw.chars().all(|c| c.is_ascii_alphabetic()) && !raw.is_empty() {
                format!("\\mathrm{{{t}}}")
            } else {
                t
            }
        }
        "mn" => math_text(&text()),
        "mo" => {
            let raw = text();
            match raw.trim() {
                "lim" | "max" | "min" | "sup" | "inf" | "sin" | "cos" | "log" | "ln" | "exp" | "det" => format!("\\{}", raw.trim()),
                "<" => "<".into(),
                t if t.chars().count() > 1 && t.chars().all(|c| c.is_ascii_alphabetic()) => format!("\\operatorname{{{t}}}"),
                t => math_text(t),
            }
        }
        "mtext" => {
            let t = text();
            if t.trim().is_empty() {
                if t.is_empty() { String::new() } else { "\\ ".into() }
            } else {
                format!("\\text{{{}}}", t.replace('\\', "").replace('{', "\\{").replace('}', "\\}").replace('$', "\\$").replace('%', "\\%").replace('&', "\\&").replace('#', "\\#").replace('_', "\\_"))
            }
        }
        "ms" => format!("\\text{{\"{}\"}}", text().trim()),
        "mspace" => match n.attribute("width").and_then(|w| w.trim_end_matches("em").parse::<f32>().ok()) {
            Some(w) if w >= 1.5 => "\\qquad ".into(),
            Some(w) if w >= 0.8 => "\\quad ".into(),
            Some(w) if w > 0.1 => "\\,".into(),
            _ => String::new(),
        },
        "mfrac" => {
            if n.attribute("linethickness").is_some_and(|t| t.trim_end_matches("px").trim_end_matches("pt").trim() == "0") {
                format!("\\genfrac{{}}{{}}{{0pt}}{{}}{}{}", arg(&kid(0)), arg(&kid(1)))
            } else {
                format!("\\frac{}{}", arg(&kid(0)), arg(&kid(1)))
            }
        }
        "msqrt" => format!("\\sqrt{}", arg(&all(" "))),
        "mroot" => format!("\\sqrt[{}]{}", kid(1).trim(), arg(&kid(0))),
        "msub" => format!("{}_{}", base(&kid(0)), arg(&kid(1))),
        "msup" => {
            let e = kid(1);
            // (a prime is written as one)
            if !e.trim().is_empty() && e.trim().chars().all(|c| c == '\'') {
                format!("{}{}", base(&kid(0)), e.trim())
            } else {
                format!("{}^{}", base(&kid(0)), arg(&e))
            }
        }
        "msubsup" => format!("{}_{}^{}", base(&kid(0)), arg(&kid(1)), arg(&kid(2))),
        "munder" => {
            let (b, u) = (kid(0), kid(1));
            let mark = kids.get(1).map(|k| words(*k)).unwrap_or_default();
            if big(&b) || b.trim().starts_with("\\operatorname") {
                format!("{}_{}", b.trim(), arg(&u))
            } else if let Some(a) = accent(&mark, false) {
                format!("{a}{}", arg(&b))
            } else if b.trim().starts_with("\\underbrace") {
                format!("{}_{}", b.trim(), arg(&u))
            } else {
                format!("\\underset{}{}", arg(&u), arg(&b))
            }
        }
        "mover" => {
            let (b, o) = (kid(0), kid(1));
            let mark = kids.get(1).map(|k| words(*k)).unwrap_or_default();
            if big(&b) {
                format!("{}^{}", b.trim(), arg(&o))
            } else if let Some(a) = accent(&mark, true) {
                format!("{a}{}", arg(&b))
            } else if b.trim().starts_with("\\overbrace") {
                format!("{}^{}", b.trim(), arg(&o))
            } else {
                format!("\\overset{}{}", arg(&o), arg(&b))
            }
        }
        "munderover" => {
            let (b, u, o) = (kid(0), kid(1), kid(2));
            if big(&b) || b.trim().starts_with("\\operatorname") {
                format!("{}_{}^{}", b.trim(), arg(&u), arg(&o))
            } else {
                format!("\\overset{}{{\\underset{}{}}}", arg(&o), arg(&u), arg(&b))
            }
        }
        "mmultiscripts" => {
            // base, then pairs after it, then (behind <mprescripts/>) pairs before it
            let at = kids.iter().position(|k| k.has_tag_name("mprescripts")).unwrap_or(kids.len());
            let pair = |v: &[roxmltree::Node]| {
                let mut t = String::new();
                for p in v.chunks(2) {
                    let (lo, hi) = (mathml(p[0]), p.get(1).map(|k| mathml(*k)).unwrap_or_default());
                    if !lo.trim().is_empty() {
                        t += &format!("_{}", arg(&lo));
                    }
                    if !hi.trim().is_empty() {
                        t += &format!("^{}", arg(&hi));
                    }
                }
                t
            };
            let before = if at + 1 < kids.len() { format!("{{}}{}", pair(&kids[at + 1..])) } else { String::new() };
            format!("{before}{}{}", base(&kid(0)), pair(&kids[1.min(kids.len())..at]))
        }
        "mfenced" => {
            let open = n.attribute("open").unwrap_or("(");
            let close = n.attribute("close").unwrap_or(")");
            let sep = n.attribute("separators").unwrap_or(",").trim().chars().next().map(|c| format!(" {c} ")).unwrap_or_else(|| " ".into());
            let fence = |f: &str, left: bool| {
                let t = math_text(f);
                let t = if t.is_empty() { ".".to_string() } else { t };
                format!("\\{}{} ", if left { "left" } else { "right" }, t.trim())
            };
            format!("{}{} {}", fence(open, true), all(&sep), fence(close, false))
        }
        "mtable" => {
            let rows: Vec<String> = kids
                .iter()
                .filter(|r| r.has_tag_name("mtr") || r.has_tag_name("mlabeledtr"))
                .map(|r| {
                    let cells = elements(*r);
                    let (label, cells) = if r.has_tag_name("mlabeledtr") && !cells.is_empty() { (Some(words(cells[0])), &cells[1..]) } else { (None, &cells[..]) };
                    let row = cells.iter().map(|c| mathml(*c).trim().to_string()).collect::<Vec<_>>().join(" & ");
                    match label {
                        Some(l) if !l.is_empty() => format!("{row} \\qquad \\text{{{l}}}"),
                        _ => row,
                    }
                })
                .collect();
            // (lines of an equation set one under another, their signs in a
            // column: right, left, right, left …)
            let align = n.attribute("columnalign").unwrap_or("");
            let lines = align.split_whitespace().take(2).collect::<Vec<_>>() == ["right", "left"] || n.attribute("displaystyle") == Some("true");
            let env = if lines { "aligned" } else { "matrix" };
            format!("\\begin{{{env}}} {} \\end{{{env}}}", rows.join(" \\\\ "))
        }
        "mtr" | "mlabeledtr" => all(" & "),
        "menclose" => {
            let t = all(" ");
            match n.attribute("notation").unwrap_or("") {
                s if s.contains("box") => format!("\\boxed{}", arg(&t)),
                s if s.contains("updiagonalstrike") || s.contains("downdiagonalstrike") => format!("\\cancel{}", arg(&t)),
                s if s.contains("top") => format!("\\overline{}", arg(&t)),
                s if s.contains("bottom") => format!("\\underline{}", arg(&t)),
                _ => t,
            }
        }
        "mphantom" => format!("\\phantom{}", arg(&all(" "))),
        "merror" => String::new(),
        // math, mrow, mstyle, mpadded, mtd and whatever else holds pieces
        _ => {
            // (brackets that grow with what they hold: \left … \right, and
            // around a table alone a matrix in its brackets)
            if kids.len() >= 3 {
                let fence = |k: &roxmltree::Node, open: bool| {
                    let t = words(*k);
                    let ok = k.has_tag_name("mo") && t.chars().count() == 1 && if open { "([{|⟨‖".contains(&t) } else { ")]}|⟩‖".contains(&t) };
                    ok.then_some(t)
                };
                let grows = |k: &roxmltree::Node| k.attribute("stretchy") == Some("true") || k.attribute("fence") == Some("true");
                let (first, last) = (kids[0], kids[kids.len() - 1]);
                if let (Some(o), Some(c)) = (fence(&first, true), fence(&last, false)) {
                    let inner = &kids[1..kids.len() - 1];
                    let tall = inner.iter().any(|k| k.descendants().any(|d| matches!(d.tag_name().name(), "mfrac" | "mtable" | "munderover" | "munder" | "mover")));
                    if (grows(&first) && grows(&last)) || (first.attribute("stretchy").is_none() && last.attribute("stretchy").is_none() && tall && o != "|") {
                        let body = inner.iter().map(|k| mathml(*k)).filter(|t| !t.trim().is_empty()).collect::<Vec<_>>().join(" ");
                        let env = match (o.as_str(), c.as_str()) {
                            ("(", ")") => Some("pmatrix"),
                            ("[", "]") => Some("bmatrix"),
                            ("{", "}") => Some("Bmatrix"),
                            ("|", "|") => Some("vmatrix"),
                            ("‖", "‖") => Some("Vmatrix"),
                            _ => None,
                        };
                        if let ([only], Some(env), Some(rows)) = (inner, env, body.strip_prefix("\\begin{matrix}").and_then(|b| b.strip_suffix("\\end{matrix}"))) {
                            let _ = only;
                            return format!("\\begin{{{env}}}{rows}\\end{{{env}}}");
                        }
                        return format!("\\left{} {body} \\right{}", math_text(&o).trim(), math_text(&c).trim());
                    }
                }
            }
            let t = all(" ");
            if t.is_empty() && kids.is_empty() {
                math_text(&text())
            } else {
                t
            }
        }
    }
}

/// What an index or a power stands on: in braces when it is more than one
/// piece ("{x + y}^2" is not meant, but "{a_b}^c" must not be "a_b^c").
fn base(t: &str) -> String {
    let t = t.trim();
    let simple = t.chars().count() == 1 || (t.starts_with('\\') && t[1..].chars().all(|c| c.is_ascii_alphabetic())) || t.chars().all(|c| c.is_ascii_alphanumeric());
    if simple || t.is_empty() {
        t.to_string()
    } else {
        format!("{{{t}}}")
    }
}

fn tidy_math(t: &str) -> String {
    t.split_whitespace().collect::<Vec<_>>().join(" ")
}

// ---------------------------------------------------------------- pages

/// What marks, in the text made from a page, the place of an id the
/// table of contents points at (a chapter that begins inside a page).
fn cut_mark(id: &str) -> String {
    format!("\u{1}{id}\u{1}")
}

struct Page<'a> {
    /// its path in the EPUB
    path: &'a str,
    /// (page, id) -> the id written (ids are told apart between pages)
    ids: &'a HashMap<(String, String), String>,
    /// page -> the id at its head
    heads: &'a HashMap<String, String>,
    /// page -> file of the book (without .md) it begins in; (page, id) -> file
    files: &'a HashMap<String, String>,
    places: &'a HashMap<(String, String), String>,
    /// ids to cut the page at
    cuts: &'a HashSet<String>,
    /// picture in the EPUB -> name in images/
    pictures: &'a HashMap<String, String>,
    /// the chapter file being written (links inside it need no file name)
    out: String,
    cur: String,
    /// list items open: (numbered, count)
    lists: Vec<(bool, usize)>,
    /// marker of the item not yet written
    item: Option<String>,
    quote: usize,
    pre: bool,
}

fn has_type(n: roxmltree::Node, t: &str) -> bool {
    n.attributes().any(|a| (a.name() == "type" || a.name() == "role") && a.value().split_whitespace().any(|v| v == t || v.strip_prefix("doc-") == Some(t)))
}

fn class_has(n: roxmltree::Node, words: &[&str]) -> bool {
    n.attribute("class").is_some_and(|c| {
        let c = c.to_lowercase();
        words.iter().any(|w| c.contains(w))
    })
}

/// LaTeX kept as the description of a picture of a formula.
fn latex_of(alt: &str) -> Option<String> {
    let t = alt.trim();
    let t = t.strip_prefix("$$").and_then(|x| x.strip_suffix("$$")).or_else(|| t.strip_prefix('$').and_then(|x| x.strip_suffix('$'))).or_else(|| t.strip_prefix("\\(").and_then(|x| x.strip_suffix("\\)"))).or_else(|| t.strip_prefix("\\[").and_then(|x| x.strip_suffix("\\]"))).unwrap_or(t).trim();
    let tex = t.contains('\\') || (t.contains('^') || t.contains('_')) && t.contains('{');
    (tex && !t.is_empty() && t.len() < 4000).then(|| t.to_string())
}

impl Page<'_> {
    fn flush(&mut self) {
        let t = if self.pre { self.cur.clone() } else { self.cur.split_whitespace().collect::<Vec<_>>().join(" ") };
        self.cur.clear();
        if t.trim().is_empty() {
            return;
        }
        let quote = "> ".repeat(self.quote);
        let depth = self.lists.len();
        if let Some(mark) = self.item.take() {
            self.out.push_str(&format!("{quote}{}{mark}{t}\n", "   ".repeat(depth.saturating_sub(1))));
        } else if depth > 0 {
            self.out.push_str(&format!("\n{quote}{}{t}\n", "   ".repeat(depth)));
        } else {
            self.out.push_str(&format!("{quote}{t}\n\n"));
        }
    }

    fn block(&mut self, text: &str) {
        self.flush();
        if !self.lists.is_empty() && !self.out.ends_with("\n\n") {
            self.out.push('\n');
        }
        self.out.push_str(text);
        self.out.push_str("\n\n");
    }

    fn anchor(&mut self, n: roxmltree::Node) {
        let Some(id) = n.attribute("id").or_else(|| n.attribute("name").filter(|_| n.has_tag_name("a"))) else { return };
        if self.cuts.contains(id) {
            self.flush();
            self.out.push_str(&cut_mark(id));
            self.out.push('\n');
        }
        if let Some(new) = self.ids.get(&(self.path.to_string(), id.to_string())) {
            self.cur.push_str(&format!("<a id=\"{new}\"></a>"));
        }
    }

    fn link(&self, href: &str) -> Option<String> {
        if href.starts_with("http://") || href.starts_with("https://") || href.starts_with("mailto:") {
            return Some(href.to_string());
        }
        if href.contains("://") {
            return None;
        }
        let (file, frag) = target(self.path, href);
        match frag {
            Some(f) => {
                let key = (file.clone(), f);
                let id = self.ids.get(&key)?;
                let to = self.places.get(&key).or_else(|| self.files.get(&file))?;
                Some(format!("{to}.md#{id}"))
            }
            None => {
                let to = self.files.get(&file)?;
                Some(match self.heads.get(&file) {
                    Some(id) => format!("{to}.md#{id}"),
                    None => format!("{to}.md"),
                })
            }
        }
    }

    fn inline(&mut self, n: roxmltree::Node) {
        for c in n.children() {
            if c.is_text() {
                let t = c.text().unwrap_or("");
                if self.pre {
                    self.cur.push_str(t);
                } else {
                    self.cur.push_str(&emit::escape_text(t));
                }
                continue;
            }
            if c.is_element() {
                self.node(c);
            }
        }
    }

    /// Inline markup around what an element holds: the marks go inside
    /// the spaces at its ends, as Markdown wants them.
    fn wrapped(&mut self, n: roxmltree::Node, open: &str, close: &str) {
        let at = self.cur.len();
        self.inline(n);
        let inner = self.cur.split_off(at);
        if inner.trim().is_empty() {
            self.cur.push_str(&inner);
            return;
        }
        let lead = &inner[..inner.len() - inner.trim_start().len()];
        let trail = &inner[inner.trim_end().len()..];
        self.cur.push_str(&format!("{lead}{open}{}{close}{trail}", inner.trim()));
    }

    fn math(&mut self, tex: &str, display: bool) {
        let tex = tidy_math(tex);
        if tex.is_empty() {
            return;
        }
        let dollar = emit::math_form() == emit::MathForm::Dollar;
        if display {
            let t = if dollar { format!("$$\n{tex}\n$$") } else { format!("{}{tex}{}", emit::DISPLAY_OPEN, emit::DISPLAY_CLOSE) };
            self.block(&t);
        } else if dollar {
            self.cur.push_str(&format!("${tex}$"));
        } else {
            self.cur.push_str(&format!("{}{}{}", emit::MATH_OPEN, tex.replace('`', "'"), emit::MATH_CLOSE));
        }
    }

    fn table(&mut self, n: roxmltree::Node) {
        self.flush();
        let mut rows: Vec<Vec<String>> = Vec::new();
        for tr in n.descendants().filter(|d| d.has_tag_name("tr")) {
            // (not the rows of a table inside a cell)
            if tr.ancestors().skip(1).find(|a| a.has_tag_name("table")) != Some(n) {
                continue;
            }
            let mut row = Vec::new();
            for td in tr.children().filter(|c| c.has_tag_name("td") || c.has_tag_name("th")) {
                let (out, cur, lists, item, quote) = (std::mem::take(&mut self.out), std::mem::take(&mut self.cur), std::mem::take(&mut self.lists), self.item.take(), std::mem::replace(&mut self.quote, 0));
                self.anchor(td);
                self.inline(td);
                self.flush();
                let cell = std::mem::replace(&mut self.out, out);
                (self.cur, self.lists, self.item, self.quote) = (cur, lists, item, quote);
                // (what was a paragraph or a displayed formula in the cell is a line of it)
                let cell = cell.trim().replace("```math\n", "$`").replace("\n```", "`$").replace("\n\n", "<br>").replace('\n', " ").replace('|', "\\|");
                row.push(cell);
                for _ in 1..td.attribute("colspan").and_then(|c| c.parse::<usize>().ok()).unwrap_or(1) {
                    row.push(String::new());
                }
            }
            if !row.is_empty() {
                rows.push(row);
            }
        }
        let width = rows.iter().map(Vec::len).max().unwrap_or(0);
        if width == 0 {
            return;
        }
        // (a table of one cell is a frame around its words)
        if width == 1 && rows.len() == 1 {
            let t = rows[0][0].replace("<br>", "\n\n");
            self.block(&t);
            return;
        }
        let mut t = String::new();
        for (k, r) in rows.iter().enumerate() {
            let mut r = r.clone();
            r.resize(width, String::new());
            t.push_str(&format!("| {} |\n", r.join(" | ")));
            if k == 0 {
                t.push_str(&format!("|{}\n", "---|".repeat(width)));
            }
        }
        self.block(t.trim_end());
    }

    fn node(&mut self, n: roxmltree::Node) {
        let name = n.tag_name().name();
        // (what a reading system shows nothing of)
        if matches!(name, "script" | "style" | "head" | "template") || n.attribute("hidden").is_some() {
            return;
        }
        match name {
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                self.flush();
                // (cut marks and anchors of the title go before its line)
                self.anchor(n);
                let before = std::mem::take(&mut self.cur);
                self.inline(n);
                let t = self.cur.split_whitespace().collect::<Vec<_>>().join(" ").replace("<br>", " ");
                self.cur.clear();
                if !t.is_empty() {
                    let level: usize = name[1..].parse().unwrap_or(1);
                    // (the level is set when the chapter is put together)
                    self.out.push_str(&format!("\u{2}{level}\u{2} {before}{t}\n\n"));
                } else {
                    self.cur = before;
                }
            }
            "p" | "div" | "section" | "article" | "main" | "body" | "header" | "footer" | "aside" | "figure" | "figcaption" | "nav" | "address" | "details" | "summary" | "center" | "dd" | "dt" | "dl" | "caption" => {
                // (a page number of the printed book)
                if has_type(n, "pagebreak") {
                    self.page_break(n);
                    return;
                }
                self.flush();
                self.anchor(n);
                let display = name == "div" && class_has(n, &["equation", "displaymath", "display-math", "mathdisplay"]);
                if display && self.picture_formula(n) {
                    return;
                }
                if name == "dt" {
                    self.wrapped(n, "**", "**");
                } else {
                    self.inline(n);
                }
                self.flush();
            }
            "blockquote" => {
                self.flush();
                self.anchor(n);
                self.quote += 1;
                self.inline(n);
                self.flush();
                self.quote -= 1;
                self.out.push('\n');
            }
            "ul" | "ol" => {
                self.flush();
                self.anchor(n);
                self.lists.push((name == "ol", n.attribute("start").and_then(|s| s.parse::<usize>().ok()).map_or(0, |s| s.saturating_sub(1))));
                self.inline(n);
                self.flush();
                self.lists.pop();
                if self.lists.is_empty() {
                    self.out.push('\n');
                }
            }
            "li" => {
                self.flush();
                if let Some(l) = self.lists.last_mut() {
                    l.1 += 1;
                    self.item = Some(if l.0 { format!("{}. ", l.1) } else { "- ".to_string() });
                }
                self.anchor(n);
                self.inline(n);
                self.flush();
            }
            "pre" => {
                self.flush();
                self.pre = true;
                self.inline(n);
                let code = std::mem::take(&mut self.cur);
                self.pre = false;
                self.block(&format!("```\n{}\n```", code.trim_matches('\n')));
            }
            "hr" => self.block("---"),
            "table" => {
                self.anchor(n);
                self.table(n);
            }
            "br" => self.cur.push_str(if self.pre { "\n" } else { "<br>" }),
            "em" | "i" | "cite" | "dfn" | "var" => self.wrapped(n, "_", "_"),
            "strong" | "b" => self.wrapped(n, "**", "**"),
            "sup" => self.wrapped(n, "<sup>", "</sup>"),
            "sub" => self.wrapped(n, "<sub>", "</sub>"),
            "u" | "ins" => self.wrapped(n, "<u>", "</u>"),
            "code" | "tt" | "kbd" | "samp" if !self.pre => {
                let at = self.cur.len();
                self.pre = true;
                self.inline(n);
                self.pre = false;
                let t = self.cur.split_off(at);
                if !t.trim().is_empty() {
                    self.cur.push_str(&format!("`{}`", t.trim().replace('`', "'")));
                }
            }
            "a" => {
                self.anchor(n);
                let to = n.attribute("href").and_then(|h| self.link(h));
                let at = self.cur.len();
                self.inline(n);
                let inner = self.cur.split_off(at);
                let words = inner.split_whitespace().collect::<Vec<_>>().join(" ");
                match to {
                    Some(to) if !words.is_empty() => {
                        // (a link into this file needs no file name)
                        let note = has_type(n, "noteref") && !words.contains("<sup>");
                        let link = format!("[{words}]({to})");
                        self.cur.push_str(&if note { format!("<sup>{link}</sup>") } else { link });
                    }
                    _ => self.cur.push_str(&inner),
                }
            }
            "img" | "image" => {
                self.anchor(n);
                let alt = n.attribute("alt").unwrap_or("").trim();
                let src = n.attribute("src").or_else(|| n.attributes().find(|a| a.name() == "href").map(|a| a.value()));
                // a picture of a formula that says its LaTeX
                if let Some(tex) = latex_of(alt) {
                    let display = class_has(n, &["display", "equation", "block"]) || alt.starts_with("$$") || alt.starts_with("\\[") || tex.contains("\\begin{");
                    self.math(&tex, display);
                    return;
                }
                let Some(name) = src.map(|s| resolve(self.path, s)).and_then(|p| self.pictures.get(&p)) else { return };
                let alt = alt.replace(['[', ']', '\n'], " ");
                self.cur.push_str(&format!("![{}](images/{name})", alt.trim()));
            }
            "svg" => {
                // (a picture wrapped in a drawing, as covers are: the picture)
                for i in n.descendants().filter(|d| d.has_tag_name("image")) {
                    self.node(i);
                }
            }
            "math" => {
                let display = n.attribute("display") == Some("block") || n.attribute("mode") == Some("display");
                // (a description in LaTeX beside the MathML, as some books keep it)
                let tex = n.attribute("alttext").and_then(latex_of).filter(|_| !n.descendants().any(|d| d.has_tag_name("annotation"))).unwrap_or_else(|| mathml(n));
                self.math(&tex, display);
            }
            "span" | "small" | "big" | "font" | "abbr" | "q" | "mark" | "label" | "time" | "s" | "del" | "strike" | "ruby" | "bdo" | "bdi" | "wbr" | "tbody" | "thead" | "tr" | "td" | "th" | "object" | "picture" | "noscript" => {
                if has_type(n, "pagebreak") {
                    self.page_break(n);
                    return;
                }
                self.anchor(n);
                // (LaTeX kept in the text between \( \) for a script to draw:
                // MathJax's way)
                if class_has(n, &["math", "tex", "latex", "katex"]) && n.children().all(|c| c.is_text()) {
                    let t = words(n);
                    if let Some(tex) = latex_of(&t) {
                        let display = t.starts_with("\\[") || t.starts_with("$$") || class_has(n, &["display"]);
                        self.math(&tex, display);
                        return;
                    }
                }
                self.inline(n);
            }
            _ => {
                self.anchor(n);
                self.inline(n);
            }
        }
    }

    /// A formula's block that holds nothing but one picture with its LaTeX.
    fn picture_formula(&mut self, n: roxmltree::Node) -> bool {
        let imgs: Vec<roxmltree::Node> = n.descendants().filter(|d| d.has_tag_name("img")).collect();
        let [img] = imgs[..] else { return false };
        let Some(tex) = img.attribute("alt").and_then(latex_of) else { return false };
        // (its number, set beside it)
        let number = words(n);
        let tex = if number.is_empty() { tex } else { format!("{tex} \\qquad \\text{{{number}}}") };
        self.math(&tex, true);
        true
    }

    /// Where a page of the printed book begins: its number, as in a book
    /// made from a PDF (there the number links to the PDF's page).
    fn page_break(&mut self, n: roxmltree::Node) {
        let label = n.attribute("title").or_else(|| n.attribute("aria-label")).map(str::to_string).unwrap_or_else(|| words(n));
        let label = label.trim().trim_start_matches("Page ").trim_start_matches("page ").to_string();
        self.anchor(n);
        if !label.is_empty() && label.len() <= 8 {
            self.cur.push_str(&format!("<sup>[p. {label}]</sup> "));
        }
    }
}

// ---------------------------------------------------------------- the book

struct Chapter {
    title: String,
    slug: String,
    body: String,
}

/// Unpack `epub` and write it as a book into `out`.
pub fn write_epub(epub: &Path, out: &Path) -> Result<Vec<PathBuf>, String> {
    if !epub.is_file() {
        return Err(format!("{} is not a file", epub.display()));
    }
    std::fs::create_dir_all(out).map_err(|e| format!("{}: {e}", out.display()))?;
    let root = std::env::temp_dir().join(format!("pdf2md-epub-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).map_err(|e| format!("{}: {e}", root.display()))?;
    let unzip = std::process::Command::new("unzip").arg("-q").arg("-o").arg(epub).arg("-d").arg(&root).output().map_err(|e| format!("unzip is needed to open an EPUB: {e}"))?;
    // (1: warnings only, the files are there)
    if !unzip.status.success() && unzip.status.code() != Some(1) {
        let _ = std::fs::remove_dir_all(&root);
        return Err(format!("could not open the EPUB: {}", String::from_utf8_lossy(&unzip.stderr).trim()));
    }
    let result = write_unpacked(&root, epub, out);
    let _ = std::fs::remove_dir_all(&root);
    result
}

fn write_unpacked(root: &Path, epub: &Path, out: &Path) -> Result<Vec<PathBuf>, String> {
    let pack = read_package(root)?;
    let mut warnings: Vec<String> = Vec::new();
    // the pages, read (a page that is no XML is left out, and said)
    let mut texts: Vec<(String, String)> = Vec::new();
    for p in &pack.spine {
        // (the page of contents is made anew from the chapters)
        if pack.nav.as_ref() == Some(p) {
            continue;
        }
        match std::fs::read_to_string(root.join(p)) {
            Ok(t) => texts.push((p.clone(), xml_ready(&t))),
            Err(e) => warnings.push(format!("{p}: {e}")),
        }
    }
    let mut docs: Vec<(&str, roxmltree::Document)> = Vec::new();
    for (p, t) in &texts {
        match roxmltree::Document::parse_with_options(t, roxmltree::ParsingOptions { allow_dtd: true, ..Default::default() }) {
            Ok(d) => docs.push((p.as_str(), d)),
            Err(e) => warnings.push(format!("{p} could not be read: {e}")),
        }
    }
    if docs.is_empty() {
        return Err("none of the EPUB's pages could be read".into());
    }
    let order: HashMap<&str, usize> = docs.iter().enumerate().map(|(i, d)| (d.0, i)).collect();

    // Chapters: the entries of the first depth of the contents, each from
    // its place to the next one's. Without contents, a chapter for each page.
    let mut starts: Vec<(usize, Option<String>, String)> = Vec::new();
    for e in pack.toc.iter().filter(|e| e.depth == 1) {
        let Some(&i) = order.get(e.file.as_str()) else { continue };
        // (the first chapter of a page begins at the page's head; so does
        // one whose place is not found in the page)
        let known = e.frag.as_ref().filter(|f| docs[i].1.descendants().any(|n| n.attribute("id") == Some(f.as_str())));
        let frag = known.filter(|_| starts.iter().any(|s| s.0 == i)).cloned();
        if starts.last().is_some_and(|s| s.0 > i || (s.0 == i && frag.is_none())) {
            continue; // out of order, or twice the same place
        }
        starts.push((i, frag, e.title.clone()));
    }
    if starts.is_empty() {
        for (i, (p, d)) in docs.iter().enumerate() {
            let title = d.descendants().find(|n| matches!(n.tag_name().name(), "h1" | "h2" | "h3" | "title")).map(words).filter(|t| !t.is_empty()).unwrap_or_else(|| p.rsplit('/').next().unwrap_or(p).to_string());
            starts.push((i, None, title));
        }
    }
    if starts[0].0 > 0 {
        starts.insert(0, (0, None, "Front Matter".to_string()));
    }
    // names of the chapter files
    let digits = if starts.len() > 99 { 3 } else { 2 };
    let mut slugs: Vec<String> = Vec::new();
    for (k, s) in starts.iter().enumerate() {
        let words = crate::structure::slugify(&s.2);
        let words: String = words.split('-').take(8).collect::<Vec<_>>().join("-");
        slugs.push(format!("{k:0digits$}-{}", if words.is_empty() { "chapter" } else { &words }));
    }
    let chapter_of = |page: usize, cut: Option<&str>| -> usize {
        let mut c = 0;
        for (k, s) in starts.iter().enumerate() {
            if s.0 < page || (s.0 == page && (s.1.is_none() || cut.is_some_and(|x| s.1.as_deref() == Some(x)))) {
                c = k;
            }
        }
        c
    };

    // ids: one name for each (page, id) that is linked to or cut at
    let mut wanted: HashSet<(String, String)> = HashSet::new();
    let mut whole: HashSet<String> = HashSet::new();
    for (p, d) in &docs {
        for a in d.descendants().filter(|n| n.has_tag_name("a")) {
            if let Some(h) = a.attribute("href").filter(|h| !h.contains("://") && !h.starts_with("mailto:")) {
                match target(p, h) {
                    (f, Some(g)) => {
                        wanted.insert((f, g));
                    }
                    (f, None) => {
                        whole.insert(f);
                    }
                }
            }
        }
    }
    let mut count: HashMap<&str, usize> = HashMap::new();
    for (_, id) in &wanted {
        *count.entry(id.as_str()).or_default() += 1;
    }
    let stem = |p: &str| crate::structure::slugify(p.rsplit('/').next().unwrap_or(p).rsplit_once('.').map_or(p, |x| x.0));
    let mut ids: HashMap<(String, String), String> = HashMap::new();
    for (f, id) in &wanted {
        let clean: String = id.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' { c } else { '-' }).collect();
        let new = if count[id.as_str()] > 1 { format!("{}-{clean}", stem(f)) } else { clean };
        ids.insert((f.clone(), id.clone()), new);
    }
    let heads: HashMap<String, String> = whole.iter().filter(|f| order.get(f.as_str()).is_some_and(|&i| !starts.iter().any(|s| s.0 == i && s.1.is_none()))).map(|f| (f.clone(), format!("f-{}", stem(f)))).collect();
    let files: HashMap<String, String> = docs.iter().enumerate().map(|(i, d)| (d.0.to_string(), slugs[chapter_of(i, None)].clone())).collect();
    // (an id's chapter: the last cut before it in its page)
    let mut places: HashMap<(String, String), String> = HashMap::new();
    for (i, (p, d)) in docs.iter().enumerate() {
        let cuts: Vec<&str> = starts.iter().filter(|s| s.0 == i).filter_map(|s| s.1.as_deref()).collect();
        if cuts.is_empty() {
            continue;
        }
        let mut now: Option<&str> = None;
        for n in d.descendants() {
            if let Some(id) = n.attribute("id") {
                if cuts.contains(&id) {
                    now = Some(id);
                }
                places.insert((p.to_string(), id.to_string()), slugs[chapter_of(i, now)].clone());
            }
        }
    }

    // pictures: copied into images/ under their own names
    let images = out.join("images");
    let mut pictures: HashMap<String, String> = HashMap::new();
    let mut taken: HashSet<String> = HashSet::new();
    for (p, d) in &docs {
        for n in d.descendants().filter(|n| n.has_tag_name("img") || n.has_tag_name("image")) {
            // (a picture that is written as its formula is not copied)
            if n.attribute("alt").and_then(latex_of).is_some() {
                continue;
            }
            let Some(src) = n.attribute("src").or_else(|| n.attributes().find(|a| a.name() == "href").map(|a| a.value())) else { continue };
            let path = resolve(p, src);
            if pictures.contains_key(&path) || !root.join(&path).is_file() {
                continue;
            }
            let file = path.rsplit('/').next().unwrap_or(&path);
            let (name, ext) = file.rsplit_once('.').unwrap_or((file, "png"));
            let name: String = name.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' { c } else { '-' }).collect();
            let mut new = format!("{name}.{}", ext.to_lowercase());
            let mut k = 2;
            while !taken.insert(new.clone()) {
                new = format!("{name}-{k}.{}", ext.to_lowercase());
                k += 1;
            }
            std::fs::create_dir_all(&images).map_err(|e| format!("{}: {e}", images.display()))?;
            std::fs::copy(root.join(&path), images.join(&new)).map_err(|e| format!("{path}: {e}"))?;
            pictures.insert(path, new);
        }
    }

    // the pages as Markdown, cut into the chapters
    let mut bodies: Vec<String> = vec![String::new(); starts.len()];
    for (i, (p, d)) in docs.iter().enumerate() {
        let cuts: HashSet<String> = starts.iter().filter(|s| s.0 == i).filter_map(|s| s.1.clone()).collect();
        let mut page = Page { path: p, ids: &ids, heads: &heads, files: &files, places: &places, cuts: &cuts, pictures: &pictures, out: String::new(), cur: String::new(), lists: Vec::new(), item: None, quote: 0, pre: false };
        if let Some(id) = heads.get(*p) {
            page.cur.push_str(&format!("<a id=\"{id}\"></a>"));
        }
        let body = d.descendants().find(|n| n.has_tag_name("body")).unwrap_or(d.root_element());
        page.node(body);
        page.flush();
        let mut text = page.out.as_str();
        let mut chapter = chapter_of(i, None);
        // (before the page's first cut: the chapter the page begins in)
        while let Some(a) = text.find('\u{1}') {
            let Some(len) = text[a + 1..].find('\u{1}') else { break };
            bodies[chapter].push_str(&text[..a]);
            chapter = chapter_of(i, Some(&text[a + 1..a + 1 + len]));
            text = text[a + len + 2..].trim_start_matches('\n');
        }
        bodies[chapter].push_str(text);
    }
    // headings: the highest of a chapter is its title's level
    let mut chapters: Vec<Chapter> = Vec::new();
    for (k, body) in bodies.into_iter().enumerate() {
        let levels: Vec<usize> = body.lines().filter_map(|l| l.strip_prefix('\u{2}').and_then(|r| r.split('\u{2}').next()).and_then(|n| n.parse().ok())).collect();
        let top = levels.iter().copied().min().unwrap_or(1);
        let mut text = String::new();
        for line in body.lines() {
            match line.strip_prefix('\u{2}').and_then(|r| r.split_once('\u{2}')) {
                Some((n, rest)) => {
                    let level = n.parse::<usize>().unwrap_or(1) + 1 - top;
                    text.push_str(&format!("{}{rest}\n", "#".repeat(level.min(6))));
                }
                None => {
                    // (no more than one empty line)
                    if !(line.is_empty() && (text.ends_with("\n\n") || text.is_empty())) {
                        text.push_str(line);
                        text.push('\n');
                    }
                }
            }
        }
        let title = starts[k].2.clone();
        // (a chapter that names no title of its own gets the contents' words)
        if !levels.contains(&top) || levels.is_empty() {
            text = format!("# {}\n\n{text}", emit::escape_text(&title));
        }
        if text.trim().is_empty() {
            continue;
        }
        chapters.push(Chapter { title, slug: slugs[k].clone(), body: text });
    }
    if chapters.is_empty() {
        return Err("the EPUB holds no text".into());
    }
    // (links to a chapter that turned out empty go to the book's first file)
    let title = if pack.title.is_empty() { epub.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default() } else { pack.title.clone() };

    // the reader's notes and fixes of the conversion before
    let mut user = crate::userdata::collect(out);
    crate::userdata::backup(&user, out);
    let pairs: Vec<(&str, &str)> = chapters.iter().map(|c| (c.slug.as_str(), c.body.as_str())).collect();
    crate::userdata::follow_renames(&mut user, &pairs);
    let mut written = Vec::new();
    for (k, c) in chapters.iter().enumerate() {
        let mut nav = Vec::new();
        if k > 0 {
            nav.push(format!("[← {}]({}.md)", emit::escape_text(&chapters[k - 1].title), chapters[k - 1].slug));
        }
        if k + 1 < chapters.len() {
            nav.push(format!("[{} →]({}.md)", emit::escape_text(&chapters[k + 1].title), chapters[k + 1].slug));
        }
        let nav = nav.join(" · ");
        // (links into the file itself need no file name)
        let body = c.body.replace(&format!("]({}.md#", c.slug), "](#");
        let mut s = if nav.is_empty() { body } else { format!("{nav}\n\n---\n\n{}\n---\n\n{nav}\n", body) };
        let lost = crate::userdata::apply(&mut s, &user, Some(&c.slug));
        s.push_str(&emit::lost_notes(&lost));
        let p = out.join(format!("{}.md", c.slug));
        std::fs::write(&p, s).map_err(|e| format!("{}: {e}", p.display()))?;
        written.push(p);
    }
    if chapters.len() > 1 {
        let names: Vec<&str> = chapters.iter().map(|c| c.slug.as_str()).collect();
        let bodies: Vec<String> = chapters.iter().map(|c| c.body.clone()).collect();
        let mut all = emit::join_bodies(&emit::escape_text(&title), &names, &bodies, true);
        let lost = crate::userdata::apply(&mut all, &user, None);
        all.push_str(&emit::lost_notes(&lost));
        let p = out.join("book.md");
        std::fs::write(&p, all).map_err(|e| format!("{}: {e}", p.display()))?;
        written.push(p);
    }
    // The files written last time that are not written now are taken away.
    let list = out.join(".pdf2md-epub");
    let now: Vec<String> = written.iter().filter_map(|p| p.file_name().map(|n| n.to_string_lossy().into_owned())).collect();
    if let Ok(before) = std::fs::read_to_string(&list) {
        for old in before.lines().filter(|l| l.ends_with(".md") && !l.contains('/') && !now.iter().any(|n| n == l)) {
            let _ = std::fs::remove_file(out.join(old));
        }
    }
    let _ = std::fs::write(&list, now.join("\n") + "\n");
    for w in warnings {
        eprintln!("warning: {w}");
    }
    Ok(written)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tex(xml: &str) -> String {
        let doc = roxmltree::Document::parse(xml).unwrap();
        tidy_math(&mathml(doc.root_element()))
    }

    #[test]
    fn mathml_as_latex() {
        assert_eq!(tex("<math><mi>E</mi><mo>=</mo><mi>m</mi><msup><mi>c</mi><mn>2</mn></msup></math>"), "E = m c^{2}");
        assert_eq!(tex("<math><mfrac><mrow><mo>−</mo><mi>ℏ</mi></mrow><mrow><mn>2</mn><mi>m</mi></mrow></mfrac><msqrt><mi>α</mi></msqrt></math>"), "\\frac{- \\hbar}{2 m} \\sqrt{\\alpha}");
        assert_eq!(tex("<math><munderover><mo>∑</mo><mrow><mi>n</mi><mo>=</mo><mn>0</mn></mrow><mi>∞</mi></munderover><msub><mi>a</mi><mi>n</mi></msub></math>"), "\\sum_{n = 0}^{\\infty} a_{n}");
        assert_eq!(tex("<math><mover><mi>x</mi><mo>→</mo></mover><mo>⋅</mo><mover accent=\"true\"><mi>p</mi><mo>^</mo></mover><mi>sin</mi><mi>θ</mi></math>"), "\\vec{x} \\cdot \\hat{p} \\sin \\theta");
        assert_eq!(tex("<math><mrow><mo stretchy=\"true\">(</mo><mtable><mtr><mtd><mn>1</mn></mtd></mtr><mtr><mtd><mn>0</mn></mtd></mtr></mtable><mo stretchy=\"true\">)</mo></mrow><mrow><mo stretchy=\"true\">[</mo><mfrac><mi>a</mi><mi>b</mi></mfrac><mo stretchy=\"true\">]</mo></mrow></math>"), "\\begin{pmatrix} 1 \\\\ 0 \\end{pmatrix} \\left[ \\frac{a}{b} \\right]");
        assert_eq!(tex("<math><mfenced open=\"[\" close=\"]\"><mi>a</mi><mi>b</mi></mfenced><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd></mtr><mtr><mtd><mn>0</mn></mtd><mtd><mn>1</mn></mtd></mtr></mtable></math>"), "\\left[ a , b \\right] \\begin{matrix} 1 & 0 \\\\ 0 & 1 \\end{matrix}");
        // the LaTeX the formula was made from is used as it is
        assert_eq!(tex("<math><semantics><mrow><mi>x</mi></mrow><annotation encoding=\"application/x-tex\">\\vec{x}^2</annotation></semantics></math>"), "\\vec{x}^2");
    }

    #[test]
    fn links_between_pages() {
        assert_eq!(resolve("OEBPS/text/ch1.xhtml", "../images/a%20b.png"), "OEBPS/images/a b.png");
        assert_eq!(target("t/ch1.xhtml", "ch2.xhtml#sec-3"), ("t/ch2.xhtml".to_string(), Some("sec-3".to_string())));
        assert_eq!(target("t/ch1.xhtml", "#fn1"), ("t/ch1.xhtml".to_string(), Some("fn1".to_string())));
    }

    #[test]
    fn named_characters_for_an_xml_reader() {
        assert_eq!(xml_ready("<!DOCTYPE html PUBLIC \"x\" \"y\"><p>a&nbsp;b &amp; c&mdash;d</p>"), "<p>a&#160;b &amp; c&#8212;d</p>");
    }

    #[test]
    fn a_small_book() {
        let dir = std::env::temp_dir().join(format!("pdf2md-epub-test-{}", std::process::id()));
        let (root, out) = (dir.join("in"), dir.join("out"));
        std::fs::create_dir_all(root.join("META-INF")).unwrap();
        std::fs::create_dir_all(root.join("t")).unwrap();
        std::fs::write(root.join("META-INF/container.xml"), "<container><rootfiles><rootfile full-path=\"c.opf\"/></rootfiles></container>").unwrap();
        std::fs::write(
            root.join("c.opf"),
            "<package><metadata><title>A Book</title></metadata><manifest><item id=\"n\" href=\"nav.xhtml\" properties=\"nav\"/><item id=\"a\" href=\"t/a.xhtml\"/><item id=\"b\" href=\"t/b.xhtml\"/><item id=\"p\" href=\"t/pic.png\"/></manifest><spine><itemref idref=\"a\"/><itemref idref=\"b\"/></spine></package>",
        )
        .unwrap();
        std::fs::write(root.join("nav.xhtml"), "<html><body><nav type=\"toc\"><ol><li><a href=\"t/a.xhtml\">One</a></li><li><a href=\"t/b.xhtml#two\">Two</a><ol><li><a href=\"t/b.xhtml#s\">Deeper</a></li></ol></li><li><a href=\"t/b.xhtml#three\">Three</a></li></ol></nav></body></html>").unwrap();
        std::fs::write(root.join("t/pic.png"), "x").unwrap();
        std::fs::write(
            root.join("t/a.xhtml"),
            "<html><body><h2>One</h2><p>See <a href=\"b.xhtml#s\">the <em>section</em></a> and <math><msub><mi>x</mi><mn>1</mn></msub></math>.</p><ul><li>first</li><li>second <b>bold</b></li></ul><p><img src=\"pic.png\" alt=\"A picture\"/></p><math display=\"block\"><mi>a</mi><mo>=</mo><mi>b</mi></math></body></html>",
        )
        .unwrap();
        std::fs::write(root.join("t/b.xhtml"), "<html><body><h2 id=\"two\">Two</h2><p>Text.</p><h3 id=\"s\">Deeper</h3><p>More.</p><h2 id=\"three\">Three</h2><p>End&nbsp;here.</p></body></html>").unwrap();
        let files = write_unpacked(&root, Path::new("a-book.epub"), &out).unwrap();
        let names: Vec<String> = files.iter().map(|p| p.file_name().unwrap().to_string_lossy().into_owned()).collect();
        assert_eq!(names, ["00-one.md", "01-two.md", "02-three.md", "book.md"]);
        let one = std::fs::read_to_string(out.join("00-one.md")).unwrap();
        assert!(one.contains("# One\n\nSee [the _section_](01-two.md#s) and $`x_{1}`$.\n\n- first\n- second **bold**\n\n![A picture](images/pic.png)\n\n```math\na = b\n```\n"), "{one}");
        let two = std::fs::read_to_string(out.join("01-two.md")).unwrap();
        assert!(two.contains("# Two\n\nText.\n\n## <a id=\"s\"></a>Deeper\n\nMore.\n"), "{two}");
        assert!(!two.contains("Three\n\nEnd"), "{two}");
        let three = std::fs::read_to_string(out.join("02-three.md")).unwrap();
        assert!(three.contains("# Three\n\nEnd"), "{three}");
        assert!(out.join("images/pic.png").is_file());
        let book = std::fs::read_to_string(out.join("book.md")).unwrap();
        assert!(book.starts_with("# A Book\n\n") && book.contains("[the _section_](#s)"), "{book}");
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
