#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Thickness {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

impl Thickness {
    pub fn new(left: f64, top: f64, right: f64, bottom: f64) -> Self {
        Thickness {
            left,
            top,
            right, 
            bottom,
        }
    }
}