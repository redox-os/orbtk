pub use orbgl_shapes::prelude::{Shape, Rectangle, ImageElement};

use crate::{
    structs::{Bordered, Position, Size, Thickness},
};

use super::{Selector, Theme};

pub trait UpdateableShape: Shape {
    fn update_by_selector(&mut self, selector: &mut Selector, theme: &Theme);
    fn update_by_bounds(&mut self, x: f64, y: f64, width: f64, height: f64);
}

impl UpdateableShape for Rectangle {
    fn update_by_selector(&mut self, selector: &mut Selector, theme: &Theme) {
        if !selector.dirty() {
            return;
        }

        let left = theme.uint("border-left", selector) as f64;
        let right = theme.uint("border-right", selector) as f64;
        let top = theme.uint("border-top", selector) as f64;
        let bottom = theme.uint("border-bottom", selector) as f64;
        let width = theme.uint("border-width", selector) as f64;

        // todo radius
        let radius = theme.uint("border-radius", selector) as f64;
        let brush = theme.brush("border-color", selector);
        let background = theme.brush("background", selector);

        let thickness = {
            if width > 0.0 {
                Thickness::new(width, width, width, width)
            } else {
                Thickness::new(left, top, right, bottom)
            }
        };

        self.set_border_thickness(thickness);
        self.set_border_brush(brush);
        self.set_border_radius(radius);
        self.set_background(background);
    }

    fn update_by_bounds(&mut self, x: f64, y: f64, width: f64, height: f64) {
        // todo check changes
        self.set_position(x, y);
        self.set_size(width, height);
    }
}

impl Into<Box<dyn UpdateableShape>> for Rectangle {
    fn into(self) -> Box<dyn UpdateableShape> {
        Box::new(self)
    }
}

impl UpdateableShape for ImageElement {
    fn update_by_selector(&mut self, selector: &mut Selector, theme: &Theme) {}

    fn update_by_bounds(&mut self, x: f64, y: f64, width: f64, height: f64) {
        // todo check changes
        self.set_position(x, y);
        self.set_size(width, height);
    }
}

impl Into<Box<dyn UpdateableShape>> for ImageElement {
    fn into(self) -> Box<dyn UpdateableShape> {
        Box::new(self)
    }
}
