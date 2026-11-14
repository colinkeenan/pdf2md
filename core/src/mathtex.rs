//! Rebuilding LaTeX from positioned spans.
//!
//! A displayed equation in a PDF is a 2-D picture, not a string. A fraction
//! is a numerator *above* the main line and a denominator *below* it (the
//! fraction bar is a drawn line, so it is not in the text at all). A sum has
//! its limits above and below the big Σ. Exponents are small and raised.
//!
//! [`build`] looks at where each span sits relative to the *main row* (the
//! row of full-size symbols, preferably the one containing "=") and
//! classifies it:
//!
//! * on the main row -> written as is
//! * small, next to a main-row symbol, raised/lowered -> `^{..}` / `_{..}`
//! * above / below the row -> grouped by horizontal position; a group above
//!   that overlaps a group below is a fraction `\frac{above}{below}`; groups
//!   above/below a big Σ or ∫ are its limits.
//!
//! The pieces (numerators, limits...) are built the same way, recursively.
//! [`display`] splits a displayed block into separate equations and pulls
//! out equation numbers like "(16)".

use crate::layout::{Doc, Rule, Span};

/// The parts of a span that matter for math.
#[derive(Debug, Clone)]
pub struct MSpan {
    pub text: String,
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub size: f32,
    pub family: String,
    pub math: bool,
    /// a drawn bar above it: Ā
    pub over: bool,
    /// set in a bold font (vectors like **r**, **p**)
    pub bold: bool,
    /// Already-built LaTeX standing in for a whole fraction or root (see
    /// [`resolve_bars`]); positioned where the fraction bar was.
    pub atom: Option<String>,
}

impl MSpan {
    pub fn new(doc: &Doc, s: &Span) -> MSpan {
        MSpan {
            text: s.text.clone(),
            left: s.left,
            right: s.right(),
            top: s.top,
            bottom: s.bottom(),
            size: doc.size_of(s),
            family: doc.family(s).to_string(),
            math: doc.is_math(s),
            over: s.overline,
            bold: s.bold || crate::layout::is_bold_font(doc.family(s)),
            atom: s.tex.clone(),
        }
    }
    fn cy(&self) -> f32 {
        (self.top + self.bottom) / 2.0
    }
    fn height(&self) -> f32 {
        self.bottom - self.top
    }
    /// TeX's "extension" font holds big operators and big delimiters. Their
    /// reported boxes are unreliable, so they are always kept on the main row.
    fn cmex(&self) -> bool {
        self.family.starts_with("CMEX")
    }
}

// ---------------------------------------------------------------------------
// Characters -> LaTeX

/// LaTeX for one Unicode character, if it needs translating.
pub fn sym(c: char) -> Option<&'static str> {
    Some(match c {
        // the stroke through the sign or letter after it (≠, the Feynman slash)
        '\u{338}' => "\\not ",
        'α' => "\\alpha ",
        'β' => "\\beta ",
        'γ' => "\\gamma ",
        'δ' => "\\delta ",
        'ε' | 'ϵ' | 'ǫ' => "\\epsilon ",
        'ζ' => "\\zeta ",
        'η' => "\\eta ",
        'θ' => "\\theta ",
        'ϑ' => "\\vartheta ",
        'ι' => "\\iota ",
        'κ' => "\\kappa ",
        'λ' => "\\lambda ",
        'µ' | 'μ' => "\\mu ",
        'ν' => "\\nu ",
        'ξ' => "\\xi ",
        'π' => "\\pi ",
        'ϖ' => "\\varpi ",
        'ρ' => "\\rho ",
        'ϱ' => "\\varrho ",
        'σ' => "\\sigma ",
        'ς' => "\\varsigma ",
        'τ' => "\\tau ",
        'υ' => "\\upsilon ",
        'φ' => "\\varphi ",
        'ϕ' => "\\phi ",
        'χ' => "\\chi ",
        'ψ' => "\\psi ",
        'ω' => "\\omega ",
        'Γ' => "\\Gamma ",
        'Δ' | '∆' => "\\Delta ",
        'Θ' => "\\Theta ",
        'Λ' => "\\Lambda ",
        'Ξ' => "\\Xi ",
        'Π' => "\\Pi ",
        'Σ' => "\\Sigma ",
        'Υ' => "\\Upsilon ",
        'Φ' => "\\Phi ",
        'Ψ' => "\\Psi ",
        'Ω' => "\\Omega ",
        '\u{2126}' => "\\Omega ", // the ohm sign, as some fonts encode Ω
        'ℓ' => "\\ell ",
        '□' => "\\Box ",
        'ı' => "\\imath ",
        '∂' => "\\partial ",
        '∇' => "\\nabla ",
        '∫' => "\\int ",
        '∮' => "\\oint ",
        '∑' => "\\sum ",
        '∏' => "\\prod ",
        '√' => "\\surd ",
        '∗' => "*",
        '′' => "\\prime ",
        '≈' => "\\approx ",
        '≃' => "\\simeq ",
        '≅' => "\\cong ",
        '≪' => "\\ll ",
        '≫' => "\\gg ",
        '≥' => "\\geq ",
        '≤' => "\\leq ",
        '≠' => "\\neq ",
        '→' => "\\to ",
        '←' => "\\leftarrow ",
        '↔' => "\\leftrightarrow ",
        '⇒' => "\\Rightarrow ",
        '∞' => "\\infty ",
        '±' => "\\pm ",
        '∓' => "\\mp ",
        '×' => "\\times ",
        '·' | '⋅' => "\\cdot ",
        '∼' => "\\sim ",
        '≡' => "\\equiv ",
        '∝' => "\\propto ",
        '∈' => "\\in ",
        '⊗' => "\\otimes ",
        '⊕' => "\\oplus ",
        '†' => "\\dagger ",
        '⊥' => "\\perp ",
        '∥' => "\\parallel ",
        '⟨' | '〈' | '\u{2329}' => "\\langle ",
        '⟩' | '〉' | '\u{232a}' => "\\rangle ",
        '…' => "\\dots ",
        '−' => "-",
        // In TeX fonts ħ is drawn with a glyph that comes out as "~".
        'ℏ' | 'ħ' | 'ћ' | '~' => "\\hbar ",
        '{' => "\\{",
        '}' => "\\}",
        '%' => "\\%",
        '#' => "\\#",
        '&' => "\\&",
        '$' => "\\$",
        '_' => "\\_",
        _ => return None,
    })
}

fn chars_tex(t: &str) -> String {
    let t = t.replace(". . .", "…").replace("...", "…");
    let mut out = String::new();
    for c in t.chars() {
        if c.is_whitespace() {
            continue;
        }
        match sym(c) {
            Some(s) => out.push_str(s),
            None => out.push(c),
        }
    }
    out.trim().to_string()
}

/// Function names LaTeX already knows (`\log`, `\sin`, ...).
const FUNCS: &[&str] = &[
    "log", "ln", "sin", "cos", "tan", "cot", "sec", "csc", "sinh", "cosh", "tanh", "coth", "exp",
    "lim", "max", "min", "det", "arg", "dim", "ker", "sup", "inf", "arcsin", "arccos", "arctan",
    "deg", "gcd",
];
/// Other upright operator names seen in physics books.
const OPNAMES: &[&str] = &["Tr", "Sp", "Spur", "Re", "Im", "sgn", "const"];

/// Is this word written upright as a function name inside math?
pub fn is_function_word(w: &str) -> bool {
    FUNCS.contains(&w) || OPNAMES.contains(&w)
}

fn flush_words(words: &mut Vec<String>, out: &mut Vec<String>) {
    if !words.is_empty() {
        out.push(format!("\\text{{ {} }}", words.join(" ")));
        words.clear();
    }
}

/// Text-font (upright) characters inside math: digits and operators pass
/// through, known function names become `\log` etc., other words `\text{..}`.
fn text_font_tex(t: &str) -> String {
    let chars: Vec<char> = t.chars().collect();
    let mut out: Vec<String> = Vec::new();
    let mut words: Vec<String> = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c.is_alphabetic() {
            let start = i;
            while i < chars.len() && chars[i].is_alphabetic() {
                i += 1;
            }
            let w: String = chars[start..i].iter().collect();
            if FUNCS.contains(&w.as_str()) {
                flush_words(&mut words, &mut out);
                out.push(format!("\\{w}"));
            } else if OPNAMES.contains(&w.as_str()) {
                flush_words(&mut words, &mut out);
                out.push(format!("\\operatorname{{{w}}}"));
            } else if w.chars().count() == 1 && words.is_empty() {
                out.push(chars_tex(&w));
            } else {
                words.push(w);
            }
            continue;
        }
        i += 1;
        if c.is_whitespace() {
            continue;
        }
        flush_words(&mut words, &mut out);
        // a prime in the size of the text: a′ is a', not "a \prime"
        if let Some(n) = "′″‴".chars().position(|p| p == c) {
            let marks = "'".repeat(n + 1);
            match out.last_mut() {
                Some(last) => last.push_str(&marks),
                None => out.push(format!("{{}}{marks}")),
            }
            continue;
        }
        // an accent typed after its letter: α̃
        let accent = match c {
            '\u{303}' => Some("tilde"),
            '\u{302}' => Some("hat"),
            '\u{304}' | '\u{305}' => Some("bar"),
            '\u{307}' => Some("dot"),
            '\u{308}' => Some("ddot"),
            '\u{20d7}' => Some("vec"),
            _ => None,
        };
        if let (Some(a), Some(last)) = (accent, out.last_mut()) {
            *last = format!("\\{a}{{{}}}", last.trim());
            continue;
        }
        out.push(chars_tex(&c.to_string()));
    }
    flush_words(&mut words, &mut out);
    out.join(" ")
}

/// Big operators in TeX's extension font. The PDF maps them to the
/// character in the font's slot, so Σ comes out as "X" or "P", ∫ as "Z"/"R".
fn big_op(s: &MSpan) -> Option<&'static str> {
    if !s.cmex() {
        return None;
    }
    match s.text.trim() {
        "X" | "P" => Some("\\sum"),
        "Z" | "R" => Some("\\int"),
        "Y" | "Q" => Some("\\prod"),
        "I" | "H" => Some("\\oint"),
        _ => None,
    }
}

/// LaTeX for one span on its own.
pub fn tex(s: &MSpan) -> String {
    if let Some(a) = &s.atom {
        return a.clone();
    }
    let mut t = tex_plain(s);
    // bold symbols (vectors) stay bold; spacing commands like \text stay as they are
    if s.bold && !t.is_empty() && !t.starts_with("\\text") {
        t = format!("\\boldsymbol{{{t}}}");
    }
    if s.over && !t.is_empty() {
        format!("\\overline{{{t}}}")
    } else {
        t
    }
}

fn tex_plain(s: &MSpan) -> String {
    let t = s.text.trim();
    if s.cmex() {
        if let Some(op) = big_op(s) {
            return op.to_string();
        }
        return match t {
            "(" | ")" | "[" | "]" | "|" | "/" => t.to_string(),
            // big angle brackets sit in the slots of D and E
            "D" => "\\langle".into(),
            "E" => "\\rangle".into(),
            "{" => "\\{".into(),
            "}" => "\\}".into(),
            "p" | "q" | "r" | "s" => "\\surd".into(),
            _ => String::new(), // unreadable glyph (big bracket pieces etc.)
        };
    }
    // In TeX's symbol font the "not" slash comes out as "6": "6=" is ≠,
    // and the angle brackets ⟨ ⟩ as "h" and "i".
    if s.family.starts_with("CMSY") {
        if t == "6" {
            return "\\not".into();
        }
        // letters in the symbol font: calligraphic capitals, and symbols
        // that sit in letter slots
        if t.chars().any(|c| c.is_ascii_alphabetic()) {
            return t
                .chars()
                .filter(|c| !c.is_whitespace())
                .map(|c| match c {
                    'h' => "\\langle".to_string(),
                    'i' => "\\rangle".to_string(),
                    'j' => "|".to_string(),
                    'k' => "\\|".to_string(),
                    'f' => "\\{".to_string(),
                    'g' => "\\}".to_string(),
                    'A'..='Z' => format!("\\mathcal{{{c}}}"),
                    c => chars_tex(&c.to_string()),
                })
                .collect::<Vec<_>>()
                .join(" ");
        }
    }
    // Blackboard bold (𝕀, ℝ ...): the AMS font stores them as plain letters.
    if s.family.starts_with("MSBM") && t.chars().all(|c| c.is_ascii_alphabetic()) && !t.is_empty() {
        return t.chars().map(|c| format!("\\mathbb{{{c}}}")).collect::<Vec<_>>().join(" ");
    }
    if s.math {
        chars_tex(t)
    } else {
        text_font_tex(t)
    }
}

// ---------------------------------------------------------------------------
// 2-D layout -> LaTeX

/// Turn a group of spans (an inline formula, or one displayed equation)
/// into LaTeX.
pub fn build(spans: &[MSpan]) -> String {
    build_with(spans, &[])
}

/// Like [`build`], using the drawn bars (fraction bars, root bars) that lie
/// inside the formula.
pub fn build_with(spans: &[MSpan], bars: &[Rule]) -> String {
    // pdftohtml sometimes puts two big operators in one span ("X X")
    let spans: Vec<MSpan> = spans
        .iter()
        .flat_map(|s| {
            if s.cmex() && s.text.trim().chars().count() > 1 && !s.text.contains(' ') {
                // "DX": one glyph per character
                let mut t = s.clone();
                t.text = s.text.trim().chars().map(|c| format!("{c} ")).collect::<String>().trim_end().to_string();
                split_words(&t)
            } else if s.cmex() {
                split_words(s)
            } else {
                vec![s.clone()]
            }
        })
        .collect();
    if std::env::var("PDF2MD_MATHDEBUG").is_ok_and(|v| v == "2") {
        eprintln!("--- before accents");
        for s in &spans {
            eprintln!("  {:7.1} {:7.1} {:7.1} {:7.1} {:4.1} {:8} {:?}", s.left, s.right, s.top, s.bottom, s.size, s.family, s.text);
        }
    }
    let spans = put_accents(spans);
    if std::env::var_os("PDF2MD_MATHDEBUG").is_some() {
        eprintln!("--- formula");
        for s in &spans {
            eprintln!("  {:7.1} {:7.1} {:7.1} {:7.1} {:4.1} {:8} {:?}", s.left, s.right, s.top, s.bottom, s.size, s.family, s.text);
        }
        for b in bars {
            eprintln!("  bar {:?}", b);
        }
    }
    let (spans, bars) = underbraces(spans, bars);
    tidy(&build_at(&resolve_bars(spans, &bars, 0), 0))
}

