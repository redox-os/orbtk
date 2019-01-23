use super::Margin;
use crate::structs::Spacer;

// todo: docu / tests
#[derive(Copy, Clone, PartialEq)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
    Stretch,
}

impl Default for VerticalAlignment {
    fn default() -> Self {
        VerticalAlignment::Stretch
    }
}

impl VerticalAlignment {
    pub fn align_y(&self, available_height: f64, height: f64, margin: Margin) -> f64 {
        match self {
            VerticalAlignment::Bottom => available_height - height - margin.bottom(),
            VerticalAlignment::Center => (available_height - height) / 2.0,
            _ => margin.top(),
        }
    }

    pub fn align_height(&self, available_height: f64, height: f64, margin: Margin) -> f64 {
        match self {
            VerticalAlignment::Stretch => available_height - margin.top() - margin.bottom(),
            _ => height,
        }
    }
}
