use crate::{prelude::*, utils::prelude::*};

property!(
    /// `Pos` describes the position of an element.
    Pos(Point)
);

impl From<(f64, f64)> for Pos {
    fn from(t: (f64, f64)) -> Self {
        Pos::from(Point::new(t.0, t.1))
    }
}

impl From<(i32, i32)> for Pos {
    fn from(s: (i32, i32)) -> Pos {
        Pos::from((s.0 as f64, s.1 as f64))
    }
}