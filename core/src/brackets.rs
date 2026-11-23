//! Brackets across the pieces of a sentence.
//!
//! A book sets the figures, commas and dots of a formula in the text's
//! font, so a list in brackets comes out of [`crate::blocks::line_inlines`]
//! in pieces: "$( \mu = 0$, 1, 2, 3 )", "$V(q_1 , q_2$, . . . , $q_N )$".
//! And a formula that ends a remark in brackets takes the remark's closing
//! bracket with it: "(where $c = 1 )$". Here, for a whole paragraph:
//!
//! * a formula with a bracket left open goes on through what follows —
//!   figures, commas, dots, signs, a function's name, one word, further
//!   formulas — up to the bracket that closes it;
//! * a formula with a bracket closed that it never opened takes the
//!   bracket (with a figure or a function's name behind it) from the text
//!   right before it: "(0 $+ 1 )$", "tr[1$/(z - H) ]$";
//! * what is left is the sentence's own bracket, and goes back to the text:
//!   "(where $c = 1$)", "$\mathcal{A}$(particle … in time $T$) $=$".
//!
//! Before that ([`sides`]) the two sides of a sign are put together: the
//! figure or the dots behind a formula that ends with "+" or "=", the
//! figure before one that begins with it.

use crate::Inline;

/// A round or square bracket of a formula that stands in no group of
/// braces: where, which, and whether it is sized (`\left(`).
#[derive(Clone, Copy, Debug, PartialEq)]
struct Bracket {
    at: usize,
    len: usize,
    c: char,
    sized: bool,
}

fn opens(c: char) -> bool {
    c == '(' || c == '['
}

fn partner(c: char) -> char {
    match c {
        '(' => ')',
        ')' => '(',
        '[' => ']',
        _ => '[',
    }
}

/// The brackets of a formula that have no partner in it: the closing ones
/// (in order), then the opening ones.
fn unmatched(m: &str) -> (Vec<Bracket>, Vec<Bracket>) {
    let b = m.as_bytes();
    let (mut closes, mut open): (Vec<Bracket>, Vec<Bracket>) = (Vec::new(), Vec::new());
    let (mut k, mut depth) = (0, 0usize);
    while k < b.len() {
        match b[k] {
            b'\\' => {
                let name = b[k + 1..].iter().take_while(|c| c.is_ascii_alphabetic()).count();
                let rest = &m[k + 1 + name..];
                let sized = matches!(&m[k + 1..k + 1 + name], "left" | "right" | "big" | "Big" | "bigg" | "Bigg" | "bigl" | "bigr" | "Bigl" | "Bigr");
                let next = rest.trim_start().chars().next();
                if sized && depth == 0 && next.is_some_and(|c| "()[]".contains(c)) {
                    let c = next.unwrap();
                    let len = 1 + name + (rest.len() - rest.trim_start().len()) + 1;
                    let br = Bracket { at: k, len, c, sized: true };
                    if opens(c) {
                        open.push(br);
                    } else if open.last().is_some_and(|o| o.c == partner(c)) {
                        open.pop();
                    } else {
                        closes.push(br);
                    }
                    k += len;
                } else {
                    // (a command, or one escaped sign: "\{", "\|")
                    k += 1 + name.max(usize::from(k + 1 < b.len()));
                    while k < b.len() && !m.is_char_boundary(k) {
                        k += 1;
                    }
                }
                continue;
            }
            b'{' => depth += 1,
            b'}' => depth = depth.saturating_sub(1),
            c @ (b'(' | b'[' | b')' | b']') if depth == 0 => {
                let c = c as char;
                let br = Bracket { at: k, len: 1, c, sized: false };
                if opens(c) {
                    open.push(br);
                } else if open.last().is_some_and(|o| o.c == partner(c)) {
                    open.pop();
                } else {
                    closes.push(br);
                }
            }
            _ => {}
        }
        k += 1;
    }
    (closes, open)
}

fn balanced(m: &str) -> bool {
    let (c, o) = unmatched(m);
    c.is_empty() && o.is_empty()
}

fn function(word: &str) -> Option<String> {
    if crate::mathtex::FUNCS.contains(&word) {
        Some(format!("\\{word}"))
    } else if crate::mathtex::OPNAMES.contains(&word) {
        Some(format!("\\operatorname{{{word}}}"))
    } else {
        None
    }
}

