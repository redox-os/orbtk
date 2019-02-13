use super::Margin;
use crate::{enums::Alignment, structs::Spacer};

/// Used to horizontal align a widget.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct HorizontalAlignment(pub Alignment);

property!(
    HorizontalAlignment,
    HorizontalAlignmentProperty,
    horizontal_alignment,
    shared_horizontal_alignment
);

impl Default for HorizontalAlignment {
    fn default() -> Self {
        HorizontalAlignment(Alignment::Stretch)
    }
}

impl HorizontalAlignment {
    /// Calculates the x position of the widget depending on the available width, the goal width
    /// margin and horizontal alignment.
    pub fn align_x(&self, available_width: f64, width: f64, margin: Margin) -> f64 {
        self.0
            .align_position(available_width, width, margin.left(), margin.right())
    }

    /// Calculates the width of the widget depending on the available width, the goal width
    /// margin and horizontal alignment.
    pub fn align_width(&self, available_width: f64, width: f64, margin: Margin) -> f64 {
        self.0
            .align_measure(available_width, width, margin.left(), margin.right())
    }
}

impl From<&str> for HorizontalAlignment {
    fn from(t: &str) -> Self {
        HorizontalAlignment(t.into())
    }
}
