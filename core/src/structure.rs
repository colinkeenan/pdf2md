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

pub fn assign_slugs(chapters: &mut [Chapter]) {
    for (i, ch) in chapters.iter_mut().enumerate() {
        ch.slug = format!("{:02}-{}", i, slugify(&ch.title));
    }
}