/// Text between the parts of a formula, as LaTeX: figures, commas, dots
/// ("and so on"), signs, "or"/"and" between items, "h.c.", a function's
/// name; and one word where nothing else stands (a unit, "hadrons"). None
/// for anything else: that is prose.
fn listed(t: &str) -> Option<String> {
    let c: Vec<char> = t.chars().collect();
    let mut out = String::new();
    let (mut k, mut words, mut points) = (0, 0, false);
    while k < c.len() {
        let x = c[k];
        if x.is_whitespace() {
            k += 1;
        } else if x.is_ascii_digit() {
            let mut e = k;
            while e < c.len() && (c[e].is_ascii_digit() || (c[e] == '.' && e + 1 < c.len() && c[e + 1].is_ascii_digit())) {
                e += 1;
            }
            out.push_str(&format!(" {}", c[k..e].iter().collect::<String>()));
            k = e;
        } else if x == '…' {
            out.push_str(" \\ldots");
            k += 1;
        } else if x == '.' {
            // (". . ." and "...": and so on; one full stop ends a sentence)
            let mut e = k;
            let mut dots = 0;
            while e < c.len() && (c[e] == '.' || (c[e] == ' ' && dots > 0 && e + 1 < c.len() && c[e + 1] == '.')) {
                dots += usize::from(c[e] == '.');
                e += 1;
            }
            if dots < 3 {
                return None;
            }
            out.push_str(" \\ldots");
            k = e;
        } else if ",;:".contains(x) {
            points = true;
            out.push_str(&format!(" {x}"));
            k += 1;
        } else if "+-=<>/!".contains(x) {
            out.push_str(&format!(" {x}"));
            k += 1;
        } else if x == '−' || x == '–' {
            out.push_str(" -");
            k += 1;
        } else if x == '±' {
            out.push_str(" \\pm");
            k += 1;
        } else if x.is_alphabetic() {
            let mut e = k;
            while e < c.len() && c[e].is_alphabetic() {
                e += 1;
            }
            let word: String = c[k..e].iter().collect();
            if word == "h" && c[e..].iter().collect::<String>().starts_with(".c.") {
                out.push_str(" \\text{h.c.}");
                k = e + 3;
                continue;
            }
            if let Some(f) = function(&word) {
                out.push_str(&format!(" {f}"));
            } else if word == "or" || word == "and" {
                out.push_str(&format!(" \\text{{ {word} }}"));
            } else {
                words += 1;
                let space = if k > 0 { "\\ " } else { "" };
                out.push_str(&format!(" {space}\\text{{{word}}}"));
            }
            k = e;
        } else {
            return None;
        }
    }
    // (one word and nothing listed beside it; never a sentence)
    (words == 0 || (words == 1 && !points && c.iter().filter(|x| x.is_alphabetic()).count() <= 10)).then_some(out)
}

fn joined(a: &str, b: &str) -> String {
    format!("{} {}", a.trim_end(), b.trim_start()).trim().to_string()
}

/// A formula with a bracket left open goes on through figures, commas,
/// dots and further formulas up to the bracket that closes it. The pieces
/// from `i` that make one formula: how many, the formula, and what is
/// left of the last piece (text behind the closing bracket).
fn forward(v: &[Inline], i: usize) -> Option<(usize, String, String)> {
    let Inline::Math(m) = &v[i] else { return None };
    let mut acc = m.trim().to_string();
    for (n, x) in v[i + 1..].iter().enumerate().take(24) {
        match x {
            Inline::Text(t) => {
                // (the closing bracket may stand in this text)
                for (p, c) in t.char_indices().filter(|(_, c)| *c == ')' || *c == ']') {
                    let Some(list) = listed(&t[..p]) else { break };
                    let all = joined(&joined(&acc, &list), &c.to_string());
                    if balanced(&all) {
                        return Some((n + 2, all, t[p + 1..].to_string()));
                    }
                }
                if t.chars().count() > 40 {
                    return None;
                }
                acc = joined(&acc, &listed(t)?);
            }
            Inline::Math(next) => {
                acc = joined(&acc, next);
                let (closes, open) = unmatched(&acc);
                if !closes.is_empty() {
                    return None;
                }
                if open.is_empty() {
                    return Some((n + 2, acc, String::new()));
                }
            }
            _ => return None,
        }
    }
    None
}

/// What stands at the end of `text` and belongs to the formula after it,
/// which closes a bracket `c` it never opened: that bracket, with a
/// function's name before it and a figure behind it ("tr[1", "(0 ", "[8!").
/// Where the bracket begins in `text`, and it all as LaTeX.
fn backward(text: &str, c: char) -> Option<(usize, String)> {
    let t = text.trim_end();
    let figures = t.len() - t.trim_end_matches(|x: char| x.is_ascii_digit() || ".,! ".contains(x)).len();
    let (head, tail) = t.split_at(t.len() - figures);
    // (a function's name inside the bracket: "(tr " and the formula)
    let inner = head.len() - head.trim_end_matches(|x: char| x.is_alphabetic()).len();
    let inside = function(&head[head.len() - inner..]);
    let head = if inside.is_some() { head[..head.len() - inner].trim_end() } else { head };
    let head = head.strip_suffix(partner(c))?;
    let mut latex = format!("{} {} {}", partner(c), inside.unwrap_or_default(), listed(tail)?.trim()).split_whitespace().collect::<Vec<_>>().join(" ");
    let mut at = head.len();
    let name = head.len() - head.trim_end_matches(|x: char| x.is_alphabetic()).len();
    if name > 0 {
        if let Some(f) = function(&head[head.len() - name..]) {
            latex = format!("{f} {latex}");
            at -= name;
        }
    }
    Some((at, latex))
}

/// Is a bracket `c` open in the sentence before piece `i` (looking at the
/// text and at what the formulas leave open)?
fn open_before(v: &[Inline], i: usize, c: char) -> bool {
    let mut depth = 0i32;
    for x in &v[..i] {
        match x {
            Inline::Text(t) => {
                for ch in t.chars() {
                    depth += i32::from(ch == c) - i32::from(ch == partner(c));
                    depth = depth.max(0);
                }
            }
            Inline::Math(m) => {
                let (closes, open) = unmatched(m);
                depth -= closes.iter().filter(|b| b.c == partner(c)).count() as i32;
                depth = depth.max(0) + open.iter().filter(|b| b.c == c).count() as i32;
            }
            _ => {}
        }
    }
    depth > 0
}

