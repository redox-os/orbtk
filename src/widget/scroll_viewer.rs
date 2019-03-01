use crate::{
    layout::ScrollLayout,
    properties::{OffsetProperty, ScrollViewerMode, ScrollViewerModeProperty},
    widget::{Template, Widget},
};

widget!(
    /// The `ScrollViewer` (wip) represents a layout widget that adds vertical and horizontal offset to its parent.
    ScrollViewer
    (OffsetProperty, ScrollViewerModeProperty)
);

impl Widget for ScrollViewer {
    fn create() -> Self {
        ScrollViewer::new()
            .offset(0.0)
            .scroll_viewer_mode(ScrollViewerMode::default())
            .layout(ScrollLayout::default())
            .debug_name("ScrollViewer")
    }
}
