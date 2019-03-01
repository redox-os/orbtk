use crate::{
    layout::FixedSizeLayout,
    properties::{FontProperty, FontSizeProperty, ForegroundProperty, TextProperty},
    render_object::TextRenderObject,
    styling::{colors, fonts},
    widget::{Template, Widget},
};

widget!(
    /// The `TextBlock` widget is used to draw text. It is not interactive.
    TextBlock(
        ForegroundProperty,
        TextProperty,
        FontSizeProperty,
        FontProperty
    )
);

impl Widget for TextBlock {
    fn create() -> Self {
        TextBlock::new()
            .layout(FixedSizeLayout::new())
            .render_object(TextRenderObject)
            .foreground(colors::LINK_WATER_COLOR)
            .font_size(fonts::FONT_SIZE_12)
            .font(fonts::font_into_box(fonts::ROBOTO_REGULAR_FONT))
            .text("TextBlock")
            .debug_name("TextBlock")
    }
}
