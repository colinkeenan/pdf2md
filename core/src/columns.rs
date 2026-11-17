//! A document set in two columns (an article) is unfolded into one: on
//! every page the right column goes under the left one, at the left
//! column's place, and what runs across both columns (the title and the
//! abstract, a wide figure, the page's number) keeps its place between
//! them. Everything that reads the pages afterwards sees one column of
//! text on a taller page, with the bars, pictures and link targets moved
//! along with the lines.
//!
//! Where a piece of an unfolded page was on the printed page is kept, so
//! that pictures can still be cut out of the PDF ([`Segment`]).

use crate::layout::{make_line, Doc, Figure, Line, Rule, Span};
use std::collections::HashMap;
use std::path::Path;

/// A stretch of an unfolded page, from `top` down to `bottom`: on the
/// printed page it stands `dx` further right and `dy` higher.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment {
    pub top: f32,
    pub bottom: f32,
    pub dx: f32,
    pub dy: f32,
}

/// Where an area of an unfolded page is on the printed page.
pub fn printed(segments: &[Segment], area: &Figure) -> Figure {
    let mid = (area.top + area.bottom) / 2.0;
    match segments.iter().find(|s| mid >= s.top && mid < s.bottom) {
        Some(s) => Figure { left: area.left + s.dx, right: area.right + s.dx, top: area.top - s.dy, bottom: area.bottom - s.dy },
        None => *area,
    }
}

/// One band of a printed page: lines that run across both columns, or a
/// stretch of the two columns side by side.
struct Band {
    top: f32,
    bottom: f32,
    columns: bool,
    /// where it begins on the unfolded page
    out: f32,
}

struct Plan {
    bands: Vec<Band>,
    height: f32,
}

impl Plan {
    /// Where a point of the printed page goes: (x moved by, y moved by).
    fn moved(&self, x: f32, y: f32, gutter: f32, dx: f32) -> (f32, f32) {
        let Some(b) = self.bands.iter().find(|b| y < b.bottom).or(self.bands.last()) else { return (0.0, 0.0) };
        let right = b.columns && x >= gutter;
        (if right { -dx } else { 0.0 }, b.out - b.top + if right { b.bottom - b.top } else { 0.0 })
    }
}

fn crosses(left: f32, right: f32, gutter: f32) -> bool {
    left < gutter - 3.0 && right > gutter + 3.0
}

/// The gutter of a document whose text is set in two columns, from its
/// pieces of text (per page: the page's width and its pieces): where,
/// about the middle of the page, the fewest pieces run across. None for
/// a document in one column: there, most of the long pieces — its lines
/// — cross the middle; in two columns only a title and an abstract do.
pub fn gutter_of(pages: &[(f32, &[Span])]) -> Option<f32> {
    let width = pages.first()?.0;
    if pages.iter().any(|(w, _)| (w - width).abs() > 2.0) {
        return None;
    }
    let text: Vec<&Span> = pages.iter().flat_map(|(_, spans)| spans.iter()).filter(|s| s.width >= 0.5 && s.text.chars().filter(|c| c.is_alphabetic()).count() >= 2).collect();
    let across = |x: f32| text.iter().filter(|s| crosses(s.left, s.right(), x)).count();
    let (mut best, mut least): (Vec<f32>, usize) = (Vec::new(), usize::MAX);
    let mut x = 0.42 * width;
    while x <= 0.58 * width {
        let n = across(x);
        if n < least {
            (least, best) = (n, vec![x]);
        } else if n == least {
            best.push(x);
        }
        x += 2.0;
    }
    let g = *best.get(best.len() / 2)?;
    // lines of a column: long pieces on one side of it
    let side = |right: bool| text.iter().filter(|s| s.width >= 0.2 * width && !crosses(s.left, s.right(), g) && (s.left >= g) == right).count();
    let (left, right) = (side(false), side(true));
    (left >= 40 && right >= 40 && left <= 5 * right && right <= 5 * left && least * 8 <= left + right).then_some(g)
}

