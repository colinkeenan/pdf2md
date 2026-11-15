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
# Which rules the choice was made by, on the second line of NAME.agree: a
# choice made by older rules is made again (keep in step with ocr.rs).
RULES = "rules 4"


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


# a sign the page reader left between its formulas: it leads the next one
SIGN = r"(?:[=+<>≈≃≡∝-]|\\(?:approx|simeq|equiv|propto|sim|le|ge|ne)(?![a-zA-Z]))"
TOKEN = re.compile(r"\\[a-zA-Z]+|\\.|.", re.S)


def group_at(t: str, k: int):
    """The group that begins at `k` (spaces before it aside): what is in
    its braces, or the one command or sign; and where it ends. None at the
    end of the formula, or where a brace is not closed."""
    while k < len(t) and t[k].isspace():
        k += 1
    if k >= len(t):
        return None
    if t[k] != "{":
        tok = TOKEN.match(t, k)
        return tok.group(0), tok.end()
    depth, j = 0, k
    while j < len(t):
        if t[j] == "\\":
            j += 2
            continue
        if t[j] == "{":
            depth += 1
        elif t[j] == "}":
            depth -= 1
            if depth == 0:
                return t[k + 1 : j], j + 1
        j += 1
    return None


def mended(t: str) -> str:
    """What the page reader's recognizer leaves open, closed where there is
    one way to: rows it began and did not end, an environment ended by
    another one's name, a bar "taken at" (\\right| with no \\left)."""
    stack, cuts = [], []
    for e in re.finditer(r"\\(begin|end)\{([^}]*)\}", t):
        if e.group(1) == "begin":
            stack.append(e.group(2))
        elif not stack:
            return t
        else:
            name = stack.pop()
            if name != e.group(2):
                cuts.append((e.start(2), e.end(2), name))
    for a, b, name in reversed(cuts):
        t = t[:a] + name + t[b:]
    if stack == ["aligned"] and t.startswith("\\begin{aligned}"):
        t += " \\end{aligned}"
    # (a group is what stands in braces, or in a cell of rows)
    frames, adds = [[0, 0]], []
    for tok in re.finditer(r"\\[a-zA-Z]+|\\.|[{}&]", t, re.S):
        w = tok.group(0)
        if w == "{":
            frames.append([tok.end(), 0])
        elif w == "}":
            if len(frames) == 1:
                return t
            adds.extend([(tok.start(), " \\right.")] * frames.pop()[1])
        elif w == "\\left":
            frames[-1][1] += 1
        elif w == "\\right":
            if frames[-1][1] > 0:
                frames[-1][1] -= 1
            else:
                adds.append((frames[-1][0], "\\left. "))
        elif w in ("&", "\\\\", "\\begin", "\\end") and frames[-1][1] == 0:
            at = tok.end()
            if w in ("\\begin", "\\end"):
                # (behind the environment's name, and the columns of an array)
                name = group_at(t, at)
                at = name[1] if name else at
                cols = group_at(t, at) if name and name[0] == "array" and w == "\\begin" else None
                at = cols[1] if cols else at
            frames[-1][0] = at
    if len(frames) == 1 and "\\\\" not in t:
        adds.extend([(len(t), " \\right.")] * frames[0][1])
    for at, what in sorted(adds, key=lambda x: -x[0]):
        t = t[:at] + what + t[at:]
    return t


