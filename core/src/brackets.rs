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
