//! Footnotes and references.
//!
//! * **Footnote markers** are small raised numbers glued to a word (found in
//!   `blocks.rs` as [`Inline::Sup`]).
//! * **End notes**: a "Notes" chapter whose entries start with the same
//!   raised numbers. Notes are numbered 1, 2, 3 ... through the book, so we
//!   walk the text in reading order and match marker *n* to the next
//!   expected note (looking a few notes ahead, so one missed marker does not
//!   stop everything after it). Each note moves to the end of the chapter
//!   that cites it.
//! * **Page footnotes**: notes printed at the bottom of a page match
//!   markers on the same page.
//! * **References**: a "References"/"Bibliography" chapter of `[n] ...`
//!   entries; every `[n]` in the text links to its entry, which is listed at
//!   the end of each chapter that cites it.
//!
//! Every link goes both ways: marker -> note and note -> marker (↩).

use crate::{key, plain, Block, Book, Chapter, Inline, Note};
use std::collections::HashMap;

/// Split "[12] rest" into (12, rest).
fn bracket_number(t: &str) -> Option<(u32, &str)> {
    let r = t.trim_start().strip_prefix('[')?;
    let (num, rest) = r.split_once(']')?;
    if num.is_empty() || num.len() > 3 || !num.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    Some((num.parse().ok()?, rest))
}

/// Note entries of a Notes chapter: a paragraph starting with a raised
/// number starts an entry; following paragraphs and equations belong to it.
fn parse_notes(blocks: Vec<Block>) -> Vec<Note> {
    let mut notes: Vec<Note> = Vec::new();
    for b in blocks {
        match b {
            Block::Para(mut inl) => {
                let start = match inl.first() {
                    Some(Inline::Sup(s)) if s.bytes().all(|c| c.is_ascii_digit()) => Some(s.clone()),
                    _ => None,
                };
                if let Some(label) = start {
                    inl.remove(0);
                    if let Some(Inline::Text(t)) = inl.first_mut() {
                        *t = t.trim_start().to_string();
                    }
                    notes.push(Note { label, blocks: vec![Block::Para(inl)], ..Default::default() });
                } else if let Some(n) = notes.last_mut() {
                    n.blocks.push(Block::Para(inl));
                }
            }
            Block::Equation { .. } => {
                if let Some(n) = notes.last_mut() {
                    n.blocks.push(b);
                }
            }
            _ => {}
        }
    }
    notes
}

/// Reference entries "[n] text" with continuation paragraphs.
fn parse_refs(blocks: Vec<Block>) -> Vec<(u32, Vec<Block>)> {
    let mut refs: Vec<(u32, Vec<Block>)> = Vec::new();
    for b in blocks {
        match b {
            Block::Para(mut inl) => {
                let hit = match inl.first() {
                    Some(Inline::Text(t)) => bracket_number(t).map(|(n, rest)| (n, rest.trim_start().to_string())),
                    _ => None,
                };
                if let Some((n, rest)) = hit {
                    inl[0] = Inline::Text(rest);
                    refs.push((n, vec![Block::Para(inl)]));
                } else if let Some(r) = refs.last_mut() {
                    r.1.push(Block::Para(inl));
                }
            }
            Block::Equation { .. } => {
                if let Some(r) = refs.last_mut() {
                    r.1.push(b);
                }
            }
            _ => {}
        }
    }
    refs
}

/// Positions of a longest strictly increasing subsequence of `v`.
fn longest_increasing(v: &[usize]) -> Vec<usize> {
    let n = v.len();
    let mut len = vec![1usize; n];
    let mut prev = vec![usize::MAX; n];
    for i in 0..n {
        for j in 0..i {
            if v[j] < v[i] && len[j] + 1 > len[i] {
                len[i] = len[j] + 1;
                prev[i] = j;
            }
        }
    }
    let Some(mut i) = (0..n).max_by_key(|&i| (len[i], std::cmp::Reverse(i))) else {
        return Vec::new();
    };
    let mut out = vec![i];
    while prev[i] != usize::MAX {
        i = prev[i];
        out.push(i);
    }
    out.reverse();
    out
}

