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
    /// TeX's big-symbol font with a table that names its signs otherwise
    /// than pdftohtml gives them without one (see [`FontInfo::slot_text`])
    misnamed: bool,
    /// the codes its table gives as a blank
    blanks: Vec<u8>,
}

/// The glyphs of TeX's big-symbol font (CMEX), by their places in it.
const CMEX_SLOTS: [&str; 128] = [
    "parenleftbig", "parenrightbig", "bracketleftbig", "bracketrightbig", "floorleftbig", "floorrightbig", "ceilingleftbig", "ceilingrightbig",
    "braceleftbig", "bracerightbig", "angbracketleftbig", "angbracketrightbig", "vextendsingle", "vextenddouble", "slashbig", "backslashbig",
    "parenleftBig", "parenrightBig", "parenleftbigg", "parenrightbigg", "bracketleftbigg", "bracketrightbigg", "floorleftbigg", "floorrightbigg",
    "ceilingleftbigg", "ceilingrightbigg", "braceleftbigg", "bracerightbigg", "angbracketleftbigg", "angbracketrightbigg", "slashbigg", "backslashbigg",
    "parenleftBigg", "parenrightBigg", "bracketleftBigg", "bracketrightBigg", "floorleftBigg", "floorrightBigg", "ceilingleftBigg", "ceilingrightBigg",
    "braceleftBigg", "bracerightBigg", "angbracketleftBigg", "angbracketrightBigg", "slashBigg", "backslashBigg", "slashBig", "backslashBig",
    "parenlefttp", "parenrighttp", "bracketlefttp", "bracketrighttp", "bracketleftbt", "bracketrightbt", "bracketleftex", "bracketrightex",
    "bracelefttp", "bracerighttp", "braceleftbt", "bracerightbt", "braceleftmid", "bracerightmid", "braceex", "arrowvertex",
    "parenleftbt", "parenrightbt", "parenleftex", "parenrightex", "angbracketleftBig", "angbracketrightBig", "unionsqtext", "unionsqdisplay",
    "contintegraltext", "contintegraldisplay", "circledottext", "circledotdisplay", "circleplustext", "circleplusdisplay", "circlemultiplytext", "circlemultiplydisplay",
    "summationtext", "producttext", "integraltext", "uniontext", "intersectiontext", "unionmultitext", "logicalandtext", "logicalortext",
    "summationdisplay", "productdisplay", "integraldisplay", "uniondisplay", "intersectiondisplay", "unionmultidisplay", "logicalanddisplay", "logicalordisplay",
    "coproducttext", "coproductdisplay", "hatwide", "hatwider", "hatwidest", "tildewide", "tildewider", "tildewidest",
    "bracketleftBig", "bracketrightBig", "floorleftBig", "floorrightBig", "ceilingleftBig", "ceilingrightBig", "braceleftBig", "bracerightBig",
    "radicalbig", "radicalBig", "radicalbigg", "radicalBigg", "radicalbt", "radicalvertex", "radicaltp", "arrowvertexdbl",
    "arrowtp", "arrowbt", "bracehtipdownleft", "bracehtipdownright", "bracehtipupleft", "bracehtipupright", "arrowdbltp", "arrowdblbt",
];

/// The Greek letter at a place of TeX's font of math letters (CMMI), as
/// pdftohtml gives it, and the other character a table may rightly give
/// for it (the two shapes of ϵ and φ go by either name).
fn greek_slot(slot: u8) -> Option<(&'static str, &'static str)> {
    const LETTERS: [&str; 40] = [
        "Γ", "Δ", "Θ", "Λ", "Ξ", "Π", "Σ", "Υ", "Φ", "Ψ", "Ω", "α", "β", "γ", "δ", "ϵ", "ζ", "η", "θ", "ι", "κ", "λ", "µ", "ν", "ξ", "π", "ρ", "σ", "τ", "υ", "φ", "χ", "ψ", "ω", "ε", "ϑ", "ϖ", "ϱ", "ς", "ϕ",
    ];
    let letter = *LETTERS.get(slot as usize)?;
    let also = match slot {
        1 => "∆",
        6 => "∑",
        5 => "∏",
        10 => "Ω",
        15 => "ε",
        22 => "μ",
        30 => "ϕ",
        34 => "ϵ",
        39 => "φ",
        _ => letter,
    };
    Some((letter, also))
}

