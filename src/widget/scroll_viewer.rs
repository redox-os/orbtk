use enums::ParentType;
use layout_object::ScrollLayoutObject;
use widget::{Template, Widget};
use structs::Offset;

/// Use to scroll its content.
pub struct ScrollViewer;

impl Widget for ScrollViewer {
    fn create() -> Template {
        Template::default()
            .as_parent_type(ParentType::Single)
            .with_property(Offset::default())
            .with_layout_object(ScrollLayoutObject)
            .with_debug_name("ScrollViewer")
    }
}
