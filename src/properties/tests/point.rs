use std::ops::{Add, Sub};

use super::*;

#[test]
fn test_add() {
    let point = Point::new(5, 8);
    let other_point = Point::new(10, 12);
    let result = point.add(other_point);

    assert_eq!(result.x, 15);
    assert_eq!(result.y, 20);

    let neg_point = Point::new(-5, 8);
    let other_neg_point = Point::new(-7, -12);
    let result = neg_point.add(other_neg_point);

    assert_eq!(result.x, -12);
    assert_eq!(result.y, -4);
}

#[test]
fn test_sub() {
    let point = Point::new(5, 8);
    let other_point = Point::new(10, 12);
    let result = point.sub(other_point);

    assert_eq!(result.x, -5);
    assert_eq!(result.y, -4);

    let neg_point = Point::new(-5, 8);
    let other_neg_point = Point::new(-7, -12);
    let result = neg_point.sub(other_neg_point);

    assert_eq!(result.x, 2);
    assert_eq!(result.y, 20);
}
