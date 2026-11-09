//! Write the chapters as Markdown files.

use crate::{page_label, Block, Book, Inline, Note};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Escape characters that Markdown would otherwise treat as formatting.
/// Plain text escaped for Markdown (for link texts built elsewhere).
pub fn escape_text(t: &str) -> String {
    escape(t)
}

fn escape(t: &str) -> String {
    let mut s = String::with_capacity(t.len());
    for c in t.chars() {
        match c {
            '\\' | '*' | '_' | '`' | '$' => {
                s.push('\\');
                s.push(c);
            }
            '<' => s.push_str("&lt;"),
            '>' => s.push_str("&gt;"),
            c => s.push(c),
        }
    }
    s
}

pub fn render_inlines(inl: &[Inline]) -> String {
    // an accent and its letter that came in separate pieces ("f¨" + "ur")
    let joined: Vec<Inline> = inl
        .iter()
        .map(|i| match i {
            Inline::Text(t) => Inline::Text(crate::layout::compose_accents(t)),
            Inline::Emph(t) => Inline::Emph(crate::layout::compose_accents(t)),
            Inline::Strong(t) => Inline::Strong(crate::layout::compose_accents(t)),
            other => other.clone(),
        })
        .collect();
    let inl = &joined[..];
    let mut s = String::new();
    for i in inl {
        match i {
            Inline::Text(t) => s.push_str(&escape(t)),
            Inline::Emph(t) => {
                // keep surrounding spaces outside the markers: "_word_ "
                let core = t.trim();
                if t.starts_with(' ') {
                    s.push(' ');
                }
                if !core.is_empty() {
                    s.push_str(&format!("_{}_", escape(core)));
                }
                if t.ends_with(' ') && !core.is_empty() {
                    s.push(' ');
                }
            }
            Inline::Strong(t) => {
                // like italic: spaces outside the markers
                let core = t.trim();
                if t.starts_with(' ') {
                    s.push(' ');
                }
                if !core.is_empty() {
                    s.push_str(&format!("**{}**", escape(core)));
                }
                if t.ends_with(' ') && !core.is_empty() {
                    s.push(' ');
                }
            }
            Inline::Math(m) => {
                s.push('$');
                s.push_str(m.trim());
                s.push('$');
            }
            Inline::Sup(t) => s.push_str(&format!("<sup>{}</sup>", escape(t))),
            Inline::NoteRef { label, id, href } => {
                s.push_str(&format!("<sup><a id=\"{id}\" href=\"{href}\">{label}</a></sup>"))
            }
            Inline::Cite { label, id, href } => {
                s.push_str(&format!("<a id=\"{id}\"></a>[[{label}]]({href})"))
            }
            Inline::Anchor(id) => match page_label(id) {
                // a small marker where the printed page starts, so pages
                // can be found ("p. 18") and index links visibly land there
                Some(l) => {
                    if !s.is_empty() && !s.ends_with(' ') {
                        s.push(' ');
                    }
                    let approx = if crate::page_is_approx(id) { "≈" } else { "" };
                    s.push_str(&format!("<a id=\"{id}\"><sup>[{approx}p. {l}]</sup></a>"));
                }
                None => s.push_str(&format!("<a id=\"{id}\"></a>")),
            },
            Inline::Target { id, text, emph } => {
                let t = escape(text);
                let t = if *emph { format!("<em>{t}</em>") } else { t };
                s.push_str(&format!("<a id=\"{id}\">{t}</a>"));
            }
            Inline::PageLink(_) => {}
            Inline::Picture { page, area } => s.push_str(&format!("![equation]({})", picture_file(*page, area))),
            Inline::Break => {
                let trimmed = s.trim_end().len();
                s.truncate(trimmed);
                s.push_str("<br>\n");
            }
            Inline::Link { text, href } => s.push_str(&format!("[{text}]({href})")),
        }
    }
    s
}

