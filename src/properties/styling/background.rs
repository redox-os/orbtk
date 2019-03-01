use crate::structs::{Brush, Color};

/// Used to draw the background brush of a widget.
#[derive(Clone)]
pub struct Background(pub Brush);

wip_property!(
    /// Used to set the background of a widget.
    BackgroundProperty: Background(
        /// Sets the background.
        background, 
        /// Sets the background property.
        background_prop
    )
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