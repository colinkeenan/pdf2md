//! What the reader adds to a converted book, kept when the book is
//! converted again.
//!
//! * **Notes**: `<details class="mine"> … </details>` blocks and alert
//!   boxes (`> [!NOTE]`, `> [!WARNING]` …); pdf2md never writes either.
//!   Each is put back right after the block it followed (found by that
//!   block's first words), else after the nearest anchor above it, else at
//!   the end of its chapter.
//! * **Prose fixes** (the conversion got the book's words wrong):
//!   `<span class="fix" data-was="as converted">as in the book</span>`;
//!   redone in the same paragraph.
//!
//! (Equation corrections are kept per picture by [`crate::ocr`].)

use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    /// chapter file it was in (without .md), if known
    pub slug: Option<String>,
    /// first words of the block before it
    pub after: String,
    /// nearest anchor id above it
    pub anchor: Option<String>,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fix {
    pub slug: Option<String>,
    /// first words of its paragraph as converted (with the fix undone)
    pub paragraph: String,
    /// the converted text it replaces, and the whole span
    pub was: String,
    pub span: String,
}

/// A marker the reader put inside a sentence (`<sup>[✎](#mine-3)</sup>`,
/// or any `<sup class="mine">…</sup>`), pointing to a note.
#[derive(Debug, Clone, PartialEq)]
pub struct Mark {
    pub slug: Option<String>,
    /// first words of its paragraph
    pub paragraph: String,
    /// the text right before it in the paragraph
    pub before: String,
    pub mark: String,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UserData {
    pub notes: Vec<Note>,
    pub fixes: Vec<Fix>,
    pub marks: Vec<Mark>,
}

impl UserData {
    pub fn is_empty(&self) -> bool {
        self.notes.is_empty() && self.fixes.is_empty() && self.marks.is_empty()
    }
}

/// Byte ranges of the reader's markers in a text.
fn mark_ranges(text: &str) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    for open in ["<sup>[✎](", "<sup class=\"mine\">"] {
        let mut from = 0;
        while let Some(i) = text[from..].find(open).map(|i| from + i) {
            let Some(end) = text[i..].find("</sup>").map(|e| i + e + 6) else { break };
            out.push((i, end));
            from = end;
        }
    }
    out.sort();
    out
}

fn without_marks(t: &str) -> String {
    let mut out = String::new();
    let mut last = 0;
    for (a, b) in mark_ranges(t) {
        out.push_str(&t[last..a]);
        last = b;
    }
    out.push_str(&t[last..]);
    out
}

/// The first words of a block of Markdown, without markup: what finds the
/// block again in a new conversion.
pub fn fingerprint(block: &str) -> String {
    let mut t = String::new();
    let mut rest = block;
    // drop tags, comments, link targets
    while let Some(i) = rest.find(['<', ']']) {
        t.push_str(&rest[..i]);
        let c = rest.as_bytes()[i];
        rest = &rest[i + 1..];
        if c == b'<' {
            rest = rest.find('>').map_or("", |j| &rest[j + 1..]);
        } else if rest.starts_with('(') {
            rest = rest.find(')').map_or("", |j| &rest[j + 1..]);
        }
    }
    t.push_str(rest);
    let words: Vec<String> = t
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .take(16)
        .map(str::to_lowercase)
        .collect();
    words.join(" ")
}

fn unescape(t: &str) -> String {
    t.replace("&quot;", "\"").replace("&lt;", "<").replace("&gt;", ">").replace("&#39;", "'").replace("&amp;", "&")
}

