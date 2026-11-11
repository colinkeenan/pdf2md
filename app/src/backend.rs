//! Rust <-> QML bridge. The `Converter` object is usable from QML as
//! `Converter { }` (module `org.pdf2md`). All the real work is in pdf2md-core.
//!
//! Property / method names are single words on purpose, so they are spelled the
//! same in Rust and QML regardless of cxx-qt's naming conversion.

#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, status)]
        #[qproperty(bool, busy)]
        /// How much of the conversion is done, 0 to 1 (by time).
        #[qproperty(f64, progress)]
        /// "About 40 s left", while converting.
        #[qproperty(QString, remaining)]
        /// The steps of the conversion, one per line: state (done, running,
        /// waiting), name, how much of it is done (0 to 1), and how far it is
        /// in words, parted by tabs.
        #[qproperty(QString, steps)]
        /// The conversion takes long enough (over 30 s in all) for the
        /// steps to be shown.
        #[qproperty(bool, detailed)]
        /// Last folder the user browsed (a file:// URL), remembered between runs.
        #[qproperty(QString, folder)]
        type Converter = super::ConverterRust;

        /// Start a conversion on a background thread. Output goes into
        /// `<output>/<name of the input file>/`; if that folder exists,
        /// `replace` moves it to the Trash first, otherwise a new "<name> (1)" is made.
        #[qinvokable]
        fn convert(self: Pin<&mut Converter>, input: &QString, output: &QString, replace: bool);

        /// Does the book folder `<output>/<name of the input file>/` exist?
        #[qinvokable]
        fn exists(self: &Converter, input: &QString, output: &QString) -> bool;

        /// Remember `folder` as the last folder used (saved to disk).
        #[qinvokable]
        fn remember(self: Pin<&mut Converter>, folder: &QString);
    }

    // lets us call back into the QObject from the worker thread
    impl cxx_qt::Threading for Converter {}
}

use core::pin::Pin;
use cxx_qt::Threading;
use cxx_qt_lib::QString;
use pdf2md_core::progress::{self, State, StepView, Tracker};
use pdf2md_core::{convert_pdf, write_book};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct ConverterRust {
    status: QString,
    busy: bool,
    progress: f64,
    remaining: QString,
    steps: QString,
    detailed: bool,
    folder: QString,
}

/// A conversion that takes longer than this many seconds in all (time
/// spent and time left together) shows its steps.
const DETAILED_FROM: f32 = 30.0;

impl Default for ConverterRust {
    fn default() -> Self {
        Self {
            status: QString::default(),
            busy: false,
            progress: 0.0,
            remaining: QString::default(),
            steps: QString::default(),
            detailed: false,
            folder: QString::from(load_folder().as_str()),
        }
    }
}

/// ~/.config/pdf2md/last_folder
fn config_file() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))?;
    Some(base.join("pdf2md").join("last_folder"))
}

fn home_url() -> String {
    format!("file://{}", std::env::var("HOME").unwrap_or_else(|_| "/".into()))
}

fn load_folder() -> String {
    config_file()
        .and_then(|p| std::fs::read_to_string(p).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty() && url_to_path(s).is_dir())
        .unwrap_or_else(home_url)
}

/// QML gives "file:///home/me/My%20Book.pdf"; turn that into a real path.
fn url_to_path(s: &str) -> PathBuf {
    let s = s.strip_prefix("file://").unwrap_or(s);
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).ok();
            if let Some(v) = hex.and_then(|h| u8::from_str_radix(h, 16).ok()) {
                out.push(v);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    PathBuf::from(String::from_utf8_lossy(&out).into_owned())
}

/// <output>/<pdf file name without extension>/
fn book_dir(input: &Path, output: &Path) -> PathBuf {
    let stem = input
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "book".to_string());
    output.join(stem)
}

/// "<dir> (1)", "<dir> (2)" … whichever does not exist yet.
fn new_dir(dir: &Path) -> PathBuf {
    let name = dir.file_name().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
    (1..)
        .map(|n| dir.with_file_name(format!("{name} ({n})")))
        .find(|p| !p.exists())
        .expect("some name is free")
}

/// Move everything inside `dir` into a new sibling folder "<name> (previous)"
/// (numbered if that exists) and return it. `dir` itself is left, empty.
fn move_contents_aside(dir: &Path) -> Result<PathBuf, String> {
    let name = dir.file_name().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
    let aside = (0..)
        .map(|n| dir.with_file_name(if n == 0 { format!("{name} (previous)") } else { format!("{name} (previous {n})") }))
        .find(|p| !p.exists())
        .expect("some name is free");
    std::fs::create_dir(&aside).map_err(|e| format!("could not create {}: {e}", aside.display()))?;
    let entries = std::fs::read_dir(dir).map_err(|e| format!("could not read {}: {e}", dir.display()))?;
    for e in entries.flatten() {
        let to = aside.join(e.file_name());
        std::fs::rename(e.path(), &to).map_err(|err| format!("could not move {}: {err}", e.path().display()))?;
    }
    Ok(aside)
}