/// Is the bracket `c`, open at the end of piece `i`, closed further on in
/// the sentence?
fn closed_after(v: &[Inline], i: usize, c: char) -> bool {
    let mut depth = 1i32;
    for x in &v[i + 1..] {
        match x {
            Inline::Text(t) => {
                for ch in t.chars() {
                    depth += i32::from(ch == c) - i32::from(ch == partner(c));
                    if depth == 0 {
                        return true;
                    }
                }
            }
            Inline::Math(m) => {
                let (closes, open) = unmatched(m);
                depth -= closes.iter().filter(|b| b.c == partner(c)).count() as i32;
                if depth <= 0 {
                    return true;
                }
                depth += open.iter().filter(|b| b.c == c).count() as i32;
            }
            _ => {}
        }
    }
    false
}

fn push_text(out: &mut Vec<Inline>, t: &str) {
    if t.is_empty() {
        return;
    }
    if let Some(Inline::Text(prev)) = out.last_mut() {
        prev.push_str(t);
    } else {
        out.push(Inline::Text(t.to_string()));
    }
}

/// Signs that stand between two sides: a formula that ends with one, or
/// begins with one, is not whole.
const BETWEEN: [&str; 24] = [
    "to", "simeq", "approx", "equiv", "sim", "leq", "geq", "le", "ge", "neq", "ne", "ll", "gg", "propto", "times", "cdot", "pm", "mp", "otimes",
    "oplus", "rightarrow", "leftrightarrow", "leftarrow", "cong",
];

fn ends_open(m: &str) -> bool {
    let m = m.trim_end();
    if m.ends_with(['=', '+', '-', '<', '>', '/']) {
        // ("x^{-}" and "e^{+}" end with a brace; "A( ++" is a list of signs)
        return !m.ends_with("++") && !m.ends_with("--");
    }
    let name = m.len() - m.trim_end_matches(|c: char| c.is_ascii_alphabetic()).len();
    name > 0 && m[..m.len() - name].ends_with('\\') && BETWEEN.contains(&&m[m.len() - name..])
}

fn begins_open(m: &str) -> bool {
    let m = m.trim_start();
    if m.starts_with(['=', '+', '-', '<', '>', '/']) {
        return true;
    }
    let Some(rest) = m.strip_prefix('\\') else { return false };
    let name = rest.len() - rest.trim_start_matches(|c: char| c.is_ascii_alphabetic()).len();
    BETWEEN.contains(&&rest[..name])
}

/// Signs and figures a text begins with behind a formula (" + 0 = ",
/// " = 1."): the formula goes on through them. Their length and their
/// LaTeX, and whether they end with a figure (else the formula must go on
/// behind them). The full stop after the last figure is the sentence's.
fn sign_run(t: &str, after_sign: bool) -> Option<(usize, String, bool)> {
    let mut at = 0;
    let mut out: Vec<String> = Vec::new();
    // (behind a formula that ends with a sign, "a ∗", a figure comes first)
    let (mut want_sign, mut figure_last, mut end) = (!after_sign, false, 0);
    while at < t.len() {
        let rest = &t[at..];
        let lead = rest.len() - rest.trim_start_matches(' ').len();
        let word = rest[lead..].split(' ').next().unwrap_or("");
        if word.is_empty() {
            break;
        }
        let sign = match word {
            "=" | "+" | "<" | ">" | "*" => Some(word.to_string()),
            "−" | "-" => Some("-".to_string()),
            "≤" => Some("\\leq".to_string()),
            "≥" => Some("\\geq".to_string()),
            "≈" => Some("\\approx".to_string()),
            "×" => Some("\\times".to_string()),
            _ => None,
        };
        if let (true, Some(s)) = (want_sign, sign) {
            out.push(s);
            (want_sign, figure_last) = (false, false);
            at += lead + word.len();
            end = at;
            continue;
        }
        // a figure, a decimal too; the punctuation behind it stays
        let digits = word.len() - word.trim_start_matches(|c: char| c.is_ascii_digit()).len();
        let mut n = digits;
        if digits > 0 && word[n..].starts_with('.') && word[n + 1..].starts_with(|c: char| c.is_ascii_digit()) {
            n += 1 + word[n + 1..].len() - word[n + 1..].trim_start_matches(|c: char| c.is_ascii_digit()).len();
        }
        if want_sign || n == 0 || word[n..].starts_with(|c: char| c.is_alphanumeric()) {
            break;
        }
        out.push(word[..n].to_string());
        (want_sign, figure_last) = (true, true);
        at += lead + n;
        end = at;
        if n < word.len() {
            break;
        }
    }
    (!out.is_empty()).then(|| (end, out.join(" "), figure_last))
}

/// The figure a text begins with ("1 dimension", " 2, and"): its length.
fn figure_first(t: &str) -> Option<usize> {
    let lead = t.len() - t.trim_start_matches(' ').len();
    let rest = &t[lead..];
    let mut n = rest.len() - rest.trim_start_matches(|c: char| c.is_ascii_digit()).len();
    if n == 0 || lead > 1 {
        return None;
    }
    // (a decimal; not the full stop of the sentence)
    if rest[n..].starts_with('.') && rest[n + 1..].starts_with(|c: char| c.is_ascii_digit()) {
        n += 1 + rest[n + 1..].len() - rest[n + 1..].trim_start_matches(|c: char| c.is_ascii_digit()).len();
    }
    (!rest[n..].starts_with(|c: char| c.is_alphanumeric())).then_some(lead + n)
}

/// The figure a text ends with ("is 24 "): where it begins.
fn figure_last(t: &str) -> Option<usize> {
    let body = t.strip_suffix(' ').unwrap_or(t);
    let n = body.len() - body.trim_end_matches(|c: char| c.is_ascii_digit()).len();
    let before = &body[..body.len() - n];
    (n > 0 && (before.is_empty() || before.ends_with([' ', '(', '[']))).then_some(body.len() - n)
}

