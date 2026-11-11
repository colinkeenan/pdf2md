//! How wide a formula comes out in the viewer, estimated from its LaTeX
//! (KaTeX sets math at 1.21 em of the text): to tell where words set
//! under an equation stand in it.

/// The type size, in CSS pixels.
const SIZE: f32 = 16.0;
/// KaTeX sets math this much larger than the text around it.
const MATH: f32 = 1.21 * SIZE;
// Advance widths of the characters ' ' to '~' in Noto Sans, in 1/1000 em.
const REGULAR: [u16; 95] = [
    260, 269, 408, 646, 572, 831, 732, 225, 300, 300, 551, 572, 268, 322, 268, 372,
    572, 572, 572, 572, 572, 572, 572, 572, 572, 572, 268, 268, 572, 572, 572, 434,
    899, 639, 650, 632, 730, 556, 519, 728, 741, 339, 273, 619, 524, 907, 760, 781,
    605, 781, 622, 549, 556, 731, 600, 930, 586, 566, 572, 329, 372, 329, 572, 444,
    281, 561, 615, 480, 615, 564, 344, 615, 618, 258, 258, 534, 258, 935, 618, 605,
    615, 615, 413, 479, 361, 618, 508, 786, 529, 510, 470, 380, 551, 380, 572,
];
// Widths of a–z and A–Z in KaTeX's math italic, in 1/1000 em.
const MATH_SMALL: [u16; 26] = [
    529, 429, 433, 520, 466, 597, 513, 576, 345, 469, 552, 318, 878,
    600, 485, 503, 482, 479, 469, 361, 572, 521, 743, 572, 526, 509,
];
const MATH_CAPITAL: [u16; 26] = [
    750, 809, 786, 856, 796, 782, 786, 913, 518, 651, 921, 681, 1079,
    912, 791, 781, 791, 767, 671, 723, 792, 806, 1083, 907, 803, 754,
];

/// The width of some text in pixels.
pub fn text(s: &str) -> f32 {
    let table = &REGULAR;
    let units: u32 = s
        .chars()
        .map(|c| match c {
            ' '..='~' => table[c as usize - 32] as u32,
            '‘' | '’' => 175,
            '“' | '”' => 359,
            '–' => 500,
            '—' => 1000,
            '…' => 791,
            '′' => 232,
            '″' => 408,
            '·' => 268,
            'Α'..='Ω' => 700,
            'α'..='ω' => 600,
            // combining accents sit on the letter before them
            '\u{300}'..='\u{36f}' => 0,
            // other letters and signs: about the width of a digit
            _ => 572,
        })
        .sum();
    units as f32 * SIZE / 1000.0
}

/// The widths in pixels of an inline formula, estimated from its LaTeX: of
/// the parts a line may be broken between (KaTeX lets a line end after a
/// relation or an operation sign that is not inside braces or brackets
/// sized by `\left`).
pub fn math(latex: &str) -> Vec<f32> {
    let chars: Vec<char> = latex.chars().collect();
    let mut at = 0;
    run(&chars, &mut at, false, false).iter().map(|w| w * MATH).collect()
}

/// The width of a formula in em of the formula's own type size.
pub fn math_em(latex: &str) -> f32 {
    math(latex).iter().sum::<f32>() / MATH
}

/// What a sign is to the spacing around it.
#[derive(Clone, Copy, PartialEq)]
enum Kind {
    /// a letter, a number, a closing bracket
    Ord,
    /// "+", "×": a thin space on both sides, if it stands between two things
    Bin,
    /// "=", "<": a wider space on both sides
    Rel,
    Open,
    Punct,
}

/// The widths in em of a run of LaTeX, part by part: up to the closing
/// brace of the group it is in (`group`), or to the end. `tight`: raised
/// or lowered, where signs get no space around them.
fn run(c: &[char], at: &mut usize, group: bool, tight: bool) -> Vec<f32> {
    let mut parts = vec![0.0];
    let mut prev = None;
    // how many `\left` are open
    let mut sized = 0;
    while *at < c.len() {
        if c[*at] == '}' {
            if group {
                *at += 1;
            }
            break;
        }
        let rest: String = c[*at..].iter().take(7).collect();
        let is = |name: &str| rest.strip_prefix(name).is_some_and(|r| !r.starts_with(|l: char| l.is_ascii_alphabetic()));
        if is("\\left") {
            sized += 1;
        } else if is("\\right") && sized > 0 {
            sized -= 1;
        }
        let (mut w, kind) = atom(c, at, prev, tight);
        // what is raised and what is lowered stand above each other
        let mut scripts: f32 = 0.0;
        while *at < c.len() && (c[*at] == '^' || c[*at] == '_') {
            *at += 1;
            scripts = scripts.max(0.7 * atom(c, at, None, true).0 + 0.05);
        }
        w += scripts;
        if let Some(last) = parts.last_mut() {
            *last += w;
        }
        if matches!(kind, Kind::Bin | Kind::Rel) && !group && !tight && sized == 0 {
            parts.push(0.0);
        }
        prev = Some(kind);
    }
    if parts.len() > 1 && parts.last() == Some(&0.0) {
        parts.pop();
    }
    // a sign at the very end has nothing to keep apart from on its right
    if !tight {
        if let Some(last) = parts.last_mut() {
            match prev {
                Some(Kind::Bin) => *last -= 2.0 * 0.222,
                Some(Kind::Rel) => *last -= 0.278,
                _ => {}
            }
        }
    }
    parts
}