/// The signs of TeX's symbol font (CMSY), by their places in it, as the
/// characters a font's table gives them by ("" where `mathtex` has its
/// own reading of the place: the stroke of ≠, the foot of ↦).
const CMSY_SLOTS: [&str; 128] = [
    "−", "·", "×", "∗", "÷", "⋄", "±", "∓", "⊕", "⊖", "⊗", "⊘", "⊙", "◯", "∘", "•", "≍", "≡", "⊆", "⊇", "≤", "≥", "⪯", "⪰", "∼", "≈", "⊂", "⊃", "≪", "≫", "≺", "≻",
    "←", "→", "↑", "↓", "↔", "↗", "↘", "≃", "⇐", "⇒", "⇑", "⇓", "⇔", "↖", "↙", "∝", "′", "∞", "∈", "∋", "△", "▽", "", "", "∀", "∃", "¬", "∅", "ℜ", "ℑ", "⊤", "⊥",
    "ℵ", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "∪", "∩", "⊎", "∧", "∨",
    "⊢", "⊣", "⌊", "⌋", "⌈", "⌉", "{", "}", "⟨", "⟩", "|", "‖", "↕", "⇕", "\\", "≀", "√", "⨿", "∇", "∫", "⊔", "⊓", "⊑", "⊒", "§", "†", "‡", "¶", "♣", "♢", "♡", "♠",
];

