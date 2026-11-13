"""Read a table that the book holds as a picture, cell by cell.

Given whole to the math recognizer, such a picture comes back as one
formula with its cells in the wrong rows and columns. Here the rows and
the columns are found on the picture itself (they are set apart by white
space), and each cell is read on its own.

    table_ocr.py PENDING_DIR OUT_DIR

reads every PENDING_DIR/NAME.png and writes OUT_DIR/NAME.cells: the line
"aligned", then a line for each row of the table, its cells parted by
tabs. A cell is "<", "^" or ">" (set at the left of its column, in its
middle, at its right) and its LaTeX (words as \\text{...}); an empty cell
is empty. No file is written for a picture that does not part into two
rows and two columns or more.
"""

import re
import sys
from pathlib import Path

import cv2
import numpy as np
from PIL import Image, ImageOps

import math_ocr


def bands(mask, least_gap: int):
    """The runs of rows of `mask` that have ink, as (start, end); runs
    closer together than `least_gap` are one (the parts of a fraction)."""
    has = mask.sum(axis=1) > 0
    out = []
    y = 0
    while y < len(has):
        if has[y]:
            y0 = y
            while y < len(has) and has[y]:
                y += 1
            if out and y0 - out[-1][1] < least_gap:
                out[-1] = (out[-1][0], y)
            else:
                out.append((y0, y))
        else:
            y += 1
    return out


def runs(has):
    """The runs of True in a row of flags, as (start, end)."""
    out = []
    x = 0
    while x < len(has):
        if has[x]:
            x0 = x
            while x < len(has) and has[x]:
                x += 1
            out.append((x0, x))
        else:
            x += 1
    return out


