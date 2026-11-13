# How pdf2md works

The [README](../README.md) says what pdf2md does and how to use it. This page describes what it writes and how it reads each kind of PDF, including the rules it applies and where they stop working. The rules were worked out on three books: Dyson's *Advanced Quantum Mechanics* (made with TeX), Sakurai's *Modern Quantum Mechanics* (made from an ebook) and Zee's *Quantum Field Theory in a Nutshell* (a scan).

- [The source](#the-source)
- [What is written](#what-is-written)
- [The two forms of math](#the-two-forms-of-math)
- [PDFs that hold text](#pdfs-that-hold-text)
- [PDFs made from ebooks](#pdfs-made-from-ebooks)
- [Scanned books](#scanned-books)
- [What is kept between conversions](#what-is-kept-between-conversions)
- [The app window](#the-app-window)
- [Measurements](#measurements)

## The source

```
pdf2md/
├── core/             the converter: plain Rust, no Qt
│   ├── src/lib.rs         shared types and the pipeline
│   ├── src/layout.rs      runs pdftohtml and pdftocairo; text spans, drawn bars, pictures
│   ├── src/annots.rs      link targets read straight from the PDF
│   ├── src/pages.rs       running heads, printed page numbers, columns
│   ├── src/blocks.rs      lines to headings, paragraphs, equations, tables, notes
│   ├── src/mathtex.rs     positioned glyphs to LaTeX
│   ├── src/textmath.rs    math set in the text font (ebook-made PDFs)
│   ├── src/structure.rs   chapters, heading levels, file names
│   ├── src/notes.rs       footnotes and references, linked both ways
│   ├── src/links.rs       contents, index and cross-references
│   ├── src/ocr.rs         equation pictures to LaTeX (math_ocr.py, table_ocr.py)
│   ├── src/scan.rs        scanned books (scan_read.py, scan_marks.py)
│   ├── src/userdata.rs    the reader's notes and fixes
│   ├── src/emit.rs        Markdown output
│   ├── src/measure.rs     how wide a formula comes out (for words under an equation)
│   ├── src/progress.rs    steps of a conversion and the time left
│   ├── src/main.rs        the command-line tool, pdf2md-cli
│   └── tests/mini_book.rs end-to-end test on a small made-up book
└── app/              the window: KDE Kirigami, through cxx-qt
    ├── src/backend.rs     the Rust side
    └── qml/               Main.qml, FileBrowser.qml
```

pdf2md reads a PDF with two poppler programs. `pdftohtml -xml` gives every piece of text with its position and font. `pdftocairo -svg` gives what is drawn: fraction bars, root bars, the outlines of tall brackets, and pictures.

Several switches print what the converter sees:

| | |
|---|---|
| `pdf2md-cli --dump book.pdf 10 12` | every line of PDF pages 10 to 12 with position, font size and style; scripts as `^{…}` and `_{…}`; math-font text in `«…»`; the drawn bars |
| `PDF2MD_PROGRESS=1` | the steps with their times and the estimate of the time left |
| `PDF2MD_MATHDEBUG=1` | every span and bar of each displayed formula |
| `PDF2MD_SVGDEBUG=1` | the tall outlines read from the SVG and the bracket each was taken for |
| `PDF2MD_TMDEBUG=1` | how doubtful lines of text-font math were cut into words and symbols |
| `PDF2MD_TABLEDEBUG=1` | the grey panels found and their lines |
| `PDF2MD_MARKDEBUG=1` | for each footnote marker found on a page's picture, the words looked for and how many places fit; the markers not placed |
| `PDF2MD_OCR_PYTHON`, `PDF2MD_SCAN_PYTHON` | another Python that has texify or marker |

## What is written

### Files

Each chapter is a file named by its number, such as `03-theory-of-angular-momentum.md`, with all its sections. A book whose chapters carry no numbers is numbered in order. Appendices and the index count on after the last chapter.

`00-front-matter.md` holds everything before chapter 1, each title page or preface under its own heading. The table of contents is a file of its own right after it. Where the book prints prefaces or a foreword after its contents, the files follow the book: `00a-title-pages.md` holds what comes before the contents, `00c-additional-front-matter.md` what comes after them. A page printed after the contents that holds only a title (the title of the lectures, before the first chapter) heads the file after the contents, with its page marker. A half-title, the book's title printed once more on a page of its own, stands at the top of the file that follows it, above that file's title: above the word "Contents", or above the first part. It is never the last thing in the file before. An empty page belongs to what comes before it.

`book.md` is the whole book in one file, with all links pointing inside it, so one search covers the book.

### Page markers

Where a printed page starts there is a marker, <sup>[p. 32]</sup>, with the anchor `#p32`. It stands on a line of its own under a faint bar.

Above the bar, at the right, stands the marker of the page that ends there, <sup>[p. 31]</sup>. With one bar in the window both pages are named, and each marker opens its own page of the PDF. Only the marker under the bar carries the anchor. The page that ends is the page before in the PDF; in an ebook-made PDF it is the page of the marker before.

* **At the top of a file** the bar and marker of the page the file opens on stand above the file's title: the title is printed on that page, not at the foot of the page before. That first bar has no marker above it, since nothing of the page before is in the file.
* **Among centered lines** (a title page) the bar and marker stand at the left like all others.
* **In the middle of a paragraph** the bar and marker are written into the sentence as a floating block as wide as the text. The browser puts it under the current line and goes on filling that line, so the text above the bar runs to the right margin as on the printed page, at any window width. A viewer that drops `style` attributes shows the marker inside the sentence, without the bar.
* **The link.** Each marker opens that page of the PDF in a new tab: `book.pdf#page=38`, counted in sheets of the PDF, which is how browsers count. `book.pdf` in the book's folder is a symbolic link to the source PDF, made anew at each conversion. If you move the PDF, convert again.
* **Pages without a printed number** get <sup>[PDF p. 5]</sup> (anchor `#pdf-p5`) if they say something: 30 words or more, a formula, a table or a picture. A title page gets none. The contents name a piece that comes before the first printed page number (a preface) by that PDF page, too.
* **Inside a list**, such as the contents or the index, a marker on a line of its own would end the list and turn the indented entries after it into code. There the marker is attached to the entry before it. The bar then starts at that entry's indent, and the entries after it stay under their part or chapter.
* **Ebook-made PDFs** do not have one PDF page per printed page. The link opens the PDF page on which the printed page starts, and only the pages before the first and after the last printed page get PDF markers.

### Footnotes and references

A footnote printed at the foot of a page is written at the foot of that page, just above the next page's bar. Notes and references printed at the end of the book move to the end of the chapter that cites them. A marker links to its note, and the note has a ↩ link back for each place that cites it. A footnote whose marker was not found still stands at the foot of its page and says so.

### Contents, index and cross-references

* **Contents entries** link to chapter files and to headings inside them. Each shows its printed pages, a range for a chapter ("pp. 4–21") and a page for a section, linked to that page's marker.
* **Index page numbers** link to the word on that page where the entry's term appears, allowing for endings ("anticommute" finds "anticommuting") and for names ("Lamb, Willis E." finds "Lamb"). If the term is not on that page, the link goes to the page marker. In Dyson about 40 % of index links find their term. The word found is wrapped in its anchor, so a viewer can highlight it.
* **Cross-references** that are links in the PDF stay links, and the whole reference is the link: "Section 3.5", "Chapter 3", "Appendix B", "Figure 1.1", "Table 2.1", "Problem 6.8", "(3.422)". The word decides the target, so "Section 2.1" goes to the section even if an equation (2.1) exists. In a list such as "Figures 4.3 and 4.7" each number is a link. References to other books are not links in the PDF and stay text.

### Pictures

Pictures are cut from the PDF as PNG files in `images/`; `fig-p049-1.png` is the first picture on PDF page 49. Labels that belong to a picture are part of the image, not of the text. Each picture gets a caption numbered through the book with its printed page, such as _Figure 3 (p. 43)_.

A page that is one picture and no text, such as the book's cover, is kept as that picture on its own page. On the PDF's first page it is the cover: it has no figure number and no caption.

### Tables and columns

A table is written as an HTML table with Markdown in its cells. Under every table that pdf2md builds stands a picture of the printed table in a fold, because a cell can be misread. A title or a note that the book sets as part of the table is a cell across all its columns, in a row marked `<tr class="title">` or `<tr class="note">`. A page set in two columns, such as an index, stays in two columns.

## The two forms of math

Formulas are written in one of two forms, chosen with "Math" in the window or `--math` on the command line.

**github** writes ``$`…`$`` in a sentence and a ```` ```math ```` block for a displayed equation. GitHub, GitLab and mdBun show it. Three things GitHub does made this form necessary, and shaped it:

* GitHub reads what stands between `$` signs as Markdown first. `A^{*} … B^{*}` turns italic; `\\`, `\{` and `\,` lose their backslash; a `$` beside a letter, digit, hyphen or quotation mark is not taken as math at all. In Dyson this spoiled 720 of 3,684 formulas. In the github form none are spoiled.
* GitHub breaks an equation that has a `\tag{12}` into a column of single symbols. The number is therefore written as text after the equation, `\qquad \text{(12)}`. mdBun turns it back into a tag at the right margin.
* GitHub adds a backslash to a line of a math block that ends in `\\`, and shows a paragraph that starts with an HTML comment as raw text. pdf2md writes a space after such a `\\`, and puts its invisible equation mark after a formula in a sentence, not before it.

**standard** writes `$…$`, `$$…$$` and `\tag{12}`. Most other Markdown programs know only this form and show the github form as code. Typing one of these commands into "Reader" switches "Math" to standard: `code`, `code-oss`, `codium`, `vscodium`, `code-insiders`, `obsidian`, `typora`, `marktext`, `zettlr`, `ghostwriter`, `joplin`, `logseq`, `pandoc`. The list comes from those programs' documentation, not from trying each.

**How GitHub draws math.** GitHub converts the LaTeX to MathML and leaves the drawing to the browser. Brackets, roots and sums grow only if the browser finds a math font it knows; Chrome on Linux looks for "Latin Modern Math". mdBun typesets with KaTeX, which brings its own fonts.

**Mending.** In either form, a formula that would not render is mended when written: two formulas in a sentence with nothing between them become one, math inside `\text{…}` is taken out of it, and two subscripts or two superscripts on one symbol become one. A mended formula renders. Whether it is the book's formula is another matter.

## PDFs that hold text

This is the path for a PDF made with TeX, where the math is text in math fonts. Everything is decided from the layout, measured against the book's own usual values: body font size, margins and line spacing.

### Pages and chapters

* **Printed page numbers** come from the running head ("THE DIRAC THEORY 6") or from a lone number at the foot of the page. A page without one (the page a chapter opens on, an empty page) counts on from the page before, in figures or in Roman numerals, and the pages before the first numbered one count back to page 1.
* **Chapters** start at headings in the chapter-title size: the most common large size that holds real words. Where chapters are numbered within parts ("IV.2"), that number's size is the chapter size, and a page that opens well down with a larger title than the sections' ("Preface", "Part I", "Appendix A") opens a file too. A frame or rule drawn around a title or its number is not a picture.
* **The contents** are read as lines "title … page", with dots between or with the number at the right margin. The name of a part, in larger type and without a page, is an entry of its own.
* **Section headings** are short lines that are wholly italic or bold with space above, or short plain lines with space above and below. Contents entries are matched to headings on the page they name, also when the wording differs a little. An italic line that starts with a small letter is not a heading. Nor is a line that leads straight into a displayed formula ("This can be rewritten as"), unless it is a styled title of a word or two.
* **Heading levels.** In a book without section numbers, the indentation in the contents decides `##` or `###`, and a heading the contents do not list goes by its type size. In a book that numbers its sections, the number decides: `1.3` is `#` and `1.3.1` is `##`, or one level deeper each where the chapter titles are at least a quarter larger than the `1.3` headings. An unnumbered heading takes the level of the numbered headings of its size.

### Text

* **Italic and bold** become `_…_` and `**…**`. Headings carry neither.
* **Centered text**, such as a title page, goes in `<div align="center">` and keeps its line breaks. A line counts as centered when its middle is at the middle of the column and it is clearly in from both margins.
* **Hard returns** are kept. A line that ends early, although the next line's first word would have fit, was ended on purpose, as in an address. It stays a line of its own (`<br>`), and becomes its own paragraph when there is also extra space above it.
* **Footnote markers** are small raised numbers attached to a word. End notes are matched by the longest run of rising marker numbers in reading order, so the 16 of "O¹⁶" cannot take note 16. Notes at the foot of a page match markers on that page.

### Displayed equations

A displayed equation is a line that is mostly math and indented, or one with an equation number at the right margin, together with the rows of fractions and limits around it. It is rebuilt from the positions of the glyphs and the drawn bars:

* **Fractions.** A bar with symbols right above and below it is a fraction. A clearly narrower bar inside is a nested fraction; one of the same width is a separate fraction below. Where no bar is found, a group above the main row that overlaps a group below is taken as a fraction.
* **Roots and overlines.** A bar right after a root sign is a square root. A bar just above a single symbol, with no numerator above it, is an overline (ψ̄).
* **Scripts and limits.** Small raised or lowered glyphs are scripts. Groups around Σ or ∫ are limits.
* **Several lines.** A system of equations with one number, or an equation continued on a line that starts with + or −, becomes an `aligned` block.
* **Accents** drawn as separate glyphs (˜ ˆ ˙ ¨) become `\tilde`, `\hat`, `\dot`, `\ddot`. A labelled underbrace becomes `\underbrace{…}_{\text{label}}`.

### Brackets and matrices

`pdftohtml` gives tall brackets no character, or drops them, so they are read from their outlines in the SVG. Straight strokes with the long stroke on one side are `[` or `]`. A curve bulging at mid-height is `(` or `)`. A thin bar is `|`. Two strokes meeting in a point are `⟨` or `⟩`. A curve whose outer edge is a sharp point at mid-height is `{` or `}`. The pieces of an extensible bracket are joined first.

Brackets are then paired as in text, nested ones included, and written as `\left[ … \right]`, `bmatrix`, `vmatrix` and so on.

* A lone `{` followed by two or more rows becomes `cases`.
* A lone `}` after several rows groups them: `\left. \begin{aligned} … \end{aligned} \right\}`.
* A bracket that opens on one line of a formula and closes on a later one is written with `\left\{ … \right.` and `\left. … \right\}`.
* Inside a matrix, small entries that stand free and form their own rows and columns are an inner `matrix`, as in Dyson's equation (15).

TeX's symbol font has a quirk: in the PDF's text, ⟨ ⟩ come out as "h" and "i", | and ‖ as "j" and "k", { } as "f" and "g", and capitals as calligraphic letters. These are mapped back.

### Symbols the text leaves out

A font names the glyph each character draws ("summationdisplay", "angbracketleft") and usually also gives its Unicode character. Where a font gives no characters, or not for all its glyphs, `pdftohtml` gives an empty piece of text. Some publishers' math fonts do this for the big symbols (∫, Σ, ∏, large brackets and roots), the angle brackets, the prime, the arrow over a vector and the stroke of the Feynman slash.

pdf2md reads the names from the PDF's fonts. The PDF lists the glyphs of a page in the order they are drawn, and so does the SVG, which also says where each one is. Each lost glyph is written into the empty piece at its place. A page where the two lists differ in length is left alone. The fraction bars of such a book may be drawn as thin lines, not as filled rectangles; both are read.

An arrow or a stroke belongs to the letter it is drawn over: `\vec{k}`, `\not{p}`.

| Variable | Shows |
|---|---|
| `PDF2MD_GLYPHDEBUG=1` | glyphs without a known name, glyphs that found no place, and the count written |

### Notation lists and pictures

* **Aligned columns.** Three or more lines whose columns start at the same positions, with words in the last column, become a LaTeX `array`.
* **Pictures** are found in the SVG: stroked or filled paths that are neither glyphs nor thin bars, and embedded images. Pieces close together are one picture. A group counts as a picture if it holds an image, or 3 or more pieces at least 15 pt square, or 8 or more pieces at least 40 pt wide. Text inside the picture, and short text just beside it, is a label and becomes part of the image. The picture is then rendered from the PDF with `pdftocairo -png`.

## PDFs made from ebooks

A PDF made from an ebook, with calibre or from Kindle, holds its prose as text but its math in two other ways: larger formulas as small pictures, and simple math inside sentences in the ordinary text font. pdf2md takes this path when the book has no Computer Modern fonts.

### Equation pictures

Each equation picture goes to [texify](https://github.com/VikParuchuri/texify), a math-recognition model that runs locally. The LaTeX it reads is put inline where the picture sits in a sentence, and as a displayed equation where it stands alone. Readings are cached by picture in `~/.cache/pdf2md/ocr/`.

* **Not every picture is math.** A picture stays a picture if it has a "Fig." caption, or if it is a diagram: long straight lines meeting at a corner, or one connected shape that is large both ways. Lettering such as a publisher's logo also stays a picture. A horizontal line becomes `---`. A picture of program code is read as text by [Tesseract](https://github.com/tesseract-ocr/tesseract) and written as a code block.
* **Very small pictures**, under 30 pixels wide, are single symbols such as a hatted **x**. They are read with white space added around them, because a symbol that fills its picture is read as some other letter. Where such a symbol sits inside a sentence that the PDF holds as one piece of text, the text is split at the gap it sits in.
* **Several lines.** A tall equation the model cannot read whole is read line by line. A picture of several lines that the model wrote as one long line is cut into its lines: where the model left a space before the "=" or operator a line starts with (it writes no other needless spaces), or, read line by line, if that reading agrees with the whole one or shows where the first line ends. Two lines with no "=" between them (a long formula wrapped inside a bracket or inside its words, two sentences of words and symbols) are cut where the letters and figures of the whole reading part as those of the two lines do. Lines that each start with "=" after a first line the whole reading starts with are taken as read one by one, whatever the whole reading made of the rest. A short line in small type that the whole reading left out is added, and lines of words that the whole reading gave as plain text are taken as read one by one. A bracket that opens on one line and closes on another gets a fixed large size. The rows become an `aligned` block, lined up at the "=". A multi-line equation that the model wrote as an `array` with uneven columns is turned into rows the same way.
* **Bold letters.** The model is unsure which letters are bold. The strokes of the picture tell whether it has any bold letter at all: ordinary letters of an ebook's equation pictures have strokes of 1.1 to 1.3 pixels, bold ones of 1.6 to 1.8. A reading that sets letters bold where no stroke of the picture is that thick loses its bold; a picture with thick strokes whose reading has no bold letter is listed in `check-equations.md`. Which letters are bold, the strokes do not tell reliably (a sum sign and a large letter are thick, too), so that is left to the two readings: where they set different letters bold, the list says **bold?**.
* **Words under an equation.** Some pictures label their parts with small words underneath, such as "bra" and "ket". The model drops such words or reads them as symbols. pdf2md reads the picture again without that band and puts the words back in a second row, each centred where it stood. The places are estimates, so a word may stand a little off. Sakurai has three such equations.
* **A second reading.** Each displayed equation is read again from the picture enlarged. Where the two readings are different formulas, formatting aside, the equation is listed in `check-equations.md`, biggest differences first.

**What is kept of the readings.** Everything read from a picture is kept in `~/.cache/pdf2md/ocr/` under the picture's key: the reading (`KEY.tex`), the second reading and how far the two differ (`KEY.second`, `KEY.verify`), the cells of a table (`KEY.cells`), and marks that say which checks the picture has had (`KEY.kind`, `KEY.lines`, `KEY.labels`, `KEY.small`, `KEY.bold`). `KEY.lines` and the first line of `KEY.cells` name the version of the rules that looked at the picture, so a reading kept from an earlier version is looked at again once when the rules have changed. That is why the first conversion after an update takes minutes, not seconds.

### Math in the text font

Inside a sentence, an italic letter is a variable and the brackets and "=" around it are ordinary characters. The PDF gives each line as pieces by typeface, with prose and symbols mixed in one piece. pdf2md cuts each piece into words, symbols, numbers, brackets and punctuation. A stretch with no word in it is one formula if something in it is surely math.

* **Surely math:** an italic letter with its primes; a Greek letter; a relation or operator that prose never uses, such as `=`, `≈`, `→`, `×`, `⟨`, `|`, `∂`, `ħ`; a subscript or superscript; a single bold letter (a vector); an equation picture.
* **Math only beside math:** numbers, round and square brackets, `+`, `−`, `/`, a lone upright letter as in `Z = 2`, and function names such as `sin`, `exp`, `Tr`.
* **Two or three italic letters** are symbols (`kR`, `dt`) unless they are a common word (`is`, `or`, `not`). Three need math right beside them. A longer italic word is emphasis.
* **Where a formula ends:** at a word, and at a comma, semicolon or full stop that is followed by a space and is not inside brackets. Brackets around a whole formula are prose. So are the hyphen in "x-polarized" and the apostrophe in "Green's".
* **Raised numbers.** After a word, a small raised number is a footnote marker. After a symbol, a number or a bracketed formula it is a power.
* **Linked numbers**, such as an equation reference or an index page, are never part of the formula beside them.

### Tables

Such a book sets a table on a grey panel, with a short rule under each cell of the head. The rules give the columns. The rows have no marks, so they are found from how the cells stand: centred in their row or at its top, whichever fits the table. A table that reaches the foot of its page continues at the top of the next: the two parts are written as one table, with the pictures of both under it. A table whose head starts at the foot of a page, with the rest on the next, gets those lines into its head. The caption, set centered on a panel on top of the table, is the table's title: one cell across all its columns. A note set on a panel of its own under a table is the table's foot, a cell across all its columns too, and part of its picture.

A table that the book holds as a picture is read cell by cell. The picture is cut from the page again at higher resolution. Its rows are the lines of text on it, and its columns end where no row has ink. Each cell is then read on its own, as words, as a whole number or as a formula. A picture that does not split into rows and columns this way keeps the model's reading of it as one array. Each cell is set as it is in the picture: at the left of its column, in its middle or at its right, by where its ink stands between the column's edges (a cell that fills the column is set as most of the others are). Where the book sets the caption centered on a panel on top of such a table, the caption is the table's title: one cell across all its columns, above the rows. Such a table needs no caption: every large picture that was read as a formula is looked at once, and is a table if two or more rules run unbroken across it and it parts into rows and columns. (A table picture with neither a caption nor rules cannot be told from a formula of several lines.)

## Scanned books

A scan is a PDF whose pages are pictures. pdf2md recognizes one by itself: next to no text, and an image on nearly every page. It has the pages read by [marker](https://github.com/datalab-to/marker), a page-reading model that runs locally. marker gives the prose, the formulas as LaTeX, the headings, the footnotes and the places of the pictures. pdf2md builds chapters, page markers, links and files from that as for any other book.

A page takes about 3 seconds on a recent graphics card. Pages read are cached in `~/.cache/pdf2md/scan/`, so a later conversion takes seconds and one that was stopped goes on where it was. pdf2md uses marker 1.10.2, the last version that runs inside Python alone. Tables of contents also need `pdftotext` or, for a scan without hidden text, Tesseract.

### Structure

* **Chapters** are told from the layout, because the height of a heading's box says too little about its type size. A page that opens with a heading and has no running head opens a chapter. "Chapter I.5" and the title under it are one title. Everything before the first "Chapter" or "Part" is front matter. A book without running heads goes by the heights of its headings, which is less sure.
* **Sections** are one level (`##`) unless they are numbered.
* **Headings marker could not read**, which happens with small capitals, get their words from the PDF's hidden text or from Tesseract.
* **Printed page numbers** come from the running heads and feet. A page that shows none, or whose number was misread, gets its number from the pages around it.
* **A part's title page** ("PART IV" and the part's name) becomes the title of the part's file.
* **The cover**, a first sheet that is mostly picture, is one picture of the whole page.

### Text

* **Centered pages**, such as a title page or a dedication, stay centered and keep their line ends. On other pages, a block of several lines that stops short of the right margin keeps its line ends; a block as wide as the text is a paragraph.
* **Lines read too short.** Where marker read much less than the line holds, the words come from the PDF's hidden text, if that text contains marker's words and more.
* **Words on a picture** are part of the picture.
* **Code.** What marker calls program code is written as a code block only if it looks like code; marker also calls an index page code.
* **An index in two columns** stays in two columns. marker does not say which lines are indented continuation lines, and sometimes mixes the two columns. So the column and indentation of each line come from the positions of the words in the PDF's hidden text, and the wording from marker where it has the line. Continuation lines are joined to their entry. A scan without hidden text keeps its columns, but each line is an entry. A line known only from the hidden text is mended by marker's words of the same page: a word marker does not have becomes the one word it has with the same short strokes ("fennions" is "fermions", "Feynrnan" is "Feynman"), a piece that is neither word nor page number becomes marker's formula at that place ("0.4 theory" is $`\varphi^4`$ theory), and page numbers run together get their space.
* **Lines given one by one** (an address, or the catalogue entry on a copyright page) are one paragraph with its line ends kept. A line set in from the first starts with em spaces, one for each line height it is set in by.

### Math

* **Symbols marker left as plain text** are made formulas by their shape: a letter or two followed by an argument in brackets (`J(x)`, `SU(2)`); a single letter other than "a", "A", "I" that is not an initial, a label or a unit; a short relation without spaces (`m=0`).
* **Displayed equations** keep their numbers. Each is read a second time by texify from a picture cut from the page, and the ones the two readers read differently are listed in `check-equations.md`. Without the texify environment there is no list.
* **Unfinished formulas.** A formula marker did not finish, such as a matrix cut off, is replaced by the second reading if that one is whole.

### Footnotes

marker does not see every small raised number in the text. Where a page has a footnote without its marker, pdf2md looks for the marker on the picture of that page and of the pages before and after: a small piece of ink set high in its line, right after a word (or after the stop that follows a number or a bracket), that most of five readings take for the note's number. It is put into the text after that word if the word, with the words before it, is found there in one place only, and if the markers of the page still rise from one to the next.

### Contents and tables

* **The table of contents** is read line by line from the PDF's hidden text or by Tesseract, because marker reads tables cell by cell, which shuffles a contents page. That text is poor, so an entry whose page opens with a title takes the title as marker read it. Contents set in columns are written as a table of number, title and pages.
* **Other tables** are kept as marker read them if their rows and cells hold together: at least two rows and two columns, rows of similar length, most cells filled. A table that does not hold together stays a picture. A table numbered like an equation, "(16)" beside or under it, has that number written under it as an equation number, so references to it find the table.

## What is kept between conversions

Before a book is converted again, pdf2md collects what the reader changed in the old files, from the chapter files and from `book.md`.

* **Equations.** Every equation read from a picture or a scan carries an invisible mark naming its picture, `<!-- pdf2md:eq … -->`. A file in the book's folder records what the last conversion wrote for each. An equation that now differs from that record is the reader's correction. It is stored in `~/.local/share/pdf2md/corrections/` and used from then on in place of the model's reading. Both forms of math are read, so a book keeps its corrections when it changes form.
* **Table cells.** Every table carries a mark above it, `<!-- pdf2md:table … -->`, and a corrected cell is kept by table, row and column. The rows of the title and of the note are not counted (a wrong word there is corrected with a fix, as in running text), and a cell that differs from what was written only in the files its links go to (as in `book.md`) is not a correction. If a later version of pdf2md reads the table differently, it counts as another table and the old corrections are not applied. The table of contents is made anew each time.
* **The picture under a table** keeps the state the reader left it in: open, folded, or deleted.
* **Notes.** A `<details class="mine">` block or an alert box goes back after the paragraph or equation it followed. That block is found by its first words, or else by the nearest anchor above it. A note whose place is gone goes to the end of its chapter under "Your notes whose place was not found".
* **Markers in a sentence** that point to a note go back after the same words in the same paragraph, or at the paragraph's end if those words changed.
* **Prose fixes**, `<span class="fix" data-was="…">`, are redone in the same paragraph.

A copy of the notes and fixes is kept in `~/.local/share/pdf2md/notes/`. Notes follow a chapter whose file name changes. Any other edit to the text is not kept.

## The app window

* **Replace or keep both.** If the book's folder exists, Replace moves the old files to the Trash as one folder, "<name> (previous)", once the new conversion has succeeded. The new files are written into the same folder, so a terminal or editor open in it sees them at once. Keep both writes to "<name> (1)".
* **Reader.** The command gets the book's folder as its last argument. `{folder}` or `{file}` in the command puts the folder or `book.md` elsewhere. Left empty, `xdg-open` opens the folder.
* **Converted books.** The list shows the books converted with the app, and books found beside them and in the folder browsed last, newest first.
* **Progress.** The bar goes by time. A conversion of more than 30 seconds also lists its steps: done, running or waiting. Estimates come from the speed measured so far. Before a step has done anything they are guesses, set several times too high on purpose. A step that has counted nothing new for 30 seconds says so.
* **Settings** are kept in `~/.config/pdf2md/`.

## Measurements

* **Dyson** (TeX, 149 pages): every chapter, contents entry, reference and index entry is linked, and all 93 footnotes. KaTeX renders every displayed equation. It is the sample in [`samples/dyson`](../samples/dyson) and the reference for regressions.
* **Sakurai** (ebook-made, 771 PDF pages): about two minutes for the first conversion on an RTX 5080, about 15 seconds after that. All 12,665 formulas are written where a Markdown reader takes them as math.
* **Zee** (scan, 537 pages): half an hour for the first conversion, half a minute after that. 10,262 formulas, all of which render. Of 1,190 displayed equations read twice, 403 are listed to check, 40 with large differences. Of 119 footnotes, 116 are linked to their markers, 43 of those found on the pictures of the pages.
