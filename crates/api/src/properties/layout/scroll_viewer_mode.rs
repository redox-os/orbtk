use crate::prelude::*;

/// `ScrollViewerMode` describes the vertical and horizontal scroll behavior of the `ScrollViewer`.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ScrollViewerModeValue {
    /// Vertical scroll mode.
    pub vertical: ScrollMode,

    /// Horizontal scroll mode.
    pub horizontal: ScrollMode,
}

// --- Conversions ---

impl From<(&str, &str)> for ScrollViewerModeValue {
    fn from(s: (&str, &str)) -> ScrollViewerModeValue {
        ScrollViewerModeValue {
            horizontal: ScrollMode::from(s.0),
            vertical: ScrollMode::from(s.1),
        }
    }
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
    ScrollViewerMode(ScrollViewerModeValue) : (&str, &str)
);
