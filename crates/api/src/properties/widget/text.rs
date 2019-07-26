use crate::{prelude::*, utils::String16};

property!(
    /// `Text` describes the text of a widget.
    Text(String16)
);

// --- Conversions ---

impl From<&str> for Text {
    fn from(s: &str) -> Text {
        Text(s.into())
    }
}

impl From<String> for Text {
    fn from(s: String) -> Text {
        Text(s.into())
    }
}