/// The pieces of a page in two columns, in three lots whose lines are
/// found apart: what stands at the height of a piece that runs across
/// the gutter (a line across both columns, with all its pieces), the
/// left column, the right column.
pub fn parts(spans: Vec<Span>, gutter: f32) -> Vec<Vec<Span>> {
    let wide: Vec<(f32, f32)> = spans.iter().filter(|s| s.width >= 0.5 && crosses(s.left, s.right(), gutter)).map(|s| (s.top, s.top + s.height)).collect();
    let (mut across, mut left, mut right) = (Vec::new(), Vec::new(), Vec::new());
    for s in spans {
        let mid = s.top + s.height / 2.0;
        if wide.iter().any(|w| mid >= w.0 - 1.0 && mid <= w.1 + 1.0) {
            across.push(s);
        } else if s.left + s.width / 2.0 < gutter {
            left.push(s);
        } else {
            right.push(s);
        }
    }
    vec![across, left, right].into_iter().filter(|p| !p.is_empty()).collect()
}

/// How far the right column stands from the left one: from the left
/// edges of the lines on each side of the gutter.
fn offset(doc: &Doc, g: f32) -> Option<f32> {
    let edge = |right: bool| {
        let mut lefts: Vec<f32> = doc.pages.iter().flat_map(|p| p.lines.iter()).filter(|l| !l.spans.iter().any(|s| crosses(s.left, s.right(), g)) && (l.left >= g) == right && l.right - l.left >= 0.2 * doc.pages[0].width).map(|l| l.left).collect();
        lefts.sort_by(f32::total_cmp);
        lefts.get(lefts.len() / 20).copied()
    };
    Some(edge(true)? - edge(false)?)
}

fn plan(lines: &[Line], figures: &[Figure], height: f32, gutter: f32) -> Plan {
    // what runs across the gutter: (top, bottom)
    let mut wide: Vec<(f32, f32)> = lines.iter().filter(|l| l.spans.iter().any(|s| crosses(s.left, s.right(), gutter))).map(|l| (l.top, l.top + l.size)).collect();
    wide.extend(figures.iter().filter(|f| crosses(f.left, f.right, gutter) && f.left < gutter - 30.0 && f.right > gutter + 30.0).map(|f| (f.top, f.bottom)));
    let mut marks: Vec<(f32, f32, bool)> = lines.iter().filter(|l| !l.spans.iter().any(|s| crosses(s.left, s.right(), gutter))).map(|l| (l.top, l.top + l.size, true)).collect();
    marks.extend(wide.iter().map(|w| (w.0, w.1, false)));
    marks.sort_by(|a, b| a.0.total_cmp(&b.0));
    // (a line of a column that stands beside a wide picture, within its
    // height, is no band of its own)
    let mut bands: Vec<Band> = Vec::new();
    for (top, bottom, columns) in marks {
        match bands.last_mut() {
            Some(b) if b.columns == columns || top < b.bottom - 1.0 => b.bottom = b.bottom.max(bottom),
            _ => bands.push(Band { top, bottom, columns, out: 0.0 }),
        }
    }
    // each band reaches down to the next one; the first begins at the top
    let tops: Vec<f32> = bands.iter().map(|b| b.top).collect();
    let mut out = 0.0;
    for (k, b) in bands.iter_mut().enumerate() {
        b.top = if k == 0 { 0.0 } else { tops[k] - 2.0 };
        b.bottom = tops.get(k + 1).map_or(height.max(b.bottom + 2.0), |t| t - 2.0);
        b.out = out;
        out += (b.bottom - b.top) * if b.columns { 2.0 } else { 1.0 };
    }
    Plan { bands, height: out.max(height) }
}

