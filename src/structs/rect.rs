use super::Point;

/// This sturct represents an non visual rectangle.
#[derive(Clone, Copy, Debug, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    /// Creates a new rectangle.
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Rect {
        Rect {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }

    /// Get the top left point of the rect.
    pub fn point(&self) -> Point {
        Point::new(self.x, self.y)
    }

    /// Check if this rect contains the given `point`.
    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.x
            && point.x < self.x + self.width as i32
            && point.y >= self.y
            && point.y < self.y + self.height as i32
    }

    /// Check if this rect contains another `rect`.
    pub fn contains_rect(&self, rect: &Rect) -> bool {
        let p1 = rect.point();
        let p2 = p1 + Point::new(rect.width as i32, rect.height as i32);
        self.contains(p1) && self.contains(p2)
    }

    // Check if this rect intersects another `rect`.
    pub fn intersects(&self, rect: &Rect) -> bool {
        !(rect.x >= (self.x + self.width as i32)
            || self.x >= (rect.x + rect.width as i32)
            || rect.y >= (self.y + self.height as i32)
            || self.y >= (rect.y + rect.height as i32))
    }
}
