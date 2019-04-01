use crate::
structs::{Brush, Color};

property!(
    /// `Foreground` describes the foreground brush of a visual element.
    Foreground(Brush)
);

// --- Conversions ---

impl From<&str> for Foreground {
    fn from(s: &str) -> Foreground {
        Foreground(s.into())
    }
}

impl From<Foreground> for Color {
    fn from(b: Foreground) -> Color {
        b.0.into()
    }
}