/// Labelled underbraces: a row of brace pieces from the big-symbol font
/// ("|", "{z", "}" …) joined by thick bars, with what it braces above it and
/// a small label below. Each becomes one ready-made piece
/// `\underbrace{…}_{\text{label}}` on the formula's main line.
fn underbraces(spans: Vec<MSpan>, bars: &[Rule]) -> (Vec<MSpan>, Vec<Rule>) {
    let size = spans.iter().filter(|s| !s.cmex()).map(|s| s.size).fold(0.0f32, f32::max).max(1.0);
    let piece = |s: &MSpan| {
        s.cmex() && s.atom.is_none() && s.height() < 1.0 * size && !s.text.trim().is_empty() && s.text.trim().chars().all(|c| "|{}z".contains(c))
    };
    let thick = |r: &Rule| r.bottom - r.top > 1.2 && r.bottom - r.top < 3.0;
    let mut idx: Vec<usize> = (0..spans.len()).filter(|&i| piece(&spans[i])).collect();
    if idx.len() < 2 {
        return (spans, bars.to_vec());
    }
    idx.sort_by(|&a, &b| spans[a].left.total_cmp(&spans[b].left));
    // runs of pieces at one height, joined by thick bars (or touching)
    let mut groups: Vec<Vec<usize>> = Vec::new();
    for i in idx {
        let s = &spans[i];
        let joins = |g: &Vec<usize>| {
            let last = &spans[*g.last().unwrap()];
            let same_row = (last.top - s.top).abs() <= 2.0;
            let bridged = bars.iter().any(|r| {
                thick(r) && r.left <= last.right + 2.0 && r.right >= s.left - 2.0 && r.top >= s.top - 2.0 && r.bottom <= s.bottom + 2.0
            });
            same_row && (s.left - last.right <= 0.5 * size || bridged)
        };
        match groups.iter_mut().find(|g| joins(g)) {
            Some(g) => g.push(i),
            None => groups.push(vec![i]),
        }
    }
    let mut drop = vec![false; spans.len()];
    let mut used_bars = vec![false; bars.len()];
    let mut atoms: Vec<MSpan> = Vec::new();
    for g in groups.into_iter().filter(|g| g.len() >= 2) {
        let (l, r) = (spans[g[0]].left, spans[*g.last().unwrap()].right);
        let (top, bottom) = (spans[g[0]].top, g.iter().map(|&i| spans[i].bottom).fold(0.0, f32::max));
        let cx = |s: &MSpan| (s.left + s.right) / 2.0;
        // (a big bracket around the whole formula is not braced)
        let inside = |s: &MSpan| cx(s) >= l && cx(s) <= r && (!s.cmex() || (s.left >= l - 1.0 && s.right <= r + 1.0));
        let content: Vec<usize> = (0..spans.len())
            .filter(|&i| !drop[i] && !g.contains(&i) && inside(&spans[i]) && spans[i].bottom <= top + 2.0 && spans[i].cy() >= top - 4.0 * size)
            .collect();
        let label: Vec<usize> = (0..spans.len())
            .filter(|&i| !drop[i] && !g.contains(&i) && inside(&spans[i]) && spans[i].top >= bottom - 2.0 && spans[i].top <= bottom + 1.5 * size)
            .collect();
        if content.is_empty() {
            continue;
        }
        let ck: Vec<usize> = (0..bars.len())
            .filter(|&k| !used_bars[k] && !thick(&bars[k]) && bars[k].left >= l - 2.0 && bars[k].right <= r + 2.0 && bars[k].bottom <= top)
            .collect();
        let cbars: Vec<Rule> = ck.iter().map(|&k| bars[k]).collect();
        for k in ck {
            used_bars[k] = true;
        }
        for (k, b) in bars.iter().enumerate() {
            if thick(b) && b.left >= l - 2.0 && b.right <= r + 2.0 && b.top >= top - 2.0 && b.bottom <= bottom + 2.0 {
                used_bars[k] = true;
            }
        }
        let c: Vec<MSpan> = content.iter().map(|&i| spans[i].clone()).collect();
        let lab: Vec<MSpan> = label.iter().map(|&i| spans[i].clone()).collect();
        let body = build_with(&c, &cbars);
        let tex = if lab.is_empty() {
            format!("\\underbrace{{{body}}}")
        } else {
            format!("\\underbrace{{{body}}}_{{{}}}", build(&lab))
        };
        // on the main line: the middle of the full-size symbols braced
        let mains: Vec<f32> = c.iter().filter(|s| s.size >= 0.9 * size && !s.cmex()).map(|s| s.cy()).collect();
        let mid = if mains.is_empty() { top - size } else { mains.iter().sum::<f32>() / mains.len() as f32 };
        for &i in content.iter().chain(&label).chain(&g) {
            drop[i] = true;
        }
        atoms.push(MSpan {
            text: String::new(),
            left: l,
            right: r,
            top: mid - size / 2.0,
            bottom: mid + size / 2.0,
            size,
            family: String::new(),
            math: true,
            over: false,
            bold: false,
            atom: Some(tex),
        });
    }
    let rest: Vec<MSpan> = spans.into_iter().enumerate().filter(|(i, _)| !drop[*i]).map(|(_, s)| s).chain(atoms).collect();
    let rest_bars: Vec<Rule> = bars.iter().enumerate().filter(|(k, _)| !used_bars[*k]).map(|(_, b)| *b).collect();
    (rest, rest_bars)
}

/// Accents that the PDF draws as glyphs of their own (˜ over a, ˆ over k):
/// the accent and its letter share a box. The letter becomes `\tilde{a}`.
fn put_accents(mut spans: Vec<MSpan>) -> Vec<MSpan> {
    let accent = |t: &str| match t.trim() {
        "˜" => Some("tilde"),
        "ˆ" => Some("hat"),
        "\u{20d7}" => Some("vec"),
        // the stroke through a letter (the Feynman slash) or a sign (≠)
        "\u{338}" => Some("not"),
        "˙" => Some("dot"),
        "¨" => Some("ddot"),
        "¯" => Some("bar"),
        "ˇ" => Some("check"),
        "˘" => Some("breve"),
        "´" => Some("acute"),
        _ => None,
    };
    // "(˜": an accent at the end of a longer span gets a span of its own
    spans = spans
        .into_iter()
        .flat_map(|s| {
            let n = s.text.chars().count();
            match s.text.chars().last() {
                Some(c) if n > 1 && accent(&c.to_string()).is_some() => {
                    let w = (s.right - s.left) / n as f32;
                    let mut a = s.clone();
                    a.text = s.text.chars().take(n - 1).collect();
                    a.right = s.right - w;
                    let mut b = s.clone();
                    b.text = c.to_string();
                    b.left = s.right - w;
                    vec![a, b]
                }
                _ => vec![s],
            }
        })
        .collect();
    let mut k = 0;
    while k < spans.len() {
        let Some(name) = accent(&spans[k].text) else {
            k += 1;
            continue;
        };
        let a = spans[k].clone();
        let mid = (a.left + a.right) / 2.0;
        // the letter under it: overlapping its middle, same line
        let base = (0..spans.len())
            .filter(|&j| j != k && accent(&spans[j].text).is_none() && spans[j].atom.is_none())
            // (an arrow or a stroke over a letter is given the height of its
            // font's line, which starts well above the letter's box: it is
            // over the letter it shares most of its height with)
            .filter(|&j| {
                let drawn_over = matches!(name, "vec" | "not") && a.top <= spans[j].top && a.bottom.min(spans[j].bottom) - spans[j].top >= 0.5 * (spans[j].bottom - spans[j].top);
                spans[j].left <= mid && spans[j].right >= mid && ((spans[j].top - a.top).abs() <= 0.75 * a.size || drawn_over)
            })
            // (of a letter and its index, both under the accent's middle,
            // it is over the one it stands nearer above)
            .min_by(|&x, &y| {
                // (an arrow's or a stroke's box says little of its height)
                let above = |j: usize| if matches!(name, "vec" | "not") { 0.0 } else { ((spans[j].top - a.top).abs() / 3.0).round() };
                above(x).total_cmp(&above(y)).then((spans[x].left - a.left).abs().total_cmp(&(spans[y].left - a.left).abs()))
            });
        let Some(j) = base else {
            k += 1;
            continue;
        };
        // which letter of the span: by position (glyphs spread evenly)
        let b = spans[j].clone();
        let chars: Vec<char> = b.text.chars().collect();
        let n = chars.len().max(1);
        // (by the usual widths of the letters; a space is narrow, and
        // nothing is drawn over one: "ψ iγ" has its bar over the ψ)
        let widths: Vec<f32> = chars.iter().map(|&c| crate::layout::char_width(c)).collect();
        let scale = (b.right - b.left) / widths.iter().sum::<f32>().max(0.01);
        let mut edges: Vec<f32> = vec![b.left];
        for w in &widths {
            edges.push(edges[edges.len() - 1] + w * scale);
        }
        let mut ci = (0..n).find(|&i| mid < edges[i + 1]).unwrap_or(n - 1);
        if chars.get(ci).is_some_and(|c| c.is_whitespace()) {
            let near = |i: usize| (0.5 * (edges[i] + edges[i + 1]) - mid).abs();
            let other = (0..n).filter(|&i| !chars[i].is_whitespace()).min_by(|&x, &y| near(x).total_cmp(&near(y)));
            ci = other.unwrap_or(ci);
        }
        let mut pieces: Vec<MSpan> = Vec::new();
        for (part, range) in [(0, 0..ci), (1, ci..ci + 1), (2, ci + 1..n)] {
            if range.is_empty() {
                continue;
            }
            let mut p = b.clone();
            p.text = chars[range.clone()].iter().collect();
            p.left = edges[range.start];
            p.right = edges[range.end];
            if part == 1 {
                let inner = tex(&p);
                p.atom = Some(format!("\\{name}{{{}}}", inner.trim()));
                p.text = String::new();
            }
            pieces.push(p);
        }
        spans.splice(j..j + 1, pieces);
        let k2 = spans.iter().position(|s| s.left == a.left && s.top == a.top && s.text == a.text).unwrap_or(k);
        spans.remove(k2);
        k = 0; // indices moved: start over (accents are rare)
    }
    spans
}

/// Symbols connected to a bar: those within 0.8 × size of it, plus any
/// symbol (from `cands`) that comes within 0.45 × size of one already taken.
fn stack(spans: &[MSpan], cands: &[usize], dist_to_bar: impl Fn(&MSpan) -> f32, size: f32) -> Vec<usize> {
    let mut taken: Vec<usize> = cands.iter().copied().filter(|&i| dist_to_bar(&spans[i]) <= 0.8 * size).collect();
    loop {
        let more: Vec<usize> = cands
            .iter()
            .copied()
            .filter(|i| !taken.contains(i))
            .filter(|&i| {
                taken.iter().any(|&t| {
                    let gap = (spans[i].top - spans[t].bottom).max(spans[t].top - spans[i].bottom);
                    gap <= 0.45 * size
                })
            })
            .collect();
        if more.is_empty() {
            return taken;
        }
        taken.extend(more);
    }
}

fn is_radical(s: &MSpan) -> bool {
    s.text.contains('√')
        || (s.cmex() && matches!(s.text.trim(), "p" | "q" | "r" | "s"))
        || (s.family.starts_with("CMSY") && s.text.trim() == "p")
}

