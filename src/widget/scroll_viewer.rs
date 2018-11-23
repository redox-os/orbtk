use enums::ParentType;
use layout_object::ScrollLayoutObject;
use widget::{Offset, Template, Widget};

/// Use to scroll its content.
pub struct ScrollViewer;

impl Widget for ScrollViewer {
    fn template() -> Template {
        print!("ScrollViewer -> ");
        Template::default()
            .as_parent_type(ParentType::Single)
            .with_property(Offset::default())
            .with_layout_object(ScrollLayoutObject)
    }
}
