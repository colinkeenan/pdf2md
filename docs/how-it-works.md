# How pdf2md works

The [README](../README.md) says what pdf2md does and how to use it. This page describes what it writes and how it reads each kind of PDF, including the rules it applies and where they stop working. The rules were worked out on five PDFs of four books: Dyson's *Advanced Quantum Mechanics* (made with TeX), Sakurai's *Modern Quantum Mechanics* (made from an ebook), Zee's *Quantum Field Theory in a Nutshell* twice, as a scan of the first edition and as the publisher's PDF of the second, and the UCSD lecture notes *Quantum Physics (Physics 130)* (made with TeX), the first book converted after the rules were written. [Where the rules come from](#where-the-rules-come-from) says which book each part was fitted to and which numbers are guesses.

- [Where the rules come from](#where-the-rules-come-from)
- [The source](#the-source)
- [What is written](#what-is-written)
- [The two forms of math](#the-two-forms-of-math)
- [PDFs that hold text](#pdfs-that-hold-text)
- [PDFs made from ebooks](#pdfs-made-from-ebooks)
- [Scanned books](#scanned-books)
- [What is kept between conversions](#what-is-kept-between-conversions)
- [The app window](#the-app-window)
- [Measurements](#measurements)

## Where the rules come from

No rule asks which book it is reading. Every rule is stated by what is on the page. But each was written after seeing a fault in one of five PDFs, and was checked only against those. Under each heading below, a line in italics names the PDF the rules of that part were worked out on and how far they can be expected to carry:

* **general**: follows from how printed pages are made. It should hold for another book.
* **fitted**: the idea is general, but the numbers in it were set by looking at these books. Another book may need them moved.
* **one kind of PDF**: holds only for a PDF made the same way (the same fonts, the same program, the same publisher's design).

The four PDFs:

| Short name | What it is | What is special about it |
|---|---|---|
| Dyson | made with TeX, 149 pages | Computer Modern fonts; no section numbers; end notes |
| Sakurai | made from an ebook with calibre, 771 pages | formulas are pictures; math in sentences is in the text font; tables on grey panels |
| Zee 1 | scan of the first edition, 537 pages | pages are pictures. **This PDF is gone**: the scan path has had no book to test it on since November 2026 |
| UCSD | made with TeX, 548 pages, letter paper | Computer Modern like Dyson, but numbered chapters and sections, vectors under arrows, and figures made in other programs with their own fonts |
| Zee 2 | publisher's PDF of the second edition, 605 pages | MathTime fonts that give no characters for many symbols; framed chapter numbers; exercises and notes in smaller type |

### Numbers that are guesses

Each of these was set to make the books above come out right, and none was derived. They are the first places to look when another book goes wrong. For the older ones, the book named is taken from the notes of the work, not from the history of each line.

| Number | What it decides | Where | First set on |
|---|---|---|---|
| gap > 0.15 × type size | a space between two pieces of a line | `blocks.rs` `emit_line` | Dyson, then Zee 2 |
| gap ≤ 0.3 × type size | no space before a comma or full stop | `blocks.rs` `emit_line` | Zee 2 |
| size < 0.85 × the line's | a piece is a script (raised or lowered) | `layout.rs` `make_line`, `group_lines` | Dyson |
| shares < half its height | a piece only touches a line and waits | `layout.rs` `group_lines` | Zee 2 |
| 2 × type size away, 0.3 × beside | a stray piece moves to the line beside it | `layout.rs` `group_lines` | Zee 2 |
| math share ≥ 0.7, or ≥ 0.25 and indented by 4 × body | a line is a displayed equation | `blocks.rs` `display_flags` | Dyson |
| ≤ 1.6 × type size under prose | an all-math line is the paragraph's last line, not a display | `blocks.rs` `display_flags` | Zee 2 |
| within 3 lines, 2 × body right of a bracket | a row of numbers is a matrix row | `blocks.rs` `display_flags` | Zee 2 |
| 0.75 × type size above | an accent is over that letter | `mathtex.rs` `put_accents` | Zee 2 |
| letter widths 0.27 to 0.8 em | which letter of a piece an accent or picture is at | `layout.rs` `char_width` | Sakurai |
| room ≥ 0.5 × type size | a line ended early on purpose (hard return) | `blocks.rs` `HARD_RETURN_ROOM` | Sakurai |
| a lone number in the top 13 % of the page (words: 9 %) | the page's printed number | `pages.rs` `prepare` | Dyson, then UCSD |
| under 1 % of the characters, on under 1 page in 5 | a font letters figures only | `layout.rs` `figure_fonts` | UCSD, checked on Zee 2 |
| labels within 4 lines, strokes within 2 | how far a lettered picture reaches | `layout.rs` `lettered_figures` | UCSD |
| gap above ≥ 1.4 × line spacing | a heading, or a new paragraph | `blocks.rs` | Dyson |
| chapter title ≥ 1.25 × section heading | sections go one heading level down | `structure.rs` | chosen; Sakurai is below it |
| type ≥ 1.2 or 1.3 × body in a frame | a framed title, not a table | `layout.rs` `read_drawings` | Zee 2 |
| 3 pieces of 15 pt, or 8 of 40 pt | drawn paths are a picture | `layout.rs` `parse_svg_figures` | Dyson |
| under 30 px wide | an equation picture is a single symbol | `math_ocr.py` | Sakurai |
| a shape 56 px both ways | an equation picture is a diagram | `math_ocr.py` `BIG_SHAPE` | Sakurai |
| strokes 1.1–1.3 px, bold 1.6–1.8 px | a picture has bold letters | `math_ocr.py` | Sakurai (800 pictures, at 96 dpi) |

### What the fifth PDF showed

UCSD was the first book converted with rules written for others, and it is the same kind of PDF as Dyson. It converted without stopping, and its prose, formulas and links were mostly right at once (8,800 formulas, all but one rendering). Its structure was not: it came out as 11 chapters named after figure labels, with 110 of 548 page numbers. Five things had been fitted to Dyson without anyone knowing:

* the chapter size was taken from a lone "0" on a figure's axis (now: from numbered titles first);
* the page number at the head of a page was looked for in the top 9 % of the page, and letter paper puts it at 9.7 % (now: a number alone down to 13 %);
* a drawing counted as a picture only by the number and size of its strokes (now: also by lettering in a font of its own);
* TeX's arrow over a vector was read as ħ, which Dyson never showed because it sets vectors bold, except in five places where it was wrong there too;
* a raised number after a closing bracket was taken for a footnote's mark.

So "the same kind of PDF" carried the math and the prose, and did not carry the layout numbers.

### What the sixth PDF showed

Lucid's *Advanced Theoretical Physics* is again a TeX book, and again its math and prose carried over while its layout did not. It came out as 132 files named after its examples, with the title page in four of them and the first page of the contents as a code block. Four things had been fitted to the books before it:

* it is printed for both sides of the paper, so its text stands 65 pixels further right on the odd pages than on the even ones, and every rule that asks "is this line at the margin?" was wrong on half the pages (now: the pages are moved onto one margin first). UCSD is printed the same way and had the same faults, unnoticed: paragraphs cut at page turns, the contents nested by page;
* its chapters are called "Chapter 3" on a line of their own, above the name (now: such lines say the chapter size, after numbered titles, and the two lines are one title);
* its pictures are JPEGs, and only PNGs were read (the equation pictures of an ebook are PNGs);
* its body type is 12 pt, and a "tall bracket" was anything over 10.8 pt: every italic f became a bracket (now: taller than the book's own type).

### Known weak points for another book

* **Which path a text PDF takes** is decided by whether it has Computer Modern or AMS fonts. Without them it is read as an ebook-made PDF. Zee 2 takes the right path only because it uses a few Computer Modern symbols beside its MathTime fonts. A book set wholly in another publisher's math fonts would be sent down the ebook path and its math fonts ignored (`blocks.rs` `is_text_math`).
* **Math fonts are known by name**: Computer Modern, AMS, Euler, RSFS and MathTime (`layout.rs` `is_math_font`). Text in any other math font is taken for prose.
* **Glyph names** are known for the fonts of Zee 2. Another font family names its glyphs differently, and unknown names are left out (`PDF2MD_GLYPHDEBUG=1` lists them).
* **The scan path** depends on marker 1.10.2 and was fitted to one scan that is gone.
* **Sizes in pixels** assume equation pictures at 96 dpi, as calibre makes them.

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
│   ├── src/course.rs      a folder of saved web pages and PDFs as one book
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
| `PDF2MD_LINEDEBUG=1` | the pieces of each text line as they were grouped, with their place and height (with `--dump`) |
| `PDF2MD_DISPDEBUG=1` | the rows of each displayed formula and the lines it was split into |
| `PDF2MD_FRACDEBUG=1` | what each fraction bar took from the lines around it |
| `PDF2MD_SVGDEBUG=1` | the tall outlines read from the SVG and the bracket each was taken for |
| `PDF2MD_TMDEBUG=1` | how doubtful lines of text-font math were cut into words and symbols |
| `PDF2MD_TABLEDEBUG=1` | the grey panels found and their lines |
| `PDF2MD_MARKDEBUG=1` | for each footnote marker found on a page's picture, the words looked for and how many places fit; the markers not placed |
| `PDF2MD_OCR_PYTHON`, `PDF2MD_SCAN_PYTHON` | another Python that has texify or marker |

## What is written

### Files

*Worked out on: all four; the split of front matter on Sakurai and Zee 2. General, and the file naming is the reader's choice, not a finding.*

Each chapter is a file named by its number, such as `03-theory-of-angular-momentum.md`, with all its sections. A book whose chapters carry no numbers is numbered in order. Appendices and the index count on after the last chapter.

`00-front-matter.md` holds everything before chapter 1, each title page or preface under its own heading. The table of contents is a file of its own right after it. Where the book prints prefaces or a foreword after its contents, the files follow the book: `00a-title-pages.md` holds what comes before the contents, `00c-additional-front-matter.md` what comes after them. A page printed after the contents that holds only a title (the title of the lectures, before the first chapter) heads the file after the contents, with its page marker. A half-title, the book's title printed once more on a page of its own, stands at the top of the file that follows it, above that file's title: above the word "Contents", or above the first part. It is never the last thing in the file before. An empty page belongs to what comes before it.

`book.md` is the whole book in one file, with all links pointing inside it, so one search covers the book. Where the PDF tells whom the book is by, `book.md` opens with a hidden line, `<!-- pdf2md:by Lucid -->`: for a course, its school (the capitals a course folder's name opens with, "MIT", or those of a title that names the course by them, "(UCSD Physics 130)"); else the first author's last name, from the PDF's author field or from the name on the first page. mdBun names the book's tabs by it. If the name is missing or wrong, write the line yourself: a line that is there is kept when the book is converted again.

### Page markers

*Worked out on: all four. General. The last bullet is for one kind of PDF (ebook-made, Sakurai).*

Where a printed page starts there is a marker, <sup>[p. 32]</sup>, with the anchor `#p32`. It stands on a line of its own under a faint bar.

Above the bar, at the right, stands the marker of the page that ends there, <sup>[p. 31]</sup>. With one bar in the window both pages are named, and each marker opens its own page of the PDF. Only the marker under the bar carries the anchor. The page that ends is the page before in the PDF; in an ebook-made PDF it is the page of the marker before.

* **At the top of a file** the bar and marker of the page the file opens on stand above the file's title: the title is printed on that page, not at the foot of the page before. That first bar has no marker above it, since nothing of the page before is in the file.
* **Among centered lines** (a title page) the bar and marker stand at the left like all others.
* **In the middle of a paragraph** the bar and marker are written into the sentence as a floating block as wide as the text. The browser puts it under the current line and goes on filling that line, so the text above the bar runs to the right margin as on the printed page, at any window width. A viewer that drops `style` attributes shows the marker inside the sentence, without the bar.
* **In the middle of a sentence that pictures break off.** A book sets its pictures at the top of a page, or where there is room, whether or not a sentence is going on there: the sentence stops at the end of a full line and goes on under the pictures and their captions, often a page or two later. Written as two paragraphs, the first would end in a short line and the caption under it would read as its next words. The two parts are written in one block, `<div class="over-page">`, with the page's bar, the pictures, their captions and whatever else stands between them (a table, a row of portraits) in a float as wide as the text. A viewer whose style sheet has `.over-page > p { display: inline }`, as mdBun's has, sets the two parts as one run of words: the line above the bar is filled with the words that follow, and the rest goes on under the last caption. Any other viewer, GitHub too, shows two paragraphs with the pictures between. The line that goes on is found as a line over a page turn is: it must be the next line of the paragraph that stopped (same type, at the margin, in a book that sets a paragraph's first line in; elsewhere it must also not open with a capital letter), with nothing but pictures, captions, tables and page marks in between. A word cut there by a hyphen is put together in the part that goes on. It works inside a bulleted item too, which then goes on behind its picture. *Worked out on Lucid (38 places) and Zee 2 (34); one in the article.*
* **The link.** Each marker opens that page of the PDF in a new tab: `book.pdf#page=38`, counted in sheets of the PDF, which is how browsers count. `book.pdf` in the book's folder is a symbolic link to the source PDF, made anew at each conversion. If you move the PDF, convert again.
* **Pages without a printed number** get <sup>[PDF p. 5]</sup> (anchor `#pdf-p5`), every one of them, an empty page too, so that no page of the PDF seems to be missing. An empty page's bar stands before the next page that holds something. The contents name a piece that comes before the first printed page number (a preface) by that PDF page, too.
* **Inside a list**, such as the contents or the index, a marker on a line of its own would end the list and turn the indented entries after it into code. There the marker is attached to the entry before it. The bar then starts at that entry's indent, and the entries after it stay under their part or chapter.
* **Ebook-made PDFs** do not have one PDF page per printed page. The link opens the PDF page on which the printed page starts, and only the pages before the first and after the last printed page get PDF markers.

### Footnotes and references

*Worked out on: Dyson (end notes), Zee 1 and Zee 2 (notes at the foot of the page). General.*

A footnote printed at the foot of a page is written at the foot of that page, just above the next page's bar. Notes and references printed at the end of the book move to the end of the chapter that cites them. A marker links to its note, and the note has a ↩ link back for each place that cites it. A footnote whose marker was not found still stands at the foot of its page and says so.

### Contents, index and cross-references

*Worked out on: Dyson (index), Sakurai (cross-references that are links in the PDF), Zee 2 (contents). Fitted: the reference words ("Section", "Figure", …) are English, and a book whose PDF carries no links gets none.*

* **Contents entries** link to chapter files and to headings inside them. Each shows its printed pages, a range for a chapter ("pp. 4–21") and a page for a section, linked to that page's marker.
* **Index page numbers** link to the word on that page where the entry's term appears, allowing for endings ("anticommute" finds "anticommuting") and for names ("Lamb, Willis E." finds "Lamb"). If the term is not on that page, the link goes to the page marker. In Dyson about 40 % of index links find their term. The word found is wrapped in its anchor, so a viewer can highlight it.
* **Cross-references** that are links in the PDF stay links, and the whole reference is the link: "Section 3.5", "Chapter 3", "Appendix B", "Figure 1.1", "Table 2.1", "Problem 6.8", "(3.422)". The word decides the target, so "Section 2.1" goes to the section even if an equation (2.1) exists. In a list such as "Figures 4.3 and 4.7" each number is a link. References to other books are not links in the PDF and stay text.

### Pictures

*Worked out on: Dyson; the cover on Sakurai and Zee 2. Fitted.*

Pictures are cut from the PDF as PNG files in `images/`; `fig-p049-1.png` is the first picture on PDF page 49. Labels that belong to a picture are part of the image, not of the text. Each picture gets a caption numbered through the book with its printed page, such as _Figure 3 (p. 43)_.

The book's own caption in lines as long as the text's ("Figure 1.2: This is the Cartesian plane …") is set single-spaced, in a block with `line-height:1.1` (the text has 1.6 in mdBun), so that it reads as one thing under its picture and not as the next paragraph. A short caption set centered stays as it is.

A page that is one picture and no text, such as the book's cover, is kept as that picture on its own page. On the PDF's first page it is the cover: it has no figure number and no caption.

### Tables and columns

*Worked out on: Sakurai (first paragraph), Zee 2 (frames and grids). Fitted. The framed chapter numbers and the line under the title are one kind of PDF: Zee 2's design.*

A table is written as an HTML table with Markdown in its cells. Under every table that pdf2md builds stands a picture of the printed table in a fold, because a cell can be misread. A title or a note that the book sets as part of the table is a cell across all its columns, in a row marked `<tr class="title">` or `<tr class="note">`. A page set in two columns, such as an index, stays in two columns. Columns, and a row of pictures each over its name, are a table of one row with `table-layout:fixed` and `overflow-wrap:anywhere`: its columns keep their share of the width whatever is in them, so in a narrow window (a phone) a picture is made smaller and a long name or index line is broken, and the page is not made wider than the window.

**Text in a frame of drawn lines, or in a grid of them,** is a table whose cells are the fields of the grid; a frame alone is one cell. A line of the page that crosses several fields is cut at the upright lines. A cell of symbols only is one formula. A cell with words keeps its lines where the frame has one column. The number of the equation at the margin is written under the table. The frame of a chapter's number or title, in larger type, is no table: it is drawn in the heading, around the number ("I.1"), the first letter or the whole title as the book has it. Where the book draws a line under the title from the frame to the right margin, the heading has that line, and the frame stands on it. The file's name and the links to it are the plain title. A viewer that drops `style` attributes shows the heading without frame and line.

## The two forms of math

*Worked out on: Dyson, against GitHub as it behaved in November 2026. General for any book; it depends on GitHub, which may change. The github form was checked on GitHub with Dyson only.*

Formulas are written in one of two forms, chosen with "Math" in the window or `--math` on the command line.

**github** writes ``$`…`$`` in a sentence and a ```` ```math ```` block for a displayed equation. GitHub, GitLab and mdBun show it. Three things GitHub does made this form necessary, and shaped it:

* GitHub reads what stands between `$` signs as Markdown first. `A^{*} … B^{*}` turns italic; `\\`, `\{` and `\,` lose their backslash; a `$` beside a letter, digit, hyphen or quotation mark is not taken as math at all. In Dyson this spoiled 720 of 3,684 formulas. In the github form none are spoiled.
* GitHub breaks an equation that has a `\tag{12}` into a column of single symbols. The number is therefore written as text after the equation, `\qquad \text{(12)}`. mdBun turns it back into a tag at the right margin.
* GitHub adds a backslash to a line of a math block that ends in `\\`, and shows a paragraph that starts with an HTML comment as raw text. pdf2md writes a space after such a `\\`, and puts its invisible equation mark after a formula in a sentence, not before it.

**standard** writes `$…$`, `$$…$$` and `\tag{12}`. Most other Markdown programs know only this form and show the github form as code. Typing one of these commands into "Reader" switches "Math" to standard: `code`, `code-oss`, `codium`, `vscodium`, `code-insiders`, `obsidian`, `typora`, `marktext`, `zettlr`, `ghostwriter`, `joplin`, `logseq`, `pandoc`. The list comes from those programs' documentation, not from trying each.

**How GitHub draws math.** GitHub converts the LaTeX to MathML and leaves the drawing to the browser. Brackets, roots and sums grow only if the browser finds a math font it knows; Chrome on Linux looks for "Latin Modern Math". mdBun typesets with KaTeX, which brings its own fonts.

**Mending.** In either form, a formula that would not render is mended when written: two formulas in a sentence with nothing between them become one, math inside `\text{…}` is taken out of it, and two subscripts or two superscripts on one symbol become one. A mended formula renders. Whether it is the book's formula is another matter.

## PDFs that hold text

This is the path for a PDF made with TeX or by a publisher's typesetting system, where the math is text in math fonts (Dyson and Zee 2). Everything is decided from the layout, measured against the book's own usual values: body font size, margins and line spacing.

### Pages and chapters

*Worked out on: Dyson, then Zee 2 (chapters numbered within parts, contents with the page at the right margin). Fitted: what counts as "larger" and "short" was set on these two.*

* **Printed page numbers** come from the running head ("THE DIRAC THEORY 6") or from a lone number at the foot of the page. A page without one (the page a chapter opens on, an empty page) counts on from the page before, in figures or in Roman numerals, and the pages before the first numbered one count back to page 1.
* **Margins.** A book printed for both sides of the paper has its text further right on one side's pages. Where the full lines of the odd pages and of the even pages begin and end at places that differ by the same amount, the pages further right are moved over, so that the book has one left and one right margin (`layout.rs` `even_margins`; pictures are cut from where they really are).
* **Running titles and the line at the foot.** The running head ("CHAPTER 2. VECTOR ALGEBRA 12") and a line that ends page after page (a copyright line under every page) are taken out of the text, and shown where the page turns: the title, without the page's number, in small type behind the marker of the page that begins; the foot line before the marker of the page that ends. A page that has none (the page a chapter opens on) shows none.
* **Chapters** start at headings in the chapter-title size: the most common large size that holds real words. A book that sets "Chapter 3" on a line of its own above the chapter's name says its chapter size by those lines, and the two lines are one title, written in two lines; the file is named by the name. Where chapters are numbered within parts ("IV.2"), that number's size is the chapter size, and a page that opens well down with a larger title than the sections' ("Preface", "Part I", "Appendix A") opens a file too. A frame or rule drawn around a title or its number is not a picture.
* **The contents** are read as lines "title … page", with dots between or with the number at the right margin. The name of a part, in larger type and without a page, is an entry of its own.
* **Section headings** are short lines that are wholly italic or bold with space above, or short plain lines with space above and below. Contents entries are matched to headings on the page they name, also when the wording differs a little. An italic line that starts with a small letter is not a heading. Nor is a line that leads straight into a displayed formula ("This can be rewritten as"), unless it is a styled title of a word or two.
* **Heading levels.** In a book without section numbers, the indentation in the contents decides `##` or `###`, and a heading the contents do not list goes by its type size. In a book that numbers its sections, the number decides: `1.3` is `#` and `1.3.1` is `##`, or one level deeper each where the chapter titles are at least a quarter larger than the `1.3` headings. An unnumbered heading takes the level of the numbered headings of its size.

### Text

*Worked out on: Dyson; hard returns on Sakurai; the last five bullets on Zee 2. Fitted. "Lines" is general and was the largest single fault found in Zee 2; it also improved Dyson and Sakurai.*

* **Italic and bold** become `_…_` and `**…**`. Headings carry neither.
* **Centered text**, such as a title page, goes in `<div align="center">` and keeps its line breaks. A line counts as centered when its middle is at the middle of the column and it is clearly in from both margins.
* **Hard returns** are kept. A line that ends early, although the next line's first word would have fit, was ended on purpose, as in an address. It stays a line of its own (`<br>`), and becomes its own paragraph when there is also extra space above it.
* **Footnote markers** are small raised numbers attached to a word. End notes are matched by the longest run of rising marker numbers in reading order, so the 16 of "O¹⁶" cannot take note 16. Notes at the foot of a page match markers on that page. A line that begins with a small raised number begins a note's entry, unless a small number stands right under it: then the line begins with a stacked fraction ("½μ²ϕ² term") and goes on its paragraph. Nor is it an entry if an element's symbol stands right against the number: that is an isotope's mass number ("¹³³Cs, of this alkali atom"), and a note's text stands off its number. Within a line, a raised number is no marker where it belongs to a formula: over a drawn bar it begins a small fraction's numerator (the 2 of 2π/λ); right against a capital letter of a formula and off the word before it, it is that letter's own with whatever is raised beside it (the ³P₁ state, ²ˢ⁺¹L); after a bracket whose last letter has an accent drawn over it, it is the bracket's power ((x − x̄)²). A raised figure that goes on with a slash and a figure is one raised fraction (the 7/2 of (a² + 1)⁷ᐟ²).

* **The name under a quotation** ("—N. P. Ong, Princeton University") is the last line of the quotation's paragraph, on a line of its own, and ends it: what follows is another paragraph, also where the page ends with the name.
* **Over a page** a paragraph goes on only in the type size it is set in.
* **Lines.** Pieces of text are of the line their middle is in. A math font's box for "=" or "−" is taller than the letters and reaches into the line above; such a piece waits until all lines are known and goes to the one it shares most of its height with. A root sign is of the line its foot stands in. A few small letters that touch no line and stand right over one, at the head of an integral sign, are that integral's upper limit (in a sentence, or in an exponent): they are of the line under them, not a line of their own, which the integral sign would follow. So are small letters that touch the exponent under them (the top of an exponent that holds an integral), and a sign is of no line in whose run of words it would stand.
* **Indices and limits between two lines.** A lower index hangs into the height of the next line. A small piece over the inside of a run of words, or clear above the letters of its line with a small letter of the line above right over it, is of the line above (ϵ with λ₁ over μ₁). The lower limit of a sum set in a sentence is the small piece right under its sign, with what stands beside it ("n", "=", "1"); its upper limit the one at the top of the sign's box. Dots of "and so on" raised among the indices of a symbol do not begin a line: their box stands half a line too high. Small pieces left as a line of their own in the row of a line's raised pieces, among that line's pieces, are of that row (the numerator of a small fraction in a sentence, one root sign of which stood a hair higher and took the bracket's exponent with it).
* **A column or small matrix in a sentence** has one row above the line of words and one below, between a pair of big brackets. Both rows and the brackets are of the line of words.
* **Words and formulas.** A space is written where the gap between two pieces is wider than a sixth of the line's own type size (notes and exercises are set smaller). The name of a function before its formula ("log", "tr", "cos", "det") is the formula's first word. A number beside a formula's open "=", "→" or "×" is its other side. Between a word and a formula a space squeezed to two points of the page's grid (the pieces' places are whole numbers) is still a space; "th", "st" and "s" behind a formula stay against it. In Zee 415 words stood against a formula; about 60 do now (ordinals, and words whose space the page gives as one point or none).
* **Brackets across the pieces of a sentence** (`brackets.rs`, for each paragraph). A formula's figures, commas and dots are set in the text's font, so a list in brackets comes in pieces: "(μ = 0" + ", 1, 2, 3)", "V(q₁, q₂" + ", . . . , " + "q_N)". A formula with a bracket left open goes on through figures, commas, dots, signs, "or", "h.c.", a function's name or one word, and further formulas, up to the bracket that closes it. A formula that closes a bracket it never opened takes it from the text right before it, with a figure or a function's name ("(0" + "+ 1)", "tr[1" + "/(z − H)]"). A bracket still without its partner is the sentence's own, where the sentence has the partner: it goes back to the text ("(where c = 1)"), with the full stop beside it. A bracket with an exponent on it, an interval, and brackets inside an index are left alone. In Zee 698 formulas in sentences had a bracket without its partner, 10 have now; Dyson 46 → 3, UCSD 168 → 7.
* **The two sides of a sign** (`brackets.rs` `sides`, for each paragraph, before the brackets). A formula that ends with "+", "−", "=", "→", "≪" … takes the figure the text goes on with ("d = D +" + "1"), or its dots of "and so on" ("k₁ + k₂ +" + ". . ." + "+ kₙ"); one that begins with such a sign takes the figure the text ends with ("24" + "+ 2 = 26"). Two formulas with such a sign between them are one, also over a function's name or "h.c." ("S(P) =" + "Re" + "tr U…"), and so is a list that runs through dots ("i = 1" + ", 2, . . . " + "n"). A "−" with a word right behind it is a hyphen ("(D + 1)" + "-dimensional"). In Zee formulas in sentences that end with such a sign went from 151 to 51 (most of the rest lead up to a displayed equation), formulas standing side by side from 41 to 9. "/" counts as such a sign ("1" + "/r"). A raised figure of the text's font is the power of what it stands on: of a function's name before a formula ("sin" + ² + "θ"), of a formula that ends with a bracket or a figure, of "10", and of another figure where a formula goes on behind it ("3" + ² + "− 1 = 8"); after a word or a year it stays a note's mark. An exponent or index that came out with nothing under it right behind a bracket is the bracket's: "(length)" and its power, "[1]" and its star. A function's name set right against the bracket of its formula is the formula's ("det" + "[A]", "tr log" + "[…]", also between two parts of one formula); any other word that touches a formula gets its space, and so does a unit of energy behind one ("10¹⁹" + "Gev"). A function's name set against the figure before it ("−2tr(…)", "4sin²(…)") is cut off behind the figure and joins its formula. After the brackets are mended the two sides of a sign are looked at once more: "L − L₀ =" + " [log" + "(s₀/s) …]" is two formulas side by side only then. Signs and figures set in the text's font right behind a formula are the formula's too: "a" + " + 0 = " + "a" is one formula, and "n" + " = 2." is "n = 2" with the full stop left to the sentence. A sign alone, with a word behind it, is left to the text.
* **Italic and bold words.** `pdftohtml` marks a font as italic in some PDFs and not in others; TeX's italic and slanted text fonts (CMTI, CMSL and their bold forms) are italic by their names (Lucid's emphasised words had all come out plain). A phrase in italics or bold that goes on over the end of a line is one piece, and a word cut at the line's end is put together as in plain text. A number the PDF links ("Eq. 9.3.14" in an italic passage) stays plain, so that it becomes a link.
* **The last line of a paragraph** that is all math is no displayed equation: it starts where the line above it starts, at the paragraph's line spacing (or under the words of an exercise whose number stands out at the margin). A word alone between two formulas of a passage in small type ("and", "becomes") is text.
* **An indented block** (exercises, solutions) has paragraphs of its own, each with its first line set in: the second line, back at the block's edge, goes on with it.

### Bulleted lists

*Worked out on: Lucid (450 items; no other of the books has any).*

A line that opens with "•" begins an item of a list. Its words begin a little right of the bullet, and what belongs to the item is set in to there: the further lines of its paragraph, the displayed equations under it, and the paragraphs after those. The item ends at the next bullet, at a heading, at a picture (it and its caption stand at the margin), or at a line of text that begins left of the item's words. A page may turn inside it: the page's bar and markers then stand in the item.

Inside an item, a line that begins where the item's words begin is a line of its text, however much of it is math ("• Additive Identity:" and under it "For a ∈ ℝ, a + 0 = a."): a formula set apart stands in the middle, not at the item's left. Such a line right under one that ended early, at the spacing of a paragraph's lines, is a new line of the same paragraph (`<br>`), as the book has no space between them; with a little more room above it, it is a paragraph of its own; under a line that runs to the right margin, it is that line's sentence going on. The spacing is measured from the words of the bullet's line: the bullet's own box begins higher.

It is written as a Markdown list item: "- " before its first line and every other line of it two spaces in, the fences of its equations too, so they stay part of the item. The reader's notes and corrected equations are put back before the items are written (every block still at the margin then), and are set in with their item; when the book is read again for what the reader changed, what stands two spaces in is read as if it stood at the margin. mdBun reads a math fence that is set in (it had to stand at the margin before).

### Displayed equations

*Worked out on: Dyson, then Zee 2. Fitted: a different way of setting fractions or limits will show new faults here first.*

A displayed equation is a line that is mostly math and indented, or one with an equation number at the right margin, together with the rows of fractions and limits around it. It is rebuilt from the positions of the glyphs and the drawn bars:

* **Fractions.** A bar with symbols right above and below it is a fraction. A clearly narrower bar inside is a nested fraction; one of the same width is a separate fraction below. Where no bar is found, a group above the main row that overlaps a group below is taken as a fraction. A small stacked fraction keeps both halves also close behind a figure or a sign (−¼, 2^{d/2−1}), and takes nothing from the line above but what stands at its bar: not the end of that line's words, not the index of a letter there. The box of a minus sign hangs lower than a letter's: it is over the bar all the same (−1 over 2).
* **Roots and overlines.** A bar right after a root sign is a square root. A root sign in a denominator reaches up past the fraction bar; its box has its middle at the bar, and it is under the bar all the same (1/√2). Where a display is split into lines, the sign goes to the line under its own bar. A fraction that fills a root has a bar nearly as wide as the root's own: under the root's bar and within its width it is the root's, with its denominator (√(2m(E−V)/ħ²)), and such a root stands on the line at the fraction's bar. A bar just above a single symbol, with no numerator above it, is an overline (ψ̄).
* **Scripts and limits.** Small raised or lowered glyphs are scripts. Groups around Σ or ∫ are limits. A smaller piece at a script's end, a little lower or higher, is that script's own index (M^{λ₁λ₂}, ω_{k′}), not a letter of the line, though it stands level with the line's middle. The limit of a sum may be wider than its sign ("k ≠ n"): what stands under or over a sum sign is no index of the sign before it, nor is the first of several pieces that reach under it ("n = 1, 3, 5 …" begins before the sign). Sum signs side by side share one row of limits ("p⃗" and "r = 1" under Σ Σ, level and close together): the row is cut at the gap nearest the middle between the signs, and each sign takes what stands under and over its own middle. Beside an integral only lower limits are cut so, at a real gap: its upper limit may reach over the next sign. The small circle of degrees is the power of the figure before it, `90^{\circ}`.
* **Numbers.** An equation's number is a number in brackets at the right margin: "(16)", "(3.422b)", "(B.12)", or numbered by section, "(10.2.27)". A reference to it is a link: "(3.422)" anywhere, and after "Eq." the bare number where the PDF makes it a link ("Eq. 9.4.18"); not a number after "Section", "Figure", "Table", "Example", which go to those (a worked example under its own title "Example 9.4.3" is linked too).
* **An equation in a frame.** A drawn frame alone around a displayed equation (its lines mostly symbols: a few words such as "if" or the names of units are of the equation) is that equation in a box, `\boxed{…}`, with its number; the comma or full stop that closes it may stand just outside the frame. A framed sentence stays a one-cell table.
* **The long arrow** set as a minus sign run into an arrow is `\longrightarrow`.
* **Several lines.** A system of equations with one number, or an equation continued on a line that starts with + or −, becomes an `aligned` block. So does a line that begins with "=" under a line without a relation sign. The last row and the fraction bars of a bracketed column stay with the bracket's line, and so do the exponents of its first row that stand out over the bracket's head.
* **The size of the line** is the size of the equation's largest letters. The comma or full stop that closes an equation set smaller than the text is in the text's larger type; it takes the size of the letters, or every letter would count as an index.
* **Accents** drawn as separate glyphs (˜ ˆ ˙ ¨ ¯) become `\tilde`, `\hat`, `\dot`, `\ddot`, `\bar`, over the letter they are drawn over: where the letter is in its piece of text is told by the usual widths of letters. A labelled underbrace becomes `\underbrace{…}_{\text{label}}`.

### Exponents

*Worked out on: Zee 2. One kind of PDF in part: the full-size integral sign and the full stop as a product dot inside an exponent are that book's typesetting.*

* **A small stacked fraction** set above or below the line (a power of ½) is an exponent or an index. One on the line is a symbol of the line.
* **An integral or sum inside an exponent** may be set with its sign as large as on the line, between the small letters before and after it: `e^{i \int d^{4} x …}`. A big operator that follows an exponent and is followed by more of it at the same height is inside the exponent. So is the dot of a product set as a raised full stop: `e^{- i k \cdot x}`. With small letters on both sides of it, it is a small dot among them, also after an index set higher: `e^{J \cdot K^{-1} \cdot J}`. Two signs in a row (∫∫ d⁴x d⁴y …) go in together. A sum with its index set beside it, lowered, and the summed letters behind that goes in whole; an exponent of e that is all one integral, the sign right behind the e, is `e^{\int d^{d} x …}`.
* **Dots that close the indices** of a symbol, raised over it in the size of the line, are in its exponent: `T^{\lambda \dots}`.
* **Brackets of an exponent set as large as the line** (`e^{i(k·r − ωt)}` written with `\left( … \right)` up there): an opening bracket between two small pieces and level with them, and the closing one at its height, are of the exponent. The closing one may come in one piece with the sign after it (") ="); it is cut off first.
* **A root that opens an exponent** (`e^{\sqrt{-2mE} x/\hbar}`): a root of small letters with small letters going on right behind its bar is a small piece at their height, as one that follows small letters is.
* **What stands under "lim"** is its index, all of it: `\lim_{N \to \infty}`.
* **A stroke through a letter** (Feynman's slash, p̸ and ∂̸) is written as the letter with a "/" drawn back over it, `{p\!\!\!/}`: plain TeX, the same in every renderer. KaTeX sets `\not{p}` as a stroke beside the letter with a gap after it (Zee has about a thousand).

### Brackets and matrices

*Worked out on: Dyson. One kind of PDF: reading brackets from outlines, and the "h" and "i" for ⟨ ⟩, are for TeX's fonts only. Pairing brackets and building matrices is general and is used for Zee 2 as well.*

`pdftohtml` gives tall brackets no character, or drops them, so they are read from their outlines in the SVG. Straight strokes with the long stroke on one side are `[` or `]`. A curve bulging at mid-height is `(` or `)`. A thin bar is `|`. Two strokes meeting in a point are `⟨` or `⟩`. A curve whose outer edge is a sharp point at mid-height is `{` or `}`. The pieces of an extensible bracket are joined first. An outline that lies inside a word of the text and is no taller than it is a stroke of a letter, not a bracket: a glyph the PDF uses once may come into the SVG as a bare outline (the "j" of a bold heading looked like "⟩").

Brackets are then paired as in text, nested ones included, and written as `\left[ … \right]`, `bmatrix`, `vmatrix` and so on. A bracket read from its outline has its true height: what stands under its foot, in the row below, is not in it (the exponent of the next row of a `cases` under a bracketed fraction).

* **A system of equations between braces**, one equation a row, is lined up at its "=" as the book sets it: `\left\{ \begin{aligned} x &= a \cosh \mu \cos \nu \\ y &= a \sinh \mu \sin \nu \end{aligned} \right\}`, the left sides ending at the sign and the right sides beginning there. (As a `Bmatrix` its columns were centered, a short right side standing in the middle under a long one, and rows that are one column each were not lined up at all; Lucid has 105 such systems.) It is a system if every row has one "=" outside brackets, or `→`, `≈` and the like in a column of its own; two equations side by side in a row, or a word between them ("and"), stay a matrix. A last row of words ("etc.") stands under the right sides. The "=" inside a matrix is never what the lines of a display are lined up at.
* A lone `{` followed by two or more rows becomes `cases`.
* A lone `}` after several rows groups them: `\left. \begin{aligned} … \end{aligned} \right\}`.
* A tall bar with a small index at its foot is the bar of "taken at": it closes and opens nothing (`\frac{d\psi}{dx} |_{+\epsilon} - \frac{d\psi}{dx} |_{-\epsilon}`), unless a sign of the formula stands right behind it.
* A bracket that opens on one line of a formula and closes on a later one is written with `\left\{ … \right.` and `\left. … \right\}`.
* Inside a matrix, small entries that stand free and form their own rows and columns are an inner `matrix`, as in Dyson's equation (15). The index of an index, and what goes on from a script at its height (the "iE t/ħ" of e^{−iEt/ħ}), do not stand free.
* **The rows of a column come before its fractions.** The entries of a column are set as small as a fraction in a line of text, and so close that by distances alone the "0" of the row above reads as a piece of the numerator under it, and two fractions one over the other as one inside the other (UCSD's Dirac spinors). So between a pair of tall brackets the rows are found first: a row is told by a fraction bar or by a line of full-size letters, at least a line apart, and each bar then takes its numerator and denominator from its own row only. The line between two rows is drawn close to the full-size letters, just clear of their indices, for a fraction's small rows reach far from its bar. Rows less than a line apart (a full-size numerator on its bar, a fraction inside a fraction) leave the bracket to the ordinary rules. A root's bar, and its sign, are of the row of what stands under the bar.
* **Rows with brackets of their own.** Where each row of a column or of a braced system has its own brackets, one row right under the other, a closing bracket closes the open one at its own height, and what a pair holds ends where the next row's brackets begin. Without this the rows' brackets pair across the rows and come out as one bracket inside another around a `pmatrix` (Lucid's systems of equations, the radial equations in UCSD).

TeX's symbol font has a quirk: in the PDF's text, ⟨ ⟩ come out as "h" and "i", | and ‖ as "j" and "k", { } as "f" and "g", and capitals as calligraphic letters. These are mapped back.

### Formulas set small, and systems of them

*Worked out on: Lucid, whose long derivations are set in small type and whose systems of equations stand in braces. The rules are general; they also mended formulas in UCSD, Dyson and Zee 2. The heights of root signs are those of TeX's fonts.*

"The size of the line" decides what is a letter of the line and what is an index, so a formula in two sizes, or with two rows close together, needs more care.

* **Displays in two sizes.** A result in the size of the text and its steps in small type, one under the other with nothing between them, are cut apart where the size changes. Small type is about five sixths of the text's size and an index two thirds, and the small lines have a relation sign of their own. Left as one display, every letter of the small lines reads as an index of the large one.
* **Small type going on a line.** "L₃ =" in the size of the text and a long right-hand side set small to fit a table's row: where the first small letter stands level with the full-size one before it, the small letters are letters of the line. An index hangs under the line or stands over it, and what stands under "lim" is its index.
* **Two lines of fractions with no relation sign in the first.** Two rows, each at the height of fraction bars of its own, well apart and with no bar between them, are two lines. A bar between them would make them the numerator and denominator of one fraction; so would a row that holds only the full stop of the text above.
* **A fraction takes only the bars among its own pieces.** In a system of equations the fraction of the next row has a bar a little shorter or longer than this one's, right under it. It is that row's own. Two short bars of nearly one length are siblings as well, not one fraction inside the other.
* **A root reaches no further down than its sign.** TeX has the root sign in five heights, each its own character, and what stands under the bar is no taller than the sign. Without this a root in one row of a system takes the bracket that stands under its bar in the next row.
* **Brackets grown around a small fraction.** In a numerator or denominator set small, `\left( … \right)` around a still smaller fraction grows to the size of the line. Such a pair (off the line's middle, only small pieces between them, a small piece level with it right beside it) takes the size of that row. Left full size, it would be a letter of the line and the numerator would fall apart around it.
* **A drawn bracket no taller than the type** is read from its outline only where the text has the empty piece `pdftohtml` gives for it, or where its partner of the same height is read so. That tells `\big[` in small type from an italic letter as tall.
* **A drawn bracket holds nothing well above its head.** The indices of the row above are a little over it and are not in it.
* **Each sum keeps its limits.** The lower limit of one row's sum and the upper limit of the next row's stand one over the other. They are no block of a matrix and no row of a bracket; and a limit is no part of the fraction over or under it.
* **A sign has no index.** The small "c" of −1/c², close behind the minus, is the denominator's.
* **A bracket beside a numerator** leaves its bar a fraction bar. A bar over a single piece of text with nothing above it is an overline (Ā); the short box of a large bracket that begins at the numerator's side is not "something sticking out past the bar".
* **A page of a few formulas is no page of two columns.** Two columns need lines of words on both sides of the gap; a white gap that runs through two displayed formulas is chance.

### More from Lucid: bars of "taken at", tables, sideways pages, portraits

*Worked out on: Lucid. General, except the heights of TeX's root signs; several mended formulas in UCSD, Zee and Dyson too.*

* **A bar of "taken at"** (a tall bar with an index at its foot, `dψ/dx |_{x = a/2}`, or with limits at foot and head, `… |_a^b`) is a piece of its row, `\Big|` or `\Bigg|` by its height, with what hangs at its foot and stands at its head as index and exponent. It closes a bracket only where that bracket is as tall as the bar, head and foot: `\left\{ … \right|_{0}^{a/2}`. The brace of a system of equations stays open past a bar in one of its rows.
* **Roots.** A root sign set a little over a fraction's bar, with its own bar right under that bar, is in the denominator (x/√(x²+y²+z²) set small). A root over a fraction that fills it stays on the line though its letters are small. A root sign is of the line its foot stands in, or that begins a point under its foot.
* **The lower limit of a sum set apart** stands under its tall sign, clear of every line; it is of the sign's line, not of the line of text that follows.
* **A line in the size of an index.** A line begun in the size of the text and gone on to its end in index-size type ("H = H_fs +" and two long terms set small to fit) is cut behind the last full-size letter and its index; the small part is a formula of its own size.
* **Table cells** get the bars drawn in them, so a small fraction or a root in a cell is one. A small fraction takes no larger piece that lies beyond its own half (the minus sign that opens the next row, under the 2 of a ½). A root sign and what it is the root of are one cell.
* **A light band behind a table's head** or an equation is no piece of a drawing: with the table's rules and the bars of its roots it had made a "picture" of half a table.
* **Signs and letters in the text's font beside a formula.** An upright letter or two set right against a raised or lowered piece is its base (e⁻, K⁺). A line set in with a large sum or integral sign and a short label beside it is a formula set apart.
* **Pages set sideways** (a wide figure turned to fill the page, its caption running down the page) are found by their text, which `pdftohtml` gives without width. They are turned upright in a copy of the PDF kept in the cache, and the book is read from that copy: the figure, its caption and the formulas in it come out as on any page. The link to the PDF still goes to the book's own file. A curved stroke out in the margin, beyond every line of text, is no bracket.
* **A row of pictures with a name under each** (portraits) is written as a row: each picture and its name one cell, and the caption under the row is the caption of each picture.
* **On the title page**, a line in large type that stands well apart under the title and subtitle (the author's name, the date) is a line of the page in bold, not a heading of the book.

### From the hand corrections of UCSD: lines of a display, limits, roots, matrices

*Worked out on: the 66 displayed equations of UCSD that had been corrected by hand; 35 of them are now converted right. Each rule was kept only where Dyson, Zee, Lucid, Sakurai and the article lost nothing by it. The heights of TeX's big signs are fitted.*

* **A small piece between two lines of a display goes with what it hangs on,** not with the line whose middle is nearer. An exponent or index is of the letter right before it (the square of a numerator's letter stands halfway up to the line above); what goes on at one height from a small piece is of that piece's line; the two halves of a small fraction (½) go with their short bar; an exponent at the head or an index at the foot of a tall closing bracket goes with the bracket. What stands between a pair of tall brackets stays with them.
* **Limits go with their sign.** A big operator's box is only the font's height around its baseline; the sign hangs from there, 1.0 to 2.2 em deep by its character. A small piece right over its top or right under its foot is its limit and of its line. Where lines stand so close that the lower limit of one sign is at the place of an upper limit of the next line's, the nearer line decides as before. Only TeX's own extension font is measured so.
* **A fraction's half that holds an integral or a sum** reaches as far as the sign and its limits; and the row of such a denominator, lower than a plain one, is no line of its own.
* **A root's bar is of the line of its sign,** at the middle of the sign's height. Over a fraction the bar stands nearer the line above; given to that line, the root was left without its bar and took the rest of its own line.
* **Pieces at the head and foot of a tall square bracket or brace** ("taken from 0 to ∞"; the "(+)" and "(−)" that mark a brace's two rows) hang on it as exponent and index. One over the other behind it they read as a fraction.
* **Matrix columns.** The space around a sign in an entry ("σ·L + ħ") is wider than letters stand apart and narrower than columns: where it cut an entry in two, a column that most rows lack, the columns are told again with more room. Dots side by side in a matrix's last row are entries each.
* **An index by the letters' height.** A small letter close behind a full-size one is its index when it stands a tenth of the letters' height off the row's middle; the taller boxes of signs in the row (a minus) do not count.
* **Round brackets of full size around letters that are all small** take the letters' size (the denominator of a small fraction under a root, with a ½ inside the brackets): left large they alone would be the line, and every letter an index.
* **A root sign with one small piece over another behind it** is the root of a small fraction, `\sqrt{\tfrac{2}{3}}`; **an index with a bar over it under an exponent** is a small fraction too (sinh χ/2).
* **The small minus of a small fraction's numerator** is over the bar by its own size.
* **In the text:** the upper limit standing on the head of a sum set in a line of words is of the sum's line, not an index of the line above; small pieces left as a line of their own right over a line, behind the head of a tall bracket of that line, are the bracket's exponent.

What these rules do not mend, and stays corrected by hand in the user's book (see [What is kept between conversions](#what-is-kept-between-conversions)): matrices set inside a sentence, tables and lists of states set as formulas, rows set beside a bracket's rows, a product of matrices broken over two lines, fractions whose halves each begin with a sign in an aligned block, equations that run off the page.

### Symbols the text leaves out

*Worked out on: Zee 2. One kind of PDF: MathTime fonts. The method (names from the font, places from the SVG) is general; the table of names is not.*

A font names the glyph each character draws ("summationdisplay", "angbracketleft") and usually also gives its Unicode character. Where a font gives no characters, or not for all its glyphs, `pdftohtml` gives an empty piece of text. Some publishers' math fonts do this for the big symbols (∫, Σ, ∏, large brackets and roots), the angle brackets, the prime, the arrow over a vector and the stroke of the Feynman slash.

pdf2md reads the names from the PDF's fonts. The PDF lists the glyphs of a page in the order they are drawn, and so does the SVG, which also says where each one is. Each lost glyph is written into the empty piece at its place. A page where the two lists differ in length is left alone. The fraction bars of such a book may be drawn as thin lines, not as filled rectangles; both are read.

An arrow or a stroke belongs to the letter it is drawn over: `\vec{k}`, `\not{p}`.

The names cover the big symbols, brackets and roots of a big-symbol font, the angle brackets, the prime, Greek letters as fonts name them ("Lambda1", "epsilon1") and ℏ. A figure or letter of the text font under an arrow is taken into the formula with it.

| Variable | Shows |
|---|---|
| `PDF2MD_GLYPHDEBUG=1` | glyphs without a known name, glyphs that found no place, and the count written |

### Notation lists and pictures

*Worked out on: Dyson. Fitted.*

* **The dot of a product from a full stop.** Zee's font gives the raised dot of p · q under the full stop's code. A full stop that is a piece of its own, with room on both sides, between two operands of its line (an arrow or tilde over a letter is no neighbour) is written `\cdot`. Not in TeX's own fonts, whose product dot is a sign of the symbol font: there such a full stop ends a sentence between two formulas. (Zee: 187 centred dots before, 404 now.)
* **Script capitals.** The script font (rsfs) stores ℒ, 𝒟, 𝒥 as plain letters: a letter of that font is written `\mathscr{…}`. (Dyson's Lagrangian and 226 operators in Sakurai's text, 𝒟(R), 𝒥(dx′), 𝒰(t, t₀), had come out as plain capitals.)
* **Feynman's slash.** A slash that lies over a letter, sharing most of its width with it, is that letter's stroke (p̸); a slash that divides has room of its own. Dyson makes the stroke of the plain "/": its slashed letters had come out as "p /" or "/ k". Where the slash comes in one piece with its letter ("/e", as wide as the e alone, because the slash moves the type on by nothing) it is told by the piece's width, in TeX's own math italic only: in Zee's font a piece with a dividing slash is as narrow. Dyson has 725 slashed letters.
* **The stroke of ≠.** TeX prints ≠ as a slash of no width struck over "=". The page reader gives the slash as the figure 6 of the symbol font, or, from PDFs made another way, as a combining stroke; pieces of no width are otherwise dropped (they are stamps in the margin), so the stroke is kept by name and given the width of the sign it strikes. (Before the second name was known, Lucid had 25 and the article 8 places with "=" for "≠".)
* **The two phis.** TeX's math italic has the phi with a straight stroke (`\phi`) and the curly one (`\varphi`). A PDF gives the straight one under either of two character codes, depending on the program that made it, so the code does not tell which glyph it is. A book writes `\phi` far more often than `\varphi`, if it uses both at all: in TeX's math italic the more frequent of the two characters is written `\phi` and the other `\varphi` (UCSD and Dyson had every `\phi` written `\varphi`).
* **Aligned columns.** Three or more lines whose columns start at the same positions, with words in the last column, become a LaTeX `array`. Not where the first column counts on by one ("8.4", "8.5", "8.6"): those are numbered sentences that happen to be a line each, such as short problems.
* **Pictures** are found in the SVG: stroked or filled paths that are neither glyphs nor thin bars, and embedded images. Pieces close together are one picture. A group counts as a picture if it holds an image, or 3 or more pieces at least 15 pt square, or 8 or more pieces at least 40 pt wide, or one long curved stroke. Text inside the picture, and short text just beside it, is a label and becomes part of the image; so is a line under it in the fonts the book's drawings are lettered in ("(a) (b)" under the panels, the factor under a vertex), unless it stands in or nearer over the next picture. The picture is then rendered from the PDF with `pdftocairo -png`.
* **An image** in such a book is a picture, with a caption or without. It is never read as a formula: the book's formulas are set in its math fonts.

### Articles

*Worked out on: one paper (Barandes, "New Prospects for a Causally Local Formulation of Quantum Theory", arXiv, two columns, RevTeX). Fitted: every number here was set on that one paper.*

* **Two columns** are found from the text itself: about the middle of the page there is a place that hardly any piece of text crosses, with at least 40 long pieces on each side of it. Lines are then put together for each column on its own, so that a line never takes pieces of the other column at its height.
* **Unfolding.** Each page is cut into bands: what runs across both columns (title, authors, abstract, a wide picture) and stretches of two columns. In a stretch of columns the right column is put under the left one, at the left column's place, and rules, pictures and the places links lead to move with the lines. Everything after that reads one column on a taller page; where each stretch was on the printed page is kept, so that pictures are still cut from the right place. A paragraph runs on from the foot of the left column to the head of the right one as it does over a page turn.
* **Footnotes of the left column** would land in the middle of the unfolded page. Its last lines in smaller type, under the short rule drawn over footnotes or beginning with a note's mark, are moved under the right column, to the foot of the page. A note that goes on at the foot of the next page (under that page's rule, without a mark) is joined to its beginning. A page often holds the end of one section and the beginning of the next: a note goes to the section its mark is in.
* **What runs across both columns** is judged against the page's margins, not the column's: a line that runs out over the column's right margin is centered if its middle is the page's middle. So the title, the authors and their places stay centered lines. The raised marks behind an author's name ("1, 2," and a star) are marks of notes, not powers.
* **An article** is a document without chapters that has at least two section titles in bold with a number before them, Roman or Arabic ("IV. THE UNISTOCHASTIC FORMULATION", "2. Method"). Its sections are then what chapters are in a book: each a file, named by its number without the numeral. A title in bold capitals without a number ("ACKNOWLEDGMENTS") is a section too, and counts on after the last number. Where the list of references has no title, one is given to it ("References") at the entry "[1]" in the second half of the document. `book.md` is headed by the title printed on the first page.
* **Citations** "[9]", "[1, 2]", "[10–16]" become links, number by number, to the entries of the reference list, which keeps its place as a file (in a book, each chapter gets the references it cites at its end instead). "Section IV" links to that section's file; "(4)" where the PDF itself links it, to that equation.
* **Quotations set apart** — a paragraph of two or more lines, every line in from both margins by 0.8 to 6 type sizes, not centered and no item of a list — are written as block quotations. Only in an article: in the books the same test also takes the set-in parts of problems.
* **A statement in words with an equation's number** (lines of text closed by a tall brace at the right, then "(1)") is written as a formula of text lines closed by the brace, so that it keeps its number and the links to it.
* **A caption** "Figure 2. A spacetime diagram …" (a whole number and a full stop, then a capital) is the picture's caption, as "Figure 4.1: …" is.

Three rules that came out of this work apply to every PDF with text:

* **A word cut at a line's end keeps its hyphen** when the book writes it with one inside a line ("two-dimensional"), as before — and now also when both halves are words the book uses on their own and it never writes the two as one word ("four-dimensional", "time-independent"; but "space-time" becomes "spacetime" where the book writes it so, and "in-tegrate" has a half that is no word). This mended 35 words across the five books and changed no other.
* **One letter alone in a bold font** is a symbol of a formula, with whatever index it bears (a vector **r**₁ had come out as a bold word and a stray index). Not in a line that is bold altogether, a title. A letter of TeX's bold roman font is written upright (`\mathbf`), one of a bold math font slanted (`\boldsymbol`).
* **The rest of a label at a picture's edge** ("(" just inside the picture, "R, B)" right behind it) goes into the picture with its first piece: up to six characters, each piece right behind the one before.

### A second reading of the displayed equations

*General in its way of working; the numbers are UCSD's. Needs the page reader's recognizer (the environment of scanned books); without it this step is left out.*

On UCSD's 3,966 displayed equations: 3,642 are read the same by both, 62 are written as the recognizer read them, 214 are listed to check, 48 it did not read as a formula. Of the 62, 40 were compared with the page by eye and all 40 are right. Of the 23 equations that had been corrected by hand, 5 are among the 62 and 12 are on the list. The first conversion takes about a quarter of an hour longer (RTX 5080), later ones no longer.

The rules above know every letter of a formula, because the PDF names them, but they have to work out from the letters' places how the formula is put together, and that is where they go wrong: an exponent left on the line, the halves of a small fraction taken apart, the rows of a matrix mixed. A recognizer that reads the formula as a picture has the opposite strengths. So the place of each displayed equation is cut out of its page (216 dots to the inch) and read by the recognizer, and the two readings are compared:

* **The same formula** (ways of writing aside): nothing is done.
* **The same letters and signs, each as often, put together otherwise**, and the recognizer reads them so a second time from a picture half as large again: its reading is written. The count of letters and signs is the check on the recognizer, which now and then invents a bar or a minus sign; brackets and bars count, what only orders the signs (`^`, `_`, `\frac`, the rows of an array) does not. Two more things must hold, because the recognizer reads two rows of fractions between brackets as one fraction of fractions: its reading has the same matrices, cases and arrays as the rules' (rows and cells counted), and no more fractions whose two halves are fractions themselves.
* **Anything else** (other letters or signs, or two readings that differ): the equation stays as the rules built it and is listed in `check-equations.md` with the recognizer's reading under it.
* **No formula read** (the recognizer saw words, or wrote nothing): the equation stays.

An equation's number is not part of the place cut out; it stays as the rules found it. The readings are kept in `~/.cache/pdf2md/second/` by PDF, page and place, so each place is read once; the verdict is made again when the rules build other LaTeX for it. A batch is given no more room than the rules' LaTeX says its longest formula can need, because the recognizer sometimes writes the columns of an array on and on. `PDF2MD_SECOND=0` turns the step off.

A correction made by hand is of what the rules built (the `pdf2md:fix` mark holds that LaTeX). Where such a mark exists the rules' LaTeX is written, not the recognizer's, so the correction finds its equation.

## PDFs made from ebooks

A PDF made from an ebook, with calibre or from Kindle, holds its prose as text but its math in two other ways: larger formulas as small pictures, and simple math inside sentences in the ordinary text font. pdf2md takes this path when the book has no Computer Modern fonts.

### Equation pictures

*Worked out on: Sakurai. One kind of PDF: calibre's equation pictures at 96 dpi, read by texify 0.2.1. Most bullets answer a habit of that model, and another model would need them rewritten.*

Each equation picture goes to [texify](https://github.com/VikParuchuri/texify), a math-recognition model that runs locally. The LaTeX it reads is put inline where the picture sits in a sentence, and as a displayed equation where it stands alone. Readings are cached by picture in `~/.cache/pdf2md/ocr/`.

* **Not every picture is math.** A picture stays a picture if it has a "Fig." caption, or if it is a diagram: long straight lines meeting at a corner, or one connected shape that is large both ways. Lettering such as a publisher's logo also stays a picture. A horizontal line becomes `---`. A picture of program code is read as text by [Tesseract](https://github.com/tesseract-ocr/tesseract) and written as a code block. Tesseract takes Mathematica's named characters apart (`\[Psi]` came out as `\[P=i]`), so where the page reader's recognizer is installed it reads the code once more, and its reading is the one written if it is program text too and about as long; each line is set in as far as it is in the picture, measured in letter widths. It still mixes up the letter l and the figure 1 where both are possible. A code block has `<!-- pdf2md:code … -->` above it, and a correction of its text is kept by picture like an equation's. A reading that names a picture file (`\includegraphics`) is of a drawing too faint for these tests (thin grey lines): the picture is shown.
* **Very small pictures**, under 30 pixels wide, are single symbols such as a hatted **x**. They are read with white space added around them, because a symbol that fills its picture is read as some other letter. Where such a symbol sits inside a sentence that the PDF holds as one piece of text, the text is split at the gap it sits in.
* **Several lines.** A tall equation the model cannot read whole is read line by line. A picture of several lines that the model wrote as one long line is cut into its lines: where the model left a space before the "=" or operator a line starts with (it writes no other needless spaces), or, read line by line, if that reading agrees with the whole one or shows where the first line ends. Two lines with no "=" between them (a long formula wrapped inside a bracket or inside its words, two sentences of words and symbols) are cut where the letters and figures of the whole reading part as those of the two lines do. Lines that each start with "=" after a first line the whole reading starts with are taken as read one by one, whatever the whole reading made of the rest. A short line in small type that the whole reading left out is added, and lines of words that the whole reading gave as plain text are taken as read one by one. A bracket that opens on one line and closes on another gets a fixed large size. The rows become an `aligned` block, lined up at the "=". A multi-line equation that the model wrote as an `array` with uneven columns is turned into rows the same way.
* **Bold letters.** The model is unsure which letters are bold. The strokes of the picture tell whether it has any bold letter at all: ordinary letters of an ebook's equation pictures have strokes of 1.1 to 1.3 pixels, bold ones of 1.6 to 1.8. A reading that sets letters bold where no stroke of the picture is that thick loses its bold; a picture with thick strokes whose reading has no bold letter is listed in `check-equations.md`. Which letters are bold, the strokes do not tell reliably (a sum sign and a large letter are thick, too), so that is left to the two readings: where they set different letters bold, the list says **bold?**.
* **Words under an equation.** Some pictures label their parts with small words underneath, such as "bra" and "ket". The model drops such words or reads them as symbols. pdf2md reads the picture again without that band and puts the words back in a second row, each centred where it stood. The places are estimates, so a word may stand a little off. Sakurai has three such equations.
* **A second reading.** Each displayed equation is read again from the picture enlarged. Where the two readings are different formulas, formatting aside, the equation is listed in `check-equations.md`, biggest differences first. When nothing is left to check, the list of an earlier conversion is removed.
* **A second recognizer.** Where the page reader of scanned books is installed, its recognizer (surya) reads every equation picture too (`math_check.py`; once for each picture, kept as `KEY.surya`). The two err in different places: on Sakurai, texify takes l for I, a nabla for a bold V and loses primes, and surya was right in every such difference of two samples checked against the pictures; on bold letters each misses some. So where the two read the same formula, bold and ways of writing aside, the reading with more bold letters is written — but first one whose matrices are whole: texify now and then writes an empty row into a matrix or a row with a cell too many (Sakurai's (3.381) came out two by three with two empty places), and where its own second reading or surya's is the same formula with the matrix whole and no bold letter fewer, that one is written (nine equations in Sakurai); otherwise surya's (`KEY.best`), and the equation is listed to check (`KEY.agree`) unless texify's second reading says the same, or surya reads the same formula again from the picture enlarged three times instead of two (`KEY.surya3`; in a sample of nine such equations checked against the pictures all nine were right). Where surya's two readings differ and one of them is what texify read, that is the formula. What surya leaves open in a reading is closed where there is one way to: an "=" it sets between two formulas leads the next one; rows begun with `\begin{split}` and not ended are ended; an environment ended by another one's name takes its own; a bar "taken at" (`\right|` with no `\left`) gets `\left.` at the head of its group; a formula's tag may run over several lines. "The same formula" also takes a label over a sign (`\stackrel`, `\overset`, `\underset`, `\xrightarrow[…]{…}`) for its exponent or index, `\not\in` for `\notin`, a long arrow for a short one, `x^{\prime 2}` for `x'^2`, rows in an array for rows without one, and words in sans-serif or quotes for plain ones. One thing surya misses each time it reads is the prime of a sum sign (a sum that leaves terms out): where texify read it twice, on the same sum of as many, surya's reading gets it too. On Sakurai 560 of 3,445 readings are surya's and the list of displayed equations to check went from 522 to 32 (among them now the 269 unnumbered ones, which were not looked at before); 28 of the choices these rules changed were checked against the pictures and all were right (one more, a long sum where both recognizers err, came onto the list). The list of equations whose picture has bold strokes and whose reading has no bold letter looks at the reading written, not at texify's. A third reading by texify itself was tried first and dropped: where it sided with the second reading, that one was wrong nearly as often as right.

**What is kept of the readings.** Everything read from a picture is kept in `~/.cache/pdf2md/ocr/` under the picture's key: the reading (`KEY.tex`), the second reading and how far the two differ (`KEY.second`, `KEY.verify`), the cells of a table (`KEY.cells`), and marks that say which checks the picture has had (`KEY.kind`, `KEY.lines`, `KEY.labels`, `KEY.small`, `KEY.bold`). `KEY.lines` and the first line of `KEY.cells` name the version of the rules that looked at the picture, so a reading kept from an earlier version is looked at again once when the rules have changed. That is why the first conversion after an update takes minutes, not seconds.

### Math in the text font

*Worked out on: Sakurai. Fitted: the lists of what is surely math and of common italic words are English and were filled from this book.*

Inside a sentence, an italic letter is a variable and the brackets and "=" around it are ordinary characters. The PDF gives each line as pieces by typeface, with prose and symbols mixed in one piece. pdf2md cuts each piece into words, symbols, numbers, brackets and punctuation. A stretch with no word in it is one formula if something in it is surely math.

* **Surely math:** an italic letter with its primes; a Greek letter; a relation or operator that prose never uses, such as `=`, `≈`, `→`, `×`, `⟨`, `|`, `∂`, `ħ`; a subscript or superscript; a single bold letter (a vector); an equation picture.
* **Math only beside math:** numbers, round and square brackets, `+`, `−`, `/`, a lone upright letter as in `Z = 2`, and function names such as `sin`, `exp`, `Tr`.
* **Two or three italic letters** are symbols (`kR`, `dt`) unless they are a common word (`is`, `or`, `not`). Three need math right beside them. A longer italic word is emphasis.
* **Where a formula ends:** at a word, and at a comma, semicolon or full stop that is followed by a space and is not inside brackets. Brackets around a whole formula are prose. So are the hyphen in "x-polarized" and the apostrophe in "Green's".
* **Raised numbers.** After a word, a small raised number is a footnote marker. After a symbol, a number or a bracketed formula it is a power.
* **Linked numbers**, such as an equation reference or an index page, are never part of the formula beside them.
* **Signs and accents from odd places of the fonts.** An ebook may set "<" as the arrowhead `˂`, the integral sign as the upper half of a tall one, Ω as the ohm sign: each is read as the sign it stands for. An accent that comes as a character of its own behind its letter (`C˜`), at the head of the next piece (a bold **A**, then the tilde), or in one character with its letter (α with a tilde, z with a hat, standing alone) goes over that letter. A star set on the line between a letter and its bracket joins the formula where it stands; it is not raised.

### Tables

*Worked out on: Sakurai. One kind of PDF: the grey panel with a rule under each head cell is that publisher's table design. Reading a table picture cell by cell is fitted.*

Such a book sets a table on a grey panel, with a short rule under each cell of the head. The rules give the columns. The rows have no marks, so they are found from how the cells stand: centred in their row or at its top, whichever fits the table. A table that reaches the foot of its page continues at the top of the next: the two parts are written as one table, with the pictures of both under it. A table whose head starts at the foot of a page, with the rest on the next, gets those lines into its head. The caption, set centered on a panel on top of the table, is the table's title: one cell across all its columns. A note set on a panel of its own under a table is the table's foot, a cell across all its columns too, and part of its picture.

A table that the book holds as a picture is read cell by cell. The picture is cut from the page again at higher resolution. Its rows are the lines of text on it, and its columns end where no row has ink. Each cell is then read on its own, as words, as a whole number or as a formula. A picture that does not split into rows and columns this way keeps the model's reading of it as one array. Each cell is set as it is in the picture: at the left of its column, in its middle or at its right, by where its ink stands between the column's edges (a cell that fills the column is set as most of the others are). Where the book sets the caption centered on a panel on top of such a table, the caption is the table's title: one cell across all its columns, above the rows. Such a table needs no caption: every large picture that was read as a formula is looked at once, and is a table if two or more rules run unbroken across it and it parts into rows and columns. (A table picture with neither a caption nor rules cannot be told from a formula of several lines.)

## eBooks

*Worked out on EPUBs made here (pandoc, from chapters of the UCSD notes, with and without the LaTeX kept beside the MathML) and on calibre's guide. No bought eBook has been through it yet: how publishers' files differ will show there.*

An EPUB is a zip file of XHTML pages, a list of their reading order, and a table of contents. `unzip` unpacks it; the pages are read as XML (HTML's named characters are written by their numbers first, so an EPUB 2 page with `&nbsp;` is taken).

* **Chapters.** Each entry of the first depth of the contents (the navigation page of EPUB 3, the NCX file of EPUB 2) begins a file, from its place to the next entry's. A chapter that begins inside a page is cut there, at the element the entry names. Pages before the first entry are the front matter. Without contents, every page is a file. The highest heading of a chapter becomes its title's level; a chapter without a heading gets the words of the contents.
* **Text.** Headings, paragraphs, lists in lists, quotations, code, tables and line breaks are written as the Markdown for them; a table of one cell is its words. Italic and bold marks go inside the spaces at their ends.
* **Links.** A link to a place in another page becomes a link to the chapter file that place is in. Only ids that something links to are written as anchors; an id that two pages use gets its page's name in front. A footnote's mark is raised.
* **Pictures** are copied into `images/` under their own names.
* **Math.** MathML is rewritten as LaTeX element by element: fractions, roots, indices and powers, limits under and over sums, accents (a bar, hat, tilde, arrow or dot over a letter), fenced groups, tables (a `matrix`, in its brackets a `pmatrix` or `bmatrix`; rows aligned right-left are `aligned`), labelled rows with their numbers. Where the eBook keeps the LaTeX the formula was made from (an `annotation` beside the MathML, the description of a formula's picture, or text between `\(` and `\)` for a script to draw), that LaTeX is used as it is. A formula that is only a picture stays a picture.
* **Printed pages.** An eBook that marks where the pages of the printed book begin gets <sup>[p. N]</sup> at each; there is no PDF page to open.
* **What is kept**: notes and prose fixes, as for a course's book. The book is rebuilt in place, and files it no longer writes are taken away.

## Scanned books

*All of this section was worked out on one scan (Zee 1), which has since been lost; nothing here has been run on a second scan.*

A scan is a PDF whose pages are pictures. pdf2md recognizes one by itself: next to no text, and an image on nearly every page. It has the pages read by [marker](https://github.com/datalab-to/marker), a page-reading model that runs locally. marker gives the prose, the formulas as LaTeX, the headings, the footnotes and the places of the pictures. pdf2md builds chapters, page markers, links and files from that as for any other book.

A page takes about 3 seconds on a recent graphics card. Pages read are cached in `~/.cache/pdf2md/scan/`, so a later conversion takes seconds and one that was stopped goes on where it was. pdf2md uses marker 1.10.2, the last version that runs inside Python alone. Tables of contents also need `pdftotext` or, for a scan without hidden text, Tesseract.

### Structure

*Worked out on: Zee 1. One kind of PDF: what marker 1.10.2 gives for a scan. Not tested since that scan was lost.*

* **Chapters** are told from the layout, because the height of a heading's box says too little about its type size. A page that opens with a heading and has no running head opens a chapter. "Chapter I.5" and the title under it are one title. Everything before the first "Chapter" or "Part" is front matter. A book without running heads goes by the heights of its headings, which is less sure.
* **Sections** are one level (`##`) unless they are numbered.
* **Headings marker could not read**, which happens with small capitals, get their words from the PDF's hidden text or from Tesseract.
* **Printed page numbers** come from the running heads and feet. A page that shows none, or whose number was misread, gets its number from the pages around it.
* **A part's title page** ("PART IV" and the part's name) becomes the title of the part's file.
* **The cover**, a first sheet that is mostly picture, is one picture of the whole page.

### Text

*Worked out on: Zee 1. One kind of PDF: most bullets mend what marker got wrong on that scan.*

* **Centered pages**, such as a title page or a dedication, stay centered and keep their line ends. On other pages, a block of several lines that stops short of the right margin keeps its line ends; a block as wide as the text is a paragraph.
* **Lines read too short.** Where marker read much less than the line holds, the words come from the PDF's hidden text, if that text contains marker's words and more.
* **Words on a picture** are part of the picture.
* **Code.** What marker calls program code is written as a code block only if it looks like code; marker also calls an index page code.
* **An index in two columns** stays in two columns. marker does not say which lines are indented continuation lines, and sometimes mixes the two columns. So the column and indentation of each line come from the positions of the words in the PDF's hidden text, and the wording from marker where it has the line. Continuation lines are joined to their entry. A scan without hidden text keeps its columns, but each line is an entry. A line known only from the hidden text is mended by marker's words of the same page: a word marker does not have becomes the one word it has with the same short strokes ("fennions" is "fermions", "Feynrnan" is "Feynman"), a piece that is neither word nor page number becomes marker's formula at that place ("0.4 theory" is $`\varphi^4`$ theory), and page numbers run together get their space.
* **Lines given one by one** (an address, or the catalogue entry on a copyright page) are one paragraph with its line ends kept. A line set in from the first starts with em spaces, one for each line height it is set in by.

### Math

*Worked out on: Zee 1. One kind of PDF: answers what marker left as plain text in that book.*

* **Symbols marker left as plain text** are made formulas by their shape: a letter or two followed by an argument in brackets (`J(x)`, `SU(2)`); a single letter other than "a", "A", "I" that is not an initial, a label or a unit; a short relation without spaces (`m=0`).
* **Displayed equations** keep their numbers. Each is read a second time by texify from a picture cut from the page, and the ones the two readers read differently are listed in `check-equations.md`. Without the texify environment there is no list.
* **Unfinished formulas.** A formula marker did not finish, such as a matrix cut off, is replaced by the second reading if that one is whole.

### Footnotes

*Worked out on: Zee 1. Fitted: found 43 of 46 missing markers there.*

marker does not see every small raised number in the text. Where a page has a footnote without its marker, pdf2md looks for the marker on the picture of that page and of the pages before and after: a small piece of ink set high in its line, right after a word (or after the stop that follows a number or a bracket), that most of five readings take for the note's number. It is put into the text after that word if the word, with the words before it, is found there in one place only, and if the markers of the page still rise from one to the next.

### Contents and tables

*Worked out on: Zee 1. One kind of PDF.*

* **The table of contents** is read line by line from the PDF's hidden text or by Tesseract, because marker reads tables cell by cell, which shuffles a contents page. That text is poor, so an entry whose page opens with a title takes the title as marker read it. Contents set in columns are written as a table of number, title and pages.
* **Other tables** are kept as marker read them if their rows and cells hold together: at least two rows and two columns, rows of similar length, most cells filled. A table that does not hold together stays a picture. A table numbered like an equation, "(16)" beside or under it, has that number written under it as an equation number, so references to it find the table.

## What is kept between conversions

*General: it does not depend on the book. Tried on test edits only so far, not on a book corrected by hand over months.*

Before a book is converted again, pdf2md collects what the reader changed in the old files, from the chapter files and from `book.md`.

* **Equations.** Every equation read from a picture or a scan carries an invisible mark naming its picture, `<!-- pdf2md:eq … -->`. A file in the book's folder records what the last conversion wrote for each. An equation that now differs from that record is the reader's correction. It is stored in `~/.local/share/pdf2md/corrections/` and used from then on in place of the model's reading. Both forms of math are read, so a book keeps its corrections when it changes form. mdBun's mark of an equation that the reader changed to improve on the book, not to match it (`<!-- mdBun:improved -->` behind the equation), is kept by picture too, as an empty file beside the correction, and written again behind the equation. The record of what was written says which equations had the mark, so a mark the reader has taken away since is forgotten. Like a correction, it holds for every place that has the same picture.
* **Table cells.** Every table carries a mark above it, `<!-- pdf2md:table … -->`, and a corrected cell is kept by table, row and column. The rows of the title and of the note are not counted (a wrong word there is corrected with a fix, as in running text), and a cell that differs from what was written only in the files its links go to (as in `book.md`) is not a correction. If a later version of pdf2md reads the table differently, it counts as another table and the old corrections are not applied. The table of contents is made anew each time.
* **Program code read from a picture** carries `<!-- pdf2md:code … -->` above its block. Its text is recorded and compared like an equation's, with its line breaks and indents, and a changed one is kept by picture. A note under it is kept by the picture too.
* **The picture under a table** keeps the state the reader left it in: open, folded, or deleted.
* **Notes.** A `<details class="mine">` block or an alert box goes back after the paragraph or equation it followed. That block is found by its first words, or else by the nearest anchor above it. A note under a displayed equation read from a picture is kept by that picture, so it stays under the equation when the equation's LaTeX is corrected. A note whose place is gone goes to the end of its chapter under "Your notes whose place was not found". Two notes of the same words are two notes; a note that book.md and a chapter's file both hold is one.
* **Markers in a sentence** that point to a note go back after the same words in the same paragraph, or at the paragraph's end if those words changed.
* **Prose fixes**, `<span class="fix" data-was="…">`, are redone in the same paragraph.
* **Equations built from the PDF's text** have no picture to be kept by. A corrected one stands under a mark that holds its LaTeX as converted, `<!-- pdf2md:fix … -->` (in either form of math: above a ```` ```math ```` block, or above a block between `$$`; mdBun writes the mark when an equation is corrected in it), and every displayed equation of the chapter that is converted to that LaTeX again is replaced by the corrected block. A displayed line that was no equation of the book (a piece torn off a formula in the sentence above it) is taken out: `<!-- pdf2md:gone … -->` stands for it. An alert that begins "**Edited by Claude.**" right under either is part of the fix and goes where the fix goes, and so is mdBun's mark of an improvement on the line under the corrected block. If a later version of pdf2md converts the equation differently, or the book is written in the other form of math, the fix no longer fits; it is kept with the notes whose place was not found, so it can be looked at again.

**Files that do not change are not touched.** A chapter file, a picture or `book.md` is written only if its bytes differ from the file that is there; a picture is cut to a name of its own first and moved in only if it differs; the link `book.pdf` stays if it goes to the same place. The app's Replace empties the folder into the Trash first, so it notes each file's length, a print of its bytes and its date before, and gives the files that come out the same their dates back. A conversion that changes nothing leaves every date as it was: a tool that copies by date and size (rclone, rsync) then has nothing to send, where it had sent every file before.

A copy of the notes and fixes is kept in `~/.local/share/pdf2md/notes/`. Notes follow a chapter whose file name changes. Any other edit to the text is not kept.

## The app window

* **Replace or keep both.** If the book's folder exists, Replace moves the old files to the Trash as one folder, "<name> (previous)", once the new conversion has succeeded. The new files are written into the same folder, so a terminal or editor open in it sees them at once. Keep both writes to "<name> (1)".
* **Reader.** The command gets the book's folder as its last argument. `{folder}` or `{file}` in the command puts the folder or `book.md` elsewhere. Left empty, `xdg-open` opens the folder.
* **The list.** The window lists what can be converted in the folder chosen: its PDFs, and its course folders (a folder with saved web pages, at least as many PDFs in it or its subfolders, and no Markdown yet). A book counts as converted when its folder stands beside its source, or is among the books noted in `~/.config/pdf2md/books` under its name; one that went to another folder goes there again unless a folder is chosen for it. A course's book is named as its folder without " Downloads" (with " (book)" added where the folder has no such ending).
* **A box is unchecked when its book has been converted**, as each one ends, so the next Convert takes only what is checked then. A book that failed stays checked, to be tried again.
* **Several books** are converted one after the other, in the list's order, each with its own folder and math form where the two boxes under the list ask for them. One question covers all the book folders that are already there. A course's book is rebuilt in place, not moved to the Trash. A book that fails is marked in the list and the others go on.
* **Progress.** The bar goes by time. A conversion of more than 30 seconds also lists its steps: done, running or waiting. Estimates come from the speed measured so far. Before a step has done anything they are guesses, set several times too high on purpose. A step that has counted nothing new for 30 seconds says so.
* **Settings** are kept in `~/.config/pdf2md/`.

## Measurements

* **UCSD Physics 130** (TeX, 548 pages; the sample in [`samples/ucsd`](../samples/ucsd)): about 10 seconds. 37 chapters, all 548 page markers, every contents entry linked, 99 pictures. 8,746 formulas, all of which render. Known to be wrong still: about 200 indices without their letter, about 50 root signs without what is under them, 14 integrals whose upper limit went to the line above, and 58 powers written as raised numbers.
* **Dyson** (TeX, 149 pages): every chapter, contents entry, reference and index entry is linked, and all 93 footnotes. KaTeX renders every displayed equation. It is the sample in [`samples/dyson`](../samples/dyson) and the reference for regressions.
* **Sakurai** (ebook-made, 771 PDF pages): about two minutes for the first conversion on an RTX 5080, about 15 seconds after that. All 12,665 formulas are written where a Markdown reader takes them as math.
* **Zee, second edition** (publisher's PDF, 605 pages): about 10 seconds. 14,102 formulas, all of which render; 2,272 links inside the book, none broken. Known to be wrong still: about 60 indices that lost their letter, a few root signs, and labels of some figures written as formulas.
* **Zee, first edition** (scan, 537 pages; measured before the scan was lost): half an hour for the first conversion, half a minute after that. 10,262 formulas, all of which render. Of 1,190 displayed equations read twice, 403 are listed to check, 40 with large differences. Of 119 footnotes, 116 are linked to their markers, 43 of those found on the pictures of the pages.
