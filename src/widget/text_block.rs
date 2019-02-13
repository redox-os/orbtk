use crate::{
    layout::FixedSizeLayout,
    properties::{FontProperty, FontSizeProperty, ForegroundProperty, TextProperty},
    render_object::TextRenderObject,
    styling::{colors, fonts},
    widget::{Template, Widget},
};

/// The `TextBlock` widget is used to draw text. It is not interactive.
///
/// # Properties
///
/// * `text` - String used to display the text of the text block.
/// * `foreground` - A brush that describes the foreground color.
///
/// # Others
///
/// * `FixedSizeLayout` - Used to layout the widget.
/// * `TextRenderObject` - Used to draw the text of the widget.
pub struct TextBlock;

impl Widget for TextBlock {
    type Template = TextBlockTemplate;

    fn create() -> Self::Template {
        TextBlockTemplate::new()
            .layout(FixedSizeLayout::new())
            .render_object(TextRenderObject)
            .foreground(colors::LINK_WATER_COLOR)
            .font_size(fonts::FONT_SIZE_12)
            .font(fonts::font_into_box(fonts::ROBOTO_REGULAR_FONT))
            .text("TextBlock")
            .debug_name("TextBlock")
    }
}

template!(
    TextBlockTemplate,
    [
        ForegroundProperty,
        TextProperty,
        FontSizeProperty,
        FontProperty
    ]
);
