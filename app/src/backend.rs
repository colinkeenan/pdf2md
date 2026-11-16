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
        /// The command that shows a book (the user's Markdown reader),
        /// remembered between runs. Empty: the system's own choice.
        #[qproperty(QString, reader)]
        /// How formulas are written: "github" ($`…`$ and ```math, right on
        /// GitHub and in mdBun) or "standard" ($…$ and $$…$$, for most
        /// other readers). Remembered between runs.
        #[qproperty(QString, math)]
        /// The folder of the book converted last in this session (a path);
        /// empty before a conversion has succeeded.
        #[qproperty(QString, last)]
        /// What can be converted in `folder`, by name, one per line: kind
        /// ("pdf", or "course" for a folder of saved pages and PDFs), name,
        /// path, the folder its book goes into unless another is chosen,
        /// the book's folder if it has been converted (else empty), when
        /// that was, and the name of the book's folder; parted by tabs.
        #[qproperty(QString, items)]
        /// The path of the item being converted; empty when none is.
        #[qproperty(QString, current)]
        /// How the items of the last run ended, one per line: path, "ok" or
        /// "failed", and what there is to say; parted by tabs.
        #[qproperty(QString, results)]
        type Converter = super::ConverterRust;

        /// Convert `jobs` one after the other on a background thread. A job
        /// is a line: kind, path of the PDF or course folder, output
        /// folder, math form ("github" or "standard"); parted by tabs. A
        /// book goes into `<output>/<name>/`; if that folder exists,
        /// `replace` moves its files to the Trash first (a course's book
        /// is rebuilt in place), otherwise a new "<name> (1)" is made.
        #[qinvokable]
        fn convertall(self: Pin<&mut Converter>, jobs: &QString, replace: bool);

        /// The book folders of `jobs` that are already there, one per line.
        #[qinvokable]
        fn existing(self: &Converter, jobs: &QString) -> QString;

        /// Remember `folder` as the last folder used (saved to disk).
        #[qinvokable]
        fn remember(self: Pin<&mut Converter>, folder: &QString);

        /// Use `command` to show books from now on (saved to disk).
        #[qinvokable]
        fn usereader(self: Pin<&mut Converter>, command: &QString);

        /// Write formulas as `form` says from now on (saved to disk).
        #[qinvokable]
        fn usemath(self: Pin<&mut Converter>, form: &QString);

        /// Show the book in the folder `book` (a path) with the reader.
        #[qinvokable]
        fn show(self: Pin<&mut Converter>, book: &QString);

        /// Look again for converted books (fills `books`).
        #[qinvokable]
        fn refresh(self: Pin<&mut Converter>);
    }

    // lets us call back into the QObject from the worker thread
    impl cxx_qt::Threading for Converter {}
}

use core::pin::Pin;
use cxx_qt::Threading;
use cxx_qt_lib::QString;
use pdf2md_core::progress::{self, State, StepView, Tracker};
use pdf2md_core::emit::MathForm;
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
    reader: QString,
    math: QString,
    last: QString,
    items: QString,
    current: QString,
    results: QString,
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
            reader: QString::from(read_config("reader").unwrap_or_default().as_str()),
            math: QString::from(math_choice(read_config("math").as_deref(), &read_config("reader").unwrap_or_default()).name()),
            last: QString::default(),
            items: QString::from(item_lines(&url_to_path(&load_folder())).as_str()),
            current: QString::default(),
            results: QString::default(),
        }
    }
}

/// How formulas are written: what the user chose; if nothing was chosen
/// yet, what the reader needs if it is a program known to show math only
/// between "$"; else GitHub's forms.
fn math_choice(chosen: Option<&str>, reader: &str) -> MathForm {
    chosen.and_then(MathForm::named).or_else(|| MathForm::for_reader(reader)).unwrap_or_default()
}

/// ~/.config/pdf2md/<name>
fn config(name: &str) -> Option<PathBuf> {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))?;
    Some(base.join("pdf2md").join(name))
}

/// ~/.config/pdf2md/last_folder
fn config_file() -> Option<PathBuf> {
    config("last_folder")
}

fn read_config(name: &str) -> Option<String> {
    Some(std::fs::read_to_string(config(name)?).ok()?.trim().to_string())
}

fn write_config(name: &str, text: &str) {
    if let Some(p) = config(name) {
        if let Some(dir) = p.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        let _ = std::fs::write(p, text);
    }
}

