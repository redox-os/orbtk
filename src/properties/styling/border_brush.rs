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

impl From<BorderBrush> for Color {
    fn from(b: BorderBrush) -> Color {
        b.0.into()
    }
}
