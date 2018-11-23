
use layout_object::PaddingLayoutObject;
use render_object::{RectangleRenderObject};
use theme::Selector;
use widget::{Template, Widget};
use enums::ParentType;

/// The `Container` layout surrounds its child with a padding. Draws a box arround the child.
pub struct Container;

impl Widget for Container {
    fn template() -> Template {
        print!("Container -> ");
        Template::default()
            .as_parent_type(ParentType::Single)
            .with_property(Selector::new().with("container"))
            .with_render_object(RectangleRenderObject)
            .with_layout_object(PaddingLayoutObject) 
    }
}
