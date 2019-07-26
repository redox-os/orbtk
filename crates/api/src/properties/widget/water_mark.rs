use crate::{prelude::*, utils::String16};

property!(
    /// `WaterMark` describes a placeholder text.
    WaterMark(String16)
);

// --- Conversions ---

impl From<&str> for WaterMark {
    fn from(s: &str) -> WaterMark {
        WaterMark(s.into())
    }
}

impl From<String> for WaterMark {
    fn from(s: String) -> WaterMark {
        WaterMark(s.into())
    }
}
