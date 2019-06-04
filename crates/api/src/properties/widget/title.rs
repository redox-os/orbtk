use crate::prelude::*;

property!(
    /// `Title` describes the title of a widget.
    Title(String)
);

// --- Conversions ---

impl From<&str> for Title {
    fn from(s: &str) -> Title {
        Title(s.into())
    }
}
