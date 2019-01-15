use super::*;
use crate::core::{Position, Rect, Size};

#[test]
fn test_new() {
    let source = "test".to_string();

    let builder = ImageElementBuilder::new("test");

    assert_eq!(builder.source, source);

    let image = builder.build();

    assert_eq!(image.source().to_string(), source);
}

#[test]
fn test_with_position() {
    let position = (5.0, 10.0);

    let builder = ImageElementBuilder::new("");
    let image = builder.with_position(position.0, position.1).build();

    assert_eq!(image.position(), position);
}

#[test]
fn test_with_size() {
    let size = (5.0, 10.0);

    let builder = ImageElementBuilder::new("");
    let image = builder.with_size(size.0, size.1).build();

    assert_eq!(image.size(), size);
}

#[test]
fn test_with_rect() {
    let rect = (5.0, 10.0, 20.0, 30.0);

    let builder = ImageElementBuilder::new("");
    let image = builder.with_rect(rect.0, rect.1, rect.2, rect.3).build();

    assert_eq!(image.position(), ((rect.0, rect.1)));
    assert_eq!(image.size(), ((rect.2, rect.3)));
}

#[test]
fn test_with_source_rect() {
    let rect = Rect::new(0.0, 10.0, 20.0, 30.0);

    let builder = ImageElementBuilder::new("");
    let image = builder
        .with_source_rect(rect.x, rect.y, rect.width, rect.height)
        .build();

    assert!(image.source_rect().is_some());
    assert_eq!(image.source_rect().unwrap(), rect);
}

#[test]
fn test_set_source() {
    let source = "test".to_string();

    let builder = ImageElementBuilder::new("");
    let mut image = builder.build();

    image.set_source("test");

    assert_eq!(image.source().to_string(), source);
}

#[test]
fn test_set_source_rect() {
    let rect = Rect::new(0.0, 10.0, 20.0, 30.0);

    let builder = ImageElementBuilder::new("");
    let mut image = builder.build();

    image.set_source_rect(rect);

    assert!(image.source_rect().is_some());
    assert_eq!(image.source_rect().unwrap(), rect);
}

#[test]
fn test_set_width() {
    let width = 10.0;
    let builder = ImageElementBuilder::new("");
    let mut image = builder.build();

    image.set_width(width);

    assert_eq!(image.width(), width);
}

#[test]
fn test_set_height() {
    let height = 10.0;
    let builder = ImageElementBuilder::new("");
    let mut image = builder.build();

    image.set_height(height);

    assert_eq!(image.height(), height);
}

#[test]
fn test_set_size() {
    let size = (10.0, 20.0);
    let builder = ImageElementBuilder::new("");
    let mut image = builder.build();

    image.set_size(size.0, size.1);

    assert_eq!(image.size(), size);
}

#[test]
fn test_set_x() {
    let x = 10.0;
    let builder = ImageElementBuilder::new("");
    let mut image = builder.build();

    image.set_x(x);

    assert_eq!(image.x(), x);
}

#[test]
fn test_set_y() {
    let y = 10.0;
    let builder = ImageElementBuilder::new("");
    let mut image = builder.build();

    image.set_y(y);

    assert_eq!(image.y(), y);
}

#[test]
fn test_set_position() {
    let position = (10.0, 20.0);
    let builder = ImageElementBuilder::new("");
    let mut image = builder.build();

    image.set_position(position.0, position.1);

    assert_eq!(image.position(), position);
}



