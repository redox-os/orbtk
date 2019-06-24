use crate::prelude::*;

property!(
    /// `IconFont` describes the icon font of a widget.
    IconFont(String)
);

// --- Conversions ---

impl From<&str> for IconFont {
    fn from(s: &str) -> IconFont {
        IconFont(s.into())
    }
}
