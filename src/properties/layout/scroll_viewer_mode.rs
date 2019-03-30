use super::ScrollMode;

/// `ScrollViewerMode` describes the vertical and horizontal scroll behavior of the `ScrollViewer`.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ScrollViewerModeValue {
    /// Vertical scroll mode.
    pub vertical: ScrollMode,

    /// Horizontal scroll mode.
    pub horizontal: ScrollMode,
}

impl Default for ScrollViewerModeValue {
    fn default() -> ScrollViewerModeValue {
        ScrollViewerModeValue {
            vertical: ScrollMode::Auto,
            horizontal: ScrollMode::Auto,
        }
    }
}

property!(
    /// `ScrollViewerMode` describes the vertical and horizontal scroll behavior of the `ScrollViewer`.
    ScrollViewerMode(ScrollViewerModeValue)
);

// --- Conversions ---

impl From<(&str, &str)> for ScrollViewerMode {
    fn from(s: (&str, &str)) -> ScrollViewerMode {
        ScrollViewerMode(ScrollViewerModeValue {
            vertical: ScrollMode::from(s.0),
            horizontal: ScrollMode::from(s.1),
        })
    }
}
