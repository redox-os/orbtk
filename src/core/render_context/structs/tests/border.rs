use super::*;

#[test]
fn with_brush() {
    let brush = Brush::SolidColor("#000000".to_string());

    let builder = BorderBuilder::new();
    let border = builder.with_brush(brush).build();

    let test_brush = Brush::SolidColor("#000000".to_string());
    assert_eq!(border.brush(), &test_brush);
}

#[test]
fn with_thickness() {
    let thickness = Thickness::new(0.0, 0.0, 0.0, 0.0);

    let builder = BorderBuilder::new();
    let border = builder.with_thickness(thickness).build();
    assert_eq!(border.thickness(), thickness);
}

#[test]
fn with_radius() {
    let radius = 0.0;

    let builder = BorderBuilder::new();
    let border = builder.with_radius(radius).build();
    assert_eq!(border.radius(), radius);
}

#[test]
fn set_brush() {
    let brush = Brush::SolidColor("#000000".to_string());

    let mut border = Border::default();
    border.set_brush(brush);

    let test_brush = Brush::SolidColor("#000000".to_string());
    assert_eq!(border.brush(), &test_brush);
}

#[test]
fn set_thickness() {
    let thickness = Thickness::new(0.0, 0.0, 0.0, 0.0);

    let mut border = Border::default();
    border.set_thickness(thickness);
    assert_eq!(border.thickness(), thickness);
}

#[test]
fn set_radius() {
    let radius = 0.0;

    let mut border = Border::default();
    border.set_radius(radius);
    assert_eq!(border.radius(), radius);
}
