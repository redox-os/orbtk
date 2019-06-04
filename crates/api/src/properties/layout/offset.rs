use crate::prelude::*;

property!(
    /// `Offset` describes the x- and y-axis offset of a widget.
    Offset((f64, f64))
);

impl From<f64> for Offset {
    fn from(t: f64) -> Self {
        Offset((t, t))
    }
}

impl From<(i32, i32)> for Offset {
    fn from(s: (i32, i32)) -> Offset {
        Offset::from((s.0 as f64, s.1 as f64))
    }
}
