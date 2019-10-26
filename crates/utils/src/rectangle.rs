/// Describes a new visual rectangle.
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Rectangle {
    //// X position of the rectangle.
    pub x: f64,

    /// Y position of the rectangle.
    pub y: f64,

    /// Width of the rectangle.
    pub width: f64,

    /// Height of the rectangle.
    pub height: f64,
}

impl Rectangle {
    /// Create a new rectangle with the given parameters.
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Rectangle {
            x,
            y,
            width,
            height,
        }
    }

    /// Gets x.
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Sets x.
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    /// Gets y.
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Sets y.
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    /// Gets position with x and y.
    pub fn position(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    /// Sets position with x and y.
    pub fn set_position(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    /// Gets the width.
    pub fn width(&self) -> f64 {
        self.width
    }

    /// Sets the width.
    pub fn set_width(&mut self, width: f64) {
        self.width = width;
    }

    /// Gets the height.
    pub fn height(&self) -> f64 {
        self.height
    }

    /// Sets the height.
    pub fn set_height(&mut self, height: f64) {
        self.height = height;
    }

    /// Gets the size with width and height.
    pub fn size(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    /// Sets the size with width and height.
    pub fn set_size(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }

    /// Check if this rect contains the given `point`.
    pub fn contains(&self, point: (f64, f64)) -> bool {
        point.0 >= self.x
            && point.0 < self.x + self.width
            && point.1 >= self.y
            && point.1 < self.y + self.height
    }

    pub fn contains_rect(&self, rect: &Rectangle) -> bool {
        let p1 = rect.position();
        let p2 = (p1.0 + rect.width(), p1.1 + rect.height());
        self.contains(p1) && self.contains(p2)
    }

    pub fn intersects(&self, rect: &Rectangle) -> bool {
        !(rect.x() >= (self.x + self.width)
            || self.x >= (rect.x() + rect.width())
            || rect.y() >= (self.y + self.height)
            || self.y >= (rect.y() + rect.height()))
    }
}

// --- Conversions ---

impl From<(i32, i32, i32, i32)> for Rectangle {
    fn from(t: (i32, i32, i32, i32)) -> Self {
        Rectangle::new(t.0 as f64, t.1 as f64, t.2 as f64, t.3 as f64)
    }
}

impl From<(f64, f64, f64, f64)> for Rectangle {
    fn from(t: (f64, f64, f64, f64)) -> Self {
        Rectangle::new(t.0, t.1, t.2, t.3)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_new() {
        let rect = Rectangle::new(5.0, 10.0, 20.0, 30.0);

        assert_eq!(rect.x, 5.0);
        assert_eq!(rect.y, 10.0);
        assert_eq!(rect.width, 20.0);
        assert_eq!(rect.height, 30.0);
    }
}