/// A place of CMEX that holds a sign in one of its sizes (a sum, an
/// integral, a bracket, a root; not a piece that a tall bracket is built
/// of): pdftohtml gives it, where the font has no table, as the letter of
/// that place, and `mathtex` reads those letters.
fn sized_slot(slot: u8) -> bool {
    CMEX_SLOTS.get(slot as usize).is_some_and(|n| symbol(n).is_some_and(|(_, ext)| ext))
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
            // ... and names that are not standard ones, where the glyph
            // has a code pdftohtml gives no character for (a font cut down
            // to the glyphs used numbers them from 2: ℏ in the AMS font)
            (None, Some((text, false))) if code < 32 && NOT_STANDARD.contains(&self.names[&code].as_str()) => Glyph::Lost(text),
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

    /// Is this the ϵ of TeX's math italic font in a PDF that does not say
    /// what its letters mean? pdftohtml has no character for its name
    /// ("epsilon1"; without names it is the font's own place 15).
    pub fn unnamed_epsilon(&self, code: u8) -> bool {
        self.unicode.is_none() && !self.wide && self.base.to_uppercase().starts_with("CMMI") && self.names.get(&code).map_or(code == 15, |n| n == "epsilon1")
    }

    /// The capital with doubled strokes that a glyph of a nameless font
    /// is. TeX's blackboard letters come from a font that some PDFs hold
    /// only as drawings, dot by dot, without a name or a table of
    /// characters; its glyphs are named by their codes there ("x52" for
    /// the R of the reals), and `pdftohtml` gives each as "\u{fffd}".
    fn doubled(&self, code: u8) -> Option<char> {
        const LETTERS: [char; 26] = ['𝔸', '𝔹', 'ℂ', '𝔻', '𝔼', '𝔽', '𝔾', 'ℍ', '𝕀', '𝕁', '𝕂', '𝕃', '𝕄', 'ℕ', '𝕆', 'ℙ', 'ℚ', 'ℝ', '𝕊', '𝕋', '𝕌', '𝕍', '𝕎', '𝕏', '𝕐', 'ℤ'];
        let name = self.names.get(&code)?;
        // (… or it holds one glyph only, at the place of the figure 1:
        // the 1 with a doubled stem, the identity)
        if self.names.len() == 1 && code == b'1' {
            return Some('𝟙');
        }
        let letter = u8::from_str_radix(name.strip_prefix('x')?, 16).ok().filter(|_| name.len() == 3)?;
        letter.is_ascii_uppercase().then(|| LETTERS[(letter - b'A') as usize])
    }

    /// Is the font named as PDF fonts are, or (`family` being what
    /// pdftohtml calls it) a nameless one, which goes by any name there?
    pub fn nameless(&self) -> bool {
        self.base.is_empty()
    }

    /// A glyph that pdftohtml gives as a blank, or not at all: the
    /// font's table says so, or its name.
    pub fn blank(&self, code: u8) -> bool {
        self.blanks.contains(&code) || self.name(code) == Some("space")
    }

    /// Is this TeX's big-symbol font with a table that gives its signs
    /// by other characters than their places' letters ("∑" for the sum
    /// sign that `mathtex` knows as "X", or no character at all)?
    pub fn misnamed(&self) -> bool {
        self.misnamed
    }

    /// The place in CMEX of the glyph this font has at `code`: by its
    /// name where the font names its glyphs (one cut down to those used
    /// numbers them anew), else the code itself. (Of the font of math
    /// letters only where it does not name them.)
    fn slot(&self, code: u8) -> Option<u8> {
        match self.names.get(&code) {
            Some(_) if !self.base.to_uppercase().starts_with("CMEX") => None,
            Some(name) => CMEX_SLOTS.iter().position(|n| n == name).map(|k| k as u8),
            None => (code < 128).then_some(code),
        }
    }

    /// What pdftohtml would give for the glyph of such a font without its
    /// table, which is what `mathtex` reads: the letter of a sized sign's
    /// place ("X" for the sum of a display, "P" for the one of a line),
    /// nothing for a place below the letters (the smaller brackets, which
    /// are then found by their outlines) unless the table names it. A
    /// piece of a tall bracket stays what the table says.
    /// In the font of math letters: the Greek letter of the glyph's
    /// place, where the table gives another letter or none. None for a
    /// glyph that stays as pdftohtml gave it.
    pub fn slot_text(&self, code: u8) -> Option<String> {
        let known = self.unicode.as_ref().and_then(|u| u.get(&code)).cloned().unwrap_or_default();
        // (a font drawn dot by dot, held without a name: see `doubled`)
        if self.base.is_empty() {
            return self.doubled(code).filter(|_| known.trim().is_empty()).map(|c| c.to_string());
        }
        // (the code 32 that the table gives as a blank is a blank that the
        // maker of the PDF set in this font, not the ψ or the arrow of
        // that place)
        if code == b' ' && self.blanks.contains(&code) && !self.base.to_uppercase().starts_with("CMEX") {
            return None;
        }
        // (a glyph the font names, in a font cut down and numbered anew:
        // by its name, where the table has no character for it)
        if let Some(name) = self.names.get(&code).filter(|_| !self.base.to_uppercase().starts_with("CMEX")) {
            return if known.trim().is_empty() { symbol(name).filter(|(_, big)| !big).map(|(t, _)| t.to_string()) } else { None };
        }
        if self.letters() {
            let slot = self.slot(code)?;
            let Some((letter, also)) = greek_slot(slot) else {
                // (… and a sign of that font the table has no character for;
                // the arrow over a vector as `mathtex` reads it)
                let sign = match slot {
                    64 => "∂",
                    96 => "ℓ",
                    123 => "ı",
                    124 => "ȷ",
                    125 => "℘",
                    126 => "~",
                    _ => return None,
                };
                return known.trim().is_empty().then(|| sign.to_string());
            };
            return (known.trim() != letter && known.trim() != also).then(|| letter.to_string());
        }
        // (the font of signs and the upright one: only where the table
        // has no character for the glyph)
        let upper = self.base.to_uppercase();
        if upper.starts_with("CMSY") {
            let sign = *CMSY_SLOTS.get(self.slot(code)? as usize)?;
            return (known.trim().is_empty() && !sign.is_empty()).then(|| sign.to_string());
        }
        if upper.starts_with("CMR") || upper.starts_with("CMBX") {
            let (capital, _) = greek_slot(self.slot(code).filter(|slot| *slot <= 10)?)?;
            return known.trim().is_empty().then(|| capital.to_string());
        }
        // (the first signs of the AMS symbol font: the centred dot of p·y)
        if upper.starts_with("MSAM") {
            let sign = *["⊡", "⊞", "⊠", "□", "■", "·", "◊", "⧫"].get(self.slot(code)? as usize)?;
            return known.trim().is_empty().then(|| sign.to_string());
        }
        // (any other font of TeX's: as pdftohtml gave it)
        if !upper.starts_with("CMEX") {
            return None;
        }
        match self.slot(code) {
            Some(slot) if sized_slot(slot) && slot > 32 => Some(char::from(slot).to_string()),
            _ => Some(known),
        }
    }

    /// TeX's font of math letters (CMMI)?
    fn letters(&self) -> bool {
        self.base.to_uppercase().starts_with("CMMI")
    }

    fn loses(&self) -> bool {
        self.names.keys().any(|&c| matches!(self.glyph(c), Glyph::Lost(_)))
    }
}

