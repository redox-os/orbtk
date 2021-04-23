use std::ops::{Add, Div, Sub};

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
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Size {
    width: u32,
    height: u32,
}

impl Size {
    /// Creates a new size.
    pub fn new(width: u32, height: u32) -> Self {
        Size { width, height }
    }

    /// Gets the width of the size.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Sets the width of the size.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    /// Gets the height of the size.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Sets the height of the size.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }
}

// Operations

impl Add<Size> for Size {
    type Output = Size;

    fn add(mut self, rhs: Size) -> Self::Output {
        self.width += rhs.width;
        self.height += rhs.height;

        self
    }
}

impl Sub<Size> for Size {
    type Output = Size;

    fn sub(mut self, rhs: Size) -> Self::Output {
        self.width -= rhs.width;
        self.height -= rhs.height;

        self
    }
}

impl Div<u32> for Size {
    type Output = Size;

    fn div(mut self, rhs: u32) -> Self::Output {
        self.width /= rhs;
        self.height /= rhs;
        self
    }
}

impl Div<Size> for u32 {
    type Output = Size;

    fn div(self, mut rhs: Size) -> Self::Output {
        rhs.width /= self;
        rhs.height /= self;
        rhs
    }
}

// --- Conversions ---

impl From<(u32, u32)> for Size {
    fn from(t: (u32, u32)) -> Self {
        Self {
            width: t.0,
            height: t.1,
        }
    }
}

impl From<u32> for Size {
    fn from(t: u32) -> Self {
        Size::new(t, t)
    }
}

impl From<i32> for Size {
    fn from(t: i32) -> Self {
        Size::new(t as u32, t as u32)
    }
}

impl From<(i32, i32)> for Size {
    fn from(s: (i32, i32)) -> Size {
        Size::from((s.0 as u32, s.1 as u32))
    }
}

// --- Conversions ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub() {
        let left_side = Size::new(8, 7);
        let right_side = Size::new(4, 2);

        let result = left_side - right_side;

        assert_eq!(result.width(), 4);
        assert_eq!(result.height(), 5);
    }

    #[test]
    fn test_add() {
        let expected_result = Size::new(13, 9);

        let left_side = Size::new(5, 7);
        let right_side = Size::new(8, 2);

        let result = left_side + right_side;

        assert_eq!(result.width(), 13);
        assert_eq!(result.height(), 9);
    }
}
