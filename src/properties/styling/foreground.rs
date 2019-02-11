use crate::structs::Brush;

/// Used to draw the foreground brush of a widget.
pub struct Foreground(pub Brush);

property!(
    Foreground,
    ForegroundProperty,
    foreground,
    shared_foreground
);

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