/// The width in em of one thing: a group in braces, a command with what it
/// takes, or a single sign. `prev`: what stands in front of it.
fn atom(c: &[char], at: &mut usize, prev: Option<Kind>, tight: bool) -> (f32, Kind) {
    while *at < c.len() && c[*at].is_whitespace() {
        *at += 1;
    }
    let Some(&ch) = c.get(*at) else { return (0.0, Kind::Ord) };
    *at += 1;
    // a sign with nothing in front of it to add to or compare with: "−x"
    let lone = !matches!(prev, Some(Kind::Ord));
    let bin = |w: f32| if lone { (w, Kind::Ord) } else { (w + if tight { 0.0 } else { 2.0 * 0.222 }, Kind::Bin) };
    let rel = |w: f32| (w + if tight { 0.0 } else if prev.is_none() { 0.278 } else { 2.0 * 0.278 }, Kind::Rel);
    let ord = |w: f32| (w, Kind::Ord);
    match ch {
        '{' => ord(run(c, at, true, tight).iter().sum()),
        '\\' => {
            let start = *at;
            while *at < c.len() && c[*at].is_ascii_alphabetic() {
                *at += 1;
            }
            if *at == start {
                // a sign after the backslash: "\,", "\{", "\\"
                let sign = c.get(*at).copied().unwrap_or(' ');
                *at += 1;
                return match sign {
                    ',' => ord(0.167),
                    ':' | '>' => ord(0.222),
                    ';' => ord(0.278),
                    '!' => ord(-0.167),
                    ' ' => ord(0.25),
                    '{' => (0.5, Kind::Open),
                    '\\' => ord(0.0),
                    _ => ord(0.5),
                };
            }
            let name: String = c[start..*at].iter().collect();
            match name.as_str() {
                "pm" | "mp" | "times" | "otimes" | "oplus" | "ast" | "star" | "circ" | "wedge" | "cap" | "cup" => bin(0.778),
                "cdot" => bin(0.278),
                "to" | "rightarrow" | "leftarrow" | "leftrightarrow" | "mapsto" | "Rightarrow" | "Leftrightarrow" => rel(1.0),
                "longrightarrow" | "Longrightarrow" => rel(1.6),
                "leq" | "le" | "geq" | "ge" | "neq" | "ne" | "approx" | "equiv" | "sim" | "simeq" | "cong" | "propto"
                | "in" | "subset" | "perp" => rel(0.778),
                "ll" | "gg" => rel(1.0),
                "mid" | "parallel" => rel(0.4),
                "langle" | "lbrack" | "lbrace" => (0.42, Kind::Open),
                // bold: a little wider
                "boldsymbol" | "bm" | "mathbf" => {
                    let (w, kind) = atom(c, at, prev, tight);
                    (w * 1.15, kind)
                }
                _ => (command(&name, c, at, tight), prev.filter(|_| styles(&name)).unwrap_or(Kind::Ord)),
            }
        }
        'a'..='z' => ord(MATH_SMALL[ch as usize - 'a' as usize] as f32 / 1000.0),
        'A'..='Z' => ord(MATH_CAPITAL[ch as usize - 'A' as usize] as f32 / 1000.0),
        '0'..='9' | '/' => ord(0.5),
        '=' | '<' | '>' => rel(0.778),
        '+' | '-' | '*' => bin(if ch == '*' { 0.5 } else { 0.778 }),
        '(' => (0.389, Kind::Open),
        '[' => (0.278, Kind::Open),
        ')' => ord(0.389),
        ']' | '|' | '.' | '!' | '\'' => ord(0.278),
        ':' => rel(0.278),
        ',' | ';' => (0.278 + if tight { 0.0 } else { 0.167 }, Kind::Punct),
        '&' | '~' | '^' | '_' => ord(0.0),
        _ => ord(0.6),
    }
}

/// Commands that change how the next thing looks, not (much) how wide it
/// is; a sign after one is spaced as if the command were not there.
fn styles(name: &str) -> bool {
    matches!(
        name,
        "left" | "right" | "bigl" | "bigr" | "Bigl" | "Bigr" | "big" | "Big" | "displaystyle" | "textstyle" | "limits"
            | "nolimits" | "mathrm" | "mathit" | "mathcal" | "mathbb" | "mathsf" | "hat" | "widehat" | "tilde" | "widetilde" | "bar" | "vec" | "dot" | "ddot" | "overline" | "underline"
            | "operatorname" | "mathop"
    )
}

