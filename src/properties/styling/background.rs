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

impl From<Background> for Color {
    fn from(b: Background) -> Color {
        b.0.into()
    }
}
