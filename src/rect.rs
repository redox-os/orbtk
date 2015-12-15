use super::Point;

#[derive(Clone, Copy, Debug, Default)]
pub struct Rect {
    /*
    pub x: isize,
    pub y: isize,
    */
    pub point: Option<Point>,
    pub width: usize,
    pub height: usize,
}

impl Rect {
    pub fn new(x: isize, y: isize, width: usize, height: usize) -> Rect {
        Rect {
            point: Some(Point::new(x, y)),
            width: width,
            height: height,
        }
    }

    pub fn contains(&self, p: Point) -> bool {
        if let Some(this) = self.point {
            p.x >= this.x && p.x < this.x + self.width as isize &&
            p.y >= this.y && p.y < this.y + self.height as isize
        }

        false
    }

    pub fn get_point(&self) -> Point {
        self.point.unwrap_or(Point::new(0, 0))
    }
}
