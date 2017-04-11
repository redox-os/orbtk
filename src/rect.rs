use super::Point;

#[derive(Clone, Copy, Debug, Default)]
pub struct Rect {
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

    pub fn intersects(&self, r: &Rect) -> bool {
        ! (
            r.x > (self.x + self.width as i32) ||
            (r.x + r.width as i32) < self.x ||
            r.y > (self.y + self.height as i32) ||
            (r.y + r.height as i32) < self.y
        )
    }

    pub fn contains(&self, p: Point) -> bool {
        p.x >= self.x && p.x < self.x + self.width as i32 && p.y >= self.y &&
        p.y < self.y + self.height as i32
    }

    pub fn point(&self) -> Point {
        Point::new(self.x, self.y)
    }
}