/// Unfold the pages of a document set in two columns (see the module's
/// head). Gives, for each page that was unfolded, where its stretches are
/// on the printed page; nothing for a document in one column.
pub fn unfold(doc: &mut Doc, path: &Path) -> HashMap<u32, Vec<Segment>> {
    let mut out = HashMap::new();
    let Some((g, dx)) = doc.gutter.and_then(|g| Some((g, offset(doc, g)?))) else { return out };
    let fonts = doc.fonts.clone();
    let size_of = |s: &Span| fonts.get(s.font).map_or(s.height, |f| f.size);
    let plans: HashMap<u32, Plan> = doc.pages.iter().map(|p| (p.number, plan(&p.lines, &p.figures, p.height, g))).collect();
    // where the links go: a place on the page they lead to, as unfolded
    let links = crate::annots::read_links(path);
    for page in doc.pages.iter_mut() {
        let Some(list) = links.get(&page.number) else { continue };
        for s in page.lines.iter_mut().flat_map(|l| l.spans.iter_mut()) {
            let (Some(n), Some(top)) = (s.link, s.link_top) else { continue };
            let (cx, cy) = ((s.left + s.right()) / 2.0, s.top + s.height / 2.0);
            let left = list.iter().find(|a| a.dest_page == n && cx >= a.left - 2.0 && cx <= a.right + 2.0 && cy >= a.top - 4.0 && cy <= a.bottom + 4.0).and_then(|a| a.dest_left);
            if let (Some(plan), Some(left)) = (plans.get(&n), left) {
                s.link_top = Some(top + plan.moved(left, top, g, dx).1);
            }
        }
    }
    for page in doc.pages.iter_mut() {
        let plan = &plans[&page.number];
        if !plan.bands.iter().any(|b| b.columns) {
            continue;
        }
        let at = |x: f32, y: f32| plan.moved(x, y, g, dx);
        // the lines: one that has pieces in both columns is two lines
        let mut lines: Vec<Line> = Vec::new();
        for l in std::mem::take(&mut page.lines) {
            let wide = l.spans.iter().any(|s| crosses(s.left, s.right(), g));
            let in_columns = plan.bands.iter().find(|b| l.top < b.bottom).is_some_and(|b| b.columns);
            let parts: Vec<Vec<Span>> = if wide || !in_columns {
                vec![l.spans]
            } else {
                let (a, b): (Vec<Span>, Vec<Span>) = l.spans.into_iter().partition(|s| s.left + s.width / 2.0 < g);
                vec![a, b]
            };
            for mut spans in parts.into_iter().filter(|p| !p.is_empty()) {
                // (the whole line moves as its first piece does: an index is not torn off it)
                let (mx, my) = if wide { at(0.0, l.top) } else { at(spans[0].left + spans[0].width / 2.0, l.top) };
                for s in spans.iter_mut() {
                    (s.left, s.top) = (s.left + mx, s.top + my);
                }
                lines.push(make_line(spans, &size_of));
            }
        }
        lines.sort_by(|a, b| a.top.total_cmp(&b.top));
        page.lines = lines;
        let rule = |r: &mut Rule| {
            let (mx, my) = if crosses(r.left, r.right, g) { at(0.0, r.top) } else { at((r.left + r.right) / 2.0, (r.top + r.bottom) / 2.0) };
            (r.left, r.right, r.top, r.bottom) = (r.left + mx, r.right + mx, r.top + my, r.bottom + my);
        };
        let figure = |f: &mut Figure| {
            let (mx, my) = if crosses(f.left, f.right, g) { at(0.0, (f.top + f.bottom) / 2.0) } else { at((f.left + f.right) / 2.0, (f.top + f.bottom) / 2.0) };
            (f.left, f.right, f.top, f.bottom) = (f.left + mx, f.right + mx, f.top + my, f.bottom + my);
        };
        page.rules.iter_mut().for_each(rule);
        page.shades.iter_mut().for_each(|(r, _)| rule(r));
        page.figures.iter_mut().for_each(figure);
        page.frames.iter_mut().for_each(figure);
        for im in page.images.iter_mut() {
            let (mx, my) = if crosses(im.left, im.right, g) { at(0.0, (im.top + im.bottom) / 2.0) } else { at((im.left + im.right) / 2.0, (im.top + im.bottom) / 2.0) };
            (im.left, im.right, im.top, im.bottom) = (im.left + mx, im.right + mx, im.top + my, im.bottom + my);
        }
        for grid in page.grids.iter_mut() {
            let (x0, x1, y0) = (grid.xs[0], grid.xs[grid.xs.len() - 1], grid.ys[0]);
            let (mx, my) = if crosses(x0, x1, g) { at(0.0, y0) } else { at((x0 + x1) / 2.0, y0) };
            grid.xs.iter_mut().for_each(|x| *x += mx);
            grid.ys.iter_mut().for_each(|y| *y += my);
        }
        page.figures.sort_by(|a, b| a.top.total_cmp(&b.top));
        page.turns = plan.bands.iter().filter(|b| b.columns).map(|b| b.out + (b.bottom - b.top)).collect();
        page.unfolded = page.height;
        page.height = plan.height;
        let segments = plan
            .bands
            .iter()
            .flat_map(|b| {
                let h = b.bottom - b.top;
                let first = Segment { top: b.out, bottom: b.out + h, dx: 0.0, dy: b.out - b.top };
                if b.columns { vec![first, Segment { top: b.out + h, bottom: b.out + 2.0 * h, dx, dy: b.out + h - b.top }] } else { vec![first] }
            })
            .collect();
        out.insert(page.number, segments);
    }
    out
}
