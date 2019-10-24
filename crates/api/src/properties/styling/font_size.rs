use crate::prelude::*;

property!(
    /// `FontSize` describes the font size of a text element.
    #[derive(Default)]
    FontSize(f64)
);

impl From<i32> for FontSize {
    fn from(s: i32) -> FontSize {
        FontSize(s as f64)
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
