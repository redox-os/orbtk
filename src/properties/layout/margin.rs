use crate::structs::{Spacer, Thickness};

/// The `Margin` is used to define space around a widget.
#[derive(Default, Clone, Copy)]
pub struct Margin {
    value: Thickness,
}

impl Spacer for Margin {
    /// Gets left.
    fn left(&self) -> f64 {
        self.value.left
    }

    /// Sets left.
    fn set_left(&mut self, left: f64) {
        self.value.left = left;
    }

    /// Gets top.
    fn top(&self) -> f64 {
        self.value.top
    }

    /// Sets top.
    fn set_top(&mut self, top: f64) {
        self.value.top = top;
    }

    /// Gets right.
    fn right(&self) -> f64 {
        self.value.right
    }

    /// Sets right.
    fn set_right(&mut self, right: f64) {
        self.value.right = right;
    }

    /// Gets bottom.
    fn bottom(&self) -> f64 {
        self.value.bottom
    }

    /// Sets bottom.
    fn set_bottom(&mut self, bottom: f64) {
        self.value.bottom = bottom;
    }

    /// Gets thickness.
    fn thickness(&self) -> Thickness {
        self.value
    }

    /// Sets thickness
    fn set_thickness(&mut self, thickness: Thickness) {
        self.value = thickness;
    }
}
