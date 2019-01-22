#[derive(Copy, Clone, PartialEq)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation::Vertical
    }
}
