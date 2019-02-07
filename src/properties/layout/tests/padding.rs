use crate::structs::{Spacer, Thickness};

use super::*;

#[test]
fn test_set_left() {
    let left = 10.0;

    let mut padding = Padding::default();
    padding.set_left(left);

    assert_eq!(padding.left(), left);
}

#[test]
fn test_set_top() {
    let top = 10.0;

    let mut padding = Padding::default();
    padding.set_top(top);

    assert_eq!(padding.top(), top);
}

#[test]
fn test_set_right() {
    let right = 10.0;

    let mut padding = Padding::default();
    padding.set_right(right);

    assert_eq!(padding.right(), right);
}

#[test]
fn test_set_bottom() {
    let bottom = 10.0;

    let mut padding = Padding::default();
    padding.set_bottom(bottom);

    assert_eq!(padding.bottom(), bottom);
}

#[test]
fn test_set_thickness() {
    let thickness = Thickness {
        left: 10.0,
        right: 12.0,
        top: 14.0,
        bottom: 4.0,
    };

    let mut padding = Padding::default();
    padding.set_thickness(thickness);

    assert_eq!(padding.thickness(), thickness);
}

#[test]
fn test_into() {
    let padding: Padding = (10.0, 12.0, 13.0, 14.0).into();

    assert_eq!(padding.left(), 10.0);
    assert_eq!(padding.top(), 12.0);
    assert_eq!(padding.right(), 13.0);
    assert_eq!(padding.bottom(), 14.0);

    let padding: Padding = 10.0.into();

    assert_eq!(padding.left(), 10.0);
    assert_eq!(padding.top(), 10.0);
    assert_eq!(padding.right(), 10.0);
    assert_eq!(padding.bottom(), 10.0);
}
