//! Split the block stream into chapters.
//!
//! A chapter starts at every heading set in the chapter-title font size
//! (the most common large size in the book, see `blocks::stats`). Everything
//! before the first one is the front matter (title page, table of contents).

use crate::{plain, Blk, Block, Chapter};

pub fn split_chapters(blks: Vec<Blk>, chapter_size: Option<f32>) -> Vec<Chapter> {
    let mut chapters = vec![Chapter {
        title: "Front Matter".into(),
        ..Default::default()
    }];
    for b in blks {
        let is_title = match &b.block {
            Block::Heading { size, .. } => chapter_size.is_some_and(|c| (size - c).abs() < 0.6),
            _ => false,
        };
        if !is_title {
            chapters.last_mut().unwrap().blocks.push(b);
            continue;
        }
        let Block::Heading { text, .. } = &b.block else { unreachable!() };
        // The page anchor written just before the title belongs to the new
        // chapter, not to the end of the previous one. (Only that one: the
        // anchors before it are of empty pages, which close the chapter
        // before, as the empty page after a part's title page does.)
        let prev = chapters.last_mut().unwrap();
        let mut moved = Vec::new();
        // (the title is the first thing in the columns of its page: they
        // open under it)
        if matches!(prev.blocks.last(), Some(Blk { block: Block::Columns(_), .. })) {
            moved.push(prev.blocks.pop().unwrap());
        }
        if matches!(prev.blocks.last(), Some(Blk { block: Block::Anchor(_), .. })) {
            moved.insert(0, prev.blocks.pop().unwrap());
        }
        // (a title set in lines of its own, as on the title page of a
        // part, is written as it is set: in its lines, centered if it is)
        let set = text.iter().any(|i| matches!(i, crate::Inline::Break));
        chapters.push(Chapter {
            title: plain(text),
            heading: if set { text.clone() } else { Vec::new() },
            centered: set && b.centered,
            blocks: moved,
            ..Default::default()
        });
    }
    // Headings whose level was not decided are sections.
    for ch in chapters.iter_mut() {
        for b in ch.blocks.iter_mut() {
            if let Block::Heading { level, .. } = &mut b.block {
                if *level == 0 {
                    *level = 2;
                }
            }
        }
    }
    chapters
}

pub fn slugify(s: &str) -> String {
    let mut out = String::new();
    for c in s.to_lowercase().chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c);
        } else if !out.ends_with('-') && !out.is_empty() {
            out.push('-');
        }
    }
    out.trim_end_matches('-').to_string()
}

/// The number a chapter has in the book: its title starts with it
/// ("3 Theory of …", "Chapter 3"), or its sections do ("3.1 Rotations").
/// Appendices (sections "A.1") and unnumbered chapters have none.
pub fn chapter_number(ch: &Chapter) -> Option<u32> {
    let number = |t: &str| -> Option<u32> {
        let t = t.trim_start();
        let t = t.strip_prefix("Chapter ").or_else(|| t.strip_prefix("CHAPTER ")).unwrap_or(t);
        let digits: String = t.chars().take_while(char::is_ascii_digit).collect();
        let rest = &t[digits.len()..];
        (!digits.is_empty() && digits.len() <= 3 && (rest.is_empty() || rest.starts_with([' ', '.', ':']))).then(|| digits.parse().ok())?
    };
    if let Some(n) = number(&ch.title).filter(|_| ch.title.chars().any(char::is_alphabetic) || ch.title.trim().len() <= 3) {
        return Some(n);
    }
    ch.blocks.iter().find_map(|b| match &b.block {
        Block::Heading { text, .. } => {
            let t = plain(text);
            let first = t.split_whitespace().next()?;
            let (c, sub) = first.split_once('.')?;
            (sub.starts_with(|x: char| x.is_ascii_digit()) && c.bytes().all(|x| x.is_ascii_digit())).then(|| c.parse().ok())?
        }
        _ => None,
    })
}

