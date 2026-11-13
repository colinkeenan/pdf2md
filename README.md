# pdf2md — PDF textbook to Markdown

pdf2md turns a PDF textbook into Markdown you can read, search and annotate: one file per chapter, plus `book.md` with the whole book in one file. Formulas become LaTeX, not pictures. The contents, the index, footnotes and references become links. Every printed page is marked, and the mark opens that page of the PDF, so you can check the text against the book.

It is made for physics and math textbooks and has been used on three: a TeX-made PDF, a PDF made from an ebook, and a scan. [`samples/dyson`](samples/dyson) is the first of them, converted. To see it as intended, clone the repository and open that folder with mdBun; GitHub shows it less well (see [below](#math-github-or-standard)).

The books are best read with [mdBun](https://github.com/colinkeenan/mdBun), a small Markdown viewer written alongside pdf2md. It typesets the math, flashes the target of a link, and lets you correct an equation or add a note by clicking.

## What you get

* **Chapter files** named by chapter number (`03-theory-of-angular-momentum.md`), with `00-front-matter.md` and the table of contents before them, and `book.md` holding everything.
* **Page markers** such as <sup>[p. 32]</sup> where each printed page starts, under a faint bar. Search for "p. 32" to find a page; click the marker to open that page of the PDF.
* **Links**: contents entries go to their chapters and sections, index page numbers go to the word on that page, footnote markers go to their notes and back, and "Section 3.5", "Figure 1.1" or "(3.422)" go to what they name. The words an index entry points to are marked as its target; mdBun shows them in blue, though they are not links themselves.
* **Math** as LaTeX, in sentences and as displayed equations with their numbers.
* **Pictures** cut from the PDF into `images/`, with captions numbered through the book.
* **Tables** as tables, each with a picture of the printed table under it to check against. Where the book sets a table's title or its note as part of the table, they are cells across the top and the foot of it. A table that runs over a page turn is one table.
* **Footnotes** at the foot of their page, end notes at the end of the chapter that cites them.

## Three kinds of PDF

pdf2md looks at the PDF and picks the way to read it. What you need to install depends on the kind.

| The PDF | How it is read | Needs |
|---|---|---|
| Holds text and math as text (made with TeX) | From the positions and fonts of the characters | poppler |
| Holds text, but math as small pictures (made from an ebook) | Text as above; each equation picture by a math-recognition model | poppler, texify, Tesseract |
| Holds only pictures of pages (a scan) | Every page by a page-reading model | poppler, marker, Tesseract; texify for the list of equations to check |

The models run on your own computer; nothing is sent anywhere. Their readings are cached, so the first conversion of an ebook-made or scanned book is slow (minutes to half an hour with a recent graphics card) and later ones take seconds. After an update of pdf2md, the first conversion of such a book can take a few minutes again: readings kept from before are checked by the new rules once.

## Install

**poppler** is always needed: Arch `poppler`, Debian/Ubuntu/Fedora `poppler-utils`.

**The app** needs Rust, Qt 6 and KDE Frameworks 6. Rust can come from your distribution's packages (Arch: `sudo pacman -S rust`) or from <https://rustup.rs>.

* Arch: `sudo pacman -S qt6-base qt6-declarative kirigami`
* Fedora: `sudo dnf install qt6-qtbase-devel qt6-qtdeclarative-devel qt6-qtdeclarative kf6-kirigami-devel kf6-kirigami qt6-qtbase-private-devel`
* Debian/Ubuntu: `sudo apt install qt6-base-dev qt6-declarative-dev qml6-module-qtquick-controls qml6-module-qtquick-dialogs qml6-module-org-kde-kirigami qml6-module-qt-labs-folderlistmodel` (a release that ships KF6)

There are two ways to start the app. Both are run in the cloned repository.

*Run it from the repository*, with nothing installed:

```sh
cargo run --release -p pdf2md-app
```

This builds the app into the repository's `target/release/` folder if the source has changed, then starts it. It always runs the current source, but only from a terminal. Without `--release`, cargo makes a slower debug build in `target/debug/` instead.

*Install it*, so that it appears as "pdf2md" in your app launcher and `pdf2md-app` works in any terminal:

```sh
cargo build --release -p pdf2md-app
install -Dm755 target/release/pdf2md-app ~/.local/bin/pdf2md-app
install -Dm644 app/pdf2md-app.desktop ~/.local/share/applications/pdf2md-app.desktop
install -Dm644 app/pdf2md.svg ~/.local/share/icons/hicolor/scalable/apps/pdf2md.svg
```

The first line builds the app; the others copy the program, its launcher entry and its icon into your home folder. The installed program is a copy: after you update the source (`git pull`), run the first two lines again, or the launcher keeps starting the old version.

The launcher and the terminal find the installed program only if `~/.local/bin` is on your PATH. Not every distribution puts it there: check with `echo $PATH`. If it is missing, add it and log in again:

* bash or zsh: add `export PATH="$HOME/.local/bin:$PATH"` to `~/.profile` (zsh: `~/.zprofile`). Putting the line in `~/.bashrc` or `~/.zshrc` works too, but usually only for terminals: the app launcher does not read those files.
* fish: `fish_add_path ~/.local/bin`

Building needs `qmake6` on your PATH (or set `QMAKE=/usr/lib/qt6/bin/qmake`). The command-line tool needs only Rust and poppler.

**texify**, for equations that are pictures (about 7 GB with PyTorch):

```sh
mkdir -p ~/.local/share/pdf2md/ocr && cd ~/.local/share/pdf2md/ocr
uv venv --python 3.12 .venv
uv pip install --python .venv/bin/python torch torchvision --index-url https://download.pytorch.org/whl/cu128
uv pip install --python .venv/bin/python texify "transformers==4.45.2"
```

texify 0.2.1 does not work with newer transformers. Without texify, equation pictures stay pictures.

**marker**, for scanned books (about 6 GB, plus 3 GB of models downloaded at the first book):

```sh
mkdir -p ~/.local/share/pdf2md/scan && cd ~/.local/share/pdf2md/scan
uv venv --python 3.12 .venv
uv pip install --python .venv/bin/python "marker-pdf==1.10.2"
```

Read marker's licence before you use it: its model weights are free for personal use and research only, with limits for larger organizations. The texts and a summary are in [`licenses/marker`](licenses/marker).

**Tesseract** reads program code, table cells and words that the models miss: Arch `tesseract tesseract-data-eng`.

## Use

### The app

Start pdf2md from your app launcher, or with `cargo run --release -p pdf2md-app` in the repository (see [Install](#install)). Choose the textbook and, if you like, an output folder (the default is the folder the PDF is in), then press Convert. The book is written to `<output folder>/<PDF name>/`.

* **Converting again.** If the book's folder exists, the app asks whether to replace it or keep both. Replace moves the old files to the Trash and keeps your corrections and notes (see below).
* **Reader** is the command of your Markdown viewer, for example `md` for mdBun. "Open Book" and the list of converted books in the window start it with the book's folder. Write `{file}` in the command to pass `book.md` instead, as in `okular {file}`.
* **Math** chooses how formulas are written (next section).
* **Progress.** A bar shows how far the conversion is; a long conversion also lists its steps. The first time estimates are deliberately high.

### The command line

```sh
cargo run -p pdf2md-core --bin pdf2md-cli -- book.pdf out/
cargo run -p pdf2md-core --bin pdf2md-cli -- book.pdf out/ 10 12            # only PDF pages 10 to 12
cargo run -p pdf2md-core --bin pdf2md-cli -- --math standard book.pdf out/  # formulas as $…$ and $$…$$
```

Converting into a folder that already holds the book replaces it in place and keeps your corrections and notes.

### Math: GitHub or standard

Markdown has no single way to write math, so you choose one. A book is rewritten in the chosen form when you convert it again.

| | Formula in a sentence | Displayed equation | Equation number | Shown by |
|---|---|---|---|---|
| **github** (the default) | ``$`…`$`` | ```` ```math ```` block | text after the equation | GitHub, GitLab, mdBun |
| **standard** | `$…$` | `$$…$$` | `\tag{…}` | mdBun and most other Markdown programs |

The github form exists because GitHub reads what stands between `$` signs as Markdown before it reads it as math, which garbled about one formula in five of the sample book, and because GitHub breaks an equation that has a `\tag` into a column of single symbols.

In the app, typing a reader that is known to understand only the standard form (VS Code, Obsidian, Typora, MarkText, Zettlr, ghostwriter, Joplin, Logseq, pandoc) switches "Math" to standard; you can switch it back. That list comes from those programs' documentation, not from trying each one. On the command line the form is github unless you pass `--math standard`.

**GitHub will not look as good as mdBun.** mdBun typesets formulas itself and brings its own fonts. GitHub leaves the drawing to your browser, which needs a math font of its own: without one, brackets, roots and sums stay the size of a letter. Chrome on Linux looks for "Latin Modern Math" (Arch `otf-latinmodern-math`), and even with it the math looks thinner than in mdBun. Phones mostly have no such font. The page markers do not jump to their page on GitHub either: its PDF viewer opens `book.pdf` at the front and loads a few pages at a time. Read locally, they open the PDF at the page.

## Correcting and annotating a book

No conversion is perfect, and a textbook is something you write in. What you change in the Markdown is kept when you convert the book again, as long as pdf2md can tell it is yours:

* **Equations read from a picture or a scan**: just edit the LaTeX. In mdBun, click the equation.
* **Table cells**: edit the cell where it stands, between the empty lines inside its `<td>`. The title across the top of a table and the note across its foot are not cells: correct words there as you would in the text, with a fix (next point; in mdBun, "Fix text").
* **The picture under a table**: once the table is right, fold the picture away for good by deleting the word `open` in its `<details open>`, or delete the fold.
* **Wrong words in the text**: wrap the fix so it can be redone, `<span class="fix" data-was="as converted">as in the book</span>`.
* **Your own notes**: a collapsible note or an alert box, placed after the paragraph or equation it belongs to.

  ```markdown
  <details class="mine">
  <summary>✎ My note: why the sign flips</summary>

  Your explanation, with math like $`[A,B]=-[B,A]`$.

  </details>

  > [!WARNING]
  > I think the factor should be $`\frac12`$.
  ```

  A marker inside a sentence can point to a note: `<sup>[✎](#mine-3)</sup>`, with `id="mine-3"` on the note.

Any other edit to the text is lost at the next conversion. A note whose place can no longer be found goes to the end of its chapter; it is never dropped. Corrections are stored in `~/.local/share/pdf2md/corrections/`, and a copy of your notes in `~/.local/share/pdf2md/notes/`.

**Equations to check.** For ebook-made and scanned books, each displayed equation is read twice. Where the two readings disagree, the equation is listed in `check-equations.md` in the book's folder, biggest differences first. An entry marked **bold?** is one where the two readings set different letters bold. A last group lists equations whose pictures have bold letters while the reading has none. An equation you correct drops off the list.

## Limitations

* **Recognized math is good, not perfect.** A misread formula does not always look broken. Check equations you rely on against the book; the page markers make that one click.
* **Bold letters in recognized math are unsure.** The recognizer mixes up bold and ordinary letters, which matters where a bold letter is a vector. pdf2md takes the bold away where the equation's picture has no bold letter at all, and marks doubtful equations in `check-equations.md`, but it cannot tell which letters of an equation are the bold ones.
* **Each new book brings cases not seen before.** pdf2md has been worked through three books. The rules that tell a heading from a line of text, or a formula from prose, are tuned on those.
* **TeX-made PDFs need known math fonts**: Computer Modern, AMS, or fonts named "Math", "Symbol" or "STIX". Other fonts must be added in the source.
* **Pictures are images**, so the labels inside them cannot be searched. A picture made of only one or two strokes is not found.
* **Scanned books lose some footnote markers.** A footnote whose marker was not found still stands at the foot of its page and says so. In the test scan, 3 of 119 were left that way.
* **A scanned book's table of contents** is read from poor text, so an entry can keep a misread character.

## How it works

[`docs/how-it-works.md`](docs/how-it-works.md) has the detail: the layout of the source, what exactly is written, how each kind of PDF is read, and the measurements from the test books. To see what pdf2md makes of a page, `pdf2md-cli --dump book.pdf 10 12` prints every line of PDF pages 10 to 12 with its position, font and scripts.

## Licence

pdf2md is free software under the GNU General Public License, version 3 or later, and comes without any warranty; see [`LICENSE`](LICENSE). Copyright (C) 2026 Colin Keenan.

The programs it runs have licences of their own: poppler (GPL), [texify](https://github.com/VikParuchuri/texify) (code GPL 3, model weights CC BY-SA 4.0), [Tesseract](https://github.com/tesseract-ocr/tesseract) (Apache 2.0) and [marker](https://github.com/datalab-to/marker) (code GPL 3, model weights under a licence that limits who may use them: see [`licenses/marker`](licenses/marker)). None of them is contained in pdf2md; you install them yourself.
