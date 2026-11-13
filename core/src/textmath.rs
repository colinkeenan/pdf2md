//! Where math begins and ends in a line of prose, in a book that sets its
//! math in the text font (an ebook: no TeX math fonts to tell by).
//!
//! The PDF gives such a line as pieces by typeface: "elements of form ⟨",
//! an italic "a″", "|", an italic "X", "|", an italic "a′", "⟩, where ".
//! Each piece is cut into tokens (words, symbols, numbers, brackets,
//! punctuation), every token gets a role, and a stretch of tokens with no
//! word in it is a formula if something in it is surely math: an italic
//! letter, a Greek letter, "=", "⟨", a subscript, an equation picture.
//! Numbers, brackets, "+", a lone upright letter ("Z = 2") or a function
//! name ("sin") only go along with math beside them.

use crate::blocks::{Stats, K};
use crate::layout::{char_width, Doc, Script, Span};

#[derive(Clone, Copy, PartialEq, Debug)]
enum Role {
    /// prose (also a link, a picture, an italic or bold phrase)
    Word,
    /// surely math
    Atom,
    Open,
    Close,
    Number,
    /// a lone upright letter
    Letter,
    /// "sin", "exp", "Tr"
    Func,
    /// "+", "−", "/", "!", a prime: math only beside math
    Loose,
    Punct,
    /// two or three italic letters: "kR", "dt" — or "is", "or"
    Maybe,
    /// a footnote marker
    Marker,
}

/// Short words that are set in italics as words, not as symbols.
const STOP: &[&str] = &[
    "a", "an", "as", "at", "be", "by", "do", "go", "he", "if", "in", "is", "it", "me", "my", "no", "of", "on", "or", "so", "to", "up",
    "us", "we", "the", "and", "not", "but", "for", "all", "any", "are", "can", "has", "had", "its", "may", "nor", "one", "out", "own",
    "per", "see", "set", "two", "via", "was", "who", "why", "yes", "yet", "ket", "bra", "his", "her", "our", "how", "now", "new", "old",
    "too", "off",
];
/// ... and these only in small letters ("Et" is energy times time).
const STOP_SMALL: &[&str] = &[
    "et", "al", "cf", "eg", "ie", "vs", "viz", "op", "cit", "de", "la", "le", "du", "von", "van", "der", "den", "ad", "hoc", "sic", "re",
    "ibid",
];

fn is_stop(w: &str) -> bool {
    let small = w.chars().all(char::is_lowercase);
    let cased = w.chars().skip(1).all(char::is_lowercase);
    (cased && STOP.contains(&w.to_lowercase().as_str())) || (small && STOP_SMALL.contains(&w))
}

fn is_combining(c: char) -> bool {
    matches!(c, '\u{300}'..='\u{36f}' | '\u{20d7}')
}

fn is_greek(c: char) -> bool {
    matches!(c, 'α'..='ω' | 'Α'..='Ω' | 'ϕ' | 'ϵ' | 'ϑ' | 'ϖ' | 'ϱ')
}

/// Relations, operators and brackets that prose never uses.
fn is_operator(c: char) -> bool {
    "=≈≡≠≤≥→←↔⇒∼≃≅∝±∓×∞∂∇∑∫√⟨⟩〈〉\u{2329}\u{232a}|ħℏћ∈⊗⊕≪≫∥⊥".contains(c)
}

fn is_sure(c: char) -> bool {
    is_greek(c) || is_operator(c)
}

/// ⟨ in any of its three spellings (and ⟩).
fn is_langle(t: &str) -> bool {
    matches!(t, "⟨" | "〈" | "\u{2329}")
}
fn is_rangle(t: &str) -> bool {
    matches!(t, "⟩" | "〉" | "\u{232a}")
}

fn is_prime(c: char, symbols: bool) -> bool {
    "′″‴".contains(c) || (symbols && (c == '\'' || c == '"'))
}

fn is_loose(c: char) -> bool {
    "+−/*<>·†′″‴!".contains(c)
}

