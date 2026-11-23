//! A course as a book: a folder of saved web pages and of PDFs (lecture
//! notes, problem sets, exams) becomes one book, a file for each PDF.
//!
//! The folder holds
//!
//! * pages of the course's site saved as HTML (`syllabus.html`,
//!   `lecture-notes.html`, …): a page that links to PDFs names them and
//!   puts them in order; a page without PDFs (the syllabus) is a file of
//!   the book;
//! * `topics-covered.html`, if there is one: the outline of the course, the
//!   frame of the book's contents. The PDFs of the page that tells each
//!   one's topics (the lecture notes) are listed under the topic they
//!   belong to;
//! * a subfolder of PDFs for each such page (`Lecture Notes/`), the PDFs
//!   named as the page's links name them.
//!
//! Each PDF is converted as a book of its own and written as one file.

use crate::emit;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------- HTML

#[derive(Debug, Clone, PartialEq)]
enum Tok {
    Open(String, Option<String>),
    Close(String),
    Text(String),
}

fn decode(t: &str) -> String {
    let mut out = String::new();
    let mut rest = t;
    while let Some(i) = rest.find('&') {
        out.push_str(&rest[..i]);
        rest = &rest[i..];
        let end = rest.find(';').filter(|e| *e <= 10);
        let Some(e) = end else {
            out.push('&');
            rest = &rest[1..];
            continue;
        };
        let name = &rest[1..e];
        let c = match name {
            "amp" => Some('&'),
            "lt" => Some('<'),
            "gt" => Some('>'),
            "quot" => Some('"'),
            "apos" => Some('\''),
            "nbsp" => Some(' '),
            "ldquo" => Some('“'),
            "rdquo" => Some('”'),
            "lsquo" => Some('‘'),
            "rsquo" => Some('’'),
            "ndash" => Some('–'),
            "mdash" => Some('—'),
            "hellip" => Some('…'),
            _ => name
                .strip_prefix("#x")
                .or_else(|| name.strip_prefix("#X"))
                .and_then(|h| u32::from_str_radix(h, 16).ok())
                .or_else(|| name.strip_prefix('#').and_then(|d| d.parse().ok()))
                .and_then(char::from_u32),
        };
        match c {
            Some(c) => {
                out.push(c);
                rest = &rest[e + 1..];
            }
            None => {
                out.push('&');
                rest = &rest[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

/// The tags and the text between them (comments, scripts and styles left
/// out); of a tag's attributes only `href` is kept.
fn tokens(html: &str) -> Vec<Tok> {
    let mut out = Vec::new();
    let mut rest = html;
    let mut skip: Option<String> = None;
    while !rest.is_empty() {
        if let Some(r) = rest.strip_prefix("<!--") {
            rest = r.find("-->").map_or("", |e| &r[e + 3..]);
            continue;
        }
        if rest.starts_with('<') {
            let Some(e) = rest.find('>') else { break };
            let tag = &rest[1..e];
            rest = &rest[e + 1..];
            let close = tag.starts_with('/');
            let body = tag.trim_start_matches('/').trim_end_matches('/');
            let name: String = body.chars().take_while(|c| c.is_ascii_alphanumeric()).collect::<String>().to_lowercase();
            if name.is_empty() {
                continue;
            }
            if let Some(s) = &skip {
                if close && *s == name {
                    skip = None;
                }
                continue;
            }
            if !close && (name == "script" || name == "style") {
                skip = Some(name);
                continue;
            }
            if close {
                out.push(Tok::Close(name));
            } else {
                let href = body.find("href=\"").map(|i| &body[i + 6..]).and_then(|h| h.find('"').map(|e| decode(&h[..e])));
                out.push(Tok::Open(name, href));
            }
            continue;
        }
        let e = rest.find('<').unwrap_or(rest.len());
        if skip.is_none() {
            out.push(Tok::Text(decode(&rest[..e])));
        }
        rest = &rest[e..];
    }
    out
}

fn squeeze(t: &str) -> String {
    t.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The page's own content: what `<main id="course-content-section">` or
/// `<main>` or `<body>` holds.
fn content(html: &str) -> &str {
    let from = html.rfind("<main").or_else(|| html.find("<body")).unwrap_or(0);
    let to = html[from..].find("</main>").or_else(|| html[from..].find("</body>")).map_or(html.len(), |e| from + e);
    &html[from..to]
}

/// HTML as Markdown: headings, paragraphs, lists, tables, links. `link`
/// gives the place a link goes to in the book (None: the link as it is,
/// when it is a whole address; else its words alone). The headings start
/// at `top` (2: "##").
fn markdown(html: &str, top: usize, link: &dyn Fn(&str) -> Option<String>) -> String {
    let toks = tokens(content(html));
    let highest = toks.iter().filter_map(|t| match t {
        Tok::Open(n, _) if n.len() == 2 && n.starts_with('h') => n[1..].parse::<usize>().ok(),
        _ => None,
    }).min().unwrap_or(1);
    let mut out = String::new();
    let mut cur = String::new();
    let mut lists: Vec<(bool, usize)> = Vec::new();
    let mut item: Option<String> = None; // the marker of the list item that is open
    let mut heading: Option<usize> = None;
    let mut anchors: Vec<(usize, Option<String>)> = Vec::new();
    let mut table: Option<Vec<Vec<String>>> = None;
    let mut cell = false;
    let flush = |cur: &mut String, out: &mut String, item: &mut Option<String>, heading: &mut Option<usize>, depth: usize| {
        let t = squeeze(cur);
        cur.clear();
        if t.is_empty() {
            return;
        }
        if let Some(h) = heading.take() {
            out.push_str(&format!("{} {t}\n\n", "#".repeat(h)));
        } else if let Some(mark) = item.take() {
            out.push_str(&format!("{}{mark}{t}\n", "   ".repeat(depth.saturating_sub(1))));
        } else if depth > 0 {
            out.push_str(&format!("{}{t}\n", "   ".repeat(depth)));
        } else {
            out.push_str(&format!("{t}\n\n"));
        }
    };
    for t in toks {
        match t {
            Tok::Text(t) => cur.push_str(&emit::escape_text(&t)),
            Tok::Open(n, href) => match n.as_str() {
                // (a title inside a link is the link's words)
                "h1" | "h2" | "h3" | "h4" | "h5" | "h6" if !anchors.is_empty() => {}
                "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                    flush(&mut cur, &mut out, &mut item, &mut heading, lists.len());
                    let level: usize = n[1..].parse().unwrap_or(1);
                    heading = Some((top + level - highest).min(6));
                }
                "p" | "div" | "section" | "article" | "main" if table.is_none() && anchors.is_empty() => flush(&mut cur, &mut out, &mut item, &mut heading, lists.len()),
                // (in a cell of a table: a new line)
                "p" | "li" if cell && !squeeze(&cur).is_empty() && !cur.trim_end().ends_with("<br>") => cur.push_str("<br>"),
                "ol" | "ul" if table.is_none() => {
                    flush(&mut cur, &mut out, &mut item, &mut heading, lists.len());
                    lists.push((n == "ol", 0));
                }
                "li" if table.is_none() => {
                    flush(&mut cur, &mut out, &mut item, &mut heading, lists.len());
                    if let Some(l) = lists.last_mut() {
                        l.1 += 1;
                        item = Some(if l.0 { format!("{}. ", l.1) } else { "- ".to_string() });
                    }
                }
                "table" => {
                    flush(&mut cur, &mut out, &mut item, &mut heading, lists.len());
                    table = Some(Vec::new());
                }
                "tr" => {
                    if let Some(t) = table.as_mut() {
                        t.push(Vec::new());
                    }
                }
                "td" | "th" => {
                    cur.clear();
                    cell = true;
                }
                "br" => cur.push_str(if cell { " " } else { "<br>" }),
                "em" | "i" => cur.push('_'),
                "strong" | "b" => cur.push_str("**"),
                "a" => anchors.push((cur.len(), href)),
                _ => {}
            },
            Tok::Close(n) => match n.as_str() {
                "h1" | "h2" | "h3" | "h4" | "h5" | "h6" if !anchors.is_empty() => {}
                "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "p" | "li" if table.is_none() => flush(&mut cur, &mut out, &mut item, &mut heading, lists.len()),
                "ol" | "ul" if table.is_none() => {
                    flush(&mut cur, &mut out, &mut item, &mut heading, lists.len());
                    lists.pop();
                    if lists.is_empty() {
                        out.push('\n');
                    }
                }
                "td" | "th" => {
                    if let Some(row) = table.as_mut().and_then(|t| t.last_mut()) {
                        row.push(squeeze(&cur).replace('|', "\\|"));
                    }
                    cur.clear();
                    cell = false;
                }
                "table" => {
                    if let Some(rows) = table.take() {
                        let rows: Vec<Vec<String>> = rows.into_iter().filter(|r| !r.is_empty()).collect();
                        let width = rows.iter().map(Vec::len).max().unwrap_or(0);
                        for (k, r) in rows.iter().enumerate() {
                            let mut r = r.clone();
                            r.resize(width, String::new());
                            out.push_str(&format!("| {} |\n", r.join(" | ")));
                            if k == 0 {
                                out.push_str(&format!("|{}\n", "---|".repeat(width)));
                            }
                        }
                        out.push('\n');
                    }
                    cur.clear();
                }
                "em" | "i" => cur.push('_'),
                "strong" | "b" => cur.push_str("**"),
                "a" => {
                    if let Some((at, href)) = anchors.pop() {
                        let at = at.min(cur.len());
                        let words = squeeze(&cur[at..]);
                        let to = href.as_deref().and_then(|h| link(h).or_else(|| (h.starts_with("http://") || h.starts_with("https://")).then(|| h.to_string())));
                        cur.truncate(at);
                        match to {
                            Some(to) if !words.is_empty() => cur.push_str(&format!("[{words}]({to})")),
                            _ => cur.push_str(&words),
                        }
                    }
                }
                _ => {}
            },
        }
    }
    flush(&mut cur, &mut out, &mut item, &mut heading, lists.len());
    let mut text = String::new();
    for line in out.lines() {
        // (no more than one empty line)
        if !(line.is_empty() && text.ends_with("\n\n")) {
            text.push_str(line);
            text.push('\n');
        }
    }
    text
}

/// The first list of a page, as (depth from 1, words) of each of its items.
fn outline(html: &str) -> Vec<(usize, String)> {
    let mut out: Vec<(usize, String)> = Vec::new();
    let mut depth = 0usize;
    let mut cur: Option<String> = None;
    let mut done = false;
    for t in tokens(content(html)) {
        if done {
            break;
        }
        match t {
            Tok::Open(n, _) if n == "ol" || n == "ul" => {
                if let Some(c) = cur.take() {
                    out.push((depth, squeeze(&c)));
                }
                depth += 1;
            }
            Tok::Close(n) if n == "ol" || n == "ul" => {
                if let Some(c) = cur.take() {
                    out.push((depth, squeeze(&c)));
                }
                depth = depth.saturating_sub(1);
                done = depth == 0;
            }
            Tok::Open(n, _) if n == "li" && depth > 0 => {
                if let Some(c) = cur.take() {
                    out.push((depth, squeeze(&c)));
                }
                cur = Some(String::new());
            }
            Tok::Close(n) if n == "li" => {
                if let Some(c) = cur.take() {
                    out.push((depth, squeeze(&c)));
                }
            }
            Tok::Text(t) => {
                if let Some(c) = cur.as_mut() {
                    c.push_str(&t);
                }
            }
            _ => {}
        }
    }
    out.retain(|(_, t)| !t.is_empty());
    out
}

/// One thing a page lists: a PDF by its link, or what the page says is not
/// there ("Lecture 16 Notes not available").
#[derive(Debug, Clone, Default)]
struct Entry {
    label: String,
    /// the link's last piece ("mit8_321f17_lec1"), which names the file
    name: Option<String>,
    /// what the page tells of it beside the link (the lecture's topics)
    about: String,
}

fn link_name(href: &str) -> Option<String> {
    let h = href.split(['?', '#']).next().unwrap_or(href).trim_end_matches('/');
    let last = h.rsplit('/').next()?;
    let stem = last.strip_suffix(".pdf").unwrap_or(last);
    // (a newer site names the page of "pset01.pdf" "pset01_pdf")
    let stem = if h.contains("/resources/") { stem.strip_suffix("_pdf").unwrap_or(stem) } else { stem };
    (h.contains("/resources/") || last.ends_with(".pdf")).then(|| crate::key(stem))
}

fn tidy_label(t: &str) -> String {
    let t = squeeze(t);
    let t = t.trim_matches(|c| c == '[' || c == ']').trim();
    // ("Homework #1 (PDF)", "(PDF - 2.2 MB)")
    let t = match t.rfind("(PDF") {
        Some(k) if t.ends_with(')') => t[..k].trim(),
        _ => t,
    };
    // ("(PDF - 2.2 MB)" was in brackets of its own)
    let t = if t.starts_with('(') && t.ends_with(')') && !t[1..].contains('(') { t[1..t.len() - 1].trim() } else { t };
    t.to_string()
}

/// What a page lists, in its order.
fn entries(html: &str) -> Vec<Entry> {
    let toks = tokens(content(html));
    let mut out: Vec<Entry> = Vec::new();
    // pieces of text between block tags, with the link inside each
    let mut pieces: Vec<(String, Option<String>, bool, usize)> = Vec::new(); // (text, link, in a table, row)
    let mut cur = String::new();
    let mut link: Option<String> = None;
    let mut inside_link = false;
    let (mut in_table, mut row) = (false, 0usize);
    for t in &toks {
        match t {
            Tok::Text(t) => cur.push_str(t),
            Tok::Open(n, href) if n == "a" => {
                inside_link = true;
                if let Some(name) = href.as_deref().and_then(link_name) {
                    link = Some(name);
                }
            }
            Tok::Close(n) if n == "a" => inside_link = false,
            Tok::Open(n, _) | Tok::Close(n) if matches!(n.as_str(), "p" | "li" | "td" | "th" | "tr" | "table" | "h1" | "h2" | "h3" | "h4" | "ul" | "ol" | "div") => {
                let text = squeeze(&cur);
                if !text.is_empty() {
                    pieces.push((text, link.clone(), in_table, row));
                }
                cur.clear();
                // (a link around several pieces, a picture and a title, is
                // the link of each)
                if !inside_link {
                    link = None;
                }
                if n == "table" {
                    in_table = matches!(t, Tok::Open(..));
                }
                if n == "tr" && matches!(t, Tok::Open(..)) {
                    row += 1;
                }
            }
            _ => {}
        }
    }
    let text = squeeze(&cur);
    if !text.is_empty() {
        pieces.push((text, link.take(), in_table, row));
    }
    let listed = |p: &(String, Option<String>, bool, usize)| p.1.is_some() || p.0.to_lowercase().contains("not available");
    // the head of the table a piece is in: the cells of its first row
    let head_of = |k: usize| -> Vec<&str> {
        let start = (0..=k).rev().take_while(|&i| pieces[i].2).last().unwrap_or(k);
        pieces[start..].iter().take_while(|q| q.2 && q.3 == pieces[start].3).map(|q| q.0.as_str()).collect()
    };
    let mut rows: Vec<usize> = Vec::new(); // the table row of each entry (0: none)
    for (k, p) in pieces.iter().enumerate() {
        if !listed(p) {
            continue;
        }
        // (the last cell of its row tells its topics)
        let about = if p.2 { pieces[k + 1..].iter().take_while(|q| q.2 && q.3 == p.3).last().filter(|q| q.1.is_none()).map(|q| q.0.clone()).unwrap_or_default() } else { String::new() };
        let mut label = tidy_label(&p.0);
        // (a cell that holds the link alone, "(PDF)": the row's other cells
        // name it, "1" and "Symmetries, Lie Groups and Lie Algebras" under
        // "LEC #" and "TOPICS": "Lecture 1: Symmetries, …")
        if p.2 && (label.is_empty() || label == "PDF") {
            let head = head_of(k);
            let row: Vec<&(String, Option<String>, bool, usize)> = pieces.iter().filter(|q| q.2 && q.3 == p.3).collect();
            let at = row.iter().position(|q| std::ptr::eq(*q, p)).unwrap_or(0);
            let cells: Vec<String> = row.iter().filter(|q| !std::ptr::eq(**q, p)).map(|q| tidy_label(&q.0)).filter(|t| !t.is_empty() && t != "PDF").collect();
            let counted = match head.first().map(|h| h.to_uppercase()) {
                Some(h) if h.starts_with("LEC") => "Lecture",
                Some(h) if h.starts_with("SES") => "Session",
                Some(h) if h.starts_with("WEEK") => "Week",
                _ => "",
            };
            let numbered = cells.first().is_some_and(|c| c.chars().next().is_some_and(|d| d.is_ascii_digit()) && c.chars().all(|d| d.is_ascii_digit() || " -–".contains(d)));
            label = if cells.len() >= 2 && numbered && !counted.is_empty() {
                let plural = if cells[0].contains(['-', '–']) { "s" } else { "" };
                format!("{counted}{plural} {}: {}", cells[0].replace(" - ", "–"), cells[1..].join(", "))
            } else {
                cells.join(": ")
            };
            // (a second file of the row, under its own head: "Solutions")
            if out.iter().zip(&rows).any(|(e, r)| *r == p.3 && e.label == label) {
                if let Some(h) = head.get(at.min(head.len().saturating_sub(1))).filter(|_| row.len() == head.len()) {
                    label = format!("{label} — {}", title_of(&h.to_lowercase()));
                }
            }
        }
        out.push(Entry { label, name: p.1.clone(), about });
        rows.push(if p.2 { p.3 } else { 0 });
    }
    // (a file listed twice, by its picture and by its title: once, under
    // the longer name)
    let mut once: Vec<Entry> = Vec::new();
    for e in out {
        match once.iter_mut().find(|o| o.name.is_some() && o.name == e.name) {
            Some(o) => {
                if e.label.chars().count() > o.label.chars().count() {
                    o.label = e.label;
                }
            }
            None => once.push(e),
        }
    }
    // (what all the names begin with, the course's name, is left out)
    let files: Vec<usize> = (0..once.len()).filter(|&k| once[k].name.is_some()).collect();
    if files.len() >= 2 {
        let first = once[files[0]].label.clone();
        let mut common = first.len();
        for &k in &files[1..] {
            common = common.min(first.bytes().zip(once[k].label.bytes()).take_while(|(a, b)| a == b).count());
        }
        while !first.is_char_boundary(common) {
            common -= 1;
        }
        if let Some(cut) = first[..common].rfind(", ").map(|k| k + 2) {
            for &k in &files {
                once[k].label = once[k].label[cut..].to_string();
            }
        }
    }
    once
}

// ---------------------------------------------------------------- the book

/// A file of the book.
struct File {
    slug: String,
    title: String,
    body: String,
}

/// "Lecture Notes" of "lecture-notes".
fn title_of(stem: &str) -> String {
    stem.split(['-', '_', ' '])
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut c = w.chars();
            c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// "lec2" before "lec10".
fn natural(a: &str, b: &str) -> std::cmp::Ordering {
    let parts = |s: &str| -> Vec<(String, u64)> {
        let mut v = Vec::new();
        let mut word = String::new();
        let mut num = String::new();
        for c in s.to_lowercase().chars() {
            if c.is_ascii_digit() {
                num.push(c);
            } else {
                if !num.is_empty() {
                    v.push((std::mem::take(&mut word), num.parse().unwrap_or(0)));
                    num.clear();
                }
                word.push(c);
            }
        }
        v.push((word, num.parse().unwrap_or(0)));
        v
    };
    parts(a).cmp(&parts(b))
}

fn slug_words(t: &str) -> String {
    let mut s = String::new();
    for c in t.to_lowercase().chars() {
        if c.is_ascii_alphanumeric() {
            s.push(c);
        } else if !s.ends_with('-') && !s.is_empty() {
            s.push('-');
        }
    }
    s.trim_matches('-').to_string()
}

/// One PDF as the text of one file: its chapters one after the other, the
/// headings a step down (the file's title stands above them), the links to
/// the PDF and to the pictures as they are in the course's folder. Also
/// gives the page that only says where the PDF is from, if it has one.
fn pdf_text(pdf: &Path, stem: &str, out: &Path) -> Result<(String, Option<String>), String> {
    // (the last page of each of a course's PDFs only says where it is from:
    // it is left out, and told once in the contents)
    let run = |cmd: &str, args: &[&str]| std::process::Command::new(cmd).args(args).arg(pdf).arg("-").output().ok().map(|o| String::from_utf8_lossy(&o.stdout).into_owned());
    let pages: Option<u32> = std::process::Command::new("pdfinfo")
        .arg(pdf)
        .output()
        .ok()
        .and_then(|o| String::from_utf8_lossy(&o.stdout).lines().find_map(|l| l.strip_prefix("Pages:").and_then(|n| n.trim().parse().ok())));
    let mut source = None;
    let mut range = None;
    if let Some(n) = pages.filter(|n| *n > 1) {
        let last = run("pdftotext", &["-f", &n.to_string(), "-l", &n.to_string()]).unwrap_or_default();
        let words = squeeze(&last);
        if words.contains("ocw.mit.edu") && words.len() < 400 {
            let lines: Vec<String> = last.lines().map(squeeze).filter(|l| !l.is_empty()).collect();
            source = Some(lines.join(" · "));
            range = Some((1, n - 1));
        }
    }
    let written_by_hand = handwritten(pdf);
    // (pages that are pictures of print, or set in fonts that are drawn
    // letter by letter and have no names: read by the page reader, and
    // each page's picture given under what was read, to check it by)
    let pictured = !written_by_hand && pictures_of_print(pdf);
    let book = crate::convert_pdf_as(pdf, range, written_by_hand || pictured)?;
    // the pictures, named after the PDF
    let tmp = out.join(format!(".course-{stem}"));
    let _ = std::fs::remove_dir_all(&tmp);
    emit::write_figures(&book, &tmp);
    if let Ok(files) = std::fs::read_dir(tmp.join("images")) {
        let _ = std::fs::create_dir_all(out.join("images"));
        for f in files.flatten() {
            emit::move_changed(&f.path(), &out.join("images").join(format!("{stem}-{}", f.file_name().to_string_lossy())));
        }
    }
    let _ = std::fs::remove_dir_all(&tmp);
    // (what stands before the first section, the PDF's own title and
    // date, keeps its headings; the sections go a step down)
    let bodies: Vec<String> = (0..book.chapters.len())
        .map(|i| {
            let front = book.chapters[i].title == "Front Matter";
            let mut text = String::new();
            let mut fenced = false;
            for line in emit::chapter_body(&book, i).lines() {
                if line.starts_with("```") {
                    fenced = !fenced;
                }
                if !fenced && front && line.trim_end() == "# Front Matter" {
                    continue;
                }
                if !fenced && !front && line.starts_with('#') && !line.starts_with("######") {
                    text.push('#');
                }
                text.push_str(line);
                text.push('\n');
            }
            text
        })
        .collect();
    let names: Vec<&str> = book.chapters.iter().map(|c| c.slug.as_str()).collect();
    let text = emit::join_bodies("", &names, &bodies, false);
    let last = pages.unwrap_or(0).min(range.map_or(u32::MAX, |r| r.1));
    let mut text = if written_by_hand { page_pictures(pdf, stem, out, &text, last) } else { text };
    if pictured {
        for n in 1..=last {
            if let Some(name) = page_picture(pdf, stem, out, n) {
                text.push_str(&format!("\n<details>\n<summary>Page {n} as printed</summary>\n\n![Page {n}]({name})\n\n</details>\n"));
            }
        }
    }
    let text = text
        .replace(&format!("\"{}#page=", emit::PDF_NAME), &format!("\"pdf/{stem}.pdf#page="))
        .replace("](images/", &format!("](images/{stem}-"))
        .replace(&format!("](images/{stem}-{stem}-page-"), &format!("](images/{stem}-page-"))
        .replace("src=\"images/", &format!("src=\"images/{stem}-"));
    Ok((text, source))
}

fn is_captions(p: &Path) -> bool {
    p.extension().is_some_and(|x| x.eq_ignore_ascii_case("webvtt") || x.eq_ignore_ascii_case("vtt"))
}

/// What is said in a video, as text: the captions (WebVTT) joined into
/// paragraphs, each headed by its time, which opens the video there when
/// the file names the video (a line "NOTE video <address>"; "NOTE about
/// <words>" is told on top). Noises in brackets are left out; a new
/// speaker, or a sentence's end after a good many words, begins a paragraph.
fn captions_text(vtt: &str) -> String {
    let mut video = String::new();
    let mut about = String::new();
    let mut cues: Vec<(u32, String)> = Vec::new();
    let seconds = |t: &str| -> Option<u32> {
        let t = t.trim().split('.').next()?;
        let parts: Vec<u32> = t.split(':').map(|p| p.parse().ok()).collect::<Option<Vec<u32>>>()?;
        Some(parts.iter().fold(0, |a, p| a * 60 + p))
    };
    let mut at: Option<u32> = None;
    for line in vtt.lines() {
        let line = line.trim();
        if let Some(v) = line.strip_prefix("NOTE video ") {
            video = v.trim().to_string();
        } else if let Some(a) = line.strip_prefix("NOTE about ") {
            about = a.trim().to_string();
        } else if let Some((from, _)) = line.split_once("-->") {
            at = seconds(from);
        } else if line.is_empty() {
            at = None;
        } else if let Some(t) = at {
            // (tags of the captions' own, <v Name>, <i>)
            let mut words = String::new();
            let mut tag = false;
            for c in line.chars() {
                match c {
                    '<' => tag = true,
                    '>' => tag = false,
                    c if !tag => words.push(c),
                    _ => {}
                }
            }
            let words = decode(words.trim());
            if words.is_empty() || (words.starts_with('[') && words.ends_with(']')) {
                continue;
            }
            match cues.last_mut() {
                Some((s, text)) if *s == t => {
                    text.push(' ');
                    text.push_str(&words);
                }
                _ => cues.push((t, words)),
            }
        }
    }
    let clock = |t: u32| if t >= 3600 { format!("{}:{:02}:{:02}", t / 3600, t / 60 % 60, t % 60) } else { format!("{}:{:02}", t / 60, t % 60) };
    let stamp = |t: u32| {
        if video.is_empty() {
            format!("**{}**", clock(t))
        } else {
            let join = if video.contains('?') { '&' } else { '?' };
            format!("**[{}]({video}{join}t={t}s)**", clock(t))
        }
    };
    let speaker = |t: &str| t.split(':').next().is_some_and(|w| t.contains(':') && w.len() >= 3 && w.chars().all(|c| c.is_uppercase() || c == ' '));
    let mut out = String::new();
    if !about.is_empty() {
        out.push_str(&format!("_{}_\n\n", emit::escape_text(&about)));
    }
    if !video.is_empty() {
        out.push_str(&format!("[Watch the video]({video})\n\n"));
    }
    let mut para = String::new();
    let mut began = 0u32;
    for (t, words) in cues {
        let full = para.chars().count() >= 600 && para.trim_end().ends_with(['.', '?', '!']);
        if !para.is_empty() && (full || speaker(&words)) {
            out.push_str(&format!("{} {}\n\n", stamp(began), emit::escape_text(para.trim())));
            para.clear();
        }
        if para.is_empty() {
            began = t;
        }
        para.push_str(&words);
        para.push(' ');
    }
    if !para.trim().is_empty() {
        out.push_str(&format!("{} {}\n\n", stamp(began), emit::escape_text(para.trim())));
    }
    out
}

/// Is the PDF a scan of handwriting with a layer of text that a program
/// made of it? Its pages are pictures, its text is in fonts the PDF does
/// not hold, and few of its "words" are words.
fn handwritten(pdf: &Path) -> bool {
    let run = |cmd: &str, args: &[&str]| std::process::Command::new(cmd).args(args).output().ok().map(|o| String::from_utf8_lossy(&o.stdout).into_owned()).unwrap_or_default();
    let path = pdf.to_string_lossy();
    let fonts = run("pdffonts", &[&path]);
    let rows: Vec<&str> = fonts.lines().skip(2).collect();
    // (name, type, encoding, emb, sub, uni …: "no" in the emb column, read
    // from the right, where no column has spaces)
    let held = |row: &str| row.split_whitespace().rev().nth(4).is_some_and(|emb| emb == "yes");
    if rows.is_empty() || rows.iter().any(|r| held(r)) {
        return false;
    }
    let pages: usize = run("pdfinfo", &[&path]).lines().find_map(|l| l.strip_prefix("Pages:").and_then(|n| n.trim().parse().ok())).unwrap_or(0);
    let with_picture: std::collections::HashSet<u32> = run("pdfimages", &["-list", &path]).lines().skip(2).filter_map(|l| l.split_whitespace().next()?.parse().ok()).collect();
    if pages == 0 || with_picture.len() * 5 < pages * 4 {
        return false;
    }
    let text = run("pdftotext", &[&path, "-"]);
    let tokens: Vec<&str> = text.split_whitespace().collect();
    let words = tokens.iter().filter(|t| t.chars().count() >= 3 && t.trim_end_matches(['.', ',', ';', ':']).chars().all(char::is_alphabetic)).count();
    tokens.is_empty() || words * 2 < tokens.len()
}

/// Are the PDF's pages pictures of print, or its letters drawn one by one
/// (TeX's fonts as bitmaps, without names to tell the math by)?
fn pictures_of_print(pdf: &Path) -> bool {
    let run = |cmd: &str, args: &[&str]| std::process::Command::new(cmd).args(args).output().ok().map(|o| String::from_utf8_lossy(&o.stdout).into_owned()).unwrap_or_default();
    let path = pdf.to_string_lossy();
    let fonts = run("pdffonts", &[&path]);
    let rows: Vec<&str> = fonts.lines().skip(2).collect();
    if rows.is_empty() || rows.iter().all(|r| r.contains(" Type 3 ")) {
        return true;
    }
    let pages: usize = run("pdfinfo", &[&path]).lines().find_map(|l| l.strip_prefix("Pages:").and_then(|n| n.trim().parse().ok())).unwrap_or(1);
    run("pdftotext", &[&path, "-"]).split_whitespace().count() < 10 * pages.max(1)
}

/// The picture of a page, in the book's folder; gives its name there.
fn page_picture(pdf: &Path, stem: &str, out: &Path, n: u32) -> Option<String> {
    let _ = std::fs::create_dir_all(out.join("images"));
    let name = format!("images/{stem}-page-{n:03}");
    // (made beside its place first: a picture that comes out as it was is
    // not written again, see `emit::write_changed`)
    let new = out.join(format!("images/.new-{stem}-page-{n:03}"));
    let made = std::process::Command::new("pdftocairo")
        .args(["-png", "-gray", "-r", "110", "-singlefile", "-f", &n.to_string(), "-l", &n.to_string()])
        .arg(pdf)
        .arg(&new)
        .status()
        .is_ok_and(|st| st.success());
    emit::move_changed(&new.with_extension("png"), &out.join(format!("{name}.png")));
    made.then(|| format!("{name}.png"))
}

/// The text of a handwritten PDF, page by page: each page as its picture,
/// and under it, folded away, what the machine read on it (for searching;
/// handwriting is read with mistakes, its formulas most of all).
fn page_pictures(pdf: &Path, stem: &str, out: &Path, text: &str, pages: u32) -> String {
    // what was read, by the page it is on
    let mut read: HashMap<u32, String> = HashMap::new();
    let mut page = 0u32;
    let number = |t: &str| t.find("#page=").map(|k| t[k + 6..].chars().take_while(char::is_ascii_digit).collect::<String>()).and_then(|n| n.parse::<u32>().ok());
    let mut rest = text;
    let mut bar_next = false;
    while !rest.is_empty() {
        // a page turns in the middle of a paragraph
        if let Some(k) = rest.find("<span class=\"page\"").filter(|k| !rest[..*k].contains('\n')) {
            read.entry(page).or_default().push_str(&rest[..k]);
            let tail = &rest[k..];
            let id = tail.find(" id=\"").unwrap_or(0);
            let end = tail[id..].find("</a></span>").map_or(tail.len(), |e| id + e + "</a></span>".len());
            page = number(&tail[id..end]).unwrap_or(page + 1);
            rest = &tail[end..];
            continue;
        }
        let (line, after) = rest.split_once('\n').unwrap_or((rest, ""));
        rest = after;
        if line.starts_with("<hr class=\"page\"") {
            bar_next = true;
        } else if bar_next && line.starts_with("<a id=\"p") {
            page = number(line).unwrap_or(page + 1);
            bar_next = false;
        } else if line.starts_with("<div align=\"right\"><a href=") && line.contains("#page=") {
            // (the page that ends, named above the bar)
        } else if line.trim_end() == "# Front Matter" {
        } else {
            bar_next = false;
            let t = read.entry(page).or_default();
            t.push_str(line);
            t.push('\n');
        }
    }
    let mut s = String::new();
    let attrs = " target=\"_blank\" title=\"This page in the PDF\"";
    for n in 1..=pages.max(read.keys().copied().max().unwrap_or(0)) {
        let made = page_picture(pdf, stem, out, n);
        s.push_str(&format!("<hr class=\"page\" style=\"height:2px;border:0;background:currentColor;opacity:0.25\">\n<a id=\"pdf-p{n}\" href=\"{}#page={n}\"{attrs}><sup>[PDF p. {n}]</sup></a>\n\n", emit::PDF_NAME));
        if let Some(name) = made {
            s.push_str(&format!("![Page {n}]({name})\n\n"));
        }
        let words = read.get(&n).map(|t| t.trim()).unwrap_or("");
        if !words.is_empty() {
            s.push_str(&format!("<details>\n<summary>Page {n} as a machine read it (handwriting: expect mistakes, in the formulas most of all)</summary>\n\n{words}\n\n</details>\n\n"));
        }
    }
    s
}

/// The words that tell a topic: lowercase, four letters or more, without a
/// plural's s.
fn topic_words(t: &str) -> Vec<String> {
    t.split(|c: char| !c.is_alphanumeric())
        .map(|w| w.to_lowercase())
        .filter(|w| w.chars().count() >= 4)
        .map(|w| w.strip_suffix('s').map_or(w.clone(), str::to_string))
        .collect()
}

/// Which topic each lecture belongs to: the lectures go through the topics
/// in order, and each is put where its words fit best.
fn assign(topics: &[String], lectures: &[String]) -> Vec<usize> {
    let tw: Vec<std::collections::HashSet<String>> = topics.iter().map(|t| topic_words(t).into_iter().collect()).collect();
    // (a word that most topics have tells little)
    let weight = |w: &str| 1.0 / tw.iter().filter(|t| t.contains(w)).count().max(1) as f32;
    let score: Vec<Vec<f32>> = lectures
        .iter()
        .map(|l| {
            let words: std::collections::HashSet<String> = topic_words(l).into_iter().collect();
            tw.iter().map(|t| words.iter().filter(|w| t.contains(*w)).map(|w| weight(w)).sum()).collect()
        })
        .collect();
    let (n, m) = (lectures.len(), topics.len());
    if n == 0 || m == 0 {
        return vec![0; n];
    }
    // best[l][t]: the most that lectures 0..=l can score with lecture l in topic t
    let mut best = vec![vec![f32::MIN; m]; n];
    let mut prev = vec![vec![0usize; m]; n];
    for t in 0..m {
        best[0][t] = score[0][t];
    }
    for l in 1..n {
        let mut top = (f32::MIN, 0usize);
        for t in 0..m {
            if best[l - 1][t] > top.0 {
                top = (best[l - 1][t], t);
            }
            best[l][t] = top.0 + score[l][t];
            prev[l][t] = top.1;
        }
    }
    let mut t = (0..m).max_by(|a, b| best[n - 1][*a].total_cmp(&best[n - 1][*b]).then(b.cmp(a))).unwrap_or(0);
    let mut out = vec![0; n];
    for l in (0..n).rev() {
        out[l] = t;
        t = prev[l][t];
    }
    out
}

struct Section {
    /// the page is a file of the book too (a page of text that links to a
    /// PDF or two): which of the pages without PDFs
    page: Option<usize>,
    title: String,
    /// the page (its opening words are told in the contents); empty: none
    intro: String,
    /// (entry, the file of the book it is, if it is there)
    items: Vec<(Entry, Option<usize>)>,
}

/// Convert the course in `folder` and write the book into `out`.
pub fn write_course(folder: &Path, out: &Path) -> Result<Vec<PathBuf>, String> {
    let read = |p: &Path| std::fs::read_to_string(p).map_err(|e| format!("{}: {e}", p.display()));
    let mut pages: Vec<PathBuf> = Vec::new();
    let mut folders: Vec<PathBuf> = Vec::new();
    for e in std::fs::read_dir(folder).map_err(|e| format!("{}: {e}", folder.display()))?.flatten() {
        let p = e.path();
        if p.is_dir() {
            folders.push(p);
        } else if p.extension().is_some_and(|x| x == "html" || x == "htm") {
            pages.push(p);
        }
    }
    let stem = |p: &Path| p.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
    // the pages in the order of a course's site
    const ORDER: [&str; 12] = ["syllabus", "calendar", "readings", "lecturenotes", "lecturevideos", "lectureslides", "recitations", "assignments", "problemsets", "exams", "projects", "relatedresources"];
    let rank = |p: &PathBuf| ORDER.iter().position(|o| *o == crate::key(&stem(p))).unwrap_or(ORDER.len());
    pages.sort_by(|a, b| rank(a).cmp(&rank(b)).then(stem(a).cmp(&stem(b))));
    folders.sort();
    let topics_page = pages.iter().position(|p| crate::key(&stem(p)) == "topicscovered").map(|k| pages.remove(k));
    let name = folder.file_name().map(|s| s.to_string_lossy().into_owned()).unwrap_or_else(|| "Course".into());
    let title = name.strip_suffix(" Downloads").unwrap_or(&name).trim().to_string();

    std::fs::create_dir_all(out).map_err(|e| format!("{}: {e}", out.display()))?;
    // the reader's notes and fixes from the earlier conversion
    let mut user = crate::userdata::collect(out);
    emit::set_fixed(&user);
    crate::userdata::backup(&user, out);

    // the PDFs of each folder, by the name a page's link has for them
    let mut pdfs: Vec<(PathBuf, String, bool)> = Vec::new(); // (file, key of its name, used)
    for d in &folders {
        if let Ok(files) = std::fs::read_dir(d) {
            let mut here: Vec<PathBuf> = files.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x.eq_ignore_ascii_case("pdf") || is_captions(p))).collect();
            here.sort_by(|a, b| natural(&stem(a), &stem(b)));
            pdfs.extend(here.into_iter().map(|p| {
                let k = crate::key(&stem(&p));
                (p, k, false)
            }));
        }
    }

    let mut files: Vec<File> = vec![File { slug: "00-contents".into(), title: "Contents".into(), body: String::new() }];
    let mut sections: Vec<Section> = Vec::new();
    let mut plain_pages: Vec<(String, String)> = Vec::new(); // (title, html) of pages without PDFs
    let mut todo: Vec<(usize, PathBuf, String)> = Vec::new(); // (file, pdf, label)
    let mut taken: Vec<PathBuf> = Vec::new();
    for p in &pages {
        let html = read(p)?;
        let list = entries(&html);
        let page_title = title_of(&stem(p));
        // (a page that links to none of the PDFs here is a page to read)
        if !list.iter().any(|e| e.name.as_ref().is_some_and(|n| pdfs.iter().any(|(_, k, _)| k == n))) {
            plain_pages.push((page_title, html));
            continue;
        }
        // (a page of text with a PDF or two among it, the readings with one
        // set of notes: a file of the book, its PDFs listed under it)
        let named = list.iter().filter(|e| e.name.is_some()).count();
        let mixed = named <= 3 && squeeze(&markdown(&html, 2, &|_| None)).chars().count() > 1500;
        let page = mixed.then(|| {
            plain_pages.push((page_title.clone(), html.clone()));
            plain_pages.len() - 1
        });
        let intro = if mixed { String::new() } else { html.clone() };
        let mut items = Vec::new();
        for e in list {
            let found = e.name.as_ref().and_then(|n| pdfs.iter_mut().find(|(_, k, used)| !*used && k == n));
            let file = found.map(|f| {
                f.2 = true;
                files.push(File { slug: String::new(), title: e.label.clone(), body: String::new() });
                todo.push((files.len() - 1, f.0.clone(), e.label.clone()));
                files.len() - 1
            });
            items.push((e, file));
        }
        // (the folder of this page's PDFs: the PDFs in it that the page does
        // not list come after those it lists)
        let own = folders.iter().find(|d| crate::key(&stem(d)) == crate::key(&stem(p)));
        if let Some(d) = own {
            taken.push(d.clone());
            for f in pdfs.iter_mut().filter(|f| !f.2 && f.0.parent() == Some(d.as_path())) {
                f.2 = true;
                let label = stem(&f.0);
                files.push(File { slug: String::new(), title: label.clone(), body: String::new() });
                todo.push((files.len() - 1, f.0.clone(), label.clone()));
                items.push((Entry { label, name: Some(f.1.clone()), about: String::new() }, Some(files.len() - 1)));
            }
        }
        sections.push(Section { page, title: page_title, intro, items });
    }
    // folders no page tells of
    for d in folders.iter().filter(|d| !taken.contains(d)) {
        let mut items = Vec::new();
        for f in pdfs.iter_mut().filter(|f| !f.2 && f.0.parent() == Some(d.as_path())) {
            f.2 = true;
            let label = stem(&f.0);
            files.push(File { slug: String::new(), title: label.clone(), body: String::new() });
            todo.push((files.len() - 1, f.0.clone(), label.clone()));
            items.push((Entry { label, name: Some(f.1.clone()), about: String::new() }, Some(files.len() - 1)));
        }
        if !items.is_empty() {
            sections.push(Section { page: None, title: stem(d), intro: String::new(), items });
        }
    }
    // the pages without PDFs come first, after the contents
    let first_pdf = 1;
    for (k, (t, _)) in plain_pages.iter().enumerate() {
        files.insert(first_pdf + k, File { slug: String::new(), title: t.clone(), body: String::new() });
    }
    let shift = plain_pages.len();
    for t in todo.iter_mut() {
        t.0 += shift;
    }
    for s in sections.iter_mut() {
        for i in s.items.iter_mut() {
            i.1 = i.1.map(|f| f + shift);
        }
    }
    let width = if files.len() > 99 { 3 } else { 2 };
    for (k, f) in files.iter_mut().enumerate().skip(1) {
        f.slug = format!("{k:0width$}-{}", slug_words(&f.title));
    }

    // where a page's link goes in the book: to the file of its PDF
    let by_name: HashMap<String, String> = sections.iter().flat_map(|s| s.items.iter()).filter_map(|(e, f)| Some((e.name.clone()?, format!("{}.md", files[(*f)?].slug)))).collect();
    let link = |href: &str| -> Option<String> { link_name(href).and_then(|n| by_name.get(&n).cloned()) };
    for (k, (_, html)) in plain_pages.iter().enumerate() {
        files[first_pdf + k].body = markdown(html, 2, &link);
    }

    // the PDFs
    let _ = std::fs::create_dir_all(out.join("pdf"));
    let mut warnings: Vec<String> = Vec::new();
    let mut source: Option<String> = None;
    let total = todo.len();
    for (n, (k, pdf, _)) in todo.iter().enumerate() {
        crate::progress::report(crate::progress::Step::Book, n, total);
        let name = stem(pdf);
        // (what is said in a video, from the file of its captions)
        if is_captions(pdf) {
            files[*k].body = match std::fs::read_to_string(pdf) {
                Ok(t) => captions_text(&t),
                Err(e) => format!("The captions could not be read: {e}\n"),
            };
            continue;
        }
        match pdf_text(pdf, &name, out) {
            Ok((text, from)) => {
                files[*k].body = text;
                if source.is_none() {
                    source = from;
                }
            }
            Err(e) => {
                warnings.push(format!("{}: {e}", pdf.display()));
                files[*k].body = format!("This PDF could not be converted: [{name}.pdf](pdf/{name}.pdf)\n");
            }
        }
        // the PDF beside the book, for the page markers to open
        let at = out.join("pdf").join(format!("{name}.pdf"));
        let linked = std::fs::symlink_metadata(&at).is_ok_and(|m| m.file_type().is_symlink());
        // (a link that goes there already stays as it is)
        let right = linked && matches!((pdf.canonicalize(), out.join("pdf").canonicalize()), (Ok(p), Ok(d)) if std::fs::read_link(&at).is_ok_and(|to| to == emit::relative(&d, &p)));
        if linked && !right {
            let _ = std::fs::remove_file(&at);
        }
        if let (false, Ok(p), Ok(d)) = (right, pdf.canonicalize(), out.join("pdf").canonicalize()) {
            #[cfg(unix)]
            let _ = std::os::unix::fs::symlink(emit::relative(&d, &p), &at);
            #[cfg(not(unix))]
            let _ = std::fs::copy(&p, &at);
        }
    }
    crate::progress::report(crate::progress::Step::Book, total, total);

    // the contents
    let mut c = String::new();
    for (k, f) in files.iter().skip(1).take(plain_pages.len()).enumerate() {
        c.push_str(&format!("- [{}]({}.md)\n", f.title, f.slug));
        for s in sections.iter().filter(|s| s.page == Some(k)) {
            for (e, file) in &s.items {
                if let Some(file) = file {
                    c.push_str(&format!("   - [{}]({}.md)\n", emit::escape_text(&e.label), files[*file].slug));
                }
            }
        }
    }
    if !plain_pages.is_empty() {
        c.push('\n');
    }
    // what a page says before its list, as Markdown
    let intro_of = |s: &Section| -> String {
        let md = markdown(&s.intro, 3, &link);
        let lines: Vec<&str> = md.lines().take_while(|l| !l.starts_with('|') && !l.contains(".md)") && !l.to_lowercase().contains("not available")).collect();
        lines.join("\n").trim().to_string()
    };
    let entry_line = |e: &Entry, f: Option<usize>, about: bool| -> String {
        let name = match f {
            Some(k) => format!("[{}]({}.md)", emit::escape_text(&e.label), files[k].slug),
            None => emit::escape_text(&e.label),
        };
        if about && !e.about.is_empty() {
            format!("{name} — {}", emit::escape_text(&e.about))
        } else {
            name
        }
    };
    let topics = match &topics_page {
        Some(p) => outline(&read(p)?),
        None => Vec::new(),
    };
    // the section whose page tells each PDF's topics goes under the outline
    let told = sections.iter().position(|s| s.items.iter().filter(|(e, _)| !e.about.is_empty()).count() * 2 > s.items.len());
    let under_topics = told.filter(|_| !topics.is_empty());
    if !topics.is_empty() {
        let tops: Vec<usize> = (0..topics.len()).filter(|&k| topics[k].0 == 1).collect();
        let whole: Vec<String> = tops
            .iter()
            .enumerate()
            .map(|(n, &k)| {
                let end = tops.get(n + 1).copied().unwrap_or(topics.len());
                topics[k..end].iter().map(|t| t.1.as_str()).collect::<Vec<_>>().join(" ")
            })
            .collect();
        let place: Vec<usize> = match under_topics {
            Some(s) => {
                let texts: Vec<String> = sections[s]
                    .items
                    .iter()
                    .map(|(e, f)| {
                        let heads: Vec<&str> = f.map_or(Vec::new(), |k| files[k].body.lines().filter(|l| l.starts_with('#')).collect());
                        format!("{} {}", e.about, heads.join(" "))
                    })
                    .collect();
                assign(&whole, &texts)
            }
            None => Vec::new(),
        };
        c.push_str("## Topics Covered\n\n");
        if let Some(s) = under_topics {
            let intro = intro_of(&sections[s]);
            if !intro.is_empty() {
                c.push_str(&format!("{intro}\n\n"));
            }
        }
        let mut count: Vec<usize> = Vec::new();
        for (n, &k) in tops.iter().enumerate() {
            let end = tops.get(n + 1).copied().unwrap_or(topics.len());
            for (depth, text) in &topics[k..end] {
                count.truncate(*depth);
                while count.len() < *depth {
                    count.push(0);
                }
                count[*depth - 1] += 1;
                let words = emit::escape_text(text);
                let words = if *depth == 1 { format!("**{words}**") } else { words };
                c.push_str(&format!("{}{}. {words}\n", "   ".repeat(depth - 1), count[*depth - 1]));
            }
            if let Some(s) = under_topics {
                let mine: Vec<&(Entry, Option<usize>)> = sections[s].items.iter().zip(&place).filter(|(_, t)| **t == n).map(|(i, _)| i).collect();
                if !mine.is_empty() {
                    c.push_str(&format!("\n   _{}_\n\n", sections[s].title));
                    for (e, f) in mine {
                        c.push_str(&format!("   - {}\n", entry_line(e, *f, true)));
                    }
                    c.push('\n');
                }
            }
        }
        c.push('\n');
    }
    for (k, s) in sections.iter().enumerate() {
        if under_topics == Some(k) || s.page.is_some() {
            continue;
        }
        c.push_str(&format!("## {}\n\n", s.title));
        let intro = intro_of(s);
        if !intro.is_empty() {
            c.push_str(&format!("{intro}\n\n"));
        }
        for (e, f) in &s.items {
            c.push_str(&format!("- {}\n", entry_line(e, *f, true)));
        }
        c.push('\n');
    }
    if let Some(from) = &source {
        // (where the PDFs are from, as each says on its last page)
        c.push_str(&format!("---\n\n{}\n", emit::escape_text(from)));
    }
    files[0].body = c;

    // write: each file with the way to its neighbours and to the contents
    let mut written = Vec::new();
    let mut texts: Vec<String> = Vec::new();
    for (k, f) in files.iter().enumerate() {
        let head = if k == 0 { format!("# {title}\n\n") } else { format!("# {}\n\n", emit::escape_text(&f.title)) };
        texts.push(format!("{head}{}", f.body));
    }
    let pairs: Vec<(&str, &str)> = files.iter().zip(&texts).map(|(f, t)| (f.slug.as_str(), t.as_str())).collect();
    crate::userdata::follow_renames(&mut user, &pairs);
    for (k, f) in files.iter().enumerate() {
        let mut nav = Vec::new();
        if k > 0 {
            nav.push(format!("[← {}]({}.md)", files[k - 1].title, files[k - 1].slug));
        }
        if k > 1 {
            nav.push(format!("[Contents]({}.md)", files[0].slug));
        }
        if k + 1 < files.len() {
            nav.push(format!("[{} →]({}.md)", files[k + 1].title, files[k + 1].slug));
        }
        let nav = nav.join(" · ");
        let mut s = format!("{nav}\n\n---\n\n{}\n---\n\n{nav}\n", texts[k]);
        let lost = crate::userdata::apply(&mut s, &user, Some(&f.slug));
        s.push_str(&emit::lost_notes(&lost));
        let p = out.join(format!("{}.md", f.slug));
        emit::write_changed(&p, s).map_err(|e| format!("{}: {e}", p.display()))?;
        written.push(p);
    }
    // the whole course in one file, to search all of it at once
    let names: Vec<&str> = files.iter().map(|f| f.slug.as_str()).collect();
    let mut all = emit::join_bodies("", &names, &texts, true);
    let lost = crate::userdata::apply(&mut all, &user, None);
    all.push_str(&emit::lost_notes(&lost));
    let p = out.join("book.md");
    // (the school, where the course's name opens with it: "MIT …")
    let school = title.split_whitespace().next().filter(|w| w.len() >= 2 && w.chars().all(char::is_uppercase)).unwrap_or("");
    let all = emit::by_line(out, school) + &all;
    emit::write_changed(&p, all).map_err(|e| format!("{}: {e}", p.display()))?;
    written.push(p);
    // The files written last time that are not written now (a file's
    // name changed with its title or its place) are taken away: the list
    // of what was written is kept in the folder.
    let list = out.join(".pdf2md-course");
    let now: Vec<String> = written.iter().filter_map(|p| p.file_name().map(|n| n.to_string_lossy().into_owned())).collect();
    if let Ok(before) = std::fs::read_to_string(&list) {
        for old in before.lines().filter(|l| l.ends_with(".md") && !l.contains('/') && !now.iter().any(|n| n == l)) {
            let _ = std::fs::remove_file(out.join(old));
        }
    }
    let _ = emit::write_changed(&list, now.join("\n") + "\n");
    for w in warnings {
        eprintln!("warning: {w}");
    }
    Ok(written)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_page_as_markdown() {
        let html = "<main><h3 id=\"a\">Topics</h3><ol><li>One<ol><li>Kets &amp; bras</li></ol></li><li>Two</li></ol><p>See <a href=\"https://example.org/x\">this</a> and <a href=\"/courses/c/resources/lec1/\">Lecture 1 (PDF)</a>.</p></main>";
        let md = markdown(html, 2, &|h| link_name(h).filter(|n| n == "lec1").map(|_| "01-lecture-1.md".to_string()));
        assert_eq!(md, "## Topics\n\n1. One\n   1. Kets & bras\n2. Two\n\nSee [this](https://example.org/x) and [Lecture 1 (PDF)](01-lecture-1.md).\n\n");
    }

    #[test]
    fn what_a_page_lists() {
        let html = "<main><p>Notes by the TA.</p><table><tr><th>LEC #</th><th>NOTES</th><th>TOPICS</th></tr><tr><td>1</td><td><a href=\"/c/resources/mit_lec1/\">Lecture 1 Notes (PDF)</a></td><td>Hilbert Spaces</td></tr><tr><td>2</td><td>[Lecture 2 Notes not available]</td><td></td></tr></table></main>";
        let list = entries(html);
        assert_eq!(list.len(), 2);
        assert_eq!((list[0].label.as_str(), list[0].name.as_deref(), list[0].about.as_str()), ("Lecture 1 Notes", Some("mitlec1"), "Hilbert Spaces"));
        assert_eq!((list[1].label.as_str(), list[1].name.as_deref()), ("Lecture 2 Notes not available", None));
    }

    #[test]
    fn the_outline_of_the_topics() {
        let html = "<main><h3>Topics Covered</h3><ol><li>A<ol><li>a1</li><li>a2<ol><li>deep</li></ol></li></ol></li><li>B</li></ol><ol><li>other</li></ol></main>";
        assert_eq!(outline(html), vec![(1, "A".to_string()), (2, "a1".into()), (2, "a2".into()), (3, "deep".into()), (1, "B".into())]);
    }

    #[test]
    fn lectures_go_through_the_topics_in_order() {
        let topics = vec!["Fundamental concepts kets operators".to_string(), "Quantum dynamics path integral".into(), "Symmetries angular momentum".into()];
        let lectures = vec!["Operators, kets".to_string(), "Path integrals".into(), "".into(), "Angular momentum".into()];
        let place = assign(&topics, &lectures);
        assert_eq!((place[0], place[1], place[3]), (0, 1, 2));
        assert!(place[2] == 1 || place[2] == 2);
    }

    #[test]
    fn captions_as_paragraphs() {
        let vtt = "WEBVTT\n\nNOTE video https://example.org/watch?v=abc\n\n00:00:01.000 --> 00:00:03.000 align:middle\n[SQUEAKING]\n\n00:00:10.560 --> 00:00:15.600\nPROFESSOR: Yeah, so I'm Hong,\nso you can just call me Hong.\n\n00:01:05.000 --> 00:01:08.000\nAUDIENCE: A question?\n";
        let text = captions_text(vtt);
        assert_eq!(text, "[Watch the video](https://example.org/watch?v=abc)\n\n**[0:10](https://example.org/watch?v=abc&t=10s)** PROFESSOR: Yeah, so I'm Hong, so you can just call me Hong.\n\n**[1:05](https://example.org/watch?v=abc&t=65s)** AUDIENCE: A question?\n\n");
    }

    #[test]
    fn files_in_their_natural_order() {
        let mut v = vec!["lec10", "lec2", "Lec1"];
        v.sort_by(|a, b| natural(a, b));
        assert_eq!(v, vec!["Lec1", "lec2", "lec10"]);
    }
}
