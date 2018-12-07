use enums::ParentType;
use layout_object::ScrollLayoutObject;
use widget::{Template, Widget};
use properties::{Offset, ScrollViewerMode};

/// The `ScrollViewer` represents a layout widget that adds vertial and horizontal offset to its perent. 
/// It is used to scroll the content if the content's width or height is greater than the ScrollViewers width or height.
/// 
/// # Properties
/// 
/// * `Offset` - Represents the vertial and horizontal scroll offset.
/// * `ScrollMode` - Scroll mode vertical / horizontal off the scroll viewr.
/// 
/// # Others
/// 
/// * `ParentType`- Single.
/// * `ScrollLayoutObject` - Used to layout the widget.
pub struct ScrollViewer;

impl Widget for ScrollViewer {
    fn create() -> Template {
        Template::default()
            .as_parent_type(ParentType::Single)
            .with_property(Offset::default())
            .with_property(ScrollViewerMode::default())
            .with_layout_object(ScrollLayoutObject::default())
            .with_debug_name("ScrollViewer")
    }
}
