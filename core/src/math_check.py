#!/usr/bin/env python3
"""A second opinion on equation pictures, from another recognizer.

    math_check.py PICTURES CACHE

PICTURES holds NAME.png, CACHE the readings math_ocr.py made of them
(NAME.tex, NAME.second). Each picture is read once more by the recognizer
of the page reader (surya, in the environment of scanned books), which
errs in other places than the math recognizer does:

    NAME.surya   what it read, as it gave it (kept: the choice below is
                 made anew each time, from the three readings)
    NAME.best    the reading to write, where it is not NAME.tex
    NAME.surya3  its second reading, of the picture enlarged more, where
                 the math recognizer did not bear the first out
    NAME.agree   "ok", or "check N": no two readings are the same
                 formula, and N symbols differ

The choice: where the two recognizers read the same formula, bold letters
and ways of writing aside, the reading with more bold letters (each
misses some; neither invents them). Otherwise the other recognizer's
reading: on symbols it is right far more often (l for I, a prime, a nabla
read as a bold V). Such an equation is listed to check unless the math
recognizer's second reading says the same, or the other recognizer reads
the same formula again from the picture enlarged more.
"""

import os
import re
import sys
from pathlib import Path

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import math_ocr as m  # noqa: E402  (canonical, difference, clean, fix, plausible, bold_by_picture)

from PIL import Image  # noqa: E402

BATCH = 32


def tell(what: str, n: int, total: int):
    print(f"  {what} {n}/{total}", file=sys.stderr, flush=True)


def prepared(path, scale: int = 2) -> Image.Image:
    """The picture enlarged on a white ground: as the recognizer reads a
    block of a page."""
    img = Image.open(path).convert("RGB")
    img = img.resize((img.width * scale, img.height * scale), Image.LANCZOS)
    out = Image.new("RGB", (img.width + 40, img.height + 40), "white")
    out.paste(img, (20, 20))
    return out


def as_latex(raw: str):
    """What the page reader gave for a formula, as LaTeX; None where it is
    no formula (plain words, or marked up as text)."""
    raw = raw.strip()
    if not raw or "<math" not in raw:
        return None
    # (words set off as text, a struck or underlined piece: not its formula)
    if re.search(r"<(u|b|i|sub|sup|del)\b", raw):
        return None
    parts = re.findall(r"<math[^>]*>(.*?)</math>", raw, re.S)
    between = re.sub(r"<math[^>]*>.*?</math>", " ", raw, flags=re.S)
    between = re.sub(r"<br\s*/?>", " ", between)
    # (only punctuation may stand between the lines of one formula)
    if re.search(r"[A-Za-z0-9]", between):
        return None
    lines = []
    for p in parts:
        p = re.sub(r"\\label\{[^{}]*\}", "", p)
        p = p.replace("\\begin{split}", "\\begin{aligned}").replace("\\end{split}", "\\end{aligned}")
        p = re.sub(r"\\mbox(?![a-zA-Z])", r"\\text", p)
        p = m.fix(m.clean(p.strip()))
        if p:
            lines.append(p)
    if not lines:
        return None
    t = lines[0] if len(lines) == 1 else m.as_block(lines)
    t = rows_closed(t)
    return t if m.plausible(t) else None


def rows_closed(t: str) -> str:
    """A bracket sized with \\left that closes in another row of the
    formula (or another cell) cannot be typeset: such a formula's brackets
    are written plain, at their own size."""
    if "\\\\" not in t and "&" not in t:
        return t
    # (rows inside rows, a matrix or cases in a line: left as they are)
    if re.search(r"\\begin\{(array|cases|[a-zA-Z]*matrix)\}", t):
        return t
    for row in re.split(r"\\\\|&", t):
        if len(re.findall(r"\\left(?![a-zA-Z])", row)) != len(re.findall(r"\\right(?![a-zA-Z])", row)):
            t = re.sub(r"\\(left|right)\s*\.", "", t)
            return re.sub(r"\\(left|right)(?![a-zA-Z])", "", t)
    return t


# ways of writing one thing, on top of what `canonical` takes for the same
SAME = [
    (r"\\mathscr", r"\\mathcal"),
    (r"\\star", "*"),
    (r"\\ast", "*"),
    (r"\\geqq?", r"\\ge"),
    (r"\\leqq?", r"\\le"),
    (r"\\neq", r"\\ne"),
    (r"\\varepsilon", r"\\epsilon"),
    (r"\\varphi", r"\\phi"),
    (r"\\xrightarrow", r"\\to"),
    (r"\\longrightarrow", r"\\to"),
    (r"\\(overset|mathop|limits|nolimits)", ""),
    (r"\\(begin|end)\{(array|cases|matrix)\}(\{[lcr|]*\})?(?=.|$)", ""),
    (r"\\dagger", "†"),
    (r"\\cdot", "·"),
    (r"\\(vert|mid)", "|"),
    (r"\\lbrack", "["),
    (r"\\rbrack", "]"),
    (r"\\(lim|max|min|sin|cos|tan|exp|log|ln|det|sup|inf|arg|dim|sinh|cosh|tanh|cot|sec|csc)", r"\1"),
]
BOLD = r"\\(mathbf|boldsymbol|bf|mathbit|pmb)(?![a-zA-Z])"


