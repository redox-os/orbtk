use layout_object::FlexLayoutObject;
use enums::Alignment;
use widget::{Template, Widget};
use enums::ParentType;

/// The `Column` represents a layout that orders its children vertical.
/// 
/// # Others
/// 
/// * `ParentType`- Multi.
/// * `FlexLayoutObject` - Used to layout the widget.
pub struct Column;

impl Widget for Column {
    fn create() -> Template {
        Template::default()
            .as_parent_type(ParentType::Multi)
            .with_layout_object(FlexLayoutObject::new(Alignment::Vertical)) 
            .with_debug_name("Column")
    }
}
