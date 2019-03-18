property!(
    /// `FontIcon` describes the font icon of a widget.
    FontIcon(String)
);

// --- Conversions ---

impl From<&str> for FontIcon {
    fn from(s: &str) -> FontIcon {
        FontIcon(s.into())
    }
}

impl Into<PropertySource<FontIcon>> for &str {
    fn into(self) -> PropertySource<FontIcon> {
        PropertySource::Value(FontIcon::from(self))
    }
}