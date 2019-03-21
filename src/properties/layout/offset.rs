property!(
    /// `Offset` describes the x- and y-axis offset of a widget.
    Offset((f64, f64))
);

impl From<f64> for Offset {
    fn from(t: f64) -> Self {
        Offset((t, t))
    }
}

impl Into<PropertySource<Offset>> for f64 {
    fn into(self) -> PropertySource<Offset> {
        PropertySource::Value(Offset::from(self))
    }
}