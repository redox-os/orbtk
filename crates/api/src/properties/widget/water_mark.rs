use crate::prelude::*;

property!(
    /// `WaterMark` describes a placeholder text.
    WaterMark(String)
);

// --- Conversions ---

impl From<&str> for WaterMark {
    fn from(s: &str) -> WaterMark {
        WaterMark(s.into())
    }
}
