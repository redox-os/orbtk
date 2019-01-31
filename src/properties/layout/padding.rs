use crate::structs::{Spacer, Thickness};

/// The `Padding` is used to define inner widget space.
#[derive(Default, Clone, Copy)]
pub struct Padding {
    value: Thickness,
}

property!(Padding, PaddingProperty, padding, shared_padding);

impl Spacer for Padding {
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

impl From<(f64, f64, f64, f64)> for Padding {
    fn from(t: (f64, f64, f64, f64)) -> Self {
        Padding {
            value: Thickness::new(t.0, t.1, t.2, t.3),
        }
    }
}

impl From<(f64, f64)> for Padding {
    fn from(t: (f64, f64)) -> Self {
        Padding {
            value: Thickness::new(t.0, t.1, t.0, t.1),
        }
    }
}

impl From<f64> for Padding {
    fn from(t: f64) -> Self {
        Padding {
            value: Thickness::new(t, t, t, t),
        }
    }
}