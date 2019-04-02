use crate::prelude::*;

property!(
    /// `FontIcon` describes the font icon of a widget.
    FontIcon(String)
);

// --- Conversions ---

impl From<&str> for FontIcon {
    fn from(s: &str) -> FontIcon {
        FontIcon(s.into())
    }
}
