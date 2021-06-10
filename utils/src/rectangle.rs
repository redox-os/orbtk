use crate::{Point, Size};

/// A `Rectangle` is normally expressed as a top-left corner and a size
///
/// # Examples
/// ```rust
/// # use orbtk_utils::Rectangle;
/// let rectangle = Rectangle::new((0., 5.),(10., 7.));
///
/// assert_eq!(rectangle.x(), 0.);
/// assert_eq!(rectangle.y(), 5.);
/// assert_eq!(rectangle.width(), 10.);
/// assert_eq!(rectangle.height(), 7.);
/// ```
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Rectangle {
    /// Position of the rectangle.
    position: Point,

    size: Size,
}

impl Rectangle {
    /// Create a new rectangle with the given parameters.
    pub fn new(position: impl Into<Point>, size: impl Into<Size>) -> Self {
        Rectangle {
            position: position.into(),
            size: size.into(),
        }
    }

    /// Gets x.
    pub fn x(&self) -> f64 {
        self.position.x()
    }

    /// Sets x.
    pub fn set_x(&mut self, x: impl Into<f64>) {
        self.position.set_x(x);
    }

    /// Gets y.
    pub fn y(&self) -> f64 {
        self.position.y()
    }

    /// Sets y.
    pub fn set_y(&mut self, y: impl Into<f64>) {
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
        self.size.width()
    }

    /// Sets the width.
    pub fn set_width(&mut self, width: impl Into<f64>) {
        self.size.set_width(width.into());
    }

    /// Gets the height.
    pub fn height(&self) -> f64 {
        self.size.height()
    }

    /// Sets the height.
    pub fn set_height(&mut self, height: impl Into<f64>) {
        self.size.set_height(height.into());
    }

    /// Gets the size with width and height.
    pub fn size(&self) -> Size {
        self.size
    }

    /// Sets the size with width and height.
    pub fn set_size(&mut self, width: impl Into<f64>, height: impl Into<f64>) {
        self.size.set_width(width.into());
        self.size.set_height(height.into());
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
        !(rect.x() > (self.x() + self.width())
            || self.x() > (rect.x() + rect.width())
            || rect.y() > (self.y() + self.height())
            || self.y() > (rect.y() + rect.height()))
    }

    /// Joins this rectangle with another one, the result is
    /// a rectangle in which the two parents fit.
    pub fn join_with_rectangle(&mut self, other: &Rectangle) {
        if other.x() < self.x() {
            self.set_width(self.width() + self.x() - other.x());
            self.set_x(other.x());
        }
        if other.y() < self.y() {
            self.set_height(self.height() + self.y() - other.y());
            self.set_y(other.y());
        }
        if other.x() + other.width() > self.x() + self.width() {
            self.set_width(other.x() + other.width() - self.x());
        }
        if other.y() + other.height() > self.y() + self.height() {
            self.set_height(other.y() + other.height() - self.y());
        }
    }

    /// Extends this rectangle to cover the given point.
    pub fn join_with_point(&mut self, point: &Point) {
        if point.x() < self.x() {
            self.set_width(self.width() + self.x() - point.x());
            self.set_x(point.x());
        }
        if point.y() < self.y() {
            self.set_height(self.height() + self.y() - point.y());
            self.set_y(point.y());
        }
        if point.x() > self.x() + self.width() {
            self.set_width(point.x() - self.x());
        }
        if point.y() > self.y() + self.height() {
            self.set_height(point.y() - self.y());
        }
    }

    /// Box itself inside another rectangle
    pub fn box_into(&mut self, container: Rectangle) {
        if self.x() < container.x() {
            self.set_width(self.width() - (container.x() - self.x()));
            self.set_x(container.x());
        }
        if self.y() < container.y() {
            self.set_height(self.height() - (container.y() - self.y()));
            self.set_y(container.y());
        }
        if self.x() + self.width() > container.x() + container.width() {
            self.set_width(container.width() - container.x() + self.x());
        }
        if self.y() + self.height() > container.y() + container.height() {
            self.set_height(container.height() - container.y() + self.y());
        }
    }
}

// --- Conversions ---

impl From<(Point, Size)> for Rectangle {
    fn from(t: (Point, Size)) -> Self {
        Rectangle::new(t.0, t.1)
    }
}

impl From<(i32, i32, i32, i32)> for Rectangle {
    fn from(t: (i32, i32, i32, i32)) -> Self {
        Rectangle::new((t.0 as f64, t.1 as f64), (t.2 as f64, t.3 as f64))
    }
}

