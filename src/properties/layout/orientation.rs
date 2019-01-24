/// Used to define the orientation of the `Stack`.
#[derive(Copy, Clone, PartialEq)]
pub enum Orientation {
    /// Vertical orientation.
    Vertical,

    /// Horizontal orientation.
    Horizontal,
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation::Vertical
    }
}
