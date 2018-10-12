use super::Point;

#[derive(Clone, Copy, Debug, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Default)]
pub struct LocalBounds {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Default)]
pub struct GlobalBounds {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Rect {
        Rect {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }

    // Get the top left point of this rect
    pub fn point(&self) -> Point {
        Point::new(self.x, self.y)
    }

    // Check if this rect contains a point
    pub fn contains(&self, p: Point) -> bool {
        p.x >= self.x
            && p.x < self.x + self.width as i32
            && p.y >= self.y
            && p.y < self.y + self.height as i32
    }

    // Check if this rect contains another rect
    pub fn contains_rect(&self, r: &Rect) -> bool {
        let p1 = r.point();
        let p2 = p1 + Point::new(r.width as i32, r.height as i32);
        self.contains(p1) && self.contains(p2)
    }

    // Check if this rect intersects another rect
    pub fn intersects(&self, r: &Rect) -> bool {
        !(r.x >= (self.x + self.width as i32)
            || self.x >= (r.x + r.width as i32)
            || r.y >= (self.y + self.height as i32)
            || self.y >= (r.y + r.height as i32))
    }
}
