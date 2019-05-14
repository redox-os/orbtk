use crate::prelude::*;

#[test]
fn test_into() {
    let background: Background = "#000000".into();
    assert_eq!(background.0, Brush::SolidColor(Color::rgb(0, 0, 0)));

    let background: Background = "#ffffff".into();
    assert_eq!(background.0, Brush::SolidColor(Color::rgb(255, 255, 255)));
}
