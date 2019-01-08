use crate::layout_object::FlexLayoutObject;
use crate::enums::Alignment;
use crate::widget::{Template, Widget};
use crate::enums::ParentType;

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