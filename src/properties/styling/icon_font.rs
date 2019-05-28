use orbgl_api::Font as OrbFont;

use crate::prelude::*;

property!(
    /// `IconFont` describes the icon font of a widget.
    IconFont(InnerFont)
);

// --- Conversions ---

impl From<OrbFont> for IconFont {
    fn from(s: OrbFont) -> IconFont {
        IconFont::from(InnerFont::from(s))
    }
}

impl From<&str> for IconFont {
    fn from(s: &str) -> IconFont {
        IconFont::from(InnerFont::from(OrbFont::from_path(s).unwrap()))
    }
}

impl From<String> for IconFont {
    fn from(s: String) -> IconFont {
        IconFont::from(InnerFont::from(OrbFont::from_path(s).unwrap()))
    }
}

impl From<Box<[u8]>> for IconFont {
    fn from(s: Box<[u8]>) -> IconFont {
        IconFont::from(InnerFont::from(OrbFont::from_data(s).unwrap()))
    }
}
