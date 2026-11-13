"""Read equation images as LaTeX, for pdf2md (see ocr.rs).

Usage: python math_ocr.py <folder of NAME.png> <output folder>

Writes <output folder>/NAME.tex for every image: the LaTeX of the equation,
"%CODE" and the text for program code (read with Tesseract),
or "%DRAWING" for a picture that is not math: a plot or diagram (long
straight lines meeting at a corner, like the axes of a graph, or one large
connected shape) or lettering such as a publisher's logo (plain words that
the math model reads differently each time). NAME.kind marks an image
that passed these tests. Runs texify
(https://github.com/VikParuchuri/texify) on the GPU if there is one.
"""

import os
import re
import sys
from pathlib import Path

from PIL import Image


# A connected shape at least this wide and this high (pixels) is drawn, not
# typeset. The largest symbols of 96-dpi equation images (sums, integrals)
# stay near 40; brackets and bars are long one way only.
BIG_SHAPE = 56


_told = [0.0]


def tell(what: str, n: int, total: int):
    """How far a step is, for the converter to show ("  lines 20/420"):
    its start, its end, and in between at most twice a second."""
    import time

    now = time.monotonic()
    if n == 0 or n >= total or now - _told[0] >= 0.5:
        _told[0] = now
        print(f"  {what} {n}/{total}", file=sys.stderr, flush=True)


def sort_one(path) -> bool:
    """Is the image of this file a drawing? (Run in several processes at
    once: looking at every pixel of a large image takes seconds.)"""
    return is_drawing(framed(Image.open(path).convert("RGB")))


def has_big_shape(img: Image.Image) -> bool:
    """One connected piece of ink that is large both ways: a curve, a box,
    an arrow across the picture. In math only a root sign over a tall
    formula grows like that, and all its ink is the bar along the top and
    the tick at the left."""
    g = img.convert("L")
    w, h = g.size
    if h < BIG_SHAPE or w < BIG_SHAPE:
        return False
    px = g.load()
    seen = bytearray(w * h)
    for y0 in range(h):
        for x0 in range(w):
            if seen[y0 * w + x0] or px[x0, y0] >= 140:
                continue
            seen[y0 * w + x0] = 1
            stack, pts = [(x0, y0)], []
            while stack:
                x, y = stack.pop()
                pts.append((x, y))
                for u in (x - 1, x, x + 1):
                    for v in (y - 1, y, y + 1):
                        if 0 <= u < w and 0 <= v < h and not seen[v * w + u] and px[u, v] < 140:
                            seen[v * w + u] = 1
                            stack.append((u, v))
            left, top = min(p[0] for p in pts), min(p[1] for p in pts)
            bw, bh = max(p[0] for p in pts) - left + 1, max(p[1] for p in pts) - top + 1
            if bw < BIG_SHAPE or bh < BIG_SHAPE:
                continue
            tick = max(16, min(0.4 * bh, 0.5 * bw))
            beyond = sum(1 for x, y in pts if x - left >= tick and y - top >= 6)
            if beyond > 0.1 * len(pts):
                return True
    return False


def is_drawing(img: Image.Image) -> bool:
    """A plot or diagram: one large connected shape, or axes or boxes (a
    long vertical and a long horizontal run of dark pixels that meet).
    Equations have long horizontal fraction bars, but no long straight
    vertical strokes ending in one."""
    if has_big_shape(img):
        return True
    g = img.convert("L")
    w, h = g.size
    if h < 100 or w < 100:
        return False
    px = g.load()
    dark = lambda x, y: px[x, y] < 140

    def runs_v():
        out = []
        for x in range(w):
            y = 0
            while y < h:
                if dark(x, y):
                    y0 = y
                    while y < h and dark(x, y):
                        y += 1
                    if y - y0 >= 0.4 * h:
                        out.append((x, y0, y - 1))
                else:
                    y += 1
        return out

    def run_h(y):
        out = []
        x = 0
        while x < w:
            if dark(x, y):
                x0 = x
                while x < w and dark(x, y):
                    x += 1
                if x - x0 >= 0.4 * w:
                    out.append((x0, x - 1))
            else:
                x += 1
        return out

    for x, y0, y1 in runs_v():
        for y in range(max(0, y0 - 3), min(h, y0 + 4)):
            if any(a - 3 <= x <= b + 3 for a, b in run_h(y)):
                return True
        for y in range(max(0, y1 - 3), min(h, y1 + 4)):
            if any(a - 3 <= x <= b + 3 for a, b in run_h(y)):
                return True
    return False


def clean(tex: str) -> str:
    t = tex.strip()
    for a, b in (("$$", "$$"), ("\\[", "\\]")):
        if t.startswith(a) and t.endswith(b) and len(t) > len(a) + len(b):
            t = t[len(a) : -len(b)].strip()
            break
    # Text with math in $…$ ("$S_y$ atoms"): the whole thing becomes one
    # formula, the words in \text{…}.
    parts, cur, k = [], "", 0
    while k < len(t):
        c = t[k]
        if c == "\\" and k + 1 < len(t):
            cur += t[k : k + 2]
            k += 2
            continue
        if c == "$":
            parts.append(cur)
            cur = ""
            k += 2 if t[k : k + 2] == "$$" else 1
            continue
        cur += c
        k += 1
    parts.append(cur)
    if len(parts) == 1:
        return t
    # an odd number of $, or "words" that are really math, mean the $ mark
    # nothing reliable: all math
    words = [p for i, p in enumerate(parts) if i % 2 == 0]
    if len(parts) % 2 == 0 or any(c in w for w in words for c in "\\_^{}"):
        return " ".join(p.strip() for p in parts if p.strip())
    out = []
    for i, p in enumerate(parts):
        if i % 2 == 1:
            out.append(p.strip())
        elif p.strip():
            lead = "\\ " if p[:1].isspace() and out else ""
            tail = "\\ " if p[-1:].isspace() and i + 1 < len(parts) else ""
            out.append(lead + "\\text{" + p.strip() + "}" + tail)
    return " ".join(out)


