//! pdf2md-core: turn a PDF textbook into a folder of Markdown files, one per
//! chapter.
//!
//! Pipeline (see [`convert_doc`]):
//!
//! 1. `layout`: run poppler's `pdftohtml -xml` to get every text span with
//!    its position and font.
//! 2. `pages`: drop running headers and page-number footers, learn the
//!    printed page numbers, split two-column pages.
//! 3. `blocks`: lines become headings, paragraphs (with inline `$math$`),
//!    displayed equations, TOC entries, index lines and footnotes.
//!    `mathtex` turns positioned spans into LaTeX.
//! 4. `structure`: split into chapters at chapter-title-sized headings.
//! 5. `notes`: link footnote markers and `[n]` citations to their notes.
//! 6. `links`: link the table of contents and the index.
//! 7. `emit`: write Markdown.

pub mod blocks;
pub mod emit;
pub mod layout;
pub mod links;
mod measure;
pub mod mathtex;
pub mod annots;
pub mod notes;
pub mod ocr;
pub mod pages;
pub mod structure;
mod textmath;
pub mod userdata;

use std::path::Path;

pub use emit::write_book;

/// A piece of running text.
#[derive(Debug, Clone, PartialEq)]
pub enum Inline {
    /// Plain prose (escaped for Markdown when written out).
    Text(String),
    /// Italic prose, written as `_..._`.
    Emph(String),
    /// Bold prose (a journal volume "**80**"), written as `**...**`.
    Strong(String),
    /// Inline math as LaTeX, written as `$...$`.
    Math(String),
    /// A superscript in the text font: a footnote marker candidate ("1"),
    /// or an ordinal ending ("rd" in 3rd).
    Sup(String),
    /// A footnote marker linked to its note.
    NoteRef { label: String, id: String, href: String },
    /// A citation like `[12]` linked to its reference.
    Cite { label: String, id: String, href: String },
    /// An anchor: the start of printed page 6 (`p6`, written with a
    /// visible page marker) or a link target (invisible).
    Anchor(String),
    /// A link target wrapped around the words it points at (an index
    /// term), so the viewer can show where the link landed.
    Target { id: String, text: String, emph: bool },
    /// A line break kept from the page (between centered lines).
    Break,
    /// A link; `text` is already Markdown.
    Link { text: String, href: String },
    /// A small picture inside the text (an equation image that could not be
    /// read), cut from this PDF page at this place.
    Picture { page: u32, area: layout::Figure },
    /// Invisible: the formula right after it was read from this picture
    /// (written as a comment, so a hand correction of it is kept).
    Source(String),
    /// Invisible: the page number just before it links to this PDF page
    /// (a linked index). Used by `links::link_index`, never written out.
    PageLink(u32),
}

/// One entry of the printed table of contents.
#[derive(Debug, Clone)]
pub struct TocEntry {
    pub title: Vec<Inline>,
    /// Printed page number as written in the TOC.
    pub page: String,
    pub left: f32,
    pub size: f32,
}

#[derive(Debug, Clone)]
pub enum Block {
    /// `level` 0 = not decided yet (see `structure` and `links`).
    Heading { level: usize, size: f32, id: String, text: Vec<Inline> },
    Para(Vec<Inline>),
    /// `source`: the equation image it was read from, when it is exactly
    /// one (so a hand correction of it can be kept; see `ocr`).
    Equation { latex: String, tag: Option<String>, source: Option<String> },
    Anchor(String),
    Toc(Vec<TocEntry>),
    /// A bulleted line: index entries and the linked TOC.
    Item { level: usize, text: Vec<Inline> },
    /// A footnote printed at the bottom of a page.
    PageNote { label: String, text: Vec<Inline> },
    /// A picture: area `area` of PDF page `page`, the `n`th on that page,
    /// written as an image file cut from the page.
    /// `label`: the printed page number, for the caption.
    Figure { page: u32, n: usize, area: layout::Figure, label: Option<String> },
    /// Program code (read from an image of it), written as a code block.
    Code(String),
    /// A horizontal line across the page (drawn as an image).
    Rule,
}

