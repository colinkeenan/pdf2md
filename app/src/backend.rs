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
        /// `<output>/<name of the input file>/`.
        #[qinvokable]
        fn convert(self: Pin<&mut Converter>, input: &QString, output: &QString);

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

fn run(input: &Path, output: &Path) -> Result<String, String> {
    // <output>/<pdf file name without extension>/
    let stem = input
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "book".to_string());
    let dest = output.join(stem);

    let book = convert_pdf(input)?;
    let files = write_book(&book, &dest).map_err(|e| e.to_string())?;
    let mut msg = format!("Wrote {} files to {}", files.len(), dest.display());
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

    pub fn convert(mut self: Pin<&mut Self>, input: &QString, output: &QString) {
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
            let result = std::panic::catch_unwind(|| run(&input, &output))
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