/// Dots of "and so on" at the head of a text (". . .", "..."): their length.
fn dots_first(t: &str) -> Option<usize> {
    let lead = t.len() - t.trim_start_matches(' ').len();
    let (mut n, mut dots) = (lead, 0);
    let b = t.as_bytes();
    while n < b.len() && (b[n] == b'.' || (b[n] == b' ' && dots > 0 && dots < 3 && b.get(n + 1) == Some(&b'.'))) {
        dots += usize::from(b[n] == b'.');
        n += 1;
        if dots == 3 {
            break;
        }
    }
    if t[lead..].starts_with('…') {
        return Some(lead + '…'.len_utf8());
    }
    (dots == 3).then_some(n)
}

/// The two sides of a sign, put together (for a whole paragraph): a
/// formula that ends with "+", "=", "→" … takes the figure or the dots of
/// "and so on" that the text goes on with, one that begins with such a
/// sign the figure the text ends with ("24" + "+ 2 = 26"); two formulas
/// with such a sign between them are one, as is a list that runs through
/// dots ("i = 1" + ", 2, . . . " + "n"). The "−" before a word set right
/// against it is a hyphen: "(D + 1)" + "-dimensional".
pub(crate) fn sides(mut v: Vec<Inline>) -> Vec<Inline> {
    if !v.iter().any(|x| matches!(x, Inline::Math(_) | Inline::Sup(_))) {
        return v;
    }
    let mut k = 0;
    while k < v.len() {
        // (a raised figure of the text's font: the power of what it stands on)
        if let Some(Inline::Sup(d)) = v.get(k + 1).filter(|x| matches!(x, Inline::Sup(d) if !d.is_empty() && d.split('/').count() <= 2 && d.split('/').all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit())))).cloned() {
            let after = match (v.get(k + 2), v.get(k + 3)) {
                (Some(Inline::Math(m)), _) => Some((1, m.clone())),
                (Some(Inline::Text(t)), Some(Inline::Math(m))) if t == " " => Some((2, m.clone())),
                _ => None,
            };
            match &mut v[k] {
                // ("[ … ]" + 2, "8!/(4!)^3 2" + 4)
                Inline::Math(m) if m.trim_end().ends_with([')', ']']) || m.trim_end().ends_with(|c: char| c.is_ascii_digit()) => {
                    *m = format!("{}^{{{d}}}", m.trim_end());
                    v.remove(k + 1);
                    // (what follows right behind is the same formula)
                    if let (Some(Inline::Math(_)), Some((1, next))) = (v.get(k + 1), &after) {
                        v[k] = Inline::Math(joined(if let Inline::Math(m) = &v[k] { m } else { "" }, next));
                        v.remove(k + 1);
                    }
                    continue;
                }
                Inline::Text(t) => {
                    // ("sin" + 2 + "θ": the function's power)
                    let name = t.len() - t.trim_end_matches(|c: char| c.is_alphabetic()).len();
                    let figures = t.len() - t.trim_end_matches(|c: char| c.is_ascii_digit()).len();
                    let alone = |t: &str, n: usize| t[..t.len() - n].chars().last().is_none_or(|c| !c.is_alphanumeric() && c != '.');
                    if let (Some(f), true, Some((n, m))) = (function(&t[t.len() - name..]), alone(t, name), &after) {
                        t.truncate(t.len() - name);
                        v.splice(k + 1..=k + 1 + n, [Inline::Math(format!("{f}^{{{d}}} {}", m.trim_start()))]);
                        continue;
                    }
                    // ("10" + 19; "3" + 2 before "− 1 = 8")
                    let base = t[t.len() - figures..].to_string();
                    let goes_on = after.as_ref().is_some_and(|(_, m)| begins_open(m));
                    if figures > 0 && alone(t, figures) && (base == "10" || goes_on) {
                        t.truncate(t.len() - figures);
                        v[k + 1] = Inline::Math(format!("{base}^{{{d}}}"));
                        continue;
                    }
                }
                _ => {}
            }
        }
        // (the text behind a formula)
        if k + 1 < v.len() {
            let (a, b) = v.split_at_mut(k + 1);
            if let (Inline::Math(m), Inline::Text(t)) = (&mut a[k], &mut b[0]) {
                // (a word after a "−": a hyphen, also where the line ended on it)
                let word = t.strip_prefix(' ').unwrap_or(t);
                let letters = word.len() - word.trim_start_matches(|c: char| c.is_lowercase()).len();
                if m.trim_end().ends_with('-') && !m.trim_end().ends_with("--") && letters >= if word.len() < t.len() { 4 } else { 1 } && m.trim().len() > 1 {
                    *m = m.trim_end().trim_end_matches('-').trim_end().to_string();
                    *t = format!("-{word}");
                } else if let (true, Some(n)) = (ends_open(m), figure_first(t)) {
                    *m = joined(m, t[..n].trim());
                    t.replace_range(..n, "");
                } else if let (true, Some(n)) = (ends_open(m) || m.trim_end().ends_with(','), dots_first(t)) {
                    *m = joined(m, if m.trim_end().ends_with(',') { "\\ldots" } else { "\\cdots" });
                    t.replace_range(..n, "");
                }
            }
        }
        // (signs and figures behind a formula, set in the text's font: "a"
        // + " + 0 = " + "a" is one formula, "a ∗ a⁻¹" + " = 1." ends with
        // the 1. A sign alone is taken only where a formula follows it.)
        if k + 1 < v.len() && matches!(&v[k], Inline::Math(_)) {
            let after_sign = matches!(&v[k], Inline::Math(m) if m.trim_end().ends_with('*') || ends_open(m));
            let run = match &v[k + 1] {
                Inline::Text(t) => sign_run(t, after_sign).filter(|(n, _, figure)| *figure || (t[*n..].trim().is_empty() && matches!(v.get(k + 2), Some(Inline::Math(_))))),
                _ => None,
            };
            if let Some((n, tex, _)) = run {
                if let Inline::Math(m) = &mut v[k] {
                    *m = joined(m, &tex);
                }
                if let Inline::Text(t) = &mut v[k + 1] {
                    t.replace_range(..n, "");
                }
            }
        }
        // (a word set right against a formula: a function's name before
        // its bracket is the formula's — "det" + "[ A ]", "tr log" + "[…]" —
        // and any other word has lost its space; so has a word behind a
        // formula, but for "th" and "st")
        let mut named = false;
        if k > 0 {
            let (a, b) = v.split_at_mut(k);
            if let (Inline::Text(t), Inline::Math(m)) = (&mut a[k - 1], &mut b[0]) {
                loop {
                    let body = t.strip_suffix(' ').unwrap_or(t);
                    let spaced = body.len() < t.len();
                    let n = body.len() - body.trim_end_matches(|c: char| c.is_ascii_alphabetic()).len();
                    let word = &body[body.len() - n..];
                    let alone = !body[..body.len() - n].ends_with(|c: char| c.is_alphanumeric());
                    let name = function(word).or_else(|| (word == "sign").then(|| "\\operatorname{sign}".to_string())).filter(|_| alone);
                    let takes = if spaced { m.starts_with("\\operatorname") || crate::mathtex::FUNCS.iter().any(|f| m.strip_prefix('\\').is_some_and(|r| r.starts_with(f))) } else { m.starts_with(['(', '[']) };
                    match name {
                        Some(f) if takes => {
                            *m = format!("{f} {m}");
                            t.truncate(body.len() - n);
                            named = true;
                        }
                        _ => {
                            if !spaced && n >= 2 && !m.starts_with(['(', '[']) {
                                t.push(' ');
                            }
                            break;
                        }
                    }
                }
            }
        }
        // (the name stood between two parts of one formula: "⟨" + "tr" + "[…]⟩")
        if named && k >= 2 && matches!(&v[k - 1], Inline::Text(t) if t.trim().is_empty()) && matches!(&v[k - 2], Inline::Math(_)) {
            let (Inline::Math(a), Inline::Math(b)) = (&v[k - 2], &v[k]) else { unreachable!() };
            let all = joined(a, b);
            v.splice(k - 2..=k, [Inline::Math(all)]);
            k -= 2;
            continue;
        }
        if k + 1 < v.len() {
            let (a, b) = v.split_at_mut(k + 1);
            if let (Inline::Math(_), Inline::Text(t)) = (&a[k], &mut b[0]) {
                let n = t.len() - t.trim_start_matches(|c: char| c.is_ascii_lowercase()).len();
                // (a unit of energy too, "10^{19}" + "Gev")
                let unit = ["Gev", "Mev", "Tev", "GeV", "MeV", "TeV"].iter().any(|u| t.strip_prefix(u).is_some_and(|r| !r.starts_with(|c: char| c.is_alphanumeric())));
                if unit || (n >= 2 && !["th", "st", "nd", "rd"].contains(&&t[..n])) {
                    t.insert(0, ' ');
                }
            }
        }
        // (an exponent or index with nothing under it, behind a bracket:
        // the bracket's — "(length)" and its power, "[1]" and its star)
        if let Inline::Math(m) = &mut v[k] {
            // (the sign of degrees is of the figure before it)
            *m = m.replace(" {}^{\\circ}", "^{\\circ}");
            for c in [')', ']'] {
                for script in ['^', '_'] {
                    *m = m.replace(&format!("{c} {{}}{script}{{"), &format!("{c}{script}{{"));
                }
            }
        }
        if k > 0 {
            let (a, b) = v.split_at_mut(k);
            if let (Inline::Text(t), Inline::Math(m)) = (&mut a[k - 1], &mut b[0]) {
                let body = t.strip_suffix(' ').unwrap_or(t);
                let close = body.chars().last().filter(|c| *c == ')' || *c == ']');
                let open = close.and_then(|c| body.rfind(partner(c)));
                if let (true, Some(c), Some(at)) = (m.starts_with("{}^{") || m.starts_with("{}_{"), close, open) {
                    let inner = &body[at + 1..body.len() - 1];
                    if let Some(list) = listed(inner).filter(|_| inner.chars().count() <= 14 && !inner.contains(['(', '['])) {
                        *m = format!("{} {} {c}{}", partner(c), list.trim(), &m[2..]);
                        t.truncate(at);
                    }
                }
            }
        }
        // (an upright letter or two set right against a raised or lowered
        // piece that has nothing else under it: "e" + "⁻", "K" + "⁺" — the
        // letter is what the piece stands on)
        if k > 0 {
            let (a, b) = v.split_at_mut(k);
            if let (Inline::Text(t), Inline::Math(m)) = (&mut a[k - 1], &mut b[0]) {
                let n = t.len() - t.trim_end_matches(|c: char| c.is_ascii_alphabetic()).len();
                let alone = !t[..t.len() - n].ends_with(|c: char| c.is_alphanumeric() || c == '\'' || c == '’');
                if (m.starts_with("{}^{") || m.starts_with("{}_{")) && (1..=2).contains(&n) && alone {
                    *m = format!("\\mathrm{{{}}}{}", &t[t.len() - n..], &m[2..]);
                    t.truncate(t.len() - n);
                }
            }
        }
        // (the text before a formula)
        if k > 0 {
            let (a, b) = v.split_at_mut(k);
            if let (Inline::Text(t), Inline::Math(m)) = (&mut a[k - 1], &mut b[0]) {
                if let (true, Some(at)) = (begins_open(m), figure_last(t)) {
                    *m = joined(t[at..].trim(), m);
                    t.truncate(at);
                }
            }
        }
        // (two formulas, with nothing or a list through dots between them)
        let gap = match (v.get(k), v.get(k + 1), v.get(k + 2)) {
            (Some(Inline::Math(_)), Some(Inline::Math(_)), _) => Some((1, String::new())),
            (Some(Inline::Math(_)), Some(Inline::Text(t)), Some(Inline::Math(_))) if t.trim().is_empty() => Some((2, String::new())),
            // (through dots, or over a function's name or "h.c." beside a sign)
            (Some(Inline::Math(a)), Some(Inline::Text(t)), Some(Inline::Math(b))) => listed(t).filter(|l| {
                let plain = l.replace("\\text{h.c.}", "");
                let names = !plain.contains(|c: char| c.is_ascii_digit() || ",;:".contains(c)) && (ends_open(a) || begins_open(b));
                !plain.contains("\\text") && (l.contains("\\ldots") || names)
            }).map(|l| (2, l)),
            _ => None,
        };
        if let Some((n, list)) = gap {
            let (Inline::Math(a), Inline::Math(b)) = (&v[k], &v[k + n]) else { unreachable!() };
            if !list.is_empty() || ends_open(a) || begins_open(b) {
                let all = joined(&joined(a, &list), b);
                v.splice(k..=k + n, [Inline::Math(all)]);
                // (it may go on: looked at again)
                continue;
            }
        }
        k += 1;
    }
    v.retain(|x| !matches!(x, Inline::Text(t) if t.is_empty()));
    v
}

