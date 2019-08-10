// todo: documentation
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

impl From<f64> for Point {
    fn from(t: f64) -> Self {
        Point::new(t, t)
    }
}

impl From<(f64, f64)> for Point {
    fn from(t: (f64, f64)) -> Self {
        Point::new(t.0, t.1)
    }
}

impl From<(i32, i32)> for Point {
    fn from(s: (i32, i32)) -> Point {
        Point::from((s.0 as f64, s.1 as f64))
    }
}
