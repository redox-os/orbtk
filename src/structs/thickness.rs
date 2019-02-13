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
