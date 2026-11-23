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
mod brackets;
pub mod course;
pub mod emit;
pub mod epub;
pub mod upright;
pub mod columns;
pub mod glyphs;
pub mod layout;
pub mod links;
mod measure;
pub mod mathtex;
pub mod annots;
pub mod notes;
pub mod ocr;
pub mod pages;
pub mod progress;
pub mod scan;
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
    /// Text the book sets in a drawn frame at the head of a title: a
    /// chapter's number ("I.1"), the first letter of the book's title.
    Framed(String),
    /// At the head of a title: the book draws a line under this title,
    /// from the frame at its head to the right margin.
    Ruled,
    /// A line set in from the ones above it: so many steps (em spaces).
    Indent(usize),
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
    /// The contents are set in columns (number, title, page): they are
    /// written as a table, not as a list.
    pub columns: bool,
    /// What stands in the first column: the number of the chapter
    /// ("I.5", "A"). Empty: the title starts in the first column.
    pub number: String,
    /// The title as the table shows it, where that is not `title` (the
    /// title without its number; a part's name in its two lines).
    pub label: Vec<Inline>,
    /// The name of a part of the book, set centered between its chapters.
    pub part: bool,
}

/// How a cell of a table is set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Align {
    #[default]
    Left,
    Center,
    Right,
}

/// A cell of a table.
#[derive(Debug, Clone, Default)]
pub struct Cell {
    pub text: Vec<Inline>,
    pub align: Align,
    /// 0: ordinary text; else the text is a heading of this level.
    pub heading: usize,
    /// How many columns it takes (1 or more).
    pub span: usize,
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
    /// `key`: the image it was read from, by which the reader's correction
    /// of the text is kept (see [`ocr::collect_corrections`]).
    Code { text: String, key: Option<String> },
    /// A horizontal line across the page (drawn as an image).
    Rule,
    /// A table: its rows of cells (a table of contents set in columns, a
    /// table of the book). `widths`: how wide its columns are, in percent
    /// of the page (0: as wide as needed); empty for a table that is as
    /// wide as it needs to be.
    /// `key`: what the reader's corrections of its cells are kept by (see
    /// [`table_key`]); empty for a table that is made anew each time (the
    /// contents).
    /// `title`: the table's caption, where the book sets it as the top of
    /// the table itself (a panel as wide as the table): written as a cell
    /// across all columns, above the rows.
    /// `note`: a note the book sets on a panel right under the table, as
    /// its foot: written as a cell across all columns, under the rows.
    Table { rows: Vec<Vec<Cell>>, widths: Vec<u8>, key: String, title: Vec<Inline>, note: Vec<Inline> },
    /// A picture without a number or a caption of its own: area `area` of
    /// PDF page `page`, cut from the page (a table as printed, under the
    /// table read from it). `alt`: what it is, for where it cannot be shown.
    Picture { page: u32, area: layout::Figure, alt: String },
    /// What follows is set in this many columns side by side, as on the
    /// page (an index), up to `ColumnsEnd`; `ColumnBreak` starts the next.
    Columns(usize),
    ColumnBreak,
    ColumnsEnd,
    /// A bulleted item of a list begins: what follows, up to `BulletEnd`,
    /// is the item — its paragraph, and the equations and paragraphs set
    /// in under it.
    Bullet,
    BulletEnd,
    /// A quotation set apart begins: what follows, up to `QuoteEnd`, is
    /// written as a block quotation.
    Quote,
    QuoteEnd,
}

/// A block plus the page it starts on (index into the page list).
#[derive(Debug, Clone)]
pub struct Blk {
    pub block: Block,
    pub page: usize,
    /// Every line of it is centered on the page (title pages, dedications).
    pub centered: bool,
    /// A paragraph that goes on with the sentence of the paragraph before
    /// the pictures above it: they were set into the sentence, at a page's
    /// top mostly (see `before_floats`).
    pub goes_on: bool,
}

