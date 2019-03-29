use super::*;

#[test]
fn test_into() {
    let offset: Offset = 14.0.into();
    assert_eq!(offset.0, 14.0);
    assert_eq!(offset.1, 14.0);

     let offset: Offset = (14.0, 16.0).into();
    assert_eq!(offset.0, 14.0);
    assert_eq!(offset.1, 16.0);
}