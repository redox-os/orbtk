use super::Margin;
use crate::structs::Spacer;

#[derive(Copy, Clone, PartialEq)]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
    Stretch,
}

impl Default for HorizontalAlignment {
    fn default() -> Self {
        HorizontalAlignment::Stretch
    }
}

impl HorizontalAlignment {
    pub fn align_x(&self, parent_width: f64, width: f64, margin: Margin) -> f64 {
        match self {
            HorizontalAlignment::Right => parent_width - width - margin.right(),
            HorizontalAlignment::Center => (parent_width - width) / 2.0,
            _ => margin.left(),
        }
    }

    pub fn align_width(&self, parent_width: f64, width: f64, margin: Margin) -> f64 {
        match self {
            HorizontalAlignment::Stretch => parent_width - margin.left() - margin.right(),
            _ => width,
        }
    }
}