/// The paragraph that stops in front of the pictures, their captions,
/// tables and page marks that end before block `end`: its place, if
/// nothing else stands between. (A book sets such things at the top of a
/// page, or where there is room, whether or not a sentence is going on
/// there.)
pub(crate) fn before_floats(blocks: &[Blk], end: usize) -> Option<usize> {
    let mut k = end.min(blocks.len());
    while k > 0 {
        k -= 1;
        match &blocks[k].block {
            Block::Anchor(_) | Block::Figure { .. } | Block::Picture { .. } | Block::Table { .. } | Block::PageNote { .. } => {}
            Block::Para(inl) if links::caption(inl).is_some() => {}
            // (a name under its picture)
            Block::Para(_) if blocks[k].centered && blocks[..k].iter().rev().find(|b| !matches!(b.block, Block::Anchor(_))).is_some_and(|b| matches!(b.block, Block::Figure { .. })) => {}
            // (a row of pictures, each over its name)
            Block::ColumnsEnd => {
                let open = blocks[..k].iter().rposition(|b| matches!(b.block, Block::Columns(_)))?;
                let pictures = blocks[open + 1..k].iter().all(|b| match b.block {
                    Block::Figure { .. } | Block::ColumnBreak | Block::Anchor(_) => true,
                    Block::Para(_) => b.centered,
                    _ => false,
                });
                if !pictures {
                    return None;
                }
                k = open;
            }
            Block::Para(_) if !blocks[k].centered => return Some(k),
            _ => return None,
        }
    }
    None
}

