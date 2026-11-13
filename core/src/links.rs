//! Link the table of contents and the index.
//!
//! * Each TOC entry names a printed page, so a section entry is matched to a
//!   heading *on that page* (or the page before/after) with the same words.
//!   Matched headings get the TOC's nesting level (## or ###) and an anchor.
//!   If the section title is not a separate heading line in the text (a
//!   "run-in" heading), an anchor is put at the start of the paragraph on
//!   that page that begins with the title.
//! * Index page numbers link to the place on that page where the entry's
//!   term appears (an anchor `#ix-7` put in front of the word), or to the
//!   page start (`#p32`) if the term is not found in the text.

use crate::emit::render_inlines;
use crate::structure::slugify;
use crate::{key, page_label, plain, Block, Chapter, Inline};
use std::collections::{HashMap, HashSet};

/// The body heading may be a shortened form of the TOC entry or vice versa.
fn fuzzy_eq(a: &str, b: &str) -> bool {
    let (s, l) = if a.len() <= b.len() { (a, b) } else { (b, a) };
    s.len() >= 8 && l.starts_with(s) && s.len() * 10 >= l.len() * 6
}

/// How much two titles share their words (0 = nothing, 1 = same words).
/// "Positron States" vs "Positron States – charge conjugation" = 0.67.
fn similarity(a: &str, b: &str) -> f32 {
    let words = |s: &str| -> HashSet<String> {
        s.split(|c: char| !c.is_alphanumeric())
            .filter(|w| w.chars().count() >= 2)
            .map(|w| w.to_lowercase())
            .collect()
    };
    let (wa, wb) = (words(a), words(b));
    if wa.is_empty() || wb.is_empty() {
        return 0.0;
    }
    2.0 * wa.intersection(&wb).count() as f32 / (wa.len() + wb.len()) as f32
}

/// Is page `page` (an index into the book's pages) the one a TOC entry
/// names: near its printed number, or exactly the PDF page "@N" it links to?
fn page_near(labels: &[Option<String>], pdf: &[u32], page: usize, target: &str) -> bool {
    if let Some(n) = target.strip_prefix('@') {
        return n.parse::<u32>().ok().is_some_and(|n| pdf.get(page) == Some(&n));
    }
    let Some(label) = labels.get(page).and_then(|l| l.as_ref()) else { return false };
    match (label.parse::<i64>(), target.parse::<i64>()) {
        (Ok(a), Ok(b)) => (a - b).abs() <= 1,
        _ => label == target,
    }
}

fn unique_id(used: &mut HashSet<String>, text: &str) -> String {
    let base = format!("s-{}", slugify(text));
    let mut id = base.clone();
    let mut n = 2;
    while used.contains(&id) {
        id = format!("{base}-{n}");
        n += 1;
    }
    used.insert(id.clone());
    id
}

pub fn link_toc(
    chapters: &mut [Chapter],
    labels: &[Option<String>],
    pdf: &[u32],
    removed: &[String],
    warnings: &mut Vec<String>,
) {
    let mut used_ids: Vec<HashSet<String>> = vec![HashSet::new(); chapters.len()];
    // Take the TOC entries out of the chapter that holds them: the front
    // matter, or a "Contents" chapter of its own.
    if let Some(tc) = chapters.iter().position(|c| c.blocks.iter().any(|b| matches!(b.block, Block::Toc(_)))) {
        // (all of them are linked together, and each stays on its page)
        let mut entries = Vec::new();
        let mut tocs: Vec<(usize, usize)> = Vec::new();
        for (bi, b) in chapters[tc].blocks.iter_mut().enumerate() {
            if let Block::Toc(e) = &mut b.block {
                // (entries of chapters taken out of the book make no item)
                let kept: Vec<crate::TocEntry> = e.drain(..).filter(|e| !removed.contains(&key(&plain(&e.title)))).collect();
                tocs.push((bi, kept.len()));
                entries.extend(kept);
            }
        }
        if !tocs.is_empty() {
            let shown = entries.clone();
            let mut items = link_entries(chapters, labels, pdf, removed, warnings, &mut used_ids, entries);
            debug_assert_eq!(items.len(), shown.len());
            let mut end = items.len();
            for &(bi, n) in tocs.iter().rev() {
                let page = chapters[tc].blocks[bi].page;
                let from = end.saturating_sub(n);
                let of_page: Vec<Block> = items.drain(from..end.min(items.len())).collect();
                let entries = &shown[from..end.min(shown.len())];
                end = from;
                // contents set in columns: a table; else a list
                let blocks = if entries.iter().any(|e| e.columns) { vec![contents_table(entries, of_page)] } else { of_page };
                chapters[tc].blocks.splice(bi..=bi, blocks.into_iter().map(|block| crate::Blk { block, page, centered: false }));
            }
        }
    }
    // Every remaining heading gets an id too.
    for (ci, ch) in chapters.iter_mut().enumerate() {
        for b in ch.blocks.iter_mut() {
            if let Block::Heading { id, text, .. } = &mut b.block {
                if id.is_empty() {
                    *id = unique_id(&mut used_ids[ci], &plain(text));
                }
            }
        }
    }
}

/// The entries of a table of contents set in columns, as a table: the
/// chapter's number, its title, its pages. A title that starts in the
/// first column (a preface, an index) takes the first two; the name of a
/// part is a heading, centered in the middle column.
fn contents_table(entries: &[crate::TocEntry], items: Vec<Block>) -> Block {
    use crate::{Align, Cell};
    let cell = |text: Vec<Inline>| Cell { text, span: 1, ..Default::default() };
    let rows = entries
        .iter()
        .zip(items)
        .map(|(e, item)| {
            let Block::Item { text, .. } = item else { return Vec::new() };
            // (the item: the title, " — ", the pages)
            let at = text.iter().position(|i| matches!(i, Inline::Text(t) if t.starts_with(" — "))).unwrap_or(text.len());
            let (title, mut pages) = (text[..at].to_vec(), text[at..].to_vec());
            if let Some(Inline::Text(t)) = pages.first_mut() {
                *t = t.trim_start_matches(" — ").to_string();
            }
            if e.part {
                return vec![cell(Vec::new()), Cell { text: title, align: Align::Center, heading: 2, span: 1 }, cell(Vec::new())];
            }
            let pages = Cell { text: pages, align: Align::Right, heading: 0, span: 1 };
            if e.number.is_empty() {
                vec![Cell { text: title, span: 2, ..Default::default() }, pages]
            } else {
                vec![cell(vec![Inline::Text(e.number.clone())]), cell(title), pages]
            }
        })
        .filter(|row| !row.is_empty())
        .collect();
    // (a narrow column for the numbers, one for the pages)
    Block::Table { rows, widths: vec![10, 0, 20], key: String::new(), title: Vec::new() }
}

