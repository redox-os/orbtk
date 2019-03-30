use super::Margin;
use crate::{enums::Alignment, structs::Spacer};

property!(
    /// `HorizontalAlignment` describes the vertical alignment of a widget.
    HorizontalAlignment(Alignment)
);

// --- Trait implementations ---

/// Used to align the position of a widget vertical.
pub trait HorizontalAlignmentExtension {
    /// Calculates the x position of the widget depending on the available width, the goal width
    /// margin and Horizontal alignment.
    fn align_x(&self, available_height: f64, height: f64, margin: Margin) -> f64;

    /// Calculates the width of the widget depending on the available width, the goal width
    /// margin and Horizontal alignment.
    fn align_width(&self, available_height: f64, height: f64, margin: Margin) -> f64;
}

impl HorizontalAlignmentExtension for HorizontalAlignment {
    fn align_x(&self, available_height: f64, height: f64, margin: Margin) -> f64 {
        self.0
            .align_position(available_height, height, margin.left(), margin.right())
    }

    fn align_width(&self, available_height: f64, height: f64, margin: Margin) -> f64 {
        self.0
            .align_measure(available_height, height, margin.left(), margin.right())
    }
}

// --- Conversions ---

impl From<&str> for HorizontalAlignment {
    fn from(s: &str) -> HorizontalAlignment {
        HorizontalAlignment(s.into())
    }
}
