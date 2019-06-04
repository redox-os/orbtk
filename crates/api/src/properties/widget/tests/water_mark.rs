use super::*;

#[test]
fn test_into() {
    let water_mark: WaterMark = "test".into();
    assert_eq!(water_mark.0, "test");
    assert_eq!(water_mark.0, "test");
}