/// In a book with numbered chapters, everything before chapter 1 (title
/// pages, contents, prefaces) is one chapter, the front matter: each part
/// keeps its title as a heading. Then the files can be numbered like the
/// chapters (see [`assign_slugs`]).
pub fn merge_front_matter(chapters: &mut Vec<Chapter>, chapter_size: Option<f32>) {
    let Some(first) = chapters.iter().position(|c| chapter_number(c).is_some()) else { return };
    if first < 2 || !numbers_in_order(&chapters[first..]) {
        return;
    }
    let rest = chapters.split_off(first);
    let mut front = chapters.remove(0);
    for ch in chapters.drain(..) {
        // (page anchors that lead a part stay in front of its title)
        let lead = ch.blocks.iter().take_while(|b| matches!(b.block, Block::Anchor(_))).count();
        let page = ch.blocks.first().map_or(front.blocks.last().map_or(0, |b| b.page), |b| b.page);
        let mut blocks = ch.blocks.into_iter();
        front.blocks.extend(blocks.by_ref().take(lead));
        front.blocks.push(Blk {
            block: Block::Heading { level: 1, size: chapter_size.unwrap_or(0.0), id: String::new(), text: vec![crate::Inline::Text(ch.title)] },
            page,
            centered: false,
        });
        front.blocks.extend(blocks);
    }
    chapters.push(front);
    chapters.extend(rest);
}

/// The table of contents is a file of its own: the "Contents" heading
/// and the entries under it leave the chapter they are in (the front
/// matter), and stand right after it.
pub fn split_contents(chapters: &mut Vec<Chapter>) {
    let found = chapters.iter().enumerate().find_map(|(ci, ch)| {
        let at = ch.blocks.iter().position(|b| matches!(&b.block, Block::Heading { text, .. } if crate::blocks::is_contents_title(&plain(text))))?;
        // the entries under it, with the page anchors between them
        let rest = &ch.blocks[at + 1..];
        let last = rest.iter().take_while(|b| matches!(b.block, Block::Toc(_) | Block::Anchor(_))).enumerate().filter(|(_, b)| matches!(b.block, Block::Toc(_))).map(|(k, _)| k).last()?;
        Some((ci, at, at + 1 + last))
    });
    let Some((ci, at, last)) = found else { return };
    // (the anchor of the page the heading stands on goes with it)
    let from = if at > 0 && matches!(chapters[ci].blocks[at - 1].block, Block::Anchor(_)) { at - 1 } else { at };
    let mut blocks: Vec<Blk> = chapters[ci].blocks.drain(from..=last).collect();
    let heading = blocks.remove(at - from);
    let title = match &heading.block {
        Block::Heading { text, .. } => plain(text),
        _ => "Contents".into(),
    };
    if chapters[ci].blocks.iter().all(|b| matches!(b.block, Block::Anchor(_))) && ci > 0 {
        // the chapter held nothing else: it is the contents
        chapters[ci].title = title;
        chapters[ci].blocks.splice(0..0, blocks);
        return;
    }
    chapters.insert(ci + 1, Chapter { title, blocks, ..Default::default() });
}

/// The numbered chapters come in rising order (else the numbers were
/// misread, and they are not used).
fn numbers_in_order(chapters: &[Chapter]) -> bool {
    let n: Vec<u32> = chapters.iter().filter_map(chapter_number).collect();
    n.first().is_some_and(|f| *f <= 1) && n.windows(2).all(|w| w[1] > w[0])
}

/// File names: the number in front is the chapter's number in the book
/// ("03-theory-of-angular-momentum" is chapter 3), 00 is the front matter.
/// What follows the last numbered chapter (appendices, index) counts on
/// from it; an unnumbered part between two chapters takes the number
/// before it with a letter ("03b-…"). A book without numbered chapters is
/// numbered in order.
pub fn assign_slugs(chapters: &mut [Chapter]) {
    let numbers: Vec<Option<u32>> = chapters.iter().map(chapter_number).collect();
    let first = numbers.iter().position(Option::is_some);
    // (chapter 1 comes first, after the front matter and its contents)
    let contents = |c: &Chapter| crate::blocks::is_contents_title(&c.title);
    let usable = first.is_some_and(|f| {
        numbers_in_order(&chapters[f..]) && ((f <= 1 && numbers[f] == Some(f as u32)) || (f == 2 && numbers[f] == Some(1) && contents(&chapters[1])))
    });
    if !usable {
        // (the contents, a file of their own after the front matter, are
        // "00b": the chapters keep the numbers they have without them)
        let mut n = 0;
        for (i, ch) in chapters.iter_mut().enumerate() {
            if i == 1 && crate::blocks::is_contents_title(&ch.title) {
                ch.slug = format!("00b-{}", slugify(&ch.title));
                continue;
            }
            ch.slug = format!("{:02}-{}", n, slugify(&ch.title));
            n += 1;
        }
        return;
    }
    let last = numbers.iter().rposition(Option::is_some).unwrap_or(0);
    let (mut current, mut letter) = (0u32, b'a');
    for (i, ch) in chapters.iter_mut().enumerate() {
        let prefix = match numbers[i] {
            Some(n) => {
                (current, letter) = (n, b'a');
                format!("{n:02}")
            }
            None if i == 0 => "00".to_string(),
            // after the last chapter: counting on
            None if i > last => {
                current += 1;
                format!("{current:02}")
            }
            None => {
                letter += 1;
                format!("{current:02}{}", letter as char)
            }
        };
        ch.slug = format!("{prefix}-{}", slugify(&ch.title));
    }
}

