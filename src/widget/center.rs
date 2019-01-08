use crate::{
    enums::ParentType,
    layout_object::CenterLayoutObject,
    widget::{Template, Widget},
};

/// The `Center` represents a layout widget that center its child inside of it.
///
/// # Others
///
/// * `ParentType`- Single.
/// * `CenterLayoutObject` - Used to layout the widget.
pub struct Center;

impl Widget for Center {
    fn create() -> Template {
        Template::default()
            .as_parent_type(ParentType::Single)
            .with_layout_object(CenterLayoutObject)
            .with_debug_name("Center")
    }
}
