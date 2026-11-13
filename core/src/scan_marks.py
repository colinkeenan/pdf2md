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

    want2 <TAB> LABELS <TAB> LIMIT
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
            px, py, pw, ph = of_line[k - 1]
            # (not behind a raised stroke: the 1 of an exponent "-1")
            minus = py + ph <= base - 0.45 * small and ph <= 0.3 * small and pw >= 2 * ph
            if high and little and not minus and -2 <= x - (px + pw) <= 0.6 * small:
                out.append((x, y, w, h, y0, y1, small))
    return out


def read_digit(gray, piece) -> str:
    """The digit a small piece of ink is, as most of five readings of it
    have it (enlarged more and less, with more and less of what is around
    it): one reading takes a 5 for a 3 now and then, and a piece that is
    no digit reads as one thing and another."""
    x, y, w, h = piece[:4]
    env = dict(os.environ, OMP_THREAD_LIMIT="1")
    readings = []
    for scale, mode, room in ((4, "10", 6), (3, "10", 2), (3, "13", 6), (6, "10", 2), (6, "13", 6)):
        crop = gray[max(0, y - room) : y + h + room, max(0, x - room) : x + w + room]
        crop = cv2.resize(crop, None, fx=scale, fy=scale, interpolation=cv2.INTER_CUBIC)
        crop = cv2.copyMakeBorder(crop, 40, 40, 40, 40, cv2.BORDER_CONSTANT, value=255)
        ok, png = cv2.imencode(".png", crop)
        if not ok:
            return ""
        try:
            read = subprocess.run(["tesseract", "-", "-", "--psm", mode, "-c", "tessedit_char_whitelist=0123456789"], input=png.tobytes(), capture_output=True, timeout=60, env=env)
        except (OSError, subprocess.TimeoutExpired):
            return ""
        readings.append(read.stdout.decode("utf-8", "replace").strip())
    best = max(set(readings), key=readings.count)
    return best if readings.count(best) >= 3 else ""


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
    two before a punctuation mark: an exponent follows a symbol), or the
    stop after a number or a bracket ("in 1972.", "(Exercise 4)."). The
    words are Tesseract's, which at times takes the mark into the word
    before it, as its last letter or sign."""
    pieces = raised_pieces(gray, limit)
    if not pieces:
        return []
    words = page_words(gray)
    found = []
    for piece in pieces:
        x, _, w, _, y0, y1, small = piece
        # (the word that ends at the piece, or that reaches over it and
        # ends in a sign that is no letter: the mark, read as that sign)
        over = lambda t: (t[0] + t[2]) - x
        before = [t for t in words if t[1] < y1 and t[1] + t[3] > y0 and t[0] < x and (abs(over(t)) <= 0.8 * small or (0 < over(t) <= w + 1.2 * small and not t[4][-1].isalpha()))]
        if not before:
            continue
        # (the one that ends nearest the piece: a word of the line below
        # can reach up into this line)
        word = min(before, key=lambda t: abs(over(t)))
        text = word[4]
        if over(word) > 0.5 * w and len(text) > 1 and not text[-1].isalpha():
            text = text[:-1]
        core = re.sub(r"[^A-Za-z]+$", "", text)
        tail = text[len(core) :]
        letters = len(re.search(r"[A-Za-z]*$", core).group(0))
        after_word = letters >= 3 or (letters >= 2 and re.match(r"[.,;:!?)]", tail))
        after_stop = re.search(r"[0-9)][.,;:]$", text) is not None
        if not (after_word or after_stop):
            continue
        digit = read_digit(gray, piece)
        if digit in labels:
            k = words.index(word)
            found.append((digit, [t[4] for t in words[max(0, k - 2) : k]] + [core if after_word else text.rstrip(".,;:")]))
    return found


def main():
    pdf, out = sys.argv[1], Path(sys.argv[2])
    out.mkdir(parents=True, exist_ok=True)
    requests = [line.rstrip("\n").split("\t") for line in sys.stdin if line.strip()]
    print(f"  marks 0/{len(requests)}", file=sys.stderr, flush=True)

    def look(request):
        page, labels, limit = request
        gray = page_picture(pdf, int(page))
        lines = [f"want2\t{labels}\t{limit}"]
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
