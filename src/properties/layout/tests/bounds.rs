use super::*;
use crate::structs::{Size, Position};

#[test]
fn test_contains() {
    let rect = Bounds::new(5.0, 8.0, 100.0, 80.0);
    let inner_point = Point::new(50.0, 50.0);
    let outer_point_one = Point::new(0.0, 0.0);
    let outer_point_two = Point::new(106.0, 80.0);

    assert!(rect.contains((inner_point.x, inner_point.y)));
    assert!(!rect.contains((outer_point_one.x, outer_point_one.y)));
    assert!(!rect.contains((outer_point_two.x, outer_point_two.y)));

    let neg_rect = Bounds::new(-5.0, -8.0, 100.0, 80.0);
    let neg_inner_point = Point::new(-3.0, -6.0);
    let neg_outer_point_one = Point::new(-10.0, -8.0);
    let neg_outer_point_two = Point::new(-5.0, 80.0);

    assert!(neg_rect.contains((neg_inner_point.x, neg_inner_point.y)));
    assert!(!neg_rect.contains((neg_outer_point_one.x, neg_outer_point_one.y)));
    assert!(!neg_rect.contains((neg_outer_point_two.x, neg_outer_point_two.y)));
}

#[test]
fn test_contains_rect() {
    let rect = Bounds::new(5.0, 8.0, 100.0, 80.0);
    let inner_rect = Bounds::new(10.0, 12.0, 20.0, 20.0);
    let outer_rect_one = Bounds::new(0.0, 0.0, 2.0, 3.0);
    let outer_rect_two = Bounds::new(105.0, 89.0, 100.0, 50.0);

    assert!(rect.contains_rect(&inner_rect));
    assert!(!rect.contains_rect(&outer_rect_one));
    assert!(!rect.contains_rect(&outer_rect_two));

    let neg_rect = Bounds::new(-5.0, -8.0, 100.0, 80.0);
    let inner_neg_rect = Bounds::new(-2.0, -6.0, 20.0, 20.0);
    let outer_neg_rect_one = Bounds::new(-30.0, -20.0, 2.0, 3.0);
    let outer_neg_rect_two = Bounds::new(105.0, 89.0, 100.0, 50.0);

    assert!(neg_rect.contains_rect(&inner_neg_rect));
    assert!(!neg_rect.contains_rect(&outer_neg_rect_one));
    assert!(!neg_rect.contains_rect(&outer_neg_rect_two));
}

#[test]
fn test_intersects() {
    let rect = Bounds::new(5.0, 8.0, 100.0, 80.0);
    let inner_rect = Bounds::new(2.0, 6.0, 100.0, 20.0);
    let outer_rect_one = Bounds::new(0.0, 0.0, 2.0, 3.0);
    let outer_rect_two = Bounds::new(105.0, 89.0, 100.0, 50.0);

    assert!(rect.intersects(&inner_rect));
    assert!(!rect.intersects(&outer_rect_one));
    assert!(!rect.intersects(&outer_rect_two));

    let neg_rect = Bounds::new(-5.0, -8.0, 100.0, 80.0);
    let inner_neg_rect = Bounds::new(-2.0, -6.0, 100.0, 120.0);
    let outer_neg_rect_one = Bounds::new(-30.0, -20.0, 2.0, 3.0);
    let outer_neg_rect_two = Bounds::new(105.0, 89.0, 100.0, 50.0);

    assert!(neg_rect.intersects(&inner_neg_rect));
    assert!(!neg_rect.intersects(&outer_neg_rect_one));
    assert!(!neg_rect.intersects(&outer_neg_rect_two));
}

#[test]
fn test_into() {
    let bounds: Bounds = (10.0, 12.0, 13.0, 14.0).into();

    assert_eq!(bounds.x(), 10.0);
    assert_eq!(bounds.y(), 12.0);
    assert_eq!(bounds.width(), 13.0);
    assert_eq!(bounds.height(), 14.0);
}
