//! Link annotations read straight from the PDF (with lopdf). pdftohtml
//! tells which PDF page a link goes to, but not where on that page; an
//! ebook's index links go to the exact spot where each printed page began,
//! and that position is what places the printed page markers.

use std::collections::HashMap;
use std::path::Path;

use lopdf::{Document, Object};

/// One internal link: where it sits on its page and where it points, in
/// pdftohtml's pixels (1.5 per point, measured from the top of the page).
#[derive(Debug, Clone, Copy)]
pub struct LinkAnnot {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    /// PDF page number it goes to (1 = first sheet)
    pub dest_page: u32,
    /// height on that page it goes to, if the link says
    pub dest_top: Option<f32>,
}

/// The internal links of every page, by page number. Empty if the PDF
/// cannot be read this way (the conversion then simply goes without).
pub fn read_links(path: &Path) -> HashMap<u32, Vec<LinkAnnot>> {
    let mut out: HashMap<u32, Vec<LinkAnnot>> = HashMap::new();
    let Ok(doc) = Document::load(path) else { return out };
    let pages = doc.get_pages();
    let number_of: HashMap<lopdf::ObjectId, u32> = pages.iter().map(|(n, id)| (*id, *n)).collect();
    let height_of = |id: lopdf::ObjectId| -> f32 {
        doc.get_dictionary(id)
            .ok()
            .and_then(|d| d.get(b"MediaBox").ok())
            .and_then(|b| doc.dereference(b).ok().map(|(_, o)| o.clone()))
            .and_then(|o| o.as_array().ok().cloned())
            .and_then(|a| a.get(3).and_then(num))
            .unwrap_or(792.0)
    };
    for (&n, &page_id) in &pages {
        let Ok(annots) = doc.get_page_annotations(page_id) else { continue };
        let h = height_of(page_id);
        for a in annots {
            if a.get(b"Subtype").ok().and_then(|s| s.as_name().ok()) != Some(b"Link".as_slice()) {
                continue;
            }
            let Some(rect) = a.get(b"Rect").ok().and_then(|r| r.as_array().ok()).filter(|r| r.len() == 4) else { continue };
            let Some(r) = rect.iter().map(num).collect::<Option<Vec<f32>>>() else { continue };
            // the destination: /Dest [page /XYZ left top zoom], or /A << /S /GoTo /D [...] >>
            let dest = a
                .get(b"Dest")
                .ok()
                .cloned()
                .or_else(|| {
                    let act = a.get(b"A").ok()?;
                    let act = doc.dereference(act).ok()?.1.as_dict().ok()?.clone();
                    act.get(b"D").ok().cloned()
                });
            let Some(dest) = dest.and_then(|d| doc.dereference(&d).ok().map(|(_, o)| o.clone())) else { continue };
            let Ok(arr) = dest.as_array() else { continue };
            let Some(Object::Reference(target)) = arr.first() else { continue };
            let Some(&dest_page) = number_of.get(target) else { continue };
            let dh = height_of(*target);
            let dest_top = match arr.get(1).and_then(|o| o.as_name().ok()) {
                Some(b"XYZ") => arr.get(3).and_then(num).map(|y| (dh - y) * 1.5),
                Some(b"FitH") | Some(b"FitBH") => arr.get(2).and_then(num).map(|y| (dh - y) * 1.5),
                _ => None,
            };
            let (x1, y1, x2, y2) = (r[0].min(r[2]), r[1].min(r[3]), r[0].max(r[2]), r[1].max(r[3]));
            out.entry(n).or_default().push(LinkAnnot {
                left: x1 * 1.5,
                right: x2 * 1.5,
                top: (h - y2) * 1.5,
                bottom: (h - y1) * 1.5,
                dest_page,
                dest_top,
            });
        }
    }
    out
}

fn num(o: &Object) -> Option<f32> {
    match o {
        Object::Integer(i) => Some(*i as f32),
        Object::Real(r) => Some(*r),
        _ => None,
    }
}
