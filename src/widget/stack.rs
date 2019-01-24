use crate::{
    enums::ParentType,
    layout::StretchLayout,
    widget::{Template, Widget},
};

/// The `Stack` represents a layout widget that is used to stack its children on the z-axis.
///
/// # Others
///
/// * `ParentType`- Mutli.
/// * `StretchLayout` - Used to layout the widget.
pub struct Stack;

impl Widget for Stack {
    fn create() -> Template {
        Template::default()
           .parent_type(ParentType::Multi)
            .layout(StretchLayout::default())
            .debug_name("Stack")
    }
}