def as_latex(raw: str):
    """What the page reader gave for a formula, as LaTeX; None where it is
    no formula (plain words, or marked up as text)."""
    raw = raw.strip()
    if not raw or "<math" not in raw:
        return None
    # (words set off as text, a struck or underlined piece: not its formula)
    if re.search(r"<(u|b|i|sub|sup|del)\b", raw):
        return None
    # Its lines; in a line, formulas with a few words between them ("… or
    # …", "… and …", "where …"), which are text in the formula. A sign left
    # between two formulas ("=" at the head of a line) leads the next one.
    lines, pieces, lead = [], [], ""

    def close():
        # (formulas with nothing between them are the lines of one)
        if any(p.startswith("\\text{") for p in pieces):
            lines.append(" \\quad ".join(pieces))
        else:
            lines.extend(pieces)
        pieces.clear()

    for part in re.split(r"(<math[^>]*>.*?</math>)", raw, flags=re.S):
        inner = re.fullmatch(r"<math[^>]*>(.*?)</math>", part, re.S)
        if inner:
            p = re.sub(r"<br\s*/?>|\s+", " ", inner.group(1))
            p = re.sub(r"\\label\{[^{}]*\}", "", p)
            p = p.replace("\\begin{split}", "\\begin{aligned}").replace("\\end{split}", "\\end{aligned}")
            p = re.sub(r"\\mbox(?![a-zA-Z])", r"\\text", p)
            p = mended(m.fix(m.clean(p.strip())))
            if p:
                pieces.append(lead + p)
                lead = ""
            continue
        for i, words in enumerate(re.split(r"<br\s*/?>|\n", part)):
            if i > 0:
                close()
            words = words.strip()
            if not words:
                continue
            if re.fullmatch(r"[.,;:]+", words):
                if pieces:
                    pieces[-1] += words
                continue
            sign = re.fullmatch(r"(.*?)\s*(" + SIGN + r")", words, re.S)
            if sign:
                words = sign.group(1).strip()
            text = ""
            if words:
                # (a sentence, or figures outside a formula: not one formula)
                if len(words.split()) > 5 or not re.fullmatch(r"[A-Za-z ,.;:()\"-]+", words) or words.count('"') % 2:
                    return None
                text = "\\text{" + words + "}"
            if sign:
                lead += (text + " " if text else "") + sign.group(2) + " "
            else:
                pieces.append(text)
    close()
    if not lines or lead:
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
    (r"\\(overset|mathop|limits|nolimits|mathsf|mathit|mathtt|textrm|textit|textsf)", ""),
    (r"\\dagger", "†"),
    (r"\\cdot", "·"),
    (r"\\(vert|mid)", "|"),
    (r"\\lbrack", "["),
    (r"\\rbrack", "]"),
    (r"\\(lim|max|min|sin|cos|tan|exp|log|ln|det|sup|inf|arg|dim|sinh|cosh|tanh|cot|sec|csc)", r"\1"),
]
BOLD = r"\\(mathbf|boldsymbol|bf|mathbit|pmb)(?![a-zA-Z])"


def labels_beside(t: str) -> str:
    """A label set over or under a sign, as an exponent or index of it:
    \\stackrel{a}{b} and \\overset{a}{b} are b^{a}, \\underset{a}{b} is
    b_{a}, \\xrightarrow[a]{b} is \\to_{a}^{b}."""
    found = list(re.finditer(r"\\(stackrel|overset|underset|xrightarrow)(?![a-zA-Z])", t))
    # (from the last: one inside another's label comes first)
    for f in reversed(found):
        k, under = f.end(), None
        if f.group(1) == "xrightarrow":
            low = re.compile(r"\s*\[([^\[\]]*)\]").match(t, k)
            if low:
                under, k = low.group(1), low.end()
            over = group_at(t, k)
            if over is None:
                continue
            new = "\\to" + ("_{" + under + "}" if under else "") + ("^{" + over[0] + "}" if over[0].strip() else "")
            t = t[: f.start()] + new + " " + t[over[1] :]
            continue
        label = group_at(t, k)
        base = group_at(t, label[1]) if label else None
        if base is None:
            continue
        t = t[: f.start()] + "{" + base[0] + "}" + ("_" if f.group(1) == "underset" else "^") + "{" + label[0] + "}" + t[base[1] :]
    return t


