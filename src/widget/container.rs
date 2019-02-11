use crate::{
    layout::PaddingLayout,
    properties::*,
    render_object::RectangleRenderObject,
    styling::colors,
    widget::{Template, Widget},
};

/// The `Container` represents a layout that surrounds its child with a padding. Draws a box around the child.
///
/// # Properties
///
/// * `background` - background drawing brush.
/// * `padding` - gap to the child.
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
            .background(colors::LYNCH_COLOR)
            .border_radius(0.0)
            .border_thickness(0.0)
            .border_brush("transparent")
            .render_object(RectangleRenderObject)
            .layout(PaddingLayout::new())
            .debug_name("Container")
    }
}

template!(
    ContainerTemplate,
    [
        BackgroundProperty,
        BorderRadiusProperty,
        BorderThicknessProperty,
        BorderBrushProperty,
        PaddingProperty
    ]
);
