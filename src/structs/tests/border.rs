use super::*;

#[test]
fn test_brush() {
    let brush = Brush::from("#000000");

    let builder = BorderBuilder::new();
    let border = builder.brush(brush).build();

    let test_brush = Brush::from("#000000");
    assert_eq!(border.brush(), &test_brush);
}

#[test]
fn test_thickness() {
    let thickness = Thickness::new(0.0, 0.0, 0.0, 0.0);

    let builder = BorderBuilder::new();
    let border = builder.thickness(thickness).build();
    assert_eq!(border.thickness(), thickness);
}

#[test]
fn test_radius() {
    let radius = 0.0;

    let builder = BorderBuilder::new();
    let border = builder.radius(radius).build();
    assert_eq!(border.radius(), radius);
}

#[test]
fn test_set_brush() {
    let brush = Brush::from("#000000");

    let mut border = Border::default();
    border.set_brush(brush);

    let test_brush = Brush::from("#000000");
    assert_eq!(border.brush(), &test_brush);
}

#[test]
fn test_set_thickness() {
    let thickness = Thickness::new(0.0, 0.0, 0.0, 0.0);

    let mut border = Border::default();
    border.set_thickness(thickness);
    assert_eq!(border.thickness(), thickness);
}

#[test]
fn test_set_radius() {
    let radius = 0.0;

    let mut border = Border::default();
    border.set_radius(radius);
    assert_eq!(border.radius(), radius);
}
