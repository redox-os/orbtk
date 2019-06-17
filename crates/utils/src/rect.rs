/// Describes a new visual rectangle.
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Rect {
    //// X position of the rectangle.
    pub x: f64,

    /// Y position of the rectangle.
    pub y: f64,

    /// Width of the rectangle.
    pub width: f64,

    /// Height of the rectangle.
    pub height: f64,
}

impl Rect {
    /// Create a new rectangle with the given parameters.
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Rect {
            x,
            y,
            width,
            height,
        }
    }
}

/// Contains a set of getters and setters to read and write x and y.
pub trait Position {
    /// Gets x.
    fn x(&self) -> f64;

    /// Sets x.
    fn set_x(&mut self, x: f64);

    /// Gets y.
    fn y(&self) -> f64;

    /// Sets y.
    fn set_y(&mut self, y: f64);

    /// Gets position with x and y.
    fn position(&self) -> (f64, f64);

    /// Sets position with x and y.
    fn set_position(&mut self, x: f64, y: f64);
}

/// Contains a set of getters and setters to read and write with and height.
pub trait Size {
    /// Gets the width.
    fn width(&self) -> f64;

    /// Sets the width.
    fn set_width(&mut self, width: f64);

    /// Gets the height.
    fn height(&self) -> f64;

    /// Sets the height.
    fn set_height(&mut self, height: f64);

    /// Gets the size with width and height.
    fn size(&self) -> (f64, f64);

    /// Sets the size with width and height.
    fn set_size(&mut self, width: f64, height: f64);
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_new() {
        let rect = Rect::new(5.0, 10.0, 20.0, 30.0);

        assert_eq!(rect.x, 5.0);
        assert_eq!(rect.y, 10.0);
        assert_eq!(rect.width, 20.0);
        assert_eq!(rect.height, 30.0);
    }
}
