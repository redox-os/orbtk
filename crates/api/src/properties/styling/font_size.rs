use crate::prelude::*;

property!(
    /// `FontSize` describes the font size of a text element.
    FontSize(f64)
);

impl From<i32> for FontSize {
    fn from(s: i32) -> FontSize {
        FontSize(s as f64)
    }
}
