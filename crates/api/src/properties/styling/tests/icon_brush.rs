use crate::prelude::*;

#[test]
fn test_into() {
    let icon_brush: IconBrush = "#000000".into();
    assert_eq!(icon_brush.0, Brush::SolidColor(Color::rgb(0, 0, 0)));

    let icon_brush: IconBrush = "#ffffff".into();
    assert_eq!(icon_brush.0, Brush::SolidColor(Color::rgb(255, 255, 255)));
}
