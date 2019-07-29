use crate::{prelude::*, utils::*};

property!(
    /// `BorderThickness` describes the relative thickness of a border.
    BorderThickness(Thickness) : f64, (f64, f64), (f64, f64, f64, f64)
);

// --- Trait implementations ---

impl Spacer for BorderThickness {
    fn left(&self) -> f64 {
        self.0.left
    }

    fn set_left(&mut self, left: f64) {
        self.0.left = left;
    }

    fn top(&self) -> f64 {
        self.0.top
    }

    fn set_top(&mut self, top: f64) {
        self.0.top = top;
    }

    fn right(&self) -> f64 {
        self.0.right
    }

    fn set_right(&mut self, right: f64) {
        self.0.right = right;
    }

    fn bottom(&self) -> f64 {
        self.0.bottom
    }

    fn set_bottom(&mut self, bottom: f64) {
        self.0.bottom = bottom;
    }

    fn thickness(&self) -> Thickness {
        self.0
    }

    fn set_thickness<T: Into<Thickness>>(&mut self, thickness: T) {
        self.0 = thickness.into();
    }
}