#[derive(Debug, Clone, Default)]
pub struct Note {
    pub id: String,
    pub label: String,
    pub blocks: Vec<Block>,
    /// Anchor ids of the places in the text that cite this note.
    pub backrefs: Vec<String>,
    /// A footnote printed at the foot of a page: which page (its place in
    /// the page list). It is written at the foot of that page; a note
    /// from the end of the book, at the end of its chapter.
    pub page: Option<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct Chapter {
    pub title: String,
    /// The title as it is set where that is more than its words (its
    /// lines); empty: `title`.
    pub heading: Vec<Inline>,
    /// The title is set centered.
    pub centered: bool,
    /// What stands above the title in the chapter's file (the book's
    /// title above "Contents").
    pub lead: Vec<Blk>,
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
    /// The running title at the head of every page and the line at its
    /// foot, where the book has them (index = page index).
    pub heads: Vec<Option<String>>,
    pub feet: Vec<Option<String>>,
    /// PDF page number (1 = first sheet) of every page (index = page index).
    pub pdf_pages: Vec<u32>,
    /// The PDF page each printed page starts on, by the id of its page
    /// marker ("p18" -> 24): what the marker links to.
    pub pdf_of: std::collections::HashMap<String, u32>,
    /// The PDF, to cut the pictures from when writing.
    pub source: Option<std::path::PathBuf>,
    /// A copy of it with the pages that are set sideways turned upright
    /// (see [`upright`]): what was read, and what the pictures are cut from.
    pub upright: Option<std::path::PathBuf>,
    /// PDF pages whose places were all moved to the left by so much (see
    /// `layout::Page::shift`): what is cut from such a page is cut that
    /// much further right.
    pub shifts: std::collections::HashMap<u32, f32>,
    /// Pages unfolded from two columns into one (see [`columns`]): where
    /// each stretch of such a page is on the printed page.
    pub unfolds: std::collections::HashMap<u32, Vec<columns::Segment>>,
    /// The book's title (for the top of book.md).
    pub title: String,
    /// Whom the book is by, in a word (its first author's last name), where
    /// the PDF tells: a reader names the book's tabs by it.
    pub by: String,
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

/// What the picture under a table is called: the printed table, cut from
/// the page, to check the table against. Under a table that was read from
/// that picture, and under one whose text the book itself holds.
pub const TABLE_READ: &str = "the table as printed";
/// (Of a picture that may be a table, behind the formula it was read as:
/// [`ocr::read_picture_tables`] makes the two a table and its picture, or
/// takes the picture away again. Never written out.)
pub const TABLE_MAYBE: &str = "a table, if it is one";
pub const TABLE_SET: &str = "the table as the book sets it";

/// The key of a table of the book: from the page it is on and what its
/// cells hold as converted, so that it stays the same from one conversion
/// to the next, and a cell the reader corrects is found again.
pub fn table_key(page: u32, rows: &[Vec<Cell>]) -> String {
    let mut h: u64 = 0xcbf29ce484222325;
    let mut eat = |t: &str| {
        for b in t.bytes() {
            h = (h ^ b as u64).wrapping_mul(0x100000001b3);
        }
    };
    eat(&page.to_string());
    for row in rows {
        eat("\n");
        for cell in row {
            eat("\t");
            eat(&plain(&cell.text));
        }
    }
    format!("{h:016x}")
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
            Inline::Text(t) | Inline::Emph(t) | Inline::Strong(t) | Inline::Math(t) | Inline::Sup(t) | Inline::Framed(t) => s.push_str(t),
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
    convert_pdf_as(path, pages, false)
}

/// [`convert_pdf_pages`]; `pictures`: read the pages as pictures whatever
/// text the PDF holds (a scan with a layer of text that is no use: what a
/// program made of handwriting).
pub fn convert_pdf_as(path: &Path, pages: Option<(u32, u32)>, pictures: bool) -> Result<Book, String> {
    let mut doc = layout::read_text(path, pages)?;
    // Pages set sideways: read once more from a copy with them upright.
    let original = path;
    let turned = if doc.sideways.is_empty() || pictures { None } else { upright::upright_copy(path, &doc.sideways) };
    if let Some(copy) = &turned {
        doc = layout::read_text(copy, pages)?;
    }
    let path = turned.as_deref().unwrap_or(path);
    let mut book = if pictures || scan::looks_scanned(&doc, path, pages) {
        // pictures of pages: read by the page reader
        let (first, last) = pages.unwrap_or((1, doc.pages.len() as u32));
        let read = scan::read(path, first, last.min(first + doc.pages.len() as u32 - 1))?;
        progress::report(progress::Step::Book, 0, 1);
        finish(read.blks, read.labels, read.pdf_pages, read.chapter_size)
    } else {
        layout::read_drawings(&mut doc, path, pages);
        progress::report(progress::Step::Book, 0, 1);
        let unfolds = columns::unfold(&mut doc, path);
        let mut book = convert_doc(&doc);
        book.unfolds = unfolds;
        book.shifts = doc.pages.iter().filter(|p| p.shift != 0.0).map(|p| (p.number, p.shift)).collect();
        ocr::read_picture_tables(&mut book, path);
        book
    };
    book.title = book_title(original, &doc);
    book.by = book_by(original, &doc, &book.title);
    structure::half_titles_lead(&mut book.chapters, &book.title);
    book.source = Some(original.to_path_buf());
    book.upright = turned;
    progress::report(progress::Step::Book, 1, 1);
    Ok(book)
}

/// Whom the book is by, in a word. A course's notes are by their school:
/// the title names the course by it ("Quantum Physics (UCSD Physics 130)":
/// capitals, a subject, a number). Else the first author's last name: from
/// the PDF's author metadata, else from the line of a name on the first
/// page — a name with initials on a line of its own ("F. J. Dyson"), or a
/// few capitalized words in type larger than the text's, standing well
/// apart under the title and its subtitle ("Nick Lucid"). Empty where
/// nothing tells.
fn book_by(path: &Path, doc: &layout::Doc, title: &str) -> String {
    let parts: Vec<&str> = title.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).collect();
    let roman = |w: &str| w.chars().all(|c| "IVXLC".contains(c));
    for w in parts.windows(3) {
        if w[0].chars().count() >= 3 && w[0].chars().all(char::is_uppercase) && !roman(w[0]) && w[1].chars().next().is_some_and(char::is_uppercase) && w[1].chars().any(char::is_lowercase) && w[2].chars().all(|c| c.is_ascii_digit()) {
            return w[0].to_string();
        }
    }
    let meta = std::process::Command::new("pdfinfo")
        .arg(path)
        .output()
        .ok()
        .and_then(|o| String::from_utf8_lossy(&o.stdout).lines().find_map(|l| l.strip_prefix("Author:").map(|t| t.trim().to_string())))
        .unwrap_or_default();
    let name = |w: &str| w.chars().next().is_some_and(char::is_uppercase) && w.chars().all(|c| c.is_alphabetic() || c == '.' || c == '-' || c == '’' || c == '\'');
    if let Some(first) = meta.split([',', ';', '&']).next().and_then(|a| a.split(" and ").next()) {
        let words: Vec<&str> = first.split_whitespace().collect();
        if (1..=4).contains(&words.len()) && words.iter().all(|w| name(w)) && words.last().is_some_and(|w| w.len() > 2) {
            return words.last().unwrap_or(&"").to_string();
        }
    }
    let Some(page) = doc.pages.first() else { return String::new() };
    let lines: Vec<&layout::Line> = page.lines.iter().filter(|l| !layout::plain_text(l).trim().is_empty()).collect();
    let largest = lines.iter().map(|l| l.size).fold(0.0, f32::max);
    for (k, l) in lines.iter().enumerate() {
        let text = layout::plain_text(l);
        let words: Vec<&str> = text.split_whitespace().collect();
        let named = (2..=4).contains(&words.len()) && words.iter().all(|w| name(w)) && words.last().is_some_and(|w| w.len() > 2 && !w.ends_with('.'));
        let initial = |w: &&str| w.chars().count() == 2 && w.ends_with('.');
        let apart = k > 0 && l.size > doc.body_size + 0.5 && l.top - lines[k - 1].top > 2.0 * l.size;
        if named && l.size + 0.5 < largest && (apart || words.iter().any(initial)) {
            return words.last().unwrap_or(&"").to_string();
        }
    }
    String::new()
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
    // an article in two columns: the lines in larger type that open its
    // first page
    if doc.gutter.is_some() {
        let lines: Vec<String> = doc.pages.first().map(|p| p.lines.iter().take_while(|l| l.size > doc.body_size + 0.5).map(|l| layout::plain_text(l).trim().to_string()).collect()).unwrap_or_default();
        let t = lines.join(" ");
        if words(&t) >= 2 {
            return t;
        }
    }
    // the biggest type on the first pages, all its lines together
    // ("ADVANCED" / "QUANTUM MECHANICS"), in normal capitals
    let has_words = |l: &layout::Line| words(&layout::plain_text(l)) >= 1;
    // (the first page that has such type: the heading "Contents" two pages
    // on may be set larger than the title; of lines equally large the
    // first, the second being the title's second line or what is under it)
    let big = doc.pages.iter().take(4).find_map(|p| {
        let top = p.lines.iter().filter(|l| has_words(l)).fold(None::<&layout::Line>, |m, l| if m.is_some_and(|m| m.size >= l.size) { m } else { Some(l) })?;
        (top.size > doc.body_size * 1.3).then_some((p, top))
    });
    let title = big.map(|(page, top)| {
        // the lines just above and below it in large type belong to it
        let lines: Vec<&layout::Line> = page.lines.iter().filter(|l| has_words(l)).collect();
        let k = lines.iter().position(|l| std::ptr::eq(*l, top)).unwrap_or(0);
        // (a line as large as the largest is of the title from further off)
        let part_of = |a: &layout::Line, b: &layout::Line| b.size >= 0.75 * top.size && (b.top - a.top).abs() <= if b.size >= 0.95 * top.size { 3.5 } else { 2.5 } * top.size;
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
    let labels = pages.iter().map(|p| p.label.clone()).collect();
    let pdf_pages = pages.iter().map(|p| doc.pages[p.index].number).collect();
    let mut book = finish(blks, labels, pdf_pages, stats.chapter_size);
    book.heads = pages.iter().map(|p| p.head.clone()).collect();
    book.feet = pages.iter().map(|p| p.foot.clone()).collect();
    // (a book without page numbers, whose printed pages were found by the
    // links of its index: each starts on the page its mark is on)
    if pages.iter().all(|p| p.label.is_none()) {
        for p in &pages {
            let page = &doc.pages[p.index];
            for (_, label) in &page.marks {
                book.pdf_of.entry(format!("p{label}")).or_insert(page.number);
            }
        }
    }
    book
}

/// The book of its blocks: chapters, notes, links. `labels`: the printed
/// number of each page; `pdf_pages`: its number in the PDF.
fn finish(blks: Vec<Blk>, labels: Vec<Option<String>>, pdf_pages: Vec<u32>, chapter_size: Option<f32>) -> Book {
    let mut chapters = structure::split_chapters(blks, chapter_size);
    structure::merge_front_matter(&mut chapters, chapter_size);
    let mut book = Book { labels, pdf_pages, ..Default::default() };
    for (label, n) in book.labels.iter().zip(&book.pdf_pages) {
        if let Some(l) = label {
            book.pdf_of.entry(format!("p{l}")).or_insert(*n);
        }
    }
    // (an article keeps its list of references, a file of its own)
    let article = chapter_size == Some(blocks::ARTICLE_SECTION);
    let removed = notes::attach(&mut chapters, &mut book, article);
    // (after the notes: their names count the chapters, and stay as they
    // were before the contents were a file of their own)
    structure::split_contents(&mut chapters);
    structure::assign_slugs(&mut chapters);
    links::link_toc(&mut chapters, &book.labels, &book.pdf_pages, &removed, &mut book.warnings);
    structure::heading_levels(&mut chapters, chapter_size);
    links::link_index(&mut chapters, &book.pdf_pages);
    links::link_problems(&mut chapters);
    links::link_cross_refs(&mut chapters);
    links::link_equations(&mut chapters);
    if article {
        links::link_article(&mut chapters);
    }
    book.chapters = chapters;
    book
}