/// A block plus the page it starts on (index into the page list).
#[derive(Debug, Clone)]
pub struct Blk {
    pub block: Block,
    pub page: usize,
    /// Every line of it is centered on the page (title pages, dedications).
    pub centered: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Note {
    pub id: String,
    pub label: String,
    pub blocks: Vec<Block>,
    /// Anchor ids of the places in the text that cite this note.
    pub backrefs: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Chapter {
    pub title: String,
    pub slug: String,
    pub blocks: Vec<Blk>,
    pub notes: Vec<Note>,
    pub refs: Vec<Note>,
}

#[derive(Debug, Default)]
pub struct Book {
    pub chapters: Vec<Chapter>,
    pub unplaced: Vec<String>,
    pub warnings: Vec<String>,
    /// Printed page number of every page (index = page index).
    pub labels: Vec<Option<String>>,
    /// PDF page number (1 = first sheet) of every page (index = page index).
    pub pdf_pages: Vec<u32>,
    /// The PDF, to cut the pictures from when writing.
    pub source: Option<std::path::PathBuf>,
    /// The book's title (for the top of book.md).
    pub title: String,
    /// The reader's notes and prose fixes from an earlier conversion, put
    /// back when the book is written (see `userdata`).
    pub user: userdata::UserData,
}

/// The equation numbers of a displayed equation: its tag, and the \tag{…}
/// of each numbered row inside it (a system of equations).
pub fn equation_numbers(latex: &str, tag: Option<&str>) -> Vec<String> {
    let mut v: Vec<String> = tag.map(str::to_string).into_iter().collect();
    for part in latex.split("\\tag{").skip(1) {
        if let Some(t) = part.split('}').next() {
            v.push(t.to_string());
        }
    }
    v
}

/// The anchor id of an equation number: "3.422" -> "eq-3.422".
pub fn equation_id(number: &str) -> String {
    let t: String = number.chars().map(|c| if c.is_alphanumeric() || c == '.' { c } else { '-' }).collect();
    format!("eq-{t}")
}

/// The printed page label of a page-start anchor id ("p18" -> "18"), None
/// for other anchors.
pub fn page_label(id: &str) -> Option<&str> {
    let l = id.strip_prefix('p')?;
    let l = l.strip_suffix("-approx").unwrap_or(l);
    (!l.is_empty() && l.chars().all(|c| c.is_ascii_alphanumeric())).then_some(l)
}

/// A page-start anchor whose place is estimated ("p215-approx"): the page
/// is not cited in the index, so it was spaced between known pages.
pub fn page_is_approx(id: &str) -> bool {
    id.ends_with("-approx") && page_label(id).is_some()
}

/// Plain text of some inlines (math as its LaTeX), for matching titles.
pub fn plain(inl: &[Inline]) -> String {
    let mut s = String::new();
    for i in inl {
        match i {
            Inline::Text(t) | Inline::Emph(t) | Inline::Strong(t) | Inline::Math(t) | Inline::Sup(t) => s.push_str(t),
            Inline::Link { text, .. } | Inline::Target { text, .. } => s.push_str(text),
            _ => {}
        }
    }
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Italic pieces as ordinary text (headings are styled by `#` already).
pub fn no_emph(inl: Vec<Inline>) -> Vec<Inline> {
    let mut out: Vec<Inline> = Vec::new();
    for i in inl {
        let i = match i {
            Inline::Emph(t) | Inline::Strong(t) => Inline::Text(t),
            other => other,
        };
        match (out.last_mut(), &i) {
            (Some(Inline::Text(a)), Inline::Text(b)) => a.push_str(b),
            _ => out.push(i),
        }
    }
    out
}

/// Letters and digits only, lowercase: "Conservation Laws – Existence of
/// Spin" and "Conservation Laws. Existence of Spin" get the same key.
pub fn key(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect()
}

/// Read a PDF and convert it.
pub fn convert_pdf(path: &Path) -> Result<Book, String> {
    convert_pdf_pages(path, None)
}

/// Read only pages `first..=last` (1-based) and convert them; handy for
/// checking a few pages of a big book.
pub fn convert_pdf_pages(path: &Path, pages: Option<(u32, u32)>) -> Result<Book, String> {
    let doc = layout::read_pdf(path, pages)?;
    let mut book = convert_doc(&doc);
    book.title = book_title(path, &doc);
    book.source = Some(path.to_path_buf());
    Ok(book)
}

/// The book's title: the PDF's title metadata if it reads like one (not an
/// identifier like "arXiv:…" or a file name), else the biggest text on the
/// first pages (the title page), else the file name.
fn book_title(path: &Path, doc: &layout::Doc) -> String {
    let words = |t: &str| t.split_whitespace().filter(|w| w.chars().filter(|c| c.is_alphabetic()).count() >= 2).count();
    let meta = std::process::Command::new("pdfinfo")
        .arg(path)
        .output()
        .ok()
        .and_then(|o| String::from_utf8_lossy(&o.stdout).lines().find_map(|l| l.strip_prefix("Title:").map(|t| t.trim().to_string())))
        .filter(|t| {
            let l = t.to_lowercase();
            words(t) >= 2 && !l.contains("arxiv") && !l.contains(".pdf") && !l.contains(".doc") && !l.contains(".tex") && !l.starts_with("microsoft")
        });
    if let Some(t) = meta {
        return t;
    }
    // the biggest type on the first pages, all its lines together
    // ("ADVANCED" / "QUANTUM MECHANICS"), in normal capitals
    let has_words = |l: &layout::Line| words(&layout::plain_text(l)) >= 1;
    let big = doc.pages.iter().take(4).flat_map(|p| p.lines.iter().filter(|l| has_words(l)).map(move |l| (p, l))).max_by(|a, b| a.1.size.total_cmp(&b.1.size));
    let title = big.filter(|(_, l)| l.size > doc.body_size * 1.3).map(|(page, top)| {
        // the lines just above and below it in large type belong to it
        let lines: Vec<&layout::Line> = page.lines.iter().filter(|l| has_words(l)).collect();
        let k = lines.iter().position(|l| std::ptr::eq(*l, top)).unwrap_or(0);
        let part_of = |a: &layout::Line, b: &layout::Line| b.size >= 0.75 * top.size && (b.top - a.top).abs() <= 2.5 * top.size;
        let (mut first, mut last) = (k, k);
        while first > 0 && part_of(lines[first], lines[first - 1]) {
            first -= 1;
        }
        while last + 1 < lines.len() && part_of(lines[last], lines[last + 1]) {
            last += 1;
        }
        let text: Vec<String> = lines[first..=last].iter().map(|l| layout::plain_text(l).trim().to_string()).collect();
        let t = text.join(" ");
        if t.chars().any(char::is_lowercase) {
            t
        } else {
            t.split_whitespace()
                .map(|w| {
                    let mut c = w.chars();
                    c.next().map(|f| f.to_string() + &c.as_str().to_lowercase()).unwrap_or_default()
                })
                .collect::<Vec<_>>()
                .join(" ")
        }
    });
    title.filter(|t| !t.is_empty()).unwrap_or_else(|| path.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default())
}

/// Convert an already-read document.
pub fn convert_doc(doc: &layout::Doc) -> Book {
    let pages = pages::prepare(doc);
    let stats = blocks::stats(doc, &pages);
    let blks = blocks::build(doc, &pages, &stats);
    let mut chapters = structure::split_chapters(blks, stats.chapter_size);
    structure::merge_front_matter(&mut chapters, stats.chapter_size);
    let mut book = Book {
        labels: pages.iter().map(|p| p.label.clone()).collect(),
        pdf_pages: pages.iter().map(|p| doc.pages[p.index].number).collect(),
        ..Default::default()
    };
    let removed = notes::attach(&mut chapters, &mut book);
    structure::assign_slugs(&mut chapters);
    links::link_toc(&mut chapters, &book.labels, &book.pdf_pages, &removed, &mut book.warnings);
    structure::heading_levels(&mut chapters, stats.chapter_size);
    links::link_index(&mut chapters, &book.pdf_pages);
    links::link_problems(&mut chapters);
    links::link_cross_refs(&mut chapters);
    links::link_equations(&mut chapters);
    book.chapters = chapters;
    book
}
