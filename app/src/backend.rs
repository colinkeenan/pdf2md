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
use pdf2md_core::{convert_pdf, write_book};
use std::path::{Path, PathBuf};

pub struct ConverterRust {
    status: QString,
    busy: bool,
    folder: QString,
}

impl Default for ConverterRust {
    fn default() -> Self {
        Self {
            status: QString::default(),
            busy: false,
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
            // to the Trash, so the old conversion can still be restored
            trash::delete(&dest).map_err(|e| format!("could not move {} to the Trash: {e}", dest.display()))?;
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

        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
            // PDF parsers can panic on odd files; report it instead of dying.
            let result = std::panic::catch_unwind(|| run(&input, &output, replace))
                .unwrap_or_else(|_| Err("The PDF reader crashed on this file.".to_string()));
            let msg = match result {
                Ok(m) => m,
                Err(e) => format!("Failed: {e}"),
            };
            let _ = qt_thread.queue(move |mut qobj| {
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
}