impl From<(f64, f64, f64, f64)> for Rectangle {
    fn from(t: (f64, f64, f64, f64)) -> Self {
        Rectangle::new((t.0, t.1), (t.2, t.3))
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_new() {
        let rect = Rectangle::new((5.0, 10.0), (20.0, 30.0));

        crate::f64_assert(rect.x(), 5.0);
        crate::f64_assert(rect.y(), 10.0);
        crate::f64_assert(rect.width(), 20.0);
        crate::f64_assert(rect.height(), 30.0);
    }

    #[test]
    fn test_contains() {
        let rect = Rectangle::new((5.0, 10.0), (20.0, 30.0));

        // Contains point in its origin
        let p = Point::new(5.0, 10.0);
        assert!(rect.contains(p), "{:?}", p);

        // Contains point in its bottom right corner
        let p = Point::new(25.0, 40.0);
        assert!(rect.contains(p), "{:?}", p);

        // Contains normal point
        let p = Point::new(15.0, 15.0);
        assert!(rect.contains(p), "{:?}", p);

        // Doesn't contain point with x out of rect
        let p = Point::new(30.0, 15.0);
        assert!(!rect.contains(p), "{:?}", p);

        // Doesn't contain point with y out of rect
        let p = Point::new(15.0, 50.0);
        assert!(!rect.contains(p), "{:?}", p);

        // Doesn't contain point with both x and y out of rect
        let p = Point::new(30.0, 40.0);
        assert!(!rect.contains(p), "{:?}", p);
    }

    #[test]
    fn test_contains_rect() {
        let rect = Rectangle::new((5.0, 10.0), (20.0, 30.0));

        // Contains itself
        let r = Rectangle::new((5.0, 10.0), (20.0, 30.0));
        assert!(rect.contains_rect(&r), "{:?}", r);

        // Contains rect on one of its edges
        let r = Rectangle::new((5.0, 20.0), (10.0, 20.0));
        assert!(rect.contains_rect(&r), "{:?}", r);

        // Contains rect on two of its edges
        let r = Rectangle::new((5.0, 10.0), (10.0, 20.0));
        assert!(rect.contains_rect(&r), "{:?}", r);

        // Contains rect completly inside
        let r = Rectangle::new((10.0, 20.0), (5.0, 10.0));
        assert!(rect.contains_rect(&r), "{:?}", r);

        // Does not contain rect partially inside
        let r = Rectangle::new((20.0, 25.0), (20.0, 30.0));
        assert!(!rect.contains_rect(&r), "{:?}", r);

        // Does not contain rect completely outside
        let r = Rectangle::new((50.0, 100.0), (20.0, 30.0));
        assert!(!rect.contains_rect(&r), "{:?}", r);
    }

    #[test]
    fn test_intersects() {
        let rect = Rectangle::new((5.0, 10.0), (20.0, 30.0));

        // Intersects with itself
        let r = Rectangle::new((5.0, 10.0), (20.0, 30.0));
        assert!(rect.intersects(&r), "{:?}", r);

        // Intersects with rect with origin on right edge
        let r = Rectangle::new((25.0, 10.0), (20.0, 30.0));
        assert!(rect.intersects(&r), "{:?}", r);

        // Intersects with rect with end on left edge
        let r = Rectangle::new((-15.0, 10.0), (20.0, 30.0));
        assert!(rect.intersects(&r), "{:?}", r);

        // Intersects with rect with origin on bottom edge
        let r = Rectangle::new((5.0, 40.0), (20.0, 30.0));
        assert!(rect.intersects(&r), "{:?}", r);

        // Intersects with rect with end on upper edge
        let r = Rectangle::new((5.0, -20.0), (20.0, 30.0));
        assert!(rect.intersects(&r), "{:?}", r);

        // Does not intersect with rect where origin is further
        // right than origin + width of this rect
        let r = Rectangle::new((30.0, 10.0), (20.0, 30.0));
        assert!(!rect.intersects(&r), "{:?}", r);

        // Does not intersect with rect where end + width is further
        // left than origin of this rect
        let r = Rectangle::new((-20.0, 10.0), (20.0, 30.0));
        assert!(!rect.intersects(&r), "{:?}", r);

        // Does not intersect with rect where origin is further
        // down than origin + width of this rect
        let r = Rectangle::new((5.0, 50.0), (20.0, 30.0));
        assert!(!rect.intersects(&r), "{:?}", r);

        // Does not intersect with rect where origin + height is further
        // up than origin of this rect
        let r = Rectangle::new((5.0, -30.0), (20.0, 30.0));
        assert!(!rect.intersects(&r), "{:?}", r);
    }
}
