use crate::{prelude::*, utils::*};

property!(
    /// `IconBrush` describes the icon brush of a visual element.
    IconBrush(Brush)
);

// --- Conversions ---

impl From<&str> for IconBrush {
    fn from(s: &str) -> IconBrush {
        IconBrush(s.into())
    }
}

impl From<IconBrush> for Color {
    fn from(b: IconBrush) -> Color {
        b.0.into()
    }
}
