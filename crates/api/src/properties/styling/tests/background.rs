use crate::{prelude::*, utils};

#[test]
fn test_into() {
    let background: Brush = "#000000".into();
    assert_eq!(background.0, utils::Brush::SolidColor(Color::rgb(0, 0, 0)));

    let background: Brush = "#ffffff".into();
    assert_eq!(background.0, utils::Brush::SolidColor(Color::rgb(255, 255, 255)));
}