/// Is this folder a converted book? (The converter leaves a list of what
/// it wrote in it, and the book in one file.)
fn is_book(dir: &Path) -> bool {
    dir.join(".pdf2md-written").is_file()
        || dir.join(".pdf2md-course").is_file()
        || (dir.join("book.md").is_file() && (dir.join("00-front-matter.md").is_file() || dir.join("00a-title-pages.md").is_file()))
}

/// Note that `dir` holds a converted book (~/.config/pdf2md/books: the
/// folders of the books converted, newest first).
fn note_book(dir: &Path) {
    let me = dir.to_string_lossy().into_owned();
    let mut all = vec![me.clone()];
    all.extend(read_config("books").unwrap_or_default().lines().filter(|l| *l != me && !l.is_empty()).map(str::to_string));
    write_config("books", &(all.join("\n") + "\n"));
}

/// "today", "yesterday", "5 days ago", "3 months ago": how long ago.
fn ago(when: std::time::SystemTime, now: std::time::SystemTime) -> String {
    let days = now.duration_since(when).map_or(0, |d| d.as_secs() / 86_400);
    match days {
        0 => "today".into(),
        1 => "yesterday".into(),
        2..=59 => format!("{days} days ago"),
        60..=729 => format!("{} months ago", days / 30),
        _ => format!("{} years ago", days / 365),
    }
}

/// What a job converts: a PDF, or a course (a folder of saved pages and PDFs).
#[derive(Debug, Clone, Copy, PartialEq)]
enum Kind {
    Pdf,
    Course,
}

impl Kind {
    fn name(self) -> &'static str {
        match self {
            Kind::Pdf => "pdf",
            Kind::Course => "course",
        }
    }
}

/// Is this folder a course to make a book of: no Markdown in it yet, but
/// saved pages (HTML) and, in it or its folders, PDFs — more PDFs than pages?
fn is_course(dir: &Path) -> bool {
    let ext = |p: &Path, of: &[&str]| p.extension().is_some_and(|x| of.iter().any(|o| x.eq_ignore_ascii_case(o)));
    let Ok(entries) = std::fs::read_dir(dir) else { return false };
    let (mut pages, mut pdfs) = (0, 0);
    for p in entries.flatten().map(|e| e.path()) {
        if p.is_dir() {
            let inside = std::fs::read_dir(&p).map(|d| d.flatten().map(|e| e.path()).collect::<Vec<_>>()).unwrap_or_default();
            if inside.iter().any(|f| ext(f, &["md"])) {
                return false;
            }
            pdfs += inside.iter().filter(|f| ext(f, &["pdf"])).count();
        } else if ext(&p, &["md"]) {
            return false;
        } else if ext(&p, &["html", "htm"]) {
            pages += 1;
        } else if ext(&p, &["pdf"]) {
            pdfs += 1;
        }
    }
    pages > 0 && pdfs >= pages
}

/// The name of the book's folder: the PDF's name without ".pdf"; a
/// course folder's name without " Downloads" (or with " (book)" where it
/// has no such ending: the book cannot go into the folder it is made of).
fn book_name(kind: Kind, input: &Path) -> String {
    match kind {
        Kind::Pdf => input.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_else(|| "book".to_string()),
        Kind::Course => {
            let name = input.file_name().map(|s| s.to_string_lossy().into_owned()).unwrap_or_else(|| "Course".to_string());
            match name.strip_suffix(" Downloads").map(str::trim).filter(|n| !n.is_empty()) {
                Some(n) => n.to_string(),
                None => format!("{name} (book)"),
            }
        }
    }
}

/// What can be converted in `folder`: its PDFs and its course folders, by name.
fn find_items(folder: &Path) -> Vec<(Kind, PathBuf)> {
    let Ok(entries) = std::fs::read_dir(folder) else { return Vec::new() };
    let mut items: Vec<(Kind, PathBuf)> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| !p.file_name().is_some_and(|n| n.to_string_lossy().starts_with('.')))
        .filter_map(|p| {
            if p.is_dir() {
                is_course(&p).then_some((Kind::Course, p))
            } else {
                p.extension().is_some_and(|x| x.eq_ignore_ascii_case("pdf")).then_some((Kind::Pdf, p))
            }
        })
        .collect();
    items.sort_by_key(|(_, p)| p.file_name().map(|n| n.to_string_lossy().to_lowercase()).unwrap_or_default());
    items
}

