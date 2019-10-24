use crate::{prelude::*, utils};

property!(
    /// `Brush` describes drawing brush of a visual element.
    #[derive(Default)]
    Brush(utils::Brush) : &str,
    String
);

#[cfg(test)]
mod tests {
    use utils;
    use super::*;

    #[test]
    fn test_into() {
        let background: Brush = "#000000".into();
        assert_eq!(background.0, utils::Brush::SolidColor(utils::Color::rgb(0, 0, 0)));

        let background: Brush = "#ffffff".into();
        assert_eq!(
            background.0,
            utils::Brush::SolidColor(utils::Color::rgb(255, 255, 255))
        );
    }
}
