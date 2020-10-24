use crate::Point;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RelativeDir {
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
}

impl RelativeDir {
    /// Computes the start and end points of a line that crosses a given field in the `self` direction
    pub fn cross(&self, width: f64, height: f64) -> (Point, Point) {
        let (start, end);
        let mid_width = width / 2.0;
        let mid_height = height / 2.0;
        match self {
            RelativeDir::Top => {
                start = Point::new(mid_width, height);
                end = Point::new(mid_width, 0.0);
            }
            RelativeDir::TopRight => {
                start = Point::new(0.0, height);
                end = Point::new(width, 0.0);
            }
            RelativeDir::Right => {
                start = Point::new(0.0, mid_height);
                end = Point::new(width, mid_height);
            }
            RelativeDir::BottomRight => {
                start = Point::new(0.0, 0.0);
                end = Point::new(width, height);
            }
            RelativeDir::Bottom => {
                start = Point::new(mid_width, 0.0);
                end = Point::new(mid_width, height);
            }
            RelativeDir::BottomLeft => {
                start = Point::new(width, 0.0);
                end = Point::new(0.0, height);
            }
            RelativeDir::Left => {
                start = Point::new(width, mid_height);
                end = Point::new(0.0, mid_height);
            }
            RelativeDir::TopLeft => {
                start = Point::new(width, height);
                end = Point::new(0.0, 0.0);
            }
        }
        (start, end)
    }
}
