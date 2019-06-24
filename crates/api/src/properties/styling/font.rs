use crate::prelude::*;

property!(
    /// `Font` describes the text font of a widget.
    Font(String)
);

// --- Conversions ---

impl From<&str> for Font {
    fn from(s: &str) -> Font {
        Font(s.into())
    }
}
