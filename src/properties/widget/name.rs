property!(
    /// `Name` is use for debugging purposes.
    Name(String)
);

// --- Conversions ---

impl From<&str> for Name {
    fn from(s: &str) -> Name {
        Name(s.into())
    }
}

impl Into<PropertySource<Name>> for &str {
    fn into(self) -> PropertySource<Name> {
        PropertySource::Value(Name::from(self))
    }
}