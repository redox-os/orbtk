use crate::{
    layout::FixedSizeLayout,
    properties::{TextProperty, ForegroundProperty},
    render_object::TextRenderObject,
    styling,
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
            .debug_name("TextBlock")
            .foreground(styling::LINK_WATER_COLOR)
            .text("TextBlock")
    }
}

template!(TextBlockTemplate, [ForegroundProperty, TextProperty]);
