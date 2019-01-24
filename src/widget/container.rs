use crate::{
    enums::ParentType,
    layout::PaddingLayout,
    render_object::RectangleRenderObject,
    theme::Selector,
    widget::{Template, Widget},
};

/// The `Container` represents a layout that surrounds its child with a padding. Draws a box arround the child.
///
/// # Properties
///
/// * `Selector` - CSS selector with element name `container`, used to request the theme of the widget.
///
/// # Others
///
/// * `ParentType`- Single.
/// * `PaddingLayout` - Used to layout the widget.
/// * `RectangleRenderObject` - Used to draw the widget.
pub struct Container;

impl Widget for Container {
    fn create() -> Template {
        Template::default()
           .parent_type(ParentType::Single)
            .property(Selector::from("container"))
            .render_object(RectangleRenderObject)
            .layout(PaddingLayout)
            .debug_name("Container")
    }
}
