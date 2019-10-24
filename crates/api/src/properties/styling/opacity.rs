use crate::prelude::*;

property!(
    /// `Opacity` describes the opacity of a widget.
    #[derive(Default)]
    Opacity(f64)
);

impl From<i32> for Opacity {
    fn from(s: i32) -> Opacity {
        Opacity(s as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into() {
        let opacity: Opacity = 20.0.into();
        assert_eq!(opacity.0, 20.0);
    }
}
