use crate::structs::{Spacer, Thickness};

use super::*;

#[test]
fn test_set_left() {
    let left = 10.0;

    let mut margin = Margin::default();
    margin.set_left(left);

    assert_eq!(margin.left(), left);
}

#[test]
fn test_set_top() {
    let top = 10.0;

    let mut margin = Margin::default();
    margin.set_top(top);

    assert_eq!(margin.top(), top);
}

#[test]
fn test_set_right() {
    let right = 10.0;

    let mut margin = Margin::default();
    margin.set_right(right);

    assert_eq!(margin.right(), right);
}

#[test]
fn test_set_bottom() {
    let bottom = 10.0;

    let mut margin = Margin::default();
    margin.set_bottom(bottom);

    assert_eq!(margin.bottom(), bottom);
}

#[test]
fn test_set_thickness() {
    let thickness = Thickness {
        left: 10.0,
        right: 12.0,
        top: 14.0,
        bottom: 4.0,
    };

    let mut margin = Margin::default();
    margin.set_thickness(thickness);

    assert_eq!(margin.thickness(), thickness);
}
