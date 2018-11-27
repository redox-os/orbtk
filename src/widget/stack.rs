use widget::{Template, Widget};
use layout_object::StretchLayoutObject;
use enums::ParentType;

/// Use this layout widget to overlay its children (on z axis).
pub struct Stack;

impl Widget for Stack {
    fn create() -> Template {
        Template::default()
            .as_parent_type(ParentType::Multi)
            .with_layout_object(StretchLayoutObject)
            .with_debug_name("Stack")
    }
}