/// Byte ranges of the reader's notes in a file: <details class="mine">
/// blocks (nested <details> counted) and alert boxes.
fn note_ranges(text: &str) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let mut from = 0;
    while let Some(i) = text[from..].find("<details class=\"mine\"").map(|i| from + i) {
        let mut depth = 0;
        let mut k = i;
        let mut end = text.len();
        while k < text.len() {
            let open = text[k..].find("<details").map(|x| k + x);
            let close = text[k..].find("</details>").map(|x| k + x);
            match (open, close) {
                (Some(o), Some(c)) if o < c => {
                    depth += 1;
                    k = o + 8;
                }
                (_, Some(c)) => {
                    depth -= 1;
                    k = c + 10;
                    if depth == 0 {
                        end = k;
                        break;
                    }
                }
                _ => break,
            }
        }
        out.push((i, end));
        from = end;
    }
    // alert boxes: a run of "> " lines starting with "> [!"
    let mut pos = 0;
    let mut lines = text.split_inclusive('\n').peekable();
    while let Some(line) = lines.next() {
        let start = pos;
        pos += line.len();
        if line.starts_with("> [!") && !out.iter().any(|&(a, b)| start >= a && start < b) {
            while let Some(next) = lines.peek() {
                if next.starts_with('>') {
                    pos += next.len();
                    lines.next();
                } else {
                    break;
                }
            }
            out.push((start, pos));
        }
    }
    out.sort();
    out
}

/// The reader's notes and prose fixes in one Markdown file. `slug_at`
/// names the chapter at a byte position (book.md holds them all).
fn collect_file(text: &str, slug_at: &dyn Fn(usize) -> Option<String>) -> UserData {
    let mut data = UserData::default();
    let notes = note_ranges(text);
    let inside = |p: usize| notes.iter().any(|&(a, b)| p >= a && p < b);
    for &(a, b) in &notes {
        // the block before it (skipping other notes)
        let mut end = a;
        let mut after = String::new();
        while end > 0 {
            let before = text[..end].trim_end();
            let start = before.rfind("\n\n").map_or(0, |i| i + 2);
            if inside(start) {
                end = start;
                continue;
            }
            // (as converted: fixes undone, so it is found with or without them)
            after = fingerprint(&undo_fixes(&before[start..]));
            if !after.is_empty() {
                break;
            }
            end = start;
        }
        let anchor = text[..a]
            .match_indices("id=\"")
            .filter(|(i, _)| !inside(*i))
            .last()
            .and_then(|(i, _)| text[i + 4..].split('"').next().map(str::to_string));
        data.notes.push(Note { slug: slug_at(a), after, anchor, text: text[a..b].trim_end().to_string() });
    }
    // prose fixes
    let mut from = 0;
    while let Some(i) = text[from..].find("<span class=\"fix\" data-was=\"").map(|i| from + i) {
        let Some(close) = text[i..].find("</span>").map(|c| i + c + 7) else { break };
        from = close;
        if inside(i) {
            continue;
        }
        let span = &text[i..close];
        let was = unescape(span[28..].split('"').next().unwrap_or(""));
        // its paragraph, as converted (all fixes undone)
        let p0 = text[..i].rfind("\n\n").map_or(0, |k| k + 2);
        let p1 = text[i..].find("\n\n").map_or(text.len(), |k| i + k);
        let para = undo_fixes(&text[p0..p1]);
        data.fixes.push(Fix { slug: slug_at(i), paragraph: fingerprint(&para), was, span: span.to_string() });
    }
    // markers inside sentences
    for (a, b) in mark_ranges(text) {
        if inside(a) {
            continue;
        }
        let p0 = text[..a].rfind("\n\n").map_or(0, |k| k + 2);
        let p1 = text[b..].find("\n\n").map_or(text.len(), |k| b + k);
        // the text right before it (other markers left out), up to 40 bytes
        let before = without_marks(&text[p0..a]);
        let mut cut = before.len().saturating_sub(40);
        while !before.is_char_boundary(cut) {
            cut += 1;
        }
        data.marks.push(Mark {
            slug: slug_at(a),
            paragraph: fingerprint(&undo_fixes(&text[p0..p1])),
            before: before[cut..].to_string(),
            mark: text[a..b].to_string(),
        });
    }
    data
}

fn undo_fixes(t: &str) -> String {
    let mut out = String::new();
    let mut rest = t;
    while let Some(i) = rest.find("<span class=\"fix\" data-was=\"") {
        out.push_str(&rest[..i]);
        let span = &rest[i..];
        let was = unescape(span[28..].split('"').next().unwrap_or(""));
        out.push_str(&was);
        rest = span.find("</span>").map_or("", |c| &span[c + 7..]);
    }
    out.push_str(rest);
    out
}

