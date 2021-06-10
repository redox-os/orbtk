use crate::Size;
use derive_more::{Add, Constructor, From, Sub};
use std::ops::{Add, Div, Mul, Neg};

/// A `Point` is specified by a x coordinate and an y coordinate.
///
/// # Examples
/// ```rust
/// # use orbtk_utils::Point;
/// let point = Point::new(10., 10.);
/// let other_point = Point::new(5., 7.);
/// let result = point - other_point;
///
/// assert_eq!(result.x(), 5.);
/// assert_eq!(result.y(), 3.);
/// ```
#[derive(Constructor, Add, Sub, Copy, From, Clone, Default, Debug, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    /// Returns the distance between this `Point` and the given `Point`.
    pub fn distance(&self, other: Self) -> f64 {
        ((self.x - other.x).powf(2.) + (self.y - other.y).powf(2.)).sqrt()
    }

    /// Gets the x position of the point.
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Sets the x position of the point.
    pub fn set_x(&mut self, x: impl Into<f64>) {
        self.x = x.into();
    }

    /// Gets the y position of the point.
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Sets the y position of the point.
    pub fn set_y(&mut self, y: impl Into<f64>) {
        self.y = y.into();
    }

    // Does a component-wise `min` operation between this point and another point
    pub fn min(self, other: impl Into<Point>) -> Point {
        let other = other.into();
        Point {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    // Does a component-wise `max` operation between this point and another point
    pub fn max(self, other: impl Into<Point>) -> Point {
        let other = other.into();
        Point {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    // Calculate the component-wise square root of this point
    pub fn sqrt(mut self) -> Point {
        self.x = self.x.sqrt();
        self.y = self.y.sqrt();
        self
    }

    // Calculate the component-wise absolute value of this point
    pub fn abs(mut self) -> Point {
        self.x = self.x.abs();
        self.y = self.y.abs();
        self
    }

    // Component-wise constraints this point between two values
    pub fn clamp(mut self, min: f64, max: f64) -> Point {
        self.x = self.x.max(min).min(max);
        self.y = self.y.max(min).min(max);
        self
    }
}

// Component-wise operations

impl Add<Size> for Point {
    type Output = Point;

    fn add(mut self, rhs: Size) -> Self::Output {
        self.x += rhs.width();
        self.y += rhs.height();
        self
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self.x *= rhs;
        self.y *= rhs;
        self
    }
}

impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, mut rhs: Point) -> Self::Output {
        rhs.x *= self;
        rhs.y *= self;
        rhs
    }
}

impl Mul<Point> for Point {
    type Output = Point;

    fn mul(mut self, rhs: Point) -> Self::Output {
        self.x *= rhs.x();
        self.y *= rhs.y();
        self
    }
}

impl Div<Point> for Point {
    type Output = Point;

    fn div(mut self, rhs: Point) -> Self::Output {
        self.x /= rhs.x();
        self.y /= rhs.y();
        self
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(mut self) -> Self::Output {
        self.x = -self.x();
        self.y = -self.y();
        self
    }
}

// --- Conversions ---

impl From<Size> for Point {
    fn from(s: Size) -> Self {
        Self::new(s.width(), s.height())
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

impl From<(i32, i32)> for Point {
    fn from(s: (i32, i32)) -> Point {
        Point::from((s.0 as f64, s.1 as f64))
    }
}

// --- Conversions ---

#[cfg(test)]
mod tests {
    use super::*;

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
}
