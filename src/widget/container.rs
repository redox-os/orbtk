use crate::{
    layout::PaddingLayout,
    properties::PaddingProperty,
    render_object::RectangleRenderObject,
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
    type Template = ContainerTemplate;

    fn create() -> Self::Template {
        ContainerTemplate::new()
            .padding(0.0)
            .render_object(RectangleRenderObject)
            .layout(PaddingLayout::new())
            .selector("container")
            .debug_name("Container")
    }
}

template!(ContainerTemplate, [PaddingProperty]);
