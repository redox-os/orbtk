/// Is used to control the visibility of a widget
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Visibility {
    /// The widget is visible.
    Visible,

    /// The widget will not be displayed but it takes its space in the layout.
    Hidden,

    /// The widget will not be displayed but it doesen't takes space in the layout.
    Collapsed,
}

property!(Visibility, VisibilityProperty, visibility, shared_visibility);

impl Default for Visibility {
    fn default() -> Visibility {
        Visibility::Visible
    }
}

impl From<&str> for Visibility {
    fn from(t: &str) -> Self {
        match t {
            "Hidden" | "hidden" => Visibility::Hidden,
            "Collapsed" | "collapsed" => Visibility::Collapsed,
            _ => Visibility::Visible,
        }
    }
}