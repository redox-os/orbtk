use crate::{
    layout::FixedSizeLayout,
    properties::FontIconProperty,
    render_object::FontIconRenderObject,
    widget::{Template, Widget},
};

/// The `FontIconBlock` widget is used to draw an font icon. It is not interactive.
///
/// # Properties
///
/// * `selector` - CSS selector with  element name `fonticon`, used to request the theme of the font icon block.
///
/// # Others
///
/// * `FixedSizeLayout` - Used to layout the widget.
/// * `FontIconRenderObject` - Used to draw the text of the widget.
pub struct FontIconBlock;

impl Widget for FontIconBlock {
    type Template = FontIconBlockTemplate;

    fn create() -> Self::Template {
        FontIconBlockTemplate::new()
            .layout(FixedSizeLayout::new())
            .render_object(FontIconRenderObject)
            .debug_name("FontIconBlock")
            .font_icon("")
            .selector("fonticon")
    }
}

template!(FontIconBlockTemplate, [FontIconProperty]);