/// The brackets of a paragraph's formulas, mended (see the module's head).
pub(crate) fn mend(v: Vec<Inline>) -> Vec<Inline> {
    if !v.iter().any(|x| matches!(x, Inline::Math(m) if !balanced(m))) {
        return v;
    }
    let mut v = v;
    // 1. A bracket left open: the formula goes on to where it closes.
    let mut i = 0;
    while i < v.len() {
        let open = matches!(&v[i], Inline::Math(m) if !unmatched(m).1.is_empty() && unmatched(m).0.is_empty());
        if let Some((n, formula, rest)) = open.then(|| forward(&v, i)).flatten() {
            let mut new = vec![Inline::Math(formula)];
            if !rest.is_empty() {
                new.push(Inline::Text(rest));
            }
            v.splice(i..i + n, new);
        }
        i += 1;
    }
    // 2. A bracket closed and never opened: opened in the text right before.
    for i in 1..v.len() {
        let (a, b) = v.split_at_mut(i);
        let (Inline::Text(t), Inline::Math(m)) = (&mut a[i - 1], &mut b[0]) else { continue };
        let (closes, open) = unmatched(m);
        let (Some(first), true) = (closes.first(), open.is_empty()) else { continue };
        // (the formula is more than the bracket: "(2" + ")" is a number in brackets)
        if m.chars().all(|c| ")].,;: ".contains(c)) {
            continue;
        }
        if let Some((at, latex)) = backward(t, first.c) {
            let all = joined(&latex, m);
            if balanced(&all) {
                t.truncate(at);
                *m = all;
            }
        }
    }
    // 3. What is left is the sentence's own bracket: back to the text.
    let mut out: Vec<Inline> = Vec::with_capacity(v.len());
    let mut k = 0;
    while k < v.len() {
        let Inline::Math(m) = &v[k] else {
            match &v[k] {
                Inline::Text(t) => push_text(&mut out, t),
                x => out.push(x.clone()),
            }
            k += 1;
            continue;
        };
        let (closes, open) = unmatched(m);
        let points = |c: char| ".,;: ".contains(c);
        // (an interval "[0, 1)" has both, and is left as it is)
        let cut = match (closes.first(), open.last()) {
            (Some(b), None) if !b.sized && open_before(&v, k, partner(b.c)) => Some(*b),
            (None, Some(b)) if !b.sized && closed_after(&v, k, b.c) => Some(*b),
            _ => None,
        };
        // (a bracket with an exponent or index on it is the formula's)
        let cut = cut.filter(|b| !m[b.at + 1..].trim_start().starts_with(['^', '_']));
        let Some(b) = cut else {
            out.push(v[k].clone());
            k += 1;
            continue;
        };
        let (left, right) = (m[..b.at].trim(), m[b.at + 1..].trim());
        // (the full stop or comma beside the bracket is the sentence's too)
        let (l, r) = (left.trim_end_matches(points), right.trim_start_matches(points));
        let (after_left, before_right) = (left[l.len()..].replace(' ', ""), right[..right.len() - r.len()].replace(' ', ""));
        if !l.is_empty() {
            out.push(Inline::Math(l.to_string()));
        }
        let mut text = format!("{after_left}{}{before_right}", b.c);
        if !r.is_empty() && (!opens(b.c) || !before_right.is_empty()) {
            text.push(' ');
        }
        // (a bracket opened in the middle of a formula opens a remark:
        // a space before it; at its end, what a function is taken of)
        if opens(b.c) && !l.is_empty() && !r.is_empty() {
            text.insert(0, ' ');
        }
        push_text(&mut out, &text);
        if r.is_empty() {
            k += 1;
        } else {
            // (the rest may hold another such bracket: looked at again)
            v[k] = Inline::Math(r.to_string());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn m(t: &str) -> Inline {
        Inline::Math(t.into())
    }
    fn t(t: &str) -> Inline {
        Inline::Text(t.into())
    }

    #[test]
    fn signs_and_figures_behind_a_formula_are_the_formulas() {
        // "+ 0 =" set in the text's font between two letters of a formula
        assert_eq!(sides(vec![m("a \\in \\mathbb{R} , a"), t(" + 0 = "), m("a"), t(".")]), vec![m("a \\in \\mathbb{R} , a + 0 = a"), t(".")]);
        // the full stop behind the figure is the sentence's
        assert_eq!(sides(vec![m("a * a^{- 1}"), t(" = 1.")]), vec![m("a * a^{- 1} = 1"), t(".")]);
        // a sign alone before a word is left to the text
        assert_eq!(sides(vec![m("x"), t(" = the length")]), vec![m("x"), t(" = the length")]);
    }

    #[test]
    fn the_two_sides_of_a_sign_are_one_formula() {
        assert_eq!(sides(vec![t("to be 24 "), m("+ 2 = 2 6"), t(" which")]), vec![t("to be "), m("24 + 2 = 2 6"), t(" which")]);
        assert_eq!(sides(vec![t("write "), m("d = D +"), t(" 1 for")]), vec![t("write "), m("d = D + 1"), t(" for")]);
        assert_eq!(sides(vec![t("in a "), m("( D + 1 ) -"), t("dimensional world")]), vec![t("in a "), m("( D + 1 )"), t("-dimensional world")]);
        assert_eq!(sides(vec![m("p_{1} + p_{2} \\to k_{1} + k_{2} +"), t(" . . . "), m("+ k_{n}"), t(". Then")]), vec![m("p_{1} + p_{2} \\to k_{1} + k_{2} + \\cdots + k_{n}"), t(". Then")]);
        assert_eq!(sides(vec![m("D + Ek^{2} +"), t(" . . . . The")]), vec![m("D + Ek^{2} + \\cdots"), t(" . The")]);
        assert_eq!(sides(vec![m("m_{G} \\ll"), t(" "), m("( 1 0^{2 4} )^{- 1}")]), vec![m("m_{G} \\ll ( 1 0^{2 4} )^{- 1}")]);
        assert_eq!(sides(vec![t("for "), m("i = 1"), t(", 2, . . . "), m("n"), t(" we")]), vec![t("for "), m("i = 1 , 2 , \\ldots n"), t(" we")]);
        assert_eq!(sides(vec![m("m =- j , - j +"), t(" 1, . . . , "), m("j - 1 , j")]), vec![m("m =- j , - j + 1 , \\ldots , j - 1 , j")]);
        assert_eq!(sides(vec![m("S(P) ="), t(" Re "), m("\\operatorname{tr} U")]), vec![m("S(P) = \\operatorname{Re} \\operatorname{tr} U")]);
        assert_eq!(sides(vec![m("\\bar{\\psi}_{L} \\psi_{R} +"), t(" h.c. "), m("= x"), t(" is")]), vec![m("\\bar{\\psi}_{L} \\psi_{R} + \\text{h.c.} = x"), t(" is")]);
        assert_eq!(sides(vec![t("an "), m("N -"), t(" dimensional vector")]), vec![t("an "), m("N"), t("-dimensional vector")]);
        let sup = |d: &str| Inline::Sup(d.into());
        assert_eq!(sides(vec![t("and sin"), sup("2"), t(" "), m("\\theta ]"), t(" is")]), vec![t("and "), m("\\sin^{2} \\theta ]"), t(" is")]);
        assert_eq!(sides(vec![t("which are 3"), sup("2"), t(" "), m("- 1 = 8"), t(", the")]), vec![t("which are "), m("3^{2} - 1 = 8"), t(", the")]);
        assert_eq!(sides(vec![t("scale 10"), sup("19"), t(" GeV as")]), vec![t("scale "), m("10^{19}"), t(" GeV as")]);
        assert_eq!(sides(vec![m("\\langle 1 2 \\rangle [ 3 4 ]"), sup("2"), m("/( [ 1 2 ] )")]), vec![m("\\langle 1 2 \\rangle [ 3 4 ]^{2} /( [ 1 2 ] )")]);
        assert_eq!(sides(vec![t("dimension of (length) "), m("{}^{\\tfrac{1}{2} (D - 2 )}"), t(". Thus")]), vec![t("dimension of "), m("( \\text{length} )^{\\tfrac{1}{2} (D - 2 )}"), t(". Thus")]);
        assert_eq!(sides(vec![t("equivalent to [1]"), m("{}^{*} = 5^{*}")]), vec![t("equivalent to "), m("[ 1 ]^{*} = 5^{*}")]);
        assert_eq!(sides(vec![m("( \\text{length} ) {}^{2}")]), vec![m("( \\text{length} )^{2}")]);
        assert_eq!(sides(vec![t("we need tr log"), m("[ i \\partial - m ]"), t("which is")]), vec![t("we need "), m("\\operatorname{tr} \\log [ i \\partial - m ]"), t(" which is")]);
        assert_eq!(sides(vec![t("denoted by"), m("\\psi"), t(". The "), m("n"), t("th one")]), vec![t("denoted by "), m("\\psi"), t(". The "), m("n"), t("th one")]);
        assert_eq!(sides(vec![m("G(z) \\equiv \\langle"), t("tr"), m("[ 1 /(z - H) ] \\rangle")]), vec![m("G(z) \\equiv \\langle \\operatorname{tr} [ 1 /(z - H) ] \\rangle")]);
        let greek = vec![t("with Ω"), m("{}_{1} ( t )"), t("Ψ = 0 in L"), m("A"), t("TEX")];
        assert_eq!(sides(greek.clone()), greek);
        // (a note's mark after a word or a year stays one)
        let mark = vec![t("as shown in 1905"), sup("3"), t(" by")];
        assert_eq!(sides(mark.clone()), mark);
        // (two formulas of a sentence stay two; so does a figure of the sentence)
        let two = vec![m("x"), t(" "), m("y"), t(" and "), m("a = b"), t(", 2 of them")];
        assert_eq!(sides(two.clone()), two);
        let signs = vec![m("e^{+}"), t(" 2 jets and "), m("\\sin"), t(" 2")];
        assert_eq!(sides(signs.clone()), signs);
    }

    #[test]
    fn a_list_in_brackets_is_one_formula() {
        assert_eq!(mend(vec![t("labeled by "), m("( \\mu = 0"), t(", 1, 2, 3 ) with")]), vec![t("labeled by "), m("( \\mu = 0 , 1 , 2 , 3 )"), t(" with")]);
        assert_eq!(mend(vec![m("V(q_{1} , q_{2}"), t(", . . . , "), m("q_{N} )"), t(" is")]), vec![m("V(q_{1} , q_{2} , \\ldots , q_{N} )"), t(" is")]);
        assert_eq!(mend(vec![m("24 \\to ( 8"), t(", 1"), m(") + ( 1"), t(", 3"), m(")"), t(".")]), vec![m("24 \\to ( 8 , 1 ) + ( 1 , 3 )"), t(".")]);
        assert_eq!(mend(vec![m("J_{i} (i = 1"), t(", 2, 3 or "), m("x , y , z )")]), vec![m("J_{i} (i = 1 , 2 , 3 \\text{ or } x , y , z )")]);
        assert_eq!(mend(vec![m("m( \\bar{\\psi}_{L} \\psi_{R} +"), t(" h.c."), m(")"), t(" is")]), vec![m("m( \\bar{\\psi}_{L} \\psi_{R} + \\text{h.c.} )"), t(" is")]);
        assert_eq!(mend(vec![m("\\sigma (e^{+} e^{-} \\to"), t(" hadrons) falls")]), vec![m("\\sigma (e^{+} e^{-} \\to \\ \\text{hadrons} )"), t(" falls")]);
        assert_eq!(mend(vec![m("( 1 /"), t("det"), m("[ i \\partial - m ] ) = e^{x}")]), vec![m("( 1 / \\det [ i \\partial - m ] ) = e^{x}")]);
    }

    #[test]
    fn a_bracket_opened_in_the_text_before_a_formula_is_the_formulas() {
        assert_eq!(mend(vec![t("a (0 "), m("+ 1 )"), t("-dimensional")]), vec![t("a "), m("( 0 + 1 )"), t("-dimensional")]);
        assert_eq!(mend(vec![t("that tr[1"), m("/(z - H) ] = x")]), vec![t("that "), m("\\operatorname{tr} [ 1 /(z - H) ] = x")]);
        assert_eq!(mend(vec![t("and (tr "), m("\\phi^{2} )^{2}")]), vec![t("and "), m("( \\operatorname{tr} \\phi^{2} )^{2}")]);
        // (a number in brackets is an equation's number)
        let reference = vec![t("in (2"), m(")"), t(" we")];
        assert_eq!(mend(reference.clone()), vec![t("in (2) we")]);
    }

    #[test]
    fn the_sentences_own_bracket_goes_back_to_the_text() {
        assert_eq!(mend(vec![t("(where "), m("c = 1 )")]), vec![t("(where "), m("c = 1"), t(")")]);
        assert_eq!(mend(vec![t("(try it for "), m("N = 2 . )")]), vec![t("(try it for "), m("N = 2"), t(".)")]);
        assert_eq!(mend(vec![t("defined "), m("( x^{0} \\equiv t"), t(" as usual)")]), vec![t("defined ("), m("x^{0} \\equiv t"), t(" as usual)")]);
        assert_eq!(
            mend(vec![m("\\mathcal{A} ("), t("particle to go in time "), m("T) =")]),
            vec![m("\\mathcal{A}"), t("(particle to go in time "), m("T"), t(") "), m("=")]
        );
        assert_eq!(mend(vec![m("\\theta_{s} + \\theta_{w} ( s"), t(" for strong and "), m("w"), t(" for weak)")]), vec![m("\\theta_{s} + \\theta_{w}"), t(" ("), m("s"), t(" for strong and "), m("w"), t(" for weak)")]);
        // (nothing open in the sentence: left alone; so is an interval)
        let alone = vec![t("for "), m("x )"), t(" only")];
        assert_eq!(mend(alone.clone()), alone);
        let interval = vec![t("(for "), m("x \\in [ 0 , 1 )"), t(")")];
        assert_eq!(mend(interval.clone()), interval);
        let power = vec![t("(the "), m("x )^{2}"), t(" one)")];
        assert_eq!(mend(power.clone()), power);
        // (brackets in an exponent are the exponent's)
        let script = vec![t("the "), m("\\epsilon^{( 1 )}"), t(" one")];
        assert_eq!(mend(script.clone()), script);
    }
}