/// How deep a numbered section title is: "1.3 Base Kets" 1, "1.3.1
/// Eigenkets" 2, "A.2 Units" 1; a title without such a number, none (nor
/// a chapter's "2 Quantum Dynamics", a year, or a decimal like "1.5 times").
pub fn section_depth(title: &str) -> Option<usize> {
    let t = title.trim_start();
    let number: &str = t.split_whitespace().next()?;
    let rest = t[number.len()..].trim_start();
    let mut parts = number.trim_end_matches('.').split('.');
    let first = parts.next()?;
    let chapter = (!first.is_empty() && first.len() <= 2 && first.chars().all(|c| c.is_ascii_digit())) || (first.len() == 1 && first.chars().all(|c| c.is_ascii_uppercase()));
    let parts: Vec<&str> = parts.collect();
    let numbered = chapter && !parts.is_empty() && parts.iter().all(|p| !p.is_empty() && p.len() <= 2 && p.chars().all(|c| c.is_ascii_digit()));
    // a title follows the number
    (numbered && rest.starts_with(|c: char| c.is_alphabetic())).then_some(parts.len())
}

/// The level of every section heading (chapter titles keep theirs).
///
/// A book that numbers its sections says the level itself: "1.3 …" is
/// `#`, "1.3.1 …" is `##`. A heading without a number in such a book
/// stands at the level of the numbered headings of its type size; larger
/// than all of them it is `#`, smaller it comes below the deepest.
///
/// Where the chapter titles are set clearly larger than the "1.3 …"
/// headings (a quarter or more), the sections come a level below the
/// chapter: "1.3 …" is `##`. Where they are about the same size, as in a
/// book whose chapter title is hardly larger than its sections, both are `#`.
///
/// A book that does not number its sections keeps the nesting of its
/// table of contents (set by `links::link_toc`); a heading the contents do
/// not list goes by type size, like a listed heading of its size.
pub fn heading_levels(chapters: &mut [Chapter], chapter_size: Option<f32>) {
    // (level, size) of the headings that say their level
    let mut known: Vec<(usize, f32)> = Vec::new();
    let mut numbered = 0;
    for ch in chapters.iter() {
        for (i, b) in ch.blocks.iter().enumerate() {
            if let Block::Heading { size, text, .. } = &b.block {
                if let Some(d) = section_depth(&crate::plain(text)).filter(|_| i > 0) {
                    numbered += 1;
                    if *size > 0.0 {
                        known.push((d, *size));
                    }
                }
            }
        }
    }
    let by_number = numbered >= 5;
    if !by_number {
        known.clear();
        for ch in chapters.iter() {
            for (i, b) in ch.blocks.iter().enumerate() {
                if let Block::Heading { level, size, id, .. } = &b.block {
                    if i > 0 && !id.is_empty() && *size > 0.0 && *level >= 2 {
                        known.push((*level, *size));
                    }
                }
            }
        }
    }
    // the usual type size of each level
    let mut sizes: Vec<(usize, f32)> = Vec::new();
    let deepest = known.iter().map(|k| k.0).max().unwrap_or(0);
    for level in 1..=deepest {
        let mut of: Vec<f32> = known.iter().filter(|k| k.0 == level).map(|k| k.1).collect();
        if !of.is_empty() {
            of.sort_by(f32::total_cmp);
            sizes.push((level, of[of.len() / 2]));
        }
    }
    let by_size = |size: f32| -> Option<usize> {
        if size <= 0.0 || sizes.is_empty() {
            return None;
        }
        if let Some(&(level, _)) = sizes.iter().find(|(_, s)| (s - size).abs() <= 0.6) {
            return Some(level);
        }
        let (top, largest) = sizes.iter().copied().fold((1, 0.0f32), |a, s| if s.1 > a.1 { s } else { a });
        Some(if size > largest { top } else { sizes.iter().filter(|(_, s)| *s > size).map(|(l, _)| *l).max().unwrap_or(deepest) + 1 })
    };
    // sections clearly smaller than the chapter titles: a level below them
    let section = sizes.iter().find(|(l, _)| *l == 1).map(|(_, s)| *s);
    let below = match (by_number, chapter_size, section) {
        (true, Some(c), Some(s)) if c >= 1.25 * s => 1,
        _ => 0,
    };
    for ch in chapters.iter_mut() {
        for (i, b) in ch.blocks.iter_mut().enumerate() {
            let Block::Heading { level, size, id, text } = &mut b.block else { continue };
            // the chapter's own title
            if i == 0 && *level == 1 {
                continue;
            }
            if by_number {
                // (a title in the type of the chapter titles is one of them)
                let like_chapter = below == 1 && chapter_size.is_some_and(|c| *size >= c - 0.6);
                *level = if like_chapter { 1 } else { section_depth(&crate::plain(text)).or_else(|| by_size(*size)).unwrap_or(deepest + 1) + below };
            } else if id.is_empty() {
                if let Some(l) = by_size(*size) {
                    *level = l;
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn chapter(title: &str, section: Option<&str>) -> Chapter {
        let blocks = section
            .map(|s| Blk { block: Block::Heading { level: 2, size: 0.0, id: String::new(), text: vec![crate::Inline::Text(s.into())] }, page: 0, centered: false })
            .into_iter()
            .collect();
        Chapter { title: title.into(), blocks, ..Default::default() }
    }

    #[test]
    fn files_are_numbered_like_the_chapters() {
        let mut chs = vec![
            chapter("Front Matter", None),
            chapter("Contents", None),
            chapter("Preface", None),
            chapter("Fundamental Concepts", Some("1.1 The Stern–Gerlach Experiment")),
            chapter("Quantum Dynamics", Some("2.1 Time Evolution")),
            chapter("Appendix A Units", Some("A.1 Coulomb's Law")),
            chapter("Index", None),
        ];
        merge_front_matter(&mut chs, None);
        assign_slugs(&mut chs);
        let slugs: Vec<&str> = chs.iter().map(|c| c.slug.as_str()).collect();
        assert_eq!(slugs, ["00-front-matter", "01-fundamental-concepts", "02-quantum-dynamics", "03-appendix-a-units", "04-index"]);
        // the merged parts keep their titles as headings
        assert_eq!(chs[0].blocks.len(), 2);

        // no numbered chapters: in order, nothing merged
        let mut chs = vec![chapter("Front Matter", None), chapter("Introduction", None), chapter("The Dirac Theory", None)];
        merge_front_matter(&mut chs, None);
        assign_slugs(&mut chs);
        assert_eq!(chs[2].slug, "02-the-dirac-theory");
    }

    #[test]
    fn section_numbers_say_the_level() {
        assert_eq!(super::section_depth("1.3 Base Kets and Matrix Representations"), Some(1));
        assert_eq!(super::section_depth("1.3.1 Eigenkets of an Observable"), Some(2));
        assert_eq!(super::section_depth("A.2 The SI System"), Some(1));
        assert_eq!(super::section_depth("2 Quantum Dynamics"), None);
        assert_eq!(super::section_depth("Problems"), None);
        assert_eq!(super::section_depth("1.5 times the width"), Some(1)); // a title all the same, if it is a heading
        assert_eq!(super::section_depth("1925 Heisenberg"), None);
        assert_eq!(super::section_depth("3.14"), None);
    }

    #[test]
    fn sections_come_below_a_clearly_larger_chapter_title() {
        let book = || {
            let h = |t: &str, size: f32| Blk { block: Block::Heading { level: 2, size, id: String::new(), text: vec![crate::Inline::Text(t.into())] }, page: 0, centered: false };
            let mut blocks = vec![Blk { block: Block::Para(vec![crate::Inline::Text("Words.".into())]), page: 0, centered: false }];
            for n in 1..=3 {
                blocks.push(h(&format!("1.{n} A Section"), 32.0));
                blocks.push(h(&format!("1.{n}.1 A Subsection"), 25.0));
            }
            blocks.push(h("Problems", 32.0));
            vec![Chapter { title: "One".into(), blocks, ..Default::default() }]
        };
        let levels = |chapters: &[Chapter]| -> Vec<usize> {
            chapters[0].blocks.iter().filter_map(|b| if let Block::Heading { level, .. } = &b.block { Some(*level) } else { None }).collect()
        };
        // chapter titles hardly larger than the sections: the same level
        let mut same = book();
        super::heading_levels(&mut same, Some(37.0));
        assert_eq!(levels(&same), [1, 2, 1, 2, 1, 2, 1]);
        // clearly larger: sections a level below
        let mut below = book();
        super::heading_levels(&mut below, Some(45.0));
        assert_eq!(levels(&below), [2, 3, 2, 3, 2, 3, 2]);
    }
}
