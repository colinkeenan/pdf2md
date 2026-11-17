//! Pages whose content is set sideways (a wide figure turned to fill the
//! page, its caption running down the page): turned upright in a copy of
//! the PDF, which is then read in place of the book's file. The figure,
//! its caption and the formulas in it then come out as on any page.

use std::path::{Path, PathBuf};

/// The pages (1 = first sheet) of `pdftohtml -xml` output whose text is
/// mostly set sideways, and whether it reads down the page (the page was
/// turned to the right) or up it. pdftohtml gives a piece of text that
/// is not upright with no width, and where it begins.
pub fn sideways_pages(xml: &str) -> Vec<(u32, bool)> {
    let attr = |tag: &str, name: &str| -> Option<f32> { tag.split(&format!(" {name}=\"")).nth(1)?.split('"').next()?.parse().ok() };
    let mut out = Vec::new();
    for page in xml.split("<page ").skip(1) {
        let head = format!(" {}", page.split('>').next().unwrap_or(""));
        let Some(number) = attr(&head, "number") else { continue };
        // (left, top) of every sideways piece that holds a letter or figure; the upright ones counted
        let (mut turned, mut upright): (Vec<(f32, f32)>, usize) = (Vec::new(), 0);
        for text in page.split("<text ").skip(1) {
            let tag = format!(" {}", text.split('>').next().unwrap_or(""));
            let body = text.split_once('>').map_or("", |(_, b)| b).split("</text>").next().unwrap_or("");
            if !body.chars().any(char::is_alphanumeric) {
                continue;
            }
            match (attr(&tag, "width"), attr(&tag, "left"), attr(&tag, "top")) {
                (Some(w), Some(l), Some(t)) if w < 0.5 => turned.push((l, t)),
                (Some(_), _, _) => upright += 1,
                _ => {}
            }
        }
        // (a stamp in the margin is a few pieces; a page set sideways has
        // only its running head and foot upright)
        if turned.len() < 15 || turned.len() < 3 * upright {
            continue;
        }
        // down the page: within a line (one `left`), each piece begins under the one before
        let (mut down, mut up) = (0, 0);
        for w in turned.windows(2) {
            if (w[0].0 - w[1].0).abs() <= 3.0 {
                if w[1].1 > w[0].1 {
                    down += 1;
                } else if w[1].1 < w[0].1 {
                    up += 1;
                }
            }
        }
        out.push((number as u32, down >= up));
    }
    out
}

/// A copy of the PDF with those pages turned upright, in the cache.
/// None if it cannot be made (the book is then read as it is).
pub fn upright_copy(path: &Path, pages: &[(u32, bool)]) -> Option<PathBuf> {
    let base = std::env::var_os("XDG_CACHE_HOME").map(PathBuf::from).or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".cache")))?;
    let dir = base.join("pdf2md").join("upright");
    std::fs::create_dir_all(&dir).ok()?;
    let name = path.file_stem()?.to_string_lossy().into_owned();
    let to = dir.join(format!("{name}.pdf"));
    let mut doc = lopdf::Document::load(path).ok()?;
    let ids = doc.get_pages();
    for (number, down) in pages {
        let id = *ids.get(number)?;
        let page = doc.get_object_mut(id).ok()?.as_dict_mut().ok()?;
        let now = page.get(b"Rotate").ok().and_then(|r| r.as_i64().ok()).unwrap_or(0);
        // (text that reads down the page: a quarter turn to the left sets it upright)
        let turn = if *down { 270 } else { 90 };
        page.set("Rotate", lopdf::Object::Integer((now + turn).rem_euclid(360)));
    }
    doc.save(&to).ok()?;
    Some(to)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_page_of_sideways_text_is_found() {
        let piece = |top: u32, t: &str| format!("<text top=\"{top}\" left=\"150\" width=\"0\" height=\"13\" font=\"1\">{t}</text>\n");
        let turned: String = (0..20).map(|k| piece(100 + 20 * k, "word")).collect();
        let xml = format!("<page number=\"1\" height=\"1188\" width=\"918\">\n<text top=\"61\" left=\"54\" width=\"26\" height=\"16\" font=\"0\">240</text>\n<text top=\"61\" left=\"320\" width=\"316\" height=\"16\" font=\"0\">CHAPTER</text>\n</page>\n<page number=\"2\" height=\"1188\" width=\"918\">\n<text top=\"61\" left=\"54\" width=\"26\" height=\"16\" font=\"0\">240</text>\n{turned}</page>\n");
        assert_eq!(sideways_pages(&xml), vec![(2, true)]);
    }
}
