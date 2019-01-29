use crate::{
    layout::FixedSizeLayout,
    render_object::ImageRenderObject,
    properties::ImageProperty,
    widget::{Template, Widget},
};

/// The `ImageWidget` widget is used to draw an image. It is not interactive.
///
/// # Properties
///
/// * `Selector` - CSS selector with  element name `image`, used to request the theme of the image widget.
///
/// # Others
///
/// * `ParentType`- None.
/// * `ImageSizeLayout` - Used to layout the widget.
/// * `ImageRenderObject` - Used to draw the image of the widget.
pub struct ImageWidget;

impl Widget for ImageWidget {
    type Template = ImageTemplate;

    fn create() -> Self::Template {
        ImageTemplate::new()
            .selector("imagewidget")
            .layout(FixedSizeLayout::new())
            .render_object(ImageRenderObject)
            .debug_name("ImageWidget")
    }
}

template!(ImageTemplate, [ImageProperty]);