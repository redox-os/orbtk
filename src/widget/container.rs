
use layout_object::PaddingLayoutObject;
use render_object::{RectangleRenderObject};
use theme::Selector;
use widget::Widget;
use enums::ParentType;
use application::Template;

pub struct Container;

impl Widget for Container {
    fn template() -> Template {
        Template::default()
            .as_parent_type(ParentType::Single)
            .with_property(Selector::new(Some(String::from("container"))))
            .with_render_object(Box::new(RectangleRenderObject))
            .with_layout_object(Box::new(PaddingLayoutObject))   
    }
}
