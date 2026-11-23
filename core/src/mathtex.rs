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
        // the sign of degrees
        '◦' => "\\circ ",
        'α' => "\\alpha ",
        'β' => "\\beta ",
        'γ' => "\\gamma ",
        'δ' => "\\delta ",
        // (two letters: the curly ε, and the ϵ like a small "∈")
        'ε' => "\\varepsilon ",
        'ϵ' | 'ǫ' | 'ɛ' => "\\epsilon ",
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
        // (the d'Alembertian of a symbol font whose box comes under this code)
        '✷' => "\\Box ",
        'ı' => "\\imath ",
        '∂' => "\\partial ",
        '∇' => "\\nabla ",
        '∫' => "\\int ",
        '∮' => "\\oint ",
        '•' => "\\bullet ",
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
        '⇄' => "\\rightleftarrows ",
        '∠' => "\\angle ",
        '∞' => "\\infty ",
        '±' => "\\pm ",
        '∓' => "\\mp ",
        '×' => "\\times ",
        '·' | '⋅' => "\\cdot ",
        '∼' => "\\sim ",
        '≡' => "\\equiv ",
        '∝' => "\\propto ",
        '∈' => "\\in ",
        '⊙' => "\\odot ",
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

/// The LaTeX name of an accent typed after its letter (a combining mark).
fn accent_after(c: char) -> Option<&'static str> {
    match c {
        '\u{303}' => Some("tilde"),
        '\u{302}' => Some("hat"),
        '\u{304}' | '\u{305}' => Some("bar"),
        '\u{307}' => Some("dot"),
        '\u{308}' => Some("ddot"),
        '\u{20d7}' => Some("vec"),
        _ => None,
    }
}

pub(crate) fn chars_tex(t: &str) -> String {
    let t = t.replace(". . .", "…").replace("...", "…");
    let mut out = String::new();
    // (where the last sign written begins: an accent typed after it, the
    // tilde of "0̃", goes over it)
    let mut last = 0;
    for c in t.chars() {
        if c.is_whitespace() {
            continue;
        }
        if let Some(a) = accent_after(c).filter(|_| out.len() > last) {
            let sign = out[last..].trim().to_string();
            out.truncate(last);
            out.push_str(&format!("\\{a}{{{sign}}}"));
            continue;
        }
        last = out.len();
        match sym(c) {
            Some(s) => out.push_str(s),
            None => out.push(c),
        }
    }
    out.trim().to_string()
}

/// Function names LaTeX already knows (`\log`, `\sin`, ...).
pub(crate) const FUNCS: &[&str] = &[
    "log", "ln", "sin", "cos", "tan", "cot", "sec", "csc", "sinh", "cosh", "tanh", "coth", "exp",
    "lim", "max", "min", "det", "arg", "dim", "ker", "sup", "inf", "arcsin", "arccos", "arctan",
    "deg", "gcd",
];
/// Other upright operator names seen in physics books.
pub(crate) const OPNAMES: &[&str] = &["Tr", "tr", "Sp", "Spur", "Re", "Im", "sgn", "const"];

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
        let accent = accent_after(c);
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

/// How tall a big operator of TeX's extension font is, in ems: the box
/// the PDF reports for it is only its top. (The large one of a displayed
/// line, the small one of a line of text or a fraction's half.)
fn op_depth(s: &MSpan) -> Option<f32> {
    // (not the fonts of other makers read as "CMEX/MTEX": their signs
    // have other heights, and boxes of their own)
    if !s.cmex() || s.family.contains('/') {
        return None;
    }
    match s.text.trim() {
        "X" | "Y" => Some(1.4),
        "P" | "Q" => Some(1.0),
        "Z" | "I" => Some(2.22),
        "R" | "H" => Some(1.11),
        _ => None,
    }
}

/// Where a big operator's top is. Its glyph hangs from its baseline, and
/// the box reported for it is the font's height around that line: more
/// in one PDF than in another above the sign, the same under it.
fn op_top(op: &MSpan) -> f32 {
    op.bottom - 0.45 * op.size
}

/// Is the small piece `s` a limit of the big operator `op`: right over
/// its top, or right under its foot (set under and over the sign, or
/// behind it as an integral's)?
fn limit_of(s: &MSpan, op: &MSpan) -> bool {
    let Some(depth) = op_depth(op) else { return false };
    let (size, top) = (op.size, op_top(op));
    if s.cmex() || s.size >= 0.85 * size {
        return false;
    }
    let beside = s.right > op.left - 0.2 * size && s.left <= op.right + 0.5 * size;
    let upper = s.bottom >= top - 0.3 * size && s.bottom <= top + 0.8 * size;
    let foot = s.top - top;
    let lower = foot >= (depth - 0.6) * size && foot <= (depth + 0.5) * size;
    beside && (upper || lower)
}

/// LaTeX for one span on its own.
pub fn tex(s: &MSpan) -> String {
    if let Some(a) = &s.atom {
        return a.clone();
    }
    let mut t = tex_plain(s);
    // bold symbols (vectors) stay bold; spacing commands like \text stay as they are
    if s.bold && !t.is_empty() && !t.starts_with("\\text") {
        // (a letter of a bold text font stands upright, **r**; one of a
        // bold math font is slanted)
        // (TeX's bold roman: other books' bold letters are left as they were)
        let roman = s.family.to_uppercase().starts_with("CMBX") && !s.family.to_uppercase().starts_with("CMBXTI");
        let upright = roman && !s.math && t.len() == 1 && t.starts_with(|c: char| c.is_ascii_alphabetic());
        t = if upright { format!("\\mathbf{{{t}}}") } else { format!("\\boldsymbol{{{t}}}") };
    }
    if s.over && !t.is_empty() {
        format!("\\overline{{{t}}}")
    } else {
        t
    }
}

