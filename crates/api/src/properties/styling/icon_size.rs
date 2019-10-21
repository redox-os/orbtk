use crate::prelude::*;

property!(
    /// `IconSize` describes the icon size of a text element.
    #[derive(Default)]
    IconSize(f64)
);

impl From<i32> for IconSize {
    fn from(s: i32) -> IconSize {
        IconSize(s as f64)
    }
}
