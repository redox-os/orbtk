// todo: documentation
#[derive(Copy, Clone, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

property!(Point, PointProperty, point, shared_point);