fn link_entries(
    chapters: &mut [Chapter],
    labels: &[Option<String>],
    pdf: &[u32],
    removed: &[String],
    warnings: &mut Vec<String>,
    used_ids: &mut [HashSet<String>],
    entries: Vec<crate::TocEntry>,
) -> Vec<Block> {
    // Nesting: bigger type = level 1; otherwise by indentation.
    let min_size = entries.iter().map(|e| e.size).fold(f32::MAX, f32::min);
    let mut lefts: Vec<f32> = entries.iter().filter(|e| e.size <= min_size + 1.0).map(|e| e.left).collect();
    lefts.sort_by(f32::total_cmp);
    lefts.dedup_by(|a, b| (*a - *b).abs() < 4.0);
    let has_big = entries.iter().any(|e| e.size > min_size + 1.0);
    let level_of = |e: &crate::TocEntry| -> usize {
        if e.size > min_size + 1.0 {
            return 1;
        }
        let rank = lefts.iter().position(|&l| (l - e.left).abs() < 4.0).unwrap_or(0);
        rank + if has_big { 2 } else { 1 }
    };

    // Page numbers after each entry, so you can see which file holds a
    // page: a chapter's range ("pp. 4–30"), a section's page ("p. 15"),
    // each linked to its page marker.
    let pages = page_map(chapters);
    let ranges: Vec<Option<(String, String)>> = chapters.iter().map(page_range).collect();
    let page_link = |label: &str| -> Vec<Inline> {
        match pages.get(label) {
            Some(href) => vec![
                Inline::Text(" — ".into()),
                Inline::Link { text: format!("p. {label}"), href: href.clone() },
            ],
            // ("@N" is a link target, not a page number to show)
            None if !label.is_empty() && !label.starts_with('@') => vec![Inline::Text(format!(" — p. {label}"))],
            None => Vec::new(),
        }
    };
    let chapter_pages = |ci: usize| -> Vec<Inline> {
        match &ranges[ci] {
            Some((a, b)) if a != b => vec![
                Inline::Text(" — ".into()),
                Inline::Link { text: format!("pp. {a}–{b}"), href: pages.get(a).cloned().unwrap_or_else(|| format!("{}.md#p{a}", chapters[ci].slug)) },
            ],
            Some((a, _)) => vec![
                Inline::Text(" — ".into()),
                Inline::Link { text: format!("p. {a}"), href: pages.get(a).cloned().unwrap_or_else(|| format!("{}.md#p{a}", chapters[ci].slug)) },
            ],
            None => Vec::new(),
        }
    };
    let chapter_pages: Vec<Vec<Inline>> = (0..chapters.len()).map(chapter_pages).collect();

    let mut items = Vec::new();
    let mut cursor = (1usize, 0usize); // search forward from the last match
    for e in &entries {
        let level = level_of(e);
        let title = plain(&e.title);
        let k = key(&title);
        if removed.contains(&k) {
            continue;
        }
        // (as the contents show it: in columns, without its number)
        let text = render_inlines(if e.label.is_empty() { &e.title } else { &e.label }).replace("<br>\n", "<br>");
        // 1. a whole chapter? (a linked entry: also a chapter that starts on
        //    the page it links to, with some words in common)
        let starts_there = |c: usize| {
            chapters[c].blocks.iter().find(|b| !matches!(b.block, Block::Anchor(_))).is_some_and(|b| page_near(labels, pdf, b.page, &e.page))
                && e.page.starts_with('@')
                && similarity(&chapters[c].title, &title) >= 0.3
        };
        if let Some(ci) = (1..chapters.len()).find(|&c| {
            let ck = key(&chapters[c].title);
            ck == k || fuzzy_eq(&ck, &k) || (k.len() >= 8 && ck.starts_with(&k)) || starts_there(c)
        }) {
            let mut t = vec![Inline::Link { text, href: format!("{}.md", chapters[ci].slug) }];
            t.extend(chapter_pages[ci].iter().cloned());
            items.push(Block::Item { level: level.saturating_sub(1), text: t });
            cursor = (ci, 0);
            continue;
        }
        // 1b. a part of the front matter (a preface: a chapter of its own
        //     before the front matter became one file)
        let part = chapters[0].blocks.iter().position(|b| match &b.block {
            Block::Heading { level: 1, id, text, .. } if id.is_empty() => {
                let hk = key(&plain(text));
                hk == k || fuzzy_eq(&hk, &k)
            }
            _ => false,
        });
        if let Some(bi) = part {
            let new_id = unique_id(&mut used_ids[0], &title);
            if let Block::Heading { id, .. } = &mut chapters[0].blocks[bi].block {
                *id = new_id.clone();
            }
            let mut t = vec![Inline::Link { text, href: format!("{}.md#{}", chapters[0].slug, new_id) }];
            let shown = if e.page.starts_with('@') { page_at(chapters, 0, bi).unwrap_or_default() } else { e.page.trim().to_string() };
            t.extend(page_link(&shown));
            items.push(Block::Item { level: level.saturating_sub(1), text: t });
            continue;
        }
        // 2. a heading near the printed page: same words, or mostly the
        //    same words (the TOC is often worded a little differently)
        let mut best: Option<(f32, usize, usize)> = None;
        for (ci, ch) in chapters.iter().enumerate().skip(1) {
            for (bi, b) in ch.blocks.iter().enumerate() {
                let Block::Heading { text, id, .. } = &b.block else { continue };
                if !id.is_empty() || !page_near(labels, pdf, b.page, &e.page) {
                    continue;
                }
                let ht = plain(text);
                let hk = key(&ht);
                let score = if hk == k {
                    2.0
                } else if fuzzy_eq(&hk, &k) {
                    1.5
                } else {
                    similarity(&ht, &title)
                };
                if score >= 0.5 && best.is_none_or(|(s, _, _)| score > s) {
                    best = Some((score, ci, bi));
                }
            }
        }
        // 3. the same words anywhere after the previous match
        let hit = best.map(|(_, ci, bi)| (ci, bi)).or_else(|| {
            (1..chapters.len()).find_map(|ci| {
                chapters[ci].blocks.iter().enumerate().find_map(|(bi, b)| match &b.block {
                    Block::Heading { text, id, .. }
                        if id.is_empty() && (ci, bi) >= cursor && key(&plain(text)) == k =>
                    {
                        Some((ci, bi))
                    }
                    _ => None,
                })
            })
        });
        if let Some((ci, bi)) = hit {
            let new_id = unique_id(&mut used_ids[ci], &title);
            if let Block::Heading { level: l, id, .. } = &mut chapters[ci].blocks[bi].block {
                *l = level.clamp(2, 4);
                *id = new_id.clone();
            }
            let mut t = vec![Inline::Link { text, href: format!("{}.md#{}", chapters[ci].slug, new_id) }];
            // (an entry found by its link: the printed page it lands on)
            let shown = if e.page.starts_with('@') { page_at(chapters, ci, bi).unwrap_or_default() } else { e.page.trim().to_string() };
            t.extend(page_link(&shown));
            items.push(Block::Item { level: level.saturating_sub(1), text: t });
            cursor = (ci, bi + 1);
            continue;
        }
        // 4. a paragraph on that page: a short one with mostly the same
        //    words is really an unstyled heading; a long one that starts
        //    with the title has a "run-in" heading and gets an anchor.
        let mut para_hit = None;
        'outer: for (ci, ch) in chapters.iter().enumerate().skip(1) {
            for (bi, b) in ch.blocks.iter().enumerate() {
                if !page_near(labels, pdf, b.page, &e.page) {
                    continue;
                }
                if let Block::Para(inl) = &b.block {
                    let pt = plain(inl);
                    let short = pt.split_whitespace().count() <= title.split_whitespace().count() + 3;
                    if short && similarity(&pt, &title) >= 0.6 {
                        para_hit = Some((ci, bi, true));
                        break 'outer;
                    }
                    if k.len() >= 6 && key(&pt).starts_with(&k) {
                        para_hit = Some((ci, bi, false));
                        break 'outer;
                    }
                }
            }
        }
        if let Some((ci, bi, is_heading)) = para_hit {
            let new_id = unique_id(&mut used_ids[ci], &title);
            let block = &mut chapters[ci].blocks[bi].block;
            if let Block::Para(inl) = block {
                if is_heading {
                    let text = std::mem::take(inl);
                    *block = Block::Heading { level: level.clamp(2, 4), size: 0.0, id: new_id.clone(), text };
                } else {
                    inl.insert(0, Inline::Anchor(new_id.clone()));
                }
            }
            let mut t = vec![Inline::Link { text, href: format!("{}.md#{}", chapters[ci].slug, new_id) }];
            // (an entry found by its link: the printed page it lands on)
            let shown = if e.page.starts_with('@') { page_at(chapters, ci, bi).unwrap_or_default() } else { e.page.trim().to_string() };
            t.extend(page_link(&shown));
            items.push(Block::Item { level: level.saturating_sub(1), text: t });
            cursor = (ci, bi + 1);
            continue;
        }
        // 5. a linked entry whose heading is not in the text (an image, or
        //    dropped by the ebook converter): an anchor where its page starts
        if e.page.starts_with('@') {
            let spot = (1..chapters.len()).find_map(|ci| {
                chapters[ci]
                    .blocks
                    .iter()
                    .position(|b| !matches!(b.block, Block::Anchor(_)) && page_near(labels, pdf, b.page, &e.page))
                    .map(|bi| (ci, bi))
            });
            if let Some((ci, bi)) = spot {
                let new_id = unique_id(&mut used_ids[ci], &title);
                let page = chapters[ci].blocks[bi].page;
                match &mut chapters[ci].blocks[bi].block {
                    Block::Para(inl) => inl.insert(0, Inline::Anchor(new_id.clone())),
                    _ => chapters[ci].blocks.insert(bi, crate::Blk { block: Block::Anchor(new_id.clone()), page, centered: false }),
                }
                let mut t = vec![Inline::Link { text, href: format!("{}.md#{}", chapters[ci].slug, new_id) }];
                t.extend(page_link(&page_at(chapters, ci, bi + 1).unwrap_or_default()));
                items.push(Block::Item { level: level.saturating_sub(1), text: t });
                cursor = (ci, bi + 1);
                continue;
            }
        }
        warnings.push(format!("table of contents entry not found in text: {title} (page {})", e.page));
        let mut t = e.title.clone();
        t.extend(page_link(e.page.trim()));
        items.push(Block::Item { level: level.saturating_sub(1), text: t });
    }
    items
}

