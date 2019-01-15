use super::*;
use crate::core::{Border, Bordered, Brush, Position, Rect, Size, Thickness};

#[test]
fn test_with_position() {
    let position = (5.0, 10.0);

    let builder = RectangleBuilder::new();
    let rectangle = builder.with_position(position.0, position.1).build();

    assert_eq!(rectangle.position(), position);
}

#[test]
fn test_with_background() {
    let background = Brush::SolidColor("#000000".to_string());

    let builder = RectangleBuilder::new();
    let rectangle = builder.with_background(background.clone()).build();

    assert_eq!(rectangle.background(), &background);
}

#[test]
fn test_with_size() {
    let size = (5.0, 10.0);

    let builder = RectangleBuilder::new();
    let rectangle = builder.with_size(size.0, size.1).build();

    assert_eq!(rectangle.size(), size);
}

#[test]
fn test_with_rect() {
    let rect = (5.0, 10.0, 20.0, 30.0);

    let builder = RectangleBuilder::new();
    let rectangle = builder.with_rect(rect.0, rect.1, rect.2, rect.3).build();

    assert_eq!(rectangle.position(), ((rect.0, rect.1)));
    assert_eq!(rectangle.size(), ((rect.2, rect.3)));
}

#[test]
fn test_with_border() {
    let border = Border::create().with_radius(5.0).build();

    let builder = RectangleBuilder::new();
    let rectangle = builder.with_border(border.clone()).build();

    assert_eq!(rectangle.border(), &border);
}

#[test]
fn test_set_background() {
    let background = Brush::SolidColor("#000000".to_string());

    let builder = RectangleBuilder::new();
    let mut rectangle = builder.build();

    rectangle.set_background(background.clone());

    assert_eq!(rectangle.background(), &background);
}

#[test]
fn test_build_path() {
    let builder = RectangleBuilder::new();
    let mut rectangle = builder.build();

    rectangle.build_path();
    assert_eq!(rectangle.path().len(), 2);

    let builder = RectangleBuilder::new();
    let mut rectangle = builder
        .with_border(
            Border::create()
                .with_thickness(Thickness::new(10.0, 10.0, 10.0, 10.0))
                .build(),
        )
        .build();
    rectangle.build_path();
    assert_eq!(rectangle.path().len(), 4);

    let builder = RectangleBuilder::new();
    let mut rectangle = builder
        .with_border(Border::create().with_radius(10.0).build())
        .build();
    rectangle.build_path();
    assert_eq!(rectangle.path().len(), 8);

    let builder = RectangleBuilder::new();
    let mut rectangle = builder
        .with_border(
            Border::create()
                .with_thickness(Thickness::new(10.0, 10.0, 10.0, 10.0))
                .with_radius(10.0)
                .build(),
        )
        .build();
    rectangle.build_path();
    assert_eq!(rectangle.path().len(), 16);
}

#[test]
fn test_set_width() {
    let width = 10.0;
    let builder = RectangleBuilder::new();
    let mut rectangle = builder.build();

    rectangle.set_width(width);

    assert_eq!(rectangle.width(), width);
}

#[test]
fn test_set_height() {
    let height = 10.0;
    let builder = RectangleBuilder::new();
    let mut rectangle = builder.build();

    rectangle.set_height(height);

    assert_eq!(rectangle.height(), height);
}

#[test]
fn test_set_size() {
    let size = (10.0, 20.0);
    let builder = RectangleBuilder::new();
    let mut rectangle = builder.build();

    rectangle.set_size(size.0, size.1);

    assert_eq!(rectangle.size(), size);
}

#[test]
fn test_set_x() {
    let x = 10.0;
    let builder = RectangleBuilder::new();
    let mut rectangle = builder.build();

    rectangle.set_x(x);

    assert_eq!(rectangle.x(), x);
}

#[test]
fn test_set_y() {
    let y = 10.0;
    let builder = RectangleBuilder::new();
    let mut rectangle = builder.build();

    rectangle.set_y(y);

    assert_eq!(rectangle.y(), y);
}

#[test]
fn test_set_position() {
    let position = (10.0, 20.0);
    let builder = RectangleBuilder::new();
    let mut rectangle = builder.build();

    rectangle.set_position(position.0, position.1);

    assert_eq!(rectangle.position(), position);
}

#[test]
fn test_set_border_thickness() {
    let thickness = Thickness::new(0.0, 10.0, 20.0, 30.0);
    let builder = RectangleBuilder::new();
    let mut rectangle = builder.build();

    rectangle.set_border_thickness(thickness.clone());

    assert_eq!(rectangle.border_thickness(), thickness);
}

#[test]
fn test_set_border_brush() {
    let brush = Brush::SolidColor("#000000".to_string());
    let builder = RectangleBuilder::new();
    let mut rectangle = builder.build();

    rectangle.set_border_brush(brush.clone());

    assert_eq!(rectangle.border_brush(), &brush);
}

#[test]
fn test_set_border_radius() {
    let radius = 5.0;
    let builder = RectangleBuilder::new();
    let mut rectangle = builder.build();

    rectangle.set_border_radius(radius);

    assert_eq!(rectangle.border_radius(), radius);
}

#[test]
fn test_set_border() {
    let border = Border::create().with_radius(4.0).build();
    let builder = RectangleBuilder::new();
    let mut rectangle = builder.build();

    rectangle.set_border(border.clone());

    assert_eq!(rectangle.border(), &border);
}
