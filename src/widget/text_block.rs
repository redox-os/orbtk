use layout_object::TextSizeLayoutObject;
use render_object::TextRenderObject;
use theme::Selector;
use widget::{Label, Template, Widget};

/// The `TextBlock` widget is used to draw text.
pub struct TextBlock;

impl Widget for TextBlock {
    fn template() -> Template {
        print!("TextBlock -> ");
        Template::default()
            .with_property(Label::from("TextBlock"))
            .with_property(Selector::new().with("textblock"))
            .with_layout_object(TextSizeLayoutObject)
            .with_render_object(TextRenderObject)
    }
}
