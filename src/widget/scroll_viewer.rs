use crate::{
    enums::ParentType,
    layout::ScrollLayout,
    properties::{Offset, ScrollViewerMode},
    widget::{Template, Widget},
};

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
/// * `ScrollLayout` - Used to layout the widget.
pub struct ScrollViewer;

impl Widget for ScrollViewer {
    fn create() -> Template {
        Template::new()
            .parent_type(ParentType::Single)
            .property(Offset::default())
            .property(ScrollViewerMode::default())
            .layout(ScrollLayout::default())
            .debug_name("ScrollViewer")
    }
}
