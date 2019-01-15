use super::*;

#[test]
fn test_new() {
    let rect = Thickness::new(5.0, 10.0, 20.0, 30.0);
   
    assert_eq!(rect.left, 5.0);
    assert_eq!(rect.top, 10.0);
    assert_eq!(rect.right, 20.0);
    assert_eq!(rect.bottom, 30.0);
}