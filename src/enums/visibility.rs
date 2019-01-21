/// Is used to control the visibility of a widget
#[derive(PartialEq)]
pub enum Visibility {
    /// The widget is visible.
    Visible,

    /// The widget will not be displayed but it takes its space in the layout.
    Hidden,

    /// The widget will not be displayed but it doesen't takes space in the layout.
    Collapsed,
}

impl Default for Visibility {
    fn default() -> Visibility {
        Visibility::Visible
    }
}