def loose(t: str) -> str:
    """A reading without its bold and without the ways of writing that
    differ between recognizers: letters and signs in order. A prime and an
    index on one letter in either order; "lim", "max" as words or commands;
    a label over an arrow as its exponent; rows in an array or not."""
    t = re.sub(BOLD, "", t).replace('"', "")
    t = labels_beside(t)
    # (a matrix in its brackets, cases behind their brace; an array around
    # rows, with its columns: rows all the same)
    for name, (a, b) in {"pmatrix": "()", "bmatrix": "[]", "vmatrix": "||", "Bmatrix": ("\\{", "\\}"), "cases": ("\\{", ""), "matrix": ("", "")}.items():
        t = t.replace("\\begin{" + name + "}", a + "\\begin{array}").replace("\\end{" + name + "}", "\\end{array}" + b)
    for f in reversed(list(re.finditer(r"\\begin\{array\}", t))):
        cols = group_at(t, f.end())
        t = t[: f.start()] + " " + t[cols[1] if cols and t[f.end() :].lstrip().startswith("{") else f.end() :]
    t = t.replace("\\end{array}", " ")
    t = re.sub(r"\\not\s*\\in(?![a-zA-Z])", r"\\notin", t)
    t = re.sub(r"\\not\s*=", r"\\ne ", t)
    # (primes and an exponent in one group: "x^{\prime 2}" is "x'^{2}")
    t = re.sub(r"\^\s*\{\s*((?:\\prime\s*)+)([^{}]*[^{}\s][^{}]*)\}", lambda f: "'" * f.group(1).count("prime") + "^{" + f.group(2) + "}", t)
    # (a long arrow and a short one)
    t = re.sub(r"\\long(?=(left|right|maps)[a-z]*(?![a-zA-Z]))", r"\\", t)
    t = re.sub(r"\\Long(left|right)(?=[a-z]*arrow(?![a-zA-Z]))", lambda f: "\\" + f.group(1).capitalize(), t)
    # (an exponent before an index, or after it: the same)
    group = r"(\{[^{}]*\}|\\[a-zA-Z]+|[^\\{}\s])"
    t = re.sub(r"\^\s*" + group + r"\s*_\s*" + group, r"_\2^\1", t)
    for a, b in SAME:
        # (a command's name ends where its letters end)
        t = re.sub(a + (r"(?![a-zA-Z])" if a[-1].isalpha() or (a.endswith(")") and not a.endswith("$)")) else ""), b, t)
    s = "".join(tok for tok in m.canonical(t) if tok not in ".,;")
    # ("x'_i" and "x_i'": the same)
    s = s.replace("\\prime", "'")
    s = re.sub(r"('+)(_[A-Za-z0-9]|_\\[a-zA-Z]+)", r"\2\1", s)
    return s.replace("*", "")


def primed_sums(t: str):
    """For each sum sign of a reading: where it stands with its limits, and
    whether it has a prime (a sum that leaves terms out)."""
    out = []
    for f in re.finditer(r"\\sum(?![a-zA-Z])", t):
        k, primed = f.end(), False
        while True:
            # (the prime may hang on an empty group behind the limits)
            step = re.compile(r"\s*(?:\\(?:no)?limits)?\s*(?:\{\s*\}\s*(?=\^|'))?([_^'])").match(t, k)
            if not step:
                break
            if step.group(1) == "'":
                primed, k = True, step.end()
                continue
            limit = group_at(t, step.end())
            if limit is None:
                break
            if step.group(1) == "^" and re.fullmatch(r"\s*(\\prime\s*|')+", limit[0]):
                primed = True
            k = limit[1]
        # (or on the group the sum stands in with its limits)
        if t[: f.start()].rstrip().endswith("{") and re.match(r"\s*\}\s*'", t[k:]):
            primed = True
        out.append((f.start(), k, primed))
    return out


def with_sum_primes(first: str, second: str, other: str) -> str:
    """The other recognizer drops the prime of a sum sign, each time it
    reads the picture. Where the math recognizer read one twice, on the
    same sum of as many, the other reading's sum has it too."""
    mine, theirs = primed_sums(first), primed_sums(other)
    marks = [p for _, _, p in mine]
    if not any(marks) or len(theirs) != len(mine) or [p for _, _, p in primed_sums(second)] != marks:
        return other
    for (a, b, has), want in reversed(list(zip(theirs, marks))):
        if want and not has:
            other = other[:a] + "{" + other[a:b] + "}'" + other[b:]
    return other


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
            other = with_sum_primes(first, second, other)
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
            if larger is not None:
                larger = with_sum_primes(first, second, larger)
            if larger is not None and loose(larger) == loose(other):
                n = 0
            elif larger is not None:
                # (its two readings differ: one of them may be what the
                # math recognizer read, and then that is the formula)
                again = choose(first, second, larger)
                if again[1] == 0:
                    best, n = again
        target = dst / f"{f.stem}.best"
        if best is None:
            if target.exists():
                target.unlink()
        else:
            target.write_text(best + "\n")
            changed += 1
        (dst / f"{f.stem}.agree").write_text(("ok\n" if n == 0 else f"check {n}\n") + RULES + "\n")
        doubtful += n > 0
    print(f"  {changed} of {len(math)} equations are written as the page reader's recognizer read them; {doubtful} are listed to check (no two readings agree)", file=sys.stderr, flush=True)


if __name__ == "__main__":
    main()
