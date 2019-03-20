/// Is used to control the orientation of the `Stack`.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OrientationValue {
    /// Vertical orientation.
    Vertical,

    /// Horizontal orientation.
    Horizontal,
}

impl Default for OrientationValue {
    fn default() -> OrientationValue {
        OrientationValue::Vertical
    }
}

property!(
    /// `Orientation` describes the orientation of the `Stack`.
    Orientation(OrientationValue)
);

// --- Conversions ---

impl From<&str> for Orientation {
    fn from(t: &str) -> Self {
        match t {
            "Horizontal" | "horizontal" => Orientation::from(OrientationValue::Horizontal),
            _ => Orientation::from(OrientationValue::Vertical),
        }
    }
}

impl Into<PropertySource<Orientation>> for &str {
    fn into(self) -> PropertySource<Orientation> {
        PropertySource::Value(Orientation::from(self))
    }
}
