"""Find footnote marks on the pictures of scanned pages.

The page reader does not see every small raised number that points to a
footnote (nor does Tesseract, nor the text an earlier reading left in the
PDF). But on the picture of the page such a mark is plain: a small piece of
ink, set high in its line, right behind a word or the full stop after it.

    scan_marks.py BOOK.pdf OUT_DIR < requests

Each request is a line "PAGE <TAB> LABELS <TAB> LIMIT": the page (1 = the
first sheet), the marks looked for ("1,3"), and how far down the page to
look, in points (where its footnotes begin; 0: all of it). For each page
OUT_DIR/mNNNN.txt is written:

    want <TAB> LABELS <TAB> LIMIT
    mark <TAB> LABEL <TAB> the two words before it and the word it follows

How far it is goes to stderr: "  marks 3/12".
"""

import os
import re
import subprocess
import sys
from pathlib import Path

import cv2
import numpy as np

DPI = 300


def page_picture(pdf: str, page: int):
    out = subprocess.run(["pdftoppm", "-r", str(DPI), "-f", str(page), "-l", str(page), "-gray", "-png", pdf], capture_output=True, timeout=120).stdout
    return cv2.imdecode(np.frombuffer(out, np.uint8), cv2.IMREAD_GRAYSCALE)


def raised_pieces(gray, limit: int):
    """Pieces of ink that are small, stand high in their line and come
    right behind another piece: (x, y, w, h, top and bottom of the line,
    height of the line's small letters)."""
    ink = (gray < 140).astype(np.uint8)
    height = ink.shape[0] if limit <= 0 else min(ink.shape[0], limit)
    rows = ink[:height].sum(axis=1) > 0
    lines = []
    y = 0
    while y < height:
        if rows[y]:
            y0 = y
            while y < height and rows[y]:
                y += 1
            if y - y0 >= 12:
                lines.append((y0, y))
        else:
            y += 1
    _, _, stats, _ = cv2.connectedComponentsWithStats(ink, 8)
    pieces = [(int(s[0]), int(s[1]), int(s[2]), int(s[3])) for s in stats[1:] if s[4] >= 12]
    out = []
    for y0, y1 in lines:
        of_line = sorted((p for p in pieces if p[1] >= y0 - 2 and p[1] + p[3] <= y1 + 2), key=lambda p: p[0])
        if len(of_line) < 8:
            continue
        heights = sorted(p[3] for p in of_line)
        small = heights[len(heights) // 2]
        bottoms = sorted(p[1] + p[3] for p in of_line)
        base = bottoms[len(bottoms) // 2]
        for k, (x, y, w, h) in enumerate(of_line):
            if k == 0:
                continue
            high = y + h <= base - 0.45 * small
            little = 0.45 * small <= h <= 1.25 * small and w <= small
            px, _, pw, _ = of_line[k - 1]
            if high and little and -2 <= x - (px + pw) <= 0.6 * small:
                out.append((x, y, w, h, y0, y1, small))
    return out


def read_digit(gray, piece) -> str:
    x, y, w, h = piece[:4]
    crop = gray[max(0, y - 6) : y + h + 6, max(0, x - 6) : x + w + 6]
    crop = cv2.resize(crop, None, fx=4, fy=4, interpolation=cv2.INTER_CUBIC)
    crop = cv2.copyMakeBorder(crop, 40, 40, 40, 40, cv2.BORDER_CONSTANT, value=255)
    ok, png = cv2.imencode(".png", crop)
    if not ok:
        return ""
    env = dict(os.environ, OMP_THREAD_LIMIT="1")
    try:
        read = subprocess.run(["tesseract", "-", "-", "--psm", "10", "-c", "tessedit_char_whitelist=0123456789"], input=png.tobytes(), capture_output=True, timeout=60, env=env)
    except (OSError, subprocess.TimeoutExpired):
        return ""
    return read.stdout.decode("utf-8", "replace").strip()


def page_words(gray):
    """The words Tesseract reads on the page, with their boxes."""
    ok, png = cv2.imencode(".png", gray)
    env = dict(os.environ, OMP_THREAD_LIMIT="1")
    try:
        read = subprocess.run(["tesseract", "-", "-", "--psm", "6", "tsv"], input=png.tobytes(), capture_output=True, timeout=300, env=env)
    except (OSError, subprocess.TimeoutExpired):
        return []
    out = []
    for line in read.stdout.decode("utf-8", "replace").splitlines()[1:]:
        f = line.split("\t")
        if len(f) > 11 and f[11].strip():
            out.append((int(f[6]), int(f[7]), int(f[8]), int(f[9]), f[11]))
    return out


def marks(gray, labels, limit: int):
    """The marks among `labels` found on the page: (label, the words up to
    the one it follows). A mark follows a word (three letters or more, or
    two before a punctuation mark: an exponent follows a symbol)."""
    pieces = raised_pieces(gray, limit)
    if not pieces:
        return []
    words = page_words(gray)
    found = []
    for piece in pieces:
        x, _, _, _, y0, y1, small = piece
        before = [t for t in words if t[1] < y1 and t[1] + t[3] > y0 and abs((t[0] + t[2]) - x) <= 0.8 * small]
        if not before:
            continue
        word = max(before, key=lambda t: t[0] + t[2])
        core = re.sub(r"[^A-Za-z]+$", "", word[4])
        tail = word[4][len(core) :]
        letters = len(re.search(r"[A-Za-z]*$", core).group(0))
        if not (letters >= 3 or (letters >= 2 and re.match(r"[.,;:!?)]", tail))):
            continue
        digit = read_digit(gray, piece)
        if digit in labels:
            k = words.index(word)
            found.append((digit, [t[4] for t in words[max(0, k - 2) : k]] + [core]))
    return found


def main():
    pdf, out = sys.argv[1], Path(sys.argv[2])
    out.mkdir(parents=True, exist_ok=True)
    requests = [line.rstrip("\n").split("\t") for line in sys.stdin if line.strip()]
    print(f"  marks 0/{len(requests)}", file=sys.stderr, flush=True)

    def look(request):
        page, labels, limit = request
        gray = page_picture(pdf, int(page))
        lines = [f"want\t{labels}\t{limit}"]
        if gray is not None:
            for label, words in marks(gray, labels.split(","), int(float(limit) * DPI / 72)):
                lines.append(f"mark\t{label}\t{' '.join(words)}")
        path = out / f"m{int(page):04d}.txt"
        tmp = path.with_suffix(".tmp")
        tmp.write_text("\n".join(lines) + "\n")
        os.replace(tmp, path)

    # several pages at once: the time goes into Tesseract, a program of its own
    from concurrent.futures import ThreadPoolExecutor

    with ThreadPoolExecutor(max_workers=max(1, (os.cpu_count() or 2) // 2)) as pool:
        for n, _ in enumerate(pool.map(look, requests)):
            print(f"  marks {n + 1}/{len(requests)}", file=sys.stderr, flush=True)


if __name__ == "__main__":
    main()
