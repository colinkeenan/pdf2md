# pdf2md — PDF textbook to Markdown

Converts a PDF textbook into a folder of Markdown files, one per chapter, plus `book.md`: the whole book in one file, so a browser's Ctrl+F searches all of it (its table of contents, index, notes and page markers link within the page).

* **File names follow the book's chapter numbers**: `03-theory-of-angular-momentum.md` is chapter 3, with all its sections. `00-front-matter.md` holds everything before chapter 1 (title pages, prefaces, each under its own heading), except the table of contents, which is a file of its own right after it (`00b-contents.md`), with the page markers of its pages between its entries. A half-title (the book's title alone on a page, after the prefaces) stands at the very top of the contents file, above the word "Contents", not at the end of the front matter. An empty page before a chapter belongs to what comes before it (the empty page after the title page of a part is in the part's file). What comes after the last chapter (appendices, index) counts on from it. A book whose chapters have no numbers is numbered in order, front matter 00. Notes you made when the files had other names move to the renamed files.
* **Footnotes / references**: notes and references printed at the end of the book are moved to the end of the chapter that cites them. A footnote printed at the foot of a page stays there: it stands at the foot of its page, right above the bar of the next page (also where the page turns in the middle of a paragraph). Clicking a marker jumps to the note; the note has a ↩ link back (one per citation if cited several times). A footnote whose marker is not in the text still stands at the foot of its page and says so.
* **Page markers**: where each printed page starts, a small <sup>[p. 32]</sup> marker (with anchor `#p32`), so you can search for a page ("p. 32") and see where you are. The marker always stands on a line of its own, under a translucent bar across the page, so it is easy to spot when scrolling. Where the page turns in the middle of a paragraph, the bar and the marker are written into the sentence as a block as wide as the text that floats: the browser cannot set it beside the words already on the line, so it puts it under that line and goes on filling the line, and the text above the bar runs to the right margin as on the printed page, at any width of the window. (A viewer that drops `style` attributes shows the marker inside the sentence, without the bar.)
* **Tables** of an ebook-made PDF are tables: such a book sets a table on a panel of grey, with a short rule under each cell of its head. The rules say where the columns are. The rows have no marks and stand close together, so they are found by how the cells stand: in the middle of their row's height, or at its top (whichever fits the table), a column with short cells showing where the rows are; where every row is one line, each line is a row. A table that reaches the foot of its page goes on at the top of the next. Formulas, links to equations and symbols that are pictures stay what they are inside the cells. The panel is no picture any more, and its text no picture's labels (one table's text was lost that way). A table that the book holds as a picture is read by the math recognizer as one formula, an array: under a table's caption that array is written as a table, each cell a formula (or words, if it holds only words), with the picture right under it, because a cell can be misread.
* **A page set in two columns stays in two columns** (an index): the columns stand side by side as in the book, each read from the top down.
* **Table of contents and index** are links. TOC entries point to chapter files or to headings inside them, and show their printed pages: a chapter its range ("— pp. 4–21", so you can tell which file holds page 20), a section its page ("— p. 15"), each linked to that page's marker. An index page number points to the word on that page where the entry's term appears (also "anticommute" → "anticommuting", "Lamb, Willis E." → "Lamb"); if the term is not in the text of that page (the index sometimes points to related material), it points to the page marker. In Dyson about 40 % of index links find their term. The found words are wrapped in their anchor (`<a id="ix-7">Lamb shift</a>`), so a viewer can highlight them; [mdBun](https://github.com/colinkeenan/mdBun) flashes the target of a link.
* **Math** is emitted as LaTeX (`$$ … $$`), never as pictures.
* **References inside the book** that are links in the PDF stay links, and the whole reference is the link: "Section 3.5" goes to that heading, "Chapter 3" and "Appendix B" to their files, "Figure 1.1" and "Table 2.1" to their captions, "Problem 6.8" to the problem, "(3.422)" to the equation. In a list ("Figures 4.3 and 4.7") each number is a link. The word decides the target: "Section 2.1" is the section even though an equation (2.1) exists. References to other books ("Section 3.9 of Gottfried") are not links in the PDF and stay text.
* **Pictures** (diagrams, plots, embedded images) are cut from the PDF as PNG files in `images/` (`fig-p049-1.png` = first picture on PDF page 49) and linked where they stand, with their labels inside them instead of in the text. Each gets a visible caption, numbered through the book with its printed page (_Figure 3 (p. 43)_), so searching a rendered chapter for "Figure" steps from picture to picture.

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
│   ├── src/ocr.rs         equation images -> LaTeX via a local model (src/math_ocr.py)
│   ├── src/scan.rs        scanned books: pages read by a local model (src/scan_read.py) -> blocks
│   ├── src/links.rs       TOC + index linking
│   ├── src/emit.rs        Markdown output
│   ├── src/measure.rs     how wide text and formulas come out in the viewer (where a line ends, for the page bars)
│   ├── src/progress.rs    the steps of a conversion, how far each is, the time left (for the app's progress bars)
│   ├── src/main.rs        command-line tool: pdf2md-cli
│   └── tests/mini_book.rs end-to-end test on a tiny made-up book (as pdftohtml XML)
└── app/              KDE Kirigami GUI (cxx-qt)
    ├── build.rs, src/main.rs, src/backend.rs   Rust side
    ├── pdf2md-app.desktop                      application menu launcher
    ├── pdf2md.svg                              its icon (Markdown mark over a faded PDF page)
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

`PDF2MD_PROGRESS=1` prints the steps with their times and the estimate of the time left as the conversion goes (what the app shows as progress bars; `core/src/progress.rs`). Two environment variables print what the formula builder sees: `PDF2MD_MATHDEBUG=1` prints every span (box, size, font, text) and bar of each displayed formula, `PDF2MD_SVGDEBUG=1` the tall glyph outlines read from the SVG and the bracket each was taken for.

## Build the GUI

1. Install Rust: <https://rustup.rs>
2. Install Qt 6 + KDE Frameworks 6 development packages (names vary by distro):
   * Fedora: `sudo dnf install qt6-qtbase-devel qt6-qtdeclarative-devel qt6-qtdeclarative kf6-kirigami-devel kf6-kirigami qt6-qtbase-private-devel`
   * Arch: `sudo pacman -S qt6-base qt6-declarative kirigami`
   * Debian/Ubuntu: `sudo apt install qt6-base-dev qt6-declarative-dev qml6-module-qtquick-controls qml6-module-qtquick-dialogs qml6-module-org-kde-kirigami` (needs a release that ships KF6)
3. `cargo run -p pdf2md-app`

The in-app file browser also needs the Qt Labs folder list model: Fedora `qt6-qtdeclarative` (already included), Debian/Ubuntu `qml6-module-qt-labs-folderlistmodel`, Arch `qt6-declarative`.

The book is written to `<output folder>/<PDF name>/`. If that folder already exists, the app asks whether to replace it (after the new conversion has succeeded, the old files go to the Trash as one folder, "<PDF name> (previous)", and the new ones are written into the same folder, so a terminal or editor open in it sees them at once) or keep both (the new one goes to `<PDF name> (1)`, `(2)` …).

**Reading the books.** “Reader” takes the command of your Markdown reader (for example `md`); it is remembered. The command gets the book's folder as its last argument; write `{folder}` or `{file}` (the whole book in one file, `book.md`) in it to put the book elsewhere (`okular {file}`). Left empty, the system's own choice opens the folder (`xdg-open`). When a conversion is done, “Open Book” in its message shows the new book in the reader, and under it the window lists the converted books, newest first: the ones converted with the app, and the books in the same folders and in the folder you browsed last (so books converted earlier or with the command-line tool are there too). A click shows one.

While converting, a bar shows how far the conversion is (by time) with an estimate of the time left. A conversion that takes more than 30 seconds in all (the first one of a book with equation images) also lists its steps like a terminal: what is done and how long it took, what is running with its own bar and time left, what is still to come. The estimates come from the speed measured so far; before a step has done anything they are guesses, set several times too high on purpose (a conversion that is over sooner than it said worries nobody). The time gone by is shown ticking beside each estimate, so a step that counts nothing for a while is still seen to be alive; a step that takes longer than estimated raises its estimate as it goes, and one that has counted nothing new for 30 seconds says so.

To start it from the application menu, install the program and its launcher:

```sh
cargo build --release -p pdf2md-app
install -Dm755 target/release/pdf2md-app ~/.local/bin/pdf2md-app
install -Dm644 app/pdf2md-app.desktop ~/.local/share/applications/pdf2md-app.desktop
install -Dm644 app/pdf2md.svg ~/.local/share/icons/hicolor/scalable/apps/pdf2md.svg
```

`~/.local/bin` must be on your PATH. After rebuilding, run the first `install` line again to update the menu's copy.

`qmake6` must be on your PATH (cxx-qt finds Qt through it). If it isn't: `QMAKE=/usr/lib/qt6/bin/qmake cargo run -p pdf2md-app`.

## How it reads a book

Everything is decided from the layout, measured against the book's own typical values (body font size, margins, line spacing):

* **Printed page numbers** come from the running header ("THE DIRAC THEORY 6") or a lone number at the bottom of the page. They become page anchors (`#p6`) for the index and the TOC.
* **Chapters** start at headings in the chapter-title font size: the most common large size that contains real words.
* **Centered text** (title pages, centered lines between paragraphs) goes in a `<div align="center">`, keeping its line breaks: a line counts as centered when its middle is at the middle of the text column and it is clearly in from both margins.
* **Hard returns** are kept. Text that wraps only breaks a line where the next word no longer fits; a line that ends early although the next line's first word would have fit behind it was ended on purpose (addresses on a copyright page, parts a., b., c. of a problem). Such a line stays a line of its own (`<br>`), and becomes a paragraph of its own when there is also more space above it than between the lines of a paragraph. Two lines of plain words with that extra space between them are separate paragraphs too.
* **Italic and bold prose** become `_…_` and `**…**` (bold is recognized by `<b>` or a bold font such as CMBX, e.g. journal volumes "Phys. Rev. **80**"); headings carry neither, since `#` styles them.
* **Section headings** are short lines that are entirely italic/bold with space above, or short plain lines with space above and below. TOC entries are matched to headings on the printed page they name, also when the wording differs a little; their TOC indentation decides `##` or `###`. In a book that numbers its sections the number says the level instead: `1.3 …` is `#`, `1.3.1 …` is `##` — or one level deeper each, `##` and `###`, where the chapter titles are set clearly larger (a quarter or more) than the `1.3 …` headings; a heading without a number stands at the level of the numbered headings of its type size — larger than all of them it is `#`, smaller it comes below the deepest. A heading the contents do not list, in a book without section numbers, also goes by type size. An italic line that starts with a small letter is no heading (a lemma's statement, broken up by its displayed equations).
* **Footnote markers** are small raised numbers glued to a word. End notes are matched by finding the longest run of increasing marker numbers in reading order (so an "O¹⁶" cannot steal note 16). Notes printed at the bottom of a page match markers on the same page.
* **Tables / notation lists**: 3+ lines whose columns start at the same x positions (no text running across a column edge), with words in the last column, become a LaTeX `array`.
* **Big brackets** are read from their glyph outlines in the SVG, because pdftohtml gives them no character (or drops them): straight strokes with the long vertical stroke on one side are [ or ], curved outlines bulging at mid-height are ( or ), a thin bar is |, two strokes meeting in a point are ⟨ or ⟩, and a curve whose outer edge is only a sharp point at mid-height is { or }. Pieces of tall extensible brackets (top hook, extensions, bottom hook) are joined first. Brackets are then matched like brackets in text (nested ones too) and written as `\left[ … \right]`, `bmatrix`, `vmatrix` and so on. A lone `{` followed by two or more rows becomes `cases` (split into value and condition at the widest gap all rows share).
* **Groups of rows**: a lone `}` after several rows groups them (`\left. \begin{aligned} … \end{aligned} \right\}`); a big bracket that opens on one line of a multi-line formula and closes on a later one is written `\left\{ … \right.` / `\left. … \right\}`. Labelled underbraces become `\underbrace{…}_{\text{label}}`; accents drawn as separate glyphs (˜ ˆ ˙ ¨) become `\tilde{a}`, `\hat{k}`, `\dot{\varphi}`.
* **Block matrices**: inside a matrix, small-font entries that stand free (not scripts touching a symbol) and form their own rows and columns are one block, written as an inner `matrix`, like β = ((1 0; 0 1), 0; 0, (−1 0; 0 −1)) in Dyson eq. (15). Matrix rows far from the bracket's middle stay with the bracket's equation instead of being taken for a line of their own.
* **TeX symbol-font quirks**: ⟨ ⟩ come out as "h" and "i", | and ‖ as "j" and "k", { } as "f" and "g", capitals as calligraphic letters; these are mapped back.
* **Pictures** are found in the SVG: stroked or filled paths that are neither glyphs nor thin bars, and embedded images. Pieces close together are one picture, and so are diagrams of several strokes side by side in a row. A group counts as a picture when it holds an image, or 3+ pieces at least 15 pt square, or 8+ pieces at least 40 pt wide. Text inside the picture, and short text just beside it on lines that lie within it, is a label: it is taken out of the text flow and the picture grows to include it. A short label (≤ 3 characters) beside the picture is taken even on a line with a formula, as long as it stands far from the rest of that line (an axis label "x" after an equation). Each picture is then rendered from the PDF at that box with `pdftocairo -png`.
* **Overlines** (ψ̄, Ā) are drawn bars, not text: a bar just above a single symbol, with no numerator above it and no root sign before it.
* **Displayed equations** are lines that are mostly math and indented, or carry an equation number "(16)" at the right margin, plus the rows of fractions and limits around them. `mathtex.rs` rebuilds them from glyph positions and drawn bars: a bar with symbols right above and below it is a fraction (numerator and denominator are the connected stacks touching the bar; a clearly narrower bar inside is a nested fraction, one of the same width a separate fraction stacked below), a bar right after a root sign is a square root; where no bar is found, a group above the main row overlapping a group below is taken as a fraction, small raised/lowered glyphs are scripts, groups around Σ/∫ are limits, rows and columns between tall brackets are a matrix, a system of equations with one number, or an equation continued on a line starting with + or −, becomes an `aligned` block.

## Equations that are pictures (ebook-made PDFs)

PDFs made from ebooks (calibre, Kindle) often carry their math as small images instead of text. pdf2md hands each such image to a local math-recognition model, [texify](https://github.com/VikParuchuri/texify), and puts the LaTeX it reads into the text: inline where the image sits in a line of prose, as a numbered displayed equation where it stands alone. Images that are not math stay pictures: one with a "Fig." caption below it; a diagram (long straight lines meeting at a corner, like the axes of a plot, or one connected shape large both ways, like a curve or a box — in math only a root sign grows like that, and it is told apart); lettering such as a publisher's logo (plain-text OCR finds nothing but words, and the math model reads it as two wholly different things); and a horizontal line, which is written as `---`. A very small image (a single symbol such as a hatted bold **x**, under 30 pixels wide) is read with white space added around it — filling the whole picture, it is read as some other letter. Where such a symbol stands inside a sentence that the PDF holds as one piece of text, the text is split at the gap the image sits in (a gap wider than the others, or one before a hyphen or comma). A tall equation the model cannot read whole is read line by line; so is an image of two or more lines that the model wrote as one long line, if the line-by-line reading says the same (then each line is a row of an `aligned` block). The same is done where the model itself writes a multi-line equation as a table (`\begin{array}{r l}`), with the rows cut into columns unevenly; a real table, whose columns are not cut at the "=", stays. The rows of such a block are lined up at the "=": a row that goes on with "=" has it under the "=" of the first row, one that goes on with "+" or "×" starts to the right of it. An image of program code (e.g. Mathematica) is read as text with [Tesseract](https://github.com/tesseract-ocr/tesseract) (Arch `tesseract tesseract-data-eng`) and written as a code block. Results are cached by image in `~/.cache/pdf2md/ocr/`, so converting a book again is instant.

The model needs its own Python environment (about 7 GB with PyTorch; a GPU makes it fast — a 770-page book takes about two minutes on an RTX 5080):

```sh
mkdir -p ~/.local/share/pdf2md/ocr && cd ~/.local/share/pdf2md/ocr
uv venv --python 3.12 .venv
uv pip install --python .venv/bin/python torch torchvision --index-url https://download.pytorch.org/whl/cu128
uv pip install --python .venv/bin/python texify "transformers==4.45.2"
```

(texify 0.2.1 does not work with newer transformers.)

**Your corrections are kept.** Every equation read from a picture, displayed or inside a sentence, has an invisible marker naming its picture (`<!-- pdf2md:eq … -->`). Fix an equation's LaTeX in a chapter file or in `book.md`; the next conversion of that book (into the same folder, or with Replace in the app) first collects the equations you changed and keeps each one in `~/.local/share/pdf2md/corrections/`, and uses it instead of the recognizer's reading from then on.


### Math set as text

Such a book sets the simple math inside its sentences in the text font: an italic letter is a variable, the brackets and "=" around it are ordinary characters, and the PDF holds the line as pieces by typeface, with prose and symbols mixed in one piece ("⟩, where "). There are no math fonts to tell by (the test is: no Computer Modern fonts in the book), so pdf2md cuts each piece into words, symbols, numbers, brackets and punctuation, and a stretch with no word in it is one formula if something in it is surely math:

* **Surely math:** an italic letter (with its primes: `a″` is `a''`), a Greek letter, a relation or operator prose never uses (`=`, `≈`, `→`, `×`, `⟨`, `⟩`, `|`, `∂`, `ħ` …), a subscript or superscript, a single bold letter or a short bold group (`**x′**` is a vector, `\boldsymbol{x'}`), an equation picture.
* **Math only beside math:** numbers, round and square brackets, `+`, `−`, `/`, a lone upright letter (`Z = 2`), a function name (`sin`, `exp`, `Tr`).
* **Two or three italic letters** are symbols (`kR`, `dt`) unless they are a common word (`is`, `or`, `not`, `ket`); three need math right beside them. A longer italic word, or an italic phrase, is emphasis.
* **Where a formula ends:** at a word; at a comma, semicolon or full stop that is followed by a space and is not inside brackets (`⟨x′, y′|` stays whole, `$x$, $y$ and $z$` are three); brackets around the whole formula, or without their partner, are prose (`($x$)`, `(see $x$)`). The hyphen of `x-polarized` and the apostrophe of `Green′s` are prose.
* A small raised number after a word is a footnote marker; after a symbol, a number or a formula in brackets it is a power (`$N^{2}$`). A linked number — `(2.28)`, a page in the index — is a reference, never part of the formula beside it.
* A formula that goes on in the next line is joined with its beginning.

## Scanned books

A scan is a PDF whose pages are pictures: there is no text in it to take (or only the hidden text an earlier reading left under the pictures, with every formula garbled). pdf2md sees this by itself (next to no text, an image on nearly every page) and then has the pages read by [marker](https://github.com/datalab-to/marker), a page-reading model that runs on your own computer; nothing is sent anywhere. marker gives the prose, the formulas in sentences and on lines of their own as LaTeX, the headings, the footnotes and where the pictures are; pdf2md makes chapters, page markers, linked notes and contents, numbered pictures and the files of it as for any other book.

marker is not part of pdf2md. Set it up once, in an environment of its own (about 6 GB, and 3 GB of models that it downloads at the first book):

```sh
mkdir -p ~/.local/share/pdf2md/scan && cd ~/.local/share/pdf2md/scan
uv venv --python 3.12 .venv
uv pip install --python .venv/bin/python "marker-pdf==1.10.2"
```

(`PDF2MD_SCAN_PYTHON` names another Python that has marker. Version 1.10.2 is the last that runs inside Python alone; the versions after it need a model server.) Tables of contents and tables also need `pdftotext` (poppler) or, for a scan without hidden text, Tesseract.

**marker's licences are not pdf2md's.** Its code is under the GNU GPL 3 (as pdf2md is), but its model weights are under the “AI Pubs Open RAIL-M License (Modified)”: free for personal use and research, not for organizations with more than 2 million US dollars of revenue or funding, not for competitors of its maker, and with terms about what the model writes. Whoever runs marker is bound by them, so read them before you convert a scanned book: both texts and a short summary are in [`licenses/marker`](licenses/marker).

How it goes, and what to know:

* A page takes about 3 seconds on a recent graphics card (a 540-page book: half an hour), read 16 pages at a time; the window counts the pages. Pages read are kept in `~/.cache/pdf2md/scan/`, by what the PDF holds, so converting the book again takes seconds and a conversion that was stopped goes on where it was.
* **Chapters** are told by the layout, not by type sizes (the height of a heading's box says too little): a page that opens with a heading and has no running head opens a chapter; “Chapter I.5” and the title under it are one title. What opens a page before the first “Chapter …” or “Part …” is front matter. All other headings are sections of one level (`##`), unless they are numbered (`1.2.3`). A heading marker saw but could not read (it happens to small capitals) gets its words from the PDF's hidden text or from Tesseract. A book without running heads goes by the heights of its headings, which is less sure.
* **Printed page numbers** are read from the running heads and feet; a page that shows none (the first of a chapter), or whose number was misread, gets its number from the pages around it; unnumbered pages at the end (an index) count on.
* **Equations** keep their numbers; one you correct by hand is kept for the next conversion like the equations read from images (see below). Each displayed equation is read a second time, by the math recognizer (texify) from a picture of it cut from the page, and the ones the two readers read as different formulas are listed in `check-equations.md`, biggest differences first (the two write the same formula in different ways, a matrix, a prime, an arrow: that is not counted). This needs the environment for equations that are pictures (see below); without it there is no list. A formula marker did not finish (a matrix cut off, a table of numbers without end) is replaced by the second reading if that one is whole, and a bracket that grows with what it holds but closes in another row of the formula is a plain bracket.
* **Footnotes** are linked to their markers. marker does not see every small raised number in the text (nor does Tesseract, nor the PDF's own text). Where a page has a footnote without its marker, the marker is looked for on the picture of that page, of the page before and of the page after (`scan_marks.py`): a small piece of ink set high in its line, right behind a word or the full stop after it, that reads as the note's number. It is put into the text behind that word, if the word (with the word before it) is found there in one place only. A note whose marker is still not found stands at the foot of its page and says so.
* **The table of contents** is read line by line from the PDF's hidden text (or by Tesseract), because marker reads tables cell by cell, which shuffles it. That text is poor (“It9” for “It?”, letters lost), so an entry whose page opens with a title takes that title as marker read it, and the name of a part is the title that opens the part. Where the contents are set in columns (the chapters' numbers stand apart in front of the titles), they are written as a table of three columns, page by page: number, title, pages. A title that starts in the first column (a preface, the index) starts there, and the name of a part is a heading (`##`) centered in the middle column, in its lines. Other tables are tables as marker read them, if their rows and cells hold together (two rows and two columns or more, rows of like length, most cells with something in them: marker also sees tables in empty pages), with the picture of the table right under it, because a cell can be misread; formulas in the cells that marker left unmarked are marked. A table that does not hold together is kept as a picture.
* **The title page of a part** (“PART IV”, and under it the part's name) is the title of the part's file, centered and in its lines.
* A line of which marker read too little for the room it takes (“cm.” where the page says “p.   cm.”) takes its words from the PDF's own text, if those are marker's words and more.
* **Pages set centered** (a title page, a dedication: every block of text lies about the middle of the page and is clearly narrower than it) stay centered, and each of their lines ends where it ends in the book. On other pages a block of several lines that stops short of the right margin (an address, an entry of the index) keeps its line ends; a block as wide as the text is set is a paragraph. The publisher's name at the foot of the title page is text, not a running foot. On a page made of single lines (a copyright page, a poem: no ordinary paragraph and no equation on it), lines that follow each other without space between them are one paragraph with its line ends kept (an address), and pieces side by side are one line.
* **The cover** (the first sheet, a picture over a third of it or more and only a few words) is one picture of the whole page; its title and author are not written as text. Anywhere in the book, words that lie on a picture are part of the picture.
* **An index in two columns** stays in two columns. marker gives a column's lines without saying which are set in (the lines that go on from the entry above), and at times deals the lines of both columns into one block; so which line stands in which column, and which is set in, comes from the places of the words in the PDF's own text, and each line takes marker's reading of it where marker has that line (it reads better). Lines that go on are joined to their entry: a range of pages cut at the line's end is one range (“248-52”), a word written with a hyphen elsewhere keeps it. A scan without text of its own keeps its columns but not the joining: each line is an entry.
* What marker calls program code is only written as a code block if it looks like code (it calls an index page code, too); otherwise it is text.
* **Symbols in a sentence** that marker left as plain text are made formulas by their shape: a letter or two with what they are a function of in brackets right behind (`J(x)`, `D(x-y)`, `SU(2)`); a single letter other than “a”, “A”, “I” that is not an initial (“S. Coleman”), a label (“(b)”, “Appendix B”, “Part V”) or a unit behind a number; a short relation without spaces (`m=0`); with what such a symbol is set equal or added to (`h = 1`). “a” is a symbol where it stands with one (“a and b”). A formula that goes on in the next line of the book and comes as two is one.
* Measured on the test book (537 pages): half an hour for the first conversion, half a minute for the next; 10,262 formulas, all of which render; 1190 displayed equations read a second time, 403 listed to check (40 with big differences); 119 footnotes, 107 linked to their markers (34 of them found on the pages' pictures) and 12 without.

## Your notes and fixes

Converting a book again keeps what you added, as long as it is marked:

* **Your own notes**: a collapsible note, opened with a click,

  ```markdown
  <details class="mine">
  <summary>✎ My note: why the sign flips</summary>

  Your explanation, with math like $[A,B]=-[B,A]$.

  </details>
  ```

  or an alert box that is always visible, e.g. for where you think the book is wrong:

  ```markdown
  > [!WARNING]
  > I think the factor should be $\frac12$.
  ```

  Each goes back right after the paragraph or equation it followed (found by that block's first words, else by the nearest anchor above it). A note whose place is gone ends up at the end of its chapter under "Your notes whose place was not found", never dropped.
* **Markers inside a sentence** that point to a note: `<sup>[✎](#mine-3)</sup>` (with `id="mine-3"` on the note), or any `<sup class="mine">…</sup>`. Each goes back right after the same words in the same paragraph (at the paragraph's end if those words changed).
* **Fixes to the converted text** (the conversion got the book's words wrong): `<span class="fix" data-was="as converted">as in the book</span>`. It reads as normal text and is redone in the same paragraph. (Equations need no marking: just edit them.)

Notes and fixes are collected from the chapter files and from `book.md` (each counted once), and a copy is kept in `~/.local/share/pdf2md/notes/<book folder>.md`. Other edits to the text are not kept.

**Words under an equation.** Some equation images explain their parts with small words set under them ("bra" and "ket" under the two halves of a product). The model drops such words or reads them as symbols. An image of one line of math with a band of small type below it that Tesseract reads as words (not a denominator under its bar, not the limits under a sum; an underbrace with words is read well as it is) is read again without the band, and the words go back in a second row, each centred where it stood in the image: `\begin{array}{l} … \\ \mathrlap{\kern2.1em\mathclap{\scriptsize\text{bra}}}… \end{array}`. The places are fractions of the equation's width in the image, turned into em by an estimate of how wide the equation comes out, so a word may stand a little off. Sakurai has three such equations.

**Equations to check.** Each equation is read a second time from the picture enlarged; where the two readings are different formulas (formatting aside), it is listed in `check-equations.md` in the book folder, biggest differences first, with links. A corrected equation drops off the list. To use another Python, set `PDF2MD_OCR_PYTHON`. Without the model, equation images stay pictures. The recognizer is good but not perfect: check equations you rely on against the book.

* **Cells of tables**: correct a cell where it stands, between the empty lines inside its `<td>` … `</td>` (or fill an empty `<td></td>`, or empty a cell). Every table of the book carries an invisible mark above it (`<!-- pdf2md:table … -->`), and the next conversion keeps each cell that is not what the last one wrote there, by table, row and column (`~/.local/share/pdf2md/corrections/`, with the corrected equations). A table whose reading changes in a later version of pdf2md is another table: corrections of the old reading are not put into it. The table of contents is made anew each time and keeps no corrections.

## Licence

pdf2md is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. It is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; see [`LICENSE`](LICENSE).

Copyright (C) 2026 Colin Keenan

The programs it runs have licences of their own: poppler (GPL), [texify](https://github.com/VikParuchuri/texify) (code GPL 3, model weights CC BY-SA 4.0), Tesseract (Apache 2.0), and for scanned books [marker](https://github.com/datalab-to/marker) (code GPL 3, model weights under a licence with limits on who may use them: see [`licenses/marker`](licenses/marker)). None of them is contained in pdf2md; you install them yourself.

## Known limitations

* **Formulas that would not render are mended when written** (every book): two formulas in a sentence with nothing between them are one (written one right behind the other they read as `$$`, the mark of a displayed formula, and put what follows in the file out of joint); math inside `\text{…}` is taken out of it; two subscripts or two superscripts on one symbol are one. A mended formula renders; whether it is the book's formula is another matter (a table read as formulas stays wrong).

Tested on one book so far (Dyson, *Advanced Quantum Mechanics*, a TeX book): all chapters, every TOC entry, all 93 footnotes, all references and the whole index are linked.

* **Math is good but not perfect.** All of Dyson's displayed equations are valid LaTeX (KaTeX renders every one), and every problem found so far in it is fixed. A new book will bring cases not seen yet, and a garbled formula does not always look broken. Some glyphs of TeX's big-symbol font are lost by `pdftohtml`; a lone bracket gets its partner back by guessing.
* Math-font detection knows TeX's Computer Modern / AMS fonts and fonts named "Math", "Symbol" or "STIX". Books typeset with other math fonts will need their font names added in `layout.rs` (`is_math_font`) — check with `--dump`.
* Pictures are PNG files, so their labels are not searchable text. A picture made of only one or two strokes is not recognized as one.
* Footnotes at the foot of a page are written at the foot of their page (tested on a scanned book and an ebook-made one; Dyson has only end notes).