/// The width in em of a command with what it takes.
fn command(name: &str, c: &[char], at: &mut usize, tight: bool) -> f32 {
    match name {
        _ if styles(name) => 0.0,
        "text" | "textrm" | "textit" | "textbf" | "mbox" => {
            while *at < c.len() && c[*at].is_whitespace() {
                *at += 1;
            }
            if c.get(*at) != Some(&'{') {
                return 0.0;
            }
            let start = *at + 1;
            let mut depth = 0;
            while *at < c.len() {
                match c[*at] {
                    '{' => depth += 1,
                    '}' => depth -= 1,
                    _ => {}
                }
                *at += 1;
                if depth == 0 {
                    break;
                }
            }
            let words: String = c[start..(*at).saturating_sub(1).max(start)].iter().collect();
            // set in the math font's roman, a little narrower than the text font
            0.9 * text(&words) / SIZE
        }
        // \begin{array}{cc}: the names take no room
        "begin" | "end" => {
            let array = c[*at..].iter().collect::<String>().trim_start().starts_with("{array}");
            for _ in 0..if array && name == "begin" { 2 } else { 1 } {
                while *at < c.len() && c[*at].is_whitespace() {
                    *at += 1;
                }
                if c.get(*at) == Some(&'{') {
                    while *at < c.len() && c[*at] != '}' {
                        *at += 1;
                    }
                    *at += 1;
                }
            }
            // (and the columns stand a little apart from the brackets)
            if array && name == "begin" {
                1.2
            } else {
                0.0
            }
        }
        "frac" | "tfrac" | "dfrac" | "binom" => {
            let (above, below) = (atom(c, at, None, true).0, atom(c, at, None, true).0);
            0.7 * above.max(below) + 0.24
        }
        "sqrt" => 0.85 + atom(c, at, None, tight).0,
        "quad" => 1.0,
        "qquad" => 2.0,
        "ldots" | "dots" | "cdots" => 1.17 + 2.0 * 0.167,
        "sum" | "prod" => 1.06 + 0.167,
        "int" | "oint" => 0.47 + 0.167,
        // sin, cos, exp, ...: upright letters
        "sin" | "cos" | "tan" | "cot" | "sec" | "csc" | "exp" | "log" | "ln" | "det" | "dim" | "lim" | "max" | "min"
        | "sinh" | "cosh" | "tanh" | "arg" | "tr" | "Tr" => 0.47 * name.len() as f32 + 0.167,
        "vert" | "lvert" | "rvert" | "rbrack" => 0.278,
        "prime" => 0.275,
        "iota" => 0.354,
        "rangle" => 0.389,
        "epsilon" => 0.406,
        "ell" => 0.417,
        "dagger" => 0.444,
        "varepsilon" => 0.466,
        "delta" | "xi" => 0.483,
        "theta" | "emptyset" | "Vert" | "lVert" | "rVert" | "rbrace" => 0.5,
        "zeta" | "rho" => 0.514,
        "eta" | "hbar" => 0.536,
        "tau" | "forall" | "exists" | "nu" => 0.555,
        "gamma" | "kappa" | "upsilon" | "lambda" | "partial" => 0.58,
        "vartheta" | "phi" | "mu" | "pi" | "sigma" => 0.6,
        "beta" | "Gamma" | "chi" => 0.623,
        "alpha" | "varphi" | "omega" | "Xi" | "neg" => 0.655,
        "psi" | "Lambda" => 0.69,
        "Sigma" | "Phi" | "Omega" | "Re" | "Im" | "angle" => 0.722,
        "Pi" => 0.75,
        "Theta" | "Upsilon" | "Psi" => 0.778,
        "Delta" | "nabla" => 0.833,
        "infty" => 1.0,
        _ => 0.6,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn whole(latex: &str) -> f32 {
        math(latex).iter().sum()
    }

    #[test]
    fn widths_follow_the_font() {
        // "Hello" in 16 px Noto Sans: 741 + 564 + 258 + 258 + 605 thousandths
        assert!((text("Hello") - 38.816).abs() < 0.01);
        assert!(text("mmm") > 2.0 * text("iii"));
    }

    #[test]
    fn a_formula_is_measured_by_what_shows() {
        // a fraction is as wide as its wider half, set smaller
        assert!(whole(r"\frac{abc}{d}") < whole("abc"));
        // what is raised and lowered on one letter stands in one column
        assert!((whole("x_{ab}^{c}") - whole("x_{ab}")).abs() < 0.01);
        // names of styles and accents take no room
        assert!((whole(r"\hat{\mathrm{x}}") - whole("x")).abs() < 0.01);
        assert!(whole(r"\langle a | b \rangle") > whole("ab"));
        // a sign between two things has space around it, a leading one has none
        assert!(whole("a - b") > whole("- ab") + 0.4 * MATH);
    }

    #[test]
    fn a_line_may_end_after_a_sign_of_a_formula() {
        assert_eq!(math("a = b + c").len(), 3);
        // not inside braces, sized brackets or what is raised; not after a leading sign
        assert_eq!(math(r"{a + b}").len(), 1);
        assert_eq!(math(r"\left( a + b \right) c").len(), 1);
        assert_eq!(math("e^{a + b}").len(), 1);
        assert_eq!(math("- a").len(), 1);
    }
}
