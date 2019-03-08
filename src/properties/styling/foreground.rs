use crate::{
    widget::PropertySource,
    structs::{Brush, Color},
};

/// Used to draw the foreground brush of a widget.
#[derive(Clone)]
pub struct Foreground(pub Brush);

property!(Foreground, ForegroundProperty, foreground, foreground_prop);

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

wip_property!(WipForeground(Brush));

impl From<&str> for WipForeground {
    fn from(s: &str) -> WipForeground {
        WipForeground(s.into())
    }
}

impl Into<PropertySource<WipForeground>> for &str {
    fn into(self) -> PropertySource<WipForeground> {
        PropertySource::Value(WipForeground::from(self))
    }
}