struct Tok {
    /// first character and the one after the last
    a: usize,
    b: usize,
    role: Role,
}

/// The tokens of one piece of text. Prose (letters, spaces, quotes) stays
/// together; in an italic piece a run of one to three letters is a symbol
/// or a candidate for one.
fn tokens(cs: &[char], italic: bool) -> Vec<Tok> {
    let mut out: Vec<Tok> = Vec::new();
    let mut prose: Option<usize> = None;
    let flush = |out: &mut Vec<Tok>, prose: &mut Option<usize>, end: usize| {
        if let Some(a) = prose.take() {
            if cs[a..end].iter().any(|c| !c.is_whitespace()) {
                out.push(Tok { a, b: end, role: Role::Word });
            }
        }
    };
    let mut i = 0;
    while i < cs.len() {
        let c = cs[i];
        let prev = i.checked_sub(1).map(|k| cs[k]);
        let next = cs.get(i + 1).copied();
        if is_combining(c) {
            // an accent typed after its letter
            if let Some(last) = out.last_mut().filter(|t| t.b == i && prose.is_none()) {
                last.b = i + 1;
                i += 1;
                continue;
            }
        }
        let role = if is_sure(c) {
            // with its primes: ψ′
            let mut j = i + 1;
            while j < cs.len() && is_prime(cs[j], italic) {
                j += 1;
            }
            if j > i + 1 {
                flush(&mut out, &mut prose, i);
                out.push(Tok { a: i, b: j, role: Role::Atom });
                i = j;
                continue;
            }
            Some(Role::Atom)
        } else if "′″‴".contains(c) && !italic && prev.is_some_and(char::is_alphabetic) && next.is_some_and(char::is_alphabetic) {
            // the apostrophe of "Green′s"
            None
        } else if "([{".contains(c) {
            Some(Role::Open)
        } else if ")]}".contains(c) {
            Some(Role::Close)
        } else if c.is_ascii_digit() {
            // "3d", "H2", "2nd" are words
            let mut j = i;
            while j < cs.len() && (cs[j].is_ascii_digit() || (".,".contains(cs[j]) && cs.get(j + 1).is_some_and(char::is_ascii_digit) && j > i)) {
                j += 1;
            }
            let glued = prev.is_some_and(char::is_alphabetic) && prose.is_some() || cs.get(j).is_some_and(|c| c.is_alphabetic() && !is_greek(*c));
            if glued && !italic {
                prose.get_or_insert(i);
                i = j;
                continue;
            }
            flush(&mut out, &mut prose, i);
            out.push(Tok { a: i, b: j, role: Role::Number });
            i = j;
            continue;
        } else if c == '-' {
            // a minus sign, not the hyphen of "x-polarized"
            let free = |c: Option<char>| c.is_none_or(char::is_whitespace);
            (next.is_some_and(|c| c.is_ascii_digit()) && !prev.is_some_and(char::is_alphanumeric) || (free(prev) && free(next))).then_some(Role::Loose)
        } else if c == '/' {
            (!(prev.is_some_and(char::is_alphabetic) && next.is_some_and(char::is_alphabetic)) || italic).then_some(Role::Loose)
        } else if is_loose(c) || (italic && is_prime(c, true)) {
            Some(Role::Loose)
        } else if ",;:.?".contains(c) {
            Some(Role::Punct)
        } else if italic && c.is_alphabetic() {
            let mut j = i;
            while j < cs.len() && cs[j].is_alphabetic() && !is_greek(cs[j]) {
                j += 1;
            }
            // primes after the last letter: a″, dx′
            let mut p = j;
            while p < cs.len() && is_prime(cs[p], true) {
                p += 1;
            }
            if j - i == 1 || (p > j && j - i <= 3) {
                flush(&mut out, &mut prose, i);
                for k in i..j - 1 {
                    out.push(Tok { a: k, b: k + 1, role: Role::Atom });
                }
                out.push(Tok { a: j - 1, b: p, role: Role::Atom });
                j = p;
            } else if j - i <= 3 {
                flush(&mut out, &mut prose, i);
                out.push(Tok { a: i, b: j, role: Role::Maybe });
            } else {
                prose.get_or_insert(i);
            }
            i = j;
            continue;
        } else {
            None
        };
        match role {
            Some(role) => {
                flush(&mut out, &mut prose, i);
                out.push(Tok { a: i, b: i + 1, role });
            }
            None => {
                prose.get_or_insert(i);
            }
        }
        i += 1;
    }
    flush(&mut out, &mut prose, cs.len());
    if italic {
        return out;
    }
    // a lone letter or a function name at the edge of a stretch of prose,
    // where math may stand beside it: its own token
    let mut cut: Vec<Tok> = Vec::with_capacity(out.len() + 2);
    for t in out {
        if t.role != Role::Word {
            cut.push(t);
            continue;
        }
        let word = |a: usize, b: usize| -> Option<Role> {
            let w: String = cs[a..b].iter().collect();
            if b - a == 1 && cs[a].is_ascii_alphabetic() && !matches!(cs[a], 'a' | 'A' | 'I') {
                Some(Role::Letter)
            } else if crate::mathtex::is_function_word(&w) {
                Some(Role::Func)
            } else {
                None
            }
        };
        // the words of the stretch
        let mut words: Vec<(usize, usize)> = Vec::new();
        let mut k = t.a;
        while k < t.b {
            if cs[k].is_whitespace() {
                k += 1;
                continue;
            }
            let a = k;
            while k < t.b && !cs[k].is_whitespace() {
                k += 1;
            }
            words.push((a, k));
        }
        let (first, last) = (words[0], words[words.len() - 1]);
        let lead = word(first.0, first.1).filter(|r| *r == Role::Letter);
        let trail = if words.len() > 1 || lead.is_none() { word(last.0, last.1) } else { None };
        let mut a = t.a;
        if let Some(role) = lead {
            cut.push(Tok { a: first.0, b: first.1, role });
            a = first.1;
        }
        let b = if trail.is_some() { last.0 } else { t.b };
        if cs[a..b].iter().any(|c| !c.is_whitespace()) {
            cut.push(Tok { a, b, role: Role::Word });
        }
        if let Some(role) = trail {
            cut.push(Tok { a: last.0, b: last.1, role });
        }
    }
    cut
}

