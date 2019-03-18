/// Is used to control the visibility of a widget
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VisibilityValue {
    /// The widget is visible.
    Visible,

    /// The widget will not be displayed but it takes its space in the layout.
    Hidden,

    /// The widget will not be displayed but it doesn't takes space in the layout.
    Collapsed,
}

impl Default for VisibilityValue {
    fn default() -> VisibilityValue {
        VisibilityValue::Visible
    }
}

property!(
    /// `Visibility` describes the visibility of a widget.
    Visibility(VisibilityValue)
);

// --- Conversions ---

impl From<&str> for Visibility {
    fn from(t: &str) -> Self {
        match t {
            "Hidden" | "hidden" => Visibility::from(VisibilityValue::Hidden),
            "Collapsed" | "collapsed" => Visibility::from(VisibilityValue::Collapsed),
            _ => Visibility::from(VisibilityValue::Visible),
        }
    }
}

impl Into<PropertySource<Visibility>> for &str {
    fn into(self) -> PropertySource<Visibility> {
        PropertySource::Value(Visibility::from(self))
    }
}