/// The items of `folder` as lines for the window (see the `items`
/// property). A book converted before is looked for beside its source,
/// then among the books noted when they were converted (one that went to
/// another folder goes there again).
fn item_lines(folder: &Path) -> String {
    let noted: Vec<PathBuf> = read_config("books").unwrap_or_default().lines().filter(|l| !l.is_empty()).map(PathBuf::from).collect();
    let now = std::time::SystemTime::now();
    find_items(folder)
        .iter()
        .map(|(kind, path)| {
            let name = book_name(*kind, path);
            let beside = folder.join(&name);
            let book = Some(beside).filter(|b| is_book(b)).or_else(|| noted.iter().find(|b| b.file_name().is_some_and(|n| n.to_string_lossy() == name.as_str()) && is_book(b)).cloned());
            let when = book.as_ref().and_then(|b| [".pdf2md-written", ".pdf2md-course", "book.md"].iter().find_map(|f| std::fs::metadata(b.join(f)).and_then(|m| m.modified()).ok()));
            let out = book.as_ref().and_then(|b| b.parent()).unwrap_or(folder);
            format!(
                "{}\t{}\t{}\t{}\t{}\t{}\t{name}",
                kind.name(),
                path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default(),
                path.display(),
                out.display(),
                book.as_ref().map(|b| b.display().to_string()).unwrap_or_default(),
                when.map(|w| ago(w, now)).unwrap_or_default()
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// One thing to convert.
#[derive(Debug, Clone, PartialEq)]
struct Job {
    kind: Kind,
    input: PathBuf,
    output: PathBuf,
    math: MathForm,
}

impl Job {
    /// `<output>/<name of the book>/`
    fn book_dir(&self) -> PathBuf {
        self.output.join(book_name(self.kind, &self.input))
    }
}

/// The jobs the window asks for (see `convertall`); lines that name no
/// input or no output folder are left out.
fn parse_jobs(text: &str) -> Vec<Job> {
    text.lines()
        .filter_map(|l| {
            let f: Vec<&str> = l.split('\t').collect();
            let (input, output) = (url_to_path(f.get(1)?), url_to_path(f.get(2)?));
            (!input.as_os_str().is_empty() && !output.as_os_str().is_empty()).then(|| Job {
                kind: if f[0] == "course" { Kind::Course } else { Kind::Pdf },
                input,
                output,
                math: f.get(3).and_then(|m| MathForm::named(m)).unwrap_or_default(),
            })
        })
        .collect()
}

/// The program and arguments that show the book in `dir` with the
/// reader `command`: its words, with "{folder}" for the book's folder and
/// "{file}" for the whole book in one file; a command that names neither
/// gets the folder as its last argument. An empty command is the
/// system's own choice (`xdg-open`).
fn reader_command(command: &str, dir: &Path) -> Vec<String> {
    let folder = dir.to_string_lossy().into_owned();
    let file = dir.join("book.md").to_string_lossy().into_owned();
    let mut words: Vec<String> = command.split_whitespace().map(|w| w.replace("{folder}", &folder).replace("{file}", &file)).collect();
    if words.is_empty() {
        words.push("xdg-open".into());
    }
    if !command.contains("{folder}") && !command.contains("{file}") {
        words.push(folder);
    }
    words
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

/// A step that has counted nothing new for this many seconds says so,
/// unless that is its usual pace (scanned pages come 16 at a time, a
/// minute apart): then when it is half as long again as usual.
const QUIET_FROM: f32 = 30.0;

/// A step as a line for the window: state, name, fraction and how far it
/// is in words ("155 of 771 pages; 12 s so far, about 5 s left"), parted
/// by tabs. The time so far ticks, so a step that counts nothing for
/// minutes is still seen to be alive.
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
            let times = format!("{} so far, about {} left", progress::elapsed(v.spent), progress::duration(v.seconds, true));
            // (nothing counted yet: some programs print all at the end)
            let mut of = match (unit.is_empty() || v.total == 0, v.done) {
                (true, _) => String::new(),
                (false, 0) => format!("{} {unit}", v.total),
                (false, done) => format!("{done} of {} {unit}", v.total),
            };
            if !of.is_empty() && v.done > 0 && v.quiet >= QUIET_FROM.max(1.5 * v.pace) {
                of.push_str(&format!(" (none new for {})", progress::elapsed(v.quiet)));
            }
            // (the window sets what follows "; " on a line of its own)
            ("running", if of.is_empty() { times } else { format!("{of}; {times}") })
        }
        State::Waiting => ("waiting", String::new()),
    };
    format!("{state}\t{}\t{:.3}\t{words}", v.step.label(), v.fraction)
}

/// Convert; gives back what to tell the user and the book's folder.
fn run(job: &Job, replace: bool) -> Result<(String, PathBuf), String> {
    let mut dest = job.book_dir();
    if job.kind == Kind::Course {
        // (a course's book is rebuilt in place: it keeps the reader's notes
        // itself and removes the files it no longer writes)
        if dest.exists() && !replace {
            dest = new_dir(&dest);
        }
        let files = pdf2md_core::course::write_course(&job.input, &dest)?;
        note_book(&dest);
        return Ok((format!("Wrote {} files to {}", files.len(), dest.display()), dest));
    }
    // equations corrected by hand in the earlier conversion are kept
    let kept = pdf2md_core::ocr::collect_corrections(&dest);
    // and the reader's notes and prose fixes
    let user = pdf2md_core::userdata::collect(&dest);
    pdf2md_core::userdata::backup(&user, &dest);
    let (notes, fixes) = (user.notes.len() + user.marks.len(), user.fixes.len());
    let mut book = convert_pdf(&job.input)?;
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
        msg.push_str(&format!("; kept {kept} hand correction(s) (equations, table cells, pictures under tables folded away)"));
    }
    if notes + fixes > 0 {
        msg.push_str(&format!("; kept {notes} note(s) and {fixes} fix(es)"));
    }
    if !book.warnings.is_empty() {
        msg.push_str(&format!(" ({} warnings)", book.warnings.len()));
    }
    note_book(&dest);
    Ok((msg, dest))
}

/// What to tell the user when all jobs are through: one book's own
/// message; of several, how many were written and what failed.
fn summary(results: &[(String, Result<String, String>)]) -> String {
    let failed: Vec<String> = results.iter().filter_map(|(name, r)| r.as_ref().err().map(|e| format!("{name}: {e}"))).collect();
    match (results, failed.len()) {
        ([(_, Ok(msg))], _) => msg.clone(),
        ([(_, Err(e))], _) => format!("Failed: {e}"),
        (_, 0) => format!("Converted {} books.", results.len()),
        (_, n) => format!("Failed for {n} of {} books. {}", results.len(), failed.join(" · ")),
    }
}

impl qobject::Converter {
    pub fn usereader(mut self: Pin<&mut Self>, command: &QString) {
        let command = String::from(command).trim().to_string();
        let changed = command != String::from(self.reader());
        write_config("reader", &command);
        self.as_mut().set_reader(QString::from(command.as_str()));
        // a reader known to show math only between "$" gets its books
        // written that way (the choice stays open, see `usemath`)
        if changed {
            if let Some(form) = MathForm::for_reader(&command) {
                self.as_mut().usemath(&QString::from(form.name()));
            }
        }
    }

    pub fn usemath(mut self: Pin<&mut Self>, form: &QString) {
        let form = MathForm::named(&String::from(form)).unwrap_or_default();
        write_config("math", form.name());
        self.as_mut().set_math(QString::from(form.name()));
    }

    pub fn show(mut self: Pin<&mut Self>, book: &QString) {
        let dir = PathBuf::from(String::from(book));
        let words = reader_command(&String::from(self.reader()), &dir);
        let started = std::process::Command::new(&words[0])
            .args(&words[1..])
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn();
        match started {
            // (waited for, so that it does not stay behind as a dead process)
            Ok(mut child) => {
                std::thread::spawn(move || {
                    let _ = child.wait();
                });
            }
            Err(e) => self.as_mut().set_status(QString::from(format!("Failed: could not start the reader “{}” ({e}).", words[0]).as_str())),
        }
    }

    pub fn refresh(mut self: Pin<&mut Self>) {
        let items = item_lines(&url_to_path(&String::from(self.folder())));
        self.as_mut().set_items(QString::from(items.as_str()));
    }

    pub fn remember(mut self: Pin<&mut Self>, folder: &QString) {
        let s = String::from(folder);
        if let Some(p) = config_file() {
            if let Some(dir) = p.parent() {
                let _ = std::fs::create_dir_all(dir);
            }
            let _ = std::fs::write(p, &s);
        }
        self.as_mut().set_folder(QString::from(s.as_str()));
        // (what can be converted there)
        self.as_mut().set_items(QString::from(item_lines(&url_to_path(&s)).as_str()));
    }

    pub fn existing(&self, jobs: &QString) -> QString {
        let there: Vec<String> = parse_jobs(&String::from(jobs)).iter().map(Job::book_dir).filter(|d| d.exists()).map(|d| d.display().to_string()).collect();
        QString::from(there.join("\n").as_str())
    }

    pub fn convertall(mut self: Pin<&mut Self>, jobs: &QString, replace: bool) {
        if *self.busy() {
            return;
        }
        let jobs = parse_jobs(&String::from(jobs));
        if jobs.is_empty() {
            self.as_mut().set_status(QString::from("Check a book to convert first."));
            return;
        }
        self.as_mut().set_busy(true);
        self.as_mut().set_last(QString::default());
        self.as_mut().set_results(QString::default());
        self.as_mut().set_status(QString::from("Converting…"));
        self.as_mut().set_progress(0.0);
        self.as_mut().set_remaining(QString::default());
        self.as_mut().set_steps(QString::default());
        self.as_mut().set_detailed(false);

        // The converter reports its steps to the tracker (a new one for
        // each book); a second thread shows what the tracker makes of
        // them, a few times a second (the time left changes also while a
        // step is silent).
        let tracker = Arc::new(Mutex::new(Tracker::new(Instant::now())));
        // (which book of how many: "Book 2 of 5"; empty for one book alone)
        let which = Arc::new(Mutex::new(String::new()));
        let over = Arc::new(AtomicBool::new(false));
        {
            let tracker = tracker.clone();
            progress::watch(move |step, done, total| {
                tracker.lock().unwrap_or_else(|e| e.into_inner()).update(step, done, total, Instant::now());
            });
        }
        {
            let (tracker, which, over, qt_thread) = (tracker.clone(), which.clone(), over.clone(), self.qt_thread());
            std::thread::spawn(move || {
                while !over.load(Ordering::Relaxed) {
                    let now = Instant::now();
                    let (left, spent, fraction, steps) = {
                        let t = tracker.lock().unwrap_or_else(|e| e.into_inner());
                        (t.remaining(now), t.spent(now), t.fraction(now), t.steps(now))
                    };
                    let lines: Vec<String> = steps.iter().map(step_line).collect();
                    let which = which.lock().unwrap_or_else(|e| e.into_inner()).clone();
                    let over = over.clone();
                    let _ = qt_thread.queue(move |mut qobj| {
                        // (the conversion may have ended meanwhile)
                        if over.load(Ordering::Relaxed) {
                            return;
                        }
                        qobj.as_mut().set_progress(fraction as f64);
                        let times = format!("{} so far, about {} left", progress::elapsed(spent), progress::duration(left, true));
                        let times = if which.is_empty() { times } else { format!("{which}: {times}") };
                        qobj.as_mut().set_remaining(QString::from(times.as_str()));
                        qobj.as_mut().set_steps(QString::from(lines.join("\n").as_str()));
                        // (the estimate can stay low all the way through a
                        // long conversion: the time spent counts too)
                        qobj.as_mut().set_detailed(spent + left > DETAILED_FROM);
                    });
                    std::thread::sleep(Duration::from_millis(250));
                }
            });
        }

        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
            let mut results: Vec<(PathBuf, String, Result<String, String>)> = Vec::new();
            let mut last: Option<PathBuf> = None;
            for (n, job) in jobs.iter().enumerate() {
                let name = book_name(job.kind, &job.input);
                *tracker.lock().unwrap_or_else(|e| e.into_inner()) = Tracker::new(Instant::now());
                if jobs.len() > 1 {
                    *which.lock().unwrap_or_else(|e| e.into_inner()) = format!("Book {} of {}", n + 1, jobs.len());
                }
                {
                    let (path, so_far) = (job.input.display().to_string(), result_lines(&results));
                    let status = if jobs.len() > 1 { format!("Converting {name} ({} of {})…", n + 1, jobs.len()) } else { format!("Converting {name}…") };
                    let _ = qt_thread.queue(move |mut qobj| {
                        qobj.as_mut().set_current(QString::from(path.as_str()));
                        qobj.as_mut().set_results(QString::from(so_far.as_str()));
                        qobj.as_mut().set_status(QString::from(status.as_str()));
                    });
                }
                pdf2md_core::emit::set_math_form(job.math);
                // PDF parsers can panic on odd files; report it instead of dying.
                let result = std::panic::catch_unwind(|| run(job, replace)).unwrap_or_else(|_| Err("The PDF reader crashed on this file.".to_string()));
                let result = result.map(|(msg, dir)| {
                    last = Some(dir);
                    msg
                });
                results.push((job.input.clone(), name, result));
            }
            progress::unwatch();
            over.store(true, Ordering::Relaxed);
            let msg = summary(&results.iter().map(|(_, name, r)| (name.clone(), r.clone())).collect::<Vec<_>>());
            let (lines, book) = (result_lines(&results), last.map(|d| d.to_string_lossy().into_owned()));
            // how the steps of the last book ended, with the time each took
            let now = Instant::now();
            let steps: Vec<String> = tracker.lock().unwrap_or_else(|e| e.into_inner()).steps(now).iter().map(step_line).collect();
            let _ = qt_thread.queue(move |mut qobj| {
                qobj.as_mut().set_steps(QString::from(steps.join("\n").as_str()));
                qobj.as_mut().set_progress(1.0);
                qobj.as_mut().set_remaining(QString::default());
                qobj.as_mut().set_status(QString::from(msg.as_str()));
                qobj.as_mut().set_last(QString::from(book.unwrap_or_default().as_str()));
                qobj.as_mut().set_results(QString::from(lines.as_str()));
                qobj.as_mut().set_current(QString::default());
                qobj.as_mut().refresh();
                qobj.as_mut().set_busy(false);
            });
        });
    }
}