fn render_block(b: &Block, out: &mut String, in_list: &mut bool) {
    let is_item = matches!(b, Block::Item { .. });
    if *in_list && !is_item {
        out.push('\n');
    }
    *in_list = is_item;
    match b {
        Block::Heading { level, id, text, .. } => {
            let hashes = "#".repeat((*level).clamp(1, 6));
            // (a heading is styled by its #s: no italic or bold inside)
            let plain = crate::no_emph(text.clone());
            out.push_str(&format!("{hashes} <a id=\"{id}\"></a>{}\n\n", render_inlines(&plain)));
        }
        Block::Para(inl) => {
            let t = render_inlines(inl);
            if !t.trim().is_empty() {
                out.push_str(t.trim());
                out.push_str("\n\n");
            }
        }
        Block::Equation { latex, tag } => {
            // an invisible anchor per equation number, for references to it
            for t in crate::equation_numbers(latex, tag.as_deref()) {
                out.push_str(&format!("<a id=\"{}\"></a>\n\n", crate::equation_id(&t)));
            }
            if latex.is_empty() {
                // only an equation number survived
                if let Some(t) = tag {
                    out.push_str(&format!("$$\n\\tag{{{t}}}\n$$\n\n"));
                }
                return;
            }
            let tag = tag.as_ref().map(|t| format!(" \\tag{{{t}}}")).unwrap_or_default();
            out.push_str(&format!("$$\n{latex}{tag}\n$$\n\n"));
        }
        Block::Anchor(id) => match page_label(id) {
            Some(l) => {
                let approx = if crate::page_is_approx(id) { "≈" } else { "" };
                out.push_str(&format!("<a id=\"{id}\"><sup>[{approx}p. {l}]</sup></a>\n\n"))
            }
            None => out.push_str(&format!("<a id=\"{id}\"></a>\n\n")),
        },
        Block::Item { level, text } => {
            out.push_str(&format!("{}- {}\n", "  ".repeat(*level), render_inlines(text)));
        }
        Block::Toc(entries) => {
            for e in entries {
                out.push_str(&format!("- {}\n", render_inlines(&e.title)));
            }
            out.push('\n');
        }
        // numbered by chapter_markdown, which writes the caption
        Block::Figure { page, n, .. } => {
            out.push_str(&format!("![Figure]({})\n\n", figure_file(*page, *n)));
        }
        Block::Code(code) => {
            out.push_str(&format!("```\n{code}\n```\n\n"));
        }
        Block::PageNote { label, text } => {
            out.push_str(&format!("{label}. {}\n\n", render_inlines(text)));
        }
    }
}

fn backlinks(n: &Note) -> String {
    match n.backrefs.len() {
        0 => String::new(),
        1 => format!(" [↩](#{})", n.backrefs[0]),
        _ => n
            .backrefs
            .iter()
            .enumerate()
            .map(|(i, b)| format!(" [↩{}](#{})", i + 1, b))
            .collect(),
    }
}

fn render_note(n: &Note, label: &str, out: &mut String) {
    let mut first = true;
    let mut in_list = false;
    for b in &n.blocks {
        if first {
            first = false;
            if let Block::Para(inl) = b {
                out.push_str(&format!(
                    "<a id=\"{}\"></a>**{label}** {}{}\n\n",
                    n.id,
                    render_inlines(inl).trim(),
                    backlinks(n)
                ));
                continue;
            }
            out.push_str(&format!("<a id=\"{}\"></a>**{label}**{}\n\n", n.id, backlinks(n)));
        }
        render_block(b, out, &mut in_list);
    }
}

pub fn chapter_markdown(book: &Book, i: usize) -> String {
    let mut s = chapter_body(book, i);
    let mut nav = Vec::new();
    if i > 0 {
        let p = &book.chapters[i - 1];
        nav.push(format!("[← {}]({}.md)", p.title, p.slug));
    }
    if i + 1 < book.chapters.len() {
        let n = &book.chapters[i + 1];
        nav.push(format!("[{} →]({}.md)", n.title, n.slug));
    }
    if !nav.is_empty() {
        s.push_str("---\n\n");
        s.push_str(&nav.join(" · "));
        s.push('\n');
    }
    s
}

