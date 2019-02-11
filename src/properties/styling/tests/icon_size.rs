use super::*;

#[test]
fn test_into() {
    let icon_size: IconSize = 20.0.into();
    assert_eq!(icon_size.0, 20.0);
}