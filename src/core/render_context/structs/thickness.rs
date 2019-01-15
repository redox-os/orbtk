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