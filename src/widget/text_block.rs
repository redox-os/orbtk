use crate::{
    layout::FixedSizeLayout,
    properties::TextProperty,
    render_object::TextRenderObject,
    widget::{Template, Widget},
};

/// The `TextBlock` widget is used to draw text. It is not interactive.
///
/// # Properties
///
/// * `Text` - String used to display the text of the text block.
/// * `Selector` - CSS selector with  element name `textblock`, used to request the theme of the text block.
///
/// # Others
///
/// * `ParentType`- None.
/// * `TextSizeLayout` - Used to layout the widget.
/// * `TextRenderObject` - Used to draw the text of the widget.
pub struct TextBlock;

impl Widget for TextBlock {
    type Template = TextBlockTemplate;

    fn create() -> Self::Template {
        TextBlockTemplate::new()
            .layout(FixedSizeLayout::new())
            .render_object(TextRenderObject)
            .debug_name("TextBlock")
            .text("TextBlock")
            .selector("textblock")
    }
}

template!(
    TextBlockTemplate, [TextProperty]
);