/// Notes and fixes in a converted book's folder (chapter files and
/// book.md; the same note found in both counts once).
pub fn collect(dir: &Path) -> UserData {
    let mut all = UserData::default();
    let Ok(entries) = std::fs::read_dir(dir) else { return all };
    let mut files: Vec<_> = entries.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "md")).collect();
    // chapter files first, book.md last: its notes count only if new
    files.sort_by_key(|p| p.file_name().is_some_and(|n| n == "book.md"));
    for p in files {
        let Ok(text) = std::fs::read_to_string(&p) else { continue };
        let name = p.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
        let data = if name == "book" {
            // chapters start with <a id="slug"></a>; their title follows, under
            // the bar and marker of the page they open on
            let starts: Vec<(usize, String)> = text
                .match_indices("<a id=\"")
                .filter_map(|(i, _)| {
                    let id = text[i + 7..].split('"').next()?;
                    let line_end = text[i..].find('\n').map_or(text.len(), |k| i + k);
                    (text[i..line_end].trim() == format!("<a id=\"{id}\"></a>") && text[line_end..].lines().filter(|l| !l.trim().is_empty()).take(8).any(|l| l.starts_with("# ")) && id.chars().next().is_some_and(|c| c.is_ascii_digit()))
                        .then(|| (i, id.to_string()))
                })
                .collect();
            collect_file(&text, &|pos| starts.iter().rev().find(|(i, _)| *i <= pos).map(|(_, s)| s.clone()))
        } else if name == "check-equations" {
            continue;
        } else {
            collect_file(&text, &|_| Some(name.clone()))
        };
        for n in data.notes {
            if !all.notes.iter().any(|m| m.text == n.text) {
                all.notes.push(n);
            }
        }
        for f in data.fixes {
            if !all.fixes.iter().any(|g| g.span == f.span && g.paragraph == f.paragraph) {
                all.fixes.push(f);
            }
        }
        for m in data.marks {
            if !all.marks.iter().any(|n| n.mark == m.mark && n.paragraph == m.paragraph) {
                all.marks.push(m);
            }
        }
    }
    all
}

/// The chapter files are named differently than when the notes were made
/// (the number in front changed, or prefaces became part of the front
/// matter): each note, fix and marker moves to the file that now has its
/// chapter's name, or else to the file that holds its paragraph. `files`:
/// name (without .md) and text of each chapter file as it is written now.
pub fn follow_renames(data: &mut UserData, files: &[(&str, &str)]) {
    let name = |slug: &str| slug.split_once('-').map_or(slug, |(_, n)| n).to_string();
    let prints: Vec<std::collections::HashSet<String>> =
        files.iter().map(|(_, text)| text.split("\n\n").map(fingerprint).filter(|f| !f.is_empty()).collect()).collect();
    let moved = |slug: &mut Option<String>, paragraph: &str| {
        let Some(old) = slug.as_deref() else { return };
        if files.iter().any(|(s, _)| *s == old) {
            return;
        }
        let by_name = files.iter().find(|(s, _)| name(s) == name(old)).map(|(s, _)| s.to_string());
        let by_text = || prints.iter().position(|p| !paragraph.is_empty() && p.contains(paragraph)).map(|i| files[i].0.to_string());
        if let Some(new) = by_name.or_else(by_text) {
            *slug = Some(new);
        }
    };
    for n in data.notes.iter_mut() {
        moved(&mut n.slug, &n.after);
    }
    for f in data.fixes.iter_mut() {
        moved(&mut f.slug, &f.paragraph);
    }
    for m in data.marks.iter_mut() {
        moved(&mut m.slug, &m.paragraph);
    }
}

