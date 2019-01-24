use crate::{
    layout::ImageSizeLayout,
    render_object::ImageRenderObject,
    theme::Selector,
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
    fn create() -> Template {
        Template::default()
            .property(Selector::from("imagewidget"))
            .layout(ImageSizeLayout)
            .render_object(ImageRenderObject)
            .debug_name("ImageWidget")
    }
}
