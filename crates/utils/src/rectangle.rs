use crate::Point;

/// Describes a new visual rectangle.
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Rectangle {
    /// Position of the rectangle.
    position: Point,

    /// Width of the rectangle.
    width: f64,

    /// Height of the rectangle.
    height: f64,
}

impl Rectangle {
    /// Create a new rectangle with the given parameters.
    pub fn new(position: impl Into<Point>, width: f64, height: f64) -> Self {
        Rectangle {
            position: position.into(),
            width,
            height,
        }
    }

    /// Gets x.
    pub fn x(&self) -> f64 {
        self.position.x()
    }

    /// Sets x.
    pub fn set_x(&mut self, x: f64) {
        self.position.set_x(x);
    }

    /// Gets y.
    pub fn y(&self) -> f64 {
        self.position.y()
    }

    /// Sets y.
    pub fn set_y(&mut self, y: f64) {
        self.position.set_y(y);
    }

    /// Gets position as `Point`.
    pub fn position(&self) -> Point {
        self.position
    }

    /// Sets position.
    pub fn set_position(&mut self, position: impl Into<Point>) {
        self.position = position.into();
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

    /// Checks if this rect contains the given `point`.
    pub fn contains(&self, point: impl Into<Point>) -> bool {
        let point: Point = point.into();
        point.x() >= self.x()
            && point.x() <= self.x() + self.width()
            && point.y() >= self.y()
            && point.y() <= self.y() + self.height()
    }

    /// Checks if this rect contains the given `rect`.
    pub fn contains_rect(&self, rect: &Rectangle) -> bool {
        let p1 = rect.position();
        let p2 = (p1.x() + rect.width(), p1.y() + rect.height());
        self.contains(p1) && self.contains(p2)
    }

    /// Checks if this rect intersects with the given `rect`.
    pub fn intersects(&self, rect: &Rectangle) -> bool {
        !(rect.x() >= (self.x() + self.width())
            || self.x() >= (rect.x() + rect.width())
            || rect.y() >= (self.y() + self.height())
            || self.y() >= (rect.y() + rect.height()))
    }
}

// --- Conversions ---

impl From<(i32, i32, i32, i32)> for Rectangle {
    fn from(t: (i32, i32, i32, i32)) -> Self {
        Rectangle::new((t.0 as f64, t.1 as f64), t.2 as f64, t.3 as f64)
    }
}

impl From<(f64, f64, f64, f64)> for Rectangle {
    fn from(t: (f64, f64, f64, f64)) -> Self {
        Rectangle::new((t.0, t.1), t.2, t.3)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_new() {
        let rect = Rectangle::new((5.0, 10.0), 20.0, 30.0);

        assert_eq!(rect.x(), 5.0);
        assert_eq!(rect.y(), 10.0);
        assert_eq!(rect.width(), 20.0);
        assert_eq!(rect.height(), 30.0);
    }

    #[test]
    fn test_contains() {
        let rect = Rectangle::new((5.0, 10.0), 20.0, 30.0);

        let p = Point::new(5.0, 10.0);
        assert!(rect.contains(p), "{:?}", p);

        let p = Point::new(25.0, 40.0);
        assert!(rect.contains(p), "{:?}", p);

        let p = Point::new(15.0, 15.0);
        assert!(rect.contains(p), "{:?}", p);

        let p = Point::new(30.0, 15.0);
        assert!(!rect.contains(p), "{:?}", p);
    }
}