/// Put notes and fixes into a newly written Markdown file (`slug`: the
/// chapter file it is; None for book.md). Returns the notes that found no
/// place (for the end of their chapter).
pub fn apply(text: &mut String, data: &UserData, slug: Option<&str>) -> Vec<Note> {
    let ours = |s: &Option<String>| slug.is_none() || s.as_deref() == slug;
    // fixes: in the paragraph with the same first words
    for f in data.fixes.iter().filter(|f| ours(&f.slug)) {
        let mut pos = 0;
        for block in text.clone().split("\n\n") {
            if fingerprint(block) == f.paragraph {
                if let Some(k) = block.find(&f.was) {
                    let at = pos + k;
                    text.replace_range(at..at + f.was.len(), &f.span);
                }
                break;
            }
            pos += block.len() + 2;
        }
    }
    // markers: in their paragraph after the same text, else at its end
    let mut lost = Vec::new();
    for m in data.marks.iter().filter(|m| ours(&m.slug)) {
        let mut pos = 0;
        let mut placed = false;
        for block in text.clone().split("\n\n") {
            if fingerprint(&undo_fixes(block)) == m.paragraph {
                let at = if m.before.is_empty() {
                    None
                } else {
                    // find the context, skipping markers already put back
                    let plain = without_marks(block);
                    plain.find(&m.before).map(|k| {
                        let end_plain = k + m.before.len();
                        // map the position back into the block with markers
                        let mut seen = 0;
                        let mut last = 0;
                        let mut real = end_plain;
                        for (a, b) in mark_ranges(block) {
                            if seen + (a - last) >= end_plain {
                                break;
                            }
                            seen += a - last;
                            real += b - a;
                            last = b;
                        }
                        real
                    })
                };
                let at = pos + at.unwrap_or(block.len());
                text.insert_str(at, &m.mark);
                placed = true;
                break;
            }
            pos += block.len() + 2;
        }
        if !placed {
            lost.push(Note { slug: m.slug.clone(), after: m.paragraph.clone(), anchor: None, text: format!("A marker {} that followed “{}”", m.mark, m.before.trim()) });
        }
    }
    // notes: after their block, else after the block holding their anchor
    let mut inserts: Vec<(usize, &Note)> = Vec::new();
    for n in data.notes.iter().filter(|n| ours(&n.slug)) {
        let mut pos = 0;
        let mut at = None;
        for block in text.split("\n\n") {
            if !n.after.is_empty() && fingerprint(&undo_fixes(block)) == n.after {
                // after the block's text (before a final newline of the file)
                at = Some(pos + block.trim_end_matches('\n').len());
                break;
            }
            pos += block.len() + 2;
        }
        if at.is_none() {
            if let Some(a) = &n.anchor {
                if let Some(i) = text.find(&format!("id=\"{a}\"")) {
                    at = Some(text[i..].find("\n\n").map_or(text.len(), |k| i + k));
                }
            }
        }
        match at {
            Some(p) => inserts.push((p, n)),
            None => lost.push(n.clone()),
        }
    }
    // from the end, so earlier places stay valid; notes at one place keep
    // their order
    inserts.sort_by_key(|(p, _)| std::cmp::Reverse(*p));
    let mut i = 0;
    while i < inserts.len() {
        let p = inserts[i].0;
        let mut same: Vec<&Note> = inserts[i..].iter().take_while(|(q, _)| *q == p).map(|(_, n)| *n).collect();
        i += same.len();
        same.sort_by_key(|n| data.notes.iter().position(|m| m == *n));
        let block: String = same.iter().map(|n| format!("\n\n{}", n.text)).collect();
        text.insert_str(p, &block);
    }
    lost
}

/// Keep a copy of the notes and fixes outside the book folder:
/// ~/.local/share/pdf2md/notes/<book folder name>.md (overwritten each time).
pub fn backup(data: &UserData, book_dir: &Path) {
    if data.is_empty() {
        return;
    }
    let Some(base) = std::env::var_os("XDG_DATA_HOME")
        .map(std::path::PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| std::path::PathBuf::from(h).join(".local/share")))
    else {
        return;
    };
    let dir = base.join("pdf2md").join("notes");
    if std::fs::create_dir_all(&dir).is_err() {
        return;
    }
    let name = book_dir.file_name().map(|s| s.to_string_lossy().into_owned()).unwrap_or_else(|| "book".into());
    let mut s = format!("# Notes and fixes in {name}\n\n");
    for n in &data.notes {
        s.push_str(&format!("<!-- in {} after: {} -->\n\n{}\n\n", n.slug.as_deref().unwrap_or("?"), n.after, n.text));
    }
    for f in &data.fixes {
        s.push_str(&format!("<!-- fix in {} -->\n{}\n\n", f.slug.as_deref().unwrap_or("?"), f.span));
    }
    let _ = std::fs::write(dir.join(format!("{name}.md")), s);
}

#[cfg(test)]
mod tests {
    use super::*;

