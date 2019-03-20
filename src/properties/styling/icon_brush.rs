use crate::
structs::{Brush, Color};

property!(
    /// `IconBrush` describes the icon brush of a visual element.
    IconBrush(Brush));

// --- Conversions ---

impl From<&str> for IconBrush {
    fn from(s: &str) -> IconBrush {
        IconBrush(s.into())
    }
}

impl Into<PropertySource<IconBrush>> for &str {
    fn into(self) -> PropertySource<IconBrush> {
        PropertySource::Value(IconBrush::from(self))
    }
}

impl From<IconBrush> for Color {
    fn from(b: IconBrush) -> Color {
        b.0.into()
    }
}
