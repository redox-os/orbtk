use crate::structs::{Brush, Color};

/// Used to draw the foreground brush of a widget.
#[derive(Clone)]
pub struct Foreground(pub Brush);

property!(
    Foreground,
    ForegroundProperty,
    foreground,
    shared_foreground
);

impl From<Foreground> for Color {
    fn from(b: Foreground) -> Color {
        b.0.into()
    }
}

impl Default for Foreground {
    fn default() -> Foreground {
       "#000000".into()
    }
}

impl From<&str> for Foreground {
    fn from(s: &str) -> Foreground {
        Foreground(s.into())
    }
}