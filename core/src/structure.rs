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
        // chapter, not to the end of the previous one.
        let prev = chapters.last_mut().unwrap();
        let mut moved = Vec::new();
        while matches!(prev.blocks.last(), Some(Blk { block: Block::Anchor(_), .. })) {
            moved.push(prev.blocks.pop().unwrap());
        }
        moved.reverse();
        chapters.push(Chapter {
            title: plain(text),
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
    let usable = first.is_some_and(|f| f <= 1 && numbers_in_order(&chapters[f..]) && numbers[f] == Some(f as u32));
    if !usable {
        for (i, ch) in chapters.iter_mut().enumerate() {
            ch.slug = format!("{:02}-{}", i, slugify(&ch.title));
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
}