# Commands KaTeX does not know, with what it does know.
SUBSTITUTE = [
    (re.compile(r"\\mbox\b"), r"\\text"),
    (re.compile(r"\\boldmath\b"), r"\\boldsymbol"),
    (re.compile(r"\\begin\{tabular\}"), r"\\begin{array}"),
    (re.compile(r"\\end\{tabular\}"), r"\\end{array}"),
]
# Commands that mean the model went astray.
# (\\[ and \\] as display math, not the row spacing in "\\\\[4pt]"; # unless escaped)
NONSENSE = re.compile(
    r"\\(includegraphics|multicolumn|if|o)(?![a-zA-Z])|(?<!\\)\\[\[\]]|(?<!\\)#|[_^]\s*($|[}&]|\\\\)"
)
REPEAT = re.compile(r"(\\[a-zA-Z]+|\\.|[^\s])(\s*\1){7,}")


BIG = r"\\[bB]igg?[lrm]?"
DELIMS = set("()[]|./") | {
    "\\{", "\\}", "\\|", "\\langle", "\\rangle", "\\lfloor", "\\rfloor", "\\lceil", "\\rceil",
    "\\vert", "\\Vert", "\\lvert", "\\rvert", "\\lVert", "\\rVert", "\\uparrow", "\\downarrow",
    "\\updownarrow", "\\Uparrow", "\\Downarrow", "\\backslash", "\\lbrace", "\\rbrace",
    "\\lbrack", "\\rbrack", "\\langle", "<", ">",
}


def fix(t: str) -> str:
    # the equation's number is text beside the image: a \tag here is invented
    t = re.sub(r"\\(tag\*?|label)\{[^{}]*\}", "", t).strip()
    for pat, rep in SUBSTITUTE:
        t = pat.sub(rep, t)
    # \bigg{(} -> \bigg(  and  \bigg{ -> \bigg\{  (a size needs a delimiter)
    t = re.sub(r"(" + BIG + r")\s*\{\s*([()\[\]|.])\s*\}", r"\1\2", t)
    t = re.sub(r"(" + BIG + r")\s*\{", r"\1\\{", t)
    t = re.sub(r"(" + BIG + r")\s*\}", r"\1\\}", t)
    return t


def plausible(t: str) -> bool:
    """Cheap checks that catch most LaTeX a renderer would reject: balanced
    braces, matched \\left/\\right and \\begin/\\end, no runaway repetition
    (a model stuck emitting the same token until it hits its length limit)."""
    if not t.strip() or NONSENSE.search(t) or REPEAT.search(t):
        return False
    # a size (\\big, \\Bigl …) must be followed by a delimiter
    for m in re.finditer(BIG + r"(?![a-zA-Z])\s*(\\[a-zA-Z]+|\\.|.)?", t):
        nxt = m.group(1) or ""
        if nxt not in DELIMS:
            return False
    depth = 0
    k = 0
    while k < len(t):
        c = t[k]
        if c == "\\":
            k += 2
            continue
        if c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
            if depth < 0:
                return False
        k += 1
    if depth != 0:
        return False
    # every \\right closes an earlier \\left
    open_ = 0
    for m in re.finditer(r"\\(left|right)(?![a-zA-Z])", t):
        open_ += 1 if m.group(1) == "left" else -1
        if open_ < 0:
            return False
    if open_ != 0:
        return False
    stack = []
    for kind, name in re.findall(r"\\(begin|end)\{([^}]*)\}", t):
        if kind == "begin":
            stack.append(name)
        elif not stack or stack.pop() != name:
            return False
    return not stack


