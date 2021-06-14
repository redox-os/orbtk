/// The `ScrollMode` defines the mode of a scroll direction.
#[derive(Copy, Debug, Clone, PartialEq)]
pub enum ScrollMode {
    /// Scrolling will process by `ScrollViewer` logic
    Auto,

    /// Scrolling could be handled from outside. It will not be
    /// process by `ScrollViewer` logic.
    Custom,

    /// Scrolling will be disabled.
    Disabled,
}

impl Default for ScrollMode {
    fn default() -> Self {
        ScrollMode::Auto
    }
}

impl From<&str> for ScrollMode {
    fn from(s: &str) -> ScrollMode {
        match s {
            "Custom" | "custom" => ScrollMode::Custom,
            "Disabled" | "disabled" => ScrollMode::Disabled,
            _ => ScrollMode::Auto,
        }
    }
}

/// `ScrollViewerMode` describes the vertical and horizontal scroll
/// behavior of the `ScrollViewer`.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ScrollViewerMode {
    /// Vertical scroll mode.
    pub vertical: ScrollMode,

    /// Horizontal scroll mode.
    pub horizontal: ScrollMode,
}

// --- Conversions ---

impl From<(&str, &str)> for ScrollViewerMode {
    fn from(s: (&str, &str)) -> ScrollViewerMode {
        ScrollViewerMode {
            horizontal: ScrollMode::from(s.0),
            vertical: ScrollMode::from(s.1),
        }
    }
}

impl Default for ScrollViewerMode {
    fn default() -> ScrollViewerMode {
        ScrollViewerMode {
            vertical: ScrollMode::Auto,
            horizontal: ScrollMode::Auto,
        }
    }
}
