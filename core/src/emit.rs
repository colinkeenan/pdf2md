//! Write the chapters as Markdown files.

use crate::progress::Step;
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

/// A number right behind a formula is part of it ("$t =$0" is "$t = 0$"):
/// viewers do not take a "$" with a digit behind it for the end of a
/// formula (it could be a price), and would show the LaTeX as it is.
/// A formula read from a picture is left as read (it may be corrected by
/// hand); there a space parts the two.
fn numbers_into_formulas(mut inl: Vec<Inline>) -> Vec<Inline> {
    for i in 1..inl.len() {
        let read = i >= 2 && matches!(inl[i - 2], Inline::Source(_));
        let (before, after) = inl.split_at_mut(i);
        let (Some(Inline::Math(m)), Inline::Text(t)) = (before.last_mut(), &mut after[0]) else { continue };
        if !t.starts_with(|c: char| c.is_ascii_digit()) {
            continue;
        }
        if read {
            t.insert(0, ' ');
            continue;
        }
        // the digits, and a decimal point between digits
        let chars: Vec<char> = t.chars().collect();
        let n = (0..chars.len())
            .take_while(|&k| chars[k].is_ascii_digit() || (chars[k] == '.' && chars.get(k + 1).is_some_and(char::is_ascii_digit)))
            .count();
        *m = format!("{} {}", m.trim(), &t[..n]);
        *t = t[n..].to_string();
    }
    inl
}

/// LaTeX that would not render, made to: math inside `\text{…}` ("^" and
/// "_" mean nothing in text) is taken out of it, and two subscripts or
/// two superscripts on one symbol are one.
pub fn mend(latex: &str) -> String {
    let mut t = latex.to_string();
    // \text{ R^{-1}=1 \Rightarrow … }: a formula, no words in it
    let mut from = 0;
    while let Some(at) = t[from..].find("\\text{").map(|i| i + from) {
        let open = at + "\\text".len();
        let Some(close) = closing_brace(&t, open) else { break };
        let inside = &t[open + 1..close];
        let commands_out: String = {
            // (the letters of commands are no words)
            let mut out = String::new();
            let mut cs = inside.chars().peekable();
            while let Some(c) = cs.next() {
                if c == '\\' {
                    while cs.peek().is_some_and(|n| n.is_ascii_alphabetic()) {
                        cs.next();
                    }
                    out.push(' ');
                } else {
                    out.push(c);
                }
            }
            out
        };
        let math = inside.contains('^') || inside.contains('_');
        let words = commands_out.split(|c: char| !c.is_alphabetic()).any(|w| w.chars().count() >= 3);
        if math && !words {
            t = format!("{}{{{}}}{}", &t[..at], inside, &t[close + 1..]);
            from = at;
        } else {
            from = close + 1;
        }
    }
    // x_{a}_{b}: x_{a b}
    for mark in ['_', '^'] {
        let mut k = 0;
        while let Some(at) = t[k..].find(mark).map(|i| i + k) {
            k = at + 1;
            if at > 0 && t.as_bytes()[at - 1] == b'\\' {
                continue;
            }
            let Some((first, end)) = script(&t, at + 1) else { continue };
            let rest = &t[end..];
            let gap = rest.len() - rest.trim_start().len();
            if !rest.trim_start().starts_with(mark) {
                continue;
            }
            let Some((second, end2)) = script(&t, end + gap + 1) else { continue };
            t = format!("{}{mark}{{{first} {second}}}{}", &t[..at], &t[end2..]);
            k = at;
        }
    }
    t
}

