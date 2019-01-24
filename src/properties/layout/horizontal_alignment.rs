use super::Margin;
use crate::structs::Spacer;

/// Used to horizontal align a widget.
#[derive(Copy, Clone, PartialEq)]
pub enum HorizontalAlignment {
    /// Align left.
    Left,

    /// Align center.
    Center,

    /// Align right.
    Right,

    /// Stretch to available width.
    Stretch,
}

impl Default for HorizontalAlignment {
    fn default() -> Self {
        HorizontalAlignment::Stretch
    }
}

impl HorizontalAlignment {
    /// Calculates the x position of the widget depending on the available width, the goal width
    /// margin and horizontal alignment.
    pub fn align_x(&self, available_width: f64, width: f64, margin: Margin) -> f64 {
        match self {
            HorizontalAlignment::Right => available_width - width - margin.right(),
            HorizontalAlignment::Center => (available_width - width) / 2.0,
            _ => margin.left(),
        }
    }

    /// Calculates the width of the widget depending on the available width, the goal width
    /// margin and horizontal alignment.
    pub fn align_width(&self, available_width: f64, width: f64, margin: Margin) -> f64 {
        match self {
            HorizontalAlignment::Stretch => available_width - margin.left() - margin.right(),
            _ => width,
        }
    }
}

impl<T: Into<String>> From<T> for HorizontalAlignment {
    fn from(t: T) -> Self {
        match &t.into()[..] {
            "Right" | "right" => {
                HorizontalAlignment::Right
            },
            "Center" | "center" => {
                HorizontalAlignment::Right
            },
            "Left" | "left" => {
                HorizontalAlignment::Right
            },
            _ => HorizontalAlignment::Stretch
        }
    }
}