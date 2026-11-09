"""Read equation images as LaTeX, for pdf2md (see ocr.rs).

Usage: python math_ocr.py <folder of NAME.png> <output folder>

Writes <output folder>/NAME.tex for every image: the LaTeX of the equation,
or "%DRAWING" for a plot or diagram (an image with long straight lines
meeting at a corner, like the axes of a graph). Runs texify
(https://github.com/VikParuchuri/texify) on the GPU if there is one.
"""

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


def fix(t: str) -> str:
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


def main():
    src, dst = Path(sys.argv[1]), Path(sys.argv[2])
    files = sorted(src.glob("*.png"))
    todo = []
    for f in files:
        img = Image.open(f).convert("RGB")
        if is_drawing(img):
            (dst / f"{f.stem}.tex").write_text("%DRAWING\n")
        else:
            todo.append((f, img))
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
            if plausible(t):
                (dst / f"{f.stem}.tex").write_text(t + "\n")
            else:
                doubtful.append((f, img))
        print(f"  {min(i + batch, len(todo))}/{len(todo)}", file=sys.stderr, flush=True)
    # Doubtful readings: try again with a longer limit and a little
    # randomness; an image that never reads plausibly stays a picture.
    for f, img in doubtful:
        answer = "%UNREADABLE"
        for temperature in (0.2, 0.4, 0.6):
            t = fix(clean(batch_inference([img], model, processor, temperature=temperature, max_tokens=768)[0]))
            if plausible(t):
                answer = t
                break
        (dst / f"{f.stem}.tex").write_text(answer + "\n")
    if doubtful:
        unread = sum((dst / f"{f.stem}.tex").read_text().startswith("%UNREADABLE") for f, _ in doubtful)
        print(f"  {len(doubtful)} read again, {unread} unreadable (kept as pictures)", file=sys.stderr, flush=True)


if __name__ == "__main__":
    main()