def cells_of(gray):
    """The rows of the table as (top, bottom) and its columns as (left,
    right), with the picture's ink; None if it is no table. A row is a
    line of text. A column ends where no row has ink for a stretch and, in
    some row, what stands left and right of there is clearly apart (not
    just two words of one cell)."""
    # (the ground may be grey: ink is what is clearly darker than most of it)
    ground = int(np.median(gray))
    ink = (gray < ground - 70).astype(np.uint8)
    h, w = ink.shape
    # rules across or down the table are no text
    for y in range(h):
        if ink[y].sum() > 0.5 * w:
            ink[y] = 0
    for x in range(w):
        if ink[:, x].sum() > 0.6 * h:
            ink[:, x] = 0
    # lines of text: the parts of a fraction (a pixel or two apart) are one
    rows = [r for r in bands(ink, 3) if r[1] - r[0] >= 6]
    if len(rows) < 2:
        return None
    heights = sorted(b - a for a, b in rows)
    line = heights[len(heights) // 2]
    # where no row has ink
    free = runs(ink[[y for a, b in rows for y in range(a, b)]].sum(axis=0) == 0)
    wide = 0.75 * line
    seps = []
    for a, b in free:
        if a == 0 or b == w or b - a < 3:
            continue
        # In every row that has something on both sides of here, the two
        # stand clearly apart (by most of a line's height): then they are
        # two cells. Where in some row they stand as close as two words,
        # it is a gap inside a cell that the rows happen to share.
        votes = []
        for top, bottom in rows:
            has = ink[top:bottom].sum(axis=0) > 0
            left = a
            while left > 0 and not has[left - 1]:
                left -= 1
            right = b
            while right < w and not has[right]:
                right += 1
            if left > 0 and right < w:
                # (not across a whole column that the row leaves empty)
                votes.append(right - left)
        # (a row that leaves a column empty there does not tell; but if
        # every row has that far from one side to the other, they all do)
        near = [v for v in votes if v < 6 * line] or votes
        if near and min(near) >= wide:
            seps.append((a + b) // 2)
    cols = list(zip([0] + seps, seps + [w]))
    if len(cols) < 2:
        return None
    # Two lines that reach into each other's height (a fraction below, a
    # lowered figure above) came out as one tall row. They are two where
    # some cell has a clear gap in its height and every other cell lies
    # wholly above the gap or wholly below it.
    parted = []
    for top, bottom in rows:
        cut = None
        if bottom - top > 1.8 * line:
            spans = []
            for left, right in cols:
                ys = np.where(ink[top:bottom, left:right].sum(axis=1) > 0)[0]
                spans.append((int(ys[0]), int(ys[-1]) + 1) if len(ys) else None)
            for k, (left, right) in enumerate(cols):
                inner = bands(ink[top:bottom, left:right], max(3, int(0.4 * line)))
                for (_, g0), (g1, _) in zip(inner, inner[1:]):
                    others = [s for j, s in enumerate(spans) if j != k and s is not None]
                    if others and all(s[1] <= g1 or s[0] >= g0 for s in others):
                        cut = top + g0
                        break
                if cut:
                    break
        if cut:
            parted += [(top, cut), (cut, bottom)]
        else:
            parted.append((top, bottom))
    return parted, cols, ink


def alignments(spans, rows):
    """How each cell is set in its column: "<" at the left, "^" in the
    middle, ">" at the right, by where its ink starts and ends between the
    leftmost and the rightmost ink of the column. A cell that fills the
    column from side to side does not tell: it is set as most of the
    others of its column are (in the middle if none tells)."""
    # (the head is the first row)
    heights = sorted(b - a for a, b in rows)
    near = 0.5 * heights[len(heights) // 2]
    signs = [[""] * len(r) for r in spans]
    for c in range(len(spans[0])):
        cells = [r[c] for r in spans]
        there = [s for s in cells if s is not None]
        if not there:
            continue
        left, right = min(s[0] for s in there), max(s[1] for s in there)
        for r, s in enumerate(cells):
            if s is None:
                continue
            at_left, at_right = s[0] - left <= near, right - s[1] <= near
            signs[r][c] = "?" if at_left and at_right else "<" if at_left else ">" if at_right else "^"
        # (the cells under the head first: a head is often set in the
        # middle of a column whose cells are not)
        tells = lambda rows_: [signs[r][c] for r in rows_ if signs[r][c] and signs[r][c] in "<^>"]
        told = tells(range(1, len(cells))) or tells(range(len(cells)))
        most = max("^<>", key=told.count) if told else "^"
        for r in range(len(cells)):
            if signs[r][c] == "?":
                signs[r][c] = most
    return signs


def number(img: Image.Image):
    """A cell that is a whole number ("3", "-2"), read as figures only (the
    math recognizer makes signs of lone figures); None if it is not one,
    or Tesseract is missing."""
    import os
    import re
    import subprocess
    import tempfile

    g = img.convert("L")
    g = g.resize((g.width * 3, g.height * 3), Image.LANCZOS)
    with tempfile.NamedTemporaryFile(suffix=".png") as tmp:
        g.save(tmp.name)
        env = dict(os.environ, OMP_THREAD_LIMIT="1")
        try:
            # what it is without help: letters mean it is no number
            free = subprocess.run(["tesseract", tmp.name, "-", "--psm", "7"], capture_output=True, text=True, timeout=60, env=env).stdout.strip()
            # (nor is a lone letter one, unless it is what a 1 or a 0 reads as)
            if re.search(r"[A-Za-z]{2,}|[=+(),/\\[\]{}]|[A-HJ-NP-Za-km-np-z]", free):
                return None
            # (read freely it keeps its minus sign, which the figures-only reading can lose)
            free = re.sub(r"[—–−~_]", "-", free).replace(" ", "")
            if re.fullmatch(r"-?\d{1,3}", free):
                return free
            for mode in ("7", "10"):
                only = subprocess.run(["tesseract", tmp.name, "-", "--psm", mode, "-c", "tessedit_char_whitelist=0123456789-"], capture_output=True, text=True, timeout=60, env=env).stdout.strip().replace(" ", "")
                if re.fullmatch(r"-?\d{1,3}", only):
                    return ("-" + only) if free.startswith("-") and not only.startswith("-") else only
        except (OSError, subprocess.TimeoutExpired):
            return None
    return None


def words(img: Image.Image):
    """The cell as plain words, if it holds only words ("Bosons", "Numbers
    of States"); else None."""
    text = (math_ocr.ocr_text(img) or "").strip()
    parts = text.split()
    if parts and all(p.isalpha() for p in parts) and any(len(p) >= 4 for p in parts) and "\n" not in text:
        return text
    return None


def main():
    src, dst = Path(sys.argv[1]), Path(sys.argv[2])
    files = sorted(src.glob("*.png"))
    todo = []
    for f in files:
        gray = cv2.imread(str(f), cv2.IMREAD_GRAYSCALE)
        found = cells_of(gray) if gray is not None else None
        if found is None:
            continue
        rows, cols, ink = found
        ground = int(np.median(gray))
        grid = []
        # (where each cell's ink starts and ends in its column)
        spans = []
        for top, bottom in rows:
            crops = []
            spans.append([])
            for left, right in cols:
                part = ink[top:bottom, left:right]
                xs = np.where(part.sum(axis=0) > 0)[0]
                spans[-1].append((int(xs[0]), int(xs[-1]) + 1) if len(xs) else None)
                if len(xs) == 0:
                    crops.append(None)
                    continue
                # (only the cell's own lines: its neighbours above and below stay out)
                ys = np.where(part.sum(axis=1) > 0)[0]
                piece = gray[top + ys[0] : top + ys[-1] + 1, max(0, left + xs[0] - 4) : left + xs[-1] + 5]
                # on white, with room around it, as the recognizer reads best
                piece = np.where(piece.astype(int) >= ground - 30, 255, piece).astype(np.uint8)
                img = ImageOps.expand(Image.fromarray(piece).convert("RGB"), border=20, fill="white")
                # (stacked: a fraction, which only the math recognizer reads)
                stacked = len(bands(part, 1)) > 1
                crops.append((img, stacked))
            grid.append(crops)
        todo.append((f, grid, alignments(spans, rows)))
    if not todo:
        return
    from texify.inference import batch_inference
    from texify.model.model import load_model
    from texify.model.processor import load_processor

    model, processor = load_model(), load_processor()
    for f, grid, aligned in todo:
        lines = ["aligned"]
        for crops, signs in zip(grid, aligned):
            cells = []
            for crop in crops:
                if crop is None:
                    cells.append("")
                    continue
                img, stacked = crop
                text = None if stacked else words(img)
                if text is not None:
                    cells.append("\\text{" + text + "}")
                    continue
                # (a narrow cell that is not stacked may be a lone number)
                plain = None if stacked or img.width > 32 + 4 * img.height else number(img)
                if plain is not None:
                    cells.append(plain)
                    continue
                t = " ".join((math_ocr.read_one(img, model, processor, batch_inference) or "").split())
                # (what the recognizer makes of a lone figure: a bold sigma
                # with the figure raised, a bold l for 1)
                lone = re.fullmatch(r"\\mathbf\{\\Sigma\}\^\{(-?\d+)\}|\\mathbf\{(\d+)\}", t)
                if lone:
                    t = lone.group(1) or lone.group(2)
                elif t == "\\mathbf{l}":
                    t = "1"
                # (an array of one cell is that cell; "l" set as a word is 1)
                one = re.fullmatch(r"\\begin\{array\}\{[lcr ]*\}(.*)\\end\{array\}", t)
                if one and "&" not in one.group(1) and "\\\\" not in one.group(1):
                    t = one.group(1).strip()
                t = t.replace("\\operatorname{l}", "1")
                cells.append(t)
            lines.append("\t".join(sign + cell if cell else "" for sign, cell in zip(signs, cells)))
        (dst / f"{f.stem}.cells").write_text("\n".join(lines) + "\n")
        print(f"  table {f.stem}: {len(lines) - 1} rows, {max(len(l.split(chr(9))) for l in lines)} columns", file=sys.stderr, flush=True)


if __name__ == "__main__":
    main()
