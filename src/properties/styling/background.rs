use crate::structs::{Brush, Color};

/// Used to draw the background brush of a widget.
#[derive(Clone)]
pub struct Background(pub Brush);

property!(
    Background,
    BackgroundProperty,
    background,
    shared_background
);

impl From<Background> for Color {
    fn from(b: Background) -> Color {
        b.0.into()
    }
}

impl Default for Background {
    fn default() -> Background {
        "#000000".into()
    }
}

impl From<&str> for Background {
    fn from(s: &str) -> Background {
        Background(s.into())
    }
}
