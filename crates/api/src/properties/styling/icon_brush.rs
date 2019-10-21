use crate::{prelude::*, utils::*};

property!(
    /// `IconBrush` describes the icon brush of a visual element.
    #[derive(Default)]
    IconBrush(Brush) : &str,
    String
);

// --- Conversions ---

impl From<IconBrush> for Color {
    fn from(b: IconBrush) -> Color {
        b.0.into()
    }
}
