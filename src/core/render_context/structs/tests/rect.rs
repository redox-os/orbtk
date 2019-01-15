use super::*;

#[test]
fn test_new() {
    let rect = Rect::new(5.0, 10.0, 20.0, 30.0);
   
    assert_eq!(rect.x, 5.0);
    assert_eq!(rect.y, 10.0);
    assert_eq!(rect.width, 20.0);
    assert_eq!(rect.height, 30.0);
}