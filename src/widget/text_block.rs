use layout_object::TextSizeLayoutObject;
use render_object::TextRenderObject;
use theme::Selector;
use widget::{Label, Widget};
use application::Template;

pub struct TextBlock;

impl Widget for TextBlock {
    fn template() -> Template {
        Template::default()
            .with_property(Label::from("TextBlock"))
            .with_property(Selector::new().with("textblock"))
            .with_layout_object(TextSizeLayoutObject)
            .with_render_object(TextRenderObject)
    }
}