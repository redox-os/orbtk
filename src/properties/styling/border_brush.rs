use crate::structs::{Brush, Color};

/// Used to draw the border brush of a widget.
#[derive(Clone)]
pub struct BorderBrush(pub Brush);

property!(
    BorderBrush,
    BorderBrushProperty,
    border_brush,
    shared_border_brush
);

impl From<BorderBrush> for Color {
    fn from(b: BorderBrush) -> Color {
        b.0.into()
    }
}

impl Default for BorderBrush {
    fn default() -> BorderBrush {
       "#000000".into()
    }
}

impl From<&str> for BorderBrush {
    fn from(s: &str) -> BorderBrush {
        BorderBrush(s.into())
    }
}