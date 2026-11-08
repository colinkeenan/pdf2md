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
            atom: None,
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
        '⟨' | '〈' => "\\langle ",
        '⟩' | '〉' => "\\rangle ",
        '…' => "\\dots ",
        '−' => "-",
        // In TeX fonts ħ is drawn with a glyph that comes out as "~".
        'ℏ' | '~' => "\\hbar ",
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
    if std::env::var_os("PDF2MD_MATHDEBUG").is_some() {
        eprintln!("--- formula");
        for s in &spans {
            eprintln!("  {:7.1} {:7.1} {:7.1} {:7.1} {:4.1} {:8} {:?}", s.left, s.right, s.top, s.bottom, s.size, s.family, s.text);
        }
        for b in bars {
            eprintln!("  bar {:?}", b);
        }
    }
    tidy(&build_at(&resolve_bars(spans, bars, 0), 0))
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
                .filter(|&i| i != ri && within(&spans[i]) && spans[i].top >= b.top - 2.0 && spans[i].cy() <= b.bottom + 2.5 * size)
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
        let below: Vec<usize> = (0..spans.len())
            .filter(|&i| within(&spans[i]) && spans[i].top >= b.bottom - 2.0 && spans[i].cy() <= b.bottom + 3.5 * size)
            .collect();
        // A fraction nested inside this one has a clearly narrower bar; a
        // bar of about the same width right above or below is a separate
        // fraction stacked on this one (as in a column vector).
        let width = b.right - b.left;
        let nested = |r: &Rule| {
            let c = (r.left + r.right) / 2.0;
            r.right - r.left < 0.9 * width && c >= b.left - 2.0 && c <= b.right + 2.0
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
        let above: Vec<usize> = above.into_iter().filter(|&i| !blocked(&spans[i])).collect();
        let below: Vec<usize> = below.into_iter().filter(|&i| !blocked(&spans[i])).collect();
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
        let t = format!(
            "\\{}{{{}}}{{{}}}",
            if small { "tfrac" } else { "frac" },
            build_at(&resolve_bars(n, &nbars, depth + 1), depth + 1),
            build_at(&resolve_bars(d, &dbars, depth + 1), depth + 1)
        );
            let y = (b.top + b.bottom) / 2.0;
        let atom = MSpan {
            text: String::new(),
            left: b.left,
            right: b.right,
            top: y - size / 2.0,
            bottom: y + size / 2.0,
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

fn tidy(s: &str) -> String {
    let s = s.split_whitespace().collect::<Vec<_>>().join(" ");
    s.replace("\\not =", "\\neq").replace("\\not=", "\\neq")
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
        let (inside, outside): (Vec<MSpan>, Vec<MSpan>) = rest.into_iter().partition(|s| {
            s.left >= d0.right - 1.0
                && s.right <= d1.left + 1.0
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
    for k in 0..cand.len() {
        let r = root(&mut group, k);
        clusters.entry(r).or_default().push(cand[k]);
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
fn cases(spans: &[MSpan], depth: usize) -> Option<String> {
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
    if rows.len() < 2 {
        return None;
    }
    for c in small {
        let k = (0..rows.len())
            .min_by(|&a, &b| (rows[a][0].cy() - c.cy()).abs().total_cmp(&(rows[b][0].cy() - c.cy()).abs()))
            .unwrap_or(0);
        rows[k].push(c);
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
            if s.cmex() || (s.size >= 0.9 * max && d.abs() <= 0.3 * h) {
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
            role[i] = if d.abs() < 0.1 * h {
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
            items.push(Item { left: s.left, right: s.right, tex: t, kind: Kind::Atom });
        } else {
            let t = tex(s);
            if !t.is_empty() {
                items.push(Item { left: s.left, right: s.right, tex: t, kind: Kind::Atom });
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
            items.push(Item { left: spans[i].left, right: spans[i].right, tex: t, kind });
        }
    }

    // 4. Write everything out left to right, attaching scripts.
    items.sort_by(|a, b| {
        a.left
            .total_cmp(&b.left)
            .then((a.kind != Kind::Atom).cmp(&(b.kind != Kind::Atom)))
    });
    let mut out: Vec<String> = Vec::new();
    let mut prev_right: Option<f32> = None;
    let mut i = 0;
    while i < items.len() {
        if items[i].kind == Kind::Atom {
            // a wide gap separates two formulas on one line
            if prev_right.is_some_and(|r| items[i].left - r > 2.0 * max) {
                out.push("\\quad".into());
            }
            prev_right = Some(prev_right.map_or(items[i].right, |r| r.max(items[i].right)));
            out.push(items[i].tex.clone());
            i += 1;
            continue;
        }
        let mut sups: Vec<&str> = Vec::new();
        let mut subs: Vec<&str> = Vec::new();
        while i < items.len() && items[i].kind != Kind::Atom {
            prev_right = Some(prev_right.map_or(items[i].right, |r| r.max(items[i].right)));
            match items[i].kind {
                Kind::Sup => sups.push(&items[i].tex),
                _ => subs.push(&items[i].tex),
            }
            i += 1;
        }
        let (sup, sub) = (sups.join(" "), subs.join(" "));
        // "+ ½": small stacked numbers after an operator are a fraction.
        let after_op = out.last().is_none_or(|p| {
            matches!(
                p.as_str(),
                "+" | "-" | "=" | "(" | "[" | "," | "\\cdot" | "\\times" | "\\pm" | "\\mp"
            )
        });
        if !sup.is_empty() && !sub.is_empty() && after_op {
            out.push(format!("\\tfrac{{{sup}}}{{{sub}}}"));
            continue;
        }
        let mut t = out.pop().unwrap_or_else(|| "{}".to_string());
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
    let ok = !inner.is_empty()
        && inner.len() <= 6
        && inner.chars().next()?.is_ascii_digit()
        && inner.chars().all(|c| c.is_ascii_alphanumeric() || c == '.');
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
    let matrix_row = |c: f32| tall.iter().any(|b| c > b.top && c < b.bottom && (c - b.cy()).abs() > 0.6 * max);
    for (c, rel, op) in rows {
        if matrix_row(c) {
            continue;
        }
        let tagged = tags.iter().any(|t| (t.cy() - c).abs() <= 0.5 * max);
        let far = anchors.last().is_none_or(|&(a, _)| c - a >= 1.1 * max);
        let own_line = !bars.is_empty() && !fraction_row(c) && !in_bracket(c);
        if far && (rel || tagged || own_line) {
            anchors.push((c, false));
        } else if far && op && !anchors.is_empty() {
            anchors.push((c, true));
        }
    }
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
        let t = build_with(&groups[k], &gbars[k]);
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
fn aligned(rows: &[String]) -> String {
    let body: Vec<String> = rows
        .iter()
        .filter(|t| !t.is_empty())
        .map(|t| match t.find(" = ") {
            Some(p) => format!("{} &= {}", &t[..p], &t[p + 3..]),
            None => format!("& \\quad {t}"),
        })
        .collect();
    format!("\\begin{{aligned}} {} \\end{{aligned}}", body.join(" \\\\ "))
}

#[cfg(test)]
mod tests {
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
