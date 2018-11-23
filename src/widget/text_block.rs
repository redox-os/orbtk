use layout_object::TextSizeLayoutObject;
use render_object::TextRenderObject;
use theme::Selector;
use widget::{Label, Widget};
use application::Template;

pub struct TextBlock;

impl Widget for TextBlock {
    fn template() -> Template {
        Template::default()
            .with_property(Label(String::from("")))
            .with_property(Selector::new(Some(String::from("textblock"))))
            .with_layout_object(Box::new(TextSizeLayoutObject))
            .with_render_object(Box::new(TextRenderObject))
    }
}