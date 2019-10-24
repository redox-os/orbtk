use crate::prelude::*;

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
    #[derive(Default)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into() {
        let visibility: Visibility = "Hidden".into();
        assert_eq!(visibility.0, VisibilityValue::Hidden);

        let visibility: Visibility = "hidden".into();
        assert_eq!(visibility.0, VisibilityValue::Hidden);

        let visibility: Visibility = "Collapsed".into();
        assert_eq!(visibility.0, VisibilityValue::Collapsed);

        let visibility: Visibility = "collapsed".into();
        assert_eq!(visibility.0, VisibilityValue::Collapsed);

        let visibility: Visibility = "Visible".into();
        assert_eq!(visibility.0, VisibilityValue::Visible);

        let visibility: Visibility = "visible".into();
        assert_eq!(visibility.0, VisibilityValue::Visible);

        let visibility: Visibility = "other".into();
        assert_eq!(visibility.0, VisibilityValue::Visible);
    }
}