/// The whole book in one file (`book.md`), so a browser's Ctrl+F searches
/// all of it. Links to chapter files become links inside the page (each
/// chapter starts with an anchor named after its file); an id used in
/// more than one chapter (each chapter's "ref-4") gets the chapter's file
/// name in front, in that chapter's links too.
pub fn book_markdown(book: &Book) -> String {
    let bodies: Vec<String> = (0..book.chapters.len()).map(|i| chapter_body(book, i)).collect();
    let mut seen: HashMap<String, usize> = HashMap::new();
    for body in &bodies {
        let ids: HashSet<&str> = scan(body, "id=\"", '"').into_iter().map(|(a, b)| &body[a..b]).collect();
        for id in ids {
            *seen.entry(id.to_string()).or_default() += 1;
        }
    }
    let slugs: HashMap<&str, usize> = book.chapters.iter().enumerate().map(|(i, c)| (c.slug.as_str(), i)).collect();
    let rename = |ci: usize, id: &str| {
        if seen.get(id).copied().unwrap_or(0) > 1 {
            format!("{}--{id}", book.chapters[ci].slug)
        } else {
            id.to_string()
        }
    };
    // the book's title on top (a viewer names its tab after it)
    let mut out = if book.title.is_empty() { String::new() } else { format!("# {}\n\n", book.title) };
    for (ci, body) in bodies.iter().enumerate() {
        if ci > 0 {
            out.push_str("---\n\n");
        }
        out.push_str(&format!("<a id=\"{}\"></a>\n\n", book.chapters[ci].slug));
        // (start, end) of every id and link target, in order
        let mut spots: Vec<(usize, usize, bool)> = Vec::new();
        spots.extend(scan(body, "id=\"", '"').into_iter().map(|(a, b)| (a, b, true)));
        spots.extend(scan(body, "href=\"", '"').into_iter().map(|(a, b)| (a, b, false)));
        spots.extend(scan(body, "](", ')').into_iter().map(|(a, b)| (a, b, false)));
        spots.sort();
        let mut last = 0;
        for (a, b, is_id) in spots {
            let t = &body[a..b];
            let new = if is_id {
                rename(ci, t)
            } else if let Some(id) = t.strip_prefix('#') {
                format!("#{}", rename(ci, id))
            } else {
                let (file, frag) = t.split_once('#').map_or((t, None), |(f, g)| (f, Some(g)));
                match file.strip_suffix(".md").and_then(|s| slugs.get(s)) {
                    Some(&target) => match frag {
                        Some(id) => format!("#{}", rename(target, id)),
                        None => format!("#{}", book.chapters[target].slug),
                    },
                    None => continue,
                }
            };
            out.push_str(&body[last..a]);
            out.push_str(&new);
            last = b;
        }
        out.push_str(&body[last..]);
    }
    out
}

/// (start, end) of the text between each `open` and the next `close`.
fn scan(s: &str, open: &str, close: char) -> Vec<(usize, usize)> {
    let mut v = Vec::new();
    let mut from = 0;
    while let Some(k) = s[from..].find(open) {
        let a = from + k + open.len();
        let Some(len) = s[a..].find(close) else { break };
        // a link target or id has no spaces or line breaks
        if !s[a..a + len].contains(char::is_whitespace) {
            v.push((a, a + len));
        }
        from = a;
    }
    v
}

/// A chapter's text without the links to the chapters before and after.
fn chapter_body(book: &Book, i: usize) -> String {
    let ch = &book.chapters[i];
    let mut s = format!("# {}\n\n", ch.title);
    let mut in_list = false;
    // a run of centered blocks goes in one centered div (page markers
    // between them don't break it)
    let mut centered = false;
    // pictures are numbered through the whole book
    let is_fig = |b: &crate::Blk| matches!(b.block, crate::Block::Figure { .. });
    let mut fig_no = book.chapters[..i].iter().flat_map(|c| &c.blocks).filter(|b| is_fig(b)).count();
    for (bi, b) in ch.blocks.iter().enumerate() {
        let neutral = matches!(b.block, crate::Block::Anchor(_));
        if !neutral && b.centered != centered {
            if in_list {
                s.push('\n');
                in_list = false;
            }
            s.push_str(if b.centered { "<div align=\"center\">\n\n" } else { "</div>\n\n" });
            centered = b.centered;
        }
        if let crate::Block::Figure { page, n, label, .. } = &b.block {
            fig_no += 1;
            if in_list {
                s.push('\n');
                in_list = false;
            }
            // the book's own caption right after it ("Fig. 1.1 …"): that is
            // the caption; otherwise a visible one of ours, so a search for
            // "Figure" finds each picture
            let next = ch.blocks[bi + 1..].iter().find(|b| !matches!(b.block, crate::Block::Anchor(_)));
            if let Some(own) = next.and_then(|b| match &b.block {
                crate::Block::Para(inl) => crate::links::figure_caption_number(&crate::plain(inl)),
                _ => None,
            }) {
                s.push_str(&format!("![Fig. {own}]({})\n\n", figure_file(*page, *n)));
                continue;
            }
            let caption = match label {
                Some(l) => format!("Figure {fig_no} (p. {l})"),
                None => format!("Figure {fig_no}"),
            };
            s.push_str(&format!("![{caption}]({})\n\n_{caption}_\n\n", figure_file(*page, *n)));
            continue;
        }
        render_block(&b.block, &mut s, &mut in_list);
    }
    if in_list {
        s.push('\n');
    }
    if centered {
        s.push_str("</div>\n\n");
    }
    if !ch.notes.is_empty() {
        s.push_str("---\n\n## Notes\n\n");
        for n in &ch.notes {
            render_note(n, &format!("{}.", n.label), &mut s);
        }
    }
    if !ch.refs.is_empty() {
        s.push_str("---\n\n## References\n\n");
        for n in &ch.refs {
            render_note(n, &n.label, &mut s);
        }
    }
    s
}