/// First and last printed page of a chapter (from its page markers).
fn page_range(ch: &Chapter) -> Option<(String, String)> {
    let mut labels: Vec<String> = Vec::new();
    for b in &ch.blocks {
        match &b.block {
            Block::Anchor(id) => labels.extend(page_label(id).map(str::to_string)),
            Block::Para(inl) => {
                for i in inl {
                    if let Inline::Anchor(id) = i {
                        labels.extend(page_label(id).map(str::to_string));
                    }
                }
            }
            _ => {}
        }
    }
    Some((labels.first()?.clone(), labels.last()?.clone()))
}

/// The printed page at a place in the book: the last page marker before
/// block `bi` of chapter `ci`, or, before the chapter's first marker, the
/// chapter's first page (a chapter starts on a new page; its marker can sit
/// a line below the chapter's opening headings).
fn page_at(chapters: &[Chapter], ci: usize, bi: usize) -> Option<String> {
    let label_in = |b: &crate::Blk| -> Option<String> {
        match &b.block {
            Block::Anchor(id) => page_label(id).map(str::to_string),
            Block::Para(inl) => inl.iter().rev().find_map(|i| match i {
                Inline::Anchor(id) => page_label(id).map(str::to_string),
                _ => None,
            }),
            _ => None,
        }
    };
    let here = chapters.get(ci)?.blocks[..bi.min(chapters[ci].blocks.len())].iter().rev().find_map(label_in);
    here.or_else(|| page_range(&chapters[ci]).map(|(first, _)| first))
        .or_else(|| chapters[..ci].iter().rev().find_map(|c| c.blocks.iter().rev().find_map(label_in)))
}

/// Printed page label -> slug of the chapter file that contains it.
fn page_map(chapters: &[Chapter]) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for ch in chapters {
        for b in &ch.blocks {
            // label -> "file.md#id" of its marker
            let mut add = |id: &str| {
                if let Some(l) = page_label(id) {
                    map.entry(l.to_string()).or_insert_with(|| format!("{}.md#{id}", ch.slug));
                }
            };
            match &b.block {
                Block::Anchor(id) => add(id),
                Block::Para(inl) => {
                    for i in inl {
                        if let Inline::Anchor(id) = i {
                            add(id);
                        }
                    }
                }
                _ => {}
            }
        }
    }
    map
}

/// "amplitude, 4, 29, 55–58" -> the numbers become links; `target` gives
/// the link for a page label (None: leave the number as text).
fn link_numbers(t: &str, target: &mut dyn FnMut(&str) -> Option<String>, out: &mut Vec<Inline>) {
    let mut buf = String::new();
    let chars: Vec<char> = t.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let starts = chars[i].is_ascii_digit()
            && (i == 0 || chars[i - 1].is_whitespace() || chars[i - 1] == ',');
        if !starts {
            buf.push(chars[i]);
            i += 1;
            continue;
        }
        let mut j = i;
        while j < chars.len() && (chars[j].is_ascii_digit() || chars[j] == '–' || chars[j] == '-') {
            j += 1;
        }
        let tok: String = chars[i..j].iter().collect();
        let ends = j == chars.len() || matches!(chars[j], ',' | ';' | ' ' | '.');
        let first: String = tok.chars().take_while(|c| c.is_ascii_digit()).collect();
        match ends.then(|| target(&first)).flatten() {
            Some(href) => {
                if !buf.is_empty() {
                    out.push(Inline::Text(std::mem::take(&mut buf)));
                }
                out.push(Inline::Link { text: tok, href });
            }
            None => buf.push_str(&tok),
        }
        i = j;
    }
    if !buf.is_empty() {
        out.push(Inline::Text(buf));
    }
}

/// A place in the text: chapter, block, inline, byte offset in the inline.
type Spot = (usize, usize, usize, usize);

