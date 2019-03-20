use crate::
structs::{Brush, Color};

property!(
    /// `BorderBrush` describes the border brush.
    BorderBrush(Brush)
);

// --- Conversions ---

impl From<&str> for BorderBrush {
    fn from(s: &str) -> BorderBrush {
        BorderBrush(s.into())
    }
}

impl Into<PropertySource<BorderBrush>> for &str {
    fn into(self) -> PropertySource<BorderBrush> {
        PropertySource::Value(BorderBrush::from(self))
    }
}

impl From<BorderBrush> for Color {
    fn from(b: BorderBrush) -> Color {
        b.0.into()
    }
}
