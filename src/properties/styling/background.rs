use crate::structs::Brush;

/// Used to draw the background brush of a widget.
pub struct Background(pub Brush);

property!(
    Background,
    BackgroundProperty,
    background,
    shared_background
);

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