/// Turn fraction bars and root bars into ready-made pieces.
///
/// A bar with symbols right above *and* right below it (within its width)
/// is a fraction: what is above is the numerator, what is below the
/// denominator. A bar starting right after a root sign is a square root
/// over what is below it. Bars are taken widest first, so in a fraction
/// inside a fraction the outer one claims the inner one, which is then
/// resolved when its numerator or denominator is built.
///
/// Each fraction or root is replaced by one span carrying its LaTeX
/// ([`MSpan::atom`]), placed on the bar's line, so the rest of the formula
/// is assembled around it as usual.
fn resolve_bars(mut spans: Vec<MSpan>, bars: &[Rule], depth: usize) -> Vec<MSpan> {
    if bars.is_empty() || spans.is_empty() || depth > 6 {
        return spans;
    }
    let size = spans.iter().filter(|s| !s.cmex()).map(|s| s.size).fold(0.0f32, f32::max).max(1.0);
    // A piece of text that goes on past the end of a root's bar ("π e"
    // under the bar of √2π, the e after it) is cut there: the root ends
    // where its bar ends.
    for b in bars {
        let root = spans.iter().any(|s| is_radical(s) && (b.left - s.right).abs() <= 3.0 && s.top <= b.bottom + size && s.bottom >= b.top - 2.0);
        let over = |s: &MSpan| s.atom.is_none() && !s.cmex() && s.left < b.right - 2.0 && s.right > b.right + 3.0 && s.top >= b.bottom - 2.0 && s.top <= b.bottom + 1.5 * size;
        while let Some(k) = spans.iter().position(|s| root && over(s) && s.text.trim().chars().count() >= 2) {
            let whole = spans[k].clone();
            let chars: Vec<char> = whole.text.chars().collect();
            let w = (whole.right - whole.left) / chars.len() as f32;
            let cut = (((b.right - whole.left) / w).round() as usize).clamp(1, chars.len() - 1);
            let (mut first, mut second) = (whole.clone(), whole.clone());
            (first.text, first.right) = (chars[..cut].iter().collect(), whole.left + w * cut as f32);
            (second.text, second.left) = (chars[cut..].iter().collect(), whole.left + w * cut as f32);
            if first.text.trim().is_empty() || second.text.trim().is_empty() || first.right > b.right + 3.0 {
                break;
            }
            spans.splice(k..=k, [first, second]);
        }
    }
    let mut order: Vec<usize> = (0..bars.len()).collect();
    order.sort_by(|&a, &b| (bars[b].right - bars[b].left).total_cmp(&(bars[a].right - bars[a].left)));
    let mut used = vec![false; bars.len()];
    for &bi in &order {
        if used[bi] {
            continue;
        }
        let b = bars[bi];
        let within = |s: &MSpan| {
            let c = (s.left + s.right) / 2.0;
            c >= b.left - 2.0 && c <= b.right + 2.0
        };
        // bars lying in a region (to hand down for nested fractions)
        let inner = |used: &mut Vec<bool>, above: bool| -> Vec<Rule> {
            let mut v = Vec::new();
            for (k, r) in bars.iter().enumerate() {
                let c = (r.left + r.right) / 2.0;
                let side = if above {
                    r.bottom < b.top && r.top >= b.top - 3.5 * size
                } else {
                    r.top > b.bottom && r.bottom <= b.bottom + 3.5 * size
                };
                // nested bars are clearly narrower (same width = a sibling)
                let narrower = r.right - r.left < 0.9 * (b.right - b.left);
                if !used[k] && k != bi && narrower && c >= b.left - 2.0 && c <= b.right + 2.0 && side {
                    used[k] = true;
                    v.push(*r);
                }
            }
            v
        };

        // a square root?
        if let Some(ri) = (0..spans.len()).find(|&i| {
            let s = &spans[i];
            is_radical(s) && (b.left - s.right).abs() <= 3.0 && s.top <= b.bottom + size && s.bottom >= b.top - 2.0
        }) {
            let under: Vec<usize> = (0..spans.len())
                // (the box of a sign, the minus of √−g, starts a little above the bar)
                .filter(|&i| i != ri && within(&spans[i]) && (spans[i].top >= b.top - 2.0 || (spans[i].top >= b.top - 0.35 * size && spans[i].cy() >= b.bottom + 0.3 * size)) && spans[i].cy() <= b.bottom + 2.5 * size)
                .collect();
            if under.is_empty() {
                continue;
            }
            used[bi] = true;
            let nested = inner(&mut used, false);
            let body: Vec<MSpan> = under.iter().map(|&i| spans[i].clone()).collect();
            let t = format!("\\sqrt{{{}}}", build_at(&resolve_bars(body, &nested, depth + 1), depth + 1));
            let left = spans[ri].left;
            let atom = MSpan {
                text: String::new(),
                left,
                right: b.right,
                top: b.bottom + 1.0,
                bottom: b.bottom + 1.0 + size,
                size,
                family: String::new(),
                math: true,
                over: false,
                bold: false,
                atom: Some(t),
            };
            let mut k = 0;
            spans.retain(|_| {
                k += 1;
                k - 1 != ri && !under.contains(&(k - 1))
            });
            spans.push(atom);
            continue;
        }

        // a fraction? The numerator is the stack of symbols that starts
        // right above the bar and grows upwards through symbols close to
        // each other (so it never reaches the equation on the line above);
        // the denominator likewise below.
        let above: Vec<usize> = (0..spans.len())
            .filter(|&i| within(&spans[i]) && spans[i].bottom <= b.top + 2.0 && spans[i].cy() >= b.top - 3.5 * size)
            .collect();
        // (a root sign in the denominator reaches up past the bar)
        let below: Vec<usize> = (0..spans.len())
            .filter(|&i| {
                let s = &spans[i];
                // (a sign of a symbol font has a box taller than the letters':
                // the minus of √−g starts a little above the bar it is under)
                let tall_under = s.top >= b.bottom - 0.35 * size && s.cy() >= b.bottom + 0.3 * size;
                within(s) && (s.top >= b.bottom - 2.0 || tall_under || (is_radical(s) && s.cy() >= b.top)) && s.cy() <= b.bottom + 3.5 * size
            })
            .collect();
        // A fraction nested inside this one has a clearly narrower bar; a
        // bar of about the same width right above or below is a separate
        // fraction stacked on this one (as in a column vector).
        let width = b.right - b.left;
        // (a root's bar, starting right at its root sign, always belongs to
        // what is under the root)
        let root_bar = |r: &Rule| spans.iter().any(|s| is_radical(s) && (r.left - s.right).abs() <= 3.0 && s.top <= r.bottom + size && s.bottom >= r.top - 2.0);
        let nested = |r: &Rule| {
            let c = (r.left + r.right) / 2.0;
            (r.right - r.left < 0.9 * width || root_bar(r)) && c >= b.left - 2.0 && c <= b.right + 2.0
        };
        // Nothing beyond a bar that is not nested in this fraction belongs to it.
        let blocked = |s: &MSpan| {
            bars.iter().any(|r| {
                let between = (r.bottom < b.top && r.top >= s.bottom - 2.0) || (r.top > b.bottom && r.bottom <= s.top + 2.0);
                *r != b && !nested(r) && between && r.left < s.right && r.right > s.left
            }) || bars.iter().any(|r| {
                // between this bar and another one, but nearer the other
                // (the numerator of a fraction stacked right below)
                let gap_b = (s.top - b.bottom).max(b.top - s.bottom);
                let gap_r = (s.top - r.bottom).max(r.top - s.bottom);
                let same_side = if s.top >= b.bottom - 2.0 { r.top > b.bottom } else { r.bottom < b.top };
                // ... and the other bar is a fraction (has its own part
                // beyond it), not an overline
                let far = |t: &MSpan| {
                    let c = (t.left + t.right) / 2.0;
                    let beyond = if r.top > b.bottom { t.top >= r.bottom - 2.0 && t.top - r.bottom <= 0.8 * size } else { t.bottom <= r.top + 2.0 && r.top - t.bottom <= 0.8 * size };
                    c >= r.left - 2.0 && c <= r.right + 2.0 && beyond
                };
                // ... unless it is a script on a row that is nearer this bar
                let row_near_b = spans.iter().any(|u| {
                    let (ub, ur) = ((u.top - b.bottom).max(b.top - u.bottom), (u.top - r.bottom).max(r.top - u.bottom));
                    within(u) && ub < ur && u.top < s.bottom - 1.0 && u.bottom > s.top + 1.0
                });
                *r != b && !nested(r) && same_side && r.left < s.right && r.right > s.left && gap_r < gap_b && !row_near_b && spans.iter().any(far)
            })
        };
        // A small script hanging on a symbol outside the bar (the ′ of /e′
        // right before a fraction) belongs to that symbol.
        let hangs_outside = |s: &MSpan| {
            s.size < 0.9 * size
                && spans.iter().any(|t| {
                    let c = (t.left + t.right) / 2.0;
                    t.size >= 0.9 * size
                        && (c < b.left || c > b.right)
                        && (s.left - t.right).abs() <= 1.5
                        && s.top < t.bottom
                        && s.bottom > t.top
                })
        };
        let above: Vec<usize> = above.into_iter().filter(|&i| !blocked(&spans[i]) && !hangs_outside(&spans[i])).collect();
        let below: Vec<usize> = below.into_iter().filter(|&i| !blocked(&spans[i]) && !hangs_outside(&spans[i])).collect();
        // One row of text (plus its scripts) on each side, unless a nested
        // fraction's bar makes room for more.
        let reach_up = bars
            .iter()
            .filter(|r| nested(r) && r.bottom < b.top && r.top >= b.top - 2.5 * size)
            .map(|r| b.top - r.top + 1.7 * size)
            .fold(1.7 * size, f32::max);
        let reach_down = bars
            .iter()
            .filter(|r| nested(r) && r.top > b.bottom && r.bottom <= b.bottom + 2.5 * size)
            .map(|r| r.bottom - b.bottom + 1.7 * size)
            .fold(1.7 * size, f32::max);
        // ... and as far as a tall bracket standing right on the bar reaches
        // (plus room for its exponent)
        let tall_reach = |up: bool| {
            above
                .iter()
                .chain(&below)
                .map(|&i| &spans[i])
                .filter(|s| s.cmex() && s.height() > 1.5 * size && within(s))
                .filter(|s| if up { (b.top - s.bottom).abs() <= 0.5 * size } else { (s.top - b.bottom).abs() <= 0.5 * size })
                .map(|s| if up { b.top - s.top } else { s.bottom - b.bottom } + 0.7 * size)
                .fold(0.0f32, f32::max)
        };
        let (reach_up, reach_down) = (reach_up.max(tall_reach(true)), reach_down.max(tall_reach(false)));
        let above: Vec<usize> = above.into_iter().filter(|&i| b.top - spans[i].top <= reach_up).collect();
        let below: Vec<usize> = below.into_iter().filter(|&i| spans[i].bottom - b.bottom <= reach_down).collect();
        let num = stack(&spans, &above, |s| b.top - s.bottom, size);
        let den = stack(&spans, &below, |s| s.top - b.bottom, size);
        if num.is_empty() || den.is_empty() {
            continue;
        }
        used[bi] = true;
        let nbars = inner(&mut used, true);
        let dbars = inner(&mut used, false);
        let n: Vec<MSpan> = num.iter().map(|&i| spans[i].clone()).collect();
        let d: Vec<MSpan> = den.iter().map(|&i| spans[i].clone()).collect();
        // small numbers in a line of normal size: a text-style fraction (½)
        let small = n.iter().chain(&d).all(|s| s.size < 0.85 * size);
        // (its own size and height, for the box of one that is an exponent)
        let own = n.iter().chain(&d).map(|s| s.size).fold(0.0f32, f32::max);
        let (own_top, own_bottom) = (n.iter().map(|s| s.top).fold(f32::MAX, f32::min), d.iter().map(|s| s.bottom).fold(f32::MIN, f32::max));
        let t = format!(
            "\\{}{{{}}}{{{}}}",
            if small { "tfrac" } else { "frac" },
            build_at(&resolve_bars(n, &nbars, depth + 1), depth + 1),
            build_at(&resolve_bars(d, &dbars, depth + 1), depth + 1)
        );
        let y = (b.top + b.bottom) / 2.0;
        // A small fraction set above or below the line (the ½ of a square
        // root written as a power) is an exponent or an index: it keeps
        // its small size and its place, and is attached like any other.
        // One on the line, at the height of the letters beside it, is a
        // symbol of the line ("½ m v²").
        let beside = spans
            .iter()
            .enumerate()
            // (not the dot of a product, which is raised itself)
            .filter(|(i, s)| !num.contains(i) && !den.contains(i) && !s.cmex() && s.size >= 0.9 * size && (s.cy() - y).abs() <= 1.2 * size && s.text.trim() != ".")
            .min_by(|a, c| (a.1.left - b.right).max(b.left - a.1.right).total_cmp(&(c.1.left - b.right).max(b.left - c.1.right)))
            .map(|(_, s)| s.cy());
        let script = small && beside.is_some_and(|cy| (y - cy).abs() > 0.3 * size);
        // (among the letters of an exponent, "−½x²", it is of their size
        // and on their line: a symbol of the exponent, not its index)
        let level = spans
            .iter()
            .enumerate()
            .filter(|(i, s)| !num.contains(i) && !den.contains(i) && !s.cmex() && s.atom.is_none() && s.size < 0.9 * size && s.size > own && s.top <= y && s.bottom >= y)
            .filter(|(_, s)| (s.left - b.right).max(b.left - s.right) <= if script { 2.0 } else { 0.6 } * size && (s.cy() - y).abs() <= 0.3 * s.size)
            .map(|(_, s)| s.size)
            .fold(0.0f32, f32::max);
        // (small letters right beside it at its height: it is one of them,
        // however little the exponent is raised in a line of text)
        let script = small && (script || level > 0.0);
        let (top, bottom, size) = if script && level > 0.0 {
            (y - level / 2.0, y + level / 2.0, level)
        } else if script {
            (own_top, own_bottom, own)
        } else {
            (y - size / 2.0, y + size / 2.0, size)
        };
        let atom = MSpan {
            text: String::new(),
            left: b.left,
            right: b.right,
            top,
            bottom,
            size,
            family: String::new(),
            math: true,
            over: false,
            bold: false,
            atom: Some(t),
        };
        let mut k = 0;
        spans.retain(|_| {
            k += 1;
            !num.contains(&(k - 1)) && !den.contains(&(k - 1))
        });
        spans.push(atom);
    }
    spans
}

/// An arrow drawn over a letter that was not found under it (it stands a
/// little to the right of a small or sloping letter) goes over the letter
/// before it: "x ⃗" is `\vec{x}`, "\nabla ⃗" `\vec{\nabla}`.
fn late_arrows(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if c != '\u{20d7}' {
            out.push(c);
            continue;
        }
        let kept = out.trim_end().len();
        let before = &out[..kept];
        // the letter, or the command, the arrow comes after
        let start = match before.chars().next_back() {
            Some(l) if l.is_ascii_alphabetic() => {
                let word = before.len() - before.chars().rev().take_while(char::is_ascii_alphabetic).count();
                if before[..word].ends_with('\\') { Some(word - 1) } else { Some(before.len() - 1) }
            }
            Some(l) if l.is_alphanumeric() => Some(before.len() - l.len_utf8()),
            _ => None,
        };
        match start {
            Some(start) => {
                let letter = out[start..kept].to_string();
                out.truncate(start);
                out.push_str(&format!("\\vec{{{letter}}}"));
            }
            None => out.push(c),
        }
    }
    out
}

fn tidy(s: &str) -> String {
    let s = s.split_whitespace().collect::<Vec<_>>().join(" ");
    let s = if s.contains('\u{20d7}') { late_arrows(&s) } else { s };
    s.replace("\\not =", "\\neq").replace("\\not=", "\\neq").replace("\\not{=}", "\\neq")
}

