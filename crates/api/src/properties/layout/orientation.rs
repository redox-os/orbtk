use crate::prelude::*;

/// Is used to control the orientation of the `Stack`.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OrientationValue {
    /// Vertical orientation.
    Vertical,

    /// Horizontal orientation.
    Horizontal,
}

// --- Conversions ---

impl From<&str> for OrientationValue {
    fn from(t: &str) -> Self {
        match t {
            "Horizontal" | "horizontal" => OrientationValue::Horizontal,
            _ => OrientationValue::Vertical,
        }
    }
}

impl Default for OrientationValue {
    fn default() -> OrientationValue {
        OrientationValue::Vertical
    }
}

property!(
    /// `Orientation` describes the orientation of the `Stack`.
    Orientation(OrientationValue) : &str
);
