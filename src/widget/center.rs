use layout_object::CenterLayoutObject;
use widget::{Template, Widget};
use enums::ParentType;

/// This layout widget centers its children within itself.
pub struct Center;

impl Widget for Center {
    fn create() -> Template {
        Template::default()
            .as_parent_type(ParentType::Single)
            .with_layout_object(CenterLayoutObject) 
            .with_debug_name("Center")
    }
}
