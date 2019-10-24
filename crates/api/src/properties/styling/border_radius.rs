use crate::prelude::*;

property!(
    /// `BorderRadius` describes the degree to which the corners of a Border are rounded.
    #[derive(Default)]
    BorderRadius(f64)
);

impl From<i32> for BorderRadius {
    fn from(s: i32) -> BorderRadius {
        BorderRadius(s as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into() {
        let border_radius: BorderRadius = 20.0.into();
        assert_eq!(border_radius.0, 20.0);
    }
}