/// The pieces of a line cut into tokens, and what each is: text, math or
/// a note marker.
pub(crate) fn classify(doc: &Doc, spans: &[Span], st: &Stats) -> (Vec<Span>, Vec<K>) {
    let mut toks: Vec<Span> = Vec::with_capacity(spans.len() * 2);
    let mut roles: Vec<Role> = Vec::with_capacity(spans.len() * 2);
    // raised numbers after a closing bracket: a power, or a footnote marker
    let mut after_close: Vec<usize> = Vec::new();
    // the tokens of each linked piece of text
    let mut linked: Vec<(usize, usize)> = Vec::new();
    for (i, s) in spans.iter().enumerate() {
        let math = doc.is_math(s) || s.tex.is_some();
        let bold = s.bold || crate::layout::is_bold_font(doc.family(s));
        let t = s.text.trim();
        if math {
            toks.push(s.clone());
            roles.push(Role::Atom);
            continue;
        }
        if s.script != Script::Normal {
            // a small raised number after a word is a footnote marker
            let prev = roles.last().copied();
            let after_word = toks.last().is_some_and(|p| p.script == Script::Normal && s.left - p.right() < 0.5 * st.body)
                && matches!(prev, Some(Role::Word | Role::Punct | Role::Close | Role::Marker));
            let stacked = spans.get(i + 1).is_some_and(|nx| nx.script == Script::Sub && (nx.left - s.left).abs() < 3.0);
            let digits = !t.is_empty() && t.len() <= 3 && t.bytes().all(|b| b.is_ascii_digit());
            let symbol = !t.is_empty() && t.chars().all(|c| "*†‡§¶".contains(c));
            let ordinal = matches!(t, "st" | "nd" | "rd" | "th");
            let marker = s.script == Script::Super && ((digits || symbol) && (after_word || i == 0) && !stacked || (ordinal && i > 0));
            // after a bracket it may be a power all the same: "(x′ − a)²"
            if marker && prev == Some(Role::Close) {
                after_close.push(toks.len());
            }
            // italic letters in a script are symbols, one each: "ikx"
            let letters = t.chars().filter(|c| c.is_alphabetic()).count();
            if !marker && s.italic && (2..=4).contains(&letters) && !t.contains(char::is_whitespace) && !is_stop(t) {
                let cs: Vec<char> = t.chars().collect();
                let w = s.width / cs.len() as f32;
                let mut k = 0;
                while k < cs.len() {
                    let mut e = k + 1;
                    while e < cs.len() && (is_prime(cs[e], true) || is_combining(cs[e])) {
                        e += 1;
                    }
                    let mut one = s.clone();
                    one.text = cs[k..e].iter().collect::<String>().replace('\'', "′").replace('"', "″");
                    one.left = s.left + w * k as f32;
                    one.width = w * (e - k) as f32;
                    toks.push(one);
                    roles.push(Role::Atom);
                    k = e;
                }
                continue;
            }
            toks.push(s.clone());
            roles.push(if marker && prev != Some(Role::Close) { Role::Marker } else { Role::Atom });
            continue;
        }
        // a linked number is a reference: "2.28" of "(2.28)", a page in the index
        let reference = s.link.is_some() && t.starts_with(|c: char| c.is_ascii_digit()) && t.chars().all(|c| c.is_ascii_digit() || c == '.');
        if s.picture.is_some() || reference || s.url.is_some() {
            toks.push(s.clone());
            roles.push(Role::Word);
            continue;
        }
        if t.is_empty() {
            continue;
        }
        // an accent in a piece of its own goes with the letter before it
        if t.chars().all(is_combining) {
            if let Some(last) = toks.last_mut().filter(|_| !matches!(roles.last(), Some(Role::Word | Role::Marker))) {
                last.text = format!("{}{t}", last.text.trim_end());
                continue;
            }
        }
        let cs: Vec<char> = s.text.chars().collect();
        if bold {
            // bold symbols are vectors ("x", "|k", "dx′"); bold words, and
            // the label of a list item ("a.", "(b)"), are not
            let runs: Vec<&str> = t.split(|c: char| !c.is_alphabetic() || is_greek(c)).filter(|w| !w.is_empty()).collect();
            let label = i == 0 && t.ends_with(['.', ')']);
            let symbols = !t.is_empty() && !label && !t.contains(|c: char| c.is_ascii_digit()) && t.chars().any(char::is_alphabetic) && runs.iter().all(|w| w.chars().count() <= 2) && (!t.contains(char::is_whitespace) || t.contains(is_operator));
            if !symbols {
                toks.push(s.clone());
                roles.push(Role::Word);
                continue;
            }
        }
        if s.italic {
            // "i.e.", "e.g."
            let abbreviation = cs.len() >= 3 && t.ends_with('.') && t.chars().filter(|c| *c == '.').count() >= 2 && t.chars().all(|c| c.is_alphabetic() || c == '.');
            if abbreviation {
                toks.push(s.clone());
                roles.push(Role::Word);
                continue;
            }
            // an italic phrase: several words, one of them long or a common word
            let runs: Vec<String> = t.split(|c: char| !c.is_alphabetic()).filter(|w| !w.is_empty()).map(str::to_string).collect();
            let phrase = t.contains(char::is_whitespace) && runs.len() >= 2 && runs.iter().any(|w| w.chars().count() >= 4 || is_stop(w));
            if phrase || t.is_empty() {
                toks.push(s.clone());
                roles.push(Role::Word);
                continue;
            }
        }
        // where each character starts on the page
        let total: f32 = cs.iter().map(|&c| char_width(c)).sum();
        let scale = if total > 0.0 { s.width / total } else { 0.0 };
        let mut x = Vec::with_capacity(cs.len() + 1);
        let mut at = s.left;
        for &c in &cs {
            x.push(at);
            at += char_width(c) * scale;
        }
        x.push(s.right());
        let first = toks.len();
        for tok in tokens(&cs, s.italic || bold) {
            let mut piece = s.clone();
            piece.text = cs[tok.a..tok.b].iter().collect();
            if (s.italic || bold) && tok.role != Role::Word {
                // primes typed as quotes
                piece.text = piece.text.replace("''", "″").replace('\'', "′").replace('"', "″");
            }
            piece.left = x[tok.a];
            piece.width = x[tok.b] - x[tok.a];
            // punctuation set in italics is punctuation all the same
            if s.italic && !matches!(tok.role, Role::Word | Role::Maybe) {
                piece.italic = tok.role == Role::Atom && piece.text.chars().any(char::is_alphabetic);
            }
            // a bar or a bracket between bold letters is not itself bold
            if bold && !piece.text.chars().any(|c| c.is_alphabetic() || "·×".contains(c)) {
                piece.bold = false;
            }
            toks.push(piece);
            // in linked text a number is the reference ("(2.28)"), not math
            let linked_number = s.link.is_some() && matches!(tok.role, Role::Number | Role::Open | Role::Close | Role::Punct);
            roles.push(if linked_number { Role::Word } else { tok.role });
        }
        if s.link.is_some() {
            linked.push((first, toks.len()));
        }
    }
    let n = toks.len();
    let gap = |a: usize, b: usize| toks[b].left - toks[a].right();
    let touch = 0.15 * st.body;
    let text = |i: usize| toks[i].text.trim();

    // a lone italic letter between italic words is a word of the phrase
    // (the article "a"), not a variable
    for i in 0..n {
        let italic_word = |k: Option<usize>| k.is_some_and(|k| k < n && toks[k].italic && roles[k] == Role::Word);
        if roles[i] == Role::Atom && toks[i].italic && toks[i].script == Script::Normal && italic_word(i.checked_sub(1)) && italic_word(Some(i + 1)) {
            roles[i] = Role::Word;
        }
    }
    // two or three italic letters: symbols ("kR") unless a common word;
    // three need math right beside them
    for i in 0..n {
        if roles[i] != Role::Maybe {
            continue;
        }
        // "is", "Is" — but "iS" is a symbol
        let w = text(i);
        let mathy = |k: usize| !matches!(roles[k], Role::Word | Role::Punct | Role::Marker | Role::Maybe);
        let beside = (i > 0 && mathy(i - 1) && (roles[i - 1] == Role::Atom || gap(i - 1, i) < touch))
            || (i + 1 < n && mathy(i + 1) && (roles[i + 1] == Role::Atom || gap(i, i + 1) < touch));
        let symbol = !is_stop(w) && (w.chars().count() == 2 || beside);
        roles[i] = if symbol { Role::Atom } else { Role::Word };
    }

    // stretches without a word
    let mut kinds = vec![K::Text; n];
    let mut lo = 0;
    while lo < n {
        if matches!(roles[lo], Role::Word | Role::Marker) {
            if roles[lo] == Role::Marker {
                kinds[lo] = K::Marker;
            }
            lo += 1;
            continue;
        }
        let mut hi = lo;
        while hi < n && !matches!(roles[hi], Role::Word | Role::Marker) {
            hi += 1;
        }
        // cut at punctuation that ends a clause: followed by a space, and
        // not inside brackets ("⟨x′, y′|" stays whole)
        let ket_closes_later = |k: usize| {
            (k + 1..hi).find(|&m| is_rangle(text(m)) || is_langle(text(m))).is_some_and(|m| is_rangle(text(m)))
        };
        let mut depth = 0i32;
        let mut start = lo;
        for k in lo..hi {
            match (roles[k], text(k)) {
                (Role::Open, _) => depth += 1,
                (Role::Close, _) => depth = (depth - 1).max(0),
                (_, t) if is_langle(t) => depth += 1,
                (_, t) if is_rangle(t) => depth = (depth - 1).max(0),
                (_, "|") => {
                    if ket_closes_later(k) {
                        depth += 1
                    } else {
                        depth = (depth - 1).max(0)
                    }
                }
                (Role::Punct, t) => {
                    let ends = k + 1 == hi || gap(k, k + 1) > touch;
                    if depth == 0 && (ends || matches!(t, ":" | "?")) {
                        segment(&toks, &mut roles, &mut kinds, start, k, touch);
                        start = k + 1;
                    }
                }
                _ => {}
            }
        }
        segment(&toks, &mut roles, &mut kinds, start, hi, touch);
        lo = hi;
    }
    if std::env::var("PDF2MD_TMDEBUG").is_ok() && (0..n).any(|i| kinds[i] == K::Text && roles[i] != Role::Word && toks[i].text.chars().any(|c| is_sure(c) || c == '′')) || std::env::var("PDF2MD_TMDEBUG").is_ok_and(|v| v == "2") && (0..n).any(|i| kinds[i] == K::Text && toks[i].italic && toks[i].text.trim().chars().count() <= 3) {
        let line: Vec<String> = (0..n)
            .map(|i| {
                let t: String = if roles[i] == Role::Word { toks[i].text.chars().map(|c| if c.is_alphabetic() { 'w' } else { c }).collect() } else { toks[i].text.clone() };
                format!("{t}‹{:?}{}{}{}›", roles[i], if kinds[i] == K::Math { "*" } else { "" }, if toks[i].italic { "i" } else { "" }, if toks[i].bold { "b" } else { "" })
            })
            .collect();
        eprintln!("TM {}", line.join(" "));
    }
    // a raised number after a bracket that is no math: a footnote marker
    for &k in &after_close {
        if k < n && kinds[k] == K::Math && (k == 0 || kinds[k - 1] != K::Math) {
            kinds[k] = K::Marker;
        }
    }
    // "… 2j" / "+ 1 …": a formula that goes on in the next line starts
    // with its operator there, and is math by that
    let sign = |k: usize| roles[k] == Role::Loose && matches!(text(k), "+" | "−" | "-");
    if n >= 2 && kinds[0] == K::Text && sign(0) && roles[1] == Role::Number && kinds[1] == K::Text {
        kinds[0] = K::Math;
        kinds[1] = K::Math;
    }
    if n >= 2 && kinds[n - 1] == K::Text && sign(n - 1) && kinds[n - 2] == K::Math {
        kinds[n - 1] = K::Math;
    }
    // a number is one piece: "1.5", not "1 . 5"
    for k in 0..n {
        if kinds[k] == K::Math && roles[k] == Role::Number && toks[k].tex.is_none() && toks[k].script == Script::Normal {
            let number = toks[k].text.trim().to_string();
            toks[k].tex = Some(number);
        }
    }
    // a script with nothing before it to stand on ("cm" and a raised −1)
    for k in 0..n {
        let s = &toks[k];
        if kinds[k] == K::Math && s.script != Script::Normal && s.tex.is_none() && (k == 0 || kinds[k - 1] != K::Math) && (k + 1 == n || kinds[k + 1] != K::Math) {
            let mut flat = crate::mathtex::MSpan::new(doc, s);
            flat.atom = None;
            let t = crate::mathtex::build(&[flat]);
            if !t.is_empty() {
                toks[k].tex = Some(format!("{{}}{}{{{t}}}", if s.script == Script::Super { '^' } else { '_' }));
            }
        }
    }
    // a linked piece of text says where it links once, after its last word
    for (a, b) in linked {
        let last = (a..b).rev().find(|&k| kinds[k] == K::Text);
        for (k, t) in toks.iter_mut().enumerate().take(b).skip(a) {
            if Some(k) != last {
                t.link = None;
                t.link_top = None;
            }
        }
    }
    // two or three italic letters that are symbols: one token each
    let mut out_t = Vec::with_capacity(n);
    let mut out_k = Vec::with_capacity(n);
    for (s, k) in toks.into_iter().zip(kinds) {
        // a prime goes with the symbol before it: a″ is "a''"
        let primes = !s.text.trim().is_empty() && s.text.trim().chars().all(|c| "′″‴".contains(c));
        if k == K::Math && primes && out_k.last() == Some(&K::Math) {
            let p: &mut Span = out_t.last_mut().expect("as many spans as kinds");
            if p.tex.is_none() && p.script == Script::Normal && s.script == Script::Normal {
                p.text = format!("{}{}", p.text.trim_end(), s.text.trim());
                p.width = s.right() - p.left;
                continue;
            }
        }
        let letters = s.text.trim().chars().count();
        if k == K::Math && (s.italic || s.bold) && s.tex.is_none() && letters > 1 && s.text.trim().chars().all(char::is_alphabetic) {
            let w = s.width / letters as f32;
            for (j, c) in s.text.trim().chars().enumerate() {
                let mut one = s.clone();
                one.text = c.to_string();
                one.left = s.left + w * j as f32;
                one.width = w;
                out_t.push(one);
                out_k.push(K::Math);
            }
        } else {
            out_t.push(s);
            out_k.push(k);
        }
    }
    (out_t, out_k)
}

