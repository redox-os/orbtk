use std::ops::{Add, Sub};

use super::*;

#[test]
fn test_add() {
    let point = Point::new(5.0, 8.0);
    let other_point = Point::new(10.0, 12.0);
    let result = point.add(other_point);

    assert_eq!(result.x, 15.0);
    assert_eq!(result.y, 20.0);

    let neg_point = Point::new(-5.0, 8.0);
    let other_neg_point = Point::new(-7.0, -12.0);
    let result = neg_point.add(other_neg_point);

    assert_eq!(result.x, -12.0);
    assert_eq!(result.y, -4.0);
}

#[test]
fn test_sub() {
    let point = Point::new(5.0, 8.0);
    let other_point = Point::new(10.0, 12.0);
    let result = point.sub(other_point);

    assert_eq!(result.x, -5.0);
    assert_eq!(result.y, -4.0);

    let neg_point = Point::new(-5.0, 8.0);
    let other_neg_point = Point::new(-7.0, -12.0);
    let result = neg_point.sub(other_neg_point);

    assert_eq!(result.x, 2.0);
    assert_eq!(result.y, 20.0);
}