def loose(t: str) -> str:
    """A reading without its bold and without the ways of writing that
    differ between recognizers: letters and signs in order. A prime and an
    index on one letter in either order; "lim", "max" as words or commands."""
    t = re.sub(BOLD, "", t)
    # (an exponent before an index, or after it: the same)
    group = r"(\{[^{}]*\}|\\[a-zA-Z]+|[^\\{}\s])"
    t = re.sub(r"\^\s*" + group + r"\s*_\s*" + group, r"_\2^\1", t)
    for a, b in SAME:
        # (a command's name ends where its letters end)
        t = re.sub(a + (r"(?![a-zA-Z])" if a[-1].isalpha() or (a.endswith(")") and not a.endswith("$)")) else ""), b, t)
    s = "".join(tok for tok in m.canonical(t) if tok not in ".,;")
    # ("x'_i" and "x_i'": the same)
    s = re.sub(r"('+)(_[A-Za-z0-9]|_\\[a-zA-Z]+)", r"\2\1", s)
    return s.replace("*", "")


def bolds(t: str) -> int:
    return len(re.findall(BOLD, t))


def choose(first: str, second: str, other):
    """(the reading to write, or None for the first; how many symbols are in
    doubt: 0 when two recognizers agree)."""
    if other is None:
        n = m.difference(first, second) if second else 0
        return None, n
    lo = loose(other)
    if loose(first) == lo:
        return (other if bolds(other) > bolds(first) else None), 0
    if second and loose(second) == lo:
        return (other if bolds(other) >= bolds(second) else second), 0
    return other, max(1, m.difference(first, other))


def read(files, dst, ext: str, scale: int, state: dict):
    """Read the pictures with the page reader's recognizer, enlarged
    `scale` times; what it gives is kept as NAME.`ext`."""
    if not files:
        return
    if "rec" not in state:
        from surya.foundation import FoundationPredictor
        from surya.recognition import RecognitionPredictor

        tell("loading", 0, 1)
        state["rec"] = RecognitionPredictor(FoundationPredictor())
        state["rec"].disable_tqdm = True
        tell("loading", 1, 1)
    rec = state["rec"]
    tell("comparing", 0, len(files))
    for i in range(0, len(files), BATCH):
        part = files[i : i + BATCH]
        imgs = [prepared(f, scale) for f in part]
        preds = rec(
            images=imgs,
            bboxes=[[[0, 0, im.width, im.height]] for im in imgs],
            task_names=["block_without_boxes"] * len(imgs),
            math_mode=True,
            max_tokens=1024,
        )
        for f, p in zip(part, preds):
            (dst / f"{f.stem}.{ext}").write_text(" ".join(line.text for line in p.text_lines) + "\n")
        tell("comparing", min(i + BATCH, len(files)), len(files))


def main():
    src, dst = Path(sys.argv[1]), Path(sys.argv[2])
    files = sorted(src.glob("*.png"))
    math = []
    for f in files:
        tex = dst / f"{f.stem}.tex"
        if tex.exists() and not tex.read_text().startswith("%"):
            math.append(f)
    state = {}
    read([f for f in math if not (dst / f"{f.stem}.surya").exists()], dst, "surya", 2, state)

    def readings(f):
        first = (dst / f"{f.stem}.tex").read_text().strip()
        s = dst / f"{f.stem}.second"
        second = s.read_text().strip() if s.exists() else ""
        other = as_latex((dst / f"{f.stem}.surya").read_text())
        if other is not None:
            # (the picture has no bold letter: nor has this reading of it)
            other = m.bold_by_picture(Image.open(f), other)[0]
        return first, second, other

    # Where the math recognizer does not bear the other one out, the other
    # one reads the picture again, larger: reading the same formula twice
    # is as good a sign (9 of 9 right in a sample checked by eye).
    chosen = {f: (readings(f), None) for f in math}
    again = []
    for f in math:
        (first, second, other), _ = chosen[f]
        if other is not None and choose(first, second, other)[1] > 0 and not (dst / f"{f.stem}.surya3").exists():
            again.append(f)
    read(again, dst, "surya3", 3, state)
    changed = doubtful = 0
    for f in math:
        (first, second, other), _ = chosen[f]
        best, n = choose(first, second, other)
        third = dst / f"{f.stem}.surya3"
        if n > 0 and other is not None and third.exists():
            larger = as_latex(third.read_text())
            if larger is not None and loose(larger) == loose(other):
                n = 0
        target = dst / f"{f.stem}.best"
        if best is None:
            if target.exists():
                target.unlink()
        else:
            target.write_text(best + "\n")
            changed += 1
        (dst / f"{f.stem}.agree").write_text("ok\n" if n == 0 else f"check {n}\n")
        doubtful += n > 0
    print(f"  {changed} of {len(math)} equations are written as the page reader's recognizer read them; {doubtful} are listed to check (no two readings agree)", file=sys.stderr, flush=True)


if __name__ == "__main__":
    main()
