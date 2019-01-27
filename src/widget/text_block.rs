use crate::{
    layout::FixedSizeLayout,
    properties::Label,
    render_object::TextRenderObject,
    theme::Selector,
    widget::{Template, Widget},
};

/// The `TextBlock` widget is used to draw text. It is not interactive.
///
/// # Properties
///
/// * `Label` - String used to display the text of the text block.
/// * `Selector` - CSS selector with  element name `textblock`, used to request the theme of the text block.
///
/// # Others
///
/// * `ParentType`- None.
/// * `TextSizeLayout` - Used to layout the widget.
/// * `TextRenderObject` - Used to draw the text of the widget.
pub struct TextBlock;

impl Widget for TextBlock {
    fn create() -> Template {
        Template::default()
            .property(Label::from("TextBlock"))
            .selector("textblock")
            .layout(FixedSizeLayout)
            .render_object(TextRenderObject)
            .debug_name("TextBlock")
    }
}
