property!(
    /// `Text` describes the text of a widget.
    Text(String)
);

// --- Conversions ---

impl From<&str> for Text {
    fn from(s: &str) -> Text {
        Text(s.into())
    }
}

impl Into<PropertySource<Text>> for &str {
    fn into(self) -> PropertySource<Text> {
        PropertySource::Value(Text::from(self))
    }
}