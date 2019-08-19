/// The `ScrollMode` defines the mode of a scroll direction.
#[derive(Copy, Debug, Clone, PartialEq)]
pub enum ScrollMode {
    /// Scrolling will process by `ScrollViewer` logic
    Auto,

    /// Scrolling could be handled from outside. It will not be process by `ScrollViewer` logic.
    Custom,

    /// Scrolling will be disabled.
    Disabled
}

impl Default for ScrollMode {
    fn default() -> Self {
        ScrollMode::Auto
    }
}

impl From<(&str)> for ScrollMode {
    fn from(s: &str) -> ScrollMode {
        match s {
            "Custom" | "custom" => ScrollMode::Custom,
            "Disabled" | "disabled" => ScrollMode::Disabled,
            _ => ScrollMode::Auto,
        }
    }
}