def rows(img: Image.Image, min_gap: int = 5, min_height: int = 10):
    """Cut a multi-line equation image into its lines, at blank horizontal
    bands at least `min_gap` pixels high (the gaps inside a fraction or
    around a subscript are only a pixel or two). Very thin pieces (a
    stray accent or dot) stay with the line above or below."""
    g = img.convert("L")
    w, h = g.size
    px = g.load()
    ink = [any(px[x, y] < 160 for x in range(w)) for y in range(h)]
    cuts, y = [], 0
    while y < h:
        if not ink[y]:
            y0 = y
            while y < h and not ink[y]:
                y += 1
            if y0 > 0 and y < h and y - y0 >= min_gap:
                cuts.append((y0 + y) // 2)
        else:
            y += 1
    bounds = [0] + cuts + [h]
    pieces = [(a, b) for a, b in zip(bounds, bounds[1:])]
    merged = []
    for a, b in pieces:
        if merged and (b - a < min_height or merged[-1][1] - merged[-1][0] < min_height):
            merged[-1] = (merged[-1][0], b)
        else:
            merged.append((a, b))
    return [img.crop((0, a, w, b)) for a, b in merged]


def line_breaks(t: str) -> int:
    return t.count("\\\\")


def run_together(img, t: str) -> bool:
    """An image of three or more lines read as far fewer lines: the model
    ran them together (it was trained on single equations)."""
    n = len(rows(img))
    return n >= 3 and line_breaks(t) + 1 < (n + 1) // 2


def read_one(img, model, processor, batch_inference):
    """The first plausible reading: greedy, then with a little randomness."""
    t = fix(clean(batch_inference([img], model, processor)[0]))
    if plausible(t):
        return t
    for temperature in (0.2, 0.4, 0.6):
        t = fix(clean(batch_inference([img], model, processor, temperature=temperature, max_tokens=768)[0]))
        if plausible(t):
            return t
    return None


def read_rows(img, model, processor, batch_inference):
    """Each line of a multi-line equation image, read by itself (None if
    the image is one line, or a line cannot be read)."""
    parts = rows(img)
    if len(parts) < 2:
        return None
    lines = []
    for part in parts:
        t = read_one(close_cut(part), model, processor, batch_inference)
        if t is None:
            return None
        lines.append(t)
    return lines


def close_cut(part):
    """A line of an equation cut close to its ink, with a margin of white:
    as wide as the picture it comes from, with nothing around it, the
    model reads a short line worse (words as plain text, a full stop as a
    comma)."""
    from PIL import ImageOps

    box = ImageOps.invert(part.convert("L")).point(lambda v: 255 if v > 95 else 0).getbbox()
    return part if box is None else ImageOps.expand(part.crop(box), border=16, fill="white")


def as_block(lines) -> str:
    return "\\begin{aligned}\n" + " \\\\\n".join("& " + t for t in lines) + "\n\\end{aligned}"


def read_by_rows(img, model, processor, batch_inference):
    """A tall equation the model cannot read whole (it was trained on
    single equations): read line by line, as one aligned block."""
    lines = read_rows(img, model, processor, batch_inference)
    return None if lines is None else as_block(lines)


# What a line of an equation goes on with: a relation or an operator (or the
# space the model sets for the indent of such a line).
RELATION = r"=|<|>|\\(?:approx|equiv|simeq|sim|leq|geq|le|ge|neq|ne|propto|cong|ll|gg)(?![a-zA-Z])"
CONTINUES = re.compile(RELATION + r"|\+|-|\\(?:times|pm|mp|cdot|quad|qquad)(?![a-zA-Z])")
RELATION = re.compile(RELATION)

FULL_LINE = 16  # pixels: a line of an equation is at least this high (the limits under a sum are 10 to 13)


def outside_braces(t: str):
    """Each place of `t` that is in no braces and in no command name."""
    depth, i = 0, 0
    while i < len(t):
        c = t[i]
        if c == "\\":
            m = re.match(r"\\([a-zA-Z]+|.)", t[i:], re.S)
            i += len(m.group(0))
            if m.group(1).isalpha():
                # the space that ends a command name before a letter is no
                # place of its own (before anything else the name ends
                # without it: such a space is one the model had no need for)
                j = i
                while j < len(t) and t[j] == " ":
                    j += 1
                if j < len(t) and t[j].isalpha():
                    i = j
                    if depth == 0:
                        yield i
                        i += 1
            continue
        if c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
        elif depth == 0:
            yield i
        i += 1


def line_spaces(t: str) -> list:
    """Where the model, reading several lines as one, went from one line to
    the next: it writes no spaces except the ones TeX needs, but leaves one
    there, before the relation or operator the next line starts with. (Of
    3,000 one-line equations of the first such book, one had such a
    space; of the equations of several lines read as one, two in three.)"""
    return [i for i in outside_braces(t) if t[i] == " " and i > 0 and CONTINUES.match(t, i + 1)]


def cut_lines(t: str, places) -> list:
    """`t` cut at `places` into lines that stand by themselves, or None. A
    bracket opened on one line and closed on another cannot grow with its
    content any more (\\left needs its \\right on the same line): such
    brackets get a fixed large size."""
    lines, start = [], 0
    for p in list(places) + [len(t)]:
        lines.append(re.sub(r"^(\\q?quad(?![a-zA-Z])\s*)+", "", t[start:p].strip()))
        start = p
    if not all(lines):
        return None
    if not all(plausible(x) for x in lines):
        lines = [re.sub(r"\\(left|right)(?![a-zA-Z])", lambda m: "\\Bigl" if m.group(1) == "left" else "\\Bigr", x) for x in lines]
        if not all(plausible(x) for x in lines):
            return None
    return lines


def rule_between(parts) -> bool:
    """A horizontal rule at the edge of a line, facing the next one: a
    fraction bar, and the two "lines" are numerator and denominator."""

    def ruled(part, top: bool) -> bool:
        g = part.convert("L")
        w, h = g.size
        px = g.load()
        seen = 0
        for y in range(h) if top else range(h - 1, -1, -1):
            run = best = 0
            for x in range(w):
                run = run + 1 if px[x, y] < 160 else 0
                best = max(best, run)
            if best >= 10:
                return True
            seen += best > 0
            if seen == 3:
                break
        return False

    return any(ruled(a, False) or ruled(b, True) for a, b in zip(parts, parts[1:]))


def lines_apart(img, t: str, model, processor, batch_inference):
    """An image of several lines read with fewer lines than it has: the
    model wrote two equations, or an equation and its continuation, as one
    long line. The lines are kept apart, by the first of these that holds:

    * the whole reading shows where its lines end (`line_spaces`): cut there;
    * the whole reading is plain words, and the lines read one by one are
      formulas with the same letters: the lines;
    * every line after the first starts with a relation, and the whole
      reading starts with the first: the lines;
    * read line by line, it says the same as the whole reading: the lines;
    * the whole reading says the same as the lines without one of them (the
      model left a line out): the lines (a low line must be missing from
      it altogether);
    * two lines, and the whole reading starts with what the first line reads
      as: cut where that ends (at a relation; else see `wrapped`).

    Otherwise the "lines" are no lines (the limits under a sum, the two
    halves of a fraction) or one of the readings is wrong, and the whole
    reading stays."""
    parts = rows(img)
    n = len(parts)
    if n < 2 or line_breaks(t) + 1 >= n:
        return None
    # (cut into lines here before, into fewer than the picture has rows:
    # the rows are no lines, see below)
    if t.startswith("\\begin{aligned}"):
        return None
    if "\\begin" not in t:
        # (a line set in may start with wide spaces and no plain one)
        spaced = re.sub(r"(?<! )((?:\\q?quad\s*)+)(?=[-+=]|\\times)", r" \1", t)
        places = line_spaces(spaced)
        if 1 <= len(places) <= n - 1:
            lines = cut_lines(spaced, places)
            if lines is not None:
                return as_block(lines)
    lines = read_rows(img, model, processor, batch_inference)
    if lines is None:
        return None
    whole = t.replace("\\\\", "")
    # Lines of words ("1 Joule = 1 Volt C"), read whole as plain text with
    # no formula in it: the lines as read one by one, if they have the
    # same letters.
    letters = lambda x: re.sub(r"\\[a-zA-Z]+|[^A-Za-z]", "", x).lower()
    if "\\" not in t and len(re.findall(r"[A-Za-z]{3,}", t)) >= 2 and all(p.height >= FULL_LINE for p in parts):
        if letters(t) == letters(" ".join(lines)) and all("\\" in l for l in lines):
            return as_block(lines)
    limit = max(2, len(canonical(whole)) // 10)
    if difference(whole, " ".join(lines)) <= limit:
        return as_block(lines)
    # Lines that each go on with a relation ("= ...", "= ..."), after a
    # first line that the whole reading starts with: the lines, whatever
    # the whole reading made of the rest (it can lose a line, or read a bar
    # with its limits as a table). The rows of a fraction, or the limits
    # under a sum, do not start with "=".
    starts = lambda l: RELATION.match(re.sub(r"^(\\q?quad(?![a-zA-Z])\s*)+", "", l.strip()))
    if all(p.height >= FULL_LINE for p in parts) and all(starts(l) for l in lines[1:]) and not starts(lines[0]):
        ends = [i for i in outside_braces(t) if i > 0 and RELATION.match(t, i)] + [len(t)]
        if min(difference(t[:i], lines[0]) for i in ends) <= max(2, 0.15 * len(canonical(lines[0]))):
            return as_block(lines)
    if "\\begin" in t or rule_between(parts):
        return None
    # A low line (a short remark in small type under the equation) that
    # the model left out altogether: the whole reading says just what the
    # other lines say, and the low one reads as three symbols or more.
    # (The limits under a sum are low, too, but the whole reading has them.)
    low = [j for j, p in enumerate(parts) if p.height < FULL_LINE]
    if low:
        others = " ".join(l for j, l in enumerate(lines) if j not in low)
        if all(len(canonical(lines[j])) >= 3 for j in low) and difference(whole, others) <= 1:
            for j in low:
                lines[j] = low_line(parts[j], model, processor, batch_inference) or lines[j]
            return as_block(lines)
        return None
    for j in range(n):
        if difference(whole, " ".join(lines[:j] + lines[j + 1 :])) <= limit:
            return as_block(lines)
    if n == 2:
        places = [i for i in outside_braces(t) if i > 0 and RELATION.match(t, i)]
        if places:
            cut = min(places, key=lambda i: difference(t[:i], lines[0]))
            if difference(t[:cut], lines[0]) <= max(2, len(canonical(lines[0])) // 10):
                cut_ = cut_lines(t, [cut])
                if cut_ is not None:
                    return as_block(cut_)
        return wrapped(t, lines)
    return None


TEXT_GROUP = ("text", "mathrm", "textrm", "textbf", "textit")


def cut_places(t: str):
    """Where a line of `t` can end: (place, in words). Before each piece
    that stands in no braces, and before each word of a run of words
    (\\text{...}) that stands in no other braces."""
    stack, i = [], 0
    while i < len(t):
        c = t[i]
        if i > 0 and not stack:
            yield i, False
        elif i > 0 and stack == [True] and t[i - 1] == " " and c != " ":
            yield i, True
        if c == "\\":
            m = re.match(r"\\([a-zA-Z]+|.)", t[i:], re.S)
            i += len(m.group(0))
            if m.group(1) in TEXT_GROUP and t[i : i + 1] == "{":
                stack.append(True)
                i += 1
            continue
        if c == "{":
            stack.append(False)
        elif c == "}" and stack:
            stack.pop()
        i += 1


def wrapped(t: str, lines):
    """Two lines the model read as one where no relation starts the second
    (a formula too long for its line, wrapped inside a bracket or inside
    its words; two sentences of words and symbols): the whole reading, cut
    where its letters and figures part as those of the two lines do. Both
    parts must have nearly the letters and figures of their line (the
    whole reading is the better one, so they need not be the same). A cut
    inside a run of words ends the run and starts it again."""
    import difflib

    alnum = lambda x: re.sub(r"\\[a-zA-Z]+|[^A-Za-z0-9]", "", x).lower()

    def apart(a: str, b: str) -> int:
        ops = difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes()
        return sum(max(i2 - i1, j2 - j1) for op, i1, i2, j1, j2 in ops if op != "equal")

    first, second = alnum(lines[0]), alnum(lines[1])
    if len(first) < 4 or len(second) < 4:
        return None
    best = None
    for place, in_words in cut_places(t):
        head, tail = alnum(t[:place]), alnum(t[place:])
        # (only where the first part is about as long as the first line)
        if abs(len(head) - len(first)) > 0.2 * len(first) + 2:
            continue
        score = (apart(head, first) + apart(tail, second), difference(t[:place], lines[0]) + difference(t[place:], lines[1]))
        if best is None or score < best[0]:
            best = (score, place, in_words, apart(head, first), apart(tail, second))
    if best is None:
        return None
    _, place, in_words, d1, d2 = best
    if d1 > max(2, 0.15 * len(first)) or d2 > max(2, 0.15 * len(second)):
        return None
    head, tail = t[:place].rstrip(), t[place:].lstrip()
    if in_words:
        head, tail = head + "}", "\\text{" + tail
    pieces = [bold_words(x) for x in (head, tail)]
    if not all(plausible(x) for x in pieces):
        return None
    return as_block(pieces)


def bold_words(t: str) -> str:
    """Without the "**" the model sets around some words of a run of words
    (as Markdown marks bold: it does so for words that are not bold, and
    for one of two that are set alike)."""
    return re.sub(r"\\text\{([^{}]*\*\*[^{}]*)\}", lambda m: "\\text{" + m.group(1).replace("**", "") + "}", t)


# NAME.lines holds the number of the rules the image was looked at with (keep
# in step with `lines_looked_at` in ocr.rs), or "never" if it is not to be.
def low_line(part, model, processor, batch_inference):
    """A line no higher than its small letters, read once more: cut close
    to its ink, with a margin (the model then tells a full stop from a
    comma), and with small letters where it reads capitals of the same
    shape (it cannot tell their size with nothing beside them, but a
    capital would stand higher than this line is)."""
    from PIL import ImageOps

    t = read_one(close_cut(part), model, processor, batch_inference)
    if t is None:
        return None
    # (single letters only: not the letters of a command's name)
    return re.sub(r"(?<![A-Za-z\\])[COSVWXZ](?![A-Za-z])", lambda m: m.group(0).lower(), t)


LINES_RULE = "6"


def lines_looked_at(dst, stem: str) -> bool:
    mark = dst / f"{stem}.lines"
    return mark.exists() and mark.read_text().strip() in (LINES_RULE, "never")


BATCH = 16  # images given to the model at once: it reports after each batch

SMALL = 30  # pixels wide; keep in step with SMALL in ocr.rs
SMALL_ONES = set()  # names of the images narrower than that


def framed(img: Image.Image) -> Image.Image:
    """A very small image (a single symbol like a hatted bold x, ten pixels
    wide) with white space around it: filling the whole picture, the model
    reads it as some other letter; with a margin it reads it right."""
    if img.width >= SMALL:
        return img
    from PIL import ImageOps

    return ImageOps.expand(img, border=16, fill="white")


def read_small_again(items, dst, model, processor, batch_inference):
    """Very small images read before they were given a margin (NAME.small
    marks those read with one): read again."""
    changed = 0
    for f, img in items:
        mark, known = dst / f"{f.stem}.small", dst / f"{f.stem}.tex"
        if f.stem not in SMALL_ONES or mark.exists() or not known.exists():
            continue
        old = known.read_text().strip()
        if not old.startswith("%"):
            t = read_one(img, model, processor, batch_inference)
            if t is not None and t != old:
                known.write_text(t + "\n")
                for ext in ("verify", "second"):
                    (dst / f"{f.stem}.{ext}").unlink(missing_ok=True)
                changed += 1
        mark.write_text("\n")
    if changed:
        print(f"  {changed} very small images (single symbols) read again with a margin: read differently now", file=sys.stderr, flush=True)


def split_lines(items, dst, model, processor, batch_inference):
    """Every equation not looked at before (NAME.lines marks those that
    were): lines read as one are taken apart, see `lines_apart`."""
    changed = 0
    items = [(f, img) for f, img in items if not lines_looked_at(dst, f.stem) and (dst / f"{f.stem}.tex").exists()]
    for n, (f, img) in enumerate(items):
        tell("lines", n, len(items))
        mark, known = dst / f"{f.stem}.lines", dst / f"{f.stem}.tex"
        t = known.read_text().strip()
        if not t.startswith("%"):
            block = lines_apart(img, t, model, processor, batch_inference)
            if block is not None:
                known.write_text(block + "\n")
                for ext in ("verify", "second"):
                    (dst / f"{f.stem}.{ext}").unlink(missing_ok=True)
                changed += 1
        mark.write_text(LINES_RULE + "\n")
    if items:
        tell("lines", len(items), len(items))
    if changed:
        print(f"  {changed} equations of several lines, read as one line before, have their lines again", file=sys.stderr, flush=True)


def word_labels(img: Image.Image):
    """Words set small under parts of a one-line equation ("bra" and "ket"
    under the two halves of a product), to explain them. The model drops
    them or reads them as symbols. Gives the equation without them and
    each word with the place of its middle, as a fraction of the
    equation's width -- or None if the image has no such words: the bottom
    band of ink must be small type under one line of larger type (not a
    denominator under its bar, nor the limits under a sum) and read as
    words by Tesseract."""
    import subprocess
    import tempfile

    g = img.convert("L")
    w, h = g.size
    px = g.load()
    ink = [[px[x, y] < 160 for x in range(w)] for y in range(h)]
    bands, y = [], 0
    while y < h:
        if any(ink[y]):
            y0 = y
            while y < h and any(ink[y]):
                y += 1
            bands.append((y0, y))
        else:
            y += 1
    if len(bands) != 2:
        return None
    (m0, m1), (l0, l1) = bands
    if not (5 <= l1 - l0 <= 13) or m1 - m0 < 14 or l1 - l0 > 0.8 * (m1 - m0):
        return None
    # a long straight stroke low in the equation: a fraction bar
    for y in range(m0 + (m1 - m0) * 2 // 3, m1):
        run = longest = 0
        for x in range(w):
            run = run + 1 if ink[y][x] else 0
            longest = max(longest, run)
        if longest >= 16:
            return None
    columns = [x for x in range(w) if any(ink[y][x] for y in range(m0, m1))]
    left, right = columns[0], columns[-1] + 1
    band = img.crop((0, max(0, l0 - 2), w, min(h, l1 + 2)))
    big = Image.new("RGB", (band.width * 4 + 40, band.height * 4 + 40), "white")
    big.paste(band.resize((band.width * 4, band.height * 4), Image.LANCZOS), (20, 20))
    with tempfile.NamedTemporaryFile(suffix=".png") as tmp:
        big.save(tmp.name)
        try:
            env = dict(os.environ, OMP_THREAD_LIMIT="1")
            out = subprocess.run(["tesseract", tmp.name, "-", "--psm", "7", "tsv"], capture_output=True, text=True, timeout=60, env=env)
        except (OSError, subprocess.TimeoutExpired):
            return None
    words = []
    for line in out.stdout.splitlines()[1:]:
        c = line.split("\t")
        if len(c) < 12 or not c[11].strip() or float(c[10]) < 0:
            continue
        middle = ((int(c[6]) - 20) + int(c[8]) / 2) / 4
        words.append(((middle - left) / (right - left), c[11].strip(), float(c[10])))
    # (a word in small letters: lettering in capitals is a logo)
    if not any(re.fullmatch(r"[A-Za-z][a-z]{2,}", t) and sure >= 80 for _, t, sure in words):
        return None
    return img.crop((0, 0, w, (m1 + l0) // 2)), [(at, t) for at, t, _ in words]


def label_words(items, dst, model, processor, batch_inference):
    """Every equation not looked at before (NAME.labels marks those that
    were, and holds the words): one with words under it is read again
    without them, and the words are kept with their places."""
    found = 0
    items = [(f, img) for f, img in items if not (dst / f"{f.stem}.labels").exists() and (dst / f"{f.stem}.tex").exists()]
    for n, (f, img) in enumerate(items):
        tell("labels", n, len(items))
        mark, known = dst / f"{f.stem}.labels", dst / f"{f.stem}.tex"
        t = known.read_text().strip()
        lines = ""
        # (a brace under the equation with words below it is read well as it is)
        if not t.startswith("%") and "\\underbrace" not in t:
            labelled = word_labels(img)
            if labelled is not None:
                t = read_one(framed(labelled[0]), model, processor, batch_inference)
                if t is not None:
                    known.write_text(t + "\n")
                    lines = "".join(f"{at:.3f}\t{word}\n" for at, word in labelled[1])
                    for ext in ("verify", "second"):
                        (dst / f"{f.stem}.{ext}").unlink(missing_ok=True)
                    found += 1
        mark.write_text(lines if lines else "\n")
    if items:
        tell("labels", len(items), len(items))
    if found:
        print(f"  {found} equations with words under them: read without the words, which are put back where they stood", file=sys.stderr, flush=True)


CODE_HINT = re.compile(r"```|\\texttt|\\verb")


def looks_like_code(t: str) -> bool:
    """Program text (Mathematica and the like): calls Name[...] together
    with assignments, rules, patterns or lists."""
    call = re.search(r"[A-Za-z]\w*\[", t) and re.search(r":=|->|\w_[\s,\]]|//|==|\{", t)
    # a result like {x -> 1.5, y -> 2} (a list of rules)
    rules = re.search(r"->", t) and re.search(r"\{", t)
    return bool(call or rules)


def surely_code(t: str) -> bool:
    """Unmistakable Mathematica: its named characters (\\[Psi], which
    Tesseract may write "\\ [Psi]"), a CamelCase function with square
    brackets (ClebschGordan[...]), or at least two weaker signs together
    (:=, /. replacement, a capitalized call Sqrt[...] with -> or a pattern
    x_). Strict, because it is tried on every image: read as text, typeset
    math can show any one of the weaker signs."""
    if re.search(r"\\\s?\[[A-Z][A-Za-z]+\]", t):
        return True
    # a CamelCase function applied with square brackets (ClebschGordan[...],
    # HermiteH[...]): built from whole words, so the first part has at least
    # four letters (a product of subscripted symbols read as text, "VnmVmi",
    # has short parts); typeset math has short lowercase functions (exp, sin)
    if re.search(r"\b[A-Z][a-z]{3,}[A-Z][A-Za-z]*[\s|]?\[", t):
        return True
    signs = [
        bool(re.search(r":=", t)),
        bool(re.search(r"\s/\.\s", t)),
        bool(re.search(r"\b[A-Z][a-z]+[A-Z]?\w*\[", t) and re.search(r"->|\w_[\s,\]]", t)),
    ]
    return sum(signs) >= 2


def ocr_text(img: Image.Image):
    """Plain-text OCR (Tesseract); None if Tesseract is missing."""
    import subprocess
    import tempfile

    g = img.convert("L")
    g = g.resize((g.width * 3, g.height * 3), Image.LANCZOS)
    with tempfile.NamedTemporaryFile(suffix=".png") as tmp:
        g.save(tmp.name)
        try:
            # one thread each: many images are read at once
            env = dict(os.environ, OMP_THREAD_LIMIT="1")
            out = subprocess.run(["tesseract", tmp.name, "-", "--psm", "6"], capture_output=True, text=True, timeout=60, env=env)
        except (OSError, subprocess.TimeoutExpired):
            return None
    text = "\n".join(line.rstrip() for line in out.stdout.strip().splitlines())
    # a thin "[" right after a name is sometimes read as "|[" or " ["
    return re.sub(r"([A-Za-z])[|]\[", r"\1[", text)


def read_code(img: Image.Image):
    """The text of an image of program code; None if Tesseract is missing
    or the text does not look like code."""
    text = ocr_text(img)
    return text if text and looks_like_code(text) else None


def lettering(text) -> bool:
    """Plain-text OCR that found words and nothing else (a logo, a name set
    as a picture): at least two words, making up most of what was read."""
    text = text or ""
    chars = len(re.sub(r"\s", "", text))
    words = re.findall(r"(?<!\S)[A-Za-z]{4,}(?!\S)", text)
    return len(words) >= 2 and sum(map(len, words)) >= 0.7 * chars


def settle(items, dst, texts):
    """After both readings: words that the math model read as two wholly
    different things (the readings differ by more than twice the length
    of the first) are lettering, not math, and stay a picture. Every other
    image is marked as passed (NAME.kind), so it is not tested again."""
    pictures = 0
    for f, img in items:
        tex = dst / f"{f.stem}.tex"
        if not tex.exists() or tex.read_text().startswith("%"):
            continue
        check = dst / f"{f.stem}.verify"
        m = re.match(r"check (\d+)", check.read_text()) if check.exists() else None
        if m and int(m.group(1)) >= 2 * max(len(canonical(tex.read_text())), 1):
            text = texts[f] if f in texts else ocr_text(img)
            if lettering(text):
                as_picture(dst, f)
                pictures += 1
                continue
        (dst / f"{f.stem}.kind").write_text("math\n")
    if pictures:
        print(f"  {pictures} images of words (logos and the like) kept as pictures", file=sys.stderr, flush=True)


def as_picture(dst, f):
    """An image read as math before that is a picture after all."""
    (dst / f"{f.stem}.tex").write_text("%DRAWING\n")
    for ext in ("verify", "second"):
        (dst / f"{f.stem}.{ext}").unlink(missing_ok=True)


def canonical(t: str) -> list:
    """LaTeX reduced to what changes the formula: no spacing commands,
    \\left/\\right or sizes, optional braces; one bold style; as tokens."""
    # (the equation's number, which one reading may have and the other not)
    t = re.sub(r"\\tag\{[^{}]*\}", "", t)
    t = re.sub(r"(\\quad|\\qquad|\s)+\(\s*[0-9A-Za-z.]{1,8}\s*\)\s*(\\end\{[a-z]+\}\s*)*$", lambda m: "".join(re.findall(r"\\end\{[a-z]+\}", m.group(0))), t.strip())
    # (commands that are the same thing, while the spaces still tell
    # where a command ends)
    t = re.sub(r"\\rightarrow(?![a-zA-Z])", r"\\to", t)
    t = re.sub(r"\\cal(?![a-zA-Z])", r"\\mathcal", t)
    t = re.sub(r"\\[td]frac(?![a-zA-Z])", r"\\frac", t)
    t = re.sub(r"\\iint(?![a-zA-Z])", r"\\int\\int", t)
    t = re.sub(r"\s+", "", t)
    t = re.sub(r"\\[,;:!]|\\quad|\\qquad|~|\\ ", "", t)
    t = re.sub(r"\\(left|right|big|Big|bigg|Bigg)[lrm]?(?=[^a-zA-Z])", "", t)
    t = re.sub(r"\\(displaystyle|textstyle|mathrm|operatorname|rm|text)", "", t)
    # two ways to write the same: a matrix in brackets, primes, arrows,
    # script letters, small fractions, a formula set in rows
    t = re.sub(r"\\begin\{array\}\{[lcr|]*\}", r"\\begin{array}", t)
    t = t.replace("\\begin{pmatrix}", "(\\begin{array}").replace("\\end{pmatrix}", "\\end{array})")
    t = t.replace("\\begin{bmatrix}", "[\\begin{array}").replace("\\end{bmatrix}", "\\end{array}]")
    t = re.sub(r"\\(begin|end)\{(aligned|split|gathered|align\*?|eqnarray\*?)\}", "", t)
    t = t.replace("\\\\", "").replace("&", "")
    t = re.sub(r"\^\{?((?:\\prime)+)\}?", lambda m: "'" * m.group(1).count("prime"), t)
    t = re.sub(r"\{([^{}\\])\}", r"\1", t)
    t = re.sub(r"\\(mathbf|boldsymbol|bf|mathbit|pmb)", r"\\B", t)
    t = t.replace("\\ldots", "\\dots").replace("\\cdots", "\\dots").replace("{", "").replace("}", "")
    return re.findall(r"\\[a-zA-Z]+|.", t)


def difference(a: str, b: str) -> int:
    """How many symbols two readings differ in (0: the same formula)."""
    import difflib

    ta, tb = canonical(a), canonical(b)
    if ta == tb:
        return 0
    ops = difflib.SequenceMatcher(None, ta, tb, autojunk=False).get_opcodes()
    return sum(max(i2 - i1, j2 - j1) for op, i1, i2, j1, j2 in ops if op != "equal")


def verify(items, dst, model, processor, batch_inference):
    """A second reading of each equation, from the image enlarged 2x (line by
    line again for one read line by line). Where the two readings are
    different formulas (formatting aside) the equation is worth checking
    against the book: NAME.verify says "check N" (N symbols differ), else
    "ok"; NAME.second keeps the second reading."""
    items = [
        (f, img)
        for f, img in items
        if not (dst / f"{f.stem}.tex").read_text().startswith("%") and not (dst / f"{f.stem}.verify").exists()
    ]
    if not items:
        return
    big = lambda img: img.resize((img.width * 2, img.height * 2), Image.LANCZOS)
    first = {f: (dst / f"{f.stem}.tex").read_text().strip() for f, _ in items}
    whole = [(f, img) for f, img in items if not first[f].startswith("\\begin{aligned}\n& ")]
    by_rows = [(f, img) for f, img in items if first[f].startswith("\\begin{aligned}\n& ")]
    second = {}
    tell("checking", 0, len(items))
    for i in range(0, len(whole), BATCH):
        part = whole[i : i + BATCH]
        for (f, _), t in zip(part, batch_inference([big(img) for _, img in part], model, processor)):
            second[f] = fix(clean(t))
        tell("checking", min(i + BATCH, len(whole)), len(items))
    for n, (f, img) in enumerate(by_rows):
        second[f] = read_by_rows(big(img), model, processor, batch_inference) or ""
        tell("checking", len(whole) + n + 1, len(items))
    flagged = 0
    for f, _ in items:
        n = difference(first[f], second.get(f, ""))
        flagged += n > 0
        (dst / f"{f.stem}.verify").write_text("ok\n" if n == 0 else f"check {n}\n")
        (dst / f"{f.stem}.second").write_text(second.get(f, "") + "\n")
    tell("checking", len(items), len(items))
    print(f"  {flagged} of {len(items)} equations read differently a second time: listed to check", file=sys.stderr, flush=True)


def main():
    src, dst = Path(sys.argv[1]), Path(sys.argv[2])
    files = sorted(src.glob("*.png"))
    todo = []
    verify_only = []  # read before: the second reading or the picture tests are missing
    redrawn = 0
    # drawings among them: looked for in several processes at once
    from concurrent.futures import ProcessPoolExecutor

    tell("sorting", 0, len(files))
    with ProcessPoolExecutor() as pool:
        drawings = []
        for drawing in pool.map(sort_one, files, chunksize=1 if len(files) < 2000 else 16):
            drawings.append(drawing)
            # (not all done before the images are loaded, below)
            tell("sorting", min(len(drawings), len(files) - 1), len(files))
    for f, drawing in zip(files, drawings):
        img = Image.open(f).convert("RGB")
        if img.width < SMALL:
            SMALL_ONES.add(f.stem)
        img = framed(img)
        known = dst / f"{f.stem}.tex"
        read = known.exists() and not known.read_text().startswith("%")
        if drawing:
            if read:
                as_picture(dst, f)
                redrawn += 1
            else:
                known.write_text("%DRAWING\n")
        elif read:
            verify_only.append((f, img))
        else:
            todo.append((f, img))
    tell("sorting", len(files), len(files))
    if redrawn:
        print(f"  {redrawn} drawings that were read as math before are pictures now", file=sys.stderr, flush=True)
    if not todo:
        unread_small = any(f.stem in SMALL_ONES and not (dst / f"{f.stem}.small").exists() for f, _ in verify_only)
        if unread_small or any(not (dst / f"{f.stem}.{ext}").exists() for f, _ in verify_only for ext in ("verify", "labels")) or any(not lines_looked_at(dst, f.stem) for f, _ in verify_only):
            from texify.inference import batch_inference
            from texify.model.model import load_model
            from texify.model.processor import load_processor

            tell("loading", 0, 1)
            model, processor = load_model(), load_processor()
            tell("loading", 1, 1)
            read_small_again(verify_only, dst, model, processor, batch_inference)
            split_lines(verify_only, dst, model, processor, batch_inference)
            label_words(verify_only, dst, model, processor, batch_inference)
            verify(verify_only, dst, model, processor, batch_inference)
        settle(verify_only, dst, {})
        return
    # program code shown as an image: every image is read as text too (in
    # parallel); unmistakable code is written as code, not given to texify
    from concurrent.futures import ThreadPoolExecutor

    tell("text", 0, len(todo))
    with ThreadPoolExecutor() as pool:
        texts = []
        for text in pool.map(lambda fi: ocr_text(fi[1]), todo):
            texts.append(text)
            tell("text", len(texts), len(todo))
    plain = {f: text for (f, _), text in zip(todo, texts)}
    rest = []
    for (f, img), text in zip(todo, texts):
        if text and surely_code(text):
            (dst / f"{f.stem}.tex").write_text("%CODE\n" + text + "\n")
        else:
            rest.append((f, img))
    todo = rest
    if not todo:
        settle(verify_only, dst, {})
        return
    from texify.inference import batch_inference
    from texify.model.model import load_model
    from texify.model.processor import load_processor

    tell("loading", 0, 1)
    model, processor = load_model(), load_processor()
    tell("loading", 1, 1)
    batch = BATCH
    doubtful = []
    tell("reading", 0, len(todo))
    for i in range(0, len(todo), batch):
        part = todo[i : i + batch]
        texts = batch_inference([img for _, img in part], model, processor)
        for (f, img), t in zip(part, texts):
            t = fix(clean(t))
            if plausible(t) and not CODE_HINT.search(t) and not run_together(img, t):
                (dst / f"{f.stem}.tex").write_text(t + "\n")
            else:
                doubtful.append((f, img))
        tell("reading", min(i + batch, len(todo)), len(todo))
    # Doubtful readings: try again with a longer limit and a little
    # randomness; an image that never reads plausibly stays a picture.
    for n, (f, img) in enumerate(doubtful):
        tell("again", n, len(doubtful))
        # program code, not math: as text
        code = read_code(img)
        if code is not None:
            (dst / f"{f.stem}.tex").write_text("%CODE\n" + code + "\n")
            continue
        # several lines read as one: line by line
        if len(rows(img)) >= 3:
            t = read_by_rows(img, model, processor, batch_inference)
            if t is not None:
                (dst / f"{f.stem}.tex").write_text(t + "\n")
                continue
        answer = "%UNREADABLE"
        for temperature in (0.2, 0.4, 0.6):
            t = fix(clean(batch_inference([img], model, processor, temperature=temperature, max_tokens=768)[0]))
            if plausible(t):
                answer = t
                break
        else:
            # several lines in one image: line by line
            t = read_by_rows(img, model, processor, batch_inference)
            if t is not None:
                answer = t
        (dst / f"{f.stem}.tex").write_text(answer + "\n")
    if doubtful:
        tell("again", len(doubtful), len(doubtful))
        unread = sum((dst / f"{f.stem}.tex").read_text().startswith("%UNREADABLE") for f, _ in doubtful)
        print(f"  {len(doubtful)} read again, {unread} unreadable (kept as pictures)", file=sys.stderr, flush=True)
    for f, _ in todo:
        if f.stem in SMALL_ONES:
            (dst / f"{f.stem}.small").write_text("\n")
    read_small_again(verify_only, dst, model, processor, batch_inference)
    split_lines(todo + verify_only, dst, model, processor, batch_inference)
    label_words(todo + verify_only, dst, model, processor, batch_inference)
    verify(todo + verify_only, dst, model, processor, batch_inference)
    settle(todo + verify_only, dst, plain)


if __name__ == "__main__":
    main()