fn linear(spans: &[MSpan]) -> String {
    let mut v: Vec<&MSpan> = spans.iter().collect();
    v.sort_by(|a, b| a.left.total_cmp(&b.left));
    v.iter()
        .map(|s| tex(s))
        .filter(|t| !t.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Role {
    Base,
    Sup,
    Sub,
    Above,
    Below,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Kind {
    Atom,
    Sup,
    Sub,
}

struct Item {
    left: f32,
    right: f32,
    tex: String,
    kind: Kind,
    /// for scripts: the spans they were built from
    src: Vec<usize>,
}

fn mean_cy(spans: &[MSpan], idx: &[usize]) -> f32 {
    idx.iter().map(|&i| spans[i].cy()).sum::<f32>() / idx.len().max(1) as f32
}

fn is_relation(t: &str) -> bool {
    t.contains(['=', '≡', '≈', '<', '>', '≤', '≥', '∼', '≃'])
}

/// Group span indices into clusters of horizontally touching spans.
fn clusters(spans: &[MSpan], mut idx: Vec<usize>, tol: f32) -> Vec<Vec<usize>> {
    idx.sort_by(|&a, &b| spans[a].left.total_cmp(&spans[b].left));
    let mut out: Vec<(f32, Vec<usize>)> = Vec::new();
    for i in idx {
        match out.last_mut() {
            Some((right, c)) if spans[i].left <= *right + tol => {
                *right = right.max(spans[i].right);
                c.push(i);
            }
            _ => out.push((spans[i].right, vec![i])),
        }
    }
    out.into_iter().map(|(_, c)| c).collect()
}

fn range(spans: &[MSpan], c: &[usize]) -> (f32, f32) {
    let l = c.iter().map(|&i| spans[i].left).fold(f32::MAX, f32::min);
    let r = c.iter().map(|&i| spans[i].right).fold(f32::MIN, f32::max);
    (l, r)
}

fn overlaps(a: (f32, f32), b: (f32, f32)) -> bool {
    a.0 < b.1 && b.0 < a.1
}

fn subset(spans: &[MSpan], idx: &[usize]) -> Vec<MSpan> {
    idx.iter().map(|&i| spans[i].clone()).collect()
}

/// Split a span whose text has spaces ("0 1") into one span per word,
/// spreading them over the span's width.
fn split_words(s: &MSpan) -> Vec<MSpan> {
    let chars: Vec<char> = s.text.chars().collect();
    if !chars.iter().any(|c| c.is_whitespace()) {
        return vec![s.clone()];
    }
    let n = chars.len() as f32;
    let w = s.right - s.left;
    let mut out = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        }
        let start = i;
        while i < chars.len() && !chars[i].is_whitespace() {
            i += 1;
        }
        let mut t = s.clone();
        t.text = chars[start..i].iter().collect();
        t.left = s.left + w * start as f32 / n;
        t.right = s.left + w * i as f32 / n;
        out.push(t);
    }
    out
}

/// Find tall brackets (empty glyphs of the big-symbol font, stacked pieces
/// merged) and turn what is between each left/right pair into a matrix
/// (several rows in aligned columns) or a parenthesised group.
fn take_brackets(spans: &[MSpan], depth: usize) -> (Vec<MSpan>, Vec<Item>) {
    // Bracket glyphs: from the SVG they come with their character; pieces
    // pdftohtml gave as empty glyphs have none (shape unknown).
    let is_piece = |s: &MSpan| {
        s.cmex() && s.atom.is_none() && (s.text.trim().is_empty() || BRACKETS.contains(s.text.trim()))
    };
    let mut delims: Vec<Delim> = Vec::new();
    let mut pieces: Vec<&MSpan> = spans.iter().filter(|s| is_piece(s)).collect();
    pieces.sort_by(|a, b| a.top.total_cmp(&b.top));
    for p in pieces {
        let kind = p.text.trim().chars().next();
        if kind.is_some() {
            delims.push(Delim { left: p.left, right: p.right, top: p.top, bottom: p.bottom, kind });
            continue;
        }
        // empty pieces of one tall bracket: same x, stacked close together
        let h = p.bottom - p.top;
        match delims
            .iter_mut()
            .find(|d| d.kind.is_none() && p.left < d.right && p.right > d.left && p.top <= d.bottom + 1.5 * h)
        {
            Some(d) => {
                d.left = d.left.min(p.left);
                d.right = d.right.max(p.right);
                d.bottom = d.bottom.max(p.bottom);
            }
            None => delims.push(Delim { left: p.left, right: p.right, top: p.top, bottom: p.bottom, kind: None }),
        }
    }
    delims.sort_by(|a, b| a.left.total_cmp(&b.left));
    let mut rest: Vec<MSpan> = spans.iter().filter(|s| !is_piece(s)).cloned().collect();
    let size = rest.iter().map(|s| s.size).fold(0.0f32, f32::max).max(1.0);
    let same_height = |a: &Delim, b: &Delim| b.top < a.bottom + size && b.bottom > a.top - size;

    // Which way does each bracket face? Known for ( [ { ⟨ and ) ] } ⟩.
    // For | and for shapeless pieces: two at the same height with formula
    // between them (and no "=") are a pair; otherwise a bracket hugs the
    // text on its inner side.
    let mut side: Vec<Option<bool>> = delims.iter().map(|d| d.kind.and_then(opens_kind)).collect();
    for k in 0..delims.len() {
        if side[k].is_some() {
            continue;
        }
        let d0 = delims[k];
        let m = (k + 1..delims.len()).find(|&m| {
            let d1 = delims[m];
            let between: Vec<&MSpan> = rest
                .iter()
                .filter(|s| s.left >= d0.right - 1.0 && s.right <= d1.left + 1.0 && (s.cy() - (d0.top + d0.bottom) / 2.0).abs() <= 3.0 * size)
                .collect();
            let relation = between.iter().any(|s| s.size >= 0.9 * size && is_relation(&s.text));
            side[m].is_none() && same_height(&d0, &d1) && !between.is_empty() && !relation
        });
        if let Some(m) = m {
            side[k] = Some(true);
            side[m] = Some(false);
        }
    }
    let opens: Vec<bool> = (0..delims.len())
        .map(|k| {
            side[k].unwrap_or_else(|| {
                let d = delims[k];
                let mid = (d.top + d.bottom) / 2.0;
                let near = |s: &&MSpan| (s.cy() - mid).abs() <= 2.0 * size;
                let gap_left = rest.iter().filter(near).filter(|s| s.right <= d.left + 1.0).map(|s| d.left - s.right).fold(f32::MAX, f32::min);
                let gap_right = rest.iter().filter(near).filter(|s| s.left >= d.right - 1.0).map(|s| s.left - d.right).fold(f32::MAX, f32::min);
                gap_right < gap_left
            })
        })
        .collect();

    // Match them like brackets in text: each closing bracket closes the
    // most recent open one at its height.
    let mut stack: Vec<usize> = Vec::new();
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    let mut lone: Vec<usize> = Vec::new();
    for k in 0..delims.len() {
        if opens[k] {
            stack.push(k);
        } else if let Some(pos) = stack.iter().rposition(|&o| same_height(&delims[o], &delims[k])) {
            pairs.push((stack.remove(pos), k));
        } else {
            lone.push(k);
        }
    }
    lone.extend(stack);

    // Inner pairs first; each group becomes one piece of the formula, so an
    // outer pair contains the inner group.
    pairs.sort_by(|a, b| (delims[a.1].left - delims[a.0].left).total_cmp(&(delims[b.1].left - delims[b.0].left)));
    let piece = |left: f32, right: f32, mid: f32, tex: String| MSpan {
        text: String::new(),
        left,
        right,
        top: mid - size / 2.0,
        bottom: mid + size / 2.0,
        size,
        family: String::new(),
        math: true,
        over: false,
        bold: false,
        atom: Some(tex),
    };
    for (o, c) in pairs {
        let (d0, d1) = (delims[o], delims[c]);
        // boxes of shapeless pieces mark only the tops of the pieces; the
        // lowest piece reaches further down than its box
        let (top, bottom) = (d0.top.min(d1.top), d0.bottom.max(d1.bottom));
        // (a big ∫ or a slanted "/p" right after a bracket reaches back over
        // its edge: the middle of a span counts)
        let (inside, outside): (Vec<MSpan>, Vec<MSpan>) = rest.into_iter().partition(|s| {
            let c = (s.left + s.right) / 2.0;
            let (l, r) = (s.left.max(c.min(d0.right)), s.right.min(c.max(d1.left)));
            l >= d0.right - 1.0
                && r <= d1.left + 1.0
                && s.cy() >= top - size
                && s.cy() <= (bottom + 1.5 * size).max((top + bottom) / 2.0 + 3.0 * size)
        });
        rest = outside;
        let open = d0.kind.unwrap_or('(');
        let close = d1.kind.unwrap_or(match open {
            '[' => ']',
            '{' => '}',
            '⟨' => '⟩',
            '|' => '|',
            _ => ')',
        });
        let typed = d0.kind.is_some() && d1.kind.is_some();
        let tex = matrix(&inside, depth, open, typed).unwrap_or_else(|| {
            format!("\\left{} {} \\right{}", delim_tex(open), build_at(&inside, depth + 1), delim_tex(close))
        });
        if inside.is_empty() {
            continue; // nothing between them: nothing to write
        }
        // A bracket read from its outline has its true height, centred on
        // the formula's main line; shapeless pieces only mark their tops,
        // so then the middle of the contents is used.
        let mid = match (d0.kind, d1.kind) {
            (Some(_), _) => (d0.top + d0.bottom) / 2.0,
            (_, Some(_)) => (d1.top + d1.bottom) / 2.0,
            _ => inside.iter().map(|s| s.cy()).sum::<f32>() / inside.len() as f32,
        };
        rest.push(piece(d0.left, d1.right, mid, tex));
    }
    // A lone bracket (its partner was lost): marked, and its partner is put
    // back when the formula is written out (see `finish`).
    for k in lone {
        let d = delims[k];
        // A lone opening brace before rows of cases: "Θ(x) = { +1 for x>0
        // / 0 for x<0" (everything to its right at its height).
        if d.kind == Some('{') && opens[k] {
            let (inside, outside): (Vec<MSpan>, Vec<MSpan>) = rest.iter().cloned().partition(|s| {
                s.left >= d.right - 1.0 && s.cy() >= d.top - 0.3 * size && s.cy() <= d.bottom + 0.3 * size
            });
            if let Some(tex) = cases(&inside, depth) {
                rest = outside;
                rest.push(piece(d.left, d.right.max(inside.iter().map(|s| s.right).fold(0.0, f32::max)), (d.top + d.bottom) / 2.0, tex));
                continue;
            }
        }
        // A lone closing brace after rows: they are grouped by it.
        if d.kind == Some('}') && !opens[k] {
            let (inside, outside): (Vec<MSpan>, Vec<MSpan>) = rest.iter().cloned().partition(|s| {
                s.right <= d.left + 1.0 && s.cy() >= d.top - 0.3 * size && s.cy() <= d.bottom + 0.3 * size
            });
            if let Some(tex) = brace_group(&inside, depth) {
                rest = outside;
                let left = inside.iter().map(|s| s.left).fold(d.left, f32::min);
                rest.push(piece(left, d.right, (d.top + d.bottom) / 2.0, tex));
                continue;
            }
        }
        let c = d.kind.unwrap_or(if opens[k] { '(' } else { ')' });
        let c = if d.kind == Some('|') { '|' } else { c };
        rest.push(piece(d.left, d.right, (d.top + d.bottom) / 2.0, format!("{LONE}{c}")));
    }
    (rest, Vec::new())
}

/// A bracket found in a formula (`kind` is None for a shapeless piece).
#[derive(Clone, Copy)]
struct Delim {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
    kind: Option<char>,
}

const BRACKETS: &str = "()[]{}|⟨⟩";

/// Some(true) for an opening bracket, Some(false) for a closing one, None
/// for | (could be either).
fn opens_kind(c: char) -> Option<bool> {
    match c {
        '(' | '[' | '{' | '⟨' => Some(true),
        ')' | ']' | '}' | '⟩' => Some(false),
        _ => None,
    }
}

fn delim_tex(c: char) -> &'static str {
    match c {
        '(' => "(",
        ')' => ")",
        '[' => "[",
        ']' => "]",
        '{' => "\\{",
        '}' => "\\}",
        '⟨' => "\\langle",
        '⟩' => "\\rangle",
        _ => "|",
    }
}

fn partner(c: char) -> char {
    match c {
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        '{' => '}',
        '}' => '{',
        '⟨' => '⟩',
        '⟩' => '⟨',
        _ => '|',
    }
}

const LONE: char = '\u{1}';

