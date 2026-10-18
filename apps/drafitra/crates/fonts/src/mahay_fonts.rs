//! Fonts from the optional mahay-fonts build input (<https://github.com/raantss18/mahay>).
//!
//! Built with `MAHAY_FONTS_DIR=<mahay-fonts checkout>`, `build.rs` embeds the fonts listed in its
//! `fonts/manifest.txt` (web builds: only BIZ UDPGothic Regular and the Arabic faces).
//! Without it [`MAHAY_FONTS`] is empty and nothing changes. The font table ([`crate::ttf`])
//! lists the embedded fonts with the installed ones, under [`MahayFont::stem`]. A text style
//! can name one (`BIZ UDPGothic`), and they are fallbacks for characters a text's font lacks.
//! They are copied out of the binary only when first used.

/// A font from the optional mahay-fonts build input (empty unless built with `MAHAY_FONTS_DIR`).
pub struct MahayFont {
    pub family: &'static str,
    pub style: &'static str,
    /// ISO 15924 scripts the font is for, e.g. `"Jpan"`.
    pub scripts: &'static [&'static str],
    pub bytes: &'static [u8],
}

include!(concat!(env!("OUT_DIR"), "/mahay_fonts.rs"));

impl MahayFont {
    /// The key the font table lists this font under, like a lower-case file stem:
    /// `BIZ UDPGothic` `Regular` → `bizudpgothic-regular`.
    pub fn stem(&self) -> String {
        format!("{}-{}", self.family, self.style).chars().filter(|c| *c != ' ').flat_map(char::to_lowercase).collect()
    }
}

/// Whether `stem` is the key of an embedded mahay-fonts font.
pub(crate) fn is_embedded(stem: &str) -> bool {
    MAHAY_FONTS.iter().any(|f| f.stem() == stem)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A minimal sfnt (TrueType header, no tables), made here: no font file is committed.
    static TINY_SFNT: [u8; 12] = [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    #[test]
    fn empty_without_mahay_fonts_dir() {
        // `build.rs` and `option_env!` see the same build environment.
        if option_env!("MAHAY_FONTS_DIR").is_none_or(str::is_empty) {
            assert!(MAHAY_FONTS.is_empty());
            assert!(!is_embedded("bizudpgothic-regular"));
        }
    }

    #[test]
    fn stems_are_lower_case_without_spaces() {
        let f = MahayFont { family: "BIZ UDPGothic", style: "Bold", scripts: &["Jpan"], bytes: &[] };
        assert_eq!(f.stem(), "bizudpgothic-bold");
    }

    #[test]
    fn a_static_font_is_found_by_name_and_loaded_on_use() {
        crate::ttf::register_static("drafitrastatictest-regular", &TINY_SFNT);
        let face = crate::ttf::find("Drafitra Static Test").map(|f| f.bytes.as_slice().to_vec());
        assert_eq!(face.as_deref(), Some(&TINY_SFNT[..]));
        assert!(crate::ttf::find("drafitrastatictest-regular.ttf").is_some());
        // It has no glyphs, so it is never a fallback face, and nothing panics.
        assert!(!crate::ttf::find("Drafitra Static Test").is_some_and(|f| f.has_glyph('A')));
        let _ = crate::ttf::fallback_for('\u{65E5}');
    }

    #[test]
    fn no_barred_family_is_embedded() {
        // AGENTS.md: no Adobe fonts. Noto Sans CJK is Source Han Sans; `build.rs` skips them.
        const BARRED: [&str; 5] = ["Source Han", "Source Serif", "Source Sans", "Noto Sans CJK", "Noto Serif CJK"];
        for f in MAHAY_FONTS {
            assert!(!BARRED.iter().any(|b| f.family.starts_with(b)), "{} is barred", f.family);
        }
    }

    #[test]
    fn embedded_fonts_load_and_cover_their_scripts() {
        // Only with MAHAY_FONTS_DIR (skips otherwise): every embedded font parses, is found by
        // its family name and has a character of its script; Japanese text gets fallback glyphs.
        for f in MAHAY_FONTS {
            let face = crate::ttf::find(&format!("{} {}", f.family, f.style));
            assert!(face.is_some(), "{} {} not found", f.family, f.style);
            let sample = match f.scripts.first().copied() {
                Some("Jpan") => '\u{3042}',
                Some("Arab") => '\u{0628}',
                Some("Hang" | "Kore") => '가',
                _ => 'A',
            };
            assert!(face.is_some_and(|x| x.has_glyph(sample)), "{} lacks {sample}", f.family);
        }
        if MAHAY_FONTS.iter().any(|f| f.scripts.contains(&"Jpan")) {
            let sh = crate::shape_stroke("\u{65E5}\u{672C}", 1.0, 1.0, 0.0);
            assert_eq!(sh.glyphs.len(), 2, "Japanese drawn from a fallback font");
        }
    }
}