/// One stretch `a..b` between clause-ending punctuation: a formula if,
/// with the prose around it taken off (brackets around the whole, an
/// unmatched bracket, punctuation, a number or letter that only stands
/// next to it), something in it is surely math.
fn segment(toks: &[Span], roles: &mut [Role], kinds: &mut [K], mut a: usize, mut b: usize, touch: f32) {
    let gap = |i: usize, j: usize| toks[j].left - toks[i].right();
    let text = |i: usize| toks[i].text.trim();
    // an operator or bracket holds what stands next to it, even across a space
    let holds = |i: usize, roles: &[Role]| {
        matches!(roles[i], Role::Open | Role::Close | Role::Loose) || (roles[i] == Role::Atom && text(i).chars().all(is_operator) && !text(i).is_empty())
    };
    loop {
        if a >= b {
            return;
        }
        // which round and square brackets have their partner
        let mut matched = vec![false; b - a];
        let mut stack: Vec<usize> = Vec::new();
        for k in a..b {
            match roles[k] {
                Role::Open => stack.push(k),
                Role::Close => {
                    if let Some(o) = stack.pop() {
                        matched[o - a] = true;
                        matched[k - a] = true;
                    }
                }
                _ => {}
            }
        }
        // a bracket without its partner, inside: prose around two formulas
        // ("l (k fixed)")
        if let Some(k) = (a + 1..b.saturating_sub(1)).find(|&k| matches!(roles[k], Role::Open | Role::Close) && !matched[k - a]) {
            segment(toks, roles, kinds, a, k, touch);
            segment(toks, roles, kinds, k + 1, b, touch);
            return;
        }
        let whole = roles[a] == Role::Open && roles[b - 1] == Role::Close && {
            // the first bracket's partner is the last token
            let mut d = 0;
            let mut partner = a;
            for (k, role) in roles.iter().enumerate().take(b).skip(a) {
                match role {
                    Role::Open => d += 1,
                    Role::Close => {
                        d -= 1;
                        if d == 0 {
                            partner = k;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            partner == b - 1
        };
        let first_goes = match roles[a] {
            Role::Punct => true,
            Role::Open | Role::Close => whole || !matched[0],
            Role::Number | Role::Letter | Role::Func | Role::Loose => a + 1 >= b || !(gap(a, a + 1) < touch || holds(a + 1, roles)) || matches!(roles[a + 1], Role::Punct),
            _ => false,
        };
        let last_goes = match roles[b - 1] {
            Role::Punct => true,
            Role::Open | Role::Close => whole || !matched[b - 1 - a],
            Role::Func => true,
            Role::Number | Role::Letter | Role::Loose => b - 1 == a || !(gap(b - 2, b - 1) < touch || holds(b - 2, roles)) || matches!(roles[b - 2], Role::Punct),
            _ => false,
        };
        if !first_goes && !last_goes {
            break;
        }
        if first_goes {
            a += 1;
        }
        if last_goes && b > a {
            b -= 1;
        }
    }
    if (a..b).any(|k| roles[k] == Role::Atom) {
        for k in kinds.iter_mut().take(b).skip(a) {
            *k = K::Math;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::blocks::{line_inlines, Stats};
    use crate::emit::render_inlines;
    use crate::layout::{char_width, Doc, Font, Line, Script, Span};

    /// A line from pieces: (text, style) with style "" (upright), "i"
    /// (italic), "b" (bold) or "^" (a small raised piece).
    fn line(pieces: &[(&str, &str)]) -> String {
        let doc = Doc {
            fonts: vec![
                Font { size: 22.0, family: "TimesNewRomanPSMT".into(), math: false },
                Font { size: 17.0, family: "TimesNewRomanPSMT".into(), math: false },
            ],
            pages: Vec::new(),
            body_size: 22.0,
        };
        let st = Stats {
            left: 100.0,
            right: 900.0,
            body: 22.0,
            spacing: 26.0,
            chapter_size: None,
            section_size: None,
            toc_pages: Default::default(),
            hyphenated: Default::default(),
        };
        let mut x = 100.0;
        let mut spans = Vec::new();
        for &(text, style) in pieces {
            let raised = style == "^";
            let size = if raised { 17.0 } else { 22.0 };
            let width: f32 = text.chars().map(char_width).sum::<f32>() * size;
            spans.push(Span {
                text: text.into(),
                left: x,
                top: if raised { 96.0 } else { 100.0 },
                width,
                height: size,
                font: raised as usize,
                bold: style == "b",
                italic: style == "i",
                script: if raised { Script::Super } else { Script::Normal },
                overline: false,
                picture: None,
                tex: None,
                link: None,
                url: None,
                url_text: None,
                link_top: None,
                image_key: None,
            });
            x += width;
        }
        let l = Line { top: 100.0, left: 100.0, right: x, size: 22.0, col: 0, bars: Vec::new(), spans };
        render_inlines(&line_inlines(&doc, &l, &st))
    }

    #[test]
    fn a_bra_ket_in_a_sentence_is_one_formula() {
        // Sakurai [p. 19]: pieces by typeface, as the PDF gives them
        assert_eq!(
            line(&[("exactly ", ""), ("N", "i"), ("2", "^"), ("  numbers  of  form ", ""), ("⟨", ""), ("a″", "i"), ("|", ""), ("X", "i"), ("|", ""), ("a′", "i"), ("⟩", ""), (",  where ", ""), ("N", "i"), ("  is  the", "")]),
            "exactly $`N^{2}`$ numbers of form $`\\langle a'' | X | a' \\rangle`$, where $`N`$ is the"
        );
    }

    #[test]
    fn words_stay_words() {
        assert_eq!(line(&[("is an ", ""), ("x", "i"), ("-polarized beam, which is ", ""), ("not", "i"), (" a", "")]), "is an $`x`$-polarized beam, which is _not_ a");
        assert_eq!(line(&[("(see part b and Figure 3) in 1925, a", "")]), "(see part b and Figure 3) in 1925, a");
        assert_eq!(line(&[("the kets (", ""), ("x", "i"), (") and ", ""), ("y", "i"), (", ", ""), ("z", "i"), (".", "")]), "the kets ($`x`$) and $`y`$, $`z`$.");
    }

    #[test]
    fn numbers_and_upright_letters_go_with_the_math_beside_them() {
        assert_eq!(line(&[("atoms with Z = 2 and ", ""), ("l", "i"), (" = 0, so", "")]), "atoms with $`Z = 2`$ and $`l = 0`$, so");
        assert_eq!(line(&[("a 2 × 2 matrix ", ""), ("f", "i"), ("(", ""), ("x", "i"), (", ", ""), ("y", "i"), (") of", "")]), "a $`2 \\times 2`$ matrix $`f ( x , y )`$ of");
        assert_eq!(line(&[("vector ", ""), ("x′", "b"), (" and ", ""), ("kR", "i"), (" ≫ 1 here", "")]), "vector $`\\boldsymbol{x'}`$ and $`k R \\gg 1`$ here");
    }
}
