use crate::{prelude::*, utils::prelude::*};

property!(
    /// `Pos` describes the position of an element.
    #[derive(Default)]
    Pos(Point) : (f64, f64),
    (i32, i32)
);
