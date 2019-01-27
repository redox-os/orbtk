use crate::{
    layout::FixedSizeLayout,
    properties::{HorizontalAlignmentProperty, Text, TextProperty, VerticalAlignmentProperty},
    render_object::TextRenderObject,
    theme::SelectorProperty,
    widget::{Template, Widget},
};

#[macro_use]
use crate::widget;

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
        TextBlockTemplate(
            Template::default()
                .layout(FixedSizeLayout)
                .render_object(TextRenderObject)
                .debug_name("TextBlock"),
        )
        .text("TextBlock")
        .selector("textblock")
    }
}

provide_properties!(
    TextBlockTemplate,
    [
        TextProperty,
        VerticalAlignmentProperty,
        HorizontalAlignmentProperty,
        SelectorProperty
    ]
);
