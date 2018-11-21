use std::ops::{Add, Sub};

/// This struct represents a non visual point.
#[derive(Copy, Clone, Debug, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Creates a new point.
    pub fn new(x: i32, y: i32) -> Self {
        Point { x: x, y: y }
    }
}

impl Add for Point {
    type Output = Point;

    /// Adds the given `other` point to self point.
    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    /// Subs the given `other` point from self point.
    fn sub(self, other: Point) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
