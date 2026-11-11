use pdf2md_core::{convert_pdf_pages, write_book};
use std::{env, path::Path, process};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("--dump") {
        dump(&args[1..]);
        return;
    }
    if args.len() != 2 && args.len() != 4 {
        eprintln!("usage: pdf2md-cli <input.pdf> <output-dir> [first-page last-page]");
        eprintln!("       pdf2md-cli --dump <input.pdf> [first-page last-page]");
        process::exit(2);
    }
    let range = page_range(&args[2..]);
    // PDF2MD_PROGRESS=1: print the steps with their times as they go
    if env::var_os("PDF2MD_PROGRESS").is_some() {
        let began = std::time::Instant::now();
        pdf2md_core::progress::watch(move |step, done, total| {
            eprintln!("{:7.2} s  {} {done}/{total}", began.elapsed().as_secs_f32(), step.label());
        });
    }
    // equations corrected by hand in an earlier conversion are kept
    let kept = pdf2md_core::ocr::collect_corrections(Path::new(&args[1]));
    if kept > 0 {
        eprintln!("Kept {kept} hand-corrected equation(s) from the earlier conversion.");
    }
    // and the reader's notes and prose fixes
    let user = pdf2md_core::userdata::collect(Path::new(&args[1]));
    pdf2md_core::userdata::backup(&user, Path::new(&args[1]));
    if !user.is_empty() {
        eprintln!(
            "Kept {} note(s), {} marker(s) and {} fix(es) from the earlier conversion.",
            user.notes.len(),
            user.marks.len(),
            user.fixes.len()
        );
    }
    let mut book = convert_pdf_pages(Path::new(&args[0]), range).unwrap_or_else(|e| {
        eprintln!("error reading PDF: {e}");
        process::exit(1)
    });
    book.user = user;
    match write_book(&book, Path::new(&args[1])) {
        Ok(files) => {
            for f in &files {
                println!("{}", f.display());
            }
            for w in &book.warnings {
                eprintln!("warning: {w}");
            }
        }
        Err(e) => {
            eprintln!("error writing output: {e}");
            process::exit(1);
        }
    }
}

/// `--dump book.pdf [first last]`: print every text line with its position,
/// font size and style, so you can see what the PDF really contains.
fn dump(args: &[String]) {
    let Some(input) = args.first() else {
        eprintln!("usage: pdf2md-cli --dump <input.pdf> [first-page last-page]");
        process::exit(2);
    };
    let range = page_range(&args[1..]);
    match pdf2md_core::layout::read_pdf(Path::new(input), range) {
        Ok(doc) => print!("{}", pdf2md_core::layout::dump(&doc)),
        Err(e) => {
            eprintln!("error reading PDF: {e}");
            process::exit(1);
        }
    }
}

/// Optional "first last" page numbers.
fn page_range(args: &[String]) -> Option<(u32, u32)> {
    match (args.first(), args.get(1)) {
        (Some(a), Some(b)) => match (a.parse(), b.parse()) {
            (Ok(a), Ok(b)) => Some((a, b)),
            _ => {
                eprintln!("page numbers must be whole numbers");
                process::exit(2);
            }
        },
        _ => None,
    }
}
