/// Is used to control the visibility of a widget
#[derive(PartialEq)]
pub enum Visibility {
    /// The widget is visible.
    Visible,

    /// The widget will not be displayed but it takes it place in the layout.
    Hidden,
}