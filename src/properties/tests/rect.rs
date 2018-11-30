use super::*;

#[test]
fn test_point() {
    let rect = Rect::new(5, 8, 0, 0);
    assert_eq!(rect.point().x, 5);
    assert_eq!(rect.point().y, 8);
}

#[test]
fn test_contains() {
    let rect = Rect::new(5, 8, 100, 80);
    let inner_point = Point::new(50, 50);
    let outer_point_one = Point::new(0, 0);
    let outer_point_two = Point::new(106, 80);

    assert!(rect.contains(inner_point));
    assert!(!rect.contains(outer_point_one));
    assert!(!rect.contains(outer_point_two));

    let neg_rect = Rect::new(-5, -8, 100, 80);
    let neg_inner_point = Point::new(-3, -6);
    let neg_outer_point_one = Point::new(-10, -8);
    let neg_outer_point_two = Point::new(-5, 80);

    assert!(neg_rect.contains(neg_inner_point));
    assert!(!neg_rect.contains(neg_outer_point_one));
    assert!(!neg_rect.contains(neg_outer_point_two));
}

#[test]
fn test_contains_rect() {
    let rect = Rect::new(5, 8, 100, 80);
    let inner_rect = Rect::new(10, 12, 20, 20);
    let outer_rect_one = Rect::new(0, 0, 2, 3);
    let outer_rect_two = Rect::new(105, 89, 100, 50);

    assert!(rect.contains_rect(&inner_rect));
    assert!(!rect.contains_rect(&outer_rect_one));
    assert!(!rect.contains_rect(&outer_rect_two));

    let neg_rect = Rect::new(-5, -8, 100, 80);
    let inner_neg_rect = Rect::new(-2, -6, 20, 20);
    let outer_neg_rect_one = Rect::new(-30, -20, 2, 3);
    let outer_neg_rect_two = Rect::new(105, 89, 100, 50);

    assert!(neg_rect.contains_rect(&inner_neg_rect));
    assert!(!neg_rect.contains_rect(&outer_neg_rect_one));
    assert!(!neg_rect.contains_rect(&outer_neg_rect_two));
}

#[test]
fn test_intersects() {
    let rect = Rect::new(5, 8, 100, 80);
    let inner_rect = Rect::new(2, 6, 100, 20);
    let outer_rect_one = Rect::new(0, 0, 2, 3);
    let outer_rect_two = Rect::new(105, 89, 100, 50);

    assert!(rect.intersects(&inner_rect));
    assert!(!rect.intersects(&outer_rect_one));
    assert!(!rect.intersects(&outer_rect_two));

    let neg_rect = Rect::new(-5, -8, 100, 80);
    let inner_neg_rect = Rect::new(-2, -6, 100, 120);
    let outer_neg_rect_one = Rect::new(-30, -20, 2, 3);
    let outer_neg_rect_two = Rect::new(105, 89, 100, 50);

    assert!(neg_rect.intersects(&inner_neg_rect));
    assert!(!neg_rect.intersects(&outer_neg_rect_one));
    assert!(!neg_rect.intersects(&outer_neg_rect_two));
}
