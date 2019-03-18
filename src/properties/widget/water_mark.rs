property!(
    /// `WaterMark` describes a placeholder text.
    WaterMark(String)
);

// --- Conversions ---

impl From<&str> for WaterMark {
    fn from(s: &str) -> WaterMark {
        WaterMark(s.into())
    }
}

impl Into<PropertySource<WaterMark>> for &str {
    fn into(self) -> PropertySource<WaterMark> {
        PropertySource::Value(WaterMark::from(self))
    }
}