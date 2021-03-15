use derive_more::{Add, Constructor, From, Sub};
use std::ops::Div;

/// A `Size` specified by width and height.
///
/// # Examples
/// ```rust
/// # use orbtk_utils::Size;
/// let size = Size::new(10., 10.);
/// let other_size = Size::new(5., 7.);
/// let result = size - other_size;
///
/// assert_eq!(result.width(), 5.);
/// assert_eq!(result.height(), 3.);
/// ```
#[derive(Constructor, Add, Sub, Copy, From, Clone, Default, Debug, PartialEq)]
pub struct Size {
    width: f64,
    height: f64,
}

impl Size {
    /// Gets the width of the size.
    pub fn width(&self) -> f64 {
        self.width
    }

    /// Sets the width of the size.
    pub fn set_width(&mut self, width: f64) {
        self.width = width;
    }

    /// Gets the height of the size.
    pub fn height(&self) -> f64 {
        self.height
    }

    /// Sets the height of the size.
    pub fn set_height(&mut self, height: f64) {
        self.height = height;
    }
}

// Operations

impl Div<f64> for Size {
    type Output = Size;

    fn div(mut self, rhs: f64) -> Self::Output {
        self.width /= rhs;
        self.height /= rhs;
        self
    }
}

impl Div<Size> for f64 {
    type Output = Size;

    fn div(self, mut rhs: Size) -> Self::Output {
        rhs.width /= self;
        rhs.height /= self;
        rhs
    }
}

// --- Conversions ---

impl From<f64> for Size {
    fn from(t: f64) -> Self {
        Size::new(t, t)
    }
}

impl From<i32> for Size {
    fn from(t: i32) -> Self {
        Size::new(t as f64, t as f64)
    }
}

impl From<(i32, i32)> for Size {
    fn from(s: (i32, i32)) -> Size {
        Size::from((s.0 as f64, s.1 as f64))
    }
}

// --- Conversions ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub() {
        let expected_result = Size::new(-3., 5.);
        const ERROR_MARGIN: f64 = 0.00001;

        let left_side = Size::new(5., 7.);
        let right_side = Size::new(8., 2.);

        let result = left_side - right_side;

        assert!((result.width() - expected_result.width()).abs() < ERROR_MARGIN);
        assert!((result.height() - expected_result.height()).abs() < ERROR_MARGIN);
    }

    #[test]
    fn test_add() {
        let expected_result = Size::new(13., 9.);
        const ERROR_MARGIN: f64 = 0.00001;

        let left_side = Size::new(5., 7.);
        let right_side = Size::new(8., 2.);

        let result = left_side + right_side;

        assert!((result.width() - expected_result.width()).abs() < ERROR_MARGIN);
        assert!((result.height() - expected_result.height()).abs() < ERROR_MARGIN);
    }
}
