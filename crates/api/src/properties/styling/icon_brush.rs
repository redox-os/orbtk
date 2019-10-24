use crate::{prelude::*, utils, utils::{Color}};

property!(
    /// `IconBrush` describes the icon brush of a visual element.
    #[derive(Default)]
    IconBrush(utils::Brush) : &str,
    String
);

// --- Conversions ---

impl From<IconBrush> for Color {
    fn from(b: IconBrush) -> Color {
        b.0.into()
    }
}
