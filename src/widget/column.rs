use layout_object::FlexLayoutObject;
use enums::Alignment;
use widget::{Template, Widget};
use enums::ParentType;

/// This layout widget orders its children vertical.
pub struct Column;

impl Widget for Column {
    fn create() -> Template {
        Template::default()
            .as_parent_type(ParentType::Multi)
            .with_layout_object(FlexLayoutObject::new(Alignment::Vertical)) 
            .with_debug_name("Column")
    }
}
