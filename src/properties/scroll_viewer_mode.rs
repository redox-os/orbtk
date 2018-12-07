use enums::ScrollMode;

/// The `ScrollViewerMode` struct is used to define the vertical and horizontal scroll behavior of the `ScrollViewer`.
#[derive(Default)]
pub struct ScrollViewerMode {
    /// Vertical scroll mode.
    pub vertical: ScrollMode,

    /// Horizontal scroll mode.
    pub horizontal: ScrollMode,
}

impl ScrollViewerMode {
    pub fn new(vertical: ScrollMode, horizontal: ScrollMode) -> Self {
        ScrollViewerMode {
            vertical,
            horizontal,
        }
    }
}
