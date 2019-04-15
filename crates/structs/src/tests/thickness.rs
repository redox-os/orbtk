use super::*;

#[test]
fn test_new() {
    let rect = Thickness::new(5.0, 10.0, 20.0, 30.0);

    assert_eq!(rect.left, 5.0);
    assert_eq!(rect.top, 10.0);
    assert_eq!(rect.right, 20.0);
    assert_eq!(rect.bottom, 30.0);
}

#[test]
fn test_into() {
    let thickness: Thickness = (10.0, 12.0, 13.0, 14.0).into();

    assert_eq!(thickness.left, 10.0);
    assert_eq!(thickness.top, 12.0);
    assert_eq!(thickness.right, 13.0);
    assert_eq!(thickness.bottom, 14.0);

    let thickness: Thickness = 10.0.into();

    assert_eq!(thickness.left, 10.0);
    assert_eq!(thickness.top, 10.0);
    assert_eq!(thickness.right, 10.0);
    assert_eq!(thickness.bottom, 10.0);
}
