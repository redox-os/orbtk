use crate::{
    layout::PaddingLayout,
    properties::PaddingProperty,
    shapes::Rectangle,
    widget::{Template, Widget},
};

/// The `Container` represents a layout that surrounds its child with a padding. Draws a box arround the child.
///
/// # Properties
///
/// * `selector` - CSS selector with element name `container`, used to request the theme of the widget.
///
/// # Others
///
/// * `PaddingLayout` - Used to layout the widget.
/// * `Rectangle` - Used to draw the widget.
pub struct Container;

impl Widget for Container {
    type Template = ContainerTemplate;

    fn create() -> Self::Template {
        ContainerTemplate::new()
            .padding(0.0)
            .shape(Rectangle::default())
            .layout(PaddingLayout::new())
            .selector("container")
            .debug_name("Container")
    }
}

template!(ContainerTemplate, [PaddingProperty]);
