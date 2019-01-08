use crate::{
    enums::Alignment,
    enums::ParentType,
    layout_object::FlexLayoutObject,
    widget::{Template, Widget},
};

/// The `Row` represents a layout that orders its children horizontal.
///
/// # Others
///
/// * `ParentType`- Multi.
/// * `FlexLayoutObject` - Used to layout the widget.
pub struct Row;

impl Widget for Row {
    fn create() -> Template {
        Template::default()
            .as_parent_type(ParentType::Multi)
            .with_layout_object(FlexLayoutObject::new(Alignment::Horizontal))
            .with_debug_name("Row")
    }
}
