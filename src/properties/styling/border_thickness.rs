use crate::structs::{Spacer, Thickness};

/// Represents relative thickness of a Border.
#[derive(Default, Clone, Copy)]
pub struct BorderThickness(pub Thickness);

property!(
    BorderThickness,
    BorderThicknessProperty,
    border_thickness,
    shared_border_thickness
);


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

    fn set_thickness(&mut self, thickness: Thickness) {
        self.0 = thickness;
    }
}

impl From<(f64, f64, f64, f64)> for BorderThickness {
    fn from(t: (f64, f64, f64, f64)) -> Self {
        BorderThickness(Thickness::new(t.0, t.1, t.2, t.3))
    }
}

impl From<(f64, f64)> for BorderThickness {
    fn from(t: (f64, f64)) -> Self {
        BorderThickness(Thickness::new(t.0, t.1, t.0, t.1))
    }
}

impl From<f64> for BorderThickness {
    fn from(t: f64) -> Self {
        BorderThickness(Thickness::new(t, t, t, t))
    }
}