/// Is the usual phi of the book being converted the one that comes as φ?
static PHI_OTHER: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Tell which of the two phis is the book's usual one, from how often each
/// character stands in TeX's math italic: `curly` times as φ (the code of
/// TeX's glyph "phi" in one way of making a PDF), `stroked` times as ϕ (its
/// code in another). A book writes \phi far more often than \varphi, if
/// it has both at all: the more frequent character is \phi, the other
/// \varphi. (Read without this, φ is \varphi and ϕ is \phi, as the
/// characters are named.)
pub fn set_phi(curly: usize, stroked: usize) {
    PHI_OTHER.store(curly > stroked, std::sync::atomic::Ordering::Relaxed);
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
        // (the slash in one piece with the sign it strikes, "6∈N" for ∉ 𝒩:
        // the symbol font has no figures, a 6 in it is always the slash)
        if t.contains('6') {
            let parts: Vec<String> = t
                .split('6')
                .map(|p| {
                    let mut piece = s.clone();
                    piece.text = p.to_string();
                    if p.trim().is_empty() { String::new() } else { tex_plain(&piece) }
                })
                .collect();
            return parts.join(" \\not ").trim().to_string();
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
    // In TeX's math italic font the arrow of a vector is the glyph before
    // its letter, and comes out as "~": "~x" is x under an arrow.
    if s.family.starts_with("CMMI") && t.contains('~') {
        let chars: Vec<char> = t.chars().filter(|c| !c.is_whitespace()).collect();
        let mut out: Vec<String> = Vec::new();
        let mut k = 0;
        while k < chars.len() {
            match (chars[k], chars.get(k + 1)) {
                ('~', Some(&letter)) if letter != '~' => {
                    out.push(format!("\\vec{{{}}}", chars_tex(&letter.to_string()).trim()));
                    k += 2;
                }
                (c, _) => {
                    out.push(chars_tex(&c.to_string()).trim().to_string());
                    k += 1;
                }
            }
        }
        return out.join(" ");
    }
    // Script capitals (ℒ, ℋ): the script font stores them as plain letters.
    if s.family.to_uppercase().starts_with("RSFS") && t.chars().all(|c| c.is_ascii_alphabetic()) && !t.is_empty() {
        return t.chars().map(|c| format!("\\mathscr{{{c}}}")).collect::<Vec<_>>().join(" ");
    }
    // Blackboard bold (𝕀, ℝ ...): the AMS font stores them as plain letters.
    if s.family.starts_with("MSBM") && t.chars().all(|c| c.is_ascii_alphabetic()) && !t.is_empty() {
        return t.chars().map(|c| format!("\\mathbb{{{c}}}")).collect::<Vec<_>>().join(" ");
    }
    if s.math {
        let tex = chars_tex(t);
        // (TeX's math italic has two phis, and a PDF gives the usual one,
        // \phi with its straight stroke, under either of two character
        // codes, depending on what made the PDF: see `set_phi`)
        // (the MathTime italic draws the curly phi for the code ϕ and the
        // straight one for φ, always: the recognizer, which sees the
        // shapes, named Zee's 852 to 10 so)
        let family = s.family.to_uppercase();
        let other = (PHI_OTHER.load(std::sync::atomic::Ordering::Relaxed) && family.starts_with("CMMI")) || family.starts_with("MTMI");
        if other && (tex.contains("\\varphi") || tex.contains("\\phi")) {
            return tex.replace("\\varphi", "\u{1}").replace("\\phi", "\\varphi").replace('\u{1}', "\\phi");
        }
        tex
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
/// The comma or full stop that closes an equation set smaller than the
/// text is the text's, in its larger size: left so, it would be the only
/// piece of "the size of the line" and every letter an index. It is given
/// the size of the equation's letters.
fn level_punctuation(spans: &mut [MSpan]) {
    let max = spans.iter().filter(|s| !s.cmex()).map(|s| s.size).fold(0.0f32, f32::max);
    let mark = |s: &MSpan| !s.cmex() && s.atom.is_none() && !s.text.trim().is_empty() && s.text.trim().chars().all(|c| ",.;:".contains(c));
    if max <= 0.0 || spans.iter().any(|s| !s.cmex() && s.size >= 0.9 * max && !mark(s)) {
        return;
    }
    let next = spans.iter().filter(|s| !s.cmex() && s.size < 0.9 * max).map(|s| s.size).fold(0.0f32, f32::max);
    if next <= 0.0 {
        return;
    }
    for s in spans.iter_mut().filter(|s| !s.cmex() && s.size >= 0.9 * max) {
        // (it stands on the line: its box keeps its foot)
        let shrink = (s.bottom - s.top) * (1.0 - next / s.size);
        (s.size, s.top) = (next, s.top + shrink);
    }
}

/// Round brackets of full size around letters that are all small (the
/// brackets of "(j + ½)² − Z²α²" under a root in a small fraction's
/// denominator, set a size larger to hold the ½): they take the letters'
/// size, about their middle. Left large, they alone would be "the line"
/// and every letter beside them an index.
fn level_brackets(spans: &mut [MSpan]) {
    let max = spans.iter().filter(|s| !s.cmex()).map(|s| s.size).fold(0.0f32, f32::max);
    let bracket = |s: &MSpan| !s.cmex() && s.atom.is_none() && !s.text.trim().is_empty() && s.text.trim().chars().all(|c| "()".contains(c));
    if max <= 0.0 || spans.iter().any(|s| !s.cmex() && s.size >= 0.9 * max && !bracket(s)) {
        return;
    }
    // (a pair of them: a lone one is a piece of something not read whole)
    let has = |c: char| spans.iter().any(|s| bracket(s) && s.size >= 0.9 * max && s.text.contains(c));
    if !has('(') || !has(')') {
        return;
    }
    let letter = |s: &MSpan| !s.cmex() && s.atom.is_none() && s.size < 0.9 * max && s.text.chars().any(char::is_alphabetic);
    let next = spans.iter().filter(|s| letter(s)).map(|s| s.size).fold(0.0f32, f32::max);
    if next < 0.6 * max {
        return;
    }
    for s in spans.iter_mut().filter(|s| !s.cmex() && s.size >= 0.9 * max) {
        let shrink = (s.bottom - s.top) * (1.0 - next / s.size);
        (s.size, s.top, s.bottom) = (next, s.top + 0.5 * shrink, s.bottom - 0.5 * shrink);
    }
}

/// A formula begun in the size of the text and gone on in small type on
/// the same line ("L₃ =" and then a long right-hand side set \small to
/// fit a table's row): the small letters stand on the line as the others
/// do, and are letters of the line, not indices. Small type is about five
/// sixths of the text's size; an index is two thirds.
fn level_sizes(spans: &mut [MSpan]) {
    let max = spans.iter().filter(|s| !s.cmex()).map(|s| s.size).fold(0.0f32, f32::max);
    if max <= 0.0 {
        return;
    }
    let between = |s: &MSpan| !s.cmex() && s.size > 0.76 * max && s.size < 0.9 * max;
    // (in some books an index is that large. But an index hangs under
    // the line or stands over it, beside its letter: where small type
    // takes over from full size, the first small letter is level with
    // the full-size one before it.)
    let piece = |s: &MSpan| !s.cmex() && s.atom.is_none() && s.size > 0.76 * max && !s.text.trim().chars().all(|c| ",.;:".contains(c));
    let before = |s: &MSpan| spans.iter().filter(|m| piece(m) && m.right <= s.left + 1.0 && m.top < s.cy() && m.bottom > s.cy()).max_by(|a, b| a.right.total_cmp(&b.right));
    let turns: Vec<bool> = spans.iter().filter(|s| between(s)).filter_map(|s| before(s).filter(|m| m.size >= 0.9 * max).map(|m| (m.cy() - s.cy()).abs() <= 0.12 * max)).collect();
    // (… and none stands right under or over a full-size piece, as the
    // "ε → 0" under "lim" does)
    let stacked = spans.iter().any(|s| between(s) && spans.iter().any(|m| piece(m) && m.size >= 0.9 * max && m.left < s.right && m.right > s.left && (m.bottom <= s.top + 2.0 || m.top >= s.bottom - 2.0) && (s.top - m.bottom).max(m.top - s.bottom) <= 0.8 * max));
    if turns.is_empty() || turns.iter().any(|level| !level) || stacked {
        return;
    }
    for s in spans.iter_mut().filter(|s| !s.cmex() && s.size > 0.76 * max && s.size < 0.9 * max) {
        s.size = max;
    }
}

/// A line begun in the size of the text and gone on, to its end, in the
/// size of an index ("H = H_fs +" and then two long terms set small to
/// fit the line): where the small part begins. It is a formula of its
/// own size — in the size of the line, its letters would be indices and
/// its small fractions fall apart.
///
/// It begins behind the last full-size letter and the index on it; it
/// has fractions or tall brackets, and letters of its own size level
/// with the line (an exponent, however long, stands over the line).
fn small_tail(spans: &[MSpan]) -> Option<f32> {
    let max = spans.iter().filter(|s| !s.cmex()).map(|s| s.size).fold(0.0f32, f32::max);
    let word = |s: &MSpan| !s.cmex() && !s.text.trim().chars().all(|c| ",.;:".contains(c));
    let last = spans.iter().filter(|s| word(s) && s.atom.is_none() && s.size >= 0.9 * max).max_by(|a, b| a.right.total_cmp(&b.right))?;
    // (the index on the last letter: what touches it, and what touches that)
    let mut x = last.right;
    while let Some(n) = spans.iter().filter(|s| word(s) && s.left >= x - 2.0 && s.left <= x + 2.5 && s.right > x).map(|s| s.right).reduce(f32::max) {
        x = n;
    }
    let x = x + 1.0;
    let tail: Vec<&MSpan> = spans.iter().filter(|s| s.left >= x).collect();
    let letters: Vec<&&MSpan> = tail.iter().filter(|s| word(s)).collect();
    let size = letters.iter().map(|s| s.size).fold(0.0f32, f32::max);
    let level = letters.iter().filter(|s| s.size >= 0.9 * size && (s.cy() - last.cy()).abs() <= 0.15 * max).count();
    let built = tail.iter().any(|s| s.cmex() && s.height() > 1.1 * size);
    (letters.len() >= 8 && size >= 0.6 * max && size < 0.76 * max && level >= 2 && built).then_some(x)
}

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
    let mut spans = spans;
    // The circled signs of the extension font, which the PDF gives as the
    // letters of their places there: ⊙ ("J"), ⊕ ("L"), ⊗ ("N"). Set small
    // beside a letter it is that letter's index, the Sun's mass M_⊙: a
    // sign like any other, with the box of its whole height (the font's
    // boxes are only the tops of its signs). At the formula's own size it
    // is a big operator.
    let largest = spans.iter().filter(|s| !s.cmex()).map(|s| s.size).fold(0.0f32, f32::max);
    for s in spans.iter_mut() {
        if !s.cmex() || s.family.contains('/') || s.atom.is_some() {
            continue;
        }
        let (sign, operator) = match s.text.trim() {
            "J" | "K" => ("⊙", "\\bigodot"),
            "L" | "M" => ("⊕", "\\bigoplus"),
            "N" | "O" => ("⊗", "\\bigotimes"),
            _ => continue,
        };
        if s.size < 0.85 * largest {
            s.text = sign.into();
            s.family = "CMSY10".into();
            s.math = true;
            s.bottom = s.top + s.size;
        } else {
            s.atom = Some(operator.into());
        }
    }
    // A full stop that stands free between two operands of its line, room
    // on both sides, is the dot of a product, p · q: some fonts give their
    // raised dot under the full stop's code (a decimal point stands tight
    // in its number, the stop of a sentence has nothing behind it, and
    // the dots of "and so on" have dots beside them).
    for k in 0..spans.len() {
        let s = &spans[k];
        // (not in TeX's own fonts, whose product dot is a sign of the
        // symbol font: there a free-standing full stop ends a sentence
        // between two formulas)
        if s.text.trim() != "." || s.math || s.atom.is_some() || s.family.to_uppercase().starts_with("CM") {
            continue;
        }
        // (an arrow or a tilde over a letter is no neighbour: the letter is)
        let accent = |b: &MSpan| matches!(b.text.trim(), "\u{20d7}" | "\u{338}" | "˜" | "ˆ" | "¯" | "˙" | "¨");
        let row = |b: &MSpan| !b.cmex() && !accent(b) && b.bottom.min(s.bottom) - b.top.max(s.top) >= 0.5 * (s.bottom - s.top) && (b.size - s.size).abs() <= 0.15 * s.size;
        let operand = |b: &MSpan| row(b) && b.text.chars().any(|c| c.is_alphanumeric() || ")]".contains(c) || "([".contains(c)) && !b.text.trim().chars().all(|c| ".,;:".contains(c));
        let before = spans.iter().filter(|b| row(b) && b.right <= s.left + 0.5).max_by(|a, b| a.right.total_cmp(&b.right));
        let behind = spans.iter().filter(|b| row(b) && b.left >= s.right - 0.5).min_by(|a, b| a.left.total_cmp(&b.left));
        let room = |gap: f32| gap >= 0.12 * s.size && gap <= 0.8 * s.size;
        if let (Some(a), Some(b)) = (before, behind) {
            if operand(a) && operand(b) && room(s.left - a.right) && room(b.left - s.right) {
                spans[k].text = "·".into();
            }
        }
    }
    level_sizes(&mut spans);
    if let Some(x) = small_tail(&spans) {
        let (head, tail): (Vec<MSpan>, Vec<MSpan>) = spans.iter().cloned().partition(|s| s.left < x);
        let (hbars, tbars): (Vec<Rule>, Vec<Rule>) = bars.iter().copied().partition(|b| b.left < x);
        // (the sign that joins the two, set small in one piece with the
        // index before it: "fs +" under the H)
        let head = build_with(&head, &hbars);
        let head = match head.strip_suffix(" +}") {
            Some(h) if h.contains("_{") => format!("{h}}} +"),
            _ => head,
        };
        return format!("{head} {}", build_with(&tail, &tbars)).trim().to_string();
    }
    if std::env::var("PDF2MD_MATHDEBUG").is_ok_and(|v| v == "2") {
        eprintln!("--- before accents");
        for s in &spans {
            eprintln!("  {:7.1} {:7.1} {:7.1} {:7.1} {:4.1} {:8} {:?}", s.left, s.right, s.top, s.bottom, s.size, s.family, s.text);
        }
    }
    // (TeX's arrow over a vector, "~" in its math italic font, where it is
    // a piece of its own or ends one: the accent of the letter it is over)
    let spans: Vec<MSpan> = spans
        .into_iter()
        .map(|mut s| {
            if s.family.starts_with("CMMI") && s.atom.is_none() && s.text.trim_end().ends_with('~') {
                let kept = s.text.trim_end().trim_end_matches('~').to_string();
                s.text = format!("{kept}\u{20d7}");
            }
            s
        })
        .collect();
    let mut spans = put_accents(spans);
    // (an arrow with nothing under it: the letter is one pdftohtml has no
    // character for, the j without its dot that TeX sets under an accent)
    for k in 0..spans.len() {
        let a = &spans[k];
        if a.text.trim() == "\u{20d7}" && a.family.starts_with("CMMI") && !(0..spans.len()).any(|j| j != k && spans[j].left < 0.5 * (a.left + a.right) && spans[j].right > a.left && spans[j].top < a.bottom && spans[j].bottom > a.top) {
            let a = &mut spans[k];
            (a.atom, a.text) = (Some("\\vec{\\jmath}".to_string()), String::new());
        }
    }
    let spans = raised_dots(spans);
    let spans = index_commas(spans);
    let spans = script_brackets(spans);
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
    let (spans, bars) = bracket_rows(spans, bars);
    tidy(&build_at(&resolve_bars(spans, &bars, 0), 0))
}

/// The rows of a column between tall brackets are found before its
/// fraction bars are resolved: each bar then takes its numerator and
/// denominator from its own row only. The entries of a column are set in
/// the small size of a fraction in a line of text, so close together that
/// by distances alone the "0" of the row above reads as a piece of the
/// numerator under it, and two fractions one over the other as one inside
/// the other.
///
/// A row is told by a fraction bar or by a line of full-size letters;
/// rows less than a line apart (a full-size numerator on its bar, a
/// fraction inside a fraction) leave the bracket to [`resolve_bars`].
fn bracket_rows(spans: Vec<MSpan>, bars: Vec<Rule>) -> (Vec<MSpan>, Vec<Rule>) {
    if bars.is_empty() {
        return (spans, bars);
    }
    let size = spans.iter().filter(|s| !s.cmex()).map(|s| s.size).fold(0.0f32, f32::max).max(1.0);
    let tall = |s: &MSpan, which: &str| s.cmex() && s.atom.is_none() && s.height() > 2.0 * size && s.text.trim().chars().count() == 1 && which.contains(s.text.trim());
    let mut pairs: Vec<(MSpan, MSpan)> = Vec::new();
    for o in spans.iter().filter(|s| tall(s, "([")) {
        let level = |c: &MSpan| (c.top - o.top).abs() <= 3.0 && (c.bottom - o.bottom).abs() <= 3.0 && c.left > o.right;
        let Some(c) = spans.iter().filter(|c| tall(c, ")]") && level(c)).min_by(|a, b| a.left.total_cmp(&b.left)) else { continue };
        // (another bracket opening between them: not this one's column)
        if spans.iter().any(|x| tall(x, "([") && level(x) && x.left < c.left) {
            continue;
        }
        pairs.push((o.clone(), c.clone()));
    }
    let (mut spans, mut bars) = (spans, bars);
    for (o, c) in pairs {
        let inside = |s: &MSpan| {
            let x = (s.left + s.right) / 2.0;
            x >= o.right - 1.0 && x <= c.left + 1.0 && s.cy() >= o.top - 0.8 * size && s.cy() <= o.bottom + 0.8 * size
        };
        let bar_in = |r: &Rule| {
            r.left >= o.right - 2.0 && r.right <= c.left + 2.0 && r.top >= o.top - size && r.bottom <= o.bottom + size
        };
        // (a root sign's bar: of those that begin where the sign ends, the
        // one at its height — the fraction bar of the row above begins
        // there as well when the entries stand one under the other)
        let sign_bar = |s: &MSpan| {
            bars.iter()
                .filter(|r| is_radical(s) && bar_in(r) && (r.left - s.right).abs() <= 3.0 && s.top <= r.bottom + size && s.bottom >= r.top - 2.0)
                .min_by(|a, b| (a.top - s.cy()).abs().total_cmp(&(b.top - s.cy()).abs()))
                .copied()
        };
        let root_bar = |r: &Rule| spans.iter().any(|s| sign_bar(s) == Some(*r));
        // what tells a row: (height, is a bar)
        // (not the bar of a fraction inside a fraction: narrower than
        // another's, within its width and less than a line from it)
        let nested = |r: &Rule| bars.iter().any(|w| bar_in(w) && !root_bar(w) && r.right - r.left < 0.9 * (w.right - w.left) && r.left >= w.left - 2.0 && r.right <= w.right + 2.0 && (r.top - w.top).abs() < 1.1 * size);
        // (nor the bar of a small fraction that is the limit of a sum,
        // over the sign or under it: "n/2 − 1" on top of a Σ)
        let of_limit = |r: &Rule| spans.iter().any(|b| matches!(big_op(b), Some("\\sum" | "\\prod")) && r.left < b.right && r.right > b.left && ((r.bottom <= b.top + 2.0 && b.top - r.bottom <= size) || (r.top >= b.top + 0.8 * size && r.top - b.top <= 2.6 * size)));
        let mut marks: Vec<(f32, bool)> = bars.iter().filter(|r| bar_in(r) && !root_bar(r) && !nested(r) && !of_limit(r)).map(|r| ((r.top + r.bottom) / 2.0, true)).collect();
        if marks.is_empty() {
            continue;
        }
        // (a root sign tells none: its box is not where it is drawn)
        marks.extend(spans.iter().filter(|s| inside(s) && !s.cmex() && !is_radical(s) && s.size >= 0.9 * size && !s.text.trim().is_empty()).map(|s| (s.cy(), false)));
        marks.sort_by(|a, b| a.0.total_cmp(&b.0));
        // (height, is a fraction's row)
        let mut rows: Vec<(f32, bool)> = Vec::new();
        let mut k = 0;
        while k < marks.len() {
            let mut m = k + 1;
            while m < marks.len() && marks[m].0 - marks[k].0 <= 0.45 * size {
                m += 1;
            }
            let group = &marks[k..m];
            let fraction = group.iter().any(|g| g.1);
            let of: Vec<f32> = group.iter().filter(|g| g.1 == fraction).map(|g| g.0).collect();
            rows.push((of.iter().sum::<f32>() / of.len() as f32, fraction));
            k = m;
        }
        if rows.len() < 2 || rows.windows(2).any(|w| w[1].0 - w[0].0 < 1.1 * size) {
            continue;
        }
        // Between two rows the line is drawn close to the row of full-size
        // letters, just clear of their indices and exponents: a fraction's
        // small rows, with exponents and fractions of their own, reach far
        // from its bar, to the foot of the letters above and the head of
        // those below.
        let cuts: Vec<f32> = rows
            .windows(2)
            .map(|w| match (w[0].1, w[1].1) {
                (false, true) => w[0].0 + 0.33 * size,
                (true, false) => w[1].0 - 0.57 * size,
                _ => (w[0].0 + w[1].0) / 2.0,
            })
            .collect();
        let row_of = |y: f32| cuts.iter().filter(|&&c| y > c).count();
        let mut parts: Vec<Vec<MSpan>> = vec![Vec::new(); rows.len()];
        let mut part_bars: Vec<Vec<Rule>> = vec![Vec::new(); rows.len()];
        // (a root's bar is of the row of what stands under it)
        let under_root = |r: &Rule| {
            spans
                .iter()
                .filter(|s| inside(s) && !s.cmex() && !is_radical(s) && (s.left + s.right) / 2.0 >= r.left && (s.left + s.right) / 2.0 <= r.right && s.top >= r.top - 2.0)
                .map(|s| s.cy())
                .fold(f32::MAX, f32::min)
                .min(r.bottom + size)
        };
        for r in bars.iter().filter(|r| bar_in(r)) {
            let y = if root_bar(r) { under_root(r) } else { (r.top + r.bottom) / 2.0 };
            part_bars[row_of(y)].push(*r);
        }
        // (and the root sign of the row of its bar)
        let height = |s: &MSpan| match sign_bar(s) {
            Some(r) => under_root(&r),
            None => s.cy(),
        };
        let heights: Vec<f32> = spans.iter().map(height).collect();
        let mut rest: Vec<MSpan> = Vec::new();
        for (s, y) in spans.into_iter().zip(heights) {
            if inside(&s) && !(s.cmex() && s.atom.is_none() && s.height() > 2.0 * size) {
                parts[row_of(y)].push(s);
            } else {
                rest.push(s);
            }
        }
        bars.retain(|r| !bar_in(r));
        for (part, pbars) in parts.into_iter().zip(part_bars) {
            let full = part.iter().any(|s| !s.cmex() && s.size >= 0.9 * size);
            let had: Vec<String> = part.iter().filter_map(|s| s.atom.clone()).collect();
            for mut s in resolve_bars(part, &pbars, 1) {
                // (a fraction that is all of its row is an entry of the
                // column, of the size of the others)
                if !full && s.atom.as_ref().is_some_and(|a| !had.contains(a)) && s.size < 0.9 * size {
                    let y = (s.top + s.bottom) / 2.0;
                    (s.top, s.bottom, s.size) = (y - size / 2.0, y + size / 2.0, size);
                }
                rest.push(s);
            }
        }
        spans = rest;
    }
    (spans, bars)
}

/// Brackets of an exponent set in the size of the line: "e^{i(k·r − ωt)}"
/// with the two brackets as large as the e (the book writes \left( … \right)
/// up there), raised to the middle of the small letters between them.
/// Left so, each would be a letter of the line and the exponent would fall
/// apart around it. A full-size bracket level with a small piece right
/// beside it, and off the line's own middle, is given that piece's size
/// and height. The closing one may come in one piece with the sign after
/// it (") ="): it is cut off first.
fn script_brackets(mut spans: Vec<MSpan>) -> Vec<MSpan> {
    let size = spans.iter().filter(|s| !s.cmex()).map(|s| s.size).fold(0.0f32, f32::max).max(1.0);
    let small = |s: &MSpan| !s.cmex() && s.size < 0.85 * size;
    let full = |s: &MSpan| !s.cmex() && s.atom.is_none() && s.size >= 0.9 * size;
    // An opening bracket between two small pieces, level with both: the
    // "(" of "i(k". (Its box: a bracket alone, as tall as the type.)
    // (… or with the one before it, the one after it smaller still and
    // within the bracket's height: the numerator of a fraction that opens
    // the bracket, in the denominator of a fraction set small)
    let within = |n: &MSpan, s: &MSpan| small(n) && n.top >= s.top - 2.0 && n.bottom <= s.bottom + 2.0 && n.size < 0.8 * spans.iter().filter(|m| small(m) && (m.cy() - s.cy()).abs() <= 0.2 * size && (m.right - s.left).abs() <= 2.5).map(|m| m.size).fold(0.0f32, f32::max);
    let opening: Vec<usize> = (0..spans.len())
        .filter(|&k| {
            let s = &spans[k];
            let level = |n: &MSpan| small(n) && (n.cy() - s.cy()).abs() <= 0.2 * size;
            full(s) && matches!(s.text.trim(), "(" | "[") && s.height() <= 1.1 * size && spans.iter().any(|n| level(n) && (n.right - s.left).abs() <= 2.5) && spans.iter().any(|n| (level(n) || within(n, s)) && (n.left - s.right).abs() <= 2.5)
        })
        .collect();
    for o in opening {
        let open = spans[o].clone();
        let want = if open.text.trim() == "(" { ')' } else { ']' };
        // its partner: the next full-size closing bracket with its head at
        // the same height, a small piece right before it; alone, or in one
        // piece with what follows it on the line (") =")
        let close = (0..spans.len())
            .filter(|&k| {
                let s = &spans[k];
                full(s) && s.left >= open.right && s.text.trim_start().starts_with(want) && (s.top - open.top).abs() <= 1.5 && spans.iter().any(|n| small(n) && (n.right - s.left).abs() <= 2.5 && n.top >= open.top - 2.0 && n.bottom <= open.bottom + 2.0)
            })
            .min_by(|&a, &b| spans[a].left.total_cmp(&spans[b].left));
        let Some(c) = close else { continue };
        // (no other bracket of the line between them)
        if spans.iter().any(|s| full(s) && s.left > open.right && s.right < spans[c].left && s.top < open.bottom && s.bottom > open.top && s.text.chars().any(|ch| "()[]".contains(ch))) {
            continue;
        }
        // (the size of the larger of its neighbours, level with it)
        let Some((ssize, top, bottom)) = spans.iter().filter(|n| small(n) && (n.cy() - open.cy()).abs() <= 0.2 * size && ((n.left - open.right).abs() <= 2.5 || (n.right - open.left).abs() <= 2.5)).max_by(|a, b| a.size.total_cmp(&b.size)).map(|n| (n.size, n.top, n.bottom)) else { continue };
        let whole = spans[c].clone();
        let text = whole.text.trim_start().to_string();
        let mut first = whole.clone();
        (first.size, first.top, first.bottom) = (ssize, top, bottom);
        if text.chars().count() > 1 {
            let w = (whole.right - whole.left) / whole.text.chars().count() as f32;
            let mut rest = whole.clone();
            first.text = text.chars().take(1).collect();
            first.right = whole.left + w;
            rest.text = text.chars().skip(1).collect::<String>().trim_start().to_string();
            (rest.left, rest.top) = (whole.right - w * rest.text.chars().count() as f32, whole.bottom - 0.9 * size);
            spans[c] = first;
            spans.push(rest);
        } else {
            spans[c] = first;
        }
        let s = &mut spans[o];
        (s.size, s.top, s.bottom) = (ssize, top, bottom);
    }
    // A pair of full-size brackets off the line, with nothing but small
    // pieces between them: the brackets of a numerator or a denominator
    // set small, grown around a smaller fraction inside. They take the
    // size of the largest piece they hold.
    for o in 0..spans.len() {
        let open = spans[o].clone();
        if !(full(&open) && matches!(open.text.trim(), "(" | "[") && open.height() <= 1.1 * size) {
            continue;
        }
        let want = if open.text.trim() == "(" { ")" } else { "]" };
        let Some(c) = (0..spans.len())
            .filter(|&k| full(&spans[k]) && spans[k].left >= open.right && spans[k].text.trim() == want && (spans[k].top - open.top).abs() <= 1.5)
            .min_by(|&a, &b| spans[a].left.total_cmp(&spans[b].left))
        else {
            continue;
        };
        let close = spans[c].clone();
        let boxed = |n: &MSpan| !n.cmex() && n.left >= open.right - 1.0 && n.right <= close.left + 1.0 && n.bottom > open.top && n.top < open.bottom;
        let inside = spans.iter().filter(|n| boxed(n)).map(|n| n.size).fold(0.0f32, f32::max);
        // (off the line: the nearest letter of the line beside the pair,
        // at its height, has its middle clearly higher or lower. With no
        // such letter there is no line to be off: a bracket around a small
        // fraction in a line of text.)
        let beside = spans
            .iter()
            .filter(|n| full(n) && !n.text.trim().chars().all(|ch| "()[]".contains(ch)) && n.bottom > open.top && n.top < open.bottom && (n.right <= open.left + 1.0 || n.left >= close.right - 1.0))
            .min_by(|a, b| (a.left - close.right).max(open.left - a.right).total_cmp(&(b.left - close.right).max(open.left - b.right)));
        let off_line = beside.is_some_and(|n| (n.cy() - open.cy()).abs() > 0.25 * size);
        // (… and a small piece stands right beside one of the two, level
        // with it: the pair is of that piece's row. The brackets of
        // "(iθ/2)ⁿ" over a full-size "n!" have only the halves of their
        // fraction and an exponent beside them. Level to the point: a
        // bracket is set at the middle of its row's letters, and the
        // numerator of a small fraction inside it only a little higher.)
        let level = spans
            .iter()
            .filter(|n| small(n) && (n.cy() - open.cy()).abs() <= 0.1 * size && [open.left, open.right, close.left, close.right].iter().any(|x| (n.left - x).abs() <= 2.5 || (n.right - x).abs() <= 2.5))
            .map(|n| n.size)
            .fold(0.0f32, f32::max);
        // (a full-size letter right over or under the pair is the other
        // half of its fraction: the pair is of a full-size numerator, the
        // level piece beside it the limit of a sum)
        let partner = spans.iter().any(|n| full(n) && n.left < close.right && n.right > open.left && ((n.top >= open.bottom - 1.0 && n.top - open.bottom <= 0.45 * size) || (n.bottom <= open.top + 1.0 && open.top - n.bottom <= 0.45 * size)));
        if inside <= 0.0 || inside >= 0.85 * size || !off_line || level <= 0.0 || partner {
            continue;
        }
        let inside = inside.max(level);
        for k in [o, c] {
            let s = &mut spans[k];
            let cy = s.cy();
            (s.size, s.top, s.bottom) = (inside, cy - inside / 2.0, cy + inside / 2.0);
        }
    }
    spans
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
    // "⃗a": an arrow at the head of a piece, before the letter it is drawn
    // over (it moves the type on by nothing, so the piece is as wide as
    // its letters alone): a piece of its own, over the first letter
    spans = spans
        .into_iter()
        .flat_map(|s| {
            let chars: Vec<char> = s.text.chars().collect();
            if chars.len() < 2 || chars[0] != '\u{20d7}' || s.atom.is_some() || !chars[1].is_alphabetic() {
                return vec![s];
            }
            let letters: Vec<char> = chars[1..].to_vec();
            let units: f32 = letters.iter().map(|&c| crate::layout::char_width(c)).sum();
            let first = (s.right - s.left) * crate::layout::char_width(letters[0]) / units.max(0.01);
            let mut arrow = s.clone();
            arrow.text = "\u{20d7}".into();
            arrow.right = s.left + first;
            let mut rest = s;
            rest.text = letters.into_iter().collect();
            vec![arrow, rest]
        })
        .collect();
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
    // The same stroke in one piece with its letter, "/e" as wide as the e
    // alone: the slash moves the type on by nothing, so the piece is as
    // wide as its other letters. It is taken out and set over the letter
    // behind it.
    spans = spans
        .into_iter()
        .flat_map(|s| {
            let chars: Vec<char> = s.text.chars().collect();
            let slashes = chars.iter().filter(|&&c| c == '/').count();
            let units = |skip: bool| -> f32 { chars.iter().filter(|&&c| !(skip && c == '/')).map(|&c| crate::layout::char_width(c)).sum() };
            let width = s.right - s.left;
            // (nearer the width of its letters without the slash than with it)
            // (told by the widths of TeX's own letters: in another font a
            // piece with a dividing slash may be as narrow, Zee's "Y/")
            let overlaid = s.atom.is_none() && slashes > 0 && chars.len() > slashes && s.math && s.family.to_uppercase().starts_with("CMMI") && (width - s.size * units(true)).abs() < (width - s.size * units(false)).abs();
            if !overlaid {
                return vec![s];
            }
            let rest: Vec<char> = chars.iter().copied().filter(|&c| c != '/').collect();
            let scale = width / units(true).max(0.01);
            let mut edges = vec![s.left];
            for &c in &rest {
                edges.push(edges[edges.len() - 1] + crate::layout::char_width(c) * scale);
            }
            let mut out = Vec::new();
            let mut at = 0; // letters before the slash
            for &c in &chars {
                if c != '/' {
                    at += 1;
                    continue;
                }
                if at < rest.len() {
                    let mut stroke = s.clone();
                    stroke.text = "\u{338}".into();
                    (stroke.left, stroke.right) = (edges[at], edges[at + 1]);
                    out.push(stroke);
                }
            }
            let mut letters = s;
            letters.text = rest.into_iter().collect();
            out.insert(0, letters);
            out
        })
        .collect();
    // A slash set over a letter, with no room of its own: the stroke of
    // Feynman's p̸, which some books make of the plain "/" (a slash that
    // divides stands between its two sides, in room of its own).
    for k in 0..spans.len() {
        if spans[k].text.trim() != "/" || spans[k].atom.is_some() {
            continue;
        }
        let s = &spans[k];
        let (width, height) = (s.right - s.left, s.bottom - s.top);
        let over = spans.iter().enumerate().any(|(j, b)| {
            j != k
                && b.atom.is_none()
                && !b.cmex()
                && b.text.chars().any(char::is_alphabetic)
                && (b.size - s.size).abs() <= 0.1 * s.size
                && b.right.min(s.right) - b.left.max(s.left) >= 0.6 * width
                && b.bottom.min(s.bottom) - b.top.max(s.top) >= 0.5 * height
        });
        if over {
            spans[k].text = "\u{338}".into();
        }
    }
    let mut k = 0;
    while k < spans.len() {
        let Some(name) = accent(&spans[k].text) else {
            k += 1;
            continue;
        };
        let a = spans[k].clone();
        let mid = (a.left + a.right) / 2.0;
        // A second accent over a letter that has one: the dots over an
        // arrowed r, the arrow over a dotted p. The letter with its first
        // accent is one piece already, written out; this one goes around
        // it (the arrow inside the dots, as the book stacks them).
        let accented = (0..spans.len()).find(|&j| {
            let b = &spans[j];
            j != k
                && b.atom.as_ref().is_some_and(|x| ["\\vec{", "\\dot{", "\\ddot{", "\\hat{", "\\tilde{", "\\bar{"].iter().any(|p| x.starts_with(p)))
                && b.left <= mid + 1.0
                && b.right >= mid - 1.0
                && (b.top - a.top).abs() <= 0.75 * a.size
        });
        if let Some(j) = accented {
            let inner = spans[j].atom.clone().unwrap_or_default();
            // (an arrow is of the letter itself: under dots that are there)
            let dots = ["\\dot{", "\\ddot{"].iter().find(|p| inner.starts_with(**p));
            spans[j].atom = Some(match (name, dots) {
                ("vec", Some(p)) => format!("{p}\\vec{{{}}}}}", &inner[p.len()..inner.len() - 1]),
                ("vec", None) if inner.starts_with("\\vec{") => inner.clone(),
                _ => format!("\\{name}{{{inner}}}"),
            });
            spans.remove(k);
            k = 0;
            continue;
        }
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
        // (the letter with the mark of its arrow, which this font sets
        // before it as "~": the two are one, a dot goes over both)
        let (from, to) = if chars.get(ci) == Some(&'~') && ci + 1 < n {
            (ci, ci + 2)
        } else if ci > 0 && chars[ci - 1] == '~' {
            (ci - 1, ci + 1)
        } else {
            (ci, ci + 1)
        };
        let mut pieces: Vec<MSpan> = Vec::new();
        for (part, range) in [(0, 0..from), (1, from..to), (2, to..n)] {
            if range.is_empty() {
                continue;
            }
            let mut p = b.clone();
            p.text = chars[range.clone()].iter().collect();
            p.left = edges[range.start];
            p.right = edges[range.end];
            if part == 1 {
                let inner = tex(&p);
                // (a bar through the stem of an h is ħ, which TeX draws so)
                // (an arrow over a letter that has its arrow already, drawn
                // and marked: one arrow)
                p.atom = Some(if name == "bar" && inner.trim() == "h" {
                    "\\hbar".to_string()
                } else if name == "vec" && inner.trim().starts_with("\\vec{") {
                    inner.trim().to_string()
                } else {
                    format!("\\{name}{{{}}}", inner.trim())
                });
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

/// The dot of a product among small letters, e^{−(i/2)J·A⁻¹·J}: some books
/// set it as a full stop of the text font in the size of the line, raised
/// to the middle of the small letters on both sides of it. It is a small
/// dot at their height.
fn raised_dots(mut spans: Vec<MSpan>) -> Vec<MSpan> {
    for k in 0..spans.len() {
        let d = &spans[k];
        // (and so are the dots of "and so on" among indices, μ₁μ₂...μₙ)
        // (set in the size of the line: dots already as small as the
        // indices are in their place)
        let line_size = spans.iter().filter(|s| !s.cmex()).map(|s| s.size).fold(0.0f32, f32::max);
        let ellipsis = matches!(d.text.trim(), "..." | ". . ." | "…") && d.size >= 0.9 * line_size;
        if (d.text.trim() != "." && !ellipsis) || d.atom.is_some() {
            continue;
        }
        // (the small letters right beside it; one of them at least has the
        // dot at its middle height: the other may be an index of its own,
        // the −1 of K⁻¹ or a dagger, set higher)
        let small = |j: &usize| !spans[*j].cmex() && spans[*j].size <= 0.85 * d.size && spans[*j].top < d.bottom && spans[*j].bottom > d.top;
        let level = |j: usize| d.bottom >= spans[j].top + 0.3 * (spans[j].bottom - spans[j].top) && d.bottom <= spans[j].bottom + 1.0;
        let best = |x: &usize, y: &usize| level(*x).cmp(&level(*y)).then(spans[*x].size.total_cmp(&spans[*y].size));
        let before = (0..spans.len()).filter(small).filter(|&j| (d.left - spans[j].right).abs() <= 0.3 * d.size).max_by(best);
        let after = (0..spans.len()).filter(small).filter(|&j| (spans[j].left - d.right).abs() <= 0.3 * d.size).max_by(best);
        // (dots that end the indices, "T^{λ…}": raised to the height of
        // the index before them, nothing after)
        // (raised: their box stands higher than the letter the indices
        // are on; dots on the line after a lower index, "dx₂ … dx_N", do not)
        let raised = (0..spans.len())
            .filter(|&j| j != k && !spans[j].cmex() && spans[j].size >= 0.9 * line_size && spans[j].right <= d.left + 1.0 && !matches!(spans[j].text.trim(), "..." | ". . ." | "…" | "."))
            .max_by(|&x, &y| spans[x].right.total_cmp(&spans[y].right))
            .filter(|&j| d.left - spans[j].right < 3.0 * d.size && d.cy() < spans[j].cy() - 0.2 * d.size)
            .map(|j| spans[j].cy());
        let like = match (before, after) {
            // (… and the index before them an upper one)
            (Some(b), None) if ellipsis && level(b) && raised.is_some_and(|c| spans[b].cy() < c) => b,
            (Some(b), Some(a)) => {
                if !level(a) && !level(b) {
                    continue;
                }
                let like = if level(b) && (!level(a) || spans[b].size >= spans[a].size) { b } else { a };
                // (the dots stand with the indices, not with an index's own number)
                if ellipsis && spans[a].size != spans[b].size { if spans[a].size > spans[b].size { a } else { b } } else { like }
            }
            _ => continue,
        };
        let (top, bottom, size) = (spans[like].top, spans[like].bottom, spans[like].size);
        let d = &mut spans[k];
        (d.text, d.top, d.bottom, d.size) = (if ellipsis { "…" } else { "·" }.to_string(), top, bottom, size);
    }
    spans
}

/// The comma between two groups of indices, D_{μν,λσ}: set in the size of
/// the line, with small letters of one height close on both sides and no
/// letter of the line between them. It is a comma among the indices.
fn index_commas(mut spans: Vec<MSpan>) -> Vec<MSpan> {
    for k in 0..spans.len() {
        let d = &spans[k];
        if d.text.trim() != "," || d.atom.is_some() {
            continue;
        }
        let small = |j: &usize| !spans[*j].cmex() && spans[*j].size <= 0.85 * d.size && spans[*j].top < d.bottom && spans[*j].bottom > d.top;
        let before = (0..spans.len()).filter(small).find(|&j| (d.left - spans[j].right).abs() <= 0.2 * d.size);
        let after = (0..spans.len()).filter(small).find(|&j| (spans[j].left - d.right).abs() <= 0.35 * d.size);
        let (Some(b), Some(a)) = (before, after) else { continue };
        // (what follows the comma first is the small letter, not a letter
        // of the line with an index of its own: x_n, t_n)
        let first_after = (0..spans.len()).filter(|&j| j != k && spans[j].left >= d.right - 1.0 && spans[j].top < d.bottom && spans[j].bottom > d.top).min_by(|x, y| spans[*x].left.total_cmp(&spans[*y].left));
        let lowered = |j: usize| spans[j].bottom >= d.bottom && spans[j].top > d.top + 0.2 * d.size;
        if first_after != Some(a) || !lowered(a) || !lowered(b) || (spans[a].top - spans[b].top).abs() > 0.15 * d.size || (spans[a].size - spans[b].size).abs() > 0.5 {
            continue;
        }
        let (top, bottom, size) = (spans[b].top, spans[b].bottom, spans[b].size);
        let d = &mut spans[k];
        (d.top, d.bottom, d.size) = (top, bottom, size);
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

/// How far down from its bar a root sign reaches: TeX has the sign in
/// five heights, each its own character, and what stands under the bar
/// is no taller than the sign. (Under a sign of another font: room for a
/// fraction.)
fn root_depth(sign: &MSpan, size: f32) -> f32 {
    // (not the fonts of other makers read as "CMEX/MTEX": their signs
    // have other heights)
    if sign.family.contains('/') {
        return 2.5 * size;
    }
    let steps = match (sign.family.get(..4), sign.text.trim()) {
        (Some("CMSY"), "p" | "√") => 1.2,
        (Some("CMEX"), "p") => 1.4,
        (Some("CMEX"), "q") => 2.0,
        (Some("CMEX"), "r") => 2.6,
        (Some("CMEX"), "s") => 3.2,
        _ => return 2.5 * size,
    };
    steps * sign.size.max(0.6 * size)
}

/// How far under its bar the middle of a root sign is: half the sign's
/// height (the line of a fraction under a tall sign), half a line for a
/// sign of another maker's font, whose height is not known.
fn sign_middle(sign: &MSpan, size: f32) -> f32 {
    if sign.family.contains('/') {
        0.5 * size
    } else {
        (0.5 * root_depth(sign, size)).max(0.5 * size)
    }
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
        let over = |s: &MSpan| s.atom.is_none() && !s.cmex() && s.left < b.right - 2.0 && s.right > b.right + 3.0 && s.top >= b.bottom - 2.0 && s.top <= b.bottom + 0.6 * size;
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
                // (… by more than the space TeX leaves at a fraction's two
                // sides: two short bars of nearly one length are siblings)
                let narrower = r.right - r.left < 0.9 * (b.right - b.left) && r.right - r.left <= b.right - b.left - 2.0;
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
                .filter(|&i| i != ri && within(&spans[i]) && (spans[i].top >= b.top - 2.0 || (spans[i].top >= b.top - 0.35 * size && spans[i].cy() >= b.bottom + 0.3 * size)) && spans[i].cy() <= b.bottom + root_depth(&spans[ri], size))
                .collect();
            if under.is_empty() {
                continue;
            }
            used[bi] = true;
            let mut nested = inner(&mut used, false);
            // (… the bars under the sign: not that of a small fraction in
            // the row below it)
            let depth_down = b.bottom + root_depth(&spans[ri], size);
            for (k, r) in bars.iter().enumerate() {
                if nested.contains(r) && r.top > depth_down {
                    used[k] = false;
                }
            }
            nested.retain(|r| r.top <= depth_down);
            // (the bar of a fraction that fills the root is nearly as wide
            // as the root's own: under it and within its width, it is the
            // root's all the same)
            let mut under = under;
            for (k, r) in bars.iter().enumerate() {
                if !used[k] && k != bi && r.top > b.bottom && r.top <= depth_down && r.bottom <= b.bottom + 3.5 * size && r.left >= b.left - 2.0 && r.right <= b.right + 2.0 && under.iter().any(|&i| spans[i].bottom <= r.top + 2.0 && spans[i].left < r.right && spans[i].right > r.left) {
                    used[k] = true;
                    nested.push(*r);
                }
            }
            // (with its denominator, however far down that reaches)
            for r in nested.iter().filter(|r| r.right - r.left >= 0.6 * (b.right - b.left)) {
                for (i, s) in spans.iter().enumerate() {
                    let c = (s.left + s.right) / 2.0;
                    if i != ri && !under.contains(&i) && c >= r.left - 2.0 && c <= r.right + 2.0 && (s.top >= r.bottom - 2.0 || (s.top >= r.bottom - 0.35 * size && s.cy() >= r.bottom + 0.3 * size)) && s.cy() <= r.bottom + 1.7 * size {
                        under.push(i);
                    }
                }
            }
            let mut body: Vec<MSpan> = under.iter().map(|&i| spans[i].clone()).collect();
            level_brackets(&mut body);
            let t = format!("\\sqrt{{{}}}", build_at(&resolve_bars(body.clone(), &nested, depth + 1), depth + 1));
            let left = spans[ri].left;
            // (a root of small letters, in an exponent, is a small piece
            // at their height)
            let letters = body.iter().filter(|s| !s.cmex()).map(|s| s.size).fold(0.0f32, f32::max);
            let main: Vec<&MSpan> = body.iter().filter(|s| !s.cmex() && s.size >= 0.9 * letters).collect();
            let (top, bottom) = (main.iter().map(|s| s.top).fold(f32::MAX, f32::min), main.iter().map(|s| s.bottom).fold(f32::MIN, f32::max));
            // (the small letters go on from those right before the sign,
            // at one height: e^{−mr√(s²+1)})
            let goes_on = (0..spans.len()).any(|i| {
                let n = &spans[i];
                // (… or the small letters go on right behind its bar: the
                // root opens the exponent, e^{√(−2mE) x/ħ})
                let before = n.right <= left + 1.0 && left - n.right < letters;
                let behind = n.left >= b.right - 1.0 && n.left - b.right <= 0.35 * letters;
                i != ri && !under.contains(&i) && !n.cmex() && n.size >= 0.9 * letters && n.size < 0.85 * size && (before || behind) && (n.cy() - (top + bottom) / 2.0).abs() < 0.4 * letters
            });
            // (a root over a fraction that fills it has small letters though
            // it stands on the line: the fraction's halves are set small. In
            // an exponent they would be smaller still.)
            let on_line_fraction = letters >= 0.6 * size && nested.iter().any(|r| r.right - r.left >= 0.6 * (b.right - b.left));
            let (top, bottom, size) = if letters > 0.0 && letters < 0.85 * size && goes_on && !on_line_fraction {
                (top, bottom, letters)
            } else if let Some(f) = nested.iter().filter(|r| r.right - r.left >= 0.6 * (b.right - b.left)).min_by(|x, y| x.top.total_cmp(&y.top)) {
                // (a root over a fraction stands on the line at the fraction's bar)
                let y = (f.top + f.bottom) / 2.0;
                (y - size / 2.0, y + size / 2.0, size)
            } else {
                (b.bottom + 1.0, b.bottom + 1.0 + size, size)
            };
            let atom = MSpan {
                text: String::new(),
                left,
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
            .filter(|&i| {
                let s = &spans[i];
                // (the box of a sign of a symbol font hangs lower than the
                // letters': the minus of −1 over 2 ends a little under the bar)
                // (… by its own size: the small minus of a small fraction)
                let sign = s.atom.is_none() && matches!(s.text.trim(), "−" | "±" | "∓") && s.bottom <= b.top + 0.35 * s.size.min(size) && s.cy() <= b.top - 0.3 * s.size.min(size);
                within(s) && (s.bottom <= b.top + 2.0 || sign) && s.cy() >= b.top - 3.5 * size
            })
            .collect();
        // (a root sign in the denominator reaches up past the bar; its box
        // has its middle at the bar, a point over or under it)
        let below: Vec<usize> = (0..spans.len())
            .filter(|&i| {
                let s = &spans[i];
                // (a sign of a symbol font has a box taller than the letters':
                // the minus of √−g starts a little above the bar it is under)
                let tall_under = s.top >= b.bottom - 0.35 * size && s.cy() >= b.bottom + 0.3 * size;
                // (… or, a larger sign set a little higher: its own bar begins
                // where it ends, right under this bar)
                let own_bar_under = is_radical(s) && bars.iter().any(|r| r.top > b.bottom - 0.5 && r.top <= b.bottom + 0.5 * size && (r.left - s.right).abs() <= 3.0);
                within(s) && (s.top >= b.bottom - 2.0 || tall_under || (is_radical(s) && s.cy() >= b.top - 2.0) || own_bar_under) && s.cy() <= b.bottom + 3.5 * size
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
            ((r.right - r.left < 0.9 * width && r.right - r.left <= width - 2.0) || root_bar(r)) && c >= b.left - 2.0 && c <= b.right + 2.0
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
        // (not the half of a small stacked fraction, with its small other
        // half right across the bar: the d of 2^{d/2−1} close behind the 2)
        let touches = |u: &MSpan| (u.top - b.bottom).max(b.top - u.bottom).abs() <= 2.0;
        let halves = |s: &MSpan| touches(s) && spans.iter().any(|u| u.size < 0.9 * size && within(u) && touches(u) && (u.cy() < b.top) != (s.cy() < b.top));
        let hangs_outside = |s: &MSpan| {
            s.size < 0.9 * size
                && !halves(s)
                && spans.iter().any(|t| {
                    let c = (t.left + t.right) / 2.0;
                    t.size >= 0.9 * size
                        // (a sign has no index: the c of "−1/c²" set small,
                        // close behind the minus, is the denominator's)
                        && !(t.atom.is_none() && matches!(t.text.trim(), "−" | "+" | "=" | "±" | "∓" | "×" | "·"))
                        && (c < b.left || c > b.right)
                        && (s.left - t.right).abs() <= 1.5
                        && s.top < t.bottom
                        && s.bottom > t.top
                })
        };
        // (the lower limit of a sum over a small fraction that is the upper
        // limit of the sum in the row below is no part of its numerator;
        // nor the upper limit of the sum under a fraction part of its
        // denominator)
        let foreign = |s: &MSpan, up: bool| sum_limit(s, &spans, size).is_some_and(|(sign, lower)| if up { lower && b.top - sign > 2.2 * size + 2.0 } else { !lower && sign - b.bottom > 1.6 * size });
        let above: Vec<usize> = above.into_iter().filter(|&i| !blocked(&spans[i]) && !hangs_outside(&spans[i]) && !foreign(&spans[i], true)).collect();
        let below: Vec<usize> = below.into_iter().filter(|&i| !blocked(&spans[i]) && !hangs_outside(&spans[i]) && !foreign(&spans[i], false)).collect();
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
        // ... and as far as an integral or a sum that stands right at the
        // bar reaches, with its limits
        let op_reach = |up: bool| {
            above
                .iter()
                .chain(&below)
                .map(|&i| &spans[i])
                .filter(|s| within(s))
                .filter_map(|s| op_depth(s).map(|d| (op_top(s), d * s.size, s.size)))
                .filter(|&(top, depth, em)| if up { top < b.top && b.top - (top + depth) <= 1.2 * em } else { top >= b.bottom - 2.0 && top - b.bottom <= 0.9 * em })
                .map(|(top, depth, em)| if up { b.top - top + 1.0 * em } else { top + depth - b.bottom + 1.0 * em })
                .fold(0.0f32, f32::max)
        };
        let (reach_up, reach_down) = (reach_up.max(tall_reach(true)).max(op_reach(true)), reach_down.max(tall_reach(false)).max(op_reach(false)));
        let above: Vec<usize> = above.into_iter().filter(|&i| b.top - spans[i].top <= reach_up).collect();
        let below: Vec<usize> = below.into_iter().filter(|&i| spans[i].bottom - b.bottom <= reach_down).collect();
        let num = stack(&spans, &above, |s| b.top - s.bottom, size);
        let den = stack(&spans, &below, |s| s.top - b.bottom, size);
        if num.is_empty() || den.is_empty() {
            continue;
        }
        used[bi] = true;
        let n: Vec<MSpan> = num.iter().map(|&i| spans[i].clone()).collect();
        let d: Vec<MSpan> = den.iter().map(|&i| spans[i].clone()).collect();
        // (the bars among its numerator's pieces and among its
        // denominator's: not those of the fraction of the row above or
        // below, a little narrower than this one, in a system of equations)
        let (n_top, d_bottom) = (n.iter().map(|s| s.top).fold(f32::MAX, f32::min), d.iter().map(|s| s.bottom).fold(f32::MIN, f32::max));
        let mut nbars = inner(&mut used, true);
        let mut dbars = inner(&mut used, false);
        for (k, r) in bars.iter().enumerate() {
            let beyond = (nbars.contains(r) && r.bottom < n_top - 0.6 * size) || (dbars.contains(r) && r.top > d_bottom + 0.6 * size);
            if beyond {
                used[k] = false;
            }
        }
        nbars.retain(|r| r.bottom >= n_top - 0.6 * size);
        dbars.retain(|r| r.top <= d_bottom + 0.6 * size);
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
    // (a prime that opens an exponent stands a little higher than the
    // number after it, m′²: one exponent)
    let s = s.replace("^{{}^{\\prime} ", "^{\\prime ").replace("^{{}^{\\prime}}", "^{\\prime}");
    // (the sign of degrees is of the figure before it, also behind a space)
    let s = s.replace(" {}^{\\circ}", "^{\\circ}");
    // (the long arrow, set as a minus sign run into an arrow)
    let s = s.replace("-\\to ", "\\longrightarrow ").replace("- \\to ", "\\longrightarrow ");
    let s = if let Some(head) = s.strip_suffix("-\\to") { format!("{head}\\longrightarrow") } else { s };
    let s = barred_index(&rooted_fraction(&s));
    slashed(&s.replace("\\not =", "\\neq").replace("\\not=", "\\neq").replace("\\not{=}", "\\neq"))
}

/// The group in braces that `t` opens with: where its inside ends.
fn braced(t: &str) -> Option<usize> {
    let mut depth = 0usize;
    for (i, c) in t.char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ if depth == 0 => return None,
            _ => {}
        }
    }
    None
}

/// An index with a bar over it under an exponent, "sinh" with χ raised
/// and a barred 2 lowered: a small fraction, χ/2, whose bar was read as
/// drawn over its lower half.
fn barred_index(s: &str) -> String {
    const OPEN: &str = "_{\\overline{";
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(i) = rest.find(OPEN) {
        let inner = i + OPEN.len() - 1;
        let parts = braced(&rest[inner..]).and_then(|b| {
            let under = &rest[inner + 1..inner + b];
            let after = rest[inner + b + 1..].strip_prefix("}^")?;
            let e = braced(after)?;
            Some((under, &after[1..e], rest.len() - after.len() + e + 1))
        });
        match parts {
            Some((under, over, end)) => {
                out.push_str(&rest[..i]);
                out.push_str(&format!(" \\tfrac{{{over}}}{{{under}}}"));
                rest = &rest[end..];
            }
            None => {
                out.push_str(&rest[..inner]);
                rest = &rest[inner..];
            }
        }
    }
    out.push_str(rest);
    out
}

/// A root sign with one small piece over another right behind it is the
/// root of a small fraction, √⅔: the bars of the root and of the fraction
/// are too short to have been read, and the two pieces come out as the
/// sign's exponent and index, which a root sign does not have.
fn rooted_fraction(s: &str) -> String {
    const SIGN: &str = "\\surd_{";
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    // (the group in braces that `t` opens with: its inside and what follows)
    let group = |t: &str| -> Option<(usize, usize)> {
        let mut depth = 0usize;
        for (i, c) in t.char_indices() {
            match c {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some((1, i));
                    }
                }
                _ => {}
            }
        }
        None
    };
    while let Some(i) = rest.find(SIGN) {
        let from = i + SIGN.len() - 1;
        let parts = group(&rest[from..]).and_then(|(a, b)| {
            let under = &rest[from + a..from + b];
            let after = &rest[from + b + 1..];
            let (c, d) = group(after.strip_prefix('^')?)?;
            Some((under, &after[1 + c..1 + d], from + b + 1 + 1 + d + 1))
        });
        match parts {
            Some((under, over, end)) => {
                out.push_str(&rest[..i]);
                out.push_str(&format!("\\sqrt{{\\tfrac{{{over}}}{{{under}}}}}"));
                rest = &rest[end..];
            }
            None => {
                out.push_str(&rest[..from]);
                rest = &rest[from..];
            }
        }
    }
    out.push_str(rest);
    out
}

/// The stroke through a letter (Feynman's slash: p̸, ∂̸) as the letter with
/// a "/" drawn back over it. KaTeX sets "\not{p}" as a stroke beside the
/// letter, with a gap; this form is plain TeX and looks the same in every
/// renderer. A stroke through a sign ("\not{\equiv}") stays as it is.
fn slashed(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(i) = rest.find("\\not{") {
        out.push_str(&rest[..i]);
        let from = i + 5;
        let mut depth = 1;
        let end = rest[from..].char_indices().find_map(|(j, c)| {
            depth += i32::from(c == '{') - i32::from(c == '}');
            (depth == 0).then_some(from + j)
        });
        let Some(end) = end else {
            out.push_str(&rest[i..]);
            return out;
        };
        let inner = rest[from..end].trim();
        let sign = inner.starts_with(['=', '<', '>']) || ["equiv", "in", "sim", "simeq", "approx", "subset", "supset", "leq", "geq", "le", "ge", "cong", "parallel", "exists"].iter().any(|r| inner.strip_prefix('\\').is_some_and(|x| x == *r));
        if sign || inner.is_empty() {
            out.push_str(&rest[i..=end]);
        } else {
            out.push_str(&format!("{{{inner}\\!\\!\\!/}}"));
        }
        rest = &rest[end + 1..];
    }
    out.push_str(rest);
    out
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

/// The limit of a sum or product sign: a piece set under the sign or
/// over it (the sign's box is only its top). Gives the sign's top and
/// whether the piece is the lower limit.
fn sum_limit(s: &MSpan, spans: &[MSpan], size: f32) -> Option<(f32, bool)> {
    spans
        .iter()
        .filter(|b| matches!(big_op(b), Some("\\sum" | "\\prod")) && s.left < b.right && s.right > b.left && !s.cmex() && s.size < 0.9 * size)
        .find_map(|b| {
            if s.top >= b.top + 0.8 * size && s.top - b.top <= 2.2 * size {
                Some((b.top, true))
            } else if s.bottom <= b.top + 2.0 && b.top - s.bottom <= 0.8 * size {
                Some((b.top, false))
            } else {
                None
            }
        })
}

/// What hangs at the foot of a tall bar and stands at its head, right
/// behind it ("taken at x = a/2", "from 0 to a/2"): taken out of `rest`
/// and written as the bar's index and exponent.
fn bar_scripts(rest: &mut Vec<MSpan>, d: &Delim, size: f32, depth: usize) -> String {
    let (mid, quarter) = ((d.top + d.bottom) / 2.0, 0.25 * (d.bottom - d.top));
    let mut out = String::new();
    for (lower, mark) in [(true, '_'), (false, '^')] {
        let small = |s: &MSpan| !s.cmex() && s.size < 0.9 * size;
        let side = |s: &MSpan| if lower { s.cy() >= mid + quarter } else { s.cy() <= mid - quarter };
        let Some(seed) = rest.iter().filter(|s| small(s) && side(s) && s.left >= d.right - 1.0 && s.left <= d.right + 0.4 * size).min_by(|a, b| a.left.total_cmp(&b.left)).cloned() else { continue };
        // (and what goes on from it at its height)
        let mut reach = seed.right;
        let mut taken: Vec<usize> = Vec::new();
        loop {
            let next = (0..rest.len()).find(|&i| !taken.contains(&i) && small(&rest[i]) && side(&rest[i]) && rest[i].left >= d.right - 1.0 && rest[i].left <= reach + 0.5 * size && (rest[i].cy() - seed.cy()).abs() <= 0.6 * size);
            let Some(i) = next else { break };
            reach = reach.max(rest[i].right);
            taken.push(i);
        }
        let part: Vec<MSpan> = taken.iter().map(|&i| rest[i].clone()).collect();
        let mut k = 0;
        rest.retain(|_| {
            k += 1;
            !taken.contains(&(k - 1))
        });
        let tex = build_at(&part, depth + 1);
        if !tex.is_empty() {
            out.push_str(&format!("{mark}{{{tex}}}"));
        }
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
    // (a bar with a small index at its foot is the bar of "taken at": it
    // closes, and opens nothing — "dψ/dx |_{+ε} − dψ/dx |_{−ε}" is no
    // absolute value of what stands between the two bars)
    // (such a bar closes no bracket of another kind: the brace of a system
    // of equations stays open past the bar in one of its rows)
    let mut taken_at = vec![false; delims.len()];
    for k in 0..delims.len() {
        let d = delims[k];
        let foot = rest.iter().any(|s| !s.cmex() && s.size < 0.9 * size && s.left >= d.right - 1.0 && s.left <= d.right + 0.4 * size && s.cy() > (d.top + d.bottom) / 2.0 + 0.2 * (d.bottom - d.top) && s.top < d.bottom + 0.5 * size);
        // (… with nothing of the formula right behind the bar at its
        // middle: the 0 under an integral sign that follows an opening
        // bar is no such index)
        let mid = (d.top + d.bottom) / 2.0;
        let behind = spans.iter().any(|s| !is_piece(s) && s.left >= d.right - 1.0 && s.left <= d.right + 0.6 * size && ((s.cmex() && s.top >= d.top - size && s.top <= d.bottom) || (s.size >= 0.9 * size && s.top < mid && s.bottom > mid)));
        let foot = foot && !behind;
        if side[k].is_none() && d.kind == Some('|') && foot {
            side[k] = Some(false);
            taken_at[k] = true;
        }
    }
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
    // (… unless that bracket is as tall as the bar, head and foot:
    // "\left\{ … \right|" with the limits at the bar)
    let closes = |o: usize, k: usize| matches!(delims[o].kind, Some('|') | None) || ((delims[o].top - delims[k].top).abs() <= 3.0 && (delims[o].bottom - delims[k].bottom).abs() <= 3.0);
    let mut stack: Vec<usize> = Vec::new();
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    let mut lone: Vec<usize> = Vec::new();
    for k in 0..delims.len() {
        if opens[k] {
            stack.push(k);
        } else if let Some(pos) = stack.iter().rposition(|&o| same_height(&delims[o], &delims[k]) && !(taken_at[k] && !closes(o, k))) {
            // (of two rows that each have brackets of their own, one row
            // right under the other, a bracket closes the one of its row)
            let level = |o: usize| delims[o].kind.is_some() == delims[k].kind.is_some() && (delims[o].top - delims[k].top).abs() <= 2.0 && (delims[o].bottom - delims[k].bottom).abs() <= 2.0;
            let pos = stack.iter().rposition(|&o| same_height(&delims[o], &delims[k]) && level(o) && !(taken_at[k] && !closes(o, k))).unwrap_or(pos);
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
    // A bar of "taken at" that closed nothing stands where it is, a piece
    // of its row (in a system of equations: inside the brace), with its
    // index hanging on it.
    lone.retain(|&k| {
        if taken_at[k] {
            let d = delims[k];
            let scripts = bar_scripts(&mut rest, &d, size, depth);
            rest.push(piece(d.left, d.right, (d.top + d.bottom) / 2.0, format!("{}{scripts}", if d.bottom - d.top > 2.0 * size { "\\Bigg|" } else { "\\Big|" })));
        }
        !taken_at[k]
    });
    for (o, c) in pairs {
        let (d0, d1) = (delims[o], delims[c]);
        // boxes of shapeless pieces mark only the tops of the pieces; the
        // lowest piece reaches further down than its box
        let (top, bottom) = (d0.top.min(d1.top), d0.bottom.max(d1.bottom));
        // (a big ∫ or a slanted "/p" right after a bracket reaches back over
        // its edge: the middle of a span counts)
        // (the brackets of a row with brackets of the row under it, or
        // over it, at the same place: what they hold ends where those
        // begin, and begins where those end. Of shapeless ones only the
        // top is known.)
        let shapeless = d0.kind.is_none() && d1.kind.is_none();
        let typed_pair = d0.kind.is_some() && d1.kind.is_some();
        let other_row = |d: &&Delim| d.left < d1.right && d.right > d0.left && ((shapeless && d.kind.is_none()) || (typed_pair && d.kind.is_some()));
        let floor = delims.iter().filter(other_row).filter(|d| d.top > top + 0.8 * size && (shapeless || d.top >= bottom - 0.25 * size)).map(|d| d.top).fold(f32::MAX, f32::min);
        let ceiling = if shapeless && delims.iter().filter(other_row).any(|d| d.top < top - 0.8 * size) {
            top - 0.3 * size
        } else if typed_pair {
            delims.iter().filter(other_row).filter(|d| d.top < top - 0.8 * size && d.bottom <= top + 0.25 * size).map(|d| d.bottom - 0.25 * size).fold(top - 0.4 * size, f32::max)
        } else {
            top - size
        };
        let (inside, outside): (Vec<MSpan>, Vec<MSpan>) = rest.into_iter().partition(|s| {
            let c = (s.left + s.right) / 2.0;
            let (l, r) = (s.left.max(c.min(d0.right)), s.right.min(c.max(d1.left)));
            l >= d0.right - 1.0
                && r <= d1.left + 1.0
                && s.cy() >= ceiling
                && s.cy() < floor
                // (the lower limit of a sum in the row above reaches down
                // to the bracket's head, the upper limit of one in the row
                // below up to its foot: each is of its own sign)
                && !(typed_pair && sum_limit(s, spans, size).is_some_and(|(sign, lower)| if lower { sign < top - 0.5 * size } else { sign > bottom }))
                // (a bracket read from its outline has its true height: what
                // stands under its foot, in the row below, is not in it)
                && s.cy() <= if typed_pair { bottom + 0.5 * size } else { (bottom + 1.5 * size).max((top + bottom) / 2.0 + 3.0 * size) }
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
        // (closed by a bar of "taken at": the rows in the open bracket and
        // the bar, with what hangs at the bar)
        let by_bar = close == '|' && open != '|' && taken_at[c];
        let tex = if by_bar {
            let scripts = bar_scripts(&mut rest, &d1, size, depth);
            let body = matrix(&inside, depth, ' ', typed).unwrap_or_else(|| build_at(&inside, depth + 1));
            format!("\\left{} {body} \\right|{scripts}", delim_tex(open))
        } else {
            matrix(&inside, depth, open, typed)
                .or_else(|| (typed && open == '{' && close == '}').then(|| system_rows(&inside, depth)).flatten())
                .unwrap_or_else(|| format!("\\left{} {} \\right{}", delim_tex(open), build_at(&inside, depth + 1), delim_tex(close)))
        };
        if inside.is_empty() {
            continue; // nothing between them: nothing to write
        }
        // (a square bracket or a brace with a piece at its foot and one at
        // its head: "taken from −1 to 1", the marks of its two rows; both
        // hang on it — one over the other behind it they would read as a
        // fraction)
        // (… pieces at its own head and foot, not the index of the bracket
        // of the row above)
        let mut reach = d1.right;
        let tex = if typed && matches!(close, ']' | '}') && !by_bar && d1.bottom - d1.top > 1.5 * size {
            let (mut probe, others): (Vec<MSpan>, Vec<MSpan>) = rest.iter().cloned().partition(|s| s.cy() >= d1.top - 0.3 * size && s.cy() <= d1.bottom + 0.4 * size);
            let before = probe.clone();
            let scripts = bar_scripts(&mut probe, &d1, size, depth);
            if scripts.starts_with("_{") && scripts.contains("}^{") {
                let gone = |s: &&MSpan| !probe.iter().any(|p| p.left == s.left && p.top == s.top && p.text == s.text);
                reach = before.iter().filter(gone).map(|s| s.right).fold(reach, f32::max);
                rest = probe.into_iter().chain(others).collect();
                format!("{tex}{scripts}")
            } else {
                tex
            }
        } else {
            tex
        };
        // A bracket read from its outline has its true height, centred on
        // the formula's main line; shapeless pieces only mark their tops,
        // so then the middle of the contents is used.
        let mid = match (d0.kind, d1.kind) {
            (Some(_), _) => (d0.top + d0.bottom) / 2.0,
            (_, Some(_)) => (d1.top + d1.bottom) / 2.0,
            _ => inside.iter().map(|s| s.cy()).sum::<f32>() / inside.len() as f32,
        };
        rest.push(piece(d0.left, reach, mid, tex));
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
    if std::env::var("PDF2MD_MATHDEBUG").is_ok_and(|v| v == "4") {
        eprintln!("MATRIX rows {} size {size}", rows.len());
        for r in &rows {
            eprintln!("  row {:?}", r.iter().map(|s| format!("{}@{:.0}-{:.0},{:.0}-{:.0}/{}", s.atom.clone().unwrap_or(s.text.clone()), s.left, s.right, s.top, s.bottom, s.size)).collect::<Vec<_>>());
        }
    }
    if rows.len() < 2 {
        return None;
    }
    // columns: clusters of x positions shared by all rows
    let columns = |tol: f32| {
        let mut xs: Vec<(f32, f32)> = rows.iter().flat_map(|r| clusters_of(r, tol)).collect();
        xs.sort_by(|a, b| a.0.total_cmp(&b.0));
        let mut cols: Vec<(f32, f32)> = Vec::new();
        for (l, r) in xs {
            match cols.last_mut() {
                Some(c) if l <= c.1 => c.1 = c.1.max(r),
                _ => cols.push((l, r)),
            }
        }
        cols
    };
    // In a matrix every column has an entry in (nearly) every row. Fractions
    // inside big brackets do not: "i" on the middle row, "p·x" above "ħ".
    let filled = |&(l, r): &(f32, f32)| {
        rows.iter()
            .filter(|row| row.iter().any(|s| s.left < r && s.right > l))
            .count()
    };
    let full = |cols: &[(f32, f32)]| !cols.iter().any(|c| filled(c) * 4 < rows.len() * 3);
    // (the space around a sign in an entry, "σ·L + ħ", is wider than the
    // letters stand apart, narrower than the columns: where it cut an
    // entry in two, a column of its own that most rows lack, the entries
    // are told with more room)
    // (not the rows of a system in braces: they are lines, not entries)
    let rooms: &[f32] = if open == '{' { &[0.5] } else { &[0.5, 0.8] };
    let cols = rooms.iter().map(|t| columns(t * size)).find(|c| full(c)).unwrap_or_else(|| columns(0.5 * size));
    // Two rows with one column is as likely a fraction as a vector, unless
    // the bracket was read from its outline (then fraction bars were also
    // read and fractions are already resolved).
    if rows.len() == 2 && cols.len() < 2 && !typed {
        return None;
    }
    if !full(&cols) {
        return None;
    }
    let body: Vec<Vec<String>> = rows
        .iter()
        .map(|r| {
            cols.iter()
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
                .collect::<Vec<_>>()
        })
        .collect();
    // A system of equations in braces, one equation a row: lined up at
    // the "=" as a book sets it, the left sides ending there and the right
    // sides beginning there (the columns of a matrix are centered, which
    // leaves a short right side in the middle under a long one, and rows
    // that are one column each not lined up at all).
    if open == '{' {
        const RELATIONS: [&str; 15] = ["=", "\\approx", "\\equiv", "\\sim", "\\simeq", "\\propto", "<", ">", "\\leq", "\\geq", "\\rightarrow", "\\Rightarrow", "\\longrightarrow", "\\leftrightarrow", "\\Leftrightarrow"];
        // (a long arrow is a minus sign and an arrow as the cells are built)
        let arrow = |sign: &str| match sign.trim() {
            "-\\to" | "- \\to" => "\\longrightarrow",
            "\\to" => "\\rightarrow",
            sign => RELATIONS.iter().find(|r| **r == sign).copied().unwrap_or(""),
        };
        // (a last row of words alone, "etc.", stands under the right sides)
        let words = |cells: &[String]| {
            let row = cells.iter().map(|c| c.trim()).filter(|c| !c.is_empty()).collect::<Vec<_>>().join(" ");
            (row.starts_with("\\text{") && !row.contains('=')).then_some(row)
        };
        let lines: Option<Vec<String>> = body
            .iter()
            .enumerate()
            .map(|(k, cells)| match cells.as_slice() {
                cells if k >= 2 && k + 1 == body.len() && words(cells).is_some() => words(cells).map(|row| format!("& {row}")),
                [one] => top_level_equals(one).map(|p| format!("{} &= {}", &one[..p], &one[p + 3..])),
                [left, sign, right] if RELATIONS.contains(&arrow(sign)) && !left.trim().is_empty() => Some(format!("{left} &{} {right}", arrow(sign))),
                // (more columns, where the book lined its rows up at other
                // places as well: one equation a row still — not two side
                // by side, each with its "=", nor with a word between)
                cells if cells.iter().filter(|c| top_level_equals(c).is_some()).count() <= 1 && !cells.iter().any(|c| c.trim_start().starts_with("\\text{")) => {
                    let row = cells.iter().map(|c| c.trim()).filter(|c| !c.is_empty()).collect::<Vec<_>>().join(" ");
                    top_level_equals(&row).map(|p| format!("{} &= {}", &row[..p], &row[p + 3..]))
                }
                _ => None,
            })
            .collect();
        if let Some(lines) = lines {
            return Some(format!("\\left\\{{ \\begin{{aligned}} {} \\end{{aligned}} \\right\\}}", lines.join(" \\\\ ")));
        }
    }
    let body: Vec<String> = body.iter().map(|cells| cells.join(" & ")).collect();
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
    // a script touches the symbol it belongs to (the index of an index
    // touches a small one: the A of ℓ_A under a Y)
    let free = |s: &MSpan| !big.iter().any(|b| vgap(s, b) < 0.0 && hgap(s, b) <= 1.5) && !cells.iter().any(|b| !b.cmex() && b.atom.is_none() && b.size > 1.2 * s.size && vgap(s, b) < 0.0 && hgap(s, b) <= 1.5);
    // (nor is the limit of a sum free: it stands over or under its sign,
    // whose box is only the sign's top. The lower limit of one row's sum
    // and the upper limit of the next row's, one over the other, are no
    // block of a matrix.)
    let limit = |s: &MSpan| sum_limit(s, &cells, size).is_some();
    let mut cand: Vec<usize> = (0..cells.len()).filter(|&i| small(&cells[i]) && free(&cells[i]) && !limit(&cells[i])).collect();
    // (… and so does what goes on from a script at its height: the
    // "iE t/ħ" behind the "−" of e^{−iEt/ħ})
    loop {
        let on = |i: usize| {
            let s = &cells[i];
            (0..cells.len()).any(|j| small(&cells[j]) && !cand.contains(&j) && cells[j].right <= s.left + 1.0 && s.left - cells[j].right <= 2.0 && vgap(s, &cells[j]) < -0.5 * s.size.min(cells[j].size))
        };
        let Some(k) = (0..cand.len()).find(|&k| on(cand[k])) else { break };
        cand.remove(k);
    }
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

/// The lines of a system of equations in braces that is no matrix: its
/// equations are of unlike length, or one of them goes on in a second
/// line that begins with a sign, so the lines share no columns. Each line
/// has a relation sign or begins with a sign, and the lines stand a line
/// apart (the fractions are pieces of their lines by now, so a numerator
/// is no line). Lined up at their "=", a continuation set in under it.
fn system_rows(spans: &[MSpan], depth: usize) -> Option<String> {
    let size = spans.iter().filter(|s| !s.cmex()).map(|s| s.size).fold(0.0f32, f32::max).max(1.0);
    let rows = text_rows(spans);
    if rows.len() < 2 || rows.windows(2).any(|w| w[1][0].cy() - w[0][0].cy() < size) {
        return None;
    }
    let line = |row: &Vec<MSpan>| {
        let first = row.iter().filter(|s| !s.cmex() && s.size >= 0.9 * size).min_by(|a, b| a.left.total_cmp(&b.left));
        row.iter().any(|s| s.atom.is_none() && s.size >= 0.9 * size && is_relation(&s.text)) || first.is_some_and(|s| s.atom.is_none() && s.text.trim_start().starts_with(['+', '−', '-', '=']))
    };
    if !rows.iter().all(line) {
        return None;
    }
    let built: Vec<String> = rows.iter().map(|r| build_at(r, depth + 1)).collect();
    Some(format!("\\left\\{{ {} \\right\\}}", aligned(&built)))
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
            // (the "=" the rows line up at stands in no group or bracket)
            } else if let Some(p) = top_level_equals(&t) {
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
    // (the dots that stand for an entry, ". . .", one beside the other in
    // a matrix's last row: each is an entry, close as they stand)
    let dots = |s: &MSpan| s.atom.is_none() && matches!(s.text.trim(), "..." | "…" | "· · ·" | "···" | ". . .");
    let mut last_dots = false;
    for s in v {
        match out.last_mut() {
            Some(c) if s.left <= c.1 + tol && !(last_dots && dots(s) && s.left > c.1) => c.1 = c.1.max(s.right),
            _ => out.push((s.left, s.right)),
        }
        last_dots = dots(s);
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
    let mut levelled = spans.to_vec();
    level_brackets(&mut levelled);
    let (owned, bracket_items) = take_brackets(&levelled, depth);
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
            // (not the index of an index, with the larger index between it
            // and the symbol: the 1 of M^{λ₁…}, level with the M's middle)
            // (nor what stands under or over a sum sign, a little before
            // it: its limit, "k ≠ n", wider than the sign)
            // (… or begins a run of small pieces at one height that reaches
            // under it: the n of "n = 1, 3, 5 …")
            let mut reach = s.right;
            loop {
                let next = spans.iter().filter(|n| n.size < 0.9 * max && !n.cmex() && (n.cy() - s.cy()).abs() <= 0.25 * s.size && n.left >= s.left && n.left <= reach + 0.3 * max && n.right > reach).map(|n| n.right).fold(reach, f32::max);
                if next <= reach {
                    break;
                }
                reach = next;
            }
            let limit = bases.iter().any(|&o| matches!(big_op(&spans[o]), Some("\\sum" | "\\prod")) && reach > spans[o].left && s.left < spans[o].right && (s.cy() - y0).abs() > 0.5 * h);
            if limit {
                return false;
            }
            let between = (0..spans.len()).any(|j| j != i && j != b && spans[j].size > s.size && spans[j].size < 0.9 * max && spans[j].left >= spans[b].right - 1.5 && spans[j].right <= s.left + 1.5 && spans[j].right > spans[j].left);
            gap >= -0.2 * max && gap <= 0.45 * max && big_op(&spans[b]).is_none() && !between
        });
        if adjacent {
            let d = s.cy() - y0;
            let prime = matches!(s.text.trim(), "′" | "″" | "‴");
            role[i] = if prime {
                Role::Sup // k′ in a subscript sits level with the k
            // (by the letters' height: the taller boxes of signs in the row,
            // a minus, lower its middle towards an index)
            } else if d.abs() < 0.1 * h.min(max) {
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
                    // (a smaller piece right at a script's end, a little
                    // lower or higher: that script's own index)
                    let index = s.size < spans[j].size && gap.abs() <= 1.5 && (s.cy() - spans[j].cy()).abs() <= 0.5 * spans[j].height();
                    gap >= -0.2 * max && gap <= 0.45 * max && (level || index)
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
    // (big signs side by side: the limits of the two, level and close
    // together, are one group by their places — "p⃗" and "r = 1" under
    // Σ Σ, "Ω" and "α, μ" under ∫ Σ. It is cut at its widest gaps, one
    // for each sign more than the first.)
    let big_signs = |sums_only: bool| -> Vec<(f32, f32)> {
        let mut v: Vec<(f32, f32)> = bases.iter().map(|&b| &spans[b]).filter(|s| if sums_only { matches!(big_op(s), Some("\\sum" | "\\prod")) } else { big_op(s).is_some() }).map(|s| (s.left, s.right)).collect();
        v.sort_by(|a, b| a.0.total_cmp(&b.0));
        v
    };
    let (signs, all_signs) = (big_signs(true), big_signs(false));
    let cut_at_signs = |groups: Vec<Vec<usize>>, under: bool| -> Vec<Vec<usize>> {
        let mut out = Vec::new();
        for mut g in groups {
            let r = range(spans, &g);
            let over: Vec<&(f32, f32)> = all_signs.iter().filter(|s| overlaps(r, (s.0 - 0.2 * max, s.1 + 0.2 * max))).collect();
            if over.len() < 2 || !signs.iter().any(|s| overlaps(r, (s.0 - 0.2 * max, s.1 + 0.2 * max))) {
                out.push(g);
                continue;
            }
            g.sort_by(|&a, &b| spans[a].left.total_cmp(&spans[b].left));
            // where a cut may fall: before a piece that is no index of the
            // piece before it (place in the group, middle of the gap)
            let mut reach = f32::MIN;
            let mut gaps: Vec<(usize, f32, f32)> = Vec::new();
            for (k, &i) in g.iter().enumerate() {
                let index = k > 0 && spans[i].size < 0.9 * spans[g[k - 1]].size && spans[i].left - spans[g[k - 1]].right <= 1.5;
                if k > 0 && !index && spans[i].left >= reach - 1.0 {
                    gaps.push((k, (spans[i].left + reach) / 2.0, spans[i].left - reach));
                }
                reach = reach.max(spans[i].right);
            }
            // for each two signs, the gap nearest the middle between them
            let mut at: Vec<usize> = over
                .windows(2)
                .filter_map(|w| {
                    let x = (w[0].1 + w[1].0) / 2.0;
                    // (the limits of sums are set under and over their
                    // middle; an integral's stand behind it, and its upper
                    // one may reach over the next sign: "ω_cut−off". Beside
                    // an integral only lower limits are cut, at a real gap.)
                    let sums = signs.contains(w[0]) && signs.contains(w[1]);
                    gaps.iter().filter(|g| sums || (under && g.2 >= 0.15 * max)).min_by(|a, b| (a.1 - x).abs().total_cmp(&(b.1 - x).abs())).map(|g| g.0)
                })
                .collect();
            at.sort();
            at.dedup();
            let mut from = 0;
            for k in at.into_iter().chain([g.len()]) {
                out.push(g[from..k].to_vec());
                from = k;
            }
        }
        out
    };
    let (above, below) = (cut_at_signs(above, false), cut_at_signs(below, true));
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
                    // (not small letters that lead up to the sign from far
                    // on its left and end at it: the exponent the sign is in)
                    let (gl, gr) = range(spans, g);
                    let leads_up = gl < s.left - 1.5 * max && gr <= s.left + 0.3 * (s.right - s.left);
                    // (nor what stands under the sum sign beside this one,
                    // nearer that sign's middle)
                    let (gc, sc) = ((gl + gr) / 2.0, (s.left + s.right) / 2.0);
                    let neighbours = signs.iter().any(|q| (q.0 - s.left).abs() > 1.0 && overlaps((gl, gr), (q.0 - 0.2 * max, q.1 + 0.2 * max)) && ((q.0 + q.1) / 2.0 - gc).abs() < (sc - gc).abs());
                    if !used[k] && overlaps((gl, gr), r) && !leads_up && !neighbours {
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
            // An exponent of e that is all one integral, e^{∫d⁴x ½JK⁻¹J}:
            // the sign stands right behind the e, as large as on the line,
            // and the small raised letters after it, which are many and
            // begin with the "d", are no upper limit of it. They and the
            // sign are the exponent.
            let it = &items[i];
            let end = i + 1 + items[i + 1..].iter().take_while(|n| n.kind == Kind::Sup).count();
            let run: Vec<usize> = items[i + 1..end].iter().flat_map(|n| n.src.iter().copied()).collect();
            let exponent = out.last().is_some_and(|p| p.trim() == "e") && it.src.len() == 1 && big_op(&spans[it.src[0]]) == Some("\\int") && items[i + 1..end].iter().all(|n| !n.src.is_empty()) && {
                let sign = &spans[it.src[0]];
                let main: Vec<&MSpan> = run.iter().map(|&k| &spans[k]).filter(|r| !r.cmex()).collect();
                let hi = main.iter().map(|r| r.size).fold(0.0f32, f32::max);
                let letters: usize = main.iter().map(|r| r.text.chars().filter(|c| !c.is_whitespace()).count()).sum();
                let first = main.iter().min_by(|a, b| a.left.total_cmp(&b.left));
                hi < 0.85 * max && letters >= 6 && first.is_some_and(|r| r.text.trim().starts_with('d') && r.left >= sign.right - 2.0 && r.left - sign.right < max)
            };
            if exponent {
                let inside = build_at(&subset(spans, &run), depth + 1);
                out.push(format!("^{{\\int {inside}}}"));
                prev_right = items[i..end].iter().map(|n| n.right).fold(prev_right, |a, r| Some(a.map_or(r, |a| a.max(r))));
                i = end;
                continue;
            }
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
        if std::env::var("PDF2MD_MATHDEBUG").is_ok_and(|v| v == "3") {
            eprintln!("SCRIPTS sups {:?} subs {:?} next {:?}", sups.iter().map(|it| &it.tex).collect::<Vec<_>>(), subs.iter().map(|it| &it.tex).collect::<Vec<_>>(), items.get(i).map(|it| (&it.tex, it.src.len())));
        }
        while let Some(op) = items.get(i).filter(|it| it.kind == Kind::Atom && (it.src.first().is_some_and(|&k| big_op(&spans[k]).is_some()) || raised_dot(it))) {
            // (two signs, ∫∫ d⁴x d⁴y …: what follows the second follows both)
            let signs = if raised_dot(op) { 0 } else { items[i + 1..].iter().take_while(|it| it.kind == Kind::Atom && it.src.len() == 1 && big_op(&spans[it.src[0]]).is_some()).count() };
            let start = i + 1 + signs;
            let end = start + items[start..].iter().take_while(|it| it.kind != Kind::Atom).count();
            let run: Vec<usize> = items[start..end].iter().flat_map(|it| it.src.iter().copied()).collect();
            // (the sign may have taken what follows it for its upper limit,
            // where its lower limit, the C of a line integral, led up to it)
            let (run, taken) = if run.is_empty() && op.src.len() > 2 { (op.src[1..].to_vec(), true) } else { (run, false) };
            let before: Vec<usize> = if folded.is_empty() { sups.iter().flat_map(|it| it.src.iter().copied()).collect() } else { folded.clone() };
            // (an exponent of e that begins with the sign, e^{∫d⁴x …}: the
            // small letters after the sign stand raised, and are many)
            let opens = before.is_empty() && !run.is_empty() && out.last().is_some_and(|p| p.trim() == "e") && !raised_dot(op) && {
                let main: Vec<&MSpan> = run.iter().map(|&k| &spans[k]).filter(|r| !r.cmex()).collect();
                let hi = main.iter().map(|r| r.size).fold(0.0f32, f32::max);
                let letters: usize = main.iter().map(|r| r.text.chars().filter(|c| !c.is_whitespace()).count()).sum();
                let first = main.iter().min_by(|a, b| a.left.total_cmp(&b.left)).map_or("", |r| r.text.trim());
                !main.is_empty() && hi < 0.85 * spans[op.src[0]].size.max(hi) + f32::EPSILON && main.iter().filter(|r| r.size >= 0.9 * hi).all(|r| r.cy() < y0 - 0.25 * h) && letters >= 4 && first.starts_with('d')
            };
            if opens {
                folded.extend(items[i..start].iter().flat_map(|it| it.src.iter().copied()));
                folded.extend(run);
                prev_right = items[i..end].iter().map(|it| it.right).fold(prev_right, |a, r| Some(a.map_or(r, |a| a.max(r))));
                i = end;
                continue;
            }
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
            let (y_before, size_before) = level(&before);
            // (a sum whose sign has taken its index and the summed letters
            // behind it for its limit: letters of the exponent's size at
            // its height, right of the sign, are no limit — they and the
            // index are looked at with what follows)
            let sign_right = spans[op.src[0]].right;
            // (a sign with a lower limit hanging at its foot, under the
            // exponent's height and under the middle of the line (the limits
            // of an integral inside an exponent all stand over it), is a
            // sign of the line set small, with
            // both its limits: "C₁² ∫_{−∞}^{−a/2} …" — the upper limit
            // stands level with the 2 before the sign)
            let lower_limit = !raised_dot(op) && op.src[1..].iter().chain(run.iter()).any(|&k| !spans[k].cmex() && spans[k].cy() > y_before + 0.6 * size_before && spans[k].cy() > y0 + 0.1 * h && spans[k].left >= spans[op.src[0]].left - 1.0 && spans[k].left <= sign_right + 0.8 * size_before);
            if lower_limit {
                break;
            }
            let summed_in = op.src[1..].iter().filter(|&&k| !spans[k].cmex() && (spans[k].size - size_before).abs() <= 0.1 * size_before && (spans[k].cy() - y_before).abs() <= 0.4 * size_before && spans[k].left >= sign_right - 1.0).count();
            let widened = !taken && !raised_dot(op) && summed_in >= 2;
            let probe: Vec<usize> = if widened { op.src[1..].iter().copied().chain(run.iter().copied()).collect() } else { run.clone() };
            let run_all = &probe;
            let (y_run, size_run) = level(run_all);
            if (size_before - size_run).abs() > 0.1 * size_before || (y_before - y_run).abs() > 0.4 * size_before {
                if trace {
                    eprintln!("FOLD no: sizes {size_before} {size_run} heights {y_before} {y_run} op {:?} taken {taken} run {:?}", op.tex, run.iter().map(|&k| (&spans[k].text, spans[k].size, spans[k].cy(), spans[k].atom.is_some())).collect::<Vec<_>>());
                }
                break;
            }
            // (not the limits of an integral on the line, which are small
            // and raised too: what follows runs on at the exponent's
            // height; nor an integral after another's limits)
            let on_level: Vec<&MSpan> = run_all.iter().map(|&k| &spans[k]).filter(|r| !r.cmex() && r.size >= 0.9 * size_run && (r.cy() - y_before).abs() <= 0.4 * size_before).collect();
            let width = on_level.iter().map(|r| r.right).fold(f32::MIN, f32::max) - on_level.iter().map(|r| r.left).fold(f32::MAX, f32::min);
            let after_op = out.last().is_some_and(|p| ["\\int", "\\sum", "\\prod", "\\oint"].iter().any(|o| p.trim_start().starts_with(o)));
            let dot = raised_dot(op);
            // (one piece will do when it is long, "dτ L(q)", or opens with
            // the d of what is integrated over, "dτ" before a second sign:
            // a limit is neither)
            let letters: usize = on_level.iter().map(|r| r.text.chars().filter(|c| !c.is_whitespace()).count()).sum();
            let first = on_level.iter().min_by(|a, b| a.left.total_cmp(&b.left)).map_or("", |r| r.text.trim());
            let integrand = (on_level.len() >= 2 && width >= 1.5 * max) || (letters >= 4 && width >= 1.5 * max) || (letters >= 2 && first.starts_with('d'));
            // (a sum with its index set beside it, lowered, and the summed
            // letters after that at the exponent's height: an upper limit
            // would stand over the index, not behind it)
            let first_left = on_level.iter().map(|r| r.left).fold(f32::MAX, f32::min);
            let index: Vec<&MSpan> = run_all.iter().map(|&k| &spans[k]).filter(|r| !r.cmex() && r.right <= first_left + 1.0).collect();
            // (with its upper limit too, a small letter over the index, where
            // the sign had taken them)
            let lowered = |r: &&MSpan| r.cy() > y_before + 0.3 * size_before;
            let summed = on_level.len() >= 2 && !index.is_empty() && index.iter().all(|r| r.size < 0.9 * size_run) && if widened { index.iter().any(lowered) } else { index.iter().all(lowered) };
            let integrand = integrand || summed;
            if after_op || (!dot && !integrand) || (dot && on_level.is_empty()) {
                if trace {
                    eprintln!("FOLD no: after_op {after_op} integrand {integrand} on_level {:?} width {width} y_before {y_before} run {:?}", on_level.iter().map(|r| &r.text).collect::<Vec<_>>(), run.iter().map(|&k| (&spans[k].text, spans[k].cy(), spans[k].size)).collect::<Vec<_>>());
                }
                break;
            }
            if dot {
                dots.push(op.src[0]);
            }
            folded = before;
            folded.extend(items[i..start].iter().flat_map(|it| it.src.iter().copied()));
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
    // (… or, with its dots, as long as "10.2.27")
    let ok = !inner.is_empty() && (inner.len() <= 6 || (inner.len() <= 8 && inner.matches('.').count() == 2)) && start && inner.chars().all(|c| c.is_ascii_alphanumeric() || c == '.');
    ok.then(|| inner.to_string())
}

/// Split a displayed block (all spans of its lines) into equations, each
/// with its LaTeX and optional equation number.
pub fn display(spans: Vec<MSpan>, bars: &[Rule], right_margin: f32, body: f32) -> Vec<(String, Option<String>)> {
    display_areas(spans, bars, right_margin, body).into_iter().map(|(latex, tag, _)| (latex, tag)).collect()
}

/// Where on the page pieces and bars stand, all of them together: left,
/// top, right, bottom.
pub type Area = [f32; 4];

fn bounds(spans: &[MSpan], bars: &[Rule]) -> Area {
    let a = spans.iter().fold([f32::MAX, f32::MAX, f32::MIN, f32::MIN], |a, s| [a[0].min(s.left), a[1].min(s.top), a[2].max(s.right), a[3].max(s.bottom)]);
    bars.iter().fold(a, |a, b| [a[0].min(b.left), a[1].min(b.top), a[2].max(b.right), a[3].max(b.bottom)])
}

fn both(a: Area, b: Area) -> Area {
    [a[0].min(b[0]), a[1].min(b[1]), a[2].max(b[2]), a[3].max(b[3])]
}

/// [`display`], each equation with the area of the page it was built
/// from (without its number): the place a second reading of it is cut from.
pub fn display_areas(
    spans: Vec<MSpan>,
    bars: &[Rule],
    right_margin: f32,
    body: f32,
) -> Vec<(String, Option<String>, Area)> {
    let (tags, mut rest): (Vec<MSpan>, Vec<MSpan>) = spans
        .into_iter()
        .partition(|s| tag_of(&s.text).is_some() && s.right >= right_margin - 1.5 * body);
    if let Some(cuts) = size_zones(&rest, bars) {
        let zone = |y: f32| cuts.iter().filter(|&&c| y > c).count();
        let mut out = Vec::new();
        for z in 0..=cuts.len() {
            let part: Vec<MSpan> = rest.iter().chain(&tags).filter(|s| zone(s.cy()) == z).cloned().collect();
            let pbars: Vec<Rule> = bars.iter().filter(|b| zone((b.top + b.bottom) / 2.0) == z).copied().collect();
            out.extend(display_areas(part, &pbars, right_margin, body));
        }
        return out;
    }
    level_punctuation(&mut rest);
    level_sizes(&mut rest);
    let rest = rest;
    if rest.is_empty() {
        return tags
            .iter()
            .map(|t| (String::new(), tag_of(&t.text), bounds(std::slice::from_ref(t), &[])))
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
    // (centre, has a relation sign, starts with + or −, starts with a
    // relation sign, is more than punctuation, + or − is its very first piece)
    let mut rows: Vec<(f32, bool, bool, bool, bool, bool)> = Vec::new();
    let mut cur: Vec<&MSpan> = Vec::new();
    let close_row = |cur: &Vec<&MSpan>, rows: &mut Vec<(f32, bool, bool, bool, bool, bool)>| {
        if !cur.is_empty() {
            let c = cur.iter().map(|s| s.cy()).sum::<f32>() / cur.len() as f32;
            // (an arrow relates the two sides of its line as "=" does)
            // (not an arrow over or under a fraction's bar)
            let in_fraction = |s: &MSpan| bars.iter().any(|b| s.left >= b.left - 2.0 && s.right <= b.right + 2.0 && (s.cy() - (b.top + b.bottom) / 2.0).abs() > 0.35 * max && (s.cy() - (b.top + b.bottom) / 2.0).abs() <= 1.3 * max);
            let rel = cur.iter().any(|s| is_relation(&s.text) || (s.text.contains('→') && !in_fraction(s)));
            let first = cur.iter().min_by(|a, b| a.left.total_cmp(&b.left)).unwrap();
            // (… the line's first piece, not a sign behind a fraction that opens it)
            let fraction_first = bars.iter().any(|b| b.left < first.left && ((b.top + b.bottom) / 2.0 - c).abs() <= 0.3 * max);
            let op = first.text.trim_start().starts_with(['+', '−', '-', '×', '·']);
            // (a line that begins with the sign goes on from the line above)
            let goes_on = is_relation(first.text.trim());
            // (more than the full stop that ends a line of text)
            let solid = cur.iter().any(|s| !s.text.trim().chars().all(|c| ",.;:".contains(c)));
            rows.push((c, rel, op, goes_on, solid, op && !fraction_first));
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
    // (… a denominator that opens with an integral stands lower: the row
    // of the sign, under the bar and within its width)
    let fraction_row = |c: f32| {
        bars.iter().any(|r| (c - (r.top + r.bottom) / 2.0).abs() <= 1.3 * max)
            || bars.iter().any(|r| {
                let under = c - (r.top + r.bottom) / 2.0;
                let members: Vec<&MSpan> = rest.iter().filter(|s| !s.cmex() && s.size >= 0.9 * max && (s.cy() - c).abs() <= 0.3 * max).collect();
                let sign = rest.iter().any(|s| op_depth(s).is_some() && s.left >= r.left - 2.0 && s.right <= r.right + 2.0 && (op_top(s) + 0.5 * op_depth(s).unwrap_or(0.0) * s.size - c).abs() <= 0.5 * max && op_top(s) >= r.bottom - 2.0);
                under > 1.3 * max && under <= 1.9 * max && sign && !members.is_empty() && members.iter().all(|s| s.left >= r.left - 2.0 && s.right <= r.right + 2.0)
            })
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
    // Two rows inside one tall bracket (read from its outline).
    let same_bracket = |p: f32, c: f32| tall.iter().any(|b| p > b.top - 0.5 * max && c < b.bottom + 0.5 * max);
    // Every bracket around this row is one line high, the row at its middle,
    // and closes in the row ("[" may close lines further down).
    let one_line = |c: f32| {
        !rest.iter().any(|s| s.cmex() && s.text.trim().is_empty())
            && tall.iter().filter(|b| c > b.top - 0.5 * max && c < b.bottom + max).all(|b| b.bottom - b.top < 2.2 * max && (c - b.cy()).abs() < 0.5 * max && tall.iter().any(|q| q.text != b.text && (q.cy() - b.cy()).abs() < 0.5 * max))
    };
    let mut skipped: Vec<f32> = Vec::new();
    let mut prev_row: Option<f32> = None;
    let mut level_row: Option<f32> = None;
    let mut related = false;
    for (c, rel, op, goes_on, solid, leading_op) in rows {
        if std::env::var_os("PDF2MD_DISPDEBUG").is_some() {
            eprintln!("ROW {c} rel {rel} op {op} matrix {} inbr {} frac {}", matrix_row(c), in_bracket(c), fraction_row(c));
        }
        if matrix_row(c) {
            skipped.push(c);
            continue;
        }
        let tagged = tags.iter().any(|t| (t.cy() - c).abs() <= 0.5 * max);
        // With no bars at all, a row well apart from the one above is a line
        // of its own ("φ₁ to φ₁ + δφ on σ₁" / "φ₂ to …"), and so is that one.
        // (rows of a matrix stand in one bracket; a bracket one line high
        // around part of the row above holds no row of this one)
        let apart = bars.is_empty()
            && prev_row.is_some_and(|p| c - p >= 1.4 * max)
            && !prev_row.is_some_and(|p| (in_bracket(c) || in_bracket(p)) && (same_bracket(p, c) || !one_line(p) || !one_line(c)));
        if apart && anchors.is_empty() {
            anchors.push((prev_row.unwrap_or(c), false));
        }
        // "= …" under a line that has no relation sign: its right-hand side
        let rhs = apart && goes_on && !related && anchors.len() == 1 && tags.is_empty();
        // A full row well under the rows of a fraction, reaching out past
        // every bar: the fraction's line had no relation sign to tell it,
        // and this row is the display's second line, not a denominator.
        let (l, r) = rest
            .iter()
            .filter(|s| !s.cmex() && s.size >= 0.9 * max && (s.cy() - c).abs() <= 0.3 * max)
            .fold((f32::MAX, f32::MIN), |(l, r), s| (l.min(s.left), r.max(s.right)));
        let under_bar = bars.iter().any(|b| l >= b.left - max && r <= b.right + max);
        let second_line = anchors.is_empty()
            && !bars.is_empty()
            && !fraction_row(c)
            && !in_bracket(c)
            && !under_bar
            && prev_row.is_some_and(|p| c - p >= 1.4 * max && fraction_row(p));
        if second_line {
            let above: Vec<f32> = bars.iter().map(|b| (b.top + b.bottom) / 2.0).filter(|y| *y < c).collect();
            anchors.push((above.iter().sum::<f32>() / above.len().max(1) as f32, false));
        }
        prev_row = Some(c);
        // Two rows each at the height of fraction bars of its own, well
        // apart and with no bar between them (they are no numerator and
        // denominator of one fraction): two lines, though neither may have
        // a relation sign to tell it. One that begins with a sign goes on
        // from the line above.
        // (… the row at the middle of every bracket around it: no row of
        // a column)
        let centred = |c: f32| !rest.iter().any(|s| s.cmex() && s.text.trim().is_empty()) && tall.iter().filter(|b| c > b.top - 0.5 * max && c < b.bottom + max).all(|b| (c - b.cy()).abs() < 0.5 * max);
        // (… a bar of the line, not one of a fraction in a numerator)
        let mid = |b: &Rule| (b.top + b.bottom) / 2.0;
        let nested = |b: &Rule| bars.iter().any(|w| w.left <= b.left + 1.0 && w.right >= b.right - 1.0 && (mid(w) - mid(b)).abs() > 0.3 * max && (mid(w) - mid(b)).abs() <= 1.6 * max);
        let level = solid && bars.iter().any(|b| (c - mid(b)).abs() <= 0.3 * max && !nested(b)) && (!in_bracket(c) || centred(c));
        let two_lines = level
            && level_row.is_some_and(|p| c - p >= 2.2 * max && !bars.iter().any(|b| (b.top + b.bottom) / 2.0 > p + 0.5 * max && (b.top + b.bottom) / 2.0 < c - 0.5 * max));
        let opened = two_lines && anchors.is_empty();
        if opened {
            anchors.push((level_row.unwrap_or(c), false));
        }
        if level {
            level_row = Some(c);
        }
        let far = anchors.last().is_none_or(|&(a, _)| c - a >= 1.1 * max);
        let own_line = (!bars.is_empty() || apart) && !fraction_row(c) && !in_bracket(c);
        if two_lines && far && (opened || !op) && (leading_op || !rel) {
            anchors.push((c, leading_op));
        } else if second_line || (rhs && far) {
            anchors.push((c, true));
        } else if far && (rel || tagged || own_line) {
            related |= rel;
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
    if std::env::var_os("PDF2MD_DISPDEBUG").is_some() {
        eprintln!("DISPLAY max {max} bars {} anchors {anchors:?} skipped {skipped:?} cmex {:?}", bars.len(), rest.iter().filter(|s| s.cmex()).map(|s| format!("{:?}@{:.0},{:.0}-{:.0}", s.text, s.left, s.top, s.bottom)).collect::<Vec<_>>());
    }
    if anchors.len() <= 1 {
        let tag = tags.first().and_then(|t| tag_of(&t.text));
        let area = bounds(&rest, bars);
        return vec![(build_with(&rest, bars), tag, area)];
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
        // (a root sign's box sits high: it is of the line under its bar,
        // the highest bar that starts at it within its height, not the bar
        // of a fraction under or over the root; a tall root, read from its
        // outline, has its true middle)
        let own = if is_radical(s) && s.bottom - s.top <= 1.5 * max {
            // (… as far under the bar as half the sign's height: a tall
            // sign's middle is the line of the fraction under it)
            bars.iter().filter(|b| (b.left - s.right).abs() <= 3.0 && b.top >= s.top - 2.0 && s.bottom >= b.top - 2.0).min_by(|a, b| a.top.total_cmp(&b.top)).map_or(s.cy(), |b| b.bottom + sign_middle(s, max))
        } else if let Some(depth) = op_depth(s) {
            // (a big operator's box is only its top: its true middle)
            op_top(s) + 0.5 * depth * s.size
        } else {
            s.cy()
        };
        // (the last row of a column of small fractions has its middle at
        // the bracket's foot: a point or two of play; and the exponents of
        // its first row's numerator stand out over the bracket's head)
        // (… an exponent: it reaches into the bracket, behind a larger
        // letter there that begins lower, and is no index of a letter of
        // the line above)
        let exponent = |b: &MSpan| {
            !s.cmex()
                && s.size < 0.85 * max
                && s.bottom >= b.top + 1.0
                && rest.iter().any(|n| !n.cmex() && n.cy() > b.top && (n.right - s.left).abs() <= 1.5 && n.top >= s.top && n.top <= s.bottom && s.size < 0.85 * n.size)
                && !rest.iter().any(|m| !m.cmex() && (m.right - s.left).abs() <= 1.5 && m.top < s.top - 1.0 && m.bottom > s.top && m.size > 1.1 * s.size)
        };
        tall.iter()
            .filter(|b| (s.cy() > b.top || (s.cy() > b.top - 0.4 * max && exponent(b))) && s.cy() < b.bottom + 2.0 && b.right <= s.left + 1.0)
            .max_by(|a, b| a.right.total_cmp(&b.right))
            .filter(|b| {
                // ... and a partner bracket to its right at the same height
                tall.iter().any(|q| q.left >= s.right - 1.0 && (q.cy() - b.cy()).abs() < 0.5 * max)
            })
            .map_or(own, |b| b.cy())
    };
    // (a fraction's bar between a pair of tall brackets is of their line too)
    let bar_home = |r: &Rule| -> f32 {
        let y = (r.top + r.bottom) / 2.0;
        // (the bar of a root is of the line of its sign, which hangs
        // under it: over a fraction it stands nearer the line above)
        let sign = rest.iter().find(|s| is_radical(s) && s.bottom - s.top <= 1.5 * max && (r.left - s.right).abs() <= 3.0 && r.top >= s.top - 2.0 && s.bottom >= r.top - 2.0);
        let y = sign.map_or(y, |s| r.bottom + sign_middle(s, max));
        tall.iter()
            .filter(|b| y > b.top && y < b.bottom && b.right <= r.left + 1.0)
            .max_by(|a, b| a.right.total_cmp(&b.right))
            .filter(|b| tall.iter().any(|q| q.left >= r.right - 1.0 && (q.cy() - b.cy()).abs() < 0.5 * max))
            .map_or(y, |b| b.cy())
    };
    let mut homes: Vec<f32> = rest.iter().map(home).collect();
    let bar_homes: Vec<f32> = bars.iter().map(bar_home).collect();
    // A small piece between two lines is of the line of what it hangs on,
    // not of the line whose middle happens to be nearer: the limits of an
    // integral or a sum go with their sign (the lower one reaches far
    // down towards the next line), an exponent or an index with the
    // letter right before it (the square of a numerator's letter stands
    // halfway up to the line above).
    let mut order: Vec<usize> = (0..rest.len()).filter(|&i| !rest[i].cmex() && rest[i].size < 0.85 * max).collect();
    order.sort_by(|&a, &b| rest[a].left.total_cmp(&rest[b].left));
    for &i in &order {
        let s = &rest[i];
        // (what stands between a pair of tall brackets is of their line)
        if (homes[i] - s.cy()).abs() > 0.01 {
            continue;
        }
        // (the half of a small fraction: with its bar)
        // (a short bar with small pieces right over and right under it: not
        // the bar of a root, nor a line's fraction with an index near it)
        let touches = |t: &MSpan, b: &Rule| !t.cmex() && t.size < 0.85 * max && t.left >= b.left - 1.5 && t.right <= b.right + 1.5 && ((t.bottom - b.top).abs() <= 2.5 || (t.top - b.bottom).abs() <= 2.5);
        if let Some(k) = (0..bars.len()).find(|&k| {
            let b = &bars[k];
            b.right - b.left <= 2.0 * max && touches(s, b) && rest.iter().any(|t| touches(t, b) && (t.cy() < b.top) != (s.cy() < b.top))
        }) {
            homes[i] = bar_homes[k];
            continue;
        }
        // (what goes on at one height from a piece of its own size, or
        // stands raised or lowered behind a larger one)
        let hangs_on = |n: &MSpan| !n.cmex() && (n.right - s.left).abs() <= 1.5 && if n.size >= 1.15 * s.size { n.bottom.min(s.bottom) - n.top.max(s.top) >= 0.3 * s.height() } else { n.size >= 0.95 * s.size && (n.cy() - s.cy()).abs() <= 0.45 * s.size };
        // (… or with the tall bracket it stands at the head or foot of)
        let closes = |n: &MSpan| {
            let at = |y: f32, above: f32, below: f32| s.cy() >= y - above * max && s.cy() <= y + below * max;
            n.cmex() && matches!(n.text.trim(), ")" | "]" | "}" | "⟩" | "|") && n.height() > 1.5 * max && s.left >= n.right - 1.0 && s.left - n.right <= 0.3 * max && (at(n.top, 0.4, 0.5) || at(n.bottom, 0.5, 0.3))
        };
        let lines: Vec<usize> = (0..rest.len()).filter(|&k| k != i && (hangs_on(&rest[k]) || closes(&rest[k]))).collect();
        if let Some(&k) = lines.first() {
            if lines.iter().all(|&j| nearest(homes[j]) == nearest(homes[k])) {
                homes[i] = homes[k];
            }
            continue;
        }
        // (a limit: where the lines stand close, the lower limit of one
        // line's sign is at the place of an upper limit of the next
        // line's, and only its distance tells)
        let signs: Vec<usize> = (0..rest.len()).filter(|&k| limit_of(s, &rest[k])).collect();
        if let Some(&k) = signs.first() {
            if signs.iter().all(|&j| nearest(homes[j]) == nearest(homes[k])) {
                homes[i] = homes[k];
            }
        }
    }
    for (s, y) in rest.into_iter().zip(homes) {
        if std::env::var("PDF2MD_DISPDEBUG").is_ok_and(|v| v == "2") {
            eprintln!("  G{} {:7.1} {:7.1} {:7.1} {:7.1} {:4.1} {:8} {:?} home {y:.1}", nearest(y), s.left, s.right, s.top, s.bottom, s.size, s.family, s.text);
        }
        groups[nearest(y)].push(s);
    }
    for t in &tags {
        gtags[nearest(t.cy())] = tag_of(&t.text);
    }
    for (r, y) in bars.iter().zip(bar_homes) {
        gbars[nearest(y)].push(*r);
    }

    // Equations: a row plus the continuation rows below it.
    let mut eqs: Vec<(Vec<String>, Option<String>, Area)> = Vec::new();
    for k in 0..anchors.len() {
        MULTILINE.with(|m| m.set(true));
        let t = build_with(&groups[k], &gbars[k]);
        MULTILINE.with(|m| m.set(false));
        let area = bounds(&groups[k], &gbars[k]);
        match eqs.last_mut() {
            Some(e) if anchors[k].1 => {
                e.0.push(t);
                if e.1.is_none() {
                    e.1 = gtags[k].clone();
                }
                e.2 = both(e.2, area);
            }
            _ => eqs.push((vec![t], gtags[k].clone(), area)),
        }
    }
    // One equation number for several equations: a system, one block.
    if tags.len() == 1 && eqs.len() > 1 {
        let area = eqs.iter().map(|e| e.2).reduce(both).unwrap_or([0.0; 4]);
        let rows: Vec<String> = eqs.into_iter().flat_map(|e| e.0).collect();
        let tag = tags.first().and_then(|t| tag_of(&t.text));
        return vec![(aligned(&rows), tag, area)];
    }
    eqs.into_iter()
        .filter(|(r, t, _)| r.iter().any(|x| !x.is_empty()) || t.is_some())
        .map(|(r, t, a)| if r.len() == 1 { (r[0].clone(), t, a) } else { (aligned(&r), t, a) })
        .collect()
}

/// Displays in two sizes, one under the other with nothing between them
/// (a derivation's steps set smaller than its result): where to cut them
/// apart. "The size of the line" is one size for a whole display, and the
/// letters of the smaller one would all read as indices of the larger.
///
/// The smaller size is that of small text (about five sixths), never that
/// of an index (two thirds); its lines hold a relation sign of their own,
/// and white space parts the two.
fn size_zones(spans: &[MSpan], bars: &[Rule]) -> Option<Vec<f32>> {
    let mark = |s: &MSpan| s.text.trim().chars().all(|c| ",.;:".contains(c));
    // bands: (top, bottom, size of its letters, has a relation sign in that size)
    let mut boxes: Vec<(f32, f32, Option<&MSpan>)> = spans.iter().map(|s| (s.top, s.bottom, Some(s))).collect();
    boxes.extend(bars.iter().map(|b| (b.top, b.bottom, None)));
    boxes.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut bands: Vec<(f32, f32, Vec<&MSpan>)> = Vec::new();
    for (top, bottom, s) in boxes {
        match bands.last_mut() {
            Some(b) if top <= b.1 + 2.0 => {
                b.1 = b.1.max(bottom);
                b.2.extend(s);
            }
            _ => bands.push((top, bottom, s.into_iter().collect())),
        }
    }
    let size = |m: &[&MSpan]| m.iter().filter(|s| !s.cmex() && !mark(s)).map(|s| s.size).fold(0.0f32, f32::max);
    let related = |m: &[&MSpan], size: f32| m.iter().any(|s| !s.cmex() && s.size >= 0.9 * size && s.text.split_whitespace().any(is_relation));
    // runs of bands in one size: (members, top, bottom)
    let mut runs: Vec<(Vec<&MSpan>, f32, f32)> = Vec::new();
    for (top, bottom, members) in bands {
        match runs.last_mut() {
            Some(run) => {
                let (a, b) = (size(&run.0), size(&members));
                let (small, large) = (a.min(b), a.max(b));
                if small > 0.76 * large && small < 0.9 * large && top - run.2 >= 0.5 * small {
                    runs.push((members, top, bottom));
                } else {
                    run.0.extend(members);
                    run.2 = bottom;
                }
            }
            None => runs.push((members, top, bottom)),
        }
    }
    let mut cuts = Vec::new();
    for w in runs.windows(2) {
        let (a, b) = (size(&w[0].0), size(&w[1].0));
        if related(if a < b { &w[0].0 } else { &w[1].0 }, a.min(b)) {
            cuts.push((w[0].2 + w[1].1) / 2.0);
        }
    }
    (!cuts.is_empty()).then_some(cuts)
}

/// Rows of one `aligned` block, lined up at the first "=" (rows without
/// one, like continuation lines, are indented under the right-hand side).
/// The first " = " of a row that stands in no group: the rows line up
/// there (not at the "=" under a sum sign, "\\sum_{n = 1}").
fn top_level_equals(t: &str) -> Option<usize> {
    // (groups, and brackets of any kind: one may open what another closes,
    // "( a , b \\}")
    // (nor in a matrix or a system of its own, "\\begin{Bmatrix} x & = & …":
    // its "=" is a column of that block)
    let (mut group, mut bracket, mut block) = (0i32, 0i32, 0i32);
    let mut escaped = false;
    for (i, c) in t.char_indices() {
        let was = escaped;
        escaped = c == '\\' && !was;
        if c == '\\' && !was {
            if t[i..].starts_with("\\begin{") {
                block += 1;
            } else if t[i..].starts_with("\\end{") {
                block = (block - 1).max(0);
            }
        }
        match c {
            '{' if was => bracket += 1,
            '}' if was => bracket = (bracket - 1).max(0),
            '{' => group += 1,
            '}' => group -= 1,
            '(' | '[' => bracket += 1,
            ')' | ']' => bracket = (bracket - 1).max(0),
            ' ' if group == 0 && bracket == 0 && block == 0 && t[i..].starts_with(" = ") => return Some(i),
            _ => {}
        }
    }
    None
}

fn aligned(rows: &[String]) -> String {
    // (one row: nothing to line up)
    if let [one] = rows.iter().filter(|t| !t.is_empty()).collect::<Vec<_>>().as_slice() {
        return one.to_string();
    }
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
    fn the_equals_sign_of_a_block_is_not_the_rows() {
        // a matrix's own "=" is a column of the matrix
        assert_eq!(super::top_level_equals("\\begin{Bmatrix} x & = & a \\\\ y & = & b \\end{Bmatrix} ."), None);
        assert_eq!(super::top_level_equals("\\begin{pmatrix} a \\\\ b \\end{pmatrix} = 0"), Some(36));
        // one row: nothing to line up
        assert_eq!(super::aligned(&["".to_string(), "x = 1".to_string()]), "x = 1");
        assert_eq!(super::aligned(&["x = 1".to_string(), "= 2".to_string()]), "\\begin{aligned} x &= 1 \\\\ &= 2 \\end{aligned}");
    }

    #[test]
    fn an_index_with_a_bar_under_an_exponent_is_a_small_fraction() {
        assert_eq!(barred_index("2 \\sinh_{\\overline{2}}^{\\chi} \\cosh x"), "2 \\sinh \\tfrac{\\chi}{2} \\cosh x");
        assert_eq!(barred_index("a_{\\overline{z}} + b_{\\overline{x_{1}}}^{y^{2}}"), "a_{\\overline{z}} + b \\tfrac{y^{2}}{x_{1}}");
    }

    #[test]
    fn a_root_sign_with_a_small_fraction_behind_it() {
        assert_eq!(rooted_fraction("i \\surd_{3}^{1} Y_{1 - 1}"), "i \\sqrt{\\tfrac{1}{3}} Y_{1 - 1}");
        assert_eq!(rooted_fraction("\\surd_{a_{0}}^{2 x} + \\surd x"), "\\sqrt{\\tfrac{2 x}{a_{0}}} + \\surd x");
        assert_eq!(rooted_fraction("\\surd_{3} y"), "\\surd_{3} y");
    }

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
    fn a_root_ends_at_its_sign() {
        // { s = √(x² + y²), ϕ = arctan(y/x), z = z } .  The bracket of the
        // second row stands under the root's bar; the root is one line
        // high, as its sign is (Lucid, Advanced Theoretical Physics, 1.2.2)
        let v = vec![
            bx("s", 260.0, 268.0, 868.0, 884.0, 18.0, "CMMI12"),
            bx("=", 285.0, 299.0, 868.0, 884.0, 18.0, "CMR12"),
            bx("p", 314.0, 332.0, 865.0, 873.0, 18.0, "CMEX10"),
            bx("x", 332.0, 342.0, 868.0, 884.0, 18.0, "CMMI12"),
            bx("2", 342.0, 348.0, 867.0, 878.0, 12.0, "CMR8"),
            bx("+", 353.0, 367.0, 868.0, 884.0, 18.0, "CMR12"),
            bx("y", 371.0, 380.0, 868.0, 884.0, 18.0, "CMMI12"),
            bx("2", 380.0, 386.0, 867.0, 878.0, 12.0, "CMR8"),
            bx("{", 247.1, 256.9, 865.1, 929.6, 18.0, "CMEX10"),
            bx("ϕ", 260.0, 270.0, 890.0, 906.0, 18.0, "CMMI12"),
            bx("= arctan", 285.0, 362.0, 890.0, 906.0, 18.0, "CMR12"),
            bx("(", 368.4, 373.1, 887.4, 908.9, 18.0, "CMEX10"),
            bx("y", 376.0, 382.0, 886.0, 897.0, 12.0, "CMMI8"),
            bx("x", 376.0, 383.0, 901.0, 912.0, 12.0, "CMMI8"),
            bx(")", 385.4, 390.1, 887.4, 908.9, 18.0, "CMEX10"),
            bx("}", 395.9, 405.7, 865.1, 929.6, 18.0, "CMEX10"),
            bx(".", 412.0, 417.0, 889.0, 905.0, 18.0, "CMMI12"),
            bx("z", 260.0, 268.0, 912.0, 928.0, 18.0, "CMMI12"),
            bx("=", 285.0, 299.0, 912.0, 928.0, 18.0, "CMR12"),
            bx("z", 314.0, 322.0, 912.0, 928.0, 18.0, "CMMI12"),
        ];
        let bars = [Rule { left: 331.8, right: 386.8, top: 864.6, bottom: 865.3 }, Rule { left: 375.7, right: 382.8, top: 897.8, bottom: 898.5 }];
        assert_eq!(
            build_with(&v, &bars),
            "\\left\\{ \\begin{aligned} s &= \\sqrt{x^{2} + y^{2}} \\\\ \\phi &= \\arctan \\left( \\tfrac{y}{x} \\right) \\\\ z &= z \\end{aligned} \\right\\} ."
        );
    }

    #[test]
    fn displays_in_two_sizes_are_two() {
        // a line in the size of the text, and under it a step set small:
        // its letters are no indices of the line above
        let v = vec![
            bx("a", 100.0, 110.0, 100.0, 116.0, 18.0, "CMMI12"),
            bx("=", 116.0, 130.0, 100.0, 116.0, 18.0, "CMR12"),
            bx("b", 136.0, 146.0, 100.0, 116.0, 18.0, "CMMI12"),
            bx("=", 116.0, 128.0, 140.0, 153.0, 15.0, "CMR10"),
            bx("c", 134.0, 142.0, 140.0, 153.0, 15.0, "CMMI10"),
        ];
        let got: Vec<String> = display(v, &[], 600.0, 18.0).into_iter().map(|e| e.0).collect();
        assert_eq!(got, ["a = b", "= c"]);
    }

    #[test]
    fn small_type_on_the_line_is_of_the_line() {
        // "L =" in the size of the text, the right-hand side set small to
        // fit: level with the "=", it is no index of it
        let v = vec![
            bx("L", 156.0, 168.0, 269.0, 285.0, 18.0, "CMMI12"),
            bx("=", 195.0, 209.0, 269.0, 285.0, 18.0, "CMR12"),
            bx("x", 245.0, 254.0, 271.0, 284.0, 15.0, "CMMI10"),
            bx("+", 258.0, 270.0, 271.0, 284.0, 15.0, "CMR10"),
            bx("y", 274.0, 283.0, 271.0, 284.0, 15.0, "CMMI10"),
            bx("2", 283.0, 289.0, 269.0, 278.0, 10.0, "CMR7"),
        ];
        assert_eq!(build_with(&v, &[]), "L = x + y^{2}");
    }

    #[test]
    fn column_of_small_fractions() {
        // N (−p_z c/(−E+mc²), −(p_x+ip_y)c/(−E+mc²), 1, 0): each bar keeps
        // to its own row of the column (UCSD Physics 130, the Dirac spinors)
        let v = vec![
            bx("N", 399.0, 411.0, 741.0, 756.0, 15.0, "CMMI10"),
            bx("(", 419.6, 427.8, 712.8, 784.5, 15.0, "CMEX10"),
            bx("−", 451.0, 460.0, 708.0, 720.0, 10.0, "CMSY7"),
            bx("p", 460.0, 466.0, 708.0, 718.0, 10.0, "CMMI7"),
            bx("z", 466.0, 471.0, 712.0, 719.0, 7.0, "CMMI5"),
            bx("c", 472.0, 477.0, 708.0, 718.0, 10.0, "CMMI7"),
            bx("−", 439.0, 448.0, 720.0, 732.0, 10.0, "CMSY7"),
            bx("E", 449.0, 458.0, 720.0, 730.0, 10.0, "CMMI7"),
            bx("+", 458.0, 467.0, 720.0, 730.0, 10.0, "CMR7"),
            bx("mc", 467.0, 483.0, 720.0, 730.0, 10.0, "CMMI7"),
            bx("2", 483.0, 488.0, 719.0, 726.0, 7.0, "CMR5"),
            bx("−", 433.0, 442.0, 730.0, 742.0, 10.0, "CMSY7"),
            bx("(", 442.0, 447.0, 731.0, 741.0, 10.0, "CMR7"),
            bx("p", 447.0, 453.0, 731.0, 741.0, 10.0, "CMMI7"),
            bx("x", 453.0, 459.0, 734.0, 741.0, 7.0, "CMMI5"),
            bx("+", 459.0, 468.0, 731.0, 741.0, 10.0, "CMR7"),
            bx("ip", 469.0, 479.0, 731.0, 741.0, 10.0, "CMMI7"),
            bx("y", 479.0, 484.0, 734.0, 741.0, 7.0, "CMMI5"),
            bx(")", 485.0, 490.0, 731.0, 741.0, 10.0, "CMR7"),
            bx("c", 490.0, 495.0, 731.0, 741.0, 10.0, "CMMI7"),
            bx("−", 439.0, 448.0, 743.0, 755.0, 10.0, "CMSY7"),
            bx("E", 449.0, 458.0, 743.0, 753.0, 10.0, "CMMI7"),
            bx("+", 458.0, 467.0, 743.0, 753.0, 10.0, "CMR7"),
            bx("mc", 467.0, 483.0, 743.0, 753.0, 10.0, "CMMI7"),
            bx("2", 483.0, 488.0, 743.0, 750.0, 7.0, "CMR5"),
            bx("1", 460.0, 467.0, 753.0, 768.0, 15.0, "CMR10"),
            bx("0", 460.0, 467.0, 771.0, 786.0, 15.0, "CMR10"),
            bx(")", 500.1, 508.4, 712.8, 784.5, 15.0, "CMEX10"),
        ];
        let bars = [Rule { left: 439.2, right: 488.9, top: 718.6, bottom: 719.3 }, Rule { left: 432.5, right: 495.5, top: 742.0, bottom: 742.7 }];
        assert_eq!(
            build_with(&v, &bars),
            "N \\begin{pmatrix} \\frac{- p_{z} c}{- E + mc^{2}} \\\\ \\frac{- ( p_{x} + ip_{y} ) c}{- E + mc^{2}} \\\\ 1 \\\\ 0 \\end{pmatrix}"
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
