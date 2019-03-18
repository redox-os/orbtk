use std::fmt;

use orbfont::Font as OrbFont;

use crate::styling::fonts;

use super::InnerFont;

property!(
    /// `IconFont` describes the icon font of a widget.
    IconFont(InnerFont)
);

// --- Conversions ---

impl From<&str> for IconFont {
    fn from(s: &str) -> IconFont {
        IconFont::from(InnerFont::from(OrbFont::from_path(s).unwrap()))
    }
}

impl Into<PropertySource<IconFont>> for &str {
    fn into(self) -> PropertySource<IconFont> {
        PropertySource::Value(IconFont::from(self))
    }
}

impl From<String> for IconFont {
    fn from(s: String) -> IconFont {
        IconFont::from(InnerFont::from(OrbFont::from_path(s).unwrap()))
    }
}

impl Into<PropertySource<IconFont>> for String {
    fn into(self) -> PropertySource<IconFont> {
        PropertySource::Value(IconFont::from(self))
    }
}

impl From<Box<[u8]>> for IconFont {
    fn from(s: Box<[u8]>) -> IconFont {
        IconFont::from(InnerFont::from(OrbFont::from_data(s).unwrap()))
    }
}

impl Into<PropertySource<IconFont>> for Box<[u8]> {
    fn into(self) -> PropertySource<IconFont> {
        PropertySource::Value(IconFont::from(self))
    }
}
