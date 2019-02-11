use crate::structs::Brush;

/// Used to draw the border brush of a widget.
pub struct BorderBrush(pub Brush);

property!(
    BorderBrush,
    BorderBrushProperty,
    border_brush,
    shared_border_brush
);

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