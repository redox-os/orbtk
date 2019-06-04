use crate::prelude::*;

property!(
    /// `Text` describes the text of a widget.
    Text(String)
);

// --- Conversions ---

impl From<&str> for Text {
    fn from(s: &str) -> Text {
        Text(s.into())
    }
}
