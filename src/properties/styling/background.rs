use crate::
    structs::{Brush, Color};

property!(
    /// `Background` describes the background brush of a visual element.
    Background(Brush)
);

// --- Conversions ---

impl From<&str> for Background {
    fn from(s: &str) -> Background {
        Background(s.into())
    }
}

impl Into<PropertySource<Background>> for &str {
    fn into(self) -> PropertySource<Background> {
        PropertySource::Value(Background::from(self))
    }
}

impl From<Background> for Color {
    fn from(b: Background) -> Color {
        b.0.into()
    }
}
