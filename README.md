# pdf2md — PDF textbook to Markdown

Converts a PDF textbook into a folder of Markdown files, one per chapter.

* **Footnotes / references** are moved from the end of the book to the end of the chapter that cites them. Clicking a marker jumps to the note; the note has a ↩ link back (one per citation if cited several times).
* **Page markers**: where each printed page starts, a small <sup>[p. 32]</sup> marker (with anchor `#p32`), so you can search for a page ("p. 32") and see where you are.
* **Table of contents and index** are links. TOC entries point to chapter files or to headings inside them, and show their printed pages: a chapter its range ("— pp. 4–21", so you can tell which file holds page 20), a section its page ("— p. 15"), each linked to that page's marker. An index page number points to the word on that page where the entry's term appears (also "anticommute" → "anticommuting", "Lamb, Willis E." → "Lamb"); if the term is not in the text of that page (the index sometimes points to related material), it points to the page marker. In Dyson about 40 % of index links find their term. The found words are wrapped in their anchor (`<a id="ix-7">Lamb shift</a>`), so a viewer can highlight them; [mdBun](https://github.com/colinkeenan/mdBun) flashes the target of a link.
* **Math** is emitted as LaTeX (`$$ … $$`), never as pictures.
* **Pictures** (diagrams, plots, embedded images) are cut from the PDF as PNG files in `images/` (`fig-p049-1.png` = first picture on PDF page 49) and linked where they stand, with their labels inside them instead of in the text.

## Layout

```
pdf2md/
├── Cargo.toml        workspace
├── core/             the converter – plain Rust, no Qt (start here)
│   ├── src/lib.rs         shared types + the pipeline (convert_doc)
│   ├── src/layout.rs      runs `pdftohtml -xml` (text spans with position and font) and `pdftocairo -svg` (drawn bars, pictures)
│   ├── src/pages.rs       running headers, page-number footers, printed page numbers, two columns
│   ├── src/blocks.rs      lines -> headings, paragraphs, equations, TOC, index lines, footnotes
│   ├── src/mathtex.rs     positioned spans -> LaTeX (fractions, scripts, limits, matrices)
│   ├── src/structure.rs   chapters = headings in the chapter-title font size
│   ├── src/notes.rs       footnotes + references, linked both ways
│   ├── src/links.rs       TOC + index linking
│   ├── src/emit.rs        Markdown output
│   ├── src/main.rs        command-line tool: pdf2md-cli
│   └── tests/mini_book.rs end-to-end test on a tiny made-up book (as pdftohtml XML)
└── app/              KDE Kirigami GUI (cxx-qt)
    ├── build.rs, src/main.rs, src/backend.rs   Rust side
    ├── pdf2md-app.desktop                      application menu launcher
    └── qml/Main.qml, qml/FileBrowser.qml       Kirigami UI
```

## Try the converter first (no Qt needed)

The PDF is read with poppler's `pdftohtml` (text, positions, fonts) and `pdftocairo` (drawn bars: fraction bars, root bars, overlines like ψ̄; pictures) programs, so install it first: Arch `poppler`, Debian/Ubuntu/Fedora `poppler-utils`. (KDE systems usually have it already, because Okular uses poppler.)

```sh
cargo test -p pdf2md-core
cargo run -p pdf2md-core --bin pdf2md-cli -- book.pdf out/
cargo run -p pdf2md-core --bin pdf2md-cli -- book.pdf out/ 10 12   # convert only pages 10-12 (for checking)
cargo run -p pdf2md-core --bin pdf2md-cli -- --dump book.pdf 10 12   # fonts, positions, sub/superscripts of pages 10-12
```

`--dump` is the tool for understanding a new book: it shows every line with its position, font size and style, superscripts as `^{..}`, subscripts as `_{..}`, math-font text inside `«..»`, and the drawn bars on each page.

Two environment variables print what the formula builder sees: `PDF2MD_MATHDEBUG=1` prints every span (box, size, font, text) and bar of each displayed formula, `PDF2MD_SVGDEBUG=1` the tall glyph outlines read from the SVG and the bracket each was taken for.

## Build the GUI

1. Install Rust: <https://rustup.rs>
2. Install Qt 6 + KDE Frameworks 6 development packages (names vary by distro):
   * Fedora: `sudo dnf install qt6-qtbase-devel qt6-qtdeclarative-devel qt6-qtdeclarative kf6-kirigami-devel kf6-kirigami qt6-qtbase-private-devel`
   * Arch: `sudo pacman -S qt6-base qt6-declarative kirigami`
   * Debian/Ubuntu: `sudo apt install qt6-base-dev qt6-declarative-dev qml6-module-qtquick-controls qml6-module-qtquick-dialogs qml6-module-org-kde-kirigami` (needs a release that ships KF6)
3. `cargo run -p pdf2md-app`

The in-app file browser also needs the Qt Labs folder list model: Fedora `qt6-qtdeclarative` (already included), Debian/Ubuntu `qml6-module-qt-labs-folderlistmodel`, Arch `qt6-declarative`.

To start it from the application menu, install the program and its launcher:

```sh
cargo build --release -p pdf2md-app
install -Dm755 target/release/pdf2md-app ~/.local/bin/pdf2md-app
install -Dm644 app/pdf2md-app.desktop ~/.local/share/applications/pdf2md-app.desktop
```

`~/.local/bin` must be on your PATH. After rebuilding, run the first `install` line again to update the menu's copy.

`qmake6` must be on your PATH (cxx-qt finds Qt through it). If it isn't: `QMAKE=/usr/lib/qt6/bin/qmake cargo run -p pdf2md-app`.

## How it reads a book

Everything is decided from the layout, measured against the book's own typical values (body font size, margins, line spacing):

* **Printed page numbers** come from the running header ("THE DIRAC THEORY 6") or a lone number at the bottom of the page. They become page anchors (`#p6`) for the index and the TOC.
* **Chapters** start at headings in the chapter-title font size: the most common large size that contains real words.
* **Centered text** (title pages, centered lines between paragraphs) goes in a `<div align="center">`, keeping its line breaks: a line counts as centered when its middle is at the middle of the text column and it is clearly in from both margins.
* **Italic and bold prose** become `_…_` and `**…**` (bold is recognized by `<b>` or a bold font such as CMBX, e.g. journal volumes "Phys. Rev. **80**"); headings carry neither, since `#` styles them.
* **Section headings** are short lines that are entirely italic/bold with space above, or short plain lines with space above and below. TOC entries are matched to headings on the printed page they name, also when the wording differs a little; their TOC indentation decides `##` or `###`.
* **Footnote markers** are small raised numbers glued to a word. End notes are matched by finding the longest run of increasing marker numbers in reading order (so an "O¹⁶" cannot steal note 16). Notes printed at the bottom of a page match markers on the same page.
* **Tables / notation lists**: 3+ lines whose columns start at the same x positions (no text running across a column edge), with words in the last column, become a LaTeX `array`.
* **Big brackets** are read from their glyph outlines in the SVG, because pdftohtml gives them no character (or drops them): straight strokes with the long vertical stroke on one side are [ or ], curved outlines bulging at mid-height are ( or ), a thin bar is |, two strokes meeting in a point are ⟨ or ⟩, and a curve whose outer edge is only a sharp point at mid-height is { or }. Pieces of tall extensible brackets (top hook, extensions, bottom hook) are joined first. Brackets are then matched like brackets in text (nested ones too) and written as `\left[ … \right]`, `bmatrix`, `vmatrix` and so on. A lone `{` followed by two or more rows becomes `cases` (split into value and condition at the widest gap all rows share).
* **Groups of rows**: a lone `}` after several rows groups them (`\left. \begin{aligned} … \end{aligned} \right\}`); a big bracket that opens on one line of a multi-line formula and closes on a later one is written `\left\{ … \right.` / `\left. … \right\}`. Labelled underbraces become `\underbrace{…}_{\text{label}}`; accents drawn as separate glyphs (˜ ˆ ˙ ¨) become `\tilde{a}`, `\hat{k}`, `\dot{\varphi}`.
* **Block matrices**: inside a matrix, small-font entries that stand free (not scripts touching a symbol) and form their own rows and columns are one block, written as an inner `matrix`, like β = ((1 0; 0 1), 0; 0, (−1 0; 0 −1)) in Dyson eq. (15). Matrix rows far from the bracket's middle stay with the bracket's equation instead of being taken for a line of their own.
* **TeX symbol-font quirks**: ⟨ ⟩ come out as "h" and "i", | and ‖ as "j" and "k", { } as "f" and "g", capitals as calligraphic letters; these are mapped back.
* **Pictures** are found in the SVG: stroked or filled paths that are neither glyphs nor thin bars, and embedded images. Pieces close together are one picture, and so are diagrams of several strokes side by side in a row. A group counts as a picture when it holds an image, or 3+ pieces at least 15 pt square, or 8+ pieces at least 40 pt wide. Text inside the picture, and short text just beside it on lines that lie within it, is a label: it is taken out of the text flow and the picture grows to include it. A short label (≤ 3 characters) beside the picture is taken even on a line with a formula, as long as it stands far from the rest of that line (an axis label "x" after an equation). Each picture is then rendered from the PDF at that box with `pdftocairo -png`.
* **Overlines** (ψ̄, Ā) are drawn bars, not text: a bar just above a single symbol, with no numerator above it and no root sign before it.
* **Displayed equations** are lines that are mostly math and indented, or carry an equation number "(16)" at the right margin, plus the rows of fractions and limits around them. `mathtex.rs` rebuilds them from glyph positions and drawn bars: a bar with symbols right above and below it is a fraction (numerator and denominator are the connected stacks touching the bar; a clearly narrower bar inside is a nested fraction, one of the same width a separate fraction stacked below), a bar right after a root sign is a square root; where no bar is found, a group above the main row overlapping a group below is taken as a fraction, small raised/lowered glyphs are scripts, groups around Σ/∫ are limits, rows and columns between tall brackets are a matrix, a system of equations with one number, or an equation continued on a line starting with + or −, becomes an `aligned` block.

## Known limitations

Tested on one book so far (Dyson, *Advanced Quantum Mechanics*, a TeX book): all chapters, every TOC entry, all 93 footnotes, all references and the whole index are linked.

* **Math is good but not perfect.** All of Dyson's displayed equations are valid LaTeX (KaTeX renders every one), and every problem found so far in it is fixed. A new book will bring cases not seen yet, and a garbled formula does not always look broken. Some glyphs of TeX's big-symbol font are lost by `pdftohtml`; a lone bracket gets its partner back by guessing.
* Math-font detection knows TeX's Computer Modern / AMS fonts and fonts named "Math", "Symbol" or "STIX". Books typeset with other math fonts will need their font names added in `layout.rs` (`is_math_font`) — check with `--dump`.
* Pictures are PNG files, so their labels are not searchable text. A picture made of only one or two strokes is not recognized as one.
* Page-bottom footnotes are supported but untested on a real book (Dyson has only end notes).
