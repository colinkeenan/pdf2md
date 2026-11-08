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
pub mod mathtex;
pub mod notes;
pub mod pages;
pub mod structure;

use std::path::Path;

pub use emit::write_book;

/// A piece of running text.
#[derive(Debug, Clone, PartialEq)]
pub enum Inline {
    /// Plain prose (escaped for Markdown when written out).
    Text(String),
    /// Italic prose, written as `_..._`.
    Emph(String),
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
    /// A link; `text` is already Markdown.
    Link { text: String, href: String },
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
    Equation { latex: String, tag: Option<String> },
    Anchor(String),
    Toc(Vec<TocEntry>),
    /// A bulleted line: index entries and the linked TOC.
    Item { level: usize, text: Vec<Inline> },
    /// A footnote printed at the bottom of a page.
    PageNote { label: String, text: Vec<Inline> },
}

/// A block plus the page it starts on (index into the page list).
#[derive(Debug, Clone)]
pub struct Blk {
    pub block: Block,
    pub page: usize,
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
}

/// The printed page label of a page-start anchor id ("p18" -> "18"), None
/// for other anchors.
pub fn page_label(id: &str) -> Option<&str> {
    let l = id.strip_prefix('p')?;
    (!l.is_empty() && l.chars().all(|c| c.is_ascii_alphanumeric())).then_some(l)
}

/// Plain text of some inlines (math as its LaTeX), for matching titles.
pub fn plain(inl: &[Inline]) -> String {
    let mut s = String::new();
    for i in inl {
        match i {
            Inline::Text(t) | Inline::Emph(t) | Inline::Math(t) | Inline::Sup(t) => s.push_str(t),
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
            Inline::Emph(t) => Inline::Text(t),
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
    Ok(convert_doc(&doc))
}

/// Convert an already-read document.
pub fn convert_doc(doc: &layout::Doc) -> Book {
    let pages = pages::prepare(doc);
    let stats = blocks::stats(doc, &pages);
    let blks = blocks::build(doc, &pages, &stats);
    let mut chapters = structure::split_chapters(blks, stats.chapter_size);
    let mut book = Book {
        labels: pages.iter().map(|p| p.label.clone()).collect(),
        ..Default::default()
    };
    let removed = notes::attach(&mut chapters, &mut book);
    structure::assign_slugs(&mut chapters);
    links::link_toc(&mut chapters, &book.labels, &removed, &mut book.warnings);
    links::link_index(&mut chapters);
    book.chapters = chapters;
    book
}