thread_local! {
    /// Set while the lines of a formula of several lines are built (see
    /// [`finish`]).
    static MULTILINE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Rows and columns of a matrix, if `spans` look like one.
fn matrix(spans: &[MSpan], depth: usize, open: char, typed: bool) -> Option<String> {
    let size = spans.iter().map(|s| s.size).fold(0.0f32, f32::max).max(1.0);
    // rows of full-size entries
    let cells: Vec<MSpan> = spans.iter().flat_map(split_words).collect();
    let mut cells = small_blocks(cells, size, depth);
    cells.sort_by(|a, b| a.cy().total_cmp(&b.cy()));
    // Rows come from full-size entries only (big operators like Σ report
    // their box at the top, so they don't count); small pieces (exponents,
    // limits) then join the nearest row.
    let (big, small): (Vec<MSpan>, Vec<MSpan>) =
        cells.into_iter().partition(|c| c.size >= 0.9 * size && !c.cmex());
    let mut rows: Vec<Vec<MSpan>> = Vec::new();
    for c in big {
        match rows.last_mut() {
            Some(r) if (c.cy() - r[0].cy()).abs() <= 0.4 * size => r.push(c),
            _ => rows.push(vec![c]),
        }
    }
    if rows.is_empty() {
        return None;
    }
    for c in small {
        let k = (0..rows.len())
            .min_by(|&a, &b| (rows[a][0].cy() - c.cy()).abs().total_cmp(&(rows[b][0].cy() - c.cy()).abs()))
            .unwrap_or(0);
        rows[k].push(c);
    }
    if rows.len() < 2 {
        return None;
    }
    // columns: clusters of x positions shared by all rows
    let mut xs: Vec<(f32, f32)> = rows
        .iter()
        .flat_map(|r| clusters_of(r, 0.5 * size))
        .collect();
    xs.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut cols: Vec<(f32, f32)> = Vec::new();
    for (l, r) in xs {
        match cols.last_mut() {
            Some(c) if l <= c.1 => c.1 = c.1.max(r),
            _ => cols.push((l, r)),
        }
    }
    // Two rows with one column is as likely a fraction as a vector, unless
    // the bracket was read from its outline (then fraction bars were also
    // read and fractions are already resolved).
    if rows.len() == 2 && cols.len() < 2 && !typed {
        return None;
    }
    // In a matrix every column has an entry in (nearly) every row. Fractions
    // inside big brackets do not: "i" on the middle row, "p·x" above "ħ".
    let filled = |&(l, r): &(f32, f32)| {
        rows.iter()
            .filter(|row| row.iter().any(|s| s.left < r && s.right > l))
            .count()
    };
    if cols.iter().any(|c| filled(c) * 4 < rows.len() * 3) {
        return None;
    }
    let body: Vec<String> = rows
        .iter()
        .map(|r| {
            let cells = cols
                .iter()
                .map(|&(l, rr)| {
                    let cell: Vec<MSpan> = r
                        .iter()
                        .filter(|s| {
                            let c = (s.left + s.right) / 2.0;
                            c >= l && c <= rr
                        })
                        .cloned()
                        .collect();
                    build_at(&cell, depth + 1)
                })
                .collect::<Vec<_>>();
            cells.join(" & ")
        })
        .collect();
    let env = match open {
        ' ' => "matrix",
        '[' => "bmatrix",
        '|' => "vmatrix",
        '{' => "Bmatrix",
        _ => "pmatrix",
    };
    Some(format!("\\begin{{{env}}} {} \\end{{{env}}}", body.join(" \\\\ ")))
}

/// Blocks of a block matrix: entries in a smaller font, standing free
/// (not scripts touching a symbol), stacked in rows and columns with no
/// full-size symbol among them, like the 2×2 blocks of
/// β = ((1 0; 0 1) 0; 0 (−1 0; 0 −1)). Each block becomes one ready-made
/// piece the size of the outer entries, so the outer matrix sees one cell.
fn small_blocks(cells: Vec<MSpan>, size: f32, depth: usize) -> Vec<MSpan> {
    let small = |s: &MSpan| s.size < 0.8 * size && !s.cmex() && s.atom.is_none() && !s.text.trim().is_empty();
    let big: Vec<&MSpan> = cells.iter().filter(|s| !small(s)).collect();
    let vgap = |a: &MSpan, b: &MSpan| (a.top - b.bottom).max(b.top - a.bottom);
    let hgap = |a: &MSpan, b: &MSpan| (a.left - b.right).max(b.left - a.right);
    // a script touches the symbol it belongs to
    let free = |s: &MSpan| !big.iter().any(|b| vgap(s, b) < 0.0 && hgap(s, b) <= 1.5);
    let cand: Vec<usize> = (0..cells.len()).filter(|&i| small(&cells[i]) && free(&cells[i])).collect();
    if cand.len() < 3 {
        return cells;
    }
    // group: side by side in a row, or one above the other in a column
    let mut group: Vec<usize> = (0..cand.len()).collect();
    fn root(g: &mut [usize], mut k: usize) -> usize {
        while g[k] != k {
            g[k] = g[g[k]];
            k = g[k];
        }
        k
    }
    for a in 0..cand.len() {
        for b in a + 1..cand.len() {
            let (x, y) = (&cells[cand[a]], &cells[cand[b]]);
            let h = x.size.max(y.size);
            let same_row = vgap(x, y) < -0.3 * h && hgap(x, y) <= 3.0 * h;
            let same_col = hgap(x, y) < 0.0 && vgap(x, y) <= h;
            if same_row || same_col {
                let (ra, rb) = (root(&mut group, a), root(&mut group, b));
                group[ra] = rb;
            }
        }
    }
    let mut clusters: std::collections::BTreeMap<usize, Vec<usize>> = std::collections::BTreeMap::new();
    for (k, &c) in cand.iter().enumerate() {
        let r = root(&mut group, k);
        clusters.entry(r).or_default().push(c);
    }
    let mut used = vec![false; cells.len()];
    let mut blocks: Vec<MSpan> = Vec::new();
    for idx in clusters.into_values().filter(|c| c.len() >= 3) {
        let part: Vec<MSpan> = idx.iter().map(|&i| cells[i].clone()).collect();
        let (l, r) = (part.iter().map(|s| s.left).fold(f32::MAX, f32::min), part.iter().map(|s| s.right).fold(f32::MIN, f32::max));
        let (t, b) = (part.iter().map(|s| s.top).fold(f32::MAX, f32::min), part.iter().map(|s| s.bottom).fold(f32::MIN, f32::max));
        // nothing full-size inside the block (that would be limits of a Σ,
        // a fraction's small parts...)
        if big.iter().any(|s| s.cy() > t && s.cy() < b && (s.left + s.right) / 2.0 > l && (s.left + s.right) / 2.0 < r) {
            continue;
        }
        let Some(tex) = matrix(&part, depth + 1, ' ', true) else { continue };
        for &i in &idx {
            used[i] = true;
        }
        blocks.push(MSpan {
            text: String::new(),
            left: l,
            right: r,
            top: t,
            bottom: b,
            size,
            family: String::new(),
            math: true,
            over: false,
            bold: false,
            atom: Some(tex),
        });
    }
    if blocks.is_empty() {
        return cells;
    }
    cells.into_iter().enumerate().filter(|(i, _)| !used[*i]).map(|(_, s)| s).chain(blocks).collect()
}

/// Rows of cases after a lone "{": each row split into the value and the
/// condition at the widest gap that all rows share.
/// Lines of text stacked beside a tall brace: rows of full-size symbols,
/// with small ones (scripts) joining the nearest row.
fn text_rows(spans: &[MSpan]) -> Vec<Vec<MSpan>> {
    let size = spans.iter().filter(|s| !s.cmex()).map(|s| s.size).fold(0.0f32, f32::max).max(1.0);
    let mut sorted: Vec<MSpan> = spans.to_vec();
    sorted.sort_by(|a, b| a.cy().total_cmp(&b.cy()));
    let (big, small): (Vec<MSpan>, Vec<MSpan>) = sorted.into_iter().partition(|c| c.size >= 0.9 * size && !c.cmex());
    let mut rows: Vec<Vec<MSpan>> = Vec::new();
    for c in big {
        match rows.last_mut() {
            Some(r) if (c.cy() - r[0].cy()).abs() <= 0.4 * size => r.push(c),
            _ => rows.push(vec![c]),
        }
    }
    if rows.is_empty() {
        return rows;
    }
    for c in small {
        let k = (0..rows.len())
            .min_by(|&a, &b| (rows[a][0].cy() - c.cy()).abs().total_cmp(&(rows[b][0].cy() - c.cy()).abs()))
            .unwrap_or(0);
        rows[k].push(c);
    }
    rows
}

/// Rows grouped by a "}" on their right: "ūu = +1 for electron states /
/// = −1 for positron states }", lined up at their "=".
fn brace_group(spans: &[MSpan], depth: usize) -> Option<String> {
    let rows = text_rows(spans);
    if rows.len() < 2 {
        return None;
    }
    let body: Vec<String> = rows
        .iter()
        .map(|r| {
            let t = build_at(r, depth + 1);
            if let Some(rest) = t.strip_prefix("= ") {
                format!("&= {rest}")
            } else if let Some(p) = t.find(" = ") {
                format!("{} &= {}", &t[..p], &t[p + 3..])
            } else {
                format!("& {t}")
            }
        })
        .collect();
    Some(format!("\\left. \\begin{{aligned}} {} \\end{{aligned}} \\right\\}}", body.join(" \\\\ ")))
}

fn cases(spans: &[MSpan], depth: usize) -> Option<String> {
    let size = spans.iter().filter(|s| !s.cmex()).map(|s| s.size).fold(0.0f32, f32::max).max(1.0);
    let rows = text_rows(spans);
    if rows.len() < 2 {
        return None;
    }
    // free stretches of x shared by all rows (between their words)
    let gaps = |row: &[MSpan]| -> Vec<(f32, f32)> {
        let words: Vec<MSpan> = row.iter().flat_map(split_words).collect();
        let ext = clusters_of(&words, 1.0);
        ext.windows(2).map(|w| (w[0].1, w[1].0)).collect()
    };
    let mut common = gaps(&rows[0]);
    for r in &rows[1..] {
        let g = gaps(r);
        common = common
            .iter()
            .flat_map(|&(a, b)| g.iter().map(move |&(c, d)| (a.max(c), b.min(d))))
            .filter(|(a, b)| b > a)
            .collect();
    }
    let (a, b) = common.into_iter().max_by(|x, y| (x.1 - x.0).total_cmp(&(y.1 - y.0)))?;
    if b - a < 0.5 * size {
        return None;
    }
    let cut = (a + b) / 2.0;
    let body: Vec<String> = rows
        .iter()
        .map(|r| {
            let pieces: Vec<MSpan> = r
                .iter()
                .flat_map(|s| if s.left < cut && s.right > cut { split_words(s) } else { vec![s.clone()] })
                .collect();
            let (l, rr): (Vec<MSpan>, Vec<MSpan>) = pieces.into_iter().partition(|s| (s.left + s.right) / 2.0 < cut);
            format!("{} & {}", build_at(&l, depth + 1), build_at(&rr, depth + 1))
        })
        .collect();
    Some(format!("\\begin{{cases}} {} \\end{{cases}}", body.join(" \\\\ ")))
}

/// Horizontal extents of groups of touching spans in one row.
fn clusters_of(row: &[MSpan], tol: f32) -> Vec<(f32, f32)> {
    let mut v: Vec<&MSpan> = row.iter().collect();
    v.sort_by(|a, b| a.left.total_cmp(&b.left));
    let mut out: Vec<(f32, f32)> = Vec::new();
    for s in v {
        match out.last_mut() {
            Some(c) if s.left <= c.1 + tol => c.1 = c.1.max(s.right),
            _ => out.push((s.left, s.right)),
        }
    }
    out
}

fn build_at(spans: &[MSpan], depth: usize) -> String {
    if spans.is_empty() {
        return String::new();
    }
    if depth > 6 {
        return linear(spans);
    }
    let (owned, bracket_items) = take_brackets(spans, depth);
    let spans = &owned[..];
    if spans.is_empty() || (spans.len() == 1 && bracket_items.is_empty()) {
        let mut v: Vec<String> = bracket_items.into_iter().map(|i| i.tex).collect();
        v.push(linear(spans));
        return finish(v);
    }
    let max = spans
        .iter()
        .filter(|s| !s.cmex())
        .map(|s| s.size)
        .fold(0.0f32, f32::max);
    if max <= 0.0 {
        let mut v: Vec<String> = bracket_items.into_iter().map(|i| i.tex).collect();
        v.push(linear(spans));
        return finish(v);
    }

    // 1. Rows of full-size spans, by vertical centre.
    let mut mains: Vec<usize> = (0..spans.len())
        .filter(|&i| !spans[i].cmex() && spans[i].size >= 0.9 * max)
        .collect();
    mains.sort_by(|&a, &b| spans[a].cy().total_cmp(&spans[b].cy()));
    let mut rows: Vec<Vec<usize>> = Vec::new();
    for i in mains {
        let c = spans[i].cy();
        match rows.last_mut() {
            Some(r) if (c - mean_cy(spans, r)).abs() <= 0.3 * max => r.push(i),
            _ => rows.push(vec![i]),
        }
    }
    // The main row: the one with a relation sign, else the busiest one.
    // The main row: the one with a relation sign, else one with rows both
    // above and below it (the fraction bar level), else the busiest one.
    let nrows = rows.len();
    let Some(row) = rows.iter().enumerate().max_by_key(|(k, r)| {
        let rel = r.iter().any(|&i| is_relation(&spans[i].text));
        let middle = *k > 0 && *k + 1 < nrows;
        (rel as usize) * 1000 + (middle as usize) * 500 + r.len()
    }).map(|(_, r)| r) else {
        let mut v: Vec<String> = bracket_items.into_iter().map(|i| i.tex).collect();
        v.push(linear(spans));
        return finish(v);
    };
    let y0 = mean_cy(spans, row);
    let mut hs: Vec<f32> = row.iter().map(|&i| spans[i].height()).collect();
    hs.sort_by(f32::total_cmp);
    let h = hs[hs.len() / 2].max(1.0);

    // 2. Classify every span relative to the main row.
    let mut role: Vec<Role> = spans
        .iter()
        .map(|s| {
            let d = s.cy() - y0;
            // (a big symbol set small, above or below the line: the integral
            // sign inside an exponent is part of the exponent)
            let in_script = s.cmex() && s.size < 0.9 * max && d.abs() > 0.3 * h && !s.text.trim().is_empty();
            if (s.cmex() && !in_script) || (s.size >= 0.9 * max && d.abs() <= 0.3 * h) {
                Role::Base
            } else if d < 0.0 {
                Role::Above
            } else {
                Role::Below
            }
        })
        .collect();

    // Small spans right next to a main-row symbol are its scripts.
    let bases: Vec<usize> = (0..spans.len()).filter(|&i| role[i] == Role::Base).collect();
    let near_row = |s: &MSpan| (s.cy() - y0).abs() <= 0.95 * h;
    for i in 0..spans.len() {
        let s = &spans[i];
        if role[i] == Role::Base || s.size >= 0.9 * max || !near_row(s) {
            continue;
        }
        let adjacent = bases.iter().any(|&b| {
            let gap = s.left - spans[b].right;
            gap >= -0.2 * max && gap <= 0.45 * max && big_op(&spans[b]).is_none()
        });
        if adjacent {
            let d = s.cy() - y0;
            let prime = matches!(s.text.trim(), "′" | "″" | "‴");
            role[i] = if prime {
                Role::Sup // k′ in a subscript sits level with the k
            } else if d.abs() < 0.1 * h {
                Role::Base
            } else if d < 0.0 {
                Role::Sup
            } else {
                Role::Sub
            };
        }
    }
    // A small span stacked exactly over/under a script is a script too
    // (x^a_b, or a little ½).
    for i in 0..spans.len() {
        let s = &spans[i];
        if !matches!(role[i], Role::Above | Role::Below) || s.size >= 0.9 * max || !near_row(s) {
            continue;
        }
        let stacked = (0..spans.len()).any(|j| {
            matches!(role[j], Role::Sup | Role::Sub)
                && overlaps((s.left, s.right), (spans[j].left, spans[j].right))
        });
        if stacked {
            role[i] = if s.cy() < y0 { Role::Sup } else { Role::Sub };
        }
    }

    // A small span continuing a script to its right is part of it: the
    // whole exponent of ã^{(nᵢ−1)}_{kᵢλ}, not just its first glyphs.
    loop {
        let mut changed = false;
        for i in 0..spans.len() {
            let s = &spans[i];
            if !matches!(role[i], Role::Above | Role::Below) || s.size >= 0.9 * max || !near_row(s) {
                continue;
            }
            let follows = (0..spans.len()).find(|&j| {
                matches!(role[j], Role::Sup | Role::Sub) && {
                    let gap = s.left - spans[j].right;
                    let level = (s.cy() - spans[j].cy()).abs() <= 0.3 * s.height().max(spans[j].height());
                    gap >= -0.2 * max && gap <= 0.45 * max && level
                }
            });
            if let Some(j) = follows {
                role[i] = role[j];
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    // (small symbols level with the row joined it above)
    let bases: Vec<usize> = (0..spans.len()).filter(|&i| role[i] == Role::Base).collect();

    // 3. Group what is above and below the row.
    let tol = 0.35 * max;
    let above = clusters(spans, (0..spans.len()).filter(|&i| role[i] == Role::Above).collect(), tol);
    let below = clusters(spans, (0..spans.len()).filter(|&i| role[i] == Role::Below).collect(), tol);
    let mut used_a = vec![false; above.len()];
    let mut used_b = vec![false; below.len()];
    let mut items: Vec<Item> = bracket_items;

    // Big operators take the groups above/below them as limits.
    for &b in &bases {
        let s = &spans[b];
        if let Some(op) = big_op(s) {
            let r = (s.left - 0.2 * max, s.right + 0.2 * max);
            let take = |groups: &Vec<Vec<usize>>, used: &mut Vec<bool>| -> Vec<usize> {
                let mut v = Vec::new();
                for (k, g) in groups.iter().enumerate() {
                    if !used[k] && overlaps(range(spans, g), r) {
                        used[k] = true;
                        v.extend(g);
                    }
                }
                v
            };
            let up = take(&above, &mut used_a);
            let lo = take(&below, &mut used_b);
            let mut t = op.to_string();
            if !lo.is_empty() {
                t += &format!("_{{{}}}", build_at(&subset(spans, &lo), depth + 1));
            }
            if !up.is_empty() {
                t += &format!("^{{{}}}", build_at(&subset(spans, &up), depth + 1));
            }
            // its limits may reach further right than the sign itself
            let right = up.iter().chain(&lo).map(|&i| spans[i].right).fold(s.right, f32::max);
            // (its spans, the sign first: it may turn out to stand in an exponent)
            let src: Vec<usize> = std::iter::once(b).chain(up.iter().copied()).chain(lo.iter().copied()).collect();
            items.push(Item { left: s.left, right, tex: t, kind: Kind::Atom, src });
        } else {
            let t = tex(s);
            if !t.is_empty() {
                items.push(Item { left: s.left, right: s.right, tex: t, kind: Kind::Atom, src: Vec::new() });
            }
        }
    }

    // Fractions: a group above overlapping a group below.
    for a in 0..above.len() {
        if used_a[a] {
            continue;
        }
        let ra = range(spans, &above[a]);
        let dens: Vec<usize> = (0..below.len())
            .filter(|&k| !used_b[k] && overlaps(range(spans, &below[k]), ra))
            .collect();
        if dens.is_empty() {
            continue;
        }
        let mut num: Vec<usize> = above[a].clone();
        let mut den: Vec<usize> = Vec::new();
        used_a[a] = true;
        for &k in &dens {
            used_b[k] = true;
            den.extend(&below[k]);
        }
        // other numerator groups over the same denominator
        let rd = range(spans, &den);
        for a2 in 0..above.len() {
            if !used_a[a2] && overlaps(range(spans, &above[a2]), rd) {
                used_a[a2] = true;
                num.extend(&above[a2]);
            }
        }
        let left = range(spans, &num).0.min(rd.0);
        let right = range(spans, &num).1.max(rd.1);
        items.push(Item {
            left,
            right,
            tex: format!(
                "\\frac{{{}}}{{{}}}",
                build_at(&subset(spans, &num), depth + 1),
                build_at(&subset(spans, &den), depth + 1)
            ),
            kind: Kind::Atom,
            src: Vec::new(),
        });
    }
    // Leftover groups: small ones act as scripts, big ones are written inline.
    for (groups, used, kind) in [(&above, &used_a, Kind::Sup), (&below, &used_b, Kind::Sub)] {
        for (k, g) in groups.iter().enumerate() {
            if used[k] {
                continue;
            }
            let small = g.iter().all(|&i| spans[i].size < 0.9 * max);
            let t = build_at(&subset(spans, g), depth + 1);
            if t.is_empty() {
                continue;
            }
            items.push(Item {
                left: range(spans, g).0,
                right: range(spans, g).1,
                tex: t,
                kind: if small { kind } else { Kind::Atom },
                src: g.clone(),
            });
        }
    }
    // Scripts.
    for i in 0..spans.len() {
        let kind = match role[i] {
            Role::Sup => Kind::Sup,
            Role::Sub => Kind::Sub,
            _ => continue,
        };
        let t = tex(&spans[i]);
        if !t.is_empty() {
            items.push(Item { left: spans[i].left, right: spans[i].right, tex: t, kind, src: vec![i] });
        }
    }

    // 4. Write everything out left to right, attaching scripts.
    items.sort_by(|a, b| {
        a.left
            .total_cmp(&b.left)
            .then((a.kind != Kind::Atom).cmp(&(b.kind != Kind::Atom)))
    });
    // What is set under "lim" (N → ∞, centered under the word, so it
    // starts left of it and ends right of it) is its index, all of it: not
    // an index of what stands before, nor one half of it.
    let mut k = 0;
    while k < items.len() {
        let word = items[k].tex.trim().to_string();
        if items[k].kind != Kind::Atom || !["\\lim", "\\max", "\\min", "\\sup", "\\inf"].contains(&word.as_str()) {
            k += 1;
            continue;
        }
        let under = |j: usize, l: f32, r: f32| items[j].kind == Kind::Sub && items[j].left < r + 0.6 * max && items[j].right > l - 0.6 * max;
        let mut take: Vec<usize> = (0..items.len()).filter(|&j| under(j, items[k].left + 0.6 * max, items[k].right - 0.6 * max)).collect();
        if take.is_empty() {
            k += 1;
            continue;
        }
        // (and what goes on beside it at that height)
        loop {
            let (l, r) = take.iter().fold((f32::MAX, f32::MIN), |(l, r), &j| (l.min(items[j].left), r.max(items[j].right)));
            let more: Vec<usize> = (0..items.len()).filter(|j| !take.contains(j) && under(*j, l, r)).collect();
            if more.is_empty() {
                break;
            }
            take.extend(more);
        }
        take.sort_by(|&a, &b| items[a].left.total_cmp(&items[b].left));
        let src: Vec<usize> = take.iter().flat_map(|&j| items[j].src.iter().copied()).collect();
        let index = if take.iter().all(|&j| !items[j].src.is_empty()) { build_at(&subset(spans, &src), depth + 1) } else { take.iter().map(|&j| items[j].tex.as_str()).collect::<Vec<_>>().join(" ") };
        items[k].tex = format!("{word}_{{{index}}}");
        items[k].left = take.iter().map(|&j| items[j].left).fold(items[k].left, f32::min);
        items[k].right = take.iter().map(|&j| items[j].right).fold(items[k].right, f32::max);
        let mut gone = take.clone();
        gone.sort_unstable_by(|a, b| b.cmp(a));
        for j in gone {
            items.remove(j);
            if j < k {
                k -= 1;
            }
        }
        k += 1;
    }
    items.sort_by(|a, b| a.left.total_cmp(&b.left).then((a.kind != Kind::Atom).cmp(&(b.kind != Kind::Atom))));
    let mut out: Vec<String> = Vec::new();
    let mut prev_right: Option<f32> = None;
    let mut i = 0;
    while i < items.len() {
        if items[i].kind == Kind::Atom {
            // a wide gap separates two formulas on one line
            // (not next to a big operator, which is set wide anyway, nor
            // right after a relation sign)
            let after = out.last().map_or("", |p| p.as_str());
            let big = |t: &str| ["\\sum", "\\int", "\\prod", "\\oint", "\\lim"].iter().any(|o| t.starts_with(o));
            let next = items[i].tex.split_whitespace().next().unwrap_or("");
            let rel = |t: &str| {
                matches!(t, "=" | "<" | ">" | "\\neq" | "\\leq" | "\\geq" | "\\approx" | "\\equiv" | "\\sim" | "\\simeq")
            };
            let wide_ok = !big(after)
                && !big(&items[i].tex)
                && !rel(after.split_whitespace().last().unwrap_or(""))
                && !rel(next);
            if wide_ok && prev_right.is_some_and(|r| items[i].left - r > 0.9 * max) {
                out.push("\\quad".into());
            }
            prev_right = Some(prev_right.map_or(items[i].right, |r| r.max(items[i].right)));
            out.push(items[i].tex.clone());
            i += 1;
            continue;
        }
        let mut sups: Vec<&Item> = Vec::new();
        let mut subs: Vec<&Item> = Vec::new();
        while i < items.len() && items[i].kind != Kind::Atom {
            prev_right = Some(prev_right.map_or(items[i].right, |r| r.max(items[i].right)));
            match items[i].kind {
                Kind::Sup => sups.push(&items[i]),
                _ => subs.push(&items[i]),
            }
            i += 1;
        }
        // An integral in an exponent, e^{i∫d⁴x L}: some books set its sign
        // as large as on the line, between the small "i" before it and the
        // small integrand after it. A big operator that follows an
        // exponent and is followed by more of it, at the same height, is
        // inside the exponent, and so is what follows.
        // The same for the dot of a product in an exponent, e^{−½ x·A·x}:
        // a full stop of the text font, in the size of the line, raised
        // to the middle of the small letters beside it.
        let mut folded: Vec<usize> = Vec::new();
        let mut dots: Vec<usize> = Vec::new();
        let raised_dot = |it: &Item| it.tex.trim() == "." && it.src.len() == 1 && spans[it.src[0]].cy() < y0 - 0.25 * h;
        while let Some(op) = items.get(i).filter(|it| it.kind == Kind::Atom && (it.src.first().is_some_and(|&k| big_op(&spans[k]).is_some()) || raised_dot(it))) {
            let end = i + 1 + items[i + 1..].iter().take_while(|it| it.kind != Kind::Atom).count();
            let run: Vec<usize> = items[i + 1..end].iter().flat_map(|it| it.src.iter().copied()).collect();
            // (the sign may have taken what follows it for its upper limit,
            // where its lower limit, the C of a line integral, led up to it)
            let (run, taken) = if run.is_empty() && op.src.len() > 2 { (op.src[1..].to_vec(), true) } else { (run, false) };
            let before: Vec<usize> = if folded.is_empty() { sups.iter().flat_map(|it| it.src.iter().copied()).collect() } else { folded.clone() };
            let trace = std::env::var("PDF2MD_MATHDEBUG").is_ok_and(|v| v == "3");
            if run.is_empty() || before.is_empty() || out.is_empty() || items[i + 1..end].iter().chain(sups.iter().copied()).any(|it| it.src.is_empty()) {
                if trace {
                    eprintln!("FOLD no: run {} before {} out {} op {:?} next {:?}", run.len(), before.len(), out.len(), op.tex, items.get(i + 1).map(|it| (&it.tex, it.kind == Kind::Atom, it.src.len())));
                }
                break;
            }
            // (the letters of each, without their own small indices)
            let level = |v: &[usize]| {
                let v: Vec<usize> = v.iter().copied().filter(|&k| !spans[k].cmex() && !dots.contains(&k)).collect();
                let hi = v.iter().map(|&k| spans[k].size).fold(0.0f32, f32::max);
                let main: Vec<usize> = v.into_iter().filter(|&k| spans[k].size >= 0.9 * hi).collect();
                (mean_cy(spans, &main), hi)
            };
            let ((y_before, size_before), (y_run, size_run)) = (level(&before), level(&run));
            if (size_before - size_run).abs() > 0.1 * size_before || (y_before - y_run).abs() > 0.4 * size_before {
                if trace {
                    eprintln!("FOLD no: sizes {size_before} {size_run} heights {y_before} {y_run} op {:?}", op.tex);
                }
                break;
            }
            // (not the limits of an integral on the line, which are small
            // and raised too: what follows runs on at the exponent's
            // height; nor an integral after another's limits)
            let on_level: Vec<&MSpan> = run.iter().map(|&k| &spans[k]).filter(|r| !r.cmex() && r.size >= 0.9 * size_run && (r.cy() - y_before).abs() <= 0.4 * size_before).collect();
            let width = on_level.iter().map(|r| r.right).fold(f32::MIN, f32::max) - on_level.iter().map(|r| r.left).fold(f32::MAX, f32::min);
            let after_op = out.last().is_some_and(|p| ["\\int", "\\sum", "\\prod", "\\oint"].iter().any(|o| p.trim_start().starts_with(o)));
            let dot = raised_dot(op);
            if after_op || (!dot && (on_level.len() < 2 || width < 1.5 * max)) || (dot && on_level.is_empty()) {
                break;
            }
            if dot {
                dots.push(op.src[0]);
            }
            folded = before;
            folded.extend(op.src.iter().copied());
            if !taken {
                folded.extend(run);
            }
            prev_right = items[i..end].iter().map(|it| it.right).fold(prev_right, |a, r| Some(a.map_or(r, |a| a.max(r))));
            i = end;
        }
        // Scripts of scripts (e^{ip₁·x}): pieces in two sizes are built
        // together from their spans, so the smaller ones find their base.
        let script = |v: &[&Item]| -> String {
            let src: Vec<usize> = v.iter().flat_map(|it| it.src.iter().copied()).collect();
            let sizes = src.iter().map(|&k| spans[k].size);
            let (lo, hi) = sizes.fold((f32::MAX, 0.0f32), |(a, b), z| (a.min(z), b.max(z)));
            if v.iter().all(|it| !it.src.is_empty()) && src.len() >= 2 && lo < 0.9 * hi {
                build_at(&subset(spans, &src), depth + 1)
            } else {
                v.iter().map(|it| it.tex.as_str()).collect::<Vec<_>>().join(" ")
            }
        };
        // A smaller script in between the other group's pieces belongs to
        // that group: the ′ of a_{k′λ} is not a superscript of a.
        // (only if it sits at that group's height, not its own)
        let cy_of = |v: &[&Item]| {
            let ys: Vec<f32> = v.iter().flat_map(|g| g.src.iter()).map(|&k| spans[k].cy()).collect();
            ys.iter().sum::<f32>() / ys.len().max(1) as f32
        };
        let inside = |it: &Item, group: &[&Item], own: &[&Item]| {
            // its own group's height without it (none left: it is alone)
            let rest: Vec<&Item> = own.iter().copied().filter(|o| !std::ptr::eq(*o, it)).collect();
            let other_y = cy_of(group);
            let (l, r) = group.iter().fold((f32::MAX, f32::MIN), |(l, r), g| (l.min(g.left), r.max(g.right)));
            let big = group.iter().flat_map(|g| g.src.iter()).map(|&k| spans[k].size).fold(0.0f32, f32::max);
            let own = it.src.iter().map(|&k| spans[k].size).fold(0.0f32, f32::max);
            let y = cy_of(&[it]);
            group.len() >= 2
                && it.left > l
                && it.right < r
                && !it.src.is_empty()
                && own < 0.9 * big
                && (rest.is_empty() || (y - other_y).abs() < (y - cy_of(&rest)).abs())
        };
        let (moved_up, subs): (Vec<&Item>, Vec<&Item>) = {
            let (snapshot, own) = (sups.clone(), subs.clone());
            subs.iter().partition(|it| inside(it, &snapshot, &own))
        };
        let (moved_down, mut sups): (Vec<&Item>, Vec<&Item>) = {
            let (snapshot, own) = (subs.clone(), sups.clone());
            sups.iter().partition(|it| inside(it, &snapshot, &own))
        };
        let mut subs = subs;
        subs.extend(moved_down);
        sups.extend(moved_up);
        subs.sort_by(|a, b| a.left.total_cmp(&b.left));
        sups.sort_by(|a, b| a.left.total_cmp(&b.left));
        let (sup, sub) = (script(&sups), script(&subs));
        let sup = if folded.is_empty() {
            sup
        } else {
            // (the dot as a small one among the small letters)
            let small = folded.iter().filter(|k| !dots.contains(k) && !spans[**k].cmex()).map(|&k| spans[k].size).fold(0.0f32, f32::max);
            let mut inside = subset(spans, &folded);
            for (s, k) in inside.iter_mut().zip(&folded) {
                if dots.contains(k) {
                    (s.text, s.size) = ("·".to_string(), small);
                    let mid = s.cy();
                    (s.top, s.bottom) = (mid - small / 2.0, mid + small / 2.0);
                }
            }
            build_at(&inside, depth + 1)
        };
        // "+ ½": small stacked numbers after an operator are a fraction.
        let after_op = out.last().is_none_or(|p| {
            matches!(
                p.as_str(),
                "+" | "-" | "=" | "(" | "[" | "," | "\\cdot" | "\\times" | "\\pm" | "\\mp" | "\\{" | "\\left(" | "\\left[" | "\\left\\{"
            )
        });
        if !sup.is_empty() && !sub.is_empty() && after_op {
            out.push(format!("\\tfrac{{{sup}}}{{{sub}}}"));
            continue;
        }
        // A script can't belong to "=", "−", "," or an opening bracket: it
        // is a prescript of what follows (ₙφ written {}_{N} \varphi).
        let no_base = |p: &str| {
            matches!(
                p.split_whitespace().last().unwrap_or(""),
                "=" | "<" | ">" | "\\leq" | "\\geq" | "\\approx" | "\\equiv" | "\\sim" | "\\simeq" | "\\neq"
                    | "+" | "-" | "," | "(" | "[" | "\\{" | "\\cdot" | "\\times" | "\\pm" | "\\mp"
            )
        };
        let mut t = match out.pop() {
            Some(p) if no_base(&p) => {
                out.push(p);
                "{}".to_string()
            }
            Some(p) => p,
            None => "{}".to_string(),
        };
        if !sub.is_empty() {
            t += &format!("_{{{sub}}}");
        }
        if !sup.is_empty() {
            t += &format!("^{{{sup}}}");
        }
        out.push(t);
    }
    finish(out)
}

/// Last touches on a written-out formula: restore lost bracket partners and
/// put root signs over what they cover.
fn finish(mut out: Vec<String>) -> String {
    out.retain(|t| !t.is_empty());
    // In a formula of several lines a big bracket may open on one line and
    // close on a later one: it is left open to the end of its line
    // ("\left\{ … \right.") or closed from the start ("\left. … \right\}").
    if MULTILINE.with(|m| m.get()) {
        let mut k = 0;
        while k < out.len() {
            if let Some(c) = out[k].strip_prefix(LONE).and_then(|r| r.chars().next()) {
                match opens_kind(c) {
                    Some(true) => {
                        out[k] = format!("\\left{}", delim_tex(c));
                        out.push("\\right.".into());
                    }
                    Some(false) => {
                        out[k] = format!("\\right{}", delim_tex(c));
                        out.insert(0, "\\left.".into());
                        k += 1;
                    }
                    None => {}
                }
            }
            k += 1;
        }
    }
    // A big bracket whose partner was lost. Brackets usually enclose a sum
    // of terms, so a lone ")" gets its "(" right after the nearest fraction,
    // big operator or relation sign to its left (or at the start), and a
    // lone "(" its ")" before the nearest one to its right (or at the end).
    let stop = |t: &str| {
        ["\\frac", "\\tfrac", "\\sum", "\\int", "\\prod", "\\oint"].iter().any(|p| t.starts_with(p))
            || matches!(t, "=" | "\\neq" | "<" | ">" | "\\approx" | "\\equiv" | "\\leq" | "\\geq" | "\\sim" | "\\simeq")
    };
    let mut k = 0;
    while k < out.len() {
        if out[k].starts_with(LONE) {
            let c = out[k].chars().nth(1).unwrap_or(')');
            out[k] = out[k].replacen(&format!("{LONE}{c}"), delim_tex(c), 1);
            match opens_kind(c) {
                Some(false) => {
                    let at = (0..k).rev().find(|&j| stop(&out[j])).map_or(0, |j| j + 1);
                    out.insert(at, delim_tex(partner(c)).into());
                    k += 1;
                }
                Some(true) => {
                    let at = (k + 1..out.len()).find(|&j| stop(&out[j])).unwrap_or(out.len());
                    out.insert(at, delim_tex(partner(c)).into());
                }
                None => {}
            }
        }
        k += 1;
    }
    // A root sign covers what follows it, up to a relation sign.
    let mut res: Vec<String> = Vec::new();
    let mut k = 0;
    while k < out.len() {
        if out[k] == "\\surd" {
            let mut j = k + 1;
            while j < out.len() && !is_relation(&out[j]) {
                j += 1;
            }
            if j > k + 1 {
                res.push(format!("\\sqrt{{{}}}", out[k + 1..j].join(" ")));
                k = j;
                continue;
            }
        }
        res.push(out[k].clone());
        k += 1;
    }
    res.join(" ")
}

// ---------------------------------------------------------------------------
// Displayed equations

/// "(16)" -> "16". Equation numbers sit at the right margin.
pub fn tag_of(t: &str) -> Option<String> {
    let inner = t.trim().strip_prefix('(')?.strip_suffix(')')?;
    // "16", "3.422", "3.425b", or an appendix's "B.12" (a capital, a dot,
    // a number: not "(A)")
    let c: Vec<char> = inner.chars().collect();
    let start = c.first()?.is_ascii_digit() || (c.first()?.is_ascii_uppercase() && c.get(1) == Some(&'.') && c.get(2).is_some_and(char::is_ascii_digit));
    let ok = !inner.is_empty() && inner.len() <= 6 && start && inner.chars().all(|c| c.is_ascii_alphanumeric() || c == '.');
    ok.then(|| inner.to_string())
}

/// Split a displayed block (all spans of its lines) into equations, each
/// with its LaTeX and optional equation number.
pub fn display(
    spans: Vec<MSpan>,
    bars: &[Rule],
    right_margin: f32,
    body: f32,
) -> Vec<(String, Option<String>)> {
    let (tags, rest): (Vec<MSpan>, Vec<MSpan>) = spans
        .into_iter()
        .partition(|s| tag_of(&s.text).is_some() && s.right >= right_margin - 1.5 * body);
    if rest.is_empty() {
        return tags
            .iter()
            .map(|t| (String::new(), tag_of(&t.text)))
            .collect();
    }

    // Anchor rows: full-size rows with a relation sign (or an equation
    // number beside them). Two anchors well apart = two equations.
    let max = rest
        .iter()
        .filter(|s| !s.cmex())
        .map(|s| s.size)
        .fold(0.0f32, f32::max);
    let mut mains: Vec<&MSpan> = rest.iter().filter(|s| !s.cmex() && s.size >= 0.9 * max).collect();
    mains.sort_by(|a, b| a.cy().total_cmp(&b.cy()));
    // (centre, has a relation sign, starts with + or −)
    let mut rows: Vec<(f32, bool, bool)> = Vec::new();
    let mut cur: Vec<&MSpan> = Vec::new();
    let close_row = |cur: &Vec<&MSpan>, rows: &mut Vec<(f32, bool, bool)>| {
        if !cur.is_empty() {
            let c = cur.iter().map(|s| s.cy()).sum::<f32>() / cur.len() as f32;
            let rel = cur.iter().any(|s| is_relation(&s.text));
            let first = cur.iter().min_by(|a, b| a.left.total_cmp(&b.left)).unwrap();
            let op = first.text.trim_start().starts_with(['+', '−', '-', '×', '·']);
            rows.push((c, rel, op));
        }
    };
    for s in mains {
        if let Some(first) = cur.first() {
            if (s.cy() - first.cy()).abs() > 0.3 * max {
                close_row(&cur, &mut rows);
                cur.clear();
            }
        }
        cur.push(s);
    }
    close_row(&cur, &mut rows);

    // Anchor rows start a new equation (a relation sign or a number beside
    // them) or continue the one above (starting with + or −).
    let mut anchors: Vec<(f32, bool)> = Vec::new(); // (centre, continuation)
    // A row right above or below a fraction bar is a numerator or
    // denominator, not a line of its own.
    let fraction_row = |c: f32| {
        bars.iter().any(|r| (c - (r.top + r.bottom) / 2.0).abs() <= 1.3 * max)
    };
    // Rows inside a tall bracket are rows of a matrix, not lines.
    let in_bracket = |c: f32| {
        // shapeless bracket pieces, or brackets read from the SVG (tall)
        let pieces: Vec<&MSpan> = rest
            .iter()
            .filter(|s| s.cmex() && (s.text.trim().is_empty() || (BRACKETS.contains(s.text.trim()) && s.bottom - s.top > 1.5 * max)))
            .collect();
        pieces.iter().any(|p| {
            let column: Vec<&&MSpan> = pieces.iter().filter(|q| q.left < p.right && q.right > p.left).collect();
            let top = column.iter().map(|q| q.top).fold(f32::MAX, f32::min);
            let bottom = column.iter().map(|q| q.bottom).fold(f32::MIN, f32::max);
            let tall = column.len() >= 2 || column.iter().any(|q| !q.text.trim().is_empty());
            tall && c >= top - 0.5 * max && c <= bottom + max
        })
    };
    // Brackets read from their outline (true height): a row inside one but
    // away from its middle is a row of a matrix, never an equation of its own.
    let tall: Vec<&MSpan> = rest
        .iter()
        .filter(|s| s.cmex() && BRACKETS.contains(s.text.trim()) && !s.text.trim().is_empty() && s.bottom - s.top > 1.5 * max)
        .collect();
    // ... and lies entirely between a pair of them (a row starting with
    // "=" in front of a tall "(" is a line of its own)
    let between_pair = |l: f32, r: f32, c: f32| {
        // a brace opens cases on its right ("{") or groups rows on its left ("}")
        let brace = tall.iter().any(|b| {
            c > b.top
                && c < b.bottom
                && match b.text.trim() {
                    "{" => b.right <= l + 1.0,
                    "}" => b.left >= r - 1.0,
                    _ => false,
                }
        });
        brace || tall.iter().any(|b| {
            c > b.top
                && c < b.bottom
                && b.right <= l + 1.0
                && tall.iter().any(|q| q.left >= r - 1.0 && (q.cy() - b.cy()).abs() < 0.5 * max)
        })
    };
    let matrix_row = |c: f32| {
        let members: Vec<&MSpan> = rest
            .iter()
            .filter(|s| !s.cmex() && s.size >= 0.9 * max && (s.cy() - c).abs() <= 0.3 * max)
            // numerators and denominators belong to their fraction, not the row
            .filter(|s| {
                !bars.iter().any(|r| {
                    (s.cy() - (r.top + r.bottom) / 2.0).abs() <= 1.3 * max && s.left < r.right && s.right > r.left
                })
            })
            .collect();
        tall.iter().any(|b| c > b.top && c < b.bottom && (c - b.cy()).abs() > 0.6 * max)
            && !members.is_empty()
            && members.iter().all(|s| between_pair(s.left, s.right, c))
    };
    let mut skipped: Vec<f32> = Vec::new();
    let mut prev_row: Option<f32> = None;
    for (c, rel, op) in rows {
        if matrix_row(c) {
            skipped.push(c);
            continue;
        }
        let tagged = tags.iter().any(|t| (t.cy() - c).abs() <= 0.5 * max);
        // With no bars at all, a row well apart from the one above is a line
        // of its own ("φ₁ to φ₁ + δφ on σ₁" / "φ₂ to …"), and so is that one.
        let apart = bars.is_empty()
            && prev_row.is_some_and(|p| c - p >= 1.4 * max)
            && !in_bracket(c)
            && !prev_row.is_some_and(in_bracket);
        if apart && anchors.is_empty() {
            anchors.push((prev_row.unwrap_or(c), false));
        }
        prev_row = Some(c);
        let far = anchors.last().is_none_or(|&(a, _)| c - a >= 1.1 * max);
        let own_line = (!bars.is_empty() || apart) && !fraction_row(c) && !in_bracket(c);
        if far && (rel || tagged || own_line) {
            anchors.push((c, false));
        } else if far && op && !anchors.is_empty() {
            anchors.push((c, true));
        }
    }
    // A brace whose rows were skipped is a line of its own (at its middle)
    // unless a line already passes through it.
    let lone_brace = |b: &MSpan| match b.text.trim() {
        "{" => !tall.iter().any(|q| q.text.trim() == "}" && q.left > b.right && (q.cy() - b.cy()).abs() < 0.5 * max),
        "}" => !tall.iter().any(|q| q.text.trim() == "{" && q.right < b.left && (q.cy() - b.cy()).abs() < 0.5 * max),
        _ => false,
    };
    for b in tall.iter().filter(|b| lone_brace(b)) {
        let has_rows = skipped.iter().any(|&c| c > b.top && c < b.bottom);
        let crossed = anchors.iter().any(|&(a, _)| a > b.top && a < b.bottom);
        if has_rows && !crossed {
            anchors.push((b.cy(), false));
        }
    }
    anchors.sort_by(|a, b| a.0.total_cmp(&b.0));
    if anchors.len() <= 1 {
        let tag = tags.first().and_then(|t| tag_of(&t.text));
        return vec![(build_with(&rest, bars), tag)];
    }
    let nearest = |y: f32| -> usize {
        (0..anchors.len())
            .min_by(|&a, &b| (anchors[a].0 - y).abs().total_cmp(&(anchors[b].0 - y).abs()))
            .unwrap_or(0)
    };
    let mut groups: Vec<Vec<MSpan>> = vec![Vec::new(); anchors.len()];
    let mut gtags: Vec<Option<String>> = vec![None; anchors.len()];
    let mut gbars: Vec<Vec<Rule>> = vec![Vec::new(); anchors.len()];
    // A span inside a tall bracket goes with the bracket (its middle), not
    // with whichever line its own row happens to be nearest.
    let home = |s: &MSpan| -> f32 {
        // beside a brace: the brace's line
        let brace = tall.iter().find(|b| {
            lone_brace(b)
                && s.cy() > b.top
                && s.cy() < b.bottom
                && match b.text.trim() {
                    "{" => b.right <= s.left + 1.0,
                    "}" => b.left >= s.right - 1.0,
                    _ => false,
                }
        });
        if let Some(b) = brace {
            return b.cy();
        }
        tall.iter()
            .filter(|b| s.cy() > b.top && s.cy() < b.bottom && b.right <= s.left + 1.0)
            .max_by(|a, b| a.right.total_cmp(&b.right))
            .filter(|b| {
                // ... and a partner bracket to its right at the same height
                tall.iter().any(|q| q.left >= s.right - 1.0 && (q.cy() - b.cy()).abs() < 0.5 * max)
            })
            .map_or(s.cy(), |b| b.cy())
    };
    let homes: Vec<f32> = rest.iter().map(home).collect();
    for (s, y) in rest.into_iter().zip(homes) {
        groups[nearest(y)].push(s);
    }
    for t in &tags {
        gtags[nearest(t.cy())] = tag_of(&t.text);
    }
    for r in bars {
        gbars[nearest((r.top + r.bottom) / 2.0)].push(*r);
    }

    // Equations: a row plus the continuation rows below it.
    let mut eqs: Vec<(Vec<String>, Option<String>)> = Vec::new();
    for k in 0..anchors.len() {
        MULTILINE.with(|m| m.set(true));
        let t = build_with(&groups[k], &gbars[k]);
        MULTILINE.with(|m| m.set(false));
        match eqs.last_mut() {
            Some(e) if anchors[k].1 => {
                e.0.push(t);
                if e.1.is_none() {
                    e.1 = gtags[k].clone();
                }
            }
            _ => eqs.push((vec![t], gtags[k].clone())),
        }
    }
    // One equation number for several equations: a system, one block.
    if tags.len() == 1 && eqs.len() > 1 {
        let rows: Vec<String> = eqs.into_iter().flat_map(|e| e.0).collect();
        let tag = tags.first().and_then(|t| tag_of(&t.text));
        return vec![(aligned(&rows), tag)];
    }
    eqs.into_iter()
        .filter(|(r, t)| r.iter().any(|x| !x.is_empty()) || t.is_some())
        .map(|(r, t)| if r.len() == 1 { (r[0].clone(), t) } else { (aligned(&r), t) })
        .collect()
}

/// Rows of one `aligned` block, lined up at the first "=" (rows without
/// one, like continuation lines, are indented under the right-hand side).
/// The first " = " of a row that stands in no group: the rows line up
/// there (not at the "=" under a sum sign, "\\sum_{n = 1}").
fn top_level_equals(t: &str) -> Option<usize> {
    // (groups, and brackets of any kind: one may open what another closes,
    // "( a , b \\}")
    let (mut group, mut bracket) = (0i32, 0i32);
    let mut escaped = false;
    for (i, c) in t.char_indices() {
        let was = escaped;
        escaped = c == '\\' && !was;
        match c {
            '{' if was => bracket += 1,
            '}' if was => bracket = (bracket - 1).max(0),
            '{' => group += 1,
            '}' => group -= 1,
            '(' | '[' => bracket += 1,
            ')' | ']' => bracket = (bracket - 1).max(0),
            ' ' if group == 0 && bracket == 0 && t[i..].starts_with(" = ") => return Some(i),
            _ => {}
        }
    }
    None
}

fn aligned(rows: &[String]) -> String {
    let body: Vec<String> = rows
        .iter()
        .filter(|t| !t.is_empty())
        .map(|t| match (t.strip_prefix("= "), top_level_equals(t)) {
            // a row continuing with "= ..." lines up under the first "="
            (Some(rest), _) => format!("&= {rest}"),
            (None, Some(p)) => format!("{} &= {}", &t[..p], &t[p + 3..]),
            (None, None) => format!("& \\quad {t}"),
        })
        .collect();
    format!("\\begin{{aligned}} {} \\end{{aligned}}", body.join(" \\\\ "))
}

#[cfg(test)]
mod tests {
    #[test]
    fn arrows_after_their_letter() {
        assert_eq!(super::late_arrows("a x \u{20d7} - \\nabla \u{20d7}"), "a \\vec{x} - \\vec{\\nabla}");
        assert_eq!(super::late_arrows("( \u{20d7}"), "( \u{20d7}");
    }

    use super::*;

    fn sp(text: &str, left: f32, top: f32, w: f32, size: f32, family: &str) -> MSpan {
        let h = if size < 12.0 { 10.0 } else { 15.0 };
        MSpan {
            text: text.into(),
            left,
            right: left + w,
            top,
            bottom: top + h,
            size,
            family: family.into(),
            math: crate::layout::is_math_font(family),
            over: false,
            bold: crate::layout::is_bold_font(family),
            atom: None,
        }
    }

    #[test]
    fn scripts_and_primes() {
        // x′_µ  (from Dyson p. 12)
        let v = vec![
            sp("x", 568.0, 159.0, 9.0, 15.0, "CMMI10"),
            sp("′", 576.0, 157.0, 3.0, 10.0, "CMSY7"),
            sp("µ", 576.0, 166.0, 7.0, 10.0, "CMMI7"),
        ];
        assert_eq!(build(&v), "x_{\\mu}^{\\prime}");
    }

    #[test]
    fn fractions_and_small_fractions() {
        // ΔE = (8α³/3π)(1/n³) Ry + ⅚   (from Dyson p. 136)
        let v = vec![
            sp("∆", 305.0, 352.0, 12.0, 15.0, "CMR10"),
            sp("E", 318.0, 352.0, 11.0, 15.0, "CMMI10"),
            sp("=", 334.0, 352.0, 12.0, 15.0, "CMR10"),
            sp("8", 351.0, 342.0, 7.0, 15.0, "CMR10"),
            sp("α", 359.0, 342.0, 10.0, 15.0, "CMMI10"),
            sp("3", 368.0, 340.0, 6.0, 10.0, "CMR7"),
            sp("3", 355.0, 363.0, 7.0, 15.0, "CMR10"),
            sp("π", 363.0, 363.0, 9.0, 15.0, "CMMI10"),
            sp("1", 383.0, 342.0, 7.0, 15.0, "CMR10"),
            sp("n", 379.0, 363.0, 9.0, 15.0, "CMMI10"),
            sp("3", 388.0, 362.0, 6.0, 10.0, "CMR7"),
            sp("Ry", 399.0, 352.0, 19.0, 15.0, "CMR10"),
            sp("+", 422.0, 352.0, 12.0, 15.0, "CMR10"),
            sp("5", 439.0, 350.0, 6.0, 10.0, "CMR7"),
            sp("6", 439.0, 361.0, 6.0, 10.0, "CMR7"),
        ];
        let t = build(&v);
        assert_eq!(
            t,
            "\\Delta E = \\frac{8 \\alpha^{3}}{3 \\pi} \\frac{1}{n^{3}} \\text{ Ry } + \\tfrac{5}{6}"
        );
    }

    fn bar(left: f32, right: f32, top: f32) -> Rule {
        Rule { left, right, top, bottom: top + 0.7 }
    }

    #[test]
    fn fraction_bars() {
        // 1/c ∂/∂t, with the bars drawn in the PDF (Dyson eq. 12)
        let v = vec![
            sp("1", 372.0, 201.0, 7.0, 15.0, "CMR10"),
            sp("c", 373.0, 222.0, 6.0, 15.0, "CMMI10"),
            sp("∂", 388.0, 201.0, 8.0, 15.0, "CMMI10"),
            sp("∂t", 386.0, 222.0, 14.0, 15.0, "CMMI10"),
        ];
        let bars = [bar(372.0, 379.6, 218.4), bar(385.5, 399.8, 218.4)];
        assert_eq!(build_with(&v, &bars), "\\frac{1}{c} \\frac{\\partial}{\\partial t}");
    }

    #[test]
    fn stacked_and_nested_fractions() {
        // a/b over c/d stacked (same bar width) stay two fractions...
        let stacked = vec![
            sp("a", 400.0, 200.0, 8.0, 15.0, "CMMI10"),
            sp("b", 400.0, 220.0, 8.0, 15.0, "CMMI10"),
            sp("c", 400.0, 238.0, 8.0, 15.0, "CMMI10"),
            sp("d", 400.0, 258.0, 8.0, 15.0, "CMMI10"),
        ];
        let bars = [bar(398.0, 410.0, 217.5), bar(398.0, 410.0, 255.5)];
        let t = build_with(&stacked, &bars);
        assert!(t.contains("\\frac{a}{b}") && t.contains("\\frac{c}{d}"), "{t}");
        // ...while a narrower bar inside a denominator is a nested fraction
        let nested = vec![
            sp("1", 405.0, 200.0, 7.0, 15.0, "CMR10"),
            sp("x", 395.0, 228.0, 8.0, 15.0, "CMMI10"),
            sp("+", 405.0, 228.0, 12.0, 15.0, "CMR10"),
            sp("1", 420.0, 220.0, 7.0, 15.0, "CMR10"),
            sp("y", 420.0, 238.0, 8.0, 15.0, "CMMI10"),
        ];
        let bars = [bar(393.0, 430.0, 217.0), bar(419.0, 428.0, 235.5)];
        assert_eq!(build_with(&nested, &bars), "\\frac{1}{x + \\frac{1}{y}}");
    }

    #[test]
    fn bold_vectors() {
        // L = r × p with L, r, p in bold math italic (Dyson eq. 39)
        let v = vec![
            sp("L", 383.0, 380.0, 11.0, 15.0, "CMMIB10"),
            sp("=", 399.0, 380.0, 12.0, 15.0, "CMR10"),
            sp("r", 414.0, 380.0, 8.0, 15.0, "CMMIB10"),
            sp("×", 426.0, 380.0, 12.0, 15.0, "CMSY10"),
            sp("p", 441.0, 380.0, 9.0, 15.0, "CMMIB10"),
        ];
        assert_eq!(build(&v), "\\boldsymbol{L} = \\boldsymbol{r} \\times \\boldsymbol{p}");
    }

    #[test]
    fn sum_with_limits_and_neq() {
        // Σ_{ν=0}^{3} a ≠ b   (Σ comes out as "X" in CMEX10, ≠ as CMSY "6" + "=")
        let v = vec![
            sp("3", 409.0, 193.0, 6.0, 10.0, "CMR7"),
            sp("X", 401.0, 200.0, 22.0, 15.0, "CMEX10"),
            sp("ν", 401.0, 229.0, 6.0, 10.0, "CMMI7"),
            sp("=0", 408.0, 229.0, 15.0, 10.0, "CMR7"),
            sp("a", 425.0, 208.0, 8.0, 15.0, "CMMI10"),
            sp("6", 440.0, 208.0, 12.0, 15.0, "CMSY10"),
            sp("=", 440.0, 208.0, 12.0, 15.0, "CMR10"),
            sp("b", 455.0, 208.0, 8.0, 15.0, "CMMI10"),
        ];
        assert_eq!(build(&v), "\\sum_{\\nu = 0}^{3} a \\neq b");
    }

    /// A span with its exact box (left, right, top, bottom) as in `--dump`.
    fn bx(text: &str, l: f32, r: f32, t: f32, b: f32, size: f32, family: &str) -> MSpan {
        let mut s = sp(text, l, t, r - l, size, family);
        s.bottom = b;
        s
    }

    #[test]
    fn block_matrix() {
        // β = ((1 0; 0 1) 0; 0 (−1 0; 0 −1)), the blocks in a small font
        // without brackets of their own (Dyson eq. 15)
        let v = vec![
            bx("β", 461.0, 469.0, 786.0, 801.0, 15.0, "CMMI10"),
            bx("=", 474.0, 486.0, 786.0, 801.0, 15.0, "CMR10"),
            bx("(", 493.4, 500.8, 775.2, 811.1, 15.0, "CMEX10"),
            bx("1", 501.0, 506.0, 776.0, 782.0, 7.0, "CMR5"),
            bx("0", 501.0, 506.0, 785.0, 791.0, 7.0, "CMR5"),
            bx("0", 521.0, 526.0, 776.0, 782.0, 7.0, "CMR5"),
            bx("1", 521.0, 526.0, 785.0, 791.0, 7.0, "CMR5"),
            bx("0", 558.0, 565.0, 776.0, 791.0, 15.0, "CMR10"),
            bx("0", 510.0, 517.0, 794.0, 809.0, 15.0, "CMR10"),
            bx("−", 541.0, 549.0, 795.0, 802.0, 7.0, "CMSY5"),
            bx("1", 550.0, 555.0, 794.0, 800.0, 7.0, "CMR5"),
            bx("0", 548.0, 553.0, 803.0, 809.0, 7.0, "CMR5"),
            bx("0", 577.0, 582.0, 794.0, 800.0, 7.0, "CMR5"),
            bx("−", 570.0, 578.0, 804.0, 811.0, 7.0, "CMSY5"),
            bx("1", 578.0, 583.0, 803.0, 809.0, 7.0, "CMR5"),
            bx(")", 583.2, 590.5, 775.2, 811.1, 15.0, "CMEX10"),
        ];
        assert_eq!(
            build(&v),
            "\\beta = \\begin{pmatrix} \\begin{matrix} 1 & 0 \\\\ 0 & 1 \\end{matrix} & 0 \\\\ 0 & \\begin{matrix} - 1 & 0 \\\\ 0 & - 1 \\end{matrix} \\end{pmatrix}"
        );
    }

    #[test]
    fn cases_after_a_lone_brace() {
        // Θ_u = { = 1 for electron states u,  / = 0 for positron states.
        let v = vec![
            bx("Θ", 345.0, 357.0, 659.0, 674.0, 15.0, "CMR10"),
            bx("u", 357.0, 364.0, 665.0, 675.0, 10.0, "CMMI7"),
            bx("=", 369.0, 381.0, 659.0, 674.0, 15.0, "CMR10"),
            bx("{", 387.0, 394.7, 644.5, 689.4, 15.0, "CMEX10"),
            bx("= 1", 397.0, 420.0, 649.0, 664.0, 15.0, "CMR10"),
            bx("for electron states", 435.0, 552.0, 649.0, 664.0, 15.0, "CMR10"),
            bx("= 0", 397.0, 420.0, 671.0, 686.0, 15.0, "CMR10"),
            bx("for positron states.", 435.0, 558.0, 671.0, 686.0, 15.0, "CMR10"),
        ];
        assert_eq!(
            build(&v),
            "\\Theta_{u} = \\begin{cases} = 1 & \\text{ for electron states } \\\\ = 0 & \\text{ for positron states } . \\end{cases}"
        );
    }
}
#[cfg(test)]
mod scripts_of_scripts {
    use super::*;

    #[test]
    fn subscript_inside_an_exponent() {
        // u₁ e^{ip₁·x}  (Dyson eq. 422): the "1" under "p" is a script of
        // a script
        let mk = |t: &str, l: f32, r: f32, top: f32, b: f32, size: f32, f: &str| MSpan {
            text: t.into(), left: l, right: r, top, bottom: b, size, family: f.into(),
            math: true, over: false, bold: false, atom: None };
        let v = vec![
            mk("u", 434.0, 443.0, 1041.0, 1056.0, 15.0, "CMMI10"),
            mk("1", 442.0, 448.0, 1047.0, 1057.0, 10.0, "CMR7"),
            mk("e", 449.0, 456.0, 1041.0, 1056.0, 15.0, "CMMI10"),
            mk("ip", 456.0, 466.0, 1038.0, 1048.0, 10.0, "CMMI7"),
            mk("1", 466.0, 471.0, 1043.0, 1049.0, 7.0, "CMR5"),
            mk("·", 472.0, 476.0, 1038.0, 1048.0, 10.0, "CMSY7"),
            mk("x", 476.0, 483.0, 1038.0, 1048.0, 10.0, "CMMI7"),
        ];
        assert_eq!(build(&v), "u_{1} e^{ip_{1} \\cdot x}");
    }
}

#[cfg(test)]
mod groups_accents_underbraces {
    use super::*;

    fn mk(t: &str, l: f32, r: f32, top: f32, b: f32, size: f32, f: &str) -> MSpan {
        MSpan {
            text: t.into(), left: l, right: r, top, bottom: b, size, family: f.into(),
            math: crate::layout::is_math_font(f), over: false, bold: false, atom: None,
        }
    }

    #[test]
    fn rows_grouped_by_a_right_brace() {
        // ūu = +1 for electron states / = −1 for positron states } = ε (eq. 106)
        let v = vec![
            mk("uu", 242.0, 259.0, 865.0, 880.0, 15.0, "CMMI10"),
            mk("= +1 for electron states", 274.0, 434.0, 865.0, 880.0, 15.0, "CMR10"),
            mk("}", 445.1, 452.4, 863.4, 899.3, 15.0, "CMEX10"),
            mk("=", 459.0, 471.0, 874.0, 889.0, 15.0, "CMR10"),
            mk("ǫ", 474.0, 480.0, 874.0, 889.0, 15.0, "CMMI10"),
            mk("=", 274.0, 286.0, 883.0, 898.0, 15.0, "CMR10"),
            mk("−", 289.0, 301.0, 883.0, 900.0, 15.0, "CMSY10"),
            mk("1 for positron states", 301.0, 436.0, 883.0, 898.0, 15.0, "CMR10"),
        ];
        assert_eq!(
            build(&v),
            "\\left. \\begin{aligned} uu &= + 1 \\text{ for electron states } \\\\ &= - 1 \\text{ for positron states } \\end{aligned} \\right\\} = \\epsilon"
        );
    }

    #[test]
    fn accent_glyph_over_a_letter() {
        // ã_{kµ}: the tilde is a glyph of its own sharing the box of the a
        let v = vec![
            mk("˜", 365.0, 372.0, 178.0, 193.0, 15.0, "CMR10"),
            mk("a", 365.0, 373.0, 178.0, 193.0, 15.0, "CMMI10"),
            mk("kµ", 373.0, 387.0, 183.0, 193.0, 10.0, "CMMI7"),
        ];
        assert_eq!(build(&v), "\\tilde{a}_{k\\mu}");
    }

    #[test]
    fn labelled_underbrace() {
        // 1 − ½ α²/N² with an underbrace labelled "NR levels" (eq. 95)
        let v = vec![
            mk("1", 390.0, 397.0, 198.0, 213.0, 15.0, "CMR10"),
            mk("−", 400.0, 412.0, 198.0, 215.0, 15.0, "CMSY10"),
            mk("1", 417.0, 424.0, 188.0, 203.0, 15.0, "CMR10"),
            mk("2", 417.0, 424.0, 208.0, 223.0, 15.0, "CMR10"),
            mk("α", 430.0, 440.0, 188.0, 203.0, 15.0, "CMMI10"),
            mk("2", 440.0, 446.0, 185.0, 195.0, 10.0, "CMR7"),
            mk("N", 428.0, 440.0, 208.0, 223.0, 15.0, "CMMI10"),
            mk("2", 442.0, 448.0, 207.0, 217.0, 10.0, "CMR7"),
            mk("|", 413.0, 418.8, 228.0, 239.0, 15.0, "CMEX10"),
            mk("{z", 424.7, 436.3, 228.0, 239.0, 15.0, "CMEX10"),
            mk("}", 442.2, 448.0, 228.0, 239.0, 15.0, "CMEX10"),
            mk("NR levels", 406.0, 456.0, 239.0, 249.0, 10.0, "CMR7"),
        ];
        let bar = |l: f32, r: f32, t: f32, b: f32| Rule { left: l, right: r, top: t, bottom: b };
        let bars = vec![
            bar(417.2, 424.8, 204.7, 205.4),
            bar(428.4, 448.9, 204.7, 205.4),
            bar(419.7, 423.9, 231.1, 232.9),
            bar(437.2, 441.3, 231.1, 232.9),
        ];
        assert_eq!(
            build_with(&v, &bars),
            "1 - \\underbrace{\\frac{1}{2} \\frac{\\alpha^{2}}{N^{2}}}_{\\text{ NR levels }}"
        );
    }
}

/// A second subscript (or superscript) on the same symbol is not LaTeX
/// ("r_{i}^{2}_{i}^{j}", tensor indices side by side): give it an empty
/// base, "r_{i}^{2}{}_{i}^{j}".
pub fn fix_double_scripts(t: &str) -> String {
    let b = t.as_bytes();
    let mut out = String::with_capacity(t.len() + 4);
    let (mut sub, mut sup) = (false, false);
    let mut i = 0;
    while i < b.len() {
        let c = b[i];
        if c == b'_' || c == b'^' {
            let seen = if c == b'_' { sub } else { sup };
            if seen {
                out.push_str("{}");
                sub = false;
                sup = false;
            }
            if c == b'_' { sub = true } else { sup = true }
            out.push(c as char);
            i += 1;
            // its argument: a {…} group, a \command, or one character
            let start = i;
            if i < b.len() && b[i] == b'{' {
                let mut depth = 0;
                while i < b.len() {
                    match b[i] {
                        b'{' => depth += 1,
                        b'}' => {
                            depth -= 1;
                            if depth == 0 {
                                i += 1;
                                break;
                            }
                        }
                        b'\\' => i += 1,
                        _ => {}
                    }
                    i += 1;
                }
            } else if i < b.len() && b[i] == b'\\' {
                i += 1;
                while i < b.len() && b[i].is_ascii_alphabetic() {
                    i += 1;
                }
                if i == start + 1 && i < b.len() {
                    i += 1;
                }
            } else if i < b.len() {
                i += t[i..].chars().next().map_or(1, char::len_utf8);
            }
            // (a group has indices of its own: "P_{p p_{2}^{2}_{2}}")
            let arg = &t[start..i.min(b.len())];
            match arg.strip_prefix('{').and_then(|a| a.strip_suffix('}')) {
                Some(inner) if inner.contains(['_', '^']) => out.push_str(&format!("{{{}}}", fix_double_scripts(inner))),
                _ => out.push_str(arg),
            }
            continue;
        }
        if !c.is_ascii_whitespace() {
            sub = false;
            sup = false;
        }
        let ch = t[i..].chars().next().unwrap();
        out.push(ch);
        i += ch.len_utf8();
    }
    out
}

#[cfg(test)]
mod double_script_tests {
    use super::fix_double_scripts as f;
    #[test]
    fn empty_base_for_second_scripts() {
        assert_eq!(f("r_{i}^{2}_{i}^{j}"), "r_{i}^{2}{}_{i}^{j}");
        assert_eq!(f("x_{1}^{2}+y_{a}"), "x_{1}^{2}+y_{a}");
        assert_eq!(f("T_{a}_{b}"), "T_{a}{}_{b}");
        assert_eq!(f("e^{i\\pi}"), "e^{i\\pi}");
        assert_eq!(f("a_\\mu^\\nu"), "a_\\mu^\\nu");
    }
}
