use super::Margin;
use crate::{enums::Alignment, structs::Spacer};

/// Used to Vertical align a widget.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct VerticalAlignment(pub Alignment);

property!(
    VerticalAlignment,
    VerticalAlignmentProperty,
    vertical_alignment,
    shared_vertical_alignment
);

impl Default for VerticalAlignment {
    fn default() -> Self {
        VerticalAlignment(Alignment::Stretch)
    }
}

impl VerticalAlignment {
    /// Calculates the y position of the widget depending on the available height, the goal height
    /// margin and Vertical alignment.
    pub fn align_y(&self, available_height: f64, height: f64, margin: Margin) -> f64 {
        self.0.align_position(available_height, height, margin.top(), margin.bottom())
    }

    /// Calculates the height of the widget depending on the available height, the goal height
    /// margin and Vertical alignment.
    pub fn align_height(&self, available_height: f64, height: f64, margin: Margin) -> f64 {
        self.0.align_measure(available_height, height,margin.top(), margin.bottom())
    }
}

impl From<&str> for VerticalAlignment {
    fn from(t: &str) -> Self {
        VerticalAlignment(t.into())
    }
}
