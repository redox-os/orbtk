/// Is used to control the visibility of a widget
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Visibility {
    /// The widget is visible.
    Visible,

    /// The widget will not be displayed, isn't taken into account in the
    /// layout pipeline. It **does** consume memory in the render buffer.
    Hidden,

    /// The widget isn't displayed but `is` rendered. Thus it **does
    /// not** consume space in the layout.
    Collapsed,
}

impl Default for Visibility {
    fn default() -> Visibility {
        Visibility::Visible
    }
}

// --- Conversions ---

impl From<&str> for Visibility {
    fn from(t: &str) -> Self {
        match t {
            "Hidden" | "hidden" => Visibility::Hidden,
            "Collapsed" | "collapsed" => Visibility::Collapsed,
            _ => Visibility::Visible,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into() {
        let visibility: Visibility = "Hidden".into();
        assert_eq!(visibility, Visibility::Hidden);

        let visibility: Visibility = "hidden".into();
        assert_eq!(visibility, Visibility::Hidden);

        let visibility: Visibility = "Collapsed".into();
        assert_eq!(visibility, Visibility::Collapsed);

        let visibility: Visibility = "collapsed".into();
        assert_eq!(visibility, Visibility::Collapsed);

        let visibility: Visibility = "Visible".into();
        assert_eq!(visibility, Visibility::Visible);

        let visibility: Visibility = "visible".into();
        assert_eq!(visibility, Visibility::Visible);

        let visibility: Visibility = "other".into();
        assert_eq!(visibility, Visibility::Visible);
    }
}