/// How the jobs ended, as lines for the window (see the `results` property).
fn result_lines(results: &[(PathBuf, String, Result<String, String>)]) -> String {
    results
        .iter()
        .map(|(path, _, r)| {
            let (state, words) = match r {
                Ok(m) => ("ok", m),
                Err(e) => ("failed", e),
            };
            format!("{}\t{state}\t{}", path.display(), words.replace(['\t', '\n'], " "))
        })
        .collect::<Vec<_>>()
        .join("\n")
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
        let job = Job { kind: Kind::Pdf, input: "/x/My Book.pdf".into(), output: tmp.clone(), math: MathForm::GitHub };
        assert_eq!(job.book_dir(), book);
        assert_eq!(new_dir(&book), tmp.join("My Book (1)"));
        std::fs::create_dir_all(tmp.join("My Book (1)")).unwrap();
        assert_eq!(new_dir(&book), tmp.join("My Book (2)"));
        std::fs::remove_dir_all(&tmp).unwrap();
    }

    #[test]
    fn the_math_form_follows_the_choice_then_the_reader() {
        assert_eq!(math_choice(None, "md"), MathForm::GitHub);
        assert_eq!(math_choice(None, "code -n {folder}"), MathForm::Dollar);
        assert_eq!(math_choice(Some("github"), "obsidian"), MathForm::GitHub);
        assert_eq!(math_choice(Some("standard"), "md"), MathForm::Dollar);
    }

    #[test]
    fn the_reader_gets_the_book() {
        let dir = Path::new("/books/My Book");
        assert_eq!(reader_command("md", dir), ["md", "/books/My Book"]);
        assert_eq!(reader_command("", dir), ["xdg-open", "/books/My Book"]);
        assert_eq!(reader_command("  okular --unique {file} ", dir), ["okular", "--unique", "/books/My Book/book.md"]);
        assert_eq!(reader_command("code -n {folder}", dir), ["code", "-n", "/books/My Book"]);
    }

    #[test]
    fn how_long_ago_is_told() {
        let day = std::time::Duration::from_secs(86_400);
        let now = std::time::SystemTime::now();
        assert_eq!((ago(now, now), ago(now - day * 3, now), ago(now - day * 90, now)), ("today".to_string(), "3 days ago".to_string(), "3 months ago".to_string()));
    }

    #[test]
    fn what_can_be_converted_is_found() {
        let tmp = std::env::temp_dir().join(format!("pdf2md-items-{}", std::process::id()));
        for f in ["Zee.pdf", "b.PDF", "notes.txt", ".hidden.pdf", "Zee/.pdf2md-written", "QM 1 Downloads/syllabus.html", "QM 1 Downloads/Notes/1.pdf", "QM 1 Downloads/Notes/2.pdf", "QM 1/00-contents.md", "QM 1/pdf/1.pdf", "Site/index.html", "Site/a.html", "Site/one.pdf", "Read/page.html", "Read/a.pdf", "Read/a.md"] {
            std::fs::create_dir_all(tmp.join(f).parent().unwrap()).unwrap();
            std::fs::write(tmp.join(f), "x").unwrap();
        }
        // PDFs, and folders of pages and PDFs without Markdown (more PDFs than pages)
        let found: Vec<(Kind, String)> = find_items(&tmp).into_iter().map(|(k, p)| (k, p.file_name().unwrap().to_string_lossy().into_owned())).collect();
        assert_eq!(found, [(Kind::Pdf, "b.PDF".to_string()), (Kind::Course, "QM 1 Downloads".to_string()), (Kind::Pdf, "Zee.pdf".to_string())]);
        assert_eq!(book_name(Kind::Course, &tmp.join("QM 1 Downloads")), "QM 1");
        assert_eq!(book_name(Kind::Course, &tmp.join("Course")), "Course (book)");
        // the book beside its source is known as converted
        let lines = item_lines(&tmp);
        let zee: Vec<&str> = lines.lines().last().unwrap().split('\t').collect();
        assert_eq!((zee[0], zee[1], zee[5]), ("pdf", "Zee.pdf", "today"));
        assert_eq!(PathBuf::from(zee[4]), tmp.join("Zee"));
        assert_eq!(lines.lines().next().unwrap().split('\t').nth(4), Some(""));
        std::fs::remove_dir_all(&tmp).unwrap();
    }

    #[test]
    fn jobs_are_read_and_summed_up() {
        let jobs = parse_jobs("pdf\t/b/Zee.pdf\tfile:///out/My%20Books\tstandard\ncourse\t/b/QM 1 Downloads\t/b\tgithub\npdf\t\t/b\tgithub");
        assert_eq!(jobs.len(), 2);
        assert_eq!((jobs[0].book_dir(), jobs[0].math), (PathBuf::from("/out/My Books/Zee"), MathForm::Dollar));
        assert_eq!((jobs[1].kind, jobs[1].book_dir()), (Kind::Course, PathBuf::from("/b/QM 1")));
        let ok = |n: &str| (n.to_string(), Ok::<String, String>(format!("Wrote 3 files to /b/{n}")));
        assert_eq!(summary(&[ok("Zee")]), "Wrote 3 files to /b/Zee");
        assert_eq!(summary(&[ok("Zee"), ok("Dyson")]), "Converted 2 books.");
        assert_eq!(summary(&[ok("Zee"), ("Dyson".into(), Err("no text".into()))]), "Failed for 1 of 2 books. Dyson: no text");
    }

    #[test]
    fn a_step_is_told_in_a_line() {
        use pdf2md_core::progress::Step;
        let mut v = StepView { step: Step::Text, state: State::Running, fraction: 0.25, done: 155, total: 771, seconds: 47.0, spent: 12.6, quiet: 0.2, pace: 0.1 };
        assert_eq!(step_line(&v), "running\tReading the text\t0.250\t155 of 771 pages; 12 s so far, about 50 s left");
        // nothing new for a while: said, with the time ticking
        (v.spent, v.quiet) = (95.0, 83.0);
        assert_eq!(
            step_line(&v),
            "running\tReading the text\t0.250\t155 of 771 pages (none new for 1 min 23 s); 1 min 35 s so far, about 50 s left"
        );
        // a step that counts once a minute is not late after 83 s
        v.pace = 60.0;
        assert!(!step_line(&v).contains("none new"));
        v.state = State::Done;
        v.seconds = 83.2;
        assert_eq!(step_line(&v), "done\tReading the text\t0.250\t771 pages, 1 min 23 s");
        let w = StepView { step: Step::Files, state: State::Waiting, fraction: 0.0, done: 0, total: 0, seconds: 0.1, spent: 0.0, quiet: 0.0, pace: 0.0 };
        assert_eq!(step_line(&w), "waiting\tWriting the files\t0.000\t");
    }
}
