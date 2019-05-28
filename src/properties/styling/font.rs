use std::fmt;

use orbgl_api::Font as OrbFont;

use crate::prelude::*;

#[derive(Clone)]
pub struct InnerFont(pub OrbFont);

impl Default for InnerFont {
    fn default() -> Self {
        InnerFont::from(fonts::font_into_box(fonts::ROBOTO_REGULAR_FONT))
    }
}

impl fmt::Debug for InnerFont {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("InnerFont(orbfont::Font)")
    }
}

impl PartialEq for InnerFont {
    // todo: impl
    fn eq(&self, _other: &InnerFont) -> bool {
        false
    }
}

impl From<OrbFont> for InnerFont {
    fn from(font: OrbFont) -> InnerFont {
        InnerFont(font)
    }
}

impl From<Box<[u8]>> for InnerFont {
    fn from(s: Box<[u8]>) -> InnerFont {
        InnerFont::from(OrbFont::from_data(s).unwrap())
    }
}

property!(
    /// `Font` describes the text font of a widget.
    Font(InnerFont)
);

// --- Conversions ---

impl From<OrbFont> for Font {
    fn from(s: OrbFont) -> Font {
        Font::from(InnerFont::from(s))
    }
}

impl From<&str> for Font {
    fn from(s: &str) -> Font {
        Font::from(InnerFont::from(OrbFont::from_path(s).unwrap()))
    }
}

impl From<String> for Font {
    fn from(s: String) -> Font {
        Font::from(InnerFont::from(OrbFont::from_path(s).unwrap()))
    }
}

impl From<Box<[u8]>> for Font {
    fn from(s: Box<[u8]>) -> Font {
        Font::from(InnerFont::from(OrbFont::from_data(s).unwrap()))
    }
}
