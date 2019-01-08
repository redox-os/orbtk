use crate::{
    layout::StretchLayout,
    render_object::ImageRenderObject,
    theme::Selector,
    widget::{Template, Widget},
};

/// The `CanvasWidget` widget is used to provide custom drawing by handling the `Canvas` struct from `OrbGl` as property.
///
/// # Properties
///
/// * `Selector` - CSS selector with  element name `image`, used to request the theme of the image widget.
///
/// # Others
///
/// * `ParentType`- None.
/// * `StretchLayout` - Used to layout the widget.
/// * `ImageRenderObject` - Used to draw the image of the widget.
pub struct CanvasWidget;

impl Widget for CanvasWidget {
    fn create() -> Template {
        Template::default()
            .with_property(Selector::from("imagewidget"))
            .with_layout(StretchLayout::default())
            .with_render_object(ImageRenderObject)
            .with_debug_name("ImageWidget")
    }
}