/// Replace: the old conversion inside `dir` goes to the Trash as one folder
/// ("<name> (previous)"), and `dir` stays for the new one. If the Trash
/// fails, everything is moved back.
fn trash_contents(dir: &Path) -> Result<(), String> {
    let aside = move_contents_aside(dir)?;
    if let Err(e) = trash::delete(&aside) {
        if let Ok(entries) = std::fs::read_dir(&aside) {
            for en in entries.flatten() {
                let _ = std::fs::rename(en.path(), dir.join(en.file_name()));
            }
        }
        let _ = std::fs::remove_dir(&aside);
        return Err(format!("could not move the old conversion to the Trash: {e}"));
    }
    Ok(())
}

/// A step as a line for the window: state, name, fraction and how far it
/// is in words ("155 of 771 pages, about 5 s left"), parted by tabs.
fn step_line(v: &StepView) -> String {
    let unit = v.step.unit();
    let amount = |n: usize| if unit.is_empty() || v.total == 0 { String::new() } else { format!("{n} {unit}") };
    let (state, words) = match v.state {
        State::Done => {
            let took = progress::duration(v.seconds, false);
            let n = amount(v.total);
            ("done", if n.is_empty() { took } else { format!("{n}, {took}") })
        }
        State::Running => {
            let left = format!("about {} left", progress::duration(v.seconds, true));
            // (nothing counted yet: some programs print all their pages at the end)
            let of = match (unit.is_empty() || v.total == 0, v.done) {
                (true, _) => String::new(),
                (false, 0) => format!("{} {unit}, ", v.total),
                (false, done) => format!("{done} of {} {unit}, ", v.total),
            };
            ("running", format!("{of}{left}"))
        }
        State::Waiting => ("waiting", String::new()),
    };
    format!("{state}\t{}\t{:.3}\t{words}", v.step.label(), v.fraction)
}

fn run(input: &Path, output: &Path, replace: bool) -> Result<String, String> {
    let mut dest = book_dir(input, output);
    // equations corrected by hand in the earlier conversion are kept
    let kept = pdf2md_core::ocr::collect_corrections(&dest);
    // and the reader's notes and prose fixes
    let user = pdf2md_core::userdata::collect(&dest);
    pdf2md_core::userdata::backup(&user, &dest);
    let (notes, fixes) = (user.notes.len() + user.marks.len(), user.fixes.len());
    let mut book = convert_pdf(input)?;
    book.user = user;
    // only touch the old folder once the new book is ready
    if dest.exists() {
        if replace {
            // the old conversion to the Trash (restorable), but the folder
            // itself stays: terminals and editors inside it see the new files
            trash_contents(&dest)?;
        } else {
            dest = new_dir(&dest);
        }
    }
    let files = write_book(&book, &dest).map_err(|e| e.to_string())?;
    let mut msg = format!("Wrote {} files to {}", files.len(), dest.display());
    if kept > 0 {
        msg.push_str(&format!("; kept {kept} hand-corrected equation(s)"));
    }
    if notes + fixes > 0 {
        msg.push_str(&format!("; kept {notes} note(s) and {fixes} fix(es)"));
    }
    if !book.warnings.is_empty() {
        msg.push_str(&format!(" ({} warnings)", book.warnings.len()));
    }
    Ok(msg)
}

impl qobject::Converter {
    pub fn remember(mut self: Pin<&mut Self>, folder: &QString) {
        let s = String::from(folder);
        if let Some(p) = config_file() {
            if let Some(dir) = p.parent() {
                let _ = std::fs::create_dir_all(dir);
            }
            let _ = std::fs::write(p, &s);
        }
        self.as_mut().set_folder(QString::from(s.as_str()));
    }

    pub fn exists(&self, input: &QString, output: &QString) -> bool {
        let input = url_to_path(&String::from(input));
        let output = url_to_path(&String::from(output));
        !input.as_os_str().is_empty() && !output.as_os_str().is_empty() && book_dir(&input, &output).exists()
    }