/// Text pieces of every printed page: label -> (chapter, block, inline).
fn page_texts(chapters: &[Chapter], skip: usize) -> HashMap<String, Vec<(usize, usize, usize)>> {
    let mut map: HashMap<String, Vec<(usize, usize, usize)>> = HashMap::new();
    for (ci, ch) in chapters.iter().enumerate() {
        if ci == skip {
            continue;
        }
        let mut page: Option<String> = None;
        for (bi, b) in ch.blocks.iter().enumerate() {
            let inl = match &b.block {
                Block::Anchor(id) => {
                    if let Some(l) = page_label(id) {
                        page = Some(l.to_string());
                    }
                    continue;
                }
                Block::Para(inl) | Block::Heading { text: inl, .. } | Block::Item { text: inl, .. } => inl,
                Block::PageNote { text, .. } => text,
                _ => continue,
            };
            for (ii, x) in inl.iter().enumerate() {
                match x {
                    Inline::Anchor(id) => {
                        if let Some(l) = page_label(id) {
                            page = Some(l.to_string());
                        }
                    }
                    Inline::Text(_) | Inline::Emph(_) => {
                        if let Some(p) = &page {
                            map.entry(p.clone()).or_default().push((ci, bi, ii));
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    map
}

fn inline_text(chapters: &[Chapter], (ci, bi, ii): (usize, usize, usize)) -> &str {
    let inl = match &chapters[ci].blocks[bi].block {
        Block::Para(inl) | Block::Heading { text: inl, .. } | Block::Item { text: inl, .. } => inl,
        Block::PageNote { text, .. } => text,
        _ => return "",
    };
    match &inl[ii] {
        Inline::Text(t) | Inline::Emph(t) | Inline::Strong(t) => t,
        _ => "",
    }
}

/// Byte offset of `needle` (lowercase words) in `hay` at a word start,
/// ignoring case and treating hyphens as spaces.
fn find_term(hay: &str, needle: &str) -> Option<(usize, usize)> {
    find_term_with(hay, needle, false)
}

/// Like [`find_term`], but the last word only has to share its stem
/// ("anticommute" finds "anticommuting", "renormalization" "renormalized").
fn find_stem(hay: &str, needle: &str) -> Option<(usize, usize)> {
    let last = needle.rsplit(' ').next().unwrap_or(needle);
    let n = last.chars().count();
    if n < 5 {
        return None;
    }
    let keep = n - (n / 4).clamp(1, 3);
    let cut: String = needle.chars().take(needle.chars().count() - (n - keep)).collect();
    find_term_with(hay, &cut, true)
}

/// (byte offset, byte length) of the match; with `open_end` the match runs
/// on to the end of the word.
fn find_term_with(hay: &str, needle: &str, open_end: bool) -> Option<(usize, usize)> {
    let norm: Vec<(usize, char)> = hay
        .char_indices()
        .map(|(i, c)| (i, if c == '-' || c == '‐' || c.is_whitespace() { ' ' } else { c.to_lowercase().next().unwrap_or(c) }))
        .collect();
    let nd: Vec<char> = needle.chars().collect();
    if nd.is_empty() {
        return None;
    }
    (0..norm.len()).find_map(|k| {
        let word_start = k == 0 || !norm[k - 1].1.is_alphanumeric();
        let fits = k + nd.len() <= norm.len() && nd.iter().enumerate().all(|(m, &c)| norm[k + m].1 == c);
        let word_end = fits && (open_end || k + nd.len() == norm.len() || !norm[k + nd.len()].1.is_alphanumeric());
        (word_start && fits && word_end).then(|| {
            let mut e = k + nd.len();
            while open_end && e < norm.len() && norm[e].1.is_alphanumeric() {
                e += 1;
            }
            let end = norm.get(e).map_or(hay.len(), |x| x.0);
            (norm[k].0, end - norm[k].0)
        })
    })
}

/// What to look for on the page, best first: "Lamb shift"; for "Lamb,
/// Willis E." also "Lamb"; for a sub-entry "and Lamb shift" under "Lamb"
/// also "Lamb shift", then the main entry.
fn term_candidates(term: &str, parent: Option<&str>) -> Vec<String> {
    const SMALL: [&str; 12] = ["and", "of", "in", "for", "the", "by", "with", "on", "to", "from", "a", "an"];
    let clean = |t: &str| -> String {
        let t: String = t.chars().map(|c| if c == '-' || c == '‐' { ' ' } else { c }).collect();
        let words: Vec<String> = t
            .split(|c: char| c.is_whitespace() || matches!(c, '(' | ')' | '“' | '”' | '"'))
            .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase())
            .filter(|w| !w.is_empty())
            .collect();
        words.join(" ")
    };
    let mut v: Vec<String> = Vec::new();
    let mut add = |t: String| {
        if t.chars().count() >= 2 && !v.contains(&t) {
            v.push(t);
        }
    };
    let strip = |t: &str| -> String {
        let mut w: Vec<&str> = t.split(' ').collect();
        while w.len() > 1 && SMALL.contains(&w[0]) {
            w.remove(0);
        }
        w.join(" ")
    };
    add(clean(term));
    add(strip(&clean(term)));
    if let Some((a, b)) = term.split_once(',') {
        // "scattering, Compton": "Compton scattering"; "Lamb, Willis E.": "Lamb"
        add(clean(&format!("{b} {a}")));
        add(clean(a));
    }
    if let Some(p) = parent {
        let main = clean(p.split(',').next().unwrap_or(p));
        let sub = strip(&clean(term));
        add(format!("{sub} {main}"));
        add(format!("{main} {sub}"));
        add(main);
    }
    v
}

pub fn link_index(chapters: &mut [Chapter], pdf: &[u32]) {
    let pages = page_map(chapters);
    let Some(ix) = chapters.iter().position(|c| key(&c.title) == "index") else { return };
    let texts = page_texts(chapters, ix);
    // For a linked index (each number links to a PDF page): the text pieces
    // of every PDF page, and where each page's text starts.
    let mut pdf_texts: HashMap<u32, Vec<(usize, usize, usize)>> = HashMap::new();
    let mut pdf_start: HashMap<u32, (usize, usize)> = HashMap::new();
    for (ci, ch) in chapters.iter().enumerate() {
        if ci == ix {
            continue;
        }
        for (bi, b) in ch.blocks.iter().enumerate() {
            let Some(&n) = pdf.get(b.page) else { continue };
            if !matches!(b.block, Block::Anchor(_)) {
                pdf_start.entry(n).or_insert((ci, bi));
            }
            let inl = match &b.block {
                Block::Para(inl) | Block::Heading { text: inl, .. } | Block::Item { text: inl, .. } => inl,
                Block::PageNote { text, .. } => text,
                _ => continue,
            };
            for (ii, x) in inl.iter().enumerate() {
                if matches!(x, Inline::Text(_) | Inline::Emph(_)) {
                    pdf_texts.entry(n).or_default().push((ci, bi, ii));
                }
            }
        }
    }
    // pages a number links to where its term was not found: anchor ids
    let mut page_anchors: HashMap<u32, String> = HashMap::new();
    // anchors to put into the text: spot -> id
    let mut spots: HashMap<Spot, (String, usize)> = HashMap::new();
    let mut blocks = std::mem::take(&mut chapters[ix].blocks);
    let mut parent: Option<String> = None;
    for b in blocks.iter_mut() {
        let level = match &b.block {
            Block::Item { level, .. } => *level,
            Block::Para(_) => 0,
            _ => continue,
        };
        let (Block::Item { text, .. } | Block::Para(text)) = &mut b.block else { continue };
        // the entry's term: its text up to the first page number
        let all = plain(text);
        let cut = all
            .char_indices()
            .find(|&(i, c)| c == ',' && all[i + 1..].trim_start().starts_with(|d: char| d.is_ascii_digit()))
            .map_or(all.len(), |(i, _)| i);
        let term = all[..cut].trim().to_string();
        let cands = term_candidates(&term, if level > 0 { parent.as_deref() } else { None });
        if level == 0 {
            parent = Some(term.clone());
        }
        let chs: &[Chapter] = chapters;
        // A page number's target: for a number that links to a PDF page, the
        // term on exactly that page, else the start of that page; otherwise
        // the term on the printed page, else that page's marker.
        let mut resolve = |label: &str, linked: Option<u32>| -> Option<String> {
            if let Some(n) = linked {
                // (a printed page that began on PDF page n can run onto n + 1)
                let pieces: Vec<(usize, usize, usize)> =
                    [n, n + 1].iter().flat_map(|m| pdf_texts.get(m).into_iter().flatten().copied()).collect();
                let search = |f: fn(&str, &str) -> Option<(usize, usize)>| {
                    cands.iter().find_map(|c| {
                        pieces.iter().find_map(|&p| f(inline_text(chs, p), c).map(|(off, len)| ((p.0, p.1, p.2, off), len)))
                    })
                };
                let found = search(find_term).or_else(|| search(find_stem));
                return Some(match found {
                    Some((spot, len)) => {
                        let k = spots.len();
                        let id = spots.entry(spot).or_insert_with(|| (format!("ix-{k}"), len)).0.clone();
                        format!("{}.md#{id}", chs[spot.0].slug)
                    }
                    None => {
                        let &(ci, _) = pdf_start.get(&n)?;
                        let id = page_anchors.entry(n).or_insert_with(|| format!("pdf-page-{n}")).clone();
                        format!("{}.md#{id}", chs[ci].slug)
                    }
                });
            }
            let page_href = pages.get(label)?;
            let found = texts.get(label).and_then(|pieces| {
                let search = |f: fn(&str, &str) -> Option<(usize, usize)>| {
                    cands.iter().find_map(|c| {
                        pieces.iter().find_map(|&p| f(inline_text(chs, p), c).map(|(off, len)| ((p.0, p.1, p.2, off), len)))
                    })
                };
                search(find_term).or_else(|| search(find_stem))
            });
            Some(match found {
                Some((spot, len)) => {
                    let n = spots.len();
                    let id = spots.entry(spot).or_insert_with(|| (format!("ix-{n}"), len)).0.clone();
                    format!("{}.md#{id}", chs[spot.0].slug)
                }
                None => page_href.clone(),
            })
        };
        let old = std::mem::take(text);
        let mut k = 0;
        while k < old.len() {
            match (&old[k], old.get(k + 1)) {
                // "…, 214" + PageLink: the last number goes to that PDF page
                (Inline::Text(t), Some(Inline::PageLink(n))) => {
                    let cut = t.trim_end_matches([',', ';', '.', ' ']).trim_end_matches(|c: char| c.is_ascii_digit() || c == '–' || c == '-').len();
                    link_numbers(&t[..cut], &mut |l: &str| resolve(l, None), text);
                    let n = *n;
                    link_numbers(&t[cut..], &mut |l: &str| resolve(l, Some(n)), text);
                    k += 2;
                }
                (Inline::Text(t), _) => {
                    link_numbers(t, &mut |l: &str| resolve(l, None), text);
                    k += 1;
                }
                (Inline::PageLink(_), _) => k += 1,
                (other, _) => {
                    text.push(other.clone());
                    k += 1;
                }
            }
        }
    }
    chapters[ix].blocks = blocks;

    // Wrap the found words in their targets, splitting the text around
    // them (last offsets first, so earlier ones stay valid).
    let mut todo: Vec<(Spot, (String, usize))> = spots.into_iter().collect();
    todo.sort_by_key(|t| std::cmp::Reverse(t.0));
    for ((ci, bi, ii, off), (id, len)) in todo {
        let inl = match &mut chapters[ci].blocks[bi].block {
            Block::Para(inl) | Block::Heading { text: inl, .. } | Block::Item { text: inl, .. } => inl,
            Block::PageNote { text, .. } => text,
            _ => continue,
        };
        let (t, emph) = match &inl[ii] {
            Inline::Text(t) => (t.clone(), false),
            Inline::Emph(t) => (t.clone(), true),
            _ => continue,
        };
        // a later (overlapping) match may have shortened this piece
        let end = (off + len).min(t.len());
        let wrap = |t: &str| if emph { Inline::Emph(t.to_string()) } else { Inline::Text(t.to_string()) };
        let mut repl = Vec::new();
        if off > 0 {
            repl.push(wrap(&t[..off]));
        }
        repl.push(Inline::Target { id, text: t[off..end].to_string(), emph });
        if end < t.len() {
            repl.push(wrap(&t[end..]));
        }
        inl.splice(ii..ii + 1, repl);
    }

    // Anchors where linked pages start, for numbers whose term was not found
    // on their page (last blocks first, so earlier positions stay valid).
    let mut starts: Vec<((usize, usize), String)> = page_anchors.into_iter().filter_map(|(n, id)| Some((*pdf_start.get(&n)?, id))).collect();
    starts.sort_by_key(|s| std::cmp::Reverse(s.0));
    for ((ci, bi), id) in starts {
        let page = chapters[ci].blocks[bi].page;
        match &mut chapters[ci].blocks[bi].block {
            Block::Para(inl) => inl.insert(0, Inline::Anchor(id)),
            _ => chapters[ci].blocks.insert(bi, crate::Blk { block: Block::Anchor(id), page, centered: false }),
        }
    }
}

/// Every run of text in a chapter that references can sit in: paragraphs,
/// list items, page notes, and the chapter's notes and references.
fn text_runs(ch: &mut Chapter) -> Vec<&mut Vec<Inline>> {
    let blocks = ch.blocks.iter_mut().map(|b| &mut b.block);
    let notes = ch.notes.iter_mut().chain(ch.refs.iter_mut()).flat_map(|n| n.blocks.iter_mut());
    blocks
        .chain(notes)
        .filter_map(|b| match b {
            Block::Para(inl) | Block::Item { text: inl, .. } | Block::PageNote { text: inl, .. } => Some(inl),
            Block::Table { title, .. } if !title.is_empty() => Some(title),
            _ => None,
        })
        .collect()
}

/// A problem number: "6.8", "B.2", "6.8a" (chapter or appendix, dot, number).
fn problem_number(t: &str) -> Option<&str> {
    let t = t.trim().trim_end_matches([',', ';', ':', '.', ')']);
    let (a, b) = t.split_once('.')?;
    let b = b.trim_end_matches(|c: char| c.is_ascii_lowercase());
    (!a.is_empty() && a.len() <= 2 && a.chars().all(|c| c.is_ascii_alphanumeric()) && !b.is_empty() && b.chars().all(|c| c.is_ascii_digit()))
        .then_some(t)
}

/// "Problem 6.8" links to problem 6.8 in the "Problems" section at the end
/// of its chapter. Each problem there (a paragraph starting with its number)
/// gets an anchor problem-6.8. Runs before the equation references, so a
/// problem number is not taken for an equation number.
pub fn link_problems(chapters: &mut [Chapter]) {
    let mut target: HashMap<String, String> = HashMap::new();
    for ch in chapters.iter_mut() {
        // Problems that ran into the paragraph of the one before: a bold
        // problem number inside a paragraph of the Problems section starts
        // a new paragraph.
        let mut in_problems = false;
        let old = std::mem::take(&mut ch.blocks);
        for b in old {
            match &b.block {
                Block::Heading { text, .. } => in_problems = key(&plain(text)) == "problems",
                Block::Para(inl) if in_problems => {
                    let starts = |x: &Inline| matches!(x, Inline::Strong(t) if problem_number(t).is_some());
                    let mut parts: Vec<Vec<Inline>> = vec![Vec::new()];
                    for x in inl {
                        if starts(x) && parts.last().is_some_and(|p| p.iter().any(|y| !matches!(y, Inline::Anchor(_)))) {
                            parts.push(Vec::new());
                        }
                        parts.last_mut().unwrap().push(x.clone());
                    }
                    for p in parts {
                        let mut p = p;
                        if let Some(Inline::Text(t)) = p.last_mut() {
                            *t = t.trim_end().to_string();
                        }
                        if let Some(Inline::Text(t)) = p.first_mut() {
                            *t = t.trim_start().to_string();
                        }
                        ch.blocks.push(crate::Blk { block: Block::Para(p), page: b.page, centered: b.centered });
                    }
                    continue;
                }
                _ => {}
            }
            ch.blocks.push(b);
        }
        let mut in_problems = false;
        for b in ch.blocks.iter_mut() {
            match &mut b.block {
                Block::Heading { text, .. } => in_problems = key(&plain(text)) == "problems",
                Block::Para(inl) if in_problems => {
                    let first = inl.iter().position(|x| !matches!(x, Inline::Anchor(_)));
                    let Some(k) = first else { continue };
                    let (Inline::Strong(t) | Inline::Text(t)) = &inl[k] else { continue };
                    let Some(word) = t.split_whitespace().next() else { continue };
                    let Some(n) = problem_number(word) else { continue };
                    let n = n.trim_end_matches(|c: char| c.is_ascii_lowercase()).to_string();
                    let id = format!("problem-{n}");
                    if let std::collections::hash_map::Entry::Vacant(e) = target.entry(n) {
                        e.insert(format!("{}.md#{id}", ch.slug));
                        inl.insert(k, Inline::Anchor(id));
                    }
                }
                _ => {}
            }
        }
    }
    if target.is_empty() {
        return;
    }
    for ch in chapters.iter_mut() {
        for inl in text_runs(ch) {
            if !inl.iter().any(|x| matches!(x, Inline::Text(t) if t.contains("Problem"))) {
                continue;
            }
            // "Problem" and its number can be separate links: join the text
            let mut old: Vec<Inline> = Vec::new();
            for x in std::mem::take(inl) {
                if let (Inline::Text(b), Some(Inline::PageLink(_))) = (&x, old.last()) {
                    let n = old.len();
                    if n >= 2 {
                        if let Inline::Text(a) = &old[n - 2] {
                            if a.trim_end().ends_with("Problem") || a.trim_end().ends_with("Problems") {
                                let joined = format!("{a}{b}");
                                old.truncate(n - 2);
                                old.push(Inline::Text(joined));
                                continue;
                            }
                        }
                    }
                }
                old.push(x);
            }
            for x in old {
                match x {
                    Inline::Text(t) if t.contains("Problem") => link_problem_refs(&t, &target, inl),
                    other => inl.push(other),
                }
            }
        }
    }
}

/// Split text at "Problem 6.8" / "Problems 6.8, 6.9 and 6.10": each number
/// becomes a link to its problem.
fn link_problem_refs(t: &str, target: &HashMap<String, String>, out: &mut Vec<Inline>) {
    let mut rest = t;
    let mut buf = String::new();
    while let Some(at) = rest.find("Problem") {
        let after_word = at + "Problem".len() + if rest[at + 7..].starts_with('s') { 1 } else { 0 };
        buf.push_str(&rest[..at]);
        // the word itself goes into the first link ("Problem 6.8")
        let mut problem_word = Some(&rest[at..after_word]);
        rest = &rest[after_word..];
        // the numbers after it, separated by spaces, commas, "and", "–"
        loop {
            let lead = rest.len() - rest.trim_start_matches([' ', ',', '–', '-']).len();
            let lead = if rest[lead..].starts_with("and ") { lead + 4 } else { lead };
            let lead = lead + (rest[lead..].len() - rest[lead..].trim_start().len());
            let word: String = rest[lead..].chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '.').collect();
            let Some(n) = problem_number(&word) else { break };
            let base = n.trim_end_matches(|c: char| c.is_ascii_lowercase());
            let Some(href) = target.get(base) else { break };
            if lead == 0 {
                break; // "Problem6.8": not a reference
            }
            let text = match problem_word.take() {
                Some(w) => format!("{w}{}{n}", &rest[..lead]),
                None => {
                    buf.push_str(&rest[..lead]);
                    n.to_string()
                }
            };
            if !buf.is_empty() {
                out.push(Inline::Text(std::mem::take(&mut buf)));
            }
            out.push(Inline::Link { text, href: href.clone() });
            rest = &rest[lead + n.len()..];
        }
        // no number after the word: it stays plain text
        if let Some(w) = problem_word {
            buf.push_str(w);
        }
    }
    buf.push_str(rest);
    if !buf.is_empty() {
        out.push(Inline::Text(buf));
    }
}

/// Linked references to equations ("(3.422)" that is a link in the PDF)
/// become links to the equation's anchor, in whichever chapter it is.
pub fn link_equations(chapters: &mut [Chapter]) {
    // equation number -> (chapter file, number of the anchor to link to)
    let mut home: HashMap<String, (String, String)> = HashMap::new();
    for ch in chapters.iter() {
        for b in &ch.blocks {
            if let Block::Equation { latex, tag, .. } = &b.block {
                for t in crate::equation_numbers(latex, tag.as_deref()) {
                    home.entry(t.clone()).or_insert_with(|| (ch.slug.clone(), t));
                }
            }
        }
    }
    // "(3.425)" for the group (3.425a), (3.425b) …: its first member
    let groups: Vec<(String, (String, String))> = home
        .iter()
        .filter_map(|(t, v)| Some((t.strip_suffix('a')?.to_string(), v.clone())))
        .filter(|(base, _)| !base.is_empty() && base.ends_with(|c: char| c.is_ascii_digit()))
        .collect();
    for (base, v) in groups {
        home.entry(base).or_insert(v);
    }
    if home.is_empty() {
        return;
    }
    for ch in chapters.iter_mut() {
        for inl in text_runs(ch) {
            if !inl.iter().any(|x| matches!(x, Inline::PageLink(_))) {
                continue;
            }
            let old = std::mem::take(inl);
            let mut k = 0;
            while k < old.len() {
                if let (Inline::Text(t), Some(Inline::PageLink(_))) = (&old[k], old.get(k + 1)) {
                    // the reference is the text's last word
                    let cut = t.trim_end().rfind(char::is_whitespace).map_or(0, |i| i + 1);
                    let word = &t[cut..];
                    // (a number after "Problem", "Section", "Figure" … is
                    // that, not an equation with the same number)
                    let before = t[..cut].trim_end();
                    let problem = ["Problem", "Problems", "Section", "Sections", "Figure", "Figures", "Fig.", "Figs.", "Table", "Tables", "Chapter", "Chapters", "Appendix"]
                        .iter()
                        .any(|w| before.ends_with(w));
                    if let Some(n) = crate::blocks::equation_ref(word).filter(|n| home.contains_key(*n) && !problem) {
                        let (slug, anchor) = &home[n];
                        // keep the parentheses and punctuation around the link
                        let start = cut + word.find(n).unwrap_or(0);
                        let end = start + n.len();
                        if start > 0 {
                            inl.push(Inline::Text(t[..start].to_string()));
                        }
                        inl.push(Inline::Link { text: n.to_string(), href: format!("{slug}.md#{}", crate::equation_id(anchor)) });
                        if end < t.len() {
                            inl.push(Inline::Text(t[end..].to_string()));
                        }
                        k += 2;
                        continue;
                    }
                }
                inl.push(old[k].clone());
                k += 1;
            }
        }
    }
    // References the PDF does not link: "(3.422)" in the text, where that
    // equation exists (a dotted number in parentheses is an equation
    // reference in such books; headings are left alone)
    for ch in chapters.iter_mut() {
        for inl in text_runs(ch) {
            if !inl.iter().any(|x| matches!(x, Inline::Text(t) | Inline::Emph(t) if t.contains('('))) {
                continue;
            }
            let old = std::mem::take(inl);
            for x in old {
                // (in italic text too: the number links, the words stay italic)
                let (t, emph) = match x {
                    Inline::Text(t) => (t, false),
                    Inline::Emph(t) => (t, true),
                    other => {
                        inl.push(other);
                        continue;
                    }
                };
                let piece = |s: String| if emph { Inline::Emph(s) } else { Inline::Text(s) };
                let mut rest = t.as_str();
                let mut buf = String::new();
                while let Some(open) = rest.find('(') {
                    let after = &rest[open + 1..];
                    let close = after.find(')');
                    let n = close.map(|c| &after[..c]).filter(|n| crate::blocks::equation_ref(n) == Some(*n) && home.contains_key(*n));
                    match n {
                        Some(n) => {
                            buf.push_str(&rest[..=open]);
                            if !buf.is_empty() {
                                inl.push(piece(std::mem::take(&mut buf)));
                            }
                            let (slug, anchor) = &home[n];
                            inl.push(Inline::Link { text: n.to_string(), href: format!("{slug}.md#{}", crate::equation_id(anchor)) });
                            rest = &after[n.len()..];
                        }
                        None => {
                            buf.push_str(&rest[..=open]);
                            rest = after;
                        }
                    }
                }
                buf.push_str(rest);
                if !buf.is_empty() {
                    inl.push(piece(buf));
                }
            }
        }
    }
}

/// The number of a figure caption: "Fig. 1.1 The Stern–Gerlach …" -> "1.1".
pub fn figure_caption_number(t: &str) -> Option<String> {
    let rest = t.trim_start().strip_prefix("Fig.")?.trim_start();
    let n: String = rest.chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '.').collect();
    let n = n.trim_end_matches('.');
    (n.contains('.') && n.chars().next().is_some_and(|c| c.is_ascii_alphanumeric())).then(|| n.to_string())
}

/// The caption a paragraph starts with: ("figure", "1.1") for "Fig. 1.1 …",
/// and for a bold "Figure 4.1" or "Table 2.1" (bold, because prose can
/// start with "Figure 4.1 shows …").
pub fn caption(inl: &[Inline]) -> Option<(&'static str, String)> {
    if let Some(n) = figure_caption_number(&crate::plain(inl)) {
        return Some(("figure", n));
    }
    let first = inl.iter().find(|x| !matches!(x, Inline::Anchor(_) | Inline::PageLink(_) | Inline::Source(_)))?;
    let Inline::Strong(t) = first else { return None };
    let (kind, rest) = match t.trim_start() {
        t if t.starts_with("Figure ") => ("figure", &t[7..]),
        t if t.starts_with("Table ") => ("table", &t[6..]),
        _ => return None,
    };
    let n: String = rest.trim_start().chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '.').collect();
    let n = n.trim_end_matches('.');
    (n.contains('.') && n.starts_with(|c: char| c.is_ascii_alphanumeric())).then(|| (kind, n.to_string()))
}

/// Does a line start like a caption ("Fig. 1.1", "Figure 4.1", "Table 2.1")?
/// Such a line is no heading, however short and bold.
pub fn starts_like_caption(t: &str) -> bool {
    let t = t.trim_start();
    ["Fig. ", "Figure ", "Table "].iter().any(|w| {
        t.strip_prefix(w).is_some_and(|r| {
            let n: String = r.trim_start().chars().take_while(|c| !c.is_whitespace()).collect();
            n.starts_with(|c: char| c.is_ascii_alphanumeric()) && n.contains('.') && n.chars().any(|c| c.is_ascii_digit())
        })
    })
}

/// What stands between the members of a list of references: "and", ",".
fn list_joint(t: &str) -> bool {
    ["", "and", "or", "to", "through", "–", "—", "-"].contains(&t.trim().trim_start_matches(',').trim())
}

/// "and 1.3" / ", 1.3" / "– 1.3" right after a linked "Figures 1.2": the
/// connecting words and the number (the next member of the list).
fn list_member(body: &str) -> Option<(&str, &str)> {
    let cut = body.rfind(char::is_whitespace).map_or(0, |i| i + 1);
    let (lead, num) = body.split_at(cut);
    (list_joint(lead)
        && num.starts_with(|c: char| c.is_ascii_alphanumeric())
        && num.chars().any(|c| c.is_ascii_digit()))
    .then_some((lead, num))
}

/// Linked references to sections, chapters, appendices, figures and tables
/// ("Section 3.5", "Chapter 3", "Appendix B", "Figure 1.1", "Table 2.1")
/// become links to the heading, chapter file or caption. Captions
/// ("Fig. 1.1 …", "Table 2.1 …") get an anchor fig-1.1 / table-2.1 for
/// that. Runs before the equation references, so "Section 3.5" is not
/// taken for equation (3.5).
pub fn link_cross_refs(chapters: &mut [Chapter]) {
    // targets: "section 3.5" -> file#id, "chapter 3" -> file, "figure 1.1" -> file#fig-1.1
    let mut target: HashMap<String, String> = HashMap::new();
    for ch in chapters.iter_mut() {
        let slug = ch.slug.clone();
        if let Some(rest) = ch.title.strip_prefix("Appendix ") {
            if let Some(letter) = rest.split_whitespace().next() {
                target.entry(format!("appendix {}", letter.trim_end_matches([':', '.']))).or_insert(format!("{slug}.md"));
            }
        }
        for b in ch.blocks.iter_mut() {
            match &mut b.block {
                Block::Heading { text, id, .. } => {
                    let t = plain(text);
                    let Some(num) = t.split_whitespace().next() else { continue };
                    if num.contains('.') && num.chars().all(|c| c.is_ascii_alphanumeric() || c == '.') {
                        target.entry(format!("section {num}")).or_insert(format!("{slug}.md#{id}"));
                        // the chapter number of a numbered section
                        if let Some(c) = num.split('.').next() {
                            target.entry(format!("chapter {c}")).or_insert(format!("{slug}.md"));
                        }
                    }
                }
                // (a caption, also one that is the title of its table)
                Block::Para(inl) | Block::Table { title: inl, .. } => {
                    if let Some((kind, n)) = caption(inl) {
                        let id = format!("{}-{n}", if kind == "figure" { "fig" } else { kind });
                        if let std::collections::hash_map::Entry::Vacant(e) = target.entry(format!("{kind} {n}")) {
                            e.insert(format!("{slug}.md#{id}"));
                            inl.insert(0, Inline::Anchor(id));
                        }
                    }
                }
                _ => {}
            }
        }
    }
    // problems (anchored by `link_problems`), for "Problems 5.25 and 5.26"
    // where each number is a link of its own in the PDF
    for ch in chapters.iter_mut() {
        let slug = ch.slug.clone();
        for inl in text_runs(ch) {
            for x in inl.iter() {
                if let Some(n) = match x {
                    Inline::Anchor(id) => id.strip_prefix("problem-"),
                    _ => None,
                } {
                    target.entry(format!("problem {n}")).or_insert(format!("{slug}.md#problem-{n}"));
                }
            }
        }
    }
    if target.is_empty() {
        return;
    }
    let kinds: [(&str, &str); 10] = [
        ("Sections ", "section"),
        ("Section ", "section"),
        ("Chapter ", "chapter"),
        ("Appendix ", "appendix"),
        ("Figures ", "figure"),
        ("Figure ", "figure"),
        ("Figs. ", "figure"),
        ("Fig. ", "figure"),
        ("Tables ", "table"),
        ("Table ", "table"),
    ];
    // "Figure 1.2a": part (a) of figure 1.2
    let lookup = |target: &HashMap<String, String>, kind: &str, num: &str| {
        target.get(&format!("{kind} {num}")).or_else(|| target.get(&format!("{kind} {}", num.trim_end_matches(|c: char| c.is_ascii_lowercase())))).cloned()
    };
    for ch in chapters.iter_mut() {
        for inl in text_runs(ch) {
            if !inl.iter().any(|x| matches!(x, Inline::PageLink(_))) {
                continue;
            }
            // "Figure" and "4.5" in separate pieces (each under the link,
            // or only the number): one piece
            let mut old: Vec<Inline> = Vec::new();
            for x in std::mem::take(inl) {
                if let Inline::Text(b) = &x {
                    let at = match old.as_slice() {
                        [.., Inline::Text(_), Inline::PageLink(_)] => Some(old.len() - 2),
                        [.., Inline::Text(_)] => Some(old.len() - 1),
                        _ => None,
                    };
                    if let Some(Inline::Text(a)) = at.map(|i| &mut old[i]) {
                        let word = kinds.iter().any(|(w, _)| a.trim_end().strip_suffix(w.trim_end()).is_some_and(|r| !r.ends_with(char::is_alphabetic)));
                        let number = b.trim_start().starts_with(|c: char| c.is_ascii_digit());
                        if word && number {
                            *a = format!("{} {}", a.trim_end(), b.trim_start());
                            old.truncate(at.unwrap() + 1);
                            continue;
                        }
                    }
                }
                old.push(x);
            }
            let mut k = 0;
            // the kind of the reference linked just before this text
            // ("Figures 1.2" before " and 1.3")
            let mut list: Option<&str> = None;
            while k < old.len() {
                let in_list = list.take();
                // (kept over a link's marker and over the joining word alone)
                match &old[k] {
                    Inline::Link { href, .. } if href.contains("#problem-") => list = Some("problem"),
                    Inline::PageLink(_) => list = in_list,
                    Inline::Text(t) if matches!(old.get(k + 1), Some(Inline::PageLink(_))) && list_joint(t.trim_end()) => list = in_list,
                    _ => {}
                }
                if let (Inline::Text(t), Some(Inline::PageLink(_))) = (&old[k], old.get(k + 1)) {
                    // the reference is the end of the text: "… in Figure 1.1,"
                    let body = t.trim_end().trim_end_matches([',', ';', ':', '.', ')']);
                    let mut found = kinds.iter().find_map(|(word, kind)| {
                        let at = body.rfind(word)?;
                        if body[..at].ends_with(char::is_alphabetic) {
                            return None;
                        }
                        let href = lookup(&target, kind, body[at + word.len()..].trim())?;
                        Some((at, body.len(), href, *kind))
                    });
                    if found.is_none() {
                        if let (Some(kind), Some((lead, num))) = (in_list, list_member(body)) {
                            found = lookup(&target, kind, num).map(|href| (lead.len(), body.len(), href, kind));
                        }
                    }
                    if let Some((start, end, href, kind)) = found {
                        list = Some(kind);
                        if start > 0 {
                            inl.push(Inline::Text(t[..start].to_string()));
                        }
                        inl.push(Inline::Link { text: crate::emit::escape_text(&t[start..end]), href });
                        if end < t.len() {
                            inl.push(Inline::Text(t[end..].to_string()));
                        }
                        k += 2;
                        continue;
                    }
                }
                inl.push(old[k].clone());
                k += 1;
            }
        }
    }
}

#[cfg(test)]
mod problem_tests {
    use super::*;

    fn run(t: &str) -> String {
        let target: HashMap<String, String> =
            ["6.8", "6.9", "6.10", "B.2"].iter().map(|n| (n.to_string(), format!("p.md#problem-{n}"))).collect();
        let mut out = Vec::new();
        link_problem_refs(t, &target, &mut out);
        crate::emit::render_inlines(&out)
    }

    #[test]
    fn links_problem_numbers() {
        assert_eq!(run("see Problem 6.8."), "see [Problem 6.8](p.md#problem-6.8).");
        assert_eq!(
            run("Problems 6.8, 6.9 and 6.10 show"),
            "[Problems 6.8](p.md#problem-6.8), [6.9](p.md#problem-6.9) and [6.10](p.md#problem-6.10) show"
        );
        assert_eq!(run("in Problem 6.8a"), "in [Problem 6.8a](p.md#problem-6.8)");
        assert_eq!(run("Problem B.2"), "[Problem B.2](p.md#problem-B.2)");
        // not a problem we have, not a number, not a reference
        assert_eq!(run("Problem 7.1 and (6.8)"), "Problem 7.1 and (6.8)");
        assert_eq!(run("a Problematic 6.8"), "a Problematic 6.8");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn captions_and_reference_lists() {
        assert_eq!(caption(&[Inline::Strong("Fig. 1.1".into()), Inline::Text(" The apparatus".into())]), Some(("figure", "1.1".into())));
        assert_eq!(caption(&[Inline::Anchor("p9".into()), Inline::Strong("Figure 4.1".into())]), Some(("figure", "4.1".into())));
        assert_eq!(caption(&[Inline::Strong("Table 2.1".into())]), Some(("table", "2.1".into())));
        // prose, not a caption
        assert_eq!(caption(&[Inline::Text("Figure 4.1 shows the levels".into())]), None);
        assert!(starts_like_caption("Fig. 3.12 Energy levels"));
        assert!(starts_like_caption("Table 2.1 Solutions"));
        assert!(!starts_like_caption("Figures of merit"));
        assert!(!starts_like_caption("Table of Contents"));
        assert_eq!(list_member(" and 1.3"), Some((" and ", "1.3")));
        assert_eq!(list_member(", 1.3b"), Some((", ", "1.3b")));
        assert_eq!(list_member(" shows that 1.3"), None);
    }

    #[test]
    fn index_terms_found_in_text() {
        let c = term_candidates("Lamb, Willis E.", None);
        assert_eq!(c, vec!["lamb willis e", "willis e lamb", "lamb"]);
        let c = term_candidates("and Lamb shift", Some("level shift"));
        assert_eq!(c[0], "and lamb shift");
        assert_eq!(c[1], "lamb shift");
        let t = "the first approximation to the Lamb-shift; the 2s state";
        assert_eq!(find_term(t, "lamb shift"), Some((31, 10)));
        // whole words only
        assert_eq!(find_term("a lambda term", "lamb"), None);
        // word endings
        assert_eq!(find_term("anticommuting fields", "anticommute"), None);
        assert_eq!(find_stem("anticommuting fields", "anticommute"), Some((0, 13)));
    }
}