/// Write one `<slug>.md` per chapter into `dir` (plus `unplaced-notes.md`
/// if some notes could not be matched).
/// Where a picture's image file goes, relative to the chapter files.
fn figure_file(page: u32, n: usize) -> String {
    format!("images/fig-p{page:03}-{n}.png")
}

/// Cut each picture out of its PDF page with pdftocairo (150 dpi PNG).
/// Without pdftocairo or the PDF the Markdown still links them.
/// Where a small picture inside the text goes: named by page and place.
fn picture_file(page: u32, area: &crate::layout::Figure) -> String {
    format!("images/eq-p{page:03}-{:.0}-{:.0}.png", area.left, area.top)
}

/// Cut each picture out of its PDF page with pdftocairo (150 dpi PNG):
/// figures, and the unread equation images inside the text.
/// Without pdftocairo or the PDF the Markdown still links them.
fn write_figures(book: &Book, dir: &Path) {
    let Some(pdf) = &book.source else { return };
    let cut = |page: u32, area: &crate::layout::Figure, name: &str, pad: f32| {
        let file = dir.join(name);
        if let Some(parent) = file.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        // pdftohtml pixels are 1.5 × points; 150 dpi is 150/72 px per point
        let dpi = 150.0f32;
        let px = |v: f32| (v / 1.5 * dpi / 72.0).round().max(0.0) as i64;
        let (x, y) = (px(area.left - pad), px(area.top - pad));
        let (w, h) = (px(area.right - area.left + 2.0 * pad), px(area.bottom - area.top + 2.0 * pad));
        let stem = file.with_extension("");
        let _ = std::process::Command::new("pdftocairo")
            .args(["-png", "-singlefile", "-r", "150"])
            .args(["-f", &page.to_string(), "-l", &page.to_string()])
            .args(["-x", &x.to_string(), "-y", &y.to_string(), "-W", &w.to_string(), "-H", &h.to_string()])
            .arg(pdf)
            .arg(&stem)
            .status();
    };
    for ch in &book.chapters {
        let notes = ch.notes.iter().chain(&ch.refs).flat_map(|n| &n.blocks);
        for b in ch.blocks.iter().map(|b| &b.block).chain(notes) {
            match b {
                crate::Block::Figure { page, n, area, .. } => cut(*page, area, &figure_file(*page, *n), 10.0),
                crate::Block::Para(inl) | crate::Block::Item { text: inl, .. } | crate::Block::PageNote { text: inl, .. } | crate::Block::Heading { text: inl, .. } => {
                    for x in inl {
                        if let crate::Inline::Picture { page, area } = x {
                            cut(*page, area, &picture_file(*page, area), 1.0);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn write_book(book: &Book, dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    std::fs::create_dir_all(dir)?;
    write_figures(book, dir);
    let mut out = Vec::new();
    for (i, ch) in book.chapters.iter().enumerate() {
        let p = dir.join(format!("{}.md", ch.slug));
        std::fs::write(&p, chapter_markdown(book, i))?;
        out.push(p);
    }
    if book.chapters.len() > 1 {
        let p = dir.join("book.md");
        std::fs::write(&p, book_markdown(book))?;
        out.push(p);
    }
    if !book.unplaced.is_empty() {
        let p = dir.join("unplaced-notes.md");
        let mut s = String::from(
            "# Unplaced notes and references\n\nThese could not be matched to a place in the text.\n\n",
        );
        for u in &book.unplaced {
            s.push_str(&format!("- {}\n", u));
        }
        std::fs::write(&p, s)?;
        out.push(p);
    }
    Ok(out)
}