/// Glyph names pdftohtml does not know.
const NOT_STANDARD: [&str; 3] = ["planckover2pi", "planckover2pi1", "hslash"];

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
        "bardbl" => "‖",
        "planckover2pi" | "planckover2pi1" | "hslash" => "ℏ",
        "angleleft" => "⟨",
        "angleright" => "⟩",
        "lscript" | "ell" => "ℓ",
        "dotlessi" => "ı",
        "dotlessj" => "ȷ",
        // Greek, as the fonts name it ("epsilon1" is the other form of the
        // letter, "Gamma1" a sloping capital)
        "alpha" => "α",
        "beta" => "β",
        "gamma" => "γ",
        "delta" => "δ",
        "epsilon" => "ε",
        "epsilon1" => "ϵ",
        "zeta" => "ζ",
        "eta" => "η",
        "theta" => "θ",
        "theta1" => "ϑ",
        "iota" => "ι",
        "kappa" => "κ",
        "lambda" => "λ",
        "mu" | "mu1" => "μ",
        "nu" => "ν",
        "xi" => "ξ",
        "pi" => "π",
        "pi1" | "omega1" => "ϖ",
        "rho" => "ρ",
        "rho1" => "ϱ",
        "sigma" => "σ",
        "sigma1" => "ς",
        "tau" => "τ",
        "upsilon" => "υ",
        "phi" => "φ",
        "phi1" => "ϕ",
        "chi" => "χ",
        "psi" => "ψ",
        "omega" => "ω",
        "Gamma" | "Gamma1" => "Γ",
        "Delta" | "Delta1" => "Δ",
        "Theta" | "Theta1" => "Θ",
        "Lambda" | "Lambda1" => "Λ",
        "Xi" | "Xi1" => "Ξ",
        "Pi" | "Pi1" => "Π",
        "Sigma" | "Sigma1" => "Σ",
        "Upsilon" | "Upsilon1" => "Υ",
        "Phi" | "Phi1" => "Φ",
        "Psi" | "Psi1" => "Ψ",
        "Omega" | "Omega1" | "Omegagreek" => "Ω",
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
    let table = font
        .get(b"ToUnicode")
        .ok()
        .and_then(|t| doc.dereference(t).ok())
        .and_then(|(_, t)| t.as_stream().ok())
        .map(|s| to_unicode_all(&s.decompressed_content().unwrap_or_else(|_| s.content.clone())));
    // (TeX's big-symbol font, a sized sign of which the table gives by
    // another character than its place's letter, or by none)
    if let Some(table) = table.as_ref().filter(|_| info.base.to_uppercase().starts_with("CMEX")) {
        info.misnamed = table.iter().any(|(&code, text)| info.slot(code).is_some_and(|slot| sized_slot(slot) && slot > 32 && text.trim() != char::from(slot).to_string()));
    }
    // (… its font of math letters, with a table that gives a Greek letter
    // by another one: ν as "β" in the small type of one lecture's notes,
    // where the large type has it right)
    if let Some(table) = table.as_ref().filter(|_| info.letters()) {
        info.misnamed = table.iter().any(|(&code, text)| {
            let t = text.trim();
            let greek = t.chars().count() == 1 && t.chars().all(|c| ('\u{370}'..='\u{3ff}').contains(&c) || c == 'µ');
            greek && info.slot(code).and_then(greek_slot).is_some_and(|(letter, also)| t != letter && t != also)
        });
    }
    if let Some(table) = &table {
        info.blanks = table.iter().filter(|(_, t)| !t.is_empty() && t.chars().all(char::is_whitespace)).map(|(&code, _)| code).collect();
    }
    let gaps: Vec<u8> = table.iter().flatten().filter(|(_, t)| t.trim().is_empty() || t.chars().all(|c| c == '\u{fffd}' || c.is_control())).map(|(&code, _)| code).collect();
    info.unicode = table.map(known_only);
    // (… any font of TeX's whose table has no character for a glyph that
    // its place names: the prime of the symbol font, a Greek capital of
    // the upright one, ∂)
    // (the nameless font of blackboard capitals)
    if info.base.is_empty() && !info.wide {
        info.misnamed = info.names.keys().any(|&code| info.slot_text(code).is_some());
    }
    if !info.misnamed && (tex_font(&info.base) || info.base.to_uppercase().starts_with("MSAM")) {
        info.misnamed = gaps.iter().any(|&code| !info.blanks.contains(&code) && info.slot_text(code).is_some_and(|t| !t.trim().is_empty()));
    }
    info
}

/// A ToUnicode table of one-byte codes:
/// `<02> <2192>` between beginbfchar and endbfchar,
/// `<7b> <7d> <007B>` or `<7b> <7d> [<007B> <007C> <007D>]` between
/// beginbfrange and endbfrange.
/// (a code "mapped" to nothing or to a blank is not known)
fn known_only(mut map: HashMap<u8, String>) -> HashMap<u8, String> {
    map.retain(|_, t| !t.trim().is_empty() && !t.chars().all(|c| c == '\u{fffd}' || c.is_control()));
    map
}

