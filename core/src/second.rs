//! A second reading of the displayed equations of a PDF that holds text.
//!
//! In such a PDF (a TeX book) each formula is built by rule from where the
//! letters stand on the page. The letters themselves are sure, the PDF
//! names them; how they stand to each other (what is an exponent, which
//! pieces are the halves of a small fraction, the rows of a matrix) is
//! told from their places, and there the rules go wrong. So the place of
//! each displayed equation is cut out of its page and read by the page
//! reader's recognizer, where that is installed (the environment of
//! scanned books): it reads how a formula is put together well, and
//! single letters less surely than the PDF gives them.
//!
//! * Where it reads the formula the rules built, nothing is done.
//! * Where it reads the same letters and signs, each as often, but put
//!   together otherwise, and reads them so a second time from a larger
//!   picture, its reading is written (the rules' LaTeX is kept: a
//!   correction made by hand of what the rules built still finds its
//!   equation, see [`crate::emit`]).
//! * Where the two differ in more, the equation is listed to check.
//!
//! The readings are kept in `~/.cache/pdf2md/second/`, by PDF, page and
//! place: each place is read once. `PDF2MD_SECOND=0` turns it all off.

use crate::{Block, Book, Place};
use std::path::{Path, PathBuf};

/// The rules by which a verdict is reached (here and in `math_check.py`):
/// a verdict made by older rules is made again from the readings kept.
const RULES: &str = "places 15";

/// What became of an equation's second reading.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Second {
    /// The LaTeX the rules built, where the recognizer's reading is
    /// written in its place.
    pub rule: Option<String>,
    /// The recognizer's reading, where the equation is listed to check.
    pub doubt: Option<String>,
    /// What the place's readings are kept by.
    pub key: String,
}

fn cache_dir() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_CACHE_HOME").map(PathBuf::from).or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".cache")))?;
    Some(base.join("pdf2md").join("second"))
}

