/// Used to define the `parent_type`of a widget.
pub enum ParentType {
    /// None children could add to the widget.
    None,

    /// Only one child could be added to the widget.
    Single,

    /// Multiple children could be added tot the widget.
    Multi,
}