/// … with the codes it maps to nothing.
fn to_unicode_all(cmap: &[u8]) -> HashMap<u8, String> {
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
    ["CM", "EUFM", "EUEX", "RSFS"].iter().any(|p| u.starts_with(p))
}

/// The glyphs shown on the pages, in the order they are drawn.
pub struct Glyphs {
    pub fonts: Vec<FontInfo>,
    /// PDF page number -> (font, character code) of every glyph
    pub pages: HashMap<u32, Vec<(usize, u8)>>,
}

impl Glyphs {
    /// The glyphs of a page that its drawing (the SVG) has; `fonts` names
    /// the drawing's font of each glyph there. A space is a glyph that
    /// draws nothing, and may not be in the drawing: all of a page's, or
    /// one here and there. The drawing's fonts tell which: each of them
    /// is one font of the PDF (at one size), so a space is not drawn
    /// where the next glyph of the drawing is of another font than its
    /// own. None where the two lists cannot be matched so.
    pub fn drawn(&self, shown: &[(usize, u8)], fonts: &[u32]) -> Option<Vec<(usize, u8)>> {
        if shown.len() == fonts.len() {
            return Some(shown.to_vec());
        }
        // (in TeX's math fonts the code 32 is a sign: a bracket of CMEX)
        let space = |f: usize, code: u8| self.fonts[f].name(code) == Some("space") || (code == b' ' && self.fonts[f].name(code).is_none() && !is_math_base(&self.fonts[f].base));
        // Which font of the PDF a font of the drawing is, is learned where
        // it first comes. At a space that is a guess (the space may not be
        // drawn, and the glyph drawn there the next letter's): where the
        // lists then cease to match, the guess is taken back.
        let mut of: HashMap<u32, usize> = HashMap::new();
        let mut out: Vec<(usize, u8)> = Vec::with_capacity(fonts.len());
        let mut guesses: Vec<(usize, usize, HashMap<u32, usize>)> = Vec::new();
        let (mut i, mut steps) = (0, 0);
        let matched = loop {
            steps += 1;
            if steps > 50 * shown.len().max(1) {
                break false;
            }
            let fits = if i < shown.len() {
                let (f, code) = shown[i];
                match fonts.get(out.len()).map(|font| (font, of.get(font).copied())) {
                    Some((_, Some(m))) if m == f => true,
                    Some((font, None)) => {
                        if space(f, code) {
                            guesses.push((i, out.len(), of.clone()));
                        }
                        of.insert(*font, f);
                        true
                    }
                    _ => false,
                }
            } else {
                false
            };
            if fits {
                out.push(shown[i]);
                i += 1;
            } else if i < shown.len() && space(shown[i].0, shown[i].1) {
                i += 1;
            } else if i == shown.len() && out.len() == fonts.len() {
                break true;
            } else {
                // (the last space taken for drawn was not)
                let Some((at, drawn, known)) = guesses.pop() else { break false };
                out.truncate(drawn);
                of = known;
                i = at + 1;
            }
        };
        if matched {
            return Some(out);
        }
        // (none of the page's spaces drawn)
        let plain: Vec<(usize, u8)> = shown.iter().copied().filter(|&(_, code)| code != b' ').collect();
        (plain.len() == fonts.len()).then_some(plain)
    }
}

fn is_math_base(base: &str) -> bool {
    let u = base.to_uppercase();
    ["CMEX", "CMSY", "CMMI", "MSAM", "MSBM"].iter().any(|p| u.starts_with(p))
}

/// The glyphs of the pages `first..=last` that show a glyph pdftohtml
/// loses. None if the PDF has no such font (or cannot be read this way).
pub fn read(path: &Path, first: u32, last: u32) -> Option<Glyphs> {
    read_where(path, first, last, &|font, code| !tex_font(&font.base) && matches!(font.glyph(code), Glyph::Lost(_)))
}

