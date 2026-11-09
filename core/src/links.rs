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

fn page_near(labels: &[Option<String>], page: usize, target: &str) -> bool {
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
    removed: &[String],
    warnings: &mut Vec<String>,
) {
    let mut used_ids: Vec<HashSet<String>> = vec![HashSet::new(); chapters.len()];
    if !chapters.is_empty() {
        // Take the TOC entries out of the front matter.
        let mut entries = Vec::new();
        let mut toc_at = None;
        for (bi, b) in chapters[0].blocks.iter_mut().enumerate() {
            if let Block::Toc(e) = &mut b.block {
                entries.append(e);
                toc_at.get_or_insert(bi);
            }
        }
        if let Some(at) = toc_at {
            let items = link_entries(chapters, labels, removed, warnings, &mut used_ids, entries);
            let page = chapters[0].blocks[at].page;
            chapters[0].blocks.retain(|b| !matches!(b.block, Block::Toc(_)));
            let at = at.min(chapters[0].blocks.len());
            for (k, block) in items.into_iter().enumerate() {
                chapters[0].blocks.insert(at + k, crate::Blk { block, page });
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

fn link_entries(
    chapters: &mut [Chapter],
    labels: &[Option<String>],
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

    let mut items = Vec::new();
    let mut cursor = (1usize, 0usize); // search forward from the last match
    for e in &entries {
        let level = level_of(e);
        let title = plain(&e.title);
        let k = key(&title);
        if removed.contains(&k) {
            continue;
        }
        let text = render_inlines(&e.title);
        // 1. a whole chapter?
        if let Some(ci) = (1..chapters.len()).find(|&c| {
            let ck = key(&chapters[c].title);
            ck == k || fuzzy_eq(&ck, &k) || (k.len() >= 8 && ck.starts_with(&k))
        }) {
            items.push(Block::Item {
                level: level.saturating_sub(1),
                text: vec![Inline::Link { text, href: format!("{}.md", chapters[ci].slug) }],
            });
            cursor = (ci, 0);
            continue;
        }
        // 2. a heading near the printed page: same words, or mostly the
        //    same words (the TOC is often worded a little differently)
        let mut best: Option<(f32, usize, usize)> = None;
        for ci in 1..chapters.len() {
            for (bi, b) in chapters[ci].blocks.iter().enumerate() {
                let Block::Heading { text, id, .. } = &b.block else { continue };
                if !id.is_empty() || !page_near(labels, b.page, &e.page) {
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
            items.push(Block::Item {
                level: level.saturating_sub(1),
                text: vec![Inline::Link { text, href: format!("{}.md#{}", chapters[ci].slug, new_id) }],
            });
            cursor = (ci, bi + 1);
            continue;
        }
        // 4. a paragraph on that page: a short one with mostly the same
        //    words is really an unstyled heading; a long one that starts
        //    with the title has a "run-in" heading and gets an anchor.
        let mut para_hit = None;
        'outer: for ci in 1..chapters.len() {
            for (bi, b) in chapters[ci].blocks.iter().enumerate() {
                if !page_near(labels, b.page, &e.page) {
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
            items.push(Block::Item {
                level: level.saturating_sub(1),
                text: vec![Inline::Link { text, href: format!("{}.md#{}", chapters[ci].slug, new_id) }],
            });
            cursor = (ci, bi + 1);
            continue;
        }
        warnings.push(format!("table of contents entry not found in text: {title} (page {})", e.page));
        items.push(Block::Item { level: level.saturating_sub(1), text: e.title.clone() });
    }
    items
}

/// Printed page label -> slug of the chapter file that contains it.
fn page_map(chapters: &[Chapter]) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for ch in chapters {
        for b in &ch.blocks {
            let mut add = |id: &str| {
                if let Some(l) = id.strip_prefix('p') {
                    map.entry(l.to_string()).or_insert_with(|| ch.slug.clone());
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
        Inline::Text(t) | Inline::Emph(t) => t,
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

pub fn link_index(chapters: &mut [Chapter]) {
    let pages = page_map(chapters);
    let Some(ix) = chapters.iter().position(|c| key(&c.title) == "index") else { return };
    let texts = page_texts(chapters, ix);
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
        let mut target = |label: &str| -> Option<String> {
            let slug = pages.get(label)?;
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
                None => format!("{slug}.md#p{label}"),
            })
        };
        let old = std::mem::take(text);
        for x in old {
            match x {
                Inline::Text(t) => link_numbers(&t, &mut target, text),
                other => text.push(other),
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
