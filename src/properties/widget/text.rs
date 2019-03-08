use crate::widget::PropertySource;

/// The `Text` struct represents a string used for text drawing.
#[derive(Default, Clone)]
pub struct Text(pub String);

property!(Text, TextProperty, text, text_prop);

impl From<&str> for Text {
    fn from(s: &str) -> Text {
        Text(s.to_string())
    }
}

impl From<String> for Text {
    fn from(s: String) -> Text {
        Text(s)
    }
}

// todo tests!!!

wip_property!(WipText(String));

impl From<&str> for WipText {
    fn from(s: &str) -> WipText {
        WipText(s.into())
    }
}

impl Into<PropertySource<WipText>> for &str {
    fn into(self) -> PropertySource<WipText> {
        PropertySource::Value(WipText::from(self))
    }
}