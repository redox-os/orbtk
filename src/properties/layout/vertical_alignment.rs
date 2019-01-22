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
    pub fn align_y(&self, parent_height: f64, height: f64, margin: Margin) -> f64 {
        match self {
            VerticalAlignment::Bottom => parent_height - height - margin.bottom(),
            VerticalAlignment::Center => (parent_height - height) / 2.0,
            _ => margin.top(),
        }
    }

    pub fn align_height(&self, parent_height: f64, height: f64, margin: Margin) -> f64 {
        match self {
            VerticalAlignment::Stretch => parent_height - margin.top() - margin.bottom(),
            _ => height,
        }
    }
}
