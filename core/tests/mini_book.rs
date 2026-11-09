//! End-to-end test on a tiny made-up book, written directly as the XML that
//! `pdftohtml -xml` would print for it (so no PDF is needed).
//!
//! Page size 918 x 1188, body text CMR10 at 15 px, margins 111..806,
//! chapter titles CMSS12 at 18 px, running headers at the top (top=69),
//! page numbers at the bottom (top=1086) on chapter-start pages.

use pdf2md_core::{convert_doc, emit, layout};

const FONTS: &str = r##"
<fontspec id="0" size="15" family="AAAAAA+CMR10" color="#000000"/>
<fontspec id="1" size="15" family="AAAAAA+CMTI10" color="#000000"/>
<fontspec id="2" size="18" family="AAAAAA+CMSS12" color="#000000"/>
<fontspec id="3" size="10" family="AAAAAA+CMR7" color="#000000"/>
<fontspec id="4" size="15" family="AAAAAA+CMMI10" color="#000000"/>
<fontspec id="5" size="22" family="AAAAAA+CMSS12" color="#000000"/>
"##;

fn t(top: u32, left: u32, width: u32, font: u32, text: &str) -> String {
    let height = if font == 3 { 10 } else { 15 };
    format!(r#"<text top="{top}" left="{left}" width="{width}" height="{height}" font="{font}">{text}</text>"#)
}

fn page(n: u32, body: &[String]) -> String {
    let fonts = if n == 1 { FONTS } else { "" };
    format!(
        "<page number=\"{n}\" position=\"absolute\" top=\"0\" left=\"0\" height=\"1188\" width=\"918\">{fonts}\n{}\n</page>\n",
        body.join("\n")
    )
}

/// A full-width line of body text.
fn body(top: u32, text: &str) -> String {
    t(top, 111, 695, 0, text)
}

fn book_xml() -> String {
    let filler = "and this sentence keeps going so that the line reaches the right margin";
    let pages = [
        // 1: table of contents
        page(1, &[
            t(213, 111, 157, 5, "Table of Contents"),
            t(308, 111, 695, 2, "Alpha 1"),
            t(328, 133, 673, 0, "Section One . . . . . . . . . . . . 1"),
            t(346, 111, 695, 2, "Beta 3"),
            t(364, 111, 695, 2, "Notes 5"),
            t(382, 111, 695, 2, "References 6"),
            t(400, 111, 695, 2, "Index 7"),
            t(1086, 456, 4, 0, "i"),
        ]),
        // 2: chapter Alpha, printed page 1
        page(2, &[
            t(220, 111, 60, 2, "Alpha"),
            t(300, 111, 90, 1, "<i>Section One</i>"),
            body(332, &format!("The first chapter starts here {filler}")),
            body(350, &format!("and it cites a book [1] {filler}")),
            t(368, 111, 300, 0, "and ends this paragraph with a note."),
            t(366, 411, 6, 3, "1"),
            t(400, 400, 9, 4, "E"),
            t(400, 415, 12, 0, "="),
            t(400, 430, 20, 4, "mc"),
            t(397, 450, 6, 3, "2"),
            t(400, 779, 27, 0, "(1)"),
            body(430, &format!("After the equation the text goes on {filler}")),
            t(1086, 455, 7, 0, "1"),
        ]),
        // 3: running header, printed page 2
        page(3, &[
            t(69, 111, 695, 1, "<i>ALPHA</i> 2"),
            body(126, &format!("The chapter continues on its second page {filler}")),
            t(144, 111, 300, 0, "and stops here."),
            // a notation list: three aligned columns
            t(200, 219, 11, 4, "A"),
            t(200, 256, 4, 0, ":"),
            t(200, 290, 200, 0, "the first symbol"),
            t(218, 219, 11, 4, "B"),
            t(218, 251, 12, 0, "="),
            t(218, 290, 200, 0, "the second symbol"),
            t(236, 219, 11, 4, "C"),
            t(236, 256, 4, 0, ":"),
            t(236, 290, 200, 0, "the third symbol"),
        ]),
        // 4: chapter Beta, printed page 3
        page(4, &[
            t(220, 111, 50, 2, "Beta"),
            body(300, &format!("The second chapter has a note too {filler}")),
            t(318, 111, 200, 0, "right at the end."),
            t(316, 311, 6, 3, "2"),
            t(1086, 455, 7, 0, "3"),
        ]),
        // 5: end notes, printed page 5
        page(5, &[
            t(220, 111, 50, 2, "Notes"),
            t(314, 131, 6, 3, "1"),
            t(316, 140, 200, 0, "First note text."),
            t(332, 131, 6, 3, "2"),
            t(334, 140, 200, 0, "Second note text."),
            t(1086, 455, 7, 0, "5"),
        ]),
        // 6: references, printed page 6
        page(6, &[
            t(220, 111, 80, 2, "References"),
            t(299, 118, 400, 0, "[1] A. Author, A Book, Press, 1950."),
            t(1086, 455, 7, 0, "6"),
        ]),
        // 7: index, printed page 7
        page(7, &[
            t(197, 111, 40, 2, "Index"),
            t(273, 111, 100, 0, "alpha, 1, 2"),
            t(291, 111, 100, 0, "beta, 3"),
            t(1086, 455, 7, 0, "7"),
        ]),
    ];
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE pdf2xml SYSTEM \"pdf2xml.dtd\">\n<pdf2xml producer=\"poppler\" version=\"26.10.0\">\n{}</pdf2xml>\n",
        pages.join("")
    )
}

