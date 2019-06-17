use crate::prelude::*;

property!(
    /// `Opacity` describes the opacity of a widget.
    Opacity(f64)
);

impl From<i32> for Opacity {
    fn from(s: i32) -> Opacity {
        Opacity(s as f64)
    }
}
