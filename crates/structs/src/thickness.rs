/// Used to describes a thickness e.g a border thickness.
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Thickness {
    /// Left of thickness.
    pub left: f64,

    /// Top of thickness.
    pub top: f64,

    /// Right of thickness.
    pub right: f64,

    /// Bottom of thickness.
    pub bottom: f64,
}

impl Thickness {
    /// Create a new thickness with the given parameters.
    pub fn new(left: f64, top: f64, right: f64, bottom: f64) -> Self {
        Thickness {
            left,
            top,
            right,
            bottom,
        }
    }
}

// --- Trait implementations ---

impl Spacer for Thickness {
    fn left(&self) -> f64 {
        self.left
    }

    fn set_left(&mut self, left: f64) {
        self.left = left;
    }

    fn top(&self) -> f64 {
        self.top
    }

    fn set_top(&mut self, top: f64) {
        self.top = top;
    }

    fn right(&self) -> f64 {
        self.right
    }

    fn set_right(&mut self, right: f64) {
        self.right = right;
    }

    fn bottom(&self) -> f64 {
        self.bottom
    }

    fn set_bottom(&mut self, bottom: f64) {
        self.bottom = bottom;
    }

    fn thickness(&self) -> Thickness {
        self.clone()
    }

    fn set_thickness<T: Into<Thickness>>(&mut self, thickness: T) {
        let other = thickness.into();

        self.set_left(other.left());
        self.set_top(other.top());
        self.set_right(other.right());
        self.set_bottom(other.bottom());
    }
}

impl From<(f64, f64, f64, f64)> for Thickness {
    fn from(t: (f64, f64, f64, f64)) -> Self {
        Thickness::new(t.0, t.1, t.2, t.3)
    }
}

impl From<(f64, f64)> for Thickness {
    fn from(t: (f64, f64)) -> Self {
        Thickness::new(t.0, t.1, t.0, t.1)
    }
}

impl From<f64> for Thickness {
    fn from(t: f64) -> Self {
        Thickness::new(t, t, t, t)
    }
}

// todo:  documentation
pub trait Spacer {
    /// Gets left.
    fn left(&self) -> f64;

    /// Sets left.
    fn set_left(&mut self, left: f64);

    /// Gets top.
    fn top(&self) -> f64;

    /// Sets top.
    fn set_top(&mut self, top: f64);

    /// Gets right.
    fn right(&self) -> f64;

    /// Sets right.
    fn set_right(&mut self, right: f64);

    /// Gets bottom.
    fn bottom(&self) -> f64;

    /// Sets bottom.
    fn set_bottom(&mut self, bottom: f64);

    /// Gets thickness.
    fn thickness(&self) -> Thickness;

    /// Sets thickness.
    fn set_thickness<T: Into<Thickness>>(&mut self, thickness: T);
}