fn fnv(bytes: &[u8], mut h: u64) -> u64 {
    for b in bytes {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

/// A print of the PDF: its length and its first and last stretch.
fn pdf_print(pdf: &Path) -> Option<u64> {
    use std::io::{Read, Seek, SeekFrom};
    let mut f = std::fs::File::open(pdf).ok()?;
    let len = f.metadata().ok()?.len();
    let mut buf = vec![0u8; 1 << 16];
    let n = f.read(&mut buf).ok()?;
    let mut h = fnv(&buf[..n], fnv(&len.to_le_bytes(), 0xcbf29ce484222325));
    if len > (1 << 16) {
        f.seek(SeekFrom::End(-(1 << 16))).ok()?;
        let n = f.read(&mut buf).ok()?;
        h = fnv(&buf[..n], h);
    }
    Some(h)
}

/// The equations of a book that were built from a page's letters.
fn equations(book: &mut Book) -> Vec<(&mut String, &mut Place)> {
    let mut out = Vec::new();
    for ch in &mut book.chapters {
        let notes = ch.notes.iter_mut().chain(ch.refs.iter_mut()).flat_map(|n| n.blocks.iter_mut());
        for b in ch.blocks.iter_mut().map(|b| &mut b.block).chain(notes) {
            if let Block::Equation { latex, place: Some(place), .. } = b {
                out.push((latex, place));
            }
        }
    }
    out
}

/// Read the equations of `book` (made from `pdf`, the copy with its
/// sideways pages upright where there is one) a second time, and write
/// the recognizer's reading where the verdict says so.
pub(crate) fn read(book: &mut Book, pdf: &Path) {
    if std::env::var("PDF2MD_SECOND").is_ok_and(|v| v == "0") {
        return;
    }
    let (Some(dir), Some(print)) = (cache_dir(), pdf_print(pdf)) else { return };
    let (shifts, unfolds) = (book.shifts.clone(), book.unfolds.clone());
    let mut eqs = equations(book);
    if eqs.is_empty() {
        return;
    }
    // where each stands on the PDF's page itself, its key and its stamp
    let mut jobs: Vec<(String, u32, [f32; 4], String)> = Vec::with_capacity(eqs.len());
    for (latex, place) in eqs.iter() {
        let [left, top, right, bottom] = place.area;
        let fig = crate::layout::Figure { left, right, top, bottom };
        let fig = unfolds.get(&place.page).map_or(fig, |s| crate::columns::printed(s, &fig));
        let shift = shifts.get(&place.page).copied().unwrap_or(0.0);
        let area = [fig.left + shift, fig.top, fig.right + shift, fig.bottom];
        let at = format!("{print:016x} {} {:.0} {:.0} {:.0} {:.0}", place.page, area[0], area[1], area[2], area[3]);
        let key = format!("{:016x}", fnv(at.as_bytes(), 0xcbf29ce484222325));
        let stamp = format!("{RULES} {:016x}", fnv(latex.as_bytes(), 0xcbf29ce484222325));
        jobs.push((key, place.page, area, stamp));
    }
    let verdict = |key: &str, stamp: &str| -> Option<(String, String)> {
        let t = std::fs::read_to_string(dir.join(format!("{key}.second"))).ok()?;
        let mut lines = t.splitn(3, '\n');
        let (verdict, stamped, reading) = (lines.next()?, lines.next()?, lines.next().unwrap_or(""));
        (stamped == stamp).then(|| (verdict.to_string(), reading.trim().to_string()))
    };
    let todo: Vec<usize> = (0..jobs.len()).filter(|&i| verdict(&jobs[i].0, &jobs[i].3).is_none()).collect();
    // (without the recognizer the equations not yet read stay as the rules
    // built them)
    if let (false, Some(py)) = (todo.is_empty(), crate::scan::python()) {
        let list: String = todo
            .iter()
            .map(|&i| {
                let (key, page, a, stamp) = &jobs[i];
                format!("{key}\t{page}\t{:.1}\t{:.1}\t{:.1}\t{:.1}\t{stamp}\t{}\n", a[0], a[1], a[2], a[3], eqs[i].0.replace(['\n', '\t'], " "))
            })
            .collect();
        let file = dir.join(format!("jobs-{print:016x}.tsv"));
        if std::fs::create_dir_all(&dir).is_ok() && std::fs::write(&file, list).is_ok() {
            let unread = todo.iter().filter(|&&i| !dir.join(format!("{}.surya", jobs[i].0)).exists()).count();
            if unread > 0 {
                eprintln!("Reading {unread} displayed equations a second time, with the page reader's recognizer…");
            }
            let script = dir.join("math_check.py");
            let _ = std::fs::write(&script, crate::ocr::CHECK_SCRIPT);
            let _ = std::fs::write(dir.join("math_ocr.py"), crate::ocr::SCRIPT);
            crate::progress::report(crate::progress::Step::Check, 0, todo.len());
            let ok = crate::ocr::run_recognizer(std::process::Command::new(&py).arg(&script).arg("--places").arg(pdf).arg(&file).arg(&dir));
            let _ = std::fs::remove_file(&file);
            if !ok {
                eprintln!("The second reading failed; the equations it did not reach are as the rules built them.");
            }
        }
    }
    for (i, (latex, place)) in eqs.iter_mut().enumerate() {
        let (key, _, _, stamp) = &jobs[i];
        let mut second = Second { key: key.clone(), ..Default::default() };
        // (the 1 with a doubled stem: the sign itself, which is drawn so
        // where "\mathbb{1}" comes out as a plain 1)
        let verdict = verdict(key, stamp).map(|(v, reading)| (v, reading.replace("\\mathbb{1}", "𝟙").replace("\\mathbb 1", "𝟙").replace("\\mathbbm{1}", "𝟙")));
        match verdict {
            Some((v, reading)) if v == "take" && !reading.is_empty() => {
                second.rule = Some(std::mem::replace(*latex, crate::ocr::tidy_reading(&reading)));
            }
            Some((v, reading)) if v == "check" && !reading.is_empty() => second.doubt = Some(reading),
            _ => {}
        }
        place.second = Some(second);
    }
}
