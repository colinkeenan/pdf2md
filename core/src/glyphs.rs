//! Symbols the PDF's text leaves out, read from the PDF itself (with lopdf).
//!
//! A font says which glyph each character code draws by name
//! ("summationdisplay", "angbracketleft"), and usually also which Unicode
//! character that is (its ToUnicode table). Where the table is missing or
//! has gaps — a publisher's big-symbol font, the angle brackets of a symbol
//! font — pdftohtml gives an empty piece of text. The glyph's name still
//! says what it is.
//!
//! This module lists, for each page, the glyphs it shows in the order they
//! are drawn. That is the order of the glyphs in `pdftocairo -svg` too,
//! which tells where each one is (see `layout::restore_glyphs`).

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use lopdf::content::Content;
use lopdf::{Dictionary, Document, Object, ObjectId};

/// What one font tells about its character codes.
#[derive(Debug, Default, Clone)]
pub struct FontInfo {
    /// "MTEX" (of "KJKLEI+MTEX")
    pub base: String,
    /// two bytes a character (composite fonts): nothing is known of them
    pub wide: bool,
    names: HashMap<u8, String>,
    unicode: Option<HashMap<u8, String>>,
}

/// A glyph as it is to be written into the text.
#[derive(Debug, Clone, PartialEq)]
pub enum Glyph {
    /// pdftohtml has it: this text (from the font's ToUnicode table)
    Known(String),
    /// pdftohtml has left it out; by its name it is this
    Lost(&'static str),
    /// nothing is known
    Unknown,
}

impl FontInfo {
    pub fn glyph(&self, code: u8) -> Glyph {
        let named = self.names.get(&code).and_then(|n| symbol(n));
        match (&self.unicode, named) {
            (Some(u), _) if u.contains_key(&code) => Glyph::Known(u[&code].clone()),
            (Some(_), Some((text, _))) => Glyph::Lost(text),
            // Without a table pdftohtml goes by the standard glyph names; only
            // the names of TeX's sizes ("parenleftbigg") are lost on it.
            (None, Some((text, true))) => Glyph::Lost(text),
            _ => Glyph::Unknown,
        }
    }

    /// The glyph's name, if the font gives one.
    pub fn name(&self, code: u8) -> Option<&str> {
        self.names.get(&code).map(String::as_str)
    }

    /// Is the font a big-symbol font (as TeX's CMEX)? Its lost glyphs are
    /// then written as the characters pdftohtml gives for CMEX's.
    pub fn extension(&self) -> bool {
        self.names.values().any(|n| symbol(n).is_some_and(|(_, ext)| ext))
    }

