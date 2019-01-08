use crate::{
    enums::Alignment,
    enums::ParentType,
    layout::FlexLayout,
    widget::{Template, Widget},
};

/// The `Column` represents a layout that orders its children vertical.
///
/// # Others
///
/// * `ParentType`- Multi.
/// * `FlexLayout` - Used to layout the widget.
pub struct Column;

impl Widget for Column {
    fn create() -> Template {
        Template::default()
            .as_parent_type(ParentType::Multi)
            .with_layout(FlexLayout::new(Alignment::Vertical))
            .with_debug_name("Column")
    }
}
