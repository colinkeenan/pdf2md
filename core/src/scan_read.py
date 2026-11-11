"""Read the pages of a scanned book (pictures of pages, no text) with
marker (https://github.com/datalab-to/marker): prose, formulas as LaTeX,
headings, footnotes, and where the pictures are.

    scan_read.py BOOK.pdf OUT_DIR FIRST LAST

reads the pages FIRST..LAST (1 = the first sheet) and writes each as
OUT_DIR/pNNNN.txt, skipping the ones that are there. A page file holds

    page <TAB> width <TAB> height
    block <TAB> kind <TAB> left <TAB> top <TAB> right <TAB> bottom <TAB> lines <TAB> html
    line <TAB> text

with a `block` line for each block of the page from top to bottom, in the
PDF's points; `lines` is the number of text lines in it (0 if not known),
`html` is marker's (tabs and line breaks in it written as spaces). A table
(a table of contents is one for marker) is followed by its text line by
line, as `line`s: marker reads tables cell by cell, which shuffles a table
of contents, so its lines are taken from the text the PDF carries under
the picture of the page, or read from the picture by Tesseract. A block of
text that marker read as several lines (an address, the lines of a title
page: lines that end where the author ended them) is followed by those
lines too; `html` has them run together.
How far it is goes to stderr: "  loading 0/1", "  scan 8/537".

Everything runs on this machine; nothing is sent anywhere.
"""

import json
import os
import subprocess
import sys
from pathlib import Path

# pages given to marker at once: it reports after each lot
LOT = 16


def tell(what: str, n: int, total: int):
    print(f"  {what} {n}/{total}", file=sys.stderr, flush=True)


def leaves(block, out):
    """The blocks that hold something themselves, in reading order (a list
    is given as its items, a picture with its caption as the two)."""
    children = block.get("children")
    # (a table is one thing: its html holds all its cells)
    if children and block.get("block_type") not in ("Table", "Form", "TableOfContents"):
        for child in children:
            leaves(child, out)
    else:
        out.append(block)


TABLES = ("Table", "Form", "TableOfContents")


def table_lines(pdf: str, n: int, box):
    """The text lines of an area of page `n`: the text the PDF carries
    there, laid out as on the page, else what Tesseract reads there."""
    x0, y0, x1, y1 = (int(v) for v in box)
    area = ["-f", str(n), "-l", str(n), "-x", str(x0), "-y", str(y0), "-W", str(max(1, x1 - x0)), "-H", str(max(1, y1 - y0))]
    try:
        out = subprocess.run(["pdftotext", "-layout", *area, pdf, "-"], capture_output=True, text=True, timeout=60).stdout
        if not out.strip():
            # (cut out at 300 dots per inch: pdftoppm counts in its pixels)
            scale = 300 / 72
            cut = ["-f", str(n), "-l", str(n), "-r", "300", "-gray"]
            cut += ["-x", str(int(x0 * scale)), "-y", str(int(y0 * scale)), "-W", str(int((x1 - x0) * scale)), "-H", str(int((y1 - y0) * scale))]
            png = subprocess.run(["pdftoppm", *cut, "-png", pdf], capture_output=True, timeout=60).stdout
            env = dict(os.environ, OMP_THREAD_LIMIT="1")
            read = subprocess.run(["tesseract", "-", "-", "--psm", "6", "-c", "preserve_interword_spaces=1"], input=png, capture_output=True, timeout=120, env=env)
            out = read.stdout.decode("utf-8", "replace")
    except (OSError, subprocess.TimeoutExpired):
        return []
    return [line.rstrip().replace("\t", "    ") for line in out.splitlines() if line.strip()]


def write_page(pdf: str, n: int, page, counts, path: Path):
    left, top, right, bottom = page["bbox"]
    lines = [f"page\t{right - left:.1f}\t{bottom - top:.1f}"]
    blocks = []
    for child in page.get("children") or []:
        leaves(child, blocks)
    for b in blocks:
        html = " ".join((b.get("html") or "").split())
        x0, y0, x1, y1 = b["bbox"]
        count, read = counts.get(b.get("id"), (0, []))
        lines.append(f"block\t{b['block_type']}\t{x0 - left:.1f}\t{y0 - top:.1f}\t{x1 - left:.1f}\t{y1 - top:.1f}\t{count}\t{html}")
        if b["block_type"] in TABLES:
            lines += ["line\t" + t for t in table_lines(pdf, n, b["bbox"])]
        elif len(read) >= 2:
            lines += ["line\t" + " ".join(t.split()) for t in read]
    # (written whole or not at all: a page file that is there is done)
    tmp = path.with_suffix(".tmp")
    tmp.write_text("\n".join(lines) + "\n")
    os.replace(tmp, path)


def main():
    pdf, out = sys.argv[1], Path(sys.argv[2])
    first, last = int(sys.argv[3]), int(sys.argv[4])
    out.mkdir(parents=True, exist_ok=True)
    name = lambda n: out / f"p{n:04d}.txt"
    todo = [n for n in range(first, last + 1) if not name(n).exists()]
    total = last + 1 - first
    if not todo:
        return
    tell("loading", 0, 1)
    import logging

    logging.disable(logging.WARNING)
    from marker.config.parser import ConfigParser
    from marker.converters.pdf import PdfConverter
    from marker.models import create_model_dict
    from marker.schema import BlockTypes

    models = create_model_dict()
    tell("loading", 1, 1)
    tell("scan", total - len(todo), total)
    for i in range(0, len(todo), LOT):
        lot = todo[i : i + LOT]
        config = ConfigParser(
            {
                # (marker counts pages from 0)
                "page_range": ",".join(str(n - 1) for n in lot),
                # the page is a picture: read all of it, also where the
                # PDF carries text of an earlier reading under the picture
                "force_ocr": True,
                "output_format": "json",
                "disable_tqdm": True,
                "disable_image_extraction": True,
                # the running heads say the printed page numbers
                "keep_pageheader_in_output": True,
                "keep_pagefooter_in_output": True,
            }
        )
        converter = PdfConverter(
            config=config.generate_config_dict(),
            artifact_dict=models,
            processor_list=config.get_processors(),
            renderer=config.get_renderer(),
        )
        # (as `converter(pdf)` does it, but counting the text lines of
        # each block before marker's later steps join them)
        steps, converter.processor_list = converter.processor_list, []
        document = converter.build_document(pdf)
        counts = {}
        for page in document.pages:
            for block in page.structure_blocks(document):
                read = [t for t in block.raw_text(document).split("\n") if t.strip()]
                counts[str(block.id)] = (len(block.contained_blocks(document, (BlockTypes.Line,))), read)
        for step in steps:
            step(document)
        rendered = converter.resolve_dependencies(converter.renderer)(document)
        rendered = json.loads(rendered.model_dump_json(exclude=["metadata"]))
        pages = rendered.get("children") or []
        if len(pages) != len(lot):
            print(f"  {len(pages)} pages read where {len(lot)} were asked for (pages {lot[0]} to {lot[-1]})", file=sys.stderr, flush=True)
            sys.exit(1)
        for n, page in zip(lot, pages):
            write_page(pdf, n, page, counts, name(n))
        tell("scan", total - len(todo) + i + len(lot), total)


if __name__ == "__main__":
    main()
