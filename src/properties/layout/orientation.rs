/// Used to define the orientation of the `Stack`.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Orientation {
    /// Vertical orientation.
    Vertical,

    /// Horizontal orientation.
    Horizontal,
}

property!(Orientation, OrientationProperty, orientation, shared_orientation);

impl Default for Orientation {
    fn default() -> Self {
        Orientation::Vertical
    }
}

impl From<&str> for Orientation {
    fn from(t: &str) -> Self {
        match t {
            "Horizontal" | "horizontal" => Orientation::Horizontal,
            _ => Orientation::Vertical,
        }
    }
}