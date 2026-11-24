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
and ways of writing aside, the reading whose matrices are whole (no empty
row, rows of one length), then the one with more bold letters (each
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
RULES = "rules 5"


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


# commands that take what stands behind them
TAKES = r"\\(?:vec|hat|bar|tilde|dot|ddot|[td]?frac|sqrt|overline|underline|widehat|widetilde|mathbf|mathrm|mathcal|mathbb|mathit|mathsf|mathscr|mathfrak|boldsymbol|bm|text|left|right|[bB]igg?[lrm]?|not|operatorname|overrightarrow|overleftarrow|stackrel|overset|underset|binom|begin|end)"


def on_and_on(t: str) -> bool:
    """A reading that ends in the same thing written over and over: the
    recognizer astray, to its limit."""
    t = t.rstrip()
    if m.REPEAT.search(t):
        return True
    # (… or a few things in turn, four times and more, up to its end)
    turn = re.search(r"(\S.{5,79}?)(?:\s*\1){3,}.{0,80}$", t[-800:], re.S)
    return turn is not None


def finished(t: str) -> str:
    """A reading that the recognizer broke off, closed so that it can be
    typeset: without the command it ended on (one that takes what comes
    behind it, an exponent's sign, a lone backslash), its braces and its
    rows closed. Where it stopped in the middle of a group or of rows,
    dots stand for what is missing. A fraction that ends the formula with
    its upper half alone is no fraction: the recognizer's way of ending
    a row of formulas set beside each other."""
    if on_and_on(t):
        return t
    # (two indices on one letter: the second on its own)
    t = re.sub(r"(?<!\\)([_^])(\s*(?:\{(?:[^{}]|\{[^{}]*\})*\}|[A-Za-z0-9]))\s*(?=\1)", r"\1\2{}", t)
    whole = t.rstrip()
    while True:
        t = t.rstrip()
        slashes = len(t) - len(t.rstrip("\\"))
        end = re.search(TAKES + r"$|\\(begin|end)\s*\{[^{}]*$|(?<!\\)[\^_]$", t)
        if slashes % 2:
            t = t[:-1]
        elif end:
            t = t[: end.start()]
        else:
            break
    # (… a lone backslash is a space it set at the end: nothing is missing)
    cut = len(t) < len(whole) - 1
    depth, k = 0, 0
    while k < len(t):
        if t[k] == "\\":
            k += 2
            continue
        depth += (t[k] == "{") - (t[k] == "}")
        if depth < 0:
            return t
        k += 1
    stack = []
    for e in re.finditer(r"\\(begin|end)\{([^}]*)\}", t):
        if e.group(1) == "begin":
            stack.append(e.group(2))
        elif not stack:
            return t
        else:
            stack.pop()
    if depth or (stack and cut):
        t += " \\cdots" + "}" * depth
    # (a fraction's upper half with nothing behind it)
    for f in reversed(list(re.finditer(r"\\[td]?frac(?![a-zA-Z])", t))):
        top = group_at(t, f.end())
        rest = t[top[1] :].strip() if top else "x"
        if not rest and not t[: f.start()].strip() and not stack:
            t = top[0]
        elif not rest or rest.startswith("}"):
            t = t[: top[1]] + "{}" + t[top[1] :]
    return t + "".join(" \\end{" + name + "}" for name in reversed(stack))


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


def as_latex(raw: str, whole: bool = False):
    """What the page reader gave for a formula, as LaTeX; None where it is
    no formula (plain words, or marked up as text). `whole`: a formula it
    broke off is closed (see `finished`), not given up."""
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
            p = m.fix(m.clean(p.strip()))
            p = mended(finished(p) if whole else p)
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


def misshapen(t: str) -> int:
    """How many arrays and matrices of a reading are not whole: rows of
    unequal length, more cells in a row than columns, or a row with
    nothing in it (the math recognizer writes "{{}}&{{}}" rows into a
    matrix now and then). Single empty cells are no fault: a diagonal
    matrix is printed with them."""
    n = 0
    for a in re.finditer(r"\\begin\{(array|[a-zA-Z]*matrix)\}(\{[^{}]*\})?(.*?)\\end\{\1\}", t, re.S):
        body = a.group(3)
        if "\\begin{" in body:
            continue
        rows = re.split(r"\\\\(?:\[[^\]]*\])?", body)
        if rows and not re.sub(r"[{}\s]", "", rows[-1]):
            rows = rows[:-1]
        cells = [r.split("&") for r in rows]
        blank = [all(not re.sub(r"[{}\s]|\\(?:quad|qquad|,|;|!)", "", c) for c in r) for r in cells]
        columns = len(re.findall(r"[lcr]", a.group(2) or ""))
        if len({len(r) for r in cells}) > 1 or any(blank) or (columns and max(map(len, cells), default=0) > columns):
            n += 1
    return n


def choose(first: str, second: str, other):
    """(the reading to write, or None for the first; how many symbols are in
    doubt: 0 when two recognizers agree)."""
    if other is None:
        n = m.difference(first, second) if second else 0
        # (the same formula twice, its matrix whole only the second time)
        whole = second and n == 0 and misshapen(first) and not misshapen(second)
        return (second if whole else None), n
    lo = loose(other)
    if loose(first) == lo:
        # (of readings that are the same formula, one whose matrix is whole
        # comes before one with an empty row or rows of unequal length)
        # (… where it has the bold letters too: rows of unequal length in
        # a set of lines do no harm, lost bold letters do)
        if misshapen(first):
            for t in (second, other):
                if t and loose(t) == lo and not misshapen(t) and bolds(t) >= bolds(first):
                    return t, 0
        return (other if bolds(other) > bolds(first) and misshapen(other) <= misshapen(first) else None), 0
    if second and loose(second) == lo:
        if misshapen(other) != misshapen(second):
            return (other if misshapen(other) < misshapen(second) else second), 0
        return (other if bolds(other) >= bolds(second) else second), 0
    return other, max(1, m.difference(first, other))


def page_reader(state: dict):
    """The page reader's recognizer, loaded when first needed."""
    if "rec" not in state:
        from surya.foundation import FoundationPredictor
        from surya.recognition import RecognitionPredictor

        tell("loading", 0, 1)
        state["rec"] = RecognitionPredictor(FoundationPredictor())
        state["rec"].disable_tqdm = True
        tell("loading", 1, 1)
    return state["rec"]


def code_lines(raw: str):
    """The lines of program text in what the recognizer gave (it may mark
    the ends of lines with <br>)."""
    import html

    t = html.unescape(re.sub(r"<br\s*/?>", "\n", raw))
    return [line.strip() for line in t.splitlines() if line.strip()]


def set_in(img: Image.Image, lines):
    """The lines of program text, each set in as far as it is in the
    picture (the recognizer gives them all from the margin; in some
    languages the indent is part of the program). Program text is set in
    type of one width: the lines stand at one pitch, and each is as wide
    as its letters are many. Where the picture does not bear that out
    (an empty line in it, a line read short), the lines stay as read."""
    g = img.convert("L")
    w, h = g.size
    px = g.load()
    dark = [[x for x in range(w) if px[x, y] < 190] for y in range(h)]
    rows = [y for y in range(h) if dark[y]]
    if not rows or len(lines) < 2:
        return lines
    top, pitch = rows[0], (rows[-1] + 1 - rows[0]) / len(lines)
    edges = []
    for i in range(len(lines)):
        # (the middle of the line's height: clear of its neighbours)
        a = int(top + (i + 0.25) * pitch)
        b = max(a + 1, int(top + (i + 0.75) * pitch))
        xs = [x for y in range(a, min(b, h)) for x in dark[y]]
        if not xs:
            return lines
        edges.append((min(xs), max(xs) + 1))
    widths = sorted((r - l) / len(t) for (l, r), t in zip(edges, lines) if len(t) >= 8)
    if not widths:
        return lines
    cw = widths[len(widths) // 2]
    if any(abs((r - l) - len(t) * cw) > max(1.5 * cw, 0.1 * len(t) * cw) for (l, r), t in zip(edges, lines)):
        return lines
    left = min(l for l, _ in edges)
    ins = [round((l - left) / cw) for l, _ in edges]
    return [" " * n + t for n, t in zip(ins, lines)] if max(ins) <= 40 else lines


def read_code(files, dst, state: dict):
    """Program code (what the plain-text reader gave stands in NAME.tex
    under "%CODE"): the page reader's recognizer reads such text far
    better — the plain reader takes named characters like \\[Psi] apart.
    Its reading is written to NAME.best where it is program text too and
    about as long as the plain reader's; what it gave is kept as
    NAME.surya-code, so each picture is read once."""
    if not files:
        return
    rec = page_reader(state)
    better = 0
    for n, f in enumerate(files):
        plain = (dst / f"{f.stem}.tex").read_text().split("\n", 1)[1]
        size = len(re.sub(r"\s", "", plain))
        raw = ""
        for scale in (2, 3):
            img = prepared(f, scale)
            p = rec(images=[img], bboxes=[[[0, 0, img.width, img.height]]], task_names=["block_without_boxes"], math_mode=False, max_tokens=1024)[0]
            raw = "\n".join(line.text for line in p.text_lines)
            lines = code_lines(raw)
            text = "\n".join(lines)
            got = len(re.sub(r"\s", "", text))
            if not re.search(r"</?[a-z]+\d?>", text) and m.looks_like_code(text) and m.balanced(text) and 0.8 * size <= got <= 1.25 * size:
                (dst / f"{f.stem}.best").write_text("%CODE\n" + "\n".join(set_in(Image.open(f), lines)) + "\n")
                better += 1
                break
        (dst / f"{f.stem}.surya-code").write_text(raw + "\n")
        tell("comparing", n + 1, len(files))
    print(f"  {better} of {len(files)} pieces of program code are written as the page reader's recognizer read them", file=sys.stderr, flush=True)


def read(files, dst, ext: str, scale: int, state: dict):
    """Read the pictures with the page reader's recognizer, enlarged
    `scale` times; what it gives is kept as NAME.`ext`."""
    if not files:
        return
    rec = page_reader(state)
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


# ---------------------------------------------------------------------------
# Equations built from the letters of a PDF's page (a TeX book): a second
# reading, by the recognizer, of the place on the page each was built from.

PLACES = "places 15"
# pixels of a cut-out for each pixel of the page reader's measure (it has
# 1.5 to a point: 216 dots to the inch, twice the size of an ebook's pictures)
ZOOM = 2.0


def tidy(t: str) -> str:
    """The ways of writing in which the LaTeX built by rule differs from a
    recognizer's: a space before a bracket (which made "u ( x )" at a
    line's end look like an equation's number), the ångström, the
    variants of Greek letters (the page's letter is the one that counts),
    a root sign left bare, dots, a bullet in front."""
    # (a forced space first: "\ |" must not become the double bar "\|")
    t = re.sub(r"(?<!\\)((?:\\\\)*)\\\s", r"\1 ", t)
    t = re.sub(r"\s+(?=[()\[\]|,.;:!])", "", t)
    t = t.replace("˚ A", "Å").replace("˚A", "Å")
    t = re.sub(r"\\mathring\s*\{?\s*A\s*\}?", "Å", t)
    t = re.sub(r"\\text\s*\{\s*Å\s*\}", "Å", t)
    t = t.replace("ℑ", "\\Im ").replace("ℜ", "\\Re ")
    t = re.sub(r"\\var(phi|epsilon|theta|rho)(?![a-zA-Z])", r"\\\1", t)
    t = re.sub(r"\\surd(?![a-zA-Z])", r"\\sqrt", t)
    t = re.sub(r"\.\s*\.\s*\.", r"\\dots ", t)
    t = re.sub(r"^\s*(\\bullet|•)\s*", "", t)
    # (a letter with a stroke through it, as the rules write it and as a
    # recognizer does)
    t = re.sub(r"\{\s*([^{}]+?)\s*\\!\\!\\!/\s*\}", r"\\not \1 ", t)
    t = re.sub(r"\\(slashed|cancel)(?![a-zA-Z])", r"\\not", t)
    # (… and as it writes it without the braces: "\partial \!\!\!/", two
    # or three steps back)
    t = re.sub(r"(\\[a-zA-Z]+|[A-Za-z])\s*(?:\\!\s*){2,3}/", r"\\not \1 ", t)
    # (… which is the page's to say: the recognizer often reads p̸ as p.
    # Not the stroke of a sign: "=" and "≠" are two formulas)
    t = re.sub(r"\\not\s*\{?\s*(?=[A-Za-z]|\\(?:alpha|beta|gamma|delta|epsilon|varepsilon|partial|nabla|sigma|pi|mu|nu|kappa|lambda|xi|eta|zeta|theta|rho|tau|phi|varphi|chi|psi|omega|ell|Gamma|Delta|Sigma|Pi|Phi|Psi|Omega|Lambda)(?![a-zA-Z]))", " ", t)
    # (the 1 with a doubled stem, as a sign and by its commands)
    t = re.sub(r"\\(mathbb|mathbbm|mathds)\s*(\{\s*1\s*\}|1)", "𝟙", t)
    # (… which the recognizer often reads as a plain 1: the page's to say)
    t = t.replace("𝟙", "1")
    # (a Greek capital as a letter and by its name)
    for letter, name in zip("ΓΔΘΛΞΠΣΥΦΨΩ", ("Gamma", "Delta", "Theta", "Lambda", "Xi", "Pi", "Sigma", "Upsilon", "Phi", "Psi", "Omega")):
        t = t.replace(letter, "\\" + name + " ")
    # (a bar in the middle of a bracket, as tall as the bracket or not)
    t = re.sub(r"\\middle(?![a-zA-Z])", "", t)
    # (so is the stroke through the stem of a d, which it reads as d)
    t = t.replace("đ", "d")
    # (a prime before the index "parallel" or after it: the same, and told
    # here, before that index is written as two bars)
    t = re.sub(r"((?:\^\s*\{\s*(?:\\prime\s*)+\}|')+)\s*(_\s*\{?\s*\\(?:parallel|perp)\s*\}?)", r"\2\1", t)
    # (the double bar of a norm, three ways; three integral signs, two; a
    # heavy dot between two vectors and a light one)
    t = re.sub(r"\\parallel(?![a-zA-Z])|\\\||\\Vert(?![a-zA-Z])|\\lVert(?![a-zA-Z])|\\rVert(?![a-zA-Z])", "||", t)
    t = re.sub(r"\\iiint(?![a-zA-Z])", r"\\int\\int\\int ", t)
    t = re.sub(r"\\bullet(?![a-zA-Z])", r"\\cdot ", t)
    # (a wide accent and a narrow one; the sun's sign in two sizes; "it
    # follows" in two spellings; the ohm sign for Ω; a frame around the
    # whole, which the rules do not draw)
    t = re.sub(r"\\wide(tilde|hat)(?![a-zA-Z])", r"\\\1", t)
    t = re.sub(r"\\overline(?![a-zA-Z])", r"\\bar", t)
    t = re.sub(r"\\bigodot(?![a-zA-Z])", r"\\odot", t)
    t = re.sub(r"\\implies(?![a-zA-Z])", r"\\Rightarrow ", t)
    t = re.sub(r"\\impliedby(?![a-zA-Z])", r"\\Leftarrow ", t)
    t = re.sub(r"\\iff(?![a-zA-Z])", r"\\Leftrightarrow ", t)
    t = t.replace("\u2126", "\\Omega ")
    t = re.sub(r"\\boxed(?![a-zA-Z])", " ", t)
    # (bold by another command, which the comparison does not count: the
    # PDF's font says what is bold; a brace by its name)
    t = re.sub(r"\\bm(?![a-zA-Z])", " ", t)
    t = re.sub(r"\\lbrace(?![a-zA-Z])", r"\\{", t)
    t = re.sub(r"\\rbrace(?![a-zA-Z])", r"\\}", t)
    # (a row of "and so on" under a matrix, one in each column: how many
    # there are, the recognizer does not count)
    t = re.sub(r"(?:\\(?:dots|cdots|ldots)(?![a-zA-Z])\s*&?\s*){2,}", r"\\dots ", t)
    # (a script letter is the page's to say: the recognizer reads 𝒜 as A;
    # so is a Fraktur one, which it also reads as a script letter)
    # (a blank in its place: "\\mu}\\mathfrak{D}" must not become "\\muD")
    t = re.sub(r"\\(mathcal|cal|mathscr|mathfrak|frak)(?![a-zA-Z])", " ", t)
    return re.sub(r"\\(quad|qquad)(?![a-zA-Z])", " ", t)


# what says how a formula's signs stand to each other, and is no sign itself
STRUCTURE = {"^", "_", "\\frac", "\\beginarray", "\\endarray", "\\hline", "\\B", "\\over", ".", ",", ";", "\\begin", "\\end"}


def signs(t: str):
    """The letters and signs of a formula, each with how often it occurs:
    brackets and bars too, but not what only orders them (an exponent's
    "^", a fraction's command, the rows of an array)."""
    import collections

    return collections.Counter(tok for tok in m.canonical(tidy(t)) if tok not in STRUCTURE)


COLUMNS = r"\\begin\{(pmatrix|bmatrix|Bmatrix|vmatrix|Vmatrix|matrix|smallmatrix|cases|array)\}"


def shape(t: str):
    """The matrices, cases and arrays of a formula: of each, how many rows
    and how many cell marks. And how many fractions it has whose two
    halves are fractions themselves. The recognizer reads two rows of
    fractions between brackets as one such fraction of fractions: a
    reading with other matrices than the rules built, or more of those
    fractions, is not taken unseen."""
    grids = []
    for f in re.finditer(COLUMNS, t):
        end = t.find("\\end{" + f.group(1) + "}", f.end())
        body = t[f.end() : end if end > 0 else len(t)]
        grids.append((body.count("\\\\") + 1, body.count("&")))
    fraction = r"\\[td]?frac(?![a-zA-Z])"

    def leads(half: str) -> bool:
        # (a half that is a fraction: with its sign, in its brackets)
        return re.match(r"[\s\-+({\[]*(\\left\s*[(\[]\s*)?" + fraction, half) is not None

    stacked = 0
    for f in re.finditer(fraction, t):
        top = group_at(t, f.end())
        under = group_at(t, top[1]) if top else None
        stacked += bool(top and under and leads(top[0]) and leads(under[0]))
    return sorted(grids), stacked


def marks(t: str):
    """What only the page can say of a formula's letters, and the comparison
    does not count: script letters, a stroke through a letter, which of its
    two shapes a Greek letter has. A reading that has other marks than the
    rules' formula is not written in its place: the marks would be lost."""
    t = re.sub(r"\{\s*([^{}]+?)\s*\\!\\!\\!/\s*\}", r"\\not \1 ", t)
    t = re.sub(r"\\not\s*\{?\s*=", "", t)
    count = lambda pattern: len(re.findall(pattern, t))
    out = {
        "doubled": len(re.findall(r"\\(mathbb|mathbbm|mathds)\s*(\{\s*1\s*\}|1)|𝟙", t)),
        "script": count(r"\\(mathcal|cal|mathscr)(?![a-zA-Z])"),
        "fraktur": count(r"\\(mathfrak|frak)(?![a-zA-Z])"),
        "stroke": count(r"\\(slashed|cancel|not)(?![a-zA-Z])") + t.count("đ"),
    }
    for letter in ("phi", "epsilon", "theta", "rho"):
        out[letter] = count(r"\\var" + letter + r"(?![a-zA-Z])")
    return out


def with_shapes(rule: str, other: str) -> str:
    """The recognizer's reading with each Greek letter in the shape the
    page has it, where the rules' formula has the letter in one shape
    only: which of the two it is, the PDF names and the recognizer guesses."""
    for letter in ("phi", "epsilon", "theta", "rho"):
        curly, plain = r"\\var" + letter + r"(?![a-zA-Z])", r"\\" + letter + r"(?![a-zA-Z])"
        has_curly, has_plain = re.search(curly, rule) is not None, re.search(plain, rule) is not None
        if has_plain and not has_curly:
            other = re.sub(curly, lambda _: "\\" + letter, other)
        elif has_curly and not has_plain:
            other = re.sub(plain, lambda _: "\\var" + letter, other)
    return other


def same_formula(a: str, b: str) -> bool:
    try:
        return loose(tidy(a)) == loose(tidy(b))
    except Exception:
        return False


STROKED = r"\{\s*(\\[a-zA-Z]+|[A-Za-z])\s*\\!\\!\\!/\s*\}|\\not\s*\{?\s*(?:\\(?:partial|nabla|epsilon|varepsilon|[A-Za-z]{2,})(?![a-zA-Z])|[A-Za-z])\s*\}?"


def strokes_blind(rule: str, other: str) -> bool:
    """The same formula but for the letters that the page has with a stroke
    through them: the recognizer reads e̸ as φ, ∂̸ as a script J, p̸ as p.
    The PDF names the letter and its stroke, so whatever one letter the
    reading has in such a place counts for it."""
    if not re.search(STROKED, rule):
        return False
    try:
        mine = loose(tidy(re.sub(STROKED, " § ", rule)))
        its = loose(tidy(other))
    except Exception:
        return False
    one = r"(?:\\not)?(?:\\[a-zA-Z]+|[A-Za-z])/?"
    return re.fullmatch(re.escape(mine).replace("§", one), its) is not None


def row_closed(rule: str, other: str) -> bool:
    """The same formula, but that the recognizer closed a bracket which the
    page closes in the equation's next row (or opened one that its row
    before had opened): the rules' row has that bracket open, and the
    reading has one more at its end (its beginning) and nothing else."""
    try:
        mine, its = loose(tidy(rule)), loose(tidy(other))
    except Exception:
        return False
    for a, b in ("()", "[]"):
        if mine.count(a) > mine.count(b) and its == mine + b:
            return True
        if mine.count(b) > mine.count(a) and its == a + mine:
            return True
    return False


def columns_as_fractions(t: str) -> str:
    """A formula with each column of two rows between brackets written as
    the recognizer misreads it: a fraction between those brackets."""
    brackets = {"pmatrix": ("(", ")"), "bmatrix": ("[", "]"), "Bmatrix": ("\\{", "\\}"), "vmatrix": ("|", "|"), "matrix": ("", "")}

    def one(f):
        rows = f.group(2).split("\\\\")
        if len(rows) != 2 or "&" in f.group(2) or "\\begin" in f.group(2):
            return f.group(0)
        a, b = brackets[f.group(1)]
        return a + "\\frac{" + rows[0].strip() + "}{" + rows[1].strip() + "}" + b

    return re.sub(r"\\begin\{([pbBv]?matrix)\}(.*?)\\end\{\1\}", one, t, flags=re.S)


def agree(rule: str, other: str) -> bool:
    """Did the recognizer read the formula the rules built? Also where it
    read their columns of two rows as fractions, its known misreading (the
    rules, which have the rows from the page, are right); and where it
    wrote a system set between two braces as cases, which have only the
    left one."""
    if same_formula(rule, other) or same_formula(columns_as_fractions(rule), other) or strokes_blind(rule, other) or row_closed(rule, other):
        return True
    # (a product typed with a full stop, "e^{-ip.x}", which the recognizer
    # reads as a raised dot: the page's character is the rules' to say)
    typed = re.sub(r"(?<=[A-Za-z}])\s*\.\s*(?=[A-Za-z\\])", r" \\cdot ", rule)
    if typed != rule and same_formula(typed, other):
        return True
    if "\\begin{cases}" not in other:
        return False
    # (… between two braces drawn by the rules, or as a matrix in braces)
    as_cases = rule.replace("\\begin{Bmatrix}", "\\begin{cases}").replace("\\end{Bmatrix}", "\\end{cases}")
    return same_formula(re.sub(r"\\right\s*\\\}", "", rule), other) or same_formula(as_cases, other)


def page_picture(pdf: str, page: int, state: dict) -> Image.Image:
    """A page of the PDF as a picture, at the size the cut-outs are made
    (the last few pages are kept: equations come page by page)."""
    import subprocess
    import tempfile

    kept = state.setdefault("pages", {})
    if page not in kept:
        if len(kept) >= 4:
            kept.clear()
        with tempfile.TemporaryDirectory() as tmp:
            stem = os.path.join(tmp, "p")
            subprocess.run(["pdftoppm", "-r", str(72 * 1.5 * ZOOM), "-f", str(page), "-l", str(page), "-singlefile", "-png", pdf, stem], check=True, capture_output=True)
            kept[page] = Image.open(stem + ".png").convert("RGB")
    return kept[page]


def cut_out(pdf: str, job, state: dict, scale: float = 1.0) -> Image.Image:
    """The place of an equation on its page, on a white ground."""
    page = page_picture(pdf, job["page"], state)
    x0, y0, x1, y1 = (v * ZOOM for v in job["area"])
    edge = 8
    img = page.crop((max(0, int(x0 - edge)), max(0, int(y0 - edge)), min(page.width, int(x1 + edge)), min(page.height, int(y1 + edge))))
    if scale != 1.0:
        img = img.resize((max(1, int(img.width * scale)), max(1, int(img.height * scale))), Image.LANCZOS)
    out = Image.new("RGB", (img.width + 40, img.height + 40), "white")
    out.paste(img, (20, 20))
    return out


def read_places(pdf: str, jobs, dst, ext: str, scale: float, state: dict):
    """Read the places of these equations; what the recognizer gives is
    kept as KEY.`ext`. The widest go together: a batch takes as long as
    its largest picture."""
    if not jobs:
        return
    rec = page_reader(state)
    done = 0
    tell("comparing", 0, len(jobs))
    # (a few pages at a time, so each page is drawn once; within that by
    # length: a batch takes as long as its longest formula, and no longer
    # than the LaTeX the rules built says it can be — the recognizer now
    # and then writes the columns of an array on and on, to its limit)
    order = sorted(jobs, key=lambda j: (j["page"] // 8, len(j["rule"])))
    for i in range(0, len(order), BATCH):
        part = order[i : i + BATCH]
        imgs = [cut_out(pdf, j, state, scale) for j in part]
        limit = min(1024, 96 + max(len(j["rule"]) for j in part))
        preds = rec(images=imgs, bboxes=[[[0, 0, im.width, im.height]] for im in imgs], task_names=["block_without_boxes"] * len(imgs), math_mode=True, max_tokens=limit)
        for j, p in zip(part, preds):
            (dst / f"{j['key']}.{ext}").write_text(" ".join(line.text for line in p.text_lines) + "\n")
        done += len(part)
        tell("comparing", done, len(jobs))


def places(pdf: str, jobs_file: str, dst: Path):
    """The second reading of equations built from a page's letters. Each
    line of the jobs: key, page, the area (four numbers), a stamp (the
    rules' version and a print of the LaTeX built), that LaTeX.

    Written for each, KEY.second: the verdict, the stamp, and the
    recognizer's LaTeX.
      same   the recognizer read the formula the rules built
      take   it read the same letters and signs in another order (an
             exponent that the rules left on the line, a small fraction's
             halves), with the same matrices and no more fractions of
             fractions, the same script letters, strokes and shapes of
             Greek letters (which only the page can say), and
             read them so twice: its reading is the formula
             Also where the rules' formula holds a glyph the PDF does
             not name ("\ufffd"), and the recognizer reads the place twice
             the same: there the PDF's letters are not the surer ones
      check  the readings differ in more than that, or its two readings
             differ: to be looked at. A reading that the recognizer
             broke off is listed closed (see `finished`), so that it
             can be typeset
      unread it gave no formula (words, or nothing)"""
    jobs = []
    for line in open(jobs_file, encoding="utf-8"):
        f = line.rstrip("\n").split("\t")
        if len(f) >= 8:
            jobs.append({"key": f[0], "page": int(f[1]), "area": [float(x) for x in f[2:6]], "stamp": f[6], "rule": f[7]})
    state = {}
    read_places(pdf, [j for j in jobs if not (dst / f"{j['key']}.surya").exists()], dst, "surya", 1.0, state)

    def reading(j, ext):
        f = dst / f"{j['key']}.{ext}"
        return as_latex(f.read_text(), whole=True) if f.exists() else None

    # (where it reads other than the rules built: once more, larger)
    differs = [j for j in jobs if (o := reading(j, "surya")) is not None and not agree(j["rule"], o)]
    read_places(pdf, [j for j in differs if not (dst / f"{j['key']}.surya3").exists()], dst, "surya3", 1.5, state)
    counts = {"same": 0, "take": 0, "check": 0, "unread": 0}
    for j in jobs:
        other = reading(j, "surya")
        # (one thing written on and on, "+ \\mu + \\mu + …" for a row of
        # drawn diagrams, is no reading)
        if other is not None and on_and_on(other):
            other = None
        if other is not None:
            other = with_shapes(j["rule"], other)
        if other is None:
            verdict = "unread"
        elif agree(j["rule"], other):
            verdict = "same"
        else:
            again = reading(j, "surya3")
            # (a reading that had to be closed is shown, never taken)
            broken = as_latex((dst / f"{j['key']}.surya").read_text()) is None
            twice = again is not None and not broken and same_formula(other, again)
            (grids, stacked), (other_grids, other_stacked) = shape(j["rule"]), shape(other)
            if "\ufffd" in j["rule"]:
                # (a glyph the PDF does not name: its letters are not the
                # surer ones here. The recognizer's reading, read twice the
                # same and about as long, is the formula)
                mine, its = sum(signs(j["rule"]).values()), sum(signs(other).values())
                verdict = "take" if twice and 0.6 * mine <= its <= 1.6 * mine else "check"
            else:
                verdict = "take" if twice and signs(j["rule"]) == signs(other) and marks(j["rule"]) == marks(other) and grids == other_grids and other_stacked <= stacked else "check"
        counts[verdict] += 1
        (dst / f"{j['key']}.second").write_text(f"{verdict}\n{j['stamp']}\n{other or ''}\n")
    print(f"  {len(jobs)} equations built from the page's letters: {counts['same']} read the same by the recognizer, {counts['take']} written as it read them, {counts['check']} listed to check, {counts['unread']} not read", file=sys.stderr, flush=True)


def main():
    if len(sys.argv) > 4 and sys.argv[1] == "--places":
        return places(sys.argv[2], sys.argv[3], Path(sys.argv[4]))
    src, dst = Path(sys.argv[1]), Path(sys.argv[2])
    files = sorted(src.glob("*.png"))
    math = []
    for f in files:
        tex = dst / f"{f.stem}.tex"
        if tex.exists() and not tex.read_text().startswith("%"):
            math.append(f)
    state = {}
    code = [f for f in files if (dst / f"{f.stem}.tex").exists() and (dst / f"{f.stem}.tex").read_text().startswith("%CODE\n")]
    read_code([f for f in code if not (dst / f"{f.stem}.surya-code").exists()], dst, state)
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