    const OLD: &str = "# Chapter\n\nFirst paragraph about spin.\n\n<a id=\"eq-1.1\"></a>\n\n$$\nx=1 \\tag{1.1}\n$$\n\n<details class=\"mine\">\n<summary>✎ My note</summary>\n\nWhy *x* is 1.\n\n</details>\n\nSecond paragraph with a <span class=\"fix\" data-was=\"rnistake\">mistake</span> in it.\n\n> [!WARNING]\n> I think this is wrong.\n\nThird paragraph.\n";

    #[test]
    fn notes_follow_renamed_files() {
        let mut d = collect_file(OLD, &|_| Some("06-fundamental-concepts".into()));
        let mut e = collect_file(OLD, &|_| Some("02-preface".into()));
        let new = "# Chapter\n\nFirst paragraph about spin.\n\nSecond paragraph with a rnistake in it.\n";
        let files = [("00-front-matter", new), ("01-fundamental-concepts", "# Other\n\nNothing alike.\n")];
        // the chapter's new number
        follow_renames(&mut d, &files);
        assert!(d.notes.iter().all(|n| n.slug.as_deref() == Some("01-fundamental-concepts")));
        assert_eq!(d.fixes[0].slug.as_deref(), Some("01-fundamental-concepts"));
        // a preface that is part of the front matter now: by its paragraph
        follow_renames(&mut e, &files);
        assert_eq!(e.notes[1].slug.as_deref(), Some("00-front-matter"));
        assert_eq!(e.fixes[0].slug.as_deref(), Some("00-front-matter"));
    }

    #[test]
    fn finds_notes_and_fixes() {
        let d = collect_file(OLD, &|_| Some("01-ch".into()));
        assert_eq!(d.notes.len(), 2);
        assert_eq!(d.notes[0].anchor.as_deref(), Some("eq-1.1"));
        assert!(d.notes[0].text.starts_with("<details") && d.notes[0].text.ends_with("</details>"));
        assert_eq!(d.notes[1].after, "second paragraph with a rnistake in it");
        assert_eq!(d.fixes.len(), 1);
        assert_eq!(d.fixes[0].was, "rnistake");
    }

    #[test]
    fn keeps_markers_in_sentences() {
        let old = "# Ch\n\nThe spin is one half,<sup>[✎](#mine-1)</sup> as Stern found.\n\n<details class=\"mine\" id=\"mine-1\">\n<summary>✎ note</summary>\n\nWhy.\n\n</details>\n";
        let d = collect_file(old, &|_| Some("c".into()));
        assert_eq!(d.marks.len(), 1);
        let mut new = "# Ch\n\nThe spin is one half, as Stern found.\n".to_string();
        let lost = apply(&mut new, &d, Some("c"));
        assert!(lost.is_empty(), "{lost:?}");
        assert_eq!(new, old.trim_end().to_string() + "\n");
        // context gone: at the end of its paragraph
        let mut changed = "# Ch\n\nThe spin is one half, as Stern found.\n".replace("one half,", "1/2,");
        let d2 = UserData { marks: vec![Mark { paragraph: fingerprint("The spin is 1/2, as Stern found."), ..d.marks[0].clone() }], ..Default::default() };
        apply(&mut changed, &d2, Some("c"));
        assert!(changed.contains("as Stern found.\n<sup>") || changed.contains("as Stern found.<sup>"), "{changed}");
    }

    #[test]
    fn puts_them_back() {
        let d = collect_file(OLD, &|_| Some("01-ch".into()));
        // a fresh conversion: no notes, the mistake back, a new paragraph before
        let mut new = "# Chapter\n\nA new first paragraph.\n\nFirst paragraph about spin.\n\n<a id=\"eq-1.1\"></a>\n\n$$\nx=1 \\tag{1.1}\n$$\n\nSecond paragraph with a rnistake in it.\n\nThird paragraph.\n".to_string();
        let lost = apply(&mut new, &d, Some("01-ch"));
        assert!(lost.is_empty());
        let want = OLD.replace("# Chapter\n\n", "# Chapter\n\nA new first paragraph.\n\n");
        assert_eq!(new, want);
        // applying again to the result changes nothing
        let d2 = collect_file(&new, &|_| Some("01-ch".into()));
        assert_eq!(d2, d);
    }
}
