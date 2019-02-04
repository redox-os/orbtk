use crate::{
    layout::ScrollLayout,
    properties::{OffsetProperty, ScrollViewerMode, ScrollViewerModeProperty},
    widget::{Template, Widget},
};

/// The `ScrollViewer` (wip) represents a layout widget that adds vertical and horizontal offset to its parent.
/// It is used to scroll the content if the content's width or height is greater than the ScrollViewers width or height.
///
/// # Properties
///
/// * `offset` - Represents the vertical and horizontal scroll offset.
/// * `scroll_viewer_mode` - Scroll mode vertical / horizontal off the scroll viewer.
///
/// # Others
///
/// * `ScrollLayout` - Used to layout the widget.
pub struct ScrollViewer;

impl Widget for ScrollViewer {
    type Template = ScrollViewerTemplate;

    fn create() -> Self::Template {
        ScrollViewerTemplate::new()
            .offset(0.0)
            .scroll_viewer_mode(ScrollViewerMode::default())
            .layout(ScrollLayout::default())
            .debug_name("ScrollViewer")
    }
}

template!(
    ScrollViewerTemplate,
    [OffsetProperty, ScrollViewerModeProperty]
);
