use core::ops::{Add, Sub};

// todo: documentation
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    /// Returns the distance between this `Point` and the given `Point`.
    pub fn distance(&self, other: Self) -> f64 {
        ((self.x - other.x).powf(2.) + (self.y - other.y).powf(2.)).sqrt()
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn set_x(&mut self, x: impl Into<f64>) {
        self.x = x.into();
    }

    pub fn set_y(&mut self, y: impl Into<f64>) {
        self.y = y.into();
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
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

#[test]
fn test_sub() {
    const EXPECTED_RESULT: Point = Point { x: -3., y: 5. };
    const ERROR_MARGIN: f64 = 0.00001;

    let left_side = Point::new(5., 7.);
    let right_side = Point::new(8., 2.);

    let result = left_side - right_side;

    assert!((result.x - EXPECTED_RESULT.x).abs() < ERROR_MARGIN);
    assert!((result.y - EXPECTED_RESULT.y).abs() < ERROR_MARGIN);
}

#[test]
fn test_add() {
    const EXPECTED_RESULT: Point = Point { x: 13., y: 9. };
    const ERROR_MARGIN: f64 = 0.00001;

    let left_side = Point::new(5., 7.);
    let right_side = Point::new(8., 2.);

    let result = left_side + right_side;

    assert!((result.x - EXPECTED_RESULT.x).abs() < ERROR_MARGIN);
    assert!((result.y - EXPECTED_RESULT.y).abs() < ERROR_MARGIN);
}
