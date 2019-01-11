use crate::core::orbrender::{Border, RectangleBuilder, Thickness};

use super::{Selector, Theme};

pub trait FromTheme {
    fn from_theme(theme: &Theme, selector: &Selector) -> Self;
}

impl FromTheme for RectangleBuilder {
    fn from_theme(theme: &Theme, selector: &Selector) -> Self {
        RectangleBuilder::new()
            .with_background(theme.brush("background", selector))
            .with_border(Border::from_theme(theme, selector))
    }
}

impl FromTheme for Border {
    fn from_theme(theme: &Theme, selector: &Selector) -> Self {
        let left = theme.uint("border-left", selector) as f64;
        let right = theme.uint("border-left", selector) as f64;
        let top = theme.uint("border-left", selector) as f64;
        let bottom = theme.uint("border-left", selector) as f64;
        let width = theme.uint("border-width", selector) as f64;
        let brush = theme.brush("border-color", selector);

        let thickness = {
            if width > 0.0 {
                Thickness::new(width, width, width, width)
            } else {
                Thickness::new(left, top, right, bottom)
            }
        };

        Border::new(brush, thickness)
    }
}