    fn loses(&self) -> bool {
        self.names.keys().any(|&c| matches!(self.glyph(c), Glyph::Lost(_)))
    }
}

/// What a glyph name stands for, and whether it is a name of TeX's
/// big-symbol font. Those are written as pdftohtml writes CMEX: by the
/// letter in the glyph's slot there (Σ is "X", ∫ "Z", ⟨ "D"), which
/// `mathtex` reads.
fn symbol(name: &str) -> Option<(&'static str, bool)> {
    // wide accents ("tildewide", "hatwider")
    if name.starts_with("tildewide") {
        return Some(("˜", true));
    }
    if name.starts_with("hatwide") {
        return Some(("ˆ", true));
    }
    for size in ["display", "text", "bigg", "Bigg", "big", "Big"] {
        let Some(stem) = name.strip_suffix(size) else { continue };
        let slot = match stem {
            "summation" => "X",
            "integral" => "Z",
            "product" => "Y",
            "contintegral" => "I",
            "parenleft" => "(",
            "parenright" => ")",
            "bracketleft" => "[",
            "bracketright" => "]",
            "braceleft" => "{",
            "braceright" => "}",
            "angbracketleft" => "D",
            "angbracketright" => "E",
            "radical" => "p",
            "slash" => "/",
            _ => return None,
        };
        return Some((slot, true));
    }
    let c = match name {
        "angbracketleft" => "⟨",
        "angbracketright" => "⟩",
        "prime" => "′",
        // the arrow over a vector, and the stroke through a sign or letter
        // (≠, the Feynman slash): each is drawn over the letter it belongs to
        "vector" => "\u{20d7}",
        "negationslash" => "\u{338}",
        "greatermuch" => "≫",
        "lessmuch" => "≪",
        "similarequal" => "≃",
        "similar" => "∼",
        "approxequal" => "≈",
        "equivalence" => "≡",
        "proportional" => "∝",
        "congruent" => "≅",
        "notequal" => "≠",
        "lessequal" => "≤",
        "greaterequal" => "≥",
        "minus" => "−",
        "plusminus" => "±",
        "minusplus" => "∓",
        "multiply" => "×",
        "divide" => "÷",
        "asteriskmath" => "∗",
        "periodcentered" => "·",
        "openbullet" => "∘",
        "bullet" => "•",
        "circleplus" => "⊕",
        "circlemultiply" => "⊗",
        "arrowright" => "→",
        "arrowleft" => "←",
        "arrowboth" => "↔",
        "arrowup" => "↑",
        "arrowdown" => "↓",
        "arrowdblright" => "⇒",
        "arrowdblleft" => "⇐",
        "arrowdblboth" => "⇔",
        "infinity" => "∞",
        "nabla" | "gradient" => "∇",
        "partialdiff" => "∂",
        "radical" => "√",
        "perpendicular" => "⊥",
        "propersubset" => "⊂",
        "propersuperset" => "⊃",
        "reflexsubset" => "⊆",
        "reflexsuperset" => "⊇",
        "element" => "∈",
        "emptyset" => "∅",
        "intersection" => "∩",
        "union" => "∪",
        "logicaland" => "∧",
        "logicalor" => "∨",
        "logicalnot" => "¬",
        "universal" => "∀",
        "existential" => "∃",
        "dagger" => "†",
        "daggerdbl" => "‡",
        "aleph" => "ℵ",
        _ => return None,
    };
    Some((c, false))
}

/// The font's base name without the tag of its subset.
fn base_name(font: &Dictionary) -> String {
    let raw = font.get(b"BaseFont").ok().and_then(|b| b.as_name().ok()).map(|b| String::from_utf8_lossy(b).to_string()).unwrap_or_default();
    raw.split_once('+').map_or(raw.as_str(), |(_, f)| f).to_string()
}

fn font_info(doc: &Document, font: &Dictionary) -> FontInfo {
    let name_is = |key: &[u8], value: &[u8]| font.get(key).ok().and_then(|s| s.as_name().ok()) == Some(value);
    let mut info = FontInfo { base: base_name(font), wide: name_is(b"Subtype", b"Type0"), ..Default::default() };
    if info.wide {
        return info;
    }
    // /Encoding << /Differences [2 /summationdisplay /summationtext 26 /circumflex …] >>
    let differences = font
        .get(b"Encoding")
        .ok()
        .and_then(|e| doc.dereference(e).ok())
        .and_then(|(_, e)| e.as_dict().ok())
        .and_then(|e| e.get(b"Differences").ok())
        .and_then(|d| doc.dereference(d).ok())
        .and_then(|(_, d)| d.as_array().ok());
    if let Some(differences) = differences {
        let mut code = 0i64;
        for item in differences {
            match item {
                Object::Integer(n) => code = *n,
                Object::Name(n) => {
                    if let Ok(c) = u8::try_from(code) {
                        info.names.insert(c, String::from_utf8_lossy(n).to_string());
                    }
                    code += 1;
                }
                _ => {}
            }
        }
    }
    info.unicode = font
        .get(b"ToUnicode")
        .ok()
        .and_then(|t| doc.dereference(t).ok())
        .and_then(|(_, t)| t.as_stream().ok())
        .map(|s| to_unicode(&s.decompressed_content().unwrap_or_else(|_| s.content.clone())));
    info
}

/// A ToUnicode table of one-byte codes:
/// `<02> <2192>` between beginbfchar and endbfchar,
/// `<7b> <7d> <007B>` or `<7b> <7d> [<007B> <007C> <007D>]` between
/// beginbfrange and endbfrange.
fn to_unicode(cmap: &[u8]) -> HashMap<u8, String> {
    #[derive(Debug, PartialEq)]
    enum Tok {
        Hex(Vec<u8>),
        Open,
        Close,
        Word(String),
    }
    let text = String::from_utf8_lossy(cmap);
    let mut toks = Vec::new();
    let mut rest = text.as_ref();
    while let Some(c) = rest.chars().next() {
        if c == '<' && !rest.starts_with("<<") {
            let end = rest.find('>').unwrap_or(rest.len());
            let digits: Vec<u8> = rest[1..end].bytes().filter(u8::is_ascii_hexdigit).collect();
            let bytes = digits.chunks(2).filter_map(|p| u8::from_str_radix(std::str::from_utf8(p).ok()?, 16).ok()).collect();
            toks.push(Tok::Hex(bytes));
            rest = &rest[(end + 1).min(rest.len())..];
        } else if c == '[' || c == ']' {
            toks.push(if c == '[' { Tok::Open } else { Tok::Close });
            rest = &rest[1..];
        } else if c.is_ascii_alphabetic() {
            let end = rest.find(|x: char| !x.is_ascii_alphanumeric()).unwrap_or(rest.len());
            toks.push(Tok::Word(rest[..end].to_string()));
            rest = &rest[end..];
        } else {
            rest = &rest[c.len_utf8()..];
        }
    }
    let utf16 = |b: &[u8]| -> String {
        let units: Vec<u16> = b.chunks(2).filter(|p| p.len() == 2).map(|p| u16::from_be_bytes([p[0], p[1]])).collect();
        String::from_utf16_lossy(&units)
    };
    let mut map = HashMap::new();
    let mut mode = "";
    let mut i = 0;
    while i < toks.len() {
        match &toks[i] {
            Tok::Word(w) if w == "beginbfchar" => mode = "char",
            Tok::Word(w) if w == "beginbfrange" => mode = "range",
            Tok::Word(w) if w == "endbfchar" || w == "endbfrange" => mode = "",
            Tok::Hex(src) if mode == "char" => {
                if let (Some(Tok::Hex(dst)), [code]) = (toks.get(i + 1), src.as_slice()) {
                    map.insert(*code, utf16(dst));
                }
                i += 1;
            }
            Tok::Hex(lo) if mode == "range" => {
                let (Some(Tok::Hex(hi)), [lo]) = (toks.get(i + 1), lo.as_slice()) else {
                    i += 1;
                    continue;
                };
                let hi = hi.last().copied().unwrap_or(*lo);
                match toks.get(i + 2) {
                    Some(Tok::Hex(dst)) => {
                        let units: Vec<u16> = dst.chunks(2).filter(|p| p.len() == 2).map(|p| u16::from_be_bytes([p[0], p[1]])).collect();
                        for code in *lo..=hi {
                            let mut u = units.clone();
                            if let Some(last) = u.last_mut() {
                                *last = last.wrapping_add((code - lo) as u16);
                            }
                            map.insert(code, String::from_utf16_lossy(&u));
                        }
                        i += 2;
                    }
                    Some(Tok::Open) => {
                        let mut code = *lo;
                        i += 3;
                        while let Some(Tok::Hex(dst)) = toks.get(i) {
                            map.insert(code, utf16(dst));
                            code = code.wrapping_add(1);
                            i += 1;
                        }
                    }
                    _ => i += 1,
                }
            }
            _ => {}
        }
        i += 1;
    }
    // (a code "mapped" to nothing or to a blank is not known)
    map.retain(|_, t| !t.trim().is_empty() && !t.chars().all(|c| c == '\u{fffd}' || c.is_control()));
    map
}

/// The fonts of the PDF that pdftohtml loses glyphs of: base name, and
/// whether it is a big-symbol font. Empty for most books.
pub fn lost_fonts(path: &Path) -> HashMap<String, bool> {
    // (asked for when the text is read and again when the drawings are)
    static LAST: Mutex<Option<(PathBuf, HashMap<String, bool>)>> = Mutex::new(None);
    let mut last = LAST.lock().unwrap_or_else(|e| e.into_inner());
    if let Some((_, fonts)) = last.as_ref().filter(|(p, _)| p == path) {
        return fonts.clone();
    }
    let fonts = find_lost_fonts(path);
    *last = Some((path.to_path_buf(), fonts.clone()));
    fonts
}

fn find_lost_fonts(path: &Path) -> HashMap<String, bool> {
    let mut out = HashMap::new();
    let Ok(doc) = Document::load(path) else { return out };
    for object in doc.objects.values() {
        let Ok(font) = object.as_dict() else { continue };
        if font.get(b"Type").ok().and_then(|t| t.as_name().ok()) != Some(b"Font".as_slice()) {
            continue;
        }
        let info = font_info(&doc, font);
        // TeX's own fonts are read by their slots (see `mathtex`)
        if info.loses() && !tex_font(&info.base) {
            *out.entry(info.base.clone()).or_insert(false) |= info.extension();
        }
    }
    out
}

fn tex_font(base: &str) -> bool {
    let u = base.to_uppercase();
    ["CM", "MSAM", "MSBM", "EUFM", "EUEX", "RSFS"].iter().any(|p| u.starts_with(p))
}

/// The glyphs shown on the pages, in the order they are drawn.
pub struct Glyphs {
    pub fonts: Vec<FontInfo>,
    /// PDF page number -> (font, character code) of every glyph
    pub pages: HashMap<u32, Vec<(usize, u8)>>,
}

/// The glyphs of the pages `first..=last` that show a glyph pdftohtml
/// loses. None if the PDF has no such font (or cannot be read this way).
pub fn read(path: &Path, first: u32, last: u32) -> Option<Glyphs> {
    let doc = Document::load(path).ok()?;
    let mut out = Glyphs { fonts: Vec::new(), pages: HashMap::new() };
    let mut known: HashMap<ObjectId, usize> = HashMap::new();
    for (number, page) in doc.get_pages() {
        if number < first || number > last {
            continue;
        }
        let (own, inherited) = doc.get_page_resources(page).unwrap_or((None, Vec::new()));
        let mut resources: Vec<&Dictionary> = own.into_iter().collect();
        resources.extend(inherited.iter().filter_map(|id| doc.get_dictionary(*id).ok()));
        let content = doc.get_page_content(page);
        let mut shown = Vec::new();
        show(&doc, &content, &resources, &mut out.fonts, &mut known, &mut shown, 0);
        let loses = shown.iter().any(|&(f, c)| !tex_font(&out.fonts[f].base) && matches!(out.fonts[f].glyph(c), Glyph::Lost(_)));
        if loses {
            out.pages.insert(number, shown);
        }
    }
    (!out.pages.is_empty()).then_some(out)
}

/// An entry of a resource dictionary's sub-dictionary: /Font /T1_7.
fn resource<'a>(doc: &'a Document, resources: &[&'a Dictionary], kind: &[u8], name: &[u8]) -> Option<(Option<ObjectId>, &'a Object)> {
    resources.iter().find_map(|r| {
        let of_kind = doc.dereference(r.get(kind).ok()?).ok()?.1.as_dict().ok()?;
        doc.dereference(of_kind.get(name).ok()?).ok()
    })
}

/// Walk a content stream and note every glyph it shows.
fn show(
    doc: &Document,
    content: &[u8],
    resources: &[&Dictionary],
    fonts: &mut Vec<FontInfo>,
    known: &mut HashMap<ObjectId, usize>,
    shown: &mut Vec<(usize, u8)>,
    depth: usize,
) {
    let Ok(content) = Content::decode(content) else { return };
    let mut font: Option<usize> = None;
    let mut saved: Vec<Option<usize>> = Vec::new();
    for op in &content.operations {
        match op.operator.as_str() {
            "q" => saved.push(font),
            "Q" => font = saved.pop().unwrap_or(font),
            "Tf" => {
                let Some(name) = op.operands.first().and_then(|n| n.as_name().ok()) else { continue };
                font = resource(doc, resources, b"Font", name).and_then(|(id, object)| {
                    if let Some(&i) = id.and_then(|id| known.get(&id)) {
                        return Some(i);
                    }
                    fonts.push(font_info(doc, object.as_dict().ok()?));
                    if let Some(id) = id {
                        known.insert(id, fonts.len() - 1);
                    }
                    Some(fonts.len() - 1)
                });
            }
            "Tj" | "TJ" | "'" | "\"" => {
                let Some(f) = font else { continue };
                let mut bytes: Vec<u8> = Vec::new();
                for operand in &op.operands {
                    match operand {
                        Object::String(s, _) => bytes.extend(s),
                        Object::Array(items) => {
                            for item in items {
                                if let Object::String(s, _) = item {
                                    bytes.extend(s);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                if fonts[f].wide {
                    shown.extend(bytes.chunks(2).map(|_| (f, 0)));
                } else {
                    shown.extend(bytes.into_iter().map(|b| (f, b)));
                }
            }
            // a drawing kept apart (a form) has text of its own
            "Do" if depth < 4 => {
                let Some(name) = op.operands.first().and_then(|n| n.as_name().ok()) else { continue };
                let Some(form) = resource(doc, resources, b"XObject", name).and_then(|(_, o)| o.as_stream().ok()) else { continue };
                if form.dict.get(b"Subtype").ok().and_then(|s| s.as_name().ok()) != Some(b"Form".as_slice()) {
                    continue;
                }
                let mut inner: Vec<&Dictionary> = form.dict.get(b"Resources").ok().and_then(|r| doc.dereference(r).ok()).and_then(|(_, r)| r.as_dict().ok()).into_iter().collect();
                inner.extend(resources.iter().copied());
                let data = form.decompressed_content().unwrap_or_else(|_| form.content.clone());
                show(doc, &data, &inner, fonts, known, shown, depth + 1);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unicode_table() {
        let map = to_unicode(b"1 begincodespacerange <02> <b1> endcodespacerange 2 beginbfchar <02> <2192> <3d> <003D> endbfchar 2 beginbfrange <7b> <7d> <007B> <80> <81> [<2212> <00B1>] endbfrange");
        assert_eq!(map[&0x02], "→");
        assert_eq!(map[&0x3d], "=");
        assert_eq!(map[&0x7c], "|");
        assert_eq!(map[&0x81], "±");
        assert_eq!(map.len(), 7);
    }

    #[test]
    fn names() {
        assert_eq!(symbol("summationdisplay"), Some(("X", true)));
        assert_eq!(symbol("parenleftBigg"), Some(("(", true)));
        assert_eq!(symbol("angbracketleft"), Some(("⟨", false)));
        assert_eq!(symbol("parenlefttp"), None);
        let mut font = FontInfo { base: "MTSYN".into(), ..Default::default() };
        font.names.insert(3, "angbracketright".into());
        font.names.insert(61, "equal".into());
        font.unicode = Some(HashMap::from([(61, "=".to_string())]));
        assert_eq!(font.glyph(3), Glyph::Lost("⟩"));
        assert_eq!(font.glyph(61), Glyph::Known("=".into()));
        // without a table pdftohtml knows the standard names itself
        font.unicode = None;
        assert_eq!(font.glyph(3), Glyph::Unknown);
    }
}
