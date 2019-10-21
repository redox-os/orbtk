use crate::{prelude::*, utils::prelude::*};

property!(
    /// `Delta` describes the x- and y-axis (wheel, scroll) delta of a widget.
    #[derive(Default)]
    Delta(Point) : f64,
    (i32, i32),
    (f64, f64)
);
