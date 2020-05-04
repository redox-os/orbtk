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

    /// Returns the distance between this `Point` and the given `Point`.
    pub fn distance(&self, other: Self) -> f64 {
        ((self.x - other.x).powf(2.) + (self.y - other.y).powf(2.)).sqrt()
    }
}

impl From<f64> for Point {
    fn from(t: f64) -> Self {
        Point::new(t, t)
    }
}

impl From<i32> for Point {
    fn from(t: i32) -> Self {
        Point::new(t as f64, t as f64)
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

#[test]
fn test_distance() {
    const EXPECTED_RESULT: f64 = 9.48683;
    const ERROR_MARGIN: f64 = 0.00001;

    let point_positive = Point::new(1., 5.);
    let point_negative = Point::new(-2., -4.);

    assert!(((point_positive.distance(point_negative) - EXPECTED_RESULT).abs() < ERROR_MARGIN));
    assert!(((point_negative.distance(point_positive) - EXPECTED_RESULT).abs() < ERROR_MARGIN));
}
