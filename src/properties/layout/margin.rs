use crate::structs::{Spacer, Thickness};

/// The `Margin` is used to define space around a widget.
#[derive(Default, Clone, Copy)]
pub struct Margin {
    value: Thickness,
}

// todo: tests

impl Spacer for Margin {
    fn left(&self) -> f64 {
        self.value.left
    }

    fn set_left(&mut self, left: f64) {
        self.value.left = left;
    }

    fn top(&self) -> f64 {
        self.value.top
    }

    fn set_top(&mut self, top: f64) {
        self.value.top = top;
    }

    fn right(&self) -> f64 {
        self.value.right
    }

    fn set_right(&mut self, right: f64) {
        self.value.right = right;
    }

    fn bottom(&self) -> f64 {
        self.value.bottom
    }

    fn set_bottom(&mut self, bottom: f64) {
        self.value.bottom = bottom;
    }

    fn thickness(&self) -> Thickness {
        self.value
    }

    fn set_thickness(&mut self, thickness: Thickness) {
        self.value = thickness;
    }
}
