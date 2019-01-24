use crate::{
    properties::Selected,
    theme::Selector,
    widget::{Button, Template, Widget},
};

/// The `ToggleButton` widget can be switch its selected state. It derives from `Button` widget.
///
/// # Properties
///
/// * `Selected` - Bool value represents the selected state of the widget.
/// * `Selector` - CSS selector with  element name `togglebutton`, used to request the theme of the widget.
pub struct ToggleButton;

impl Widget for ToggleButton {
    fn create() -> Template {
        Button::create()
            .property(Selected(false))
            .property(Selector::from("togglebutton"))
            .debug_name("ToggleButton")
    }
}
