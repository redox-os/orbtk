use layout_object::FlexLayoutObject;
use enums::Alignment;
use widget::{Template, Widget};
use enums::ParentType;

/// This layout widget orders its children horizontal.
pub struct Row;

impl Widget for Row {
    fn create() -> Template {
        Template::default()
            .as_parent_type(ParentType::Multi)
            .with_layout_object(FlexLayoutObject::new(Alignment::Horizontal)) 
            .with_debug_name("Row")
    }
}