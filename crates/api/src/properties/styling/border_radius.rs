use crate::prelude::*;

property!(
    /// `IconSize` describes the degree to which the corners of a Border are rounded.
    #[derive(Default)]
    BorderRadius(f64)
);

impl From<i32> for BorderRadius {
    fn from(s: i32) -> BorderRadius {
        BorderRadius(s as f64)
    }
}
