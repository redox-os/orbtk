use crate::{
    layout::TextSizeLayout,
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
            .with_property(Label::from("TextBlock"))
            .with_property(Selector::from("textblock"))
            .with_layout(TextSizeLayout)
            .with_render_object(TextRenderObject)
            .with_debug_name("TextBlock")
    }
}