#[test]
fn converts_a_small_book() {
    let doc = layout::parse_xml(&book_xml()).unwrap();
    let book = convert_doc(&doc);

    let titles: Vec<&str> = book.chapters.iter().map(|c| c.title.as_str()).collect();
    assert_eq!(titles, ["Front Matter", "Alpha", "Beta", "Index"], "{:?}", book.warnings);
    assert!(book.warnings.is_empty(), "{:?}", book.warnings);

    // table of contents: chapters and sections are links
    let front = emit::chapter_markdown(&book, 0);
    assert!(front.contains("- [Alpha](01-alpha.md)"), "{front}");
    assert!(front.contains("  - [Section One](01-alpha.md#s-section-one)"), "{front}");
    assert!(!front.contains("Notes"), "{front}"); // moved into the chapters

    let alpha = emit::chapter_markdown(&book, 1);
    assert!(alpha.contains("## <a id=\"s-section-one\"></a>Section One"), "{alpha}");
    // footnote marker -> note, and back
    assert!(alpha.contains("note.<sup><a id=\"fnref-1\" href=\"#fn-1\">1</a></sup>"), "{alpha}");
    assert!(alpha.contains("<a id=\"fn-1\"></a>**1.** First note text. [↩](#fnref-1)"), "{alpha}");
    // citation -> reference, and back
    assert!(alpha.contains("[[1]](#ref-1)"), "{alpha}");
    assert!(alpha.contains("**[1]** A. Author, A Book, Press, 1950. [↩](#refback-1-1)"), "{alpha}");
    // displayed equation with its number
    assert!(alpha.contains("$$\nE = mc^{2} \\tag{1}\n$$"), "{alpha}");
    // the running header is gone, the printed page number became an anchor
    assert!(!alpha.contains("ALPHA"), "{alpha}");
    assert!(alpha.contains("<a id=\"p2\"><sup>[p. 2]</sup></a>"), "{alpha}");

    // aligned columns become a LaTeX array
    assert!(
        alpha.contains("\\begin{array}{lcl}\nA & : & \\text{the first symbol} \\\\\nB & = & \\text{the second symbol}"),
        "{alpha}"
    );

    let beta = emit::chapter_markdown(&book, 2);
    assert!(beta.contains("<a id=\"fn-2\"></a>**2.** Second note text."), "{beta}");
    assert!(!beta.contains("First note"), "{beta}");

    // index page numbers link to page anchors in the right chapter
    let index = emit::chapter_markdown(&book, 3);
    assert!(index.contains("- alpha, [1](01-alpha.md#p1), [2](01-alpha.md#p2)"), "{index}");
    assert!(index.contains("- beta, [3](02-beta.md#p3)"), "{index}");
    assert!(book.unplaced.is_empty(), "{:?}", book.unplaced);
}
