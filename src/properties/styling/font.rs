use orbfont::Font as OrbFont;

use crate::styling::fonts;

/// Text font.
#[derive(Clone)]
pub struct Font(pub OrbFont);

property!(
    Font,
    FontProperty,
    font,
    shared_font
);

impl Default for Font {
    fn default() -> Self {
        Font::from(fonts::font_into_box(fonts::ROBOTO_REGULAR_FONT))
    }
}


impl From<&str> for Font {
    fn from(s: &str) -> Font {
        Font(OrbFont::from_path(s).unwrap())
    }
}

impl From<Box<[u8]>> for Font {
    fn from(s: Box<[u8]>) -> Font {
        Font(OrbFont::from_data(s).unwrap())
    }
}
