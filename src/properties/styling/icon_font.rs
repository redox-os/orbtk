use orbfont::Font as OrbFont;

use crate::styling::fonts;

/// Text font.
#[derive(Clone)]
pub struct IconFont(pub OrbFont);

property!(IconFont, IconFontProperty, icon_font, shared_icon_font);

impl Default for IconFont {
    fn default() -> Self {
        IconFont::from(fonts::font_into_box(fonts::MATERIAL_ICONS_REGULAR_FONT))
    }
}

impl From<&str> for IconFont {
    fn from(s: &str) -> IconFont {
        IconFont(OrbFont::from_path(s).unwrap())
    }
}

impl From<String> for IconFont {
    fn from(s: String) -> IconFont {
        IconFont(OrbFont::from_path(s).unwrap())
    }
}

impl From<Box<[u8]>> for IconFont {
    fn from(s: Box<[u8]>) -> IconFont {
        IconFont(OrbFont::from_data(s).unwrap())
    }
}
