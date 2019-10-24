use crate::{prelude::*, utils};

property!(
    /// `Point` describes an point property of a widget.
    #[derive(Default)]
    Point(utils::Point) : (f64, f64)
);