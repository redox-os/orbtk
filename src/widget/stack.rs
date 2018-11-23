use widget::{Template, Widget};
use enums::ParentType;

/// Use this layout widget to overlay its children (on z axis).
pub struct Stack;

impl Widget for Stack {
    fn template() -> Template {
        print!("Stack -> ");
        Template::default()
            .as_parent_type(ParentType::Multi)
    }
}