/// The glyphs of the pages `first..=last` that show a glyph `wanted`.
pub fn read_where(path: &Path, first: u32, last: u32, wanted: &dyn Fn(&FontInfo, u8) -> bool) -> Option<Glyphs> {
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
        if shown.iter().any(|&(f, c)| wanted(&out.fonts[f], c)) {
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
    fn a_space_that_is_not_drawn_is_told_by_the_drawings_fonts() {
        let font = |base: &str| FontInfo { base: base.into(), ..Default::default() };
        let g = Glyphs { fonts: vec![font("CMR10"), font("Times-Roman")], pages: HashMap::new() };
        // "a b c" with its spaces in another font; the drawing has the first space only
        let shown = [(0, b'a'), (1, b' '), (0, b'b'), (1, b' '), (0, b'c')];
        assert_eq!(g.drawn(&shown, &[7, 3, 7, 7]), Some(vec![(0, b'a'), (1, b' '), (0, b'b'), (0, b'c')]));
        // (all drawn; none drawn)
        assert_eq!(g.drawn(&shown, &[7, 3, 7, 3, 7]).map(|d| d.len()), Some(5));
        assert_eq!(g.drawn(&shown, &[7, 7, 7]), Some(vec![(0, b'a'), (0, b'b'), (0, b'c')]));
        // (a letter that is not drawn: no match)
        assert_eq!(g.drawn(&shown, &[7, 3, 3, 7]), None);
        // a space that is not drawn, where a font of the drawing first comes: "a b" then a letter of a third font
        let g = Glyphs { fonts: vec![font("CMR10"), font("Times-Roman"), font("CMSY7")], pages: HashMap::new() };
        let shown = [(0, b'a'), (1, b' '), (2, b'1'), (0, b'b'), (2, b'1'), (1, b' '), (0, b'c')];
        assert_eq!(g.drawn(&shown, &[7, 9, 7, 9, 3, 7]), Some(vec![(0, b'a'), (2, b'1'), (0, b'b'), (2, b'1'), (1, b' '), (0, b'c')]));
    }

    #[test]
    fn a_sign_the_table_misnames_goes_by_its_place_in_the_font() {
        let table = |pairs: &[(u8, &str)]| Some(pairs.iter().map(|(c, t)| (*c, t.to_string())).collect::<HashMap<u8, String>>());
        // TeX's big-symbol font: both sums come as "∑", a bracket rightly
        let big = FontInfo { base: "CMEX10".into(), unicode: table(&[(88, "∑"), (80, "∑"), (0, "(")]), misnamed: true, ..Default::default() };
        assert_eq!(big.slot_text(88), Some("X".into()));
        assert_eq!(big.slot_text(80), Some("P".into()));
        assert_eq!(big.slot_text(90), Some("Z".into()));
        assert_eq!(big.slot_text(0), Some("(".into()));
        // (a font cut down to the glyphs used names them, under new codes)
        let mut cut = FontInfo { base: "CMEX10".into(), unicode: table(&[]), misnamed: true, ..Default::default() };
        cut.names.insert(3, "integraldisplay".into());
        assert_eq!(cut.slot_text(3), Some("Z".into()));
        // the font of math letters: ν given as "β" is ν, µ stays
        let letters = FontInfo { base: "CMMI7".into(), unicode: table(&[(23, "β"), (22, "µ"), (15, "ε"), (107, "k")]), misnamed: true, ..Default::default() };
        assert_eq!(letters.slot_text(23), Some("ν".into()));
        assert_eq!(letters.slot_text(22), None);
        assert_eq!(letters.slot_text(15), None);
        assert_eq!(letters.slot_text(107), None);
        assert_eq!(letters.slot_text(11), Some("α".into()));
        assert_eq!(letters.slot_text(64), Some("∂".into()));
        // the symbol font and the upright one: a glyph the table has no character for
        let signs = FontInfo { base: "CMSY7".into(), unicode: table(&[(0, "−")]), misnamed: true, ..Default::default() };
        assert_eq!(signs.slot_text(48), Some("′".into()));
        assert_eq!(signs.slot_text(104), Some("⟨".into()));
        assert_eq!(signs.slot_text(0), None);
        assert_eq!(signs.slot_text(54), None);
        let upright = FontInfo { base: "CMR7".into(), unicode: table(&[(49, "1")]), misnamed: true, ..Default::default() };
        assert_eq!(upright.slot_text(3), Some("Λ".into()));
        assert_eq!(upright.slot_text(49), None);
        assert_eq!(upright.slot_text(11), None);
    }

    #[test]
    fn unicode_table() {
        let map = known_only(to_unicode_all(b"1 begincodespacerange <02> <b1> endcodespacerange 2 beginbfchar <02> <2192> <3d> <003D> endbfchar 2 beginbfrange <7b> <7d> <007B> <80> <81> [<2212> <00B1>] endbfrange"));
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