/// The place of the "}" that closes the "{" at `open`.
fn closing_brace(t: &str, open: usize) -> Option<usize> {
    let b = t.as_bytes();
    if b.get(open) != Some(&b'{') {
        return None;
    }
    let mut depth = 0;
    let mut i = open;
    while i < b.len() {
        match b[i] {
            b'\\' => i += 1,
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

/// What a "_" or "^" is followed by at `at`: a group in braces (its
/// inside), a command, or one character; and where it ends.
fn script(t: &str, at: usize) -> Option<(String, usize)> {
    let rest = t.get(at..)?;
    let c = rest.chars().next()?;
    if c == '{' {
        let close = closing_brace(t, at)?;
        return Some((t[at + 1..close].to_string(), close + 1));
    }
    if c == '\\' {
        let n = 1 + rest[1..].chars().take_while(|c| c.is_ascii_alphabetic()).count();
        return (n > 1).then(|| (rest[..n].to_string(), at + n));
    }
    (!c.is_whitespace()).then(|| (c.to_string(), at + c.len_utf8()))
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
    let joined = numbers_into_formulas(joined);
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
                // (a formula right behind a formula is one with it: "$a$$b$"
                // would read as "$$", the mark of a displayed formula, and
                // put all that follows in the file out of joint)
                if s.ends_with('$') && !s.ends_with("\\$") && !s.ends_with("$$") && s.matches('$').count().is_multiple_of(2) {
                    s.pop();
                    s.push(' ');
                } else {
                    s.push('$');
                }
                let m = &mend(m);
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
            Inline::Source(k) => s.push_str(&format!("<!-- pdf2md:eq {k} -->")),
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

/// The bar above a page marker: where a printed page begins is easy to
/// find when scrolling. Translucent, in the colour of the text, so it shows
/// on a light and on a dark page alike.
const PAGE_BAR: &str = "<hr class=\"page\" style=\"height:2px;border:0;background:currentColor;opacity:0.25\">";

/// A paragraph with a page marker at `m`, parted for the bar: what goes
/// above it and what below. The page ends somewhere in a line of the
/// paragraph as the viewer sets it; the words up to the end of that line go
/// above the bar, so that line is full. (Lines as in the widest text column,
/// see `measure`; in a narrower window they end elsewhere.) Never past the
/// next page marker.
fn part_at_line_end(inl: &[Inline], m: usize) -> (Vec<Inline>, Vec<Inline>) {
    let is_marker = |x: &Inline| matches!(x, Inline::Anchor(id) if page_label(id).is_some());
    let end = inl.iter().enumerate().position(|(i, x)| i > m && is_marker(x)).unwrap_or(inl.len());
    let at_marker = (inl[..m].to_vec(), inl[m + 1..].to_vec());
    let Some((i, byte)) = line_end(&inl[..end], m) else { return at_marker };
    let mut above: Vec<Inline> = inl[..i].iter().enumerate().filter(|(k, _)| *k != m).map(|(_, x)| x.clone()).collect();
    let mut below = Vec::new();
    if byte > 0 {
        let cut = |t: &str| (t[..byte].to_string(), t[byte..].trim_start().to_string());
        let ((a, b), rest) = match &inl[i] {
            Inline::Text(t) => ((Inline::Text(cut(t).0), Inline::Text(cut(t).1)), cut(t).1),
            Inline::Emph(t) => ((Inline::Emph(cut(t).0), Inline::Emph(cut(t).1)), cut(t).1),
            Inline::Strong(t) => ((Inline::Strong(cut(t).0), Inline::Strong(cut(t).1)), cut(t).1),
            _ => return at_marker,
        };
        above.push(a);
        if !rest.is_empty() {
            below.push(b);
        }
        below.extend(inl[i + 1..].iter().cloned());
    } else {
        below.extend(inl[i..].iter().cloned());
    }
    (above, below)
}

/// Where the line ends in which the page marker `inl[m]` falls: in front of
/// byte `.1` of the text of `inl[.0]` (0: in front of the whole of it;
/// `inl.len()`: at the paragraph's end). None if the marker is at a line's
/// end already, or at the very start.
fn line_end(inl: &[Inline], m: usize) -> Option<(usize, usize)> {
    use crate::measure::{self, Face};
    // The words: each as wide as measured; a word runs across pieces with
    // no space between them ("$x$," is one).
    struct Word {
        width: f32,
        /// where it ends
        end: (usize, usize),
        /// a space follows it (none inside "wave-|optics" or a formula)
        spaced: bool,
        /// the line is broken after it (a kept line break)
        broken: bool,
    }
    let mut words: Vec<Word> = Vec::new();
    let mut open: Option<f32> = None;
    // the word the marker comes after or stands in, and whether it stands in it
    let mut marked: Option<(usize, bool)> = None;
    for (i, x) in inl.iter().enumerate() {
        // a link or a link target is kept whole: a line ending inside one
        // counts as ending in front of it
        let (t, face, scale, whole) = match x {
            Inline::Text(t) => (t.clone(), Face::Regular, 1.0, false),
            Inline::Emph(t) => (t.clone(), Face::Italic, 1.0, false),
            Inline::Strong(t) => (t.clone(), Face::Bold, 1.0, false),
            Inline::Target { text, emph, .. } => (text.clone(), if *emph { Face::Italic } else { Face::Regular }, 1.0, true),
            Inline::Link { text, .. } => (text.replace(['\\', '_', '*'], ""), Face::Regular, 1.0, true),
            Inline::Sup(t) => (t.replace(' ', ""), Face::Regular, measure::RAISED, true),
            Inline::NoteRef { label, .. } => (label.replace(' ', ""), Face::Regular, measure::RAISED, true),
            Inline::Cite { label, .. } => (format!("[{}]", label.replace(' ', "")), Face::Regular, 1.0, true),
            Inline::Math(l) => {
                // (a line ending inside a formula counts as ending in front of it)
                for (k, w) in measure::math(l).iter().enumerate() {
                    if k > 0 {
                        words.push(Word { width: open.take().unwrap_or(0.0), end: (i, 0), spaced: false, broken: false });
                    }
                    open = Some(open.unwrap_or(0.0) + w);
                }
                continue;
            }
            Inline::Picture { area, .. } => {
                // cut from the page at 150 dots per inch, shown dot for pixel
                let w = ((area.right - area.left) * 150.0 / 72.0).min(measure::LINE);
                open = Some(open.unwrap_or(0.0) + w);
                continue;
            }
            Inline::Break => {
                words.push(Word { width: open.take().unwrap_or(0.0), end: (i, 0), spaced: false, broken: true });
                continue;
            }
            Inline::Anchor(_) if i == m => {
                marked = match open {
                    Some(_) => Some((words.len(), true)),
                    None => Some((words.len().checked_sub(1)?, false)),
                };
                continue;
            }
            Inline::Anchor(_) | Inline::Source(_) | Inline::PageLink(_) => continue,
        };
        let mut from = 0;
        let mut before = ' ';
        for (at, c) in t.char_indices().chain([(t.len(), ' ')]) {
            let after = at + c.len_utf8().min(t.len() - at);
            // a line may also end after the hyphen or the stroke inside a
            // word ("wave-|optics", "and/|or"), and after a dash
            let inside = match c {
                '-' | '/' => before.is_alphanumeric() && t[after..].starts_with(char::is_alphabetic),
                '–' | '—' => t[after..].starts_with(|n: char| !n.is_whitespace()),
                _ => false,
            };
            before = c;
            if inside {
                let w = open.take().unwrap_or(0.0) + scale * measure::text(&t[from..after], face);
                words.push(Word { width: w, end: (i, if whole { 0 } else { after }), spaced: false, broken: false });
                from = after;
                continue;
            }
            if !c.is_whitespace() {
                continue;
            }
            if at > from {
                open = Some(open.unwrap_or(0.0) + scale * measure::text(&t[from..at], face));
            }
            // (the end of the piece is not the end of a word)
            if at < t.len() {
                if let Some(w) = open.take() {
                    words.push(Word { width: w, end: (i, if whole { 0 } else { at }), spaced: true, broken: false });
                }
            }
            from = after;
        }
    }
    if let Some(w) = open {
        words.push(Word { width: w, end: (inl.len(), 0), spaced: true, broken: false });
    }
    let (marked, inside) = marked?;
    // fill the lines word by word, up to the one the marker is in
    let (mut x, mut last) = (0.0, 0);
    let mut gap = 0.0;
    for (k, word) in words.iter().enumerate() {
        let fits = x == 0.0 || x + gap + word.width <= measure::LINE;
        if !fits && k > marked {
            break;
        }
        x = if fits { x + gap + word.width } else { word.width };
        gap = if word.spaced { measure::space() } else { 0.0 };
        last = k;
        if word.broken {
            if k >= marked {
                break;
            }
            (x, gap) = (0.0, 0.0);
        }
    }
    // (a line that ends in front of a link standing before the marker, or
    // right at the marker, leaves the marker where it is)
    let end = words[last].end;
    (end.0 > m && !(last == marked && !inside)).then_some(end)
}

fn render_block(b: &Block, out: &mut String, in_list: &mut bool) {
    let is_item = matches!(b, Block::Item { .. });
    if *in_list && !is_item {
        out.push('\n');
    }
    // A page marker stands on a line of its own, under its bar: one inside
    // a paragraph parts it at the end of the line the page ends in (so the
    // text above the bar runs to the right margin, as on the printed page);
    // one inside a heading or a list item goes in front of it.
    let is_marker = |x: &Inline| matches!(x, Inline::Anchor(id) if page_label(id).is_some());
    match b {
        Block::Para(inl) if inl.iter().any(is_marker) => {
            let m = inl.iter().position(is_marker).unwrap_or(0);
            let (above, below) = part_at_line_end(inl, m);
            if !above.is_empty() {
                render_block(&Block::Para(above), out, in_list);
            }
            if let Inline::Anchor(id) = &inl[m] {
                render_block(&Block::Anchor(id.clone()), out, in_list);
            }
            if !below.is_empty() {
                render_block(&Block::Para(below), out, in_list);
            }
            return;
        }
        Block::Heading { text, .. } | Block::Item { text, .. } | Block::PageNote { text, .. } if text.iter().any(is_marker) => {
            for m in text.iter().filter(|x| is_marker(x)) {
                if let Inline::Anchor(id) = m {
                    render_block(&Block::Anchor(id.clone()), out, in_list);
                }
            }
            let rest: Vec<Inline> = text.iter().filter(|x| !is_marker(x)).cloned().collect();
            let mut bare = b.clone();
            if let Block::Heading { text, .. } | Block::Item { text, .. } | Block::PageNote { text, .. } = &mut bare {
                *text = rest;
            }
            render_block(&bare, out, in_list);
            return;
        }
        _ => {}
    }
    *in_list = is_item;
    match b {
        Block::Heading { level, id, text, .. } => {
            let hashes = "#".repeat((*level).clamp(1, 6));
            // (a heading is styled by its #s: no italic or bold inside)
            let plain = crate::no_emph(text.clone());
            // (a line break inside it: on the heading's own line)
            out.push_str(&format!("{hashes} <a id=\"{id}\"></a>{}\n\n", render_inlines(&plain).replace("<br>\n", "<br>")));
        }
        Block::Para(inl) => {
            let t = render_inlines(inl);
            if !t.trim().is_empty() {
                out.push_str(t.trim());
                out.push_str("\n\n");
            }
        }
        Block::Equation { latex, tag, source } => {
            // an invisible anchor per equation number, for references to it
            for t in crate::equation_numbers(latex, tag.as_deref()) {
                out.push_str(&format!("<a id=\"{}\"></a>\n\n", crate::equation_id(&t)));
            }
            // which equation image this is (invisible): a hand correction
            // of the LaTeX below is kept for the next conversion; the anchor
            // is for the list of equations to check
            if let Some(k) = source {
                out.push_str(&format!("<a id=\"img-{k}\"></a>\n<!-- pdf2md:eq {k} -->\n"));
            }
            if latex.is_empty() {
                // only an equation number survived
                if let Some(t) = tag {
                    out.push_str(&format!("$$\n\\tag{{{t}}}\n$$\n\n"));
                }
                return;
            }
            let tag = tag.as_ref().map(|t| format!(" \\tag{{{t}}}")).unwrap_or_default();
            out.push_str(&format!("$$\n{}{tag}\n$$\n\n", mend(latex)));
        }
        Block::Anchor(id) => match page_label(id) {
            Some(l) => {
                let approx = if crate::page_is_approx(id) { "≈" } else { "" };
                out.push_str(&format!("{PAGE_BAR}\n<a id=\"{id}\"><sup>[{approx}p. {l}]</sup></a>\n\n"))
            }
            None => out.push_str(&format!("<a id=\"{id}\"></a>\n\n")),
        },
        Block::Item { level, text } => {
            out.push_str(&format!("{}- {}\n", "  ".repeat(*level), render_inlines(text)));
        }
        // columns side by side: a table of one row, Markdown in its cells
        Block::Columns(n) => out.push_str(&format!("<table width=\"100%\"><tr>\n<td valign=\"top\" width=\"{}%\">\n\n", 100 / (*n).max(1))),
        Block::ColumnBreak => out.push_str("</td>\n<td valign=\"top\">\n\n"),
        Block::ColumnsEnd => out.push_str("</td>\n</tr></table>\n\n"),
        Block::Picture { page, area, alt } => out.push_str(&format!("![{alt}]({})\n\n", picture_file(*page, area))),
        Block::Table { rows, widths } => {
            // (html, with each cell's text between empty lines: Markdown
            // is read inside it, so links and headings work)
            if widths.is_empty() {
                out.push_str("<table>\n");
            } else {
                out.push_str("<table width=\"100%\">\n<colgroup>");
                for w in widths {
                    out.push_str(&if *w > 0 { format!("<col width=\"{w}%\">") } else { "<col>".to_string() });
                }
                out.push_str("</colgroup>\n");
            }
            for row in rows {
                out.push_str("<tr>\n");
                for cell in row {
                    let mut tag = String::from("<td");
                    if cell.span > 1 {
                        tag.push_str(&format!(" colspan=\"{}\"", cell.span));
                    }
                    match cell.align {
                        crate::Align::Left => {}
                        crate::Align::Center => tag.push_str(" align=\"center\""),
                        crate::Align::Right => tag.push_str(" align=\"right\""),
                    }
                    let text = render_inlines(&cell.text).replace("<br>\n", "<br>");
                    if text.trim().is_empty() {
                        out.push_str(&format!("{tag}></td>\n"));
                    } else {
                        let hashes = if cell.heading > 0 { format!("{} ", "#".repeat(cell.heading.clamp(1, 6))) } else { String::new() };
                        out.push_str(&format!("{tag}>\n\n{hashes}{}\n\n</td>\n", text.trim()));
                    }
                }
                out.push_str("</tr>\n");
            }
            out.push_str("</table>\n\n");
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
        Block::Rule => out.push_str("---\n\n"),
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
    // (a title set centered, or in lines of its own: as it is set)
    let title = if ch.heading.is_empty() { ch.title.clone() } else { render_inlines(&crate::no_emph(ch.heading.clone())).replace("<br>\n", "<br>") };
    let mut s = if ch.centered { format!("<div align=\"center\">\n\n# {title}\n\n</div>\n\n") } else { format!("# {title}\n\n") };
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
                crate::Block::Para(inl) => crate::links::caption(inl).filter(|(kind, _)| *kind == "figure"),
                _ => None,
            }) {
                s.push_str(&format!("![Fig. {}]({})\n\n", own.1, figure_file(*page, *n)));
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
/// Notes whose place was not found again: at the end, never dropped.
fn lost_notes(lost: &[crate::userdata::Note]) -> String {
    if lost.is_empty() {
        return String::new();
    }
    let mut s = String::from("\n---\n\n## Your notes whose place was not found\n\nThe text these followed changed in this conversion; move each one back where it belongs.\n\n");
    for n in lost {
        s.push_str(&format!("{}\n\n", n.text));
    }
    s
}

/// The equations worth checking against the book: read from their image
/// as a different formula a second time, not corrected since. Links with
/// each equation's number and printed page, biggest differences first,
/// each group in book order. None if there are none.
fn check_list(book: &Book) -> Option<String> {
    // (symbols that differ, line of the list)
    let mut items: Vec<(usize, String)> = Vec::new();
    for ch in &book.chapters {
        let mut page: Option<String> = None;
        let see = |id: &str, page: &mut Option<String>| {
            if let Some(l) = page_label(id) {
                *page = Some(format!("{}p. {l}", if crate::page_is_approx(id) { "≈" } else { "" }));
            }
        };
        for b in &ch.blocks {
            match &b.block {
                Block::Anchor(id) => see(id, &mut page),
                Block::Para(inl) => {
                    for x in inl {
                        if let Inline::Anchor(id) = x {
                            see(id, &mut page);
                        }
                    }
                }
                Block::Equation { tag, source: Some(k), .. } => {
                    if let Some(n) = crate::ocr::needs_check(k) {
                        let name = tag.as_ref().map_or("equation".to_string(), |t| format!("({t})"));
                        let at = page.as_ref().map(|p| format!(", {p}")).unwrap_or_default();
                        let symbols = if n == 1 { "1 symbol".to_string() } else { format!("{n} symbols") };
                        items.push((n, format!("- [{name}]({}.md#img-{k}) — {}{at} — {symbols} differ", ch.slug, ch.title)));
                    }
                }
                _ => {}
            }
        }
    }
    if items.is_empty() {
        return None;
    }
    let group = |title: &str, keep: &dyn Fn(usize) -> bool| -> String {
        let lines: Vec<&str> = items.iter().filter(|(n, _)| keep(*n)).map(|(_, l)| l.as_str()).collect();
        if lines.is_empty() {
            String::new()
        } else {
            format!("## {title} ({})\n\n{}\n\n", lines.len(), lines.join("\n"))
        }
    };
    Some(format!(
        "# Equations to check\n\nThese equations were read from their pictures as a different formula on a second reading (formatting aside), so compare them with the book. The bigger the difference, the likelier the reading is wrong. Correct one in the chapter file (or in book.md); the next conversion keeps your correction and drops it from this list.\n\n{}{}{}",
        group("Big differences (more than 10 symbols)", &|n| n > 10),
        group("Medium differences (4–10 symbols)", &|n| (4..=10).contains(&n)),
        group("Small differences (1–3 symbols)", &|n| n <= 3),
    ))
}

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
    // which pictures there are, then one by one (to tell how far it is)
    let mut todo: Vec<(u32, &crate::layout::Figure, String, f32)> = Vec::new();
    for ch in &book.chapters {
        let notes = ch.notes.iter().chain(&ch.refs).flat_map(|n| &n.blocks);
        for b in ch.blocks.iter().map(|b| &b.block).chain(notes) {
            match b {
                crate::Block::Figure { page, n, area, .. } => todo.push((*page, area, figure_file(*page, *n), 10.0)),
                crate::Block::Picture { page, area, .. } => todo.push((*page, area, picture_file(*page, area), 4.0)),
                crate::Block::Para(inl) | crate::Block::Item { text: inl, .. } | crate::Block::PageNote { text: inl, .. } | crate::Block::Heading { text: inl, .. } => {
                    for x in inl {
                        if let crate::Inline::Picture { page, area } = x {
                            todo.push((*page, area, picture_file(*page, area), 1.0));
                        }
                    }
                }
                _ => {}
            }
        }
    }
    for (k, (page, area, name, pad)) in todo.iter().enumerate() {
        crate::progress::report(Step::Pictures, k, todo.len());
        cut(*page, area, name, *pad);
    }
    crate::progress::report(Step::Pictures, todo.len(), todo.len());
}

pub fn write_book(book: &Book, dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    std::fs::create_dir_all(dir)?;
    write_figures(book, dir);
    crate::progress::report(Step::Files, 0, 1);
    let mut out = Vec::new();
    let mut written = Vec::new();
    let texts: Vec<String> = (0..book.chapters.len()).map(|i| chapter_markdown(book, i)).collect();
    // notes made when the chapter files had other names go where their
    // chapters are now
    let mut user = book.user.clone();
    let files: Vec<(&str, &str)> = book.chapters.iter().zip(&texts).map(|(c, t)| (c.slug.as_str(), t.as_str())).collect();
    crate::userdata::follow_renames(&mut user, &files);
    for (ch, text) in book.chapters.iter().zip(&texts) {
        let p = dir.join(format!("{}.md", ch.slug));
        let mut s = text.clone();
        // the reader's notes and fixes from the earlier conversion
        let lost = crate::userdata::apply(&mut s, &user, Some(&ch.slug));
        s.push_str(&lost_notes(&lost));
        std::fs::write(&p, &s)?;
        written.push(s);
        out.push(p);
    }
    // what was written for each equation, to tell later edits from it
    crate::ocr::record_written(dir, &written.iter().map(String::as_str).collect::<Vec<_>>())?;
    if let Some(list) = check_list(book) {
        let p = dir.join("check-equations.md");
        std::fs::write(&p, list)?;
        out.push(p);
    }
    if book.chapters.len() > 1 {
        let p = dir.join("book.md");
        let mut s = book_markdown(book);
        let lost = crate::userdata::apply(&mut s, &user, None);
        s.push_str(&lost_notes(&lost));
        std::fs::write(&p, s)?;
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
    crate::progress::report(Step::Files, 1, 1);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_page_marker_stands_alone_under_its_bar() {
        let para = Block::Para(vec![
            Inline::Text("The page ends ".into()),
            Inline::Anchor("p8".into()),
            Inline::Text("in this sentence.".into()),
        ]);
        // the paragraph ends in the line the page ends in: the bar comes after it
        let (mut out, mut in_list) = (String::new(), false);
        render_block(&para, &mut out, &mut in_list);
        assert_eq!(out, format!("The page ends in this sentence.\n\n{PAGE_BAR}\n<a id=\"p8\"><sup>[p. 8]</sup></a>\n\n"));
        // in a heading: in front of it
        let head = Block::Heading { level: 1, size: 0.0, id: "s-x".into(), text: vec![Inline::Anchor("p9-approx".into()), Inline::Text("Title".into())] };
        let mut out = String::new();
        render_block(&head, &mut out, &mut in_list);
        assert_eq!(out, format!("{PAGE_BAR}\n<a id=\"p9-approx\"><sup>[≈p. 9]</sup></a>\n\n# <a id=\"s-x\"></a>Title\n\n"));
    }

    /// "aaaa" is 35.9 px wide and a space 4.2 px: 19 of them fill a line.
    fn words(n: usize) -> String {
        vec!["aaaa"; n].join(" ")
    }

    fn parted(before: usize, after: usize) -> (String, String) {
        let para = vec![
            Inline::Text(words(before) + " "),
            Inline::Anchor("p8".into()),
            Inline::Text(words(after)),
        ];
        let (a, b) = part_at_line_end(&para, 1);
        (render_inlines(&a).trim().to_string(), render_inlines(&b).trim().to_string())
    }

    #[test]
    fn the_text_above_a_page_bar_ends_with_a_full_line() {
        // the page ends 5 words into the first line: the line is filled
        assert_eq!(parted(5, 35), (words(19), words(21)));
        // ... 6 words into the second line
        assert_eq!(parted(25, 35), (words(38), words(22)));
        // at a line's end: nothing moves
        assert_eq!(parted(19, 21), (words(19), words(21)));
        assert_eq!(parted(38, 2), (words(38), words(2)));
        // the paragraph ends in that line: all of it goes above the bar
        assert_eq!(parted(20, 3), (words(23), String::new()));
    }

    #[test]
    fn a_line_does_not_end_inside_a_formula_or_a_link() {
        // "$x$," is one word: the comma is not left behind
        let para = vec![
            Inline::Text(words(5) + " "),
            Inline::Anchor("p8".into()),
            Inline::Text(words(13) + " "),
            Inline::Math("x".into()),
            Inline::Text(", ".to_string() + &words(10)),
        ];
        let (a, b) = part_at_line_end(&para, 1);
        assert_eq!(render_inlines(&a).trim(), format!("{} $x$,", words(18)));
        assert_eq!(render_inlines(&b).trim(), words(10));
        // a link the line would end in goes below the bar whole
        let para = vec![
            Inline::Text(words(5) + " "),
            Inline::Anchor("p8".into()),
            Inline::Text(words(12) + " "),
            Inline::Link { text: words(5), href: "#x".into() },
            Inline::Text(" ".to_string() + &words(10)),
        ];
        let (a, b) = part_at_line_end(&para, 1);
        assert_eq!(render_inlines(&a).trim(), words(17));
        assert!(render_inlines(&b).starts_with(&format!("[{}](#x)", words(5))));
        // the next page's marker is not passed
        let para = vec![
            Inline::Text(words(5) + " "),
            Inline::Anchor("p8".into()),
            Inline::Text(words(3) + " "),
            Inline::Anchor("p9".into()),
            Inline::Text(words(30)),
        ];
        let (a, b) = part_at_line_end(&para, 1);
        assert_eq!(render_inlines(&a).trim(), words(8));
        assert!(matches!(&b[0], Inline::Anchor(id) if id == "p9"));
    }

    #[test]
    fn a_number_right_behind_a_formula_is_part_of_it() {
        // two formulas with nothing between them are written as one
        let two = vec![Inline::Text("is ".into()), Inline::Math("\\pm 2".into()), Inline::Math("\\pi".into()), Inline::Text(", so".into())];
        assert_eq!(render_inlines(&two), "is $\\pm 2 \\pi$, so");
        // LaTeX that would not render is mended
        assert_eq!(mend("\\text{If:}\\ \\text{ R^{-1}=1\\Rightarrow\\mathscr{D}(R) }"), "\\text{If:}\\ { R^{-1}=1\\Rightarrow\\mathscr{D}(R) }");
        assert_eq!(mend("\\text{ for all x_i }"), "\\text{ for all x_i }");
        assert_eq!(mend("N N_{4}_{5} + a^2^{3} + x_1^2 + \\_"), "N N_{4 5} + a^{2 3} + x_1^2 + \\_");
        let inl = vec![Inline::Text("at ".into()), Inline::Math("t =".into()), Inline::Text("0.5. Then".into())];
        assert_eq!(render_inlines(&inl), "at $t = 0.5$. Then");
        // a formula read from a picture is left as read
        let inl = vec![Inline::Source("k".into()), Inline::Math("x".into()), Inline::Text("2 times".into())];
        assert_eq!(render_inlines(&inl), "<!-- pdf2md:eq k -->$x$ 2 times");
    }
}
