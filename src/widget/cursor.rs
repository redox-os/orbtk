use layout_object::StretchLayoutObject;
use render_object::{RectangleRenderObject};
use theme::Selector;
use widget::{Template, Widget};
use enums::ParentType;

/// The `Cursor` represents a text cursor used to mark text.
pub struct Cursor;

impl Widget for Cursor {
    fn create() -> Template {
        Template::default()
            .as_parent_type(ParentType::Single)
            .with_property(Selector::new().with("cursor"))
            .with_render_object(RectangleRenderObject)
            .with_layout_object(StretchLayoutObject)
            .with_debug_name("Cursor")
    }
}