    pub fn convert(mut self: Pin<&mut Self>, input: &QString, output: &QString, replace: bool) {
        if *self.busy() {
            return;
        }
        let input = url_to_path(&String::from(input));
        let output = url_to_path(&String::from(output));
        if input.as_os_str().is_empty() || output.as_os_str().is_empty() {
            self.as_mut()
                .set_status(QString::from("Choose a book and an output folder first."));
            return;
        }
        self.as_mut().set_busy(true);
        self.as_mut().set_status(QString::from("Converting…"));
        self.as_mut().set_progress(0.0);
        self.as_mut().set_remaining(QString::default());
        self.as_mut().set_steps(QString::default());
        self.as_mut().set_detailed(false);

        // The converter reports its steps to the tracker; a second thread
        // shows what the tracker makes of them, a few times a second (the
        // time left changes also while a step is silent).
        let tracker = Arc::new(Mutex::new(Tracker::new(Instant::now())));
        let over = Arc::new(AtomicBool::new(false));
        {
            let tracker = tracker.clone();
            progress::watch(move |step, done, total| {
                tracker.lock().unwrap_or_else(|e| e.into_inner()).update(step, done, total, Instant::now());
            });
        }
        {
            let (tracker, over, qt_thread) = (tracker.clone(), over.clone(), self.qt_thread());
            std::thread::spawn(move || {
                while !over.load(Ordering::Relaxed) {
                    let now = Instant::now();
                    let (left, spent, fraction, steps) = {
                        let t = tracker.lock().unwrap_or_else(|e| e.into_inner());
                        (t.remaining(now), t.spent(now), t.fraction(now), t.steps(now))
                    };
                    let lines: Vec<String> = steps.iter().map(step_line).collect();
                    let over = over.clone();
                    let _ = qt_thread.queue(move |mut qobj| {
                        // (the conversion may have ended meanwhile)
                        if over.load(Ordering::Relaxed) {
                            return;
                        }
                        qobj.as_mut().set_progress(fraction as f64);
                        qobj.as_mut().set_remaining(QString::from(format!("About {} left", progress::duration(left, true)).as_str()));
                        qobj.as_mut().set_steps(QString::from(lines.join("\n").as_str()));
                        // (the estimate can stay low all the way through a
                        // long conversion: the time spent counts too)
                        if spent + left > DETAILED_FROM {
                            qobj.as_mut().set_detailed(true);
                        }
                    });
                    std::thread::sleep(Duration::from_millis(250));
                }
            });
        }

        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
            // PDF parsers can panic on odd files; report it instead of dying.
            let result = std::panic::catch_unwind(|| run(&input, &output, replace))
                .unwrap_or_else(|_| Err("The PDF reader crashed on this file.".to_string()));
            progress::unwatch();
            over.store(true, Ordering::Relaxed);
            let msg = match result {
                Ok(m) => m,
                Err(e) => format!("Failed: {e}"),
            };
            // how the steps ended, with the time each took
            let now = Instant::now();
            let lines: Vec<String> = tracker.lock().unwrap_or_else(|e| e.into_inner()).steps(now).iter().map(step_line).collect();
            let _ = qt_thread.queue(move |mut qobj| {
                qobj.as_mut().set_steps(QString::from(lines.join("\n").as_str()));
                qobj.as_mut().set_progress(1.0);
                qobj.as_mut().set_remaining(QString::default());
                qobj.as_mut().set_status(QString::from(msg.as_str()));
                qobj.as_mut().set_busy(false);
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_keeps_the_folder() {
        let tmp = std::env::temp_dir().join(format!("pdf2md-aside-{}", std::process::id()));
        let book = tmp.join("My Book");
        std::fs::create_dir_all(book.join("images")).unwrap();
        std::fs::write(book.join("01-a.md"), "old").unwrap();
        std::fs::write(book.join(".pdf2md-written"), "k\tx").unwrap();
        std::fs::write(book.join("images/fig.png"), "png").unwrap();
        let aside = move_contents_aside(&book).unwrap();
        assert_eq!(aside, tmp.join("My Book (previous)"));
        assert!(book.is_dir() && std::fs::read_dir(&book).unwrap().next().is_none());
        assert_eq!(std::fs::read_to_string(aside.join("01-a.md")).unwrap(), "old");
        assert!(aside.join(".pdf2md-written").exists() && aside.join("images/fig.png").exists());
        // a second time: a numbered name
        std::fs::write(book.join("01-a.md"), "newer").unwrap();
        assert_eq!(move_contents_aside(&book).unwrap(), tmp.join("My Book (previous 1)"));
        std::fs::remove_dir_all(&tmp).unwrap();
    }

    #[test]
    fn keep_both_picks_the_next_free_name() {
        let tmp = std::env::temp_dir().join(format!("pdf2md-test-{}", std::process::id()));
        let book = tmp.join("My Book");
        std::fs::create_dir_all(&book).unwrap();
        assert_eq!(book_dir(Path::new("/x/My Book.pdf"), &tmp), book);
        assert_eq!(new_dir(&book), tmp.join("My Book (1)"));
        std::fs::create_dir_all(tmp.join("My Book (1)")).unwrap();
        assert_eq!(new_dir(&book), tmp.join("My Book (2)"));
        std::fs::remove_dir_all(&tmp).unwrap();
    }

    #[test]
    fn a_step_is_told_in_a_line() {
        use pdf2md_core::progress::Step;
        let mut v = StepView { step: Step::Text, state: State::Running, fraction: 0.25, done: 155, total: 771, seconds: 47.0 };
        assert_eq!(step_line(&v), "running\tReading the text\t0.250\t155 of 771 pages, about 50 s left");
        v.state = State::Done;
        v.seconds = 83.2;
        assert_eq!(step_line(&v), "done\tReading the text\t0.250\t771 pages, 1 min 23 s");
        let w = StepView { step: Step::Files, state: State::Waiting, fraction: 0.0, done: 0, total: 0, seconds: 0.1 };
        assert_eq!(step_line(&w), "waiting\tWriting the files\t0.000\t");
    }
}
