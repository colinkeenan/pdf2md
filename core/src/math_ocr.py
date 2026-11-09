"""Read equation images as LaTeX, for pdf2md (see ocr.rs).

Usage: python math_ocr.py <folder of NAME.png> <output folder>

Writes <output folder>/NAME.tex for every image: the LaTeX of the equation,
"%CODE" and the text for program code (read with Tesseract),
or "%DRAWING" for a plot or diagram (an image with long straight lines
meeting at a corner, like the axes of a graph). Runs texify
(https://github.com/VikParuchuri/texify) on the GPU if there is one.
"""

import os
import re
import sys
from pathlib import Path

from PIL import Image


def is_drawing(img: Image.Image) -> bool:
    """Axes or boxes: a long vertical and a long horizontal run of dark
    pixels that meet. Equations have long horizontal fraction bars, but no
    long straight vertical strokes ending in one."""
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


def read_by_rows(img, model, processor, batch_inference):
    """A tall equation the model cannot read whole (it was trained on
    single equations): read line by line, as one aligned block."""
    parts = rows(img)
    if len(parts) < 2:
        return None
    lines = []
    for part in parts:
        t = read_one(part, model, processor, batch_inference)
        if t is None:
            return None
        lines.append("& " + t)
    return "\\begin{aligned}\n" + " \\\\\n".join(lines) + "\n\\end{aligned}"


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


def canonical(t: str) -> list:
    """LaTeX reduced to what changes the formula: no spacing commands,
    \\left/\\right or sizes, optional braces; one bold style; as tokens."""
    t = re.sub(r"\s+", "", t)
    t = re.sub(r"\\[,;:!]|\\quad|\\qquad|~|\\ ", "", t)
    t = re.sub(r"\\(left|right|big|Big|bigg|Bigg)[lrm]?(?=[^a-zA-Z])", "", t)
    t = re.sub(r"\\(displaystyle|textstyle|mathrm|operatorname|rm)", "", t)
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
    items = [(f, img) for f, img in items if not (dst / f"{f.stem}.tex").read_text().startswith("%")]
    if not items:
        return
    big = lambda img: img.resize((img.width * 2, img.height * 2), Image.LANCZOS)
    first = {f: (dst / f"{f.stem}.tex").read_text().strip() for f, _ in items}
    whole = [(f, img) for f, img in items if not first[f].startswith("\\begin{aligned}\n& ")]
    by_rows = [(f, img) for f, img in items if first[f].startswith("\\begin{aligned}\n& ")]
    second = {}
    for i in range(0, len(whole), 16):
        part = whole[i : i + 16]
        for (f, _), t in zip(part, batch_inference([big(img) for _, img in part], model, processor)):
            second[f] = fix(clean(t))
        print(f"  checking {min(i + 16, len(whole))}/{len(whole)}", file=sys.stderr, flush=True)
    for f, img in by_rows:
        second[f] = read_by_rows(big(img), model, processor, batch_inference) or ""
    flagged = 0
    for f, _ in items:
        n = difference(first[f], second.get(f, ""))
        flagged += n > 0
        (dst / f"{f.stem}.verify").write_text("ok\n" if n == 0 else f"check {n}\n")
        (dst / f"{f.stem}.second").write_text(second.get(f, "") + "\n")
    print(f"  {flagged} of {len(items)} equations read differently a second time: listed to check", file=sys.stderr, flush=True)


def main():
    src, dst = Path(sys.argv[1]), Path(sys.argv[2])
    files = sorted(src.glob("*.png"))
    todo = []
    verify_only = []  # read before: only the second reading is missing
    for f in files:
        img = Image.open(f).convert("RGB")
        known = dst / f"{f.stem}.tex"
        if known.exists() and not known.read_text().startswith("%"):
            verify_only.append((f, img))
        elif is_drawing(img):
            (dst / f"{f.stem}.tex").write_text("%DRAWING\n")
        else:
            todo.append((f, img))
    if not todo and verify_only:
        from texify.inference import batch_inference
        from texify.model.model import load_model
        from texify.model.processor import load_processor

        verify(verify_only, dst, load_model(), load_processor(), batch_inference)
        return
    # program code shown as an image: every image is read as text too (in
    # parallel); unmistakable code is written as code, not given to texify
    from concurrent.futures import ThreadPoolExecutor

    with ThreadPoolExecutor() as pool:
        texts = list(pool.map(lambda fi: ocr_text(fi[1]), todo))
    rest = []
    for (f, img), text in zip(todo, texts):
        if text and surely_code(text):
            (dst / f"{f.stem}.tex").write_text("%CODE\n" + text + "\n")
        else:
            rest.append((f, img))
    todo = rest
    if not todo:
        return
    from texify.inference import batch_inference
    from texify.model.model import load_model
    from texify.model.processor import load_processor

    model, processor = load_model(), load_processor()
    batch = 16
    doubtful = []
    for i in range(0, len(todo), batch):
        part = todo[i : i + batch]
        texts = batch_inference([img for _, img in part], model, processor)
        for (f, img), t in zip(part, texts):
            t = fix(clean(t))
            if plausible(t) and not CODE_HINT.search(t) and not run_together(img, t):
                (dst / f"{f.stem}.tex").write_text(t + "\n")
            else:
                doubtful.append((f, img))
        print(f"  {min(i + batch, len(todo))}/{len(todo)}", file=sys.stderr, flush=True)
    # Doubtful readings: try again with a longer limit and a little
    # randomness; an image that never reads plausibly stays a picture.
    for f, img in doubtful:
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
        unread = sum((dst / f"{f.stem}.tex").read_text().startswith("%UNREADABLE") for f, _ in doubtful)
        print(f"  {len(doubtful)} read again, {unread} unreadable (kept as pictures)", file=sys.stderr, flush=True)
    verify(todo + verify_only, dst, model, processor, batch_inference)


if __name__ == "__main__":
    main()