fn blocks_text(blocks: &[Block]) -> String {
    blocks
        .iter()
        .map(|b| match b {
            Block::Para(i) => plain(i),
            Block::Equation { latex, .. } => format!("$${latex}$$"),
            _ => String::new(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Remove the Notes and References chapters, link everything, and return
/// the keys of the removed chapter titles (their TOC entries are dropped).
pub fn attach(chapters: &mut Vec<Chapter>, book: &mut Book) -> Vec<String> {
    let pdf = book.pdf_pages.clone();
    let mut removed = Vec::new();
    let mut notes: Vec<Note> = Vec::new();
    let mut refs: Vec<(u32, Vec<Block>)> = Vec::new();
    let mut i = 1;
    while i < chapters.len() {
        let k = key(&chapters[i].title);
        let is_notes = matches!(k.as_str(), "notes" | "footnotes" | "endnotes");
        let is_refs = matches!(k.as_str(), "references" | "bibliography");
        if is_notes || is_refs {
            let ch = chapters.remove(i);
            let blocks: Vec<Block> = ch.blocks.into_iter().map(|b| b.block).collect();
            if is_notes {
                notes.extend(parse_notes(blocks));
            } else {
                refs.extend(parse_refs(blocks));
            }
            removed.push(k);
        } else {
            i += 1;
        }
    }

    // ---- footnotes ---------------------------------------------------------
    // Page footnotes first: a marker matches a note printed on its own page.
    for (ci, ch) in chapters.iter_mut().enumerate() {
        // (by page, in order, so unmatched notes are listed the same way every run)
        let mut page_notes: std::collections::BTreeMap<usize, Vec<(String, Vec<Inline>, bool)>> = Default::default();
        ch.blocks.retain(|b| match &b.block {
            Block::PageNote { label, text } => {
                page_notes.entry(b.page).or_default().push((label.clone(), text.clone(), false));
                false
            }
            _ => true,
        });
        if page_notes.is_empty() {
            continue;
        }
        let mut count = 0usize;
        for b in ch.blocks.iter_mut() {
            let page = b.page;
            // (a marker can sit in a section heading too)
            let (Block::Para(inl) | Block::Heading { text: inl, .. }) = &mut b.block else { continue };
            for k in 0..inl.len() {
                let Inline::Sup(s) = &inl[k] else { continue };
                let s = s.clone();
                // the note on the marker's own page, or on the page the
                // marker links to (notes gathered at the end of the chapter)
                let linked = match inl.get(k + 1) {
                    Some(Inline::PageLink(p)) => Some(*p),
                    _ => None,
                };
                let has_note = |p: usize| page_notes.get(&p).is_some_and(|l| l.iter().any(|e| e.0 == s && !e.2));
                let same_page = has_note(page);
                // (a paragraph that crosses to the next pages: the marker
                // may stand after the page turned, which its page marks say)
                let turned = page + inl[..k].iter().filter(|i| matches!(i, Inline::Anchor(a) if crate::page_label(a).is_some())).count();
                let note_page = if same_page {
                    page
                } else if turned != page && has_note(turned) {
                    turned
                } else if let Some(p) = linked {
                    let Some(q) = page_notes.iter().find(|(q, l)| pdf.get(**q) == Some(&p) && l.iter().any(|e| e.0 == s && !e.2)).map(|(q, _)| *q) else { continue };
                    q
                } else {
                    continue;
                };
                let Some(list) = page_notes.get_mut(&note_page) else { continue };
                let x = &mut inl[k];
                if let Some(entry) = list.iter_mut().find(|e| e.0 == s && !e.2) {
                    entry.2 = true;
                    count += 1;
                    let id = format!("fn-{ci}-{count}");
                    let back = format!("fnref-{ci}-{count}");
                    ch.notes.push(Note {
                        id: id.clone(),
                        label: s.clone(),
                        blocks: vec![Block::Para(entry.1.clone())],
                        backrefs: vec![back.clone()],
                    });
                    *x = Inline::NoteRef { label: s, id: back, href: format!("#{id}") };
                }
            }
        }
        for (_, list) in page_notes {
            for (label, text, used) in list {
                if !used {
                    book.unplaced.push(format!("Footnote {label} ({}): {}", ch.title, plain(&text)));
                    book.warnings.push(format!("page footnote {label} in {} has no marker", ch.title));
                }
            }
        }
    }

    // End notes are numbered 1, 2, 3 ... in reading order. Collect every
    // raised number that names a note, then keep the longest run whose
    // numbers increase: that drops look-alikes such as the 16 in "O¹⁶"
    // that would otherwise steal note 16 from its real marker.
    let note_index: HashMap<&str, usize> =
        notes.iter().enumerate().map(|(k, n)| (n.label.as_str(), k)).collect();
    let mut cands: Vec<(usize, usize, usize, usize)> = Vec::new(); // chapter, block, inline, note
    for (ci, ch) in chapters.iter().enumerate() {
        if key(&ch.title) == "index" {
            continue;
        }
        for (bi, b) in ch.blocks.iter().enumerate() {
            let Block::Para(inl) = &b.block else { continue };
            for (ii, x) in inl.iter().enumerate() {
                if let Inline::Sup(s) = x {
                    if let Some(&k) = note_index.get(s.as_str()) {
                        cands.push((ci, bi, ii, k));
                    }
                }
            }
        }
    }
    let keep = longest_increasing(&cands.iter().map(|c| c.3).collect::<Vec<_>>());
    let mut placed = vec![false; notes.len()];
    for &c in &keep {
        let (ci, bi, ii, k) = cands[c];
        let label = notes[k].label.clone();
        let id = format!("fn-{label}");
        let back = format!("fnref-{label}");
        let mut n = notes[k].clone();
        n.id = id.clone();
        n.backrefs = vec![back.clone()];
        chapters[ci].notes.push(n);
        placed[k] = true;
        if let Block::Para(inl) = &mut chapters[ci].blocks[bi].block {
            inl[ii] = Inline::NoteRef { label, id: back, href: format!("#{id}") };
        }
    }
    for (k, n) in notes.iter().enumerate() {
        if !placed[k] {
            book.unplaced.push(format!("Footnote {}: {}", n.label, blocks_text(&n.blocks)));
            book.warnings.push(format!("footnote {} has no marker in the text", n.label));
        }
    }

    // ---- references ----------------------------------------------------------
    let refs: HashMap<u32, Vec<Block>> = refs.into_iter().collect();
    let mut cited: HashMap<u32, bool> = refs.keys().map(|k| (*k, false)).collect();
    if !refs.is_empty() {
        for ch in chapters.iter_mut() {
            if key(&ch.title) == "index" {
                continue;
            }
            let mut counts: HashMap<u32, usize> = HashMap::new();
            let mut order: Vec<u32> = Vec::new();
            let mut backs: HashMap<u32, Vec<String>> = HashMap::new();
            let mut cite = |inl: &mut Vec<Inline>| {
                let old = std::mem::take(inl);
                for x in old {
                    let Inline::Text(t) = x else {
                        inl.push(x);
                        continue;
                    };
                    let mut rest: &str = &t;
                    let mut buf = String::new();
                    while let Some(p) = rest.find('[') {
                        buf.push_str(&rest[..p]);
                        match bracket_number(&rest[p..]).filter(|(n, _)| refs.contains_key(n)) {
                            Some((n, after)) => {
                                if !buf.is_empty() {
                                    inl.push(Inline::Text(std::mem::take(&mut buf)));
                                }
                                let k = counts.entry(n).or_insert(0);
                                *k += 1;
                                if *k == 1 {
                                    order.push(n);
                                }
                                let id = format!("refback-{n}-{k}");
                                backs.entry(n).or_default().push(id.clone());
                                cited.insert(n, true);
                                inl.push(Inline::Cite {
                                    label: n.to_string(),
                                    id,
                                    href: format!("#ref-{n}"),
                                });
                                rest = after;
                            }
                            None => {
                                buf.push('[');
                                rest = &rest[p + 1..];
                            }
                        }
                    }
                    buf.push_str(rest);
                    if !buf.is_empty() {
                        inl.push(Inline::Text(buf));
                    }
                }
            };
            for b in ch.blocks.iter_mut() {
                if let Block::Para(inl) | Block::Item { text: inl, .. } = &mut b.block {
                    cite(inl);
                }
            }
            for n in ch.notes.iter_mut() {
                for b in n.blocks.iter_mut() {
                    if let Block::Para(inl) = b {
                        cite(inl);
                    }
                }
            }
            for n in order {
                ch.refs.push(Note {
                    id: format!("ref-{n}"),
                    label: format!("[{n}]"),
                    blocks: refs[&n].clone(),
                    backrefs: backs.remove(&n).unwrap_or_default(),
                });
            }
        }
        let mut keys: Vec<u32> = cited.iter().filter(|(_, c)| !**c).map(|(k, _)| *k).collect();
        keys.sort();
        for n in keys {
            book.unplaced.push(format!("[{n}] {}", blocks_text(&refs[&n])));
            book.warnings.push(format!("reference [{n}] is never cited"));
        }
    }
    removed
}
