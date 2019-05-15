use crate::prelude::*;

property!(
    /// `Pos` describes the position of an element.
    Pos(Point)
);

impl From<(f64, f64)> for Pos {
    fn from(t: (f64, f64)) -> Self {
        Pos::from(Point::new(t.0, t.1))
    }
}