use crate::prelude::*;

property!(
    /// `Margin` describes the outer widget space.
    Margin(Thickness)
);

// --- Trait implementations ---

impl Spacer for Margin {
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

// --- Conversions ---

impl From<(f64, f64, f64, f64)> for Margin {
    fn from(t: (f64, f64, f64, f64)) -> Self {
        Margin::from(Thickness::new(t.0, t.1, t.2, t.3))
    }
}

impl From<(f64, f64)> for Margin {
    fn from(t: (f64, f64)) -> Self {
        Margin::from(Thickness::new(t.0, t.1, t.0, t.1))
    }
}

impl From<f64> for Margin {
    fn from(t: f64) -> Self {
        Margin::from(Thickness::new(t, t, t